#![doc(hidden)]
//! # Common Request Types
//!
//! Shared request components and filters used across multiple OpenFIGI API endpoints.
//! Provides reusable filter structures and validation logic for request parameters.
//!
//! Note: This module is not intended for direct use by consumers of the OpenFIGI API.

use crate::{
    error::{OpenFIGIError, OtherErrorKind, Result},
    model::enums::{
        Currency, ExchCode, MarketSecDesc, MicCode, OptionType, SecurityType, SecurityType2,
        StateCode,
    },
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Common filter parameters for OpenFIGI API requests.
///
/// Provides optional filters that can be applied to various endpoints for refining search results.
/// All fields are optional and can be combined to create more specific queries.
///
/// # Validation Rules
///
/// - `exch_code` and `mic_code` are mutually exclusive
/// - Numeric ranges (`strike`, `contract_size`, `coupon`) must have start ≤ end
/// - Date ranges (`expiration`, `maturity`) must have start ≤ end and span ≤ 1 year
/// - `expiration` is required for Option or Warrant security types
/// - `maturity` is required for Pool security types
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestFilters {
    /// Exchange code of the desired instrument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exch_code: Option<ExchCode>,
    /// ISO market identifier code (MIC) of the desired instrument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mic_code: Option<MicCode>,
    /// Currency associated to the desired instrument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// Market sector description of the desired instrument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_sec_des: Option<MarketSecDesc>,
    /// Security type of the desired instrument
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_type: Option<SecurityType>,
    /// An alternative security type of the desired instrument.
    /// `securityType2` is typically less specific than `securityType`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_type2: Option<SecurityType2>,
    /// Set to `true` to include equity instruments that are not listed on an exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_unlisted_equities: Option<bool>,
    /// Will filter instruments based on option type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_type: Option<OptionType>,
    /// Will find instruments whose strike price falls in an interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike: Option<[Option<f64>; 2]>,
    /// Will find instruments whose contract size falls in an interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_size: Option<[Option<f64>; 2]>,
    /// Will find instruments whose coupon falls in an interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<[Option<f64>; 2]>,
    /// Will find instruments whose expiration date falls in an interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<[Option<NaiveDate>; 2]>,
    /// Will find instruments whose maturity date falls in an interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturity: Option<[Option<NaiveDate>; 2]>,
    /// State code of the desired instrument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_code: Option<StateCode>,
}

impl RequestFilters {
    // Helper function to validate that start <= end for Option<[Option<T>; 2]>
    fn validate_number_range(
        field: Option<&[Option<f64>; 2]>,
        field_name: &'static str,
    ) -> Result<()> {
        if let Some([Some(start), Some(end)]) = field {
            if start > end {
                return Err(OpenFIGIError::other_error(
                    OtherErrorKind::Validation,
                    format!("{field_name}: start value cannot be greater than end value"),
                ));
            }
        }
        Ok(())
    }

    // Validate that the range is valid for Option<[Option<T>; 2]>
    fn validate_date_range(
        field: Option<&[Option<NaiveDate>; 2]>,
        field_name: &'static str,
    ) -> Result<()> {
        if let Some([Some(start), Some(end)]) = field {
            if start > end {
                return Err(OpenFIGIError::other_error(
                    OtherErrorKind::Validation,
                    format!("{field_name}: start date cannot be after end date"),
                ));
            } else if *end > (*start + chrono::Duration::days(365)) {
                return Err(OpenFIGIError::other_error(
                    OtherErrorKind::Validation,
                    format!("{field_name}: date range cannot exceed 1 year"),
                ));
            }
        }
        Ok(())
    }

    /// Validates that mutually exclusive fields are not used together.
    fn validate_mutual_exclusions(&self) -> Result<()> {
        if self.exch_code.is_some() && self.mic_code.is_some() {
            return Err(OpenFIGIError::other_error(
                OtherErrorKind::Validation,
                "Cannot set both exchCode and micCode",
            ));
        }
        // Add any other mutual exclusion rules here in the future.
        Ok(())
    }

    /// Validates that all numeric and date ranges are ordered correctly.
    fn validate_ranges(&self) -> Result<()> {
        // Validate strike, contract_size, coupon
        Self::validate_number_range(self.strike.as_ref(), "strike")?;
        Self::validate_number_range(self.contract_size.as_ref(), "contract_size")?;
        Self::validate_number_range(self.coupon.as_ref(), "coupon")?;

        // Validate expiration and maturity dates
        Self::validate_date_range(self.expiration.as_ref(), "expiration")?;
        Self::validate_date_range(self.maturity.as_ref(), "maturity")?;
        Ok(())
    }

    /// Validates fields that are required only under certain conditions.
    fn validate_conditional_requirements(&self) -> Result<()> {
        // expiration is required if securityType is Option or Warrant
        if (self.security_type2 == Some(SecurityType2::Option)
            || self.security_type2 == Some(SecurityType2::Warrant))
            && self.expiration.is_none()
        {
            return Err(OpenFIGIError::other_error(
                OtherErrorKind::Validation,
                "expiration is required for Option or Warrant security types",
            ));
        }

        // maturity is required if securityType is Pool
        if (self.security_type2 == Some(SecurityType2::Pool)) && self.maturity.is_none() {
            return Err(OpenFIGIError::other_error(
                OtherErrorKind::Validation,
                "maturity is required for Pool security types",
            ));
        }
        Ok(())
    }

    /// Validates all filter parameters and their combinations.
    ///
    /// Ensures that:
    /// - Mutually exclusive fields are not both set
    /// - Numeric ranges have valid start/end values
    /// - Date ranges are valid and within acceptable limits
    /// - Required fields are present for specific security types
    ///
    /// # Errors
    ///
    /// Returns [`OpenFIGIError`] with [`OtherErrorKind::Validation`] if validation fails.
    pub fn validate(&self) -> Result<()> {
        self.validate_mutual_exclusions()?;
        self.validate_ranges()?;
        self.validate_conditional_requirements()?;

        Ok(())
    }

    /// Returns `true` if all filter fields are unset.
    ///
    /// Useful for determining if any filters have been applied to the request.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.exch_code.is_none()
            && self.mic_code.is_none()
            && self.currency.is_none()
            && self.market_sec_des.is_none()
            && self.security_type.is_none()
            && self.security_type2.is_none()
            && self.include_unlisted_equities.is_none()
            && self.option_type.is_none()
            && self.strike.is_none()
            && self.contract_size.is_none()
            && self.coupon.is_none()
            && self.expiration.is_none()
            && self.maturity.is_none()
            && self.state_code.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::enums::{Currency, ExchCode, MicCode, SecurityType2};
    use chrono::NaiveDate;

    fn create_sample_filters() -> RequestFilters {
        RequestFilters {
            currency: Some(Currency::USD),
            exch_code: Some(ExchCode::US),
            ..Default::default()
        }
    }

    #[test]
    fn test_default_filters_are_empty() {
        let filters = RequestFilters::default();
        assert!(filters.is_empty());
        assert!(filters.validate().is_ok());
    }

    #[test]
    fn test_is_empty_with_single_field() {
        let mut filters = RequestFilters::default();
        assert!(filters.is_empty());

        filters.currency = Some(Currency::USD);
        assert!(!filters.is_empty());
    }

    #[test]
    fn test_is_empty_with_multiple_fields() {
        let filters = create_sample_filters();
        assert!(!filters.is_empty());
    }

    #[test]
    fn test_valid_filters_pass_validation() {
        let filters = create_sample_filters();
        assert!(filters.validate().is_ok());
    }

    #[test]
    fn test_mutually_exclusive_exch_code_and_mic_code() {
        let filters = RequestFilters {
            exch_code: Some(ExchCode::US),
            mic_code: Some(MicCode::XNYS),
            ..Default::default()
        };

        let result = filters.validate();
        assert!(result.is_err());

        if let Err(error) = result {
            assert!(
                error
                    .to_string()
                    .contains("Cannot set both exchCode and micCode")
            );
        }
    }

    #[test]
    fn test_valid_strike_range() {
        let filters = RequestFilters {
            strike: Some([Some(100.0), Some(200.0)]),
            ..Default::default()
        };
        assert!(filters.validate().is_ok());
    }

    #[test]
    fn test_invalid_number_range() {
        let filters = RequestFilters {
            strike: Some([Some(200.0), Some(100.0)]), // start > end
            ..Default::default()
        };

        let result = filters.validate();
        assert!(result.is_err());

        if let Err(error) = result {
            assert!(
                error
                    .to_string()
                    .contains("strike: start value cannot be greater than end value")
            );
        }
    }

    #[test]
    fn test_valid_date_range() {
        let start_date =
            NaiveDate::from_ymd_opt(2024, 1, 1).expect("Should create valid start_date");
        let end_date = NaiveDate::from_ymd_opt(2024, 6, 1).expect("Should create valid end_date");
        let filters = RequestFilters {
            expiration: Some([Some(start_date), Some(end_date)]),
            ..Default::default()
        };

        assert!(filters.validate().is_ok());
    }

    #[test]
    fn test_invalid_date_range_order() {
        let start_date =
            NaiveDate::from_ymd_opt(2024, 1, 1).expect("Should create valid start_date");
        let end_date = NaiveDate::from_ymd_opt(2024, 6, 1).expect("Should create valid end_date");
        let filters = RequestFilters {
            expiration: Some([Some(start_date), Some(end_date)]),
            ..Default::default()
        };

        let result = filters.validate();
        assert!(result.is_err());

        if let Err(error) = result {
            assert!(
                error
                    .to_string()
                    .contains("expiration: start date cannot be after end date")
            );
        }
    }

    #[test]
    fn test_invalid_date_range_too_long() {
        let start_date =
            NaiveDate::from_ymd_opt(2024, 1, 1).expect("Should create valid start_date");
        let end_date = NaiveDate::from_ymd_opt(2024, 6, 1).expect("Should create valid end_date"); // > 1 year
        let filters = RequestFilters {
            expiration: Some([Some(start_date), Some(end_date)]),
            ..Default::default()
        };

        let result = filters.validate();
        assert!(result.is_err());

        if let Err(error) = result {
            assert!(
                error
                    .to_string()
                    .contains("expiration: date range cannot exceed 1 year")
            );
        }
    }

    #[test]
    fn test_option_requires_expiration() {
        let filters = RequestFilters {
            security_type2: Some(SecurityType2::Option),
            ..Default::default()
        };
        // No expiration set

        let result = filters.validate();
        assert!(result.is_err());

        if let Err(error) = result {
            assert!(
                error
                    .to_string()
                    .contains("expiration is required for Option or Warrant security types")
            );
        }
    }

    #[test]
    fn test_option_with_valid_expiration() {
        let expiration_date =
            NaiveDate::from_ymd_opt(2024, 12, 20).expect("Should create valid expiration_date");
        let filters = RequestFilters {
            security_type2: Some(SecurityType2::Option),
            expiration: Some([Some(expiration_date), None]),
            ..Default::default()
        };

        assert!(filters.validate().is_ok());
    }

    #[test]
    fn test_pool_requires_maturity() {
        let filters = RequestFilters {
            security_type2: Some(SecurityType2::Pool),
            ..Default::default()
        };
        // No maturity set

        let result = filters.validate();
        assert!(result.is_err());

        if let Err(error) = result {
            assert!(
                error
                    .to_string()
                    .contains("maturity is required for Pool security types")
            );
        }
    }

    #[test]
    fn test_pool_with_valid_maturity() {
        let maturity_date =
            NaiveDate::from_ymd_opt(2025, 1, 15).expect("Should create valid maturity_date");
        let filters = RequestFilters {
            security_type2: Some(SecurityType2::Pool),
            maturity: Some([Some(maturity_date), None]),
            ..Default::default()
        };

        assert!(filters.validate().is_ok());
    }

    #[test]
    fn test_partial_ranges_are_valid() {
        // Test with only start values
        let mut filters = RequestFilters {
            strike: Some([Some(100.0), None]),
            expiration: Some([
                Some(NaiveDate::from_ymd_opt(2024, 1, 1).expect("Should create valid date")),
                None,
            ]),
            ..Default::default()
        };

        assert!(filters.validate().is_ok());

        // Test with only end values
        filters.strike = Some([None, Some(200.0)]);
        filters.expiration = Some([
            None,
            Some(NaiveDate::from_ymd_opt(2024, 12, 31).expect("Should create valid date")),
        ]);

        assert!(filters.validate().is_ok());
    }

    #[test]
    fn test_serialization_skips_none_values() {
        let filters = RequestFilters {
            currency: Some(Currency::USD),
            exch_code: None,
            ..Default::default()
        };

        let json = serde_json::to_string(&filters).expect("Failed to serialize filters to JSON");
        assert!(json.contains("currency"));
        assert!(!json.contains("exchCode"));
    }

    #[test]
    fn test_all_fields_none_is_empty() {
        let filters = RequestFilters {
            exch_code: None,
            mic_code: None,
            currency: None,
            market_sec_des: None,
            security_type: None,
            security_type2: None,
            include_unlisted_equities: None,
            option_type: None,
            strike: None,
            contract_size: None,
            coupon: None,
            expiration: None,
            maturity: None,
            state_code: None,
        };

        assert!(filters.is_empty());
    }
}
