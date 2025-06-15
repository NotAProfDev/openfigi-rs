//! # Search Request Types
//!
//! Request structures for the OpenFIGI `/search` endpoint (see [here](https://www.openfigi.com/api/documentation#v3-post-search) for more details).
//! Provides types for building search requests with fluent builder patterns.
//!
//! ## Examples
//!
//! ### Basic search request
//!
//! ```rust
//! use openfigi_rs::model::request::SearchRequest;
//!
//! let request = SearchRequest::new("AAPL");
//! ```
//!
//! ### Search request with additional filters
//!
//! ```rust
//! use openfigi_rs::model::request::SearchRequest;
//! use openfigi_rs::model::enums::{Currency, ExchCode};
//!
//! let request = SearchRequest::builder()
//!     .query("technology stocks")
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

/// Request structure for the OpenFIGI `/search` endpoint.
///
/// Represents a search request that finds FIGIs using keywords and filters.
/// Requires a search query with optional filtering criteria and pagination support.
///
/// # Required Fields
///
/// - `query`: Search keywords for finding FIGIs
///
/// # Optional Fields
///
/// - `start`: Pagination token for retrieving subsequent pages
/// - `filters`: Additional filtering criteria (flattened into the request JSON)
///
/// # Examples
///
/// ```rust
/// use openfigi_rs::model::request::SearchRequest;
/// use openfigi_rs::model::enums::Currency;
///
/// // Simple keyword search
/// let request = SearchRequest::new("IBM");
///
/// // Search with additional filters
/// let request = SearchRequest::builder()
///     .query("technology")
///     .currency(Currency::USD)
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    /// Search keywords for finding FIGIs.
    pub query: String,
    /// Pagination token for retrieving subsequent result pages.
    ///
    /// When more results are available, the response contains a `next` property
    /// whose value should be sent in succeeding requests as the `start` value
    /// to retrieve the next "page" of results.
    pub start: Option<String>,

    /// Additional filtering criteria applied to the mapping request.
    ///
    /// These filters are flattened into the JSON structure and provide
    /// optional constraints to refine the mapping results see `RequestFilters`.
    #[serde(flatten)]
    pub filters: RequestFilters,
}

impl SearchRequest {
    /// Creates a new `SearchRequest` with required search keywords.
    ///
    /// All filter fields are initialized to their default values (typically `None`).
    /// Use [`SearchRequest::builder()`] for a more convenient fluent API.
    ///
    /// # Arguments
    ///
    /// * `query` - Search keywords for FIGIs
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::model::request::SearchRequest;
    ///
    /// let request = SearchRequest::new("IBM");
    /// assert_eq!(request.query, "IBM");
    /// ```
    #[must_use]
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            start: None,
            filters: RequestFilters::default(),
        }
    }

    /// Creates a new `SearchRequestBuilder` for fluent request construction.
    ///
    /// Provides a convenient way to build search requests with method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::model::request::SearchRequest;
    /// use openfigi_rs::model::enums::Currency;
    ///
    /// let request = SearchRequest::builder()
    ///     .query("AAPL")
    ///     .currency(Currency::USD)
    ///     .build()
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn builder() -> SearchRequestBuilder {
        SearchRequestBuilder::new()
    }

    /// Validates the search request.
    ///
    /// Ensures that:
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
    /// use openfigi_rs::model::request::SearchRequest;
    ///
    /// let request = SearchRequest::new("IBM");
    /// assert!(request.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<()> {
        // Validate the `RequestFilters` fields
        self.filters.validate()?;
        Ok(())
    }
}

/// Builder for constructing [`SearchRequest`] instances.
///
/// Provides a fluent API for setting search keywords and filter parameters.
/// All methods return `self` to enable method chaining.
///
/// # Required Fields
///
/// - `query`: Must be set via [`query()`](Self::query)
///
/// # Examples
///
/// ```rust
/// use openfigi_rs::model::request::SearchRequestBuilder;
/// use openfigi_rs::model::enums::{Currency, ExchCode};
///
/// let request = SearchRequestBuilder::new()
///     .query("technology")
///     .currency(Currency::USD)
///     .exch_code(ExchCode::US)
///     .build()
///     .unwrap();
/// ```
#[derive(Default)]
pub struct SearchRequestBuilder {
    query: Option<String>,
    start: Option<String>,
    filters: RequestFilters,
}

