//! # Filter Request Types
//!
//! Request structures for the OpenFIGI `/filter` endpoint (see [here](https://www.openfigi.com/api/documentation#v3-post-filter) for more details).
//! Provides types for building filter requests with fluent builder patterns.
//!
//! ## Examples
//!
//! ### Basic filter request
//!
//! ```rust
//! use openfigi_rs::model::request::FilterRequest;
//!
//! let request = FilterRequest::builder()
//!     .query("AAPL")
//!     .build()
//!     .unwrap();
//! ```
//!
//! ### Filter request with additional parameters
//!
//! ```rust
//! use openfigi_rs::model::request::FilterRequest;
//! use openfigi_rs::model::enums::{Currency, ExchCode};
//!
//! let request = FilterRequest::builder()
//!     .query("technology")
//!     .currency(Currency::USD)
//!     .exch_code(ExchCode::US)
//!     .build()
//!     .unwrap();
//! ```
//!
//! Note: This module is not intended for direct use by consumers of the OpenFIGI API.

use crate::{
    error::{OpenFIGIError, OtherErrorKind, Result},
    model::{
        enums::{
            Currency, ExchCode, MarketSecDesc, MicCode, OptionType, SecurityType, SecurityType2,
            StateCode,
        },
        request::common::RequestFilters,
    },
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Request structure for the OpenFIGI `/filter` endpoint.
///
/// Represents a filter request that can include search keywords and various filtering criteria.
/// At least one of `query` or a filter parameter must be specified for a valid request.
///
/// # Fields
///
/// - `query`: Optional search keywords for finding instruments
/// - `start`: Optional pagination token for retrieving subsequent pages
/// - `filters`: Additional filtering criteria (flattened into the request JSON)
///
/// # Examples
///
/// ```rust
/// use openfigi_rs::model::request::FilterRequest;
/// use openfigi_rs::model::enums::Currency;
///
/// // Simple query-based filter
/// let request = FilterRequest::builder()
///     .query("IBM")
///     .build()
///     .unwrap();
///
/// // Filter with additional criteria
/// let request = FilterRequest::builder()
///     .query("technology stocks")
///     .currency(Currency::USD)
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterRequest {
    /// Filter keywords for finding FIGIs.
    pub query: Option<String>,
    /// Pagination token for retrieving subsequent result pages.
    ///
    /// When more results are available, the response contains a `next` property
    /// whose value should be sent in succeeding requests as the `start` value
    /// to retrieve the next "page" of results.
    pub start: Option<String>,

    /// Additional filtering criteria applied to the mapping request.
    ///
    /// These filters are flattened into the JSON structure and provide
    /// optional constraints to refine the mapping results see [`RequestFilters`].
    #[serde(flatten)]
    pub filters: RequestFilters,
}

impl FilterRequest {
    /// Creates a new empty `FilterRequest`.
    ///
    /// All fields are initialized to `None` or default values.
    /// Use [`FilterRequest::builder()`] for a more convenient fluent API.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::model::request::FilterRequest;
    ///
    /// let request = FilterRequest::new();
    /// assert!(request.query.is_none());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `FilterRequestBuilder` for fluent request construction.
    ///
    /// Provides a convenient way to build filter requests with method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::model::request::FilterRequest;
    /// use openfigi_rs::model::enums::Currency;
    ///
    /// let request = FilterRequest::builder()
    ///     .query("AAPL")
    ///     .currency(Currency::USD)
    ///     .build()
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn builder() -> FilterRequestBuilder {
        FilterRequestBuilder::new()
    }

    /// Validates the filter request.
    ///
    /// Ensures that:
    /// - At least one field (query or filter) is specified
    /// - All filter validation rules are satisfied
    /// - No mutually exclusive parameters are set
    /// - Numeric and date ranges are valid
    ///
    /// # Errors
    ///
    /// Returns [`OpenFIGIError`] with [`OtherErrorKind::Validation`] if validation fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::model::request::FilterRequest;
    ///
    /// let request = FilterRequest::new();
    /// assert!(request.validate().is_err()); // No fields set
    ///
    /// let request = FilterRequest::builder()
    ///     .query("IBM")
    ///     .build()
    ///     .unwrap();
    /// assert!(request.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<()> {
        // Validate the `RequestFilters` fields
        self.filters.validate()?;

        // Ensure at least one field is set
        if self.query.is_none() && self.filters.is_empty() {
            return Err(OpenFIGIError::other_error(
                OtherErrorKind::Validation,
                "At least one field must be set in FilterRequest",
            ));
        }
        Ok(())
    }
}

