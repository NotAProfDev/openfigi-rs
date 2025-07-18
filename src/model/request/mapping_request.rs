//! # Mapping Request Types
//!
//! Request structures for the OpenFIGI `/mapping` endpoint (see [here](https://www.openfigi.com/api/documentation#v3-post-mapping) for more details).
//! Provides types for building identifier mapping requests with fluent builder patterns.
//!
//! ## Examples
//!
//! ### Basic mapping request
//!
//! ```rust
//! use openfigi_rs::model::request::MappingRequest;
//! use openfigi_rs::model::enums::IdType;
//!
//! let request = MappingRequest::new(IdType::IdIsin, "US4592001014");
//! ```
//!
//! ### Mapping request with additional filters
//!
//! ```rust
//! use openfigi_rs::model::request::MappingRequest;
//! use openfigi_rs::model::enums::{IdType, Currency, ExchCode};
//!
//! let request = MappingRequest::builder()
//!     .id_type(IdType::IdIsin)
//!     .id_value("US4592001014")
//!     .currency(Currency::USD)
//!     .exch_code(ExchCode::US)
//!     .build()
//!     .unwrap();
//! ```
//!
//! Note: This module is not intended for direct use by consumers of the OpenFIGI API.

use crate::{
    error::{OpenFIGIError, OtherErrorKind, Result},
    impl_filter_builder,
    model::{
        enums::{
            Currency, ExchCode, IdType, MarketSecDesc, MicCode, OptionType, SecurityType,
            SecurityType2, StateCode,
        },
        request::common::RequestFilters,
    },
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Request structure for the OpenFIGI `/mapping` endpoint.
///
/// Represents a mapping request that converts third-party identifiers to FIGIs.
/// Requires an identifier type and value, with optional filtering criteria.
///
/// # Required Fields
///
/// - `id_type`: The type of third-party identifier being mapped
/// - `id_value`: The value of the identifier (string or number)
///
/// # Optional Fields
///
/// - `filters`: Additional filtering criteria (flattened into the request JSON)
///
/// # Validation Rules
///
/// - `security_type2` is required when `id_type` is `BASE_TICKER` or `ID_EXCH_SYMBOL`
/// - All filter validation rules apply (see common filters documentation `RequestFilters::validate()`)
///
/// # Examples
///
/// ```rust
/// use openfigi_rs::model::request::MappingRequest;
/// use openfigi_rs::model::enums::{IdType, Currency};
///
/// // Simple identifier mapping
/// let request = MappingRequest::new(IdType::IdIsin, "US4592001014");
///
/// // Mapping with additional filters
/// let request = MappingRequest::builder()
///     .id_type(IdType::Ticker)
///     .id_value("AAPL")
///     .currency(Currency::USD)
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MappingRequest {
    /// Type of third party identifier.
    pub id_type: IdType,
    /// Value of the third party identifier.
    pub id_value: serde_json::Value, // String or Number

    /// Additional filtering criteria applied to the mapping request.
    ///
    /// These filters are flattened into the JSON structure and provide
    /// optional constraints to refine the mapping results see `RequestFilters`.
    #[serde(flatten)]
    pub filters: RequestFilters,
}

impl MappingRequest {
    /// Creates a new `MappingRequest` with required identifier information.
    ///
    /// All filter fields are initialized to their default values (typically `None`).
    /// Use [`MappingRequest::builder()`] for a more convenient fluent API.
    ///
    /// # Arguments
    ///
    /// * `id_type` - The type of third-party identifier
    /// * `id_value` - The identifier value (can be string or number)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::model::request::MappingRequest;
    /// use openfigi_rs::model::enums::IdType;
    ///
    /// let request = MappingRequest::new(IdType::IdIsin, "US4592001014");
    /// assert_eq!(request.id_type, IdType::IdIsin);
    /// ```
    #[must_use]
    pub fn new<T: Into<serde_json::Value>>(id_type: IdType, id_value: T) -> Self {
        Self {
            id_type,
            id_value: id_value.into(),
            filters: RequestFilters::default(),
        }
    }

    /// Creates a new `MappingRequestBuilder` for fluent request construction.
    ///
    /// Provides a convenient way to build mapping requests with method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::model::request::MappingRequest;
    /// use openfigi_rs::model::enums::{IdType, Currency};
    ///
    /// let request = MappingRequest::builder()
    ///     .id_type(IdType::Ticker)
    ///     .id_value("AAPL")
    ///     .currency(Currency::USD)
    ///     .build()
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn builder() -> MappingRequestBuilder {
        MappingRequestBuilder::new()
    }

    /// Validates the mapping request.
    ///
    /// Ensures that:
    /// - Required fields (`id_type`, `id_value`) are present
    /// - `security_type2` is provided when required by certain identifier types
    /// - All filter validation rules are satisfied
    /// - No mutually exclusive parameters are set
    ///
    /// # Errors
    ///
    /// Returns [`OpenFIGIError`] with [`OtherErrorKind::Validation`] if validation fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::model::request::MappingRequest;
    /// use openfigi_rs::model::enums::{IdType, SecurityType2};
    ///
    /// // This will fail validation - BASE_TICKER requires security_type2
    /// let mut request = MappingRequest::new(IdType::BaseTicker, "AAPL");
    /// assert!(request.validate().is_err());
    ///
    /// // Add required security_type2
    /// request.filters.security_type2 = Some(SecurityType2::CommonStock);
    /// assert!(request.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<()> {
        // Validate the `RequestFilters` fields
        self.filters.validate()?;

        // securityType2 is required when idType is BASE_TICKER or ID_EXCH_SYMBOL
        if (self.id_type == IdType::BASE_TICKER || self.id_type == IdType::ID_EXCH_SYMBOL)
            && self.filters.security_type2.is_none()
        {
            return Err(OpenFIGIError::other_error(
                OtherErrorKind::Validation,
                "securityType2 is required when idType is BASE_TICKER or ID_EXCH_SYMBOL",
            ));
        }
        Ok(())
    }
}