impl SearchRequestBuilder {
    /// Creates a new `SearchRequestBuilder` with all fields unset.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::model::request::SearchRequestBuilder;
    ///
    /// let builder = SearchRequestBuilder::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets search keywords for the search request.
    ///
    /// This field is required and specifies the keywords to search for when
    /// finding FIGIs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::model::request::SearchRequestBuilder;
    ///
    /// let builder = SearchRequestBuilder::new().query("AAPL");
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
    /// use openfigi_rs::model::request::SearchRequestBuilder;
    ///
    /// let builder = SearchRequestBuilder::new()
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

    /// Sets whether to include unlisted equities in the search.
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

    /// Builds and validates the `SearchRequest`.
    ///
    /// Constructs the final request object and performs validation to ensure
    /// all requirements are met.
    ///
    /// # Errors
    ///
    /// Returns [`OpenFIGIError`] if validation fails, such as:
    /// - Missing required `query` field
    /// - Mutually exclusive parameters set
    /// - Invalid parameter ranges
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::model::request::SearchRequestBuilder;
    ///
    /// let request = SearchRequestBuilder::new()
    ///     .query("IBM")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<SearchRequest> {
        let query = self.query.ok_or_else(|| {
            OpenFIGIError::other_error(OtherErrorKind::Validation, "query is required")
        })?;
        let request = SearchRequest {
            query,
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
    fn test_search_request_new_minimal() {
        let request = SearchRequest::new("ibm");
        assert_eq!(request.query, "ibm");
        assert!(request.start.is_none());
        assert!(request.filters.exch_code.is_none());
        assert!(request.filters.mic_code.is_none());
    }

    #[test]
    fn test_search_request_builder_minimal() {
        let request = SearchRequest::builder().query("ibm").build().unwrap();
        assert_eq!(request.query, "ibm");
    }

    #[test]
    fn test_search_request_builder_with_currency() {
        let request = SearchRequest::builder()
            .query("ibm")
            .currency(Currency::USD)
            .build()
            .unwrap();
        assert_eq!(request.filters.currency, Some(Currency::USD));
    }

    #[test]
    fn test_search_request_validate_exch_and_mic_code_conflict() {
        let mut request = SearchRequest::new("ibm");
        request.filters.exch_code = Some(ExchCode::A0);
        request.filters.mic_code = Some(MicCode::XCME);
        let result = request.validate();
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("Cannot set both exchCode and micCode"));
    }

    #[test]
    fn test_search_request_validate_strike_range() {
        let mut request = SearchRequest::new("ibm");
        request.filters.strike = Some([Some(10.0), Some(5.0)]);
        let result = request.validate();
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("strike: start value cannot be greater than end value"));
    }

    #[test]
    fn test_search_request_validate_expiration_required_for_option() {
        let mut request = SearchRequest::new("ibm");
        request.filters.security_type2 = Some(SecurityType2::Option);
        request.filters.expiration = None;
        let result = request.validate();
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("expiration is required for Option or Warrant security types"));
    }

    #[test]
    fn test_search_request_validate_maturity_required_for_pool() {
        let mut request = SearchRequest::new("ibm");
        request.filters.security_type2 = Some(SecurityType2::Pool);
        let result = request.validate();
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("maturity is required for Pool security types"));
    }

    #[test]
    fn test_search_request_validate_date_range_too_long() {
        let mut request = SearchRequest::new("ibm");
        let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2026, 2, 1).unwrap();
        request.filters.expiration = Some([Some(start), Some(end)]);
        let result = request.validate();
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("date range cannot exceed 1 year"));
    }

    #[test]
    fn test_serialize_deserialize_search_request() {
        let request = SearchRequest::builder()
            .query("ibm")
            .currency(Currency::USD)
            .build()
            .unwrap();
        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: SearchRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }
}