/// Builder for constructing [`FilterRequest`] instances.
///
/// Provides a fluent API for setting filter parameters and building validated requests.
/// All methods return `self` to enable method chaining.
///
/// # Examples
///
/// ```rust
/// use openfigi_rs::model::request::FilterRequestBuilder;
/// use openfigi_rs::model::enums::{Currency, ExchCode};
///
/// let request = FilterRequestBuilder::new()
///     .query("technology")
///     .currency(Currency::USD)
///     .exch_code(ExchCode::US)
///     .build()
///     .unwrap();
/// ```
#[derive(Default)]
pub struct FilterRequestBuilder {
    query: Option<String>,
    start: Option<String>,
    filters: RequestFilters,
}

impl FilterRequestBuilder {
    /// Creates a new `FilterRequestBuilder` with all fields unset.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::model::request::FilterRequestBuilder;
    ///
    /// let builder = FilterRequestBuilder::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets search keywords for the filter request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::model::request::FilterRequestBuilder;
    ///
    /// let builder = FilterRequestBuilder::new().query("AAPL");
    /// ```
    #[must_use]
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    /// Sets the pagination start token.
    ///
    /// Used for retrieving subsequent pages of results when the response
    /// contains a `next` field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::model::request::FilterRequestBuilder;
    ///
    /// let builder = FilterRequestBuilder::new()
    ///     .query("tech")
    ///     .start("next_page_token");
    /// ```
    #[must_use]
    pub fn start(mut self, start: impl Into<String>) -> Self {
        self.start = Some(start.into());
        self
    }

    /// Sets the `exch_code` for the desired instrument.
    #[must_use]
    pub fn exch_code(mut self, exch_code: ExchCode) -> Self {
        self.filters.exch_code = Some(exch_code);
        self
    }

    /// Sets the `mic_code` for the desired instrument.
    #[must_use]
    pub fn mic_code(mut self, mic_code: MicCode) -> Self {
        self.filters.mic_code = Some(mic_code);
        self
    }

    /// Sets the `currency` for the desired instrument.
    #[must_use]
    pub fn currency(mut self, currency: Currency) -> Self {
        self.filters.currency = Some(currency);
        self
    }

    /// Sets the `market_sec_des` for the desired instrument.
    #[must_use]
    pub fn market_sec_des(mut self, market_sec_des: MarketSecDesc) -> Self {
        self.filters.market_sec_des = Some(market_sec_des);
        self
    }

    /// Sets the `security_type` for the desired instrument.
    #[must_use]
    pub fn security_type(mut self, security_type: SecurityType) -> Self {
        self.filters.security_type = Some(security_type);
        self
    }

    /// Sets the `security_type2` for the desired instrument.
    #[must_use]
    pub fn security_type2(mut self, security_type2: SecurityType2) -> Self {
        self.filters.security_type2 = Some(security_type2);
        self
    }

    /// Sets whether to include unlisted equities in the filter.
    #[must_use]
    pub fn include_unlisted_equities(mut self, val: bool) -> Self {
        self.filters.include_unlisted_equities = Some(val);
        self
    }

    /// Sets the `option_type` for the desired instrument.
    #[must_use]
    pub fn option_type(mut self, option_type: OptionType) -> Self {
        self.filters.option_type = Some(option_type);
        self
    }

    /// Sets the `strike` price range for the desired instrument.
    #[must_use]
    pub fn strike(mut self, strike: [Option<f64>; 2]) -> Self {
        self.filters.strike = Some(strike);
        self
    }

    /// Sets the `contract_size` range for the desired instrument.
    #[must_use]
    pub fn contract_size(mut self, contract_size: [Option<f64>; 2]) -> Self {
        self.filters.contract_size = Some(contract_size);
        self
    }