/// Builder for constructing [`MappingRequest`] instances.
///
/// Provides a fluent API for setting identifier information and filter parameters.
/// All methods return `self` to enable method chaining.
///
/// # Required Fields
///
/// - `id_type`: Must be set via [`id_type()`](Self::id_type)
/// - `id_value`: Must be set via [`id_value()`](Self::id_value)
///
/// # Examples
///
/// ```rust
/// use openfigi_rs::model::request::MappingRequestBuilder;
/// use openfigi_rs::model::enums::{IdType, Currency, ExchCode};
///
/// let request = MappingRequestBuilder::new()
///     .id_type(IdType::IdIsin)
///     .id_value("US4592001014")
///     .currency(Currency::USD)
///     .exch_code(ExchCode::US)
///     .build()
///     .unwrap();
/// ```
#[derive(Default)]
pub struct MappingRequestBuilder {
    id_type: Option<IdType>,
    id_value: Option<serde_json::Value>,
    filters: RequestFilters,
}

impl MappingRequestBuilder {
    /// Creates a new `MappingRequestBuilder` with all fields unset.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::model::request::MappingRequestBuilder;
    ///
    /// let builder = MappingRequestBuilder::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the identifier type for the mapping request.
    ///
    /// This field is required and specifies what type of third-party identifier
    /// is being mapped to a FIGI.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::model::request::MappingRequestBuilder;
    /// use openfigi_rs::model::enums::IdType;
    ///
    /// let builder = MappingRequestBuilder::new().id_type(IdType::IdIsin);
    /// ```
    #[must_use]
    pub fn id_type(mut self, id_type: IdType) -> Self {
        self.id_type = Some(id_type);
        self
    }

    /// Sets the identifier value for the mapping request.
    ///
    /// This field is required and contains the actual identifier value to be mapped.
    /// Can accept strings, numbers, or any value that converts to `serde_json::Value`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::model::request::MappingRequestBuilder;
    /// use serde_json::json;
    ///
    /// let builder = MappingRequestBuilder::new()
    ///     .id_value("US4592001014")      // String
    ///     .id_value(json!("AAPL"))       // JSON string
    ///     .id_value(12345);              // Number
    /// ```
    #[must_use]
    pub fn id_value<T: Into<serde_json::Value>>(mut self, id_value: T) -> Self {
        self.id_value = Some(id_value.into());
        self
    }

    /// Mutable access to the request filters.
    pub fn filters_mut(&mut self) -> &mut RequestFilters {
        &mut self.filters
    }

    // Bring in common builder methods for filtering logic
    impl_filter_builder!();

    /// Builds and validates the `MappingRequest`.
    ///
    /// Constructs the final request object and performs validation to ensure
    /// all requirements are met.
    ///
    /// # Errors
    ///
    /// Returns [`OpenFIGIError`] if validation fails, such as:
    /// - Missing required fields (`id_type` or `id_value`)
    /// - Missing `security_type2` when required by identifier type
    /// - Mutually exclusive parameters set
    /// - Invalid parameter ranges
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::model::request::MappingRequestBuilder;
    /// use openfigi_rs::model::enums::IdType;
    ///
    /// let request = MappingRequestBuilder::new()
    ///     .id_type(IdType::IdIsin)
    ///     .id_value("US4592001014")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<MappingRequest> {
        let id_type = self.id_type.ok_or_else(|| {
            OpenFIGIError::other_error(OtherErrorKind::Validation, "id_type is required")
        })?;
        let id_value = self.id_value.ok_or_else(|| {
            OpenFIGIError::other_error(OtherErrorKind::Validation, "id_value is required")
        })?;
        let request = MappingRequest {
            id_type,
            id_value,
            filters: self.filters,
        };
        request.validate()?;
        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::enums::{Currency, ExchCode, IdType, MicCode, SecurityType2};
    use chrono::NaiveDate;
    use serde_json::json;

    #[test]
    fn test_mapping_request_new_minimal() {
        let request = MappingRequest::new(IdType::IdIsin, json!("US1234567890"));
        assert_eq!(request.id_type, IdType::IdIsin);
        assert_eq!(request.id_value, json!("US1234567890"));
        assert!(request.filters.exch_code.is_none());
        assert!(request.filters.mic_code.is_none());
    }

    #[test]
    fn test_mapping_request_builder_minimal() {
        let request = MappingRequest::builder()
            .id_type(IdType::IdIsin)
            .id_value("US1234567890")
            .build()
            .unwrap();
        assert_eq!(request.id_type, IdType::IdIsin);
        assert_eq!(request.id_value, json!("US1234567890"));
    }

    #[test]
    fn test_mapping_request_builder_with_currency() {
        let request = MappingRequest::builder()
            .id_type(IdType::IdIsin)
            .id_value("US1234567890")
            .currency(Currency::USD)
            .build()
            .unwrap();
        assert_eq!(request.filters.currency, Some(Currency::USD));
    }

    #[test]
    fn test_mapping_request_validate_exch_and_mic_code_conflict() {
        let mut request = MappingRequest::new(IdType::IdIsin, json!("US1234567890"));
        request.filters.exch_code = Some(ExchCode::A0);
        request.filters.mic_code = Some(MicCode::XCME);
        let result = request.validate();
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("Cannot set both exchCode and micCode"));
    }

    #[test]
    fn test_mapping_request_validate_security_type2_required() {
        let mut request = MappingRequest::new(IdType::BaseTicker, json!("IBM"));
        request.filters.security_type2 = None;
        let result = request.validate();
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("securityType2 is required"));
    }

    #[test]
    fn test_mapping_request_validate_strike_range() {
        let mut request = MappingRequest::new(IdType::IdIsin, json!("US1234567890"));
        request.filters.strike = Some([Some(10.0), Some(5.0)]);
        let result = request.validate();
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("strike: start value cannot be greater than end value"));
    }

    #[test]
    fn test_mapping_request_validate_expiration_required_for_option() {
        let mut request = MappingRequest::new(IdType::IdIsin, json!("US1234567890"));
        request.filters.security_type2 = Some(SecurityType2::Option);
        request.filters.expiration = None;
        let result = request.validate();
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("expiration is required for Option or Warrant security types"));
    }

    #[test]
    fn test_mapping_request_validate_maturity_required_for_pool() {
        let mut request = MappingRequest::new(IdType::IdIsin, json!("US1234567890"));
        request.filters.security_type2 = Some(SecurityType2::Pool);
        let result = request.validate();
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("maturity is required for Pool security types"));
    }

    #[test]
    fn test_mapping_request_validate_date_range_too_long() {
        let mut request = MappingRequest::new(IdType::IdIsin, json!("US1234567890"));
        let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2026, 2, 1).unwrap();
        request.filters.expiration = Some([Some(start), Some(end)]);
        let result = request.validate();
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("date range cannot exceed 1 year"));
    }

    #[test]
    fn test_serialize_deserialize_mapping_request() {
        let request = MappingRequest::builder()
            .id_type(IdType::IdIsin)
            .id_value("US1234567890")
            .currency(Currency::USD)
            .build()
            .unwrap();
        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: MappingRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }
}