    /// Sets the `coupon` range for the desired instrument.
    #[must_use]
    pub fn coupon(mut self, coupon: [Option<f64>; 2]) -> Self {
        self.filters.coupon = Some(coupon);
        self
    }

    /// Sets the `expiration` date range for the desired instrument.
    #[must_use]
    pub fn expiration(mut self, expiration: [Option<NaiveDate>; 2]) -> Self {
        self.filters.expiration = Some(expiration);
        self
    }

    /// Sets the `maturity` date range for the desired instrument.
    #[must_use]
    pub fn maturity(mut self, maturity: [Option<NaiveDate>; 2]) -> Self {
        self.filters.maturity = Some(maturity);
        self
    }

    /// Sets the `state_code` for the desired instrument.
    #[must_use]
    pub fn state_code(mut self, state_code: StateCode) -> Self {
        self.filters.state_code = Some(state_code);
        self
    }

    /// Builds and validates the `FilterRequest`.
    ///
    /// Constructs the final request object and performs validation to ensure
    /// all requirements are met.
    ///
    /// # Errors
    ///
    /// Returns [`OpenFIGIError`] if validation fails, such as:
    /// - No query or filter parameters specified
    /// - Mutually exclusive parameters set
    /// - Invalid parameter ranges
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::model::request::FilterRequestBuilder;
    ///
    /// let request = FilterRequestBuilder::new()
    ///     .query("IBM")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<FilterRequest> {
        let request = FilterRequest {
            query: self.query,
            start: self.start,
            filters: self.filters,
        };
        request.validate()?;
        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::enums::{Currency, ExchCode, MicCode, SecurityType2};
    use chrono::NaiveDate;

    #[test]
    fn test_filter_request_new_minimal() {
        let request = FilterRequest::new();
        assert!(request.query.is_none());
        assert!(request.start.is_none());
        assert!(request.filters.exch_code.is_none());
        assert!(request.filters.mic_code.is_none());
    }

    #[test]
    fn test_filter_request_builder_minimal() {
        let request = FilterRequest::builder().query("ibm").build().unwrap();
        assert_eq!(request.query, Some("ibm".into()));
    }

    #[test]
    fn test_filter_request_builder_with_currency() {
        let request = FilterRequest::builder()
            .query("ibm")
            .currency(Currency::USD)
            .build()
            .unwrap();
        assert_eq!(request.filters.currency, Some(Currency::USD));
    }

    #[test]
    fn test_filter_request_validate_exch_and_mic_code_conflict() {
        let mut request = FilterRequest::new();
        request.filters.exch_code = Some(ExchCode::A0);
        request.filters.mic_code = Some(MicCode::XCME);
        let result = request.validate();
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("Cannot set both exchCode and micCode"));
    }

    #[test]
    fn test_filter_request_validate_strike_range() {
        let mut request = FilterRequest::new();
        request.filters.strike = Some([Some(10.0), Some(5.0)]);
        let result = request.validate();
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("strike: start value cannot be greater than end value"));
    }

    #[test]
    fn test_filter_request_validate_expiration_required_for_option() {
        let mut request = FilterRequest::new();
        request.filters.security_type2 = Some(SecurityType2::Option);
        request.filters.expiration = None;
        let result = request.validate();
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("expiration is required for Option or Warrant security types"));
    }

    #[test]
    fn test_filter_request_validate_maturity_required_for_pool() {
        let mut request = FilterRequest::new();
        request.filters.security_type2 = Some(SecurityType2::Pool);
        let result = request.validate();
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("maturity is required for Pool security types"));
    }

    #[test]
    fn test_filter_request_validate_date_range_too_long() {
        let mut request = FilterRequest::new();
        let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2026, 2, 1).unwrap();
        request.filters.expiration = Some([Some(start), Some(end)]);
        let result = request.validate();
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("date range cannot exceed 1 year"));
    }

    #[test]
    fn test_serialize_deserialize_filter_request() {
        let request = FilterRequest::builder()
            .query("ibm")
            .currency(Currency::USD)
            .build()
            .unwrap();
        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: FilterRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }
}
