//! # OpenFIGI Filter Endpoint
//!
//! Request builders for interacting with the [/filter](https://www.openfigi.com/api/documentation#v3-post-filter) endpoint of the OpenFIGI API.
//! Provides both single and bulk filter functionality with fluent builder patterns.
//!
//! ## Key Features
//!
//! - **Single Filter**: Build and send individual filter requests
//! - **Fluent API**: Chainable method calls for easy configuration
//! - **Validation**: Automatic validation of request limits and API key requirements
//!
//! ## Examples
//!
//! ### Single Filter Request
//!
//! ```rust
//! use openfigi_rs::client::OpenFIGIClient;
//! use openfigi_rs::model::enums::{Currency, ExchCode};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = OpenFIGIClient::new();
//!
//! let result = client
//!     .filter()
//!     .query("ibm")
//!     .currency(Currency::USD)
//!     .exch_code(ExchCode::US)
//!     .send()
//!     .await?;
//! # Ok(())
//! # }
//! ```

use crate::{
    DEFAULT_ENDPOINT_FILTER,
    client::OpenFIGIClient,
    error::Result,
    model::{
        enums::{
            Currency, ExchCode, MarketSecDesc, MicCode, OptionType, SecurityType, SecurityType2,
            StateCode,
        },
        request::FilterRequestBuilder,
        response::FilterData,
    },
};
use chrono::NaiveDate;
use reqwest::Method;

/// Builder for constructing single filter requests to the `/filter` endpoint.
///
/// Provides a fluent API for configuring filter request parameters and executing requests.
/// Created via [`crate::client::OpenFIGIClient::filter`] with required query parameter.
///
/// # Examples
///
/// ```rust
/// use openfigi_rs::client::OpenFIGIClient;
/// use openfigi_rs::model::enums::Currency;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = OpenFIGIClient::new();
///
/// let response = client
///     .filter()
///     .query("ibm")
///     .currency(Currency::USD)
///     .send()
///     .await?;
/// # Ok(())
/// # }
/// ```
pub struct SingleFilterRequestBuilder {
    client: OpenFIGIClient,
    request_builder: FilterRequestBuilder,
}

impl SingleFilterRequestBuilder {
    /// Sets the required query for the filter request.
    #[must_use]
    pub fn query(mut self, query: &str) -> Self {
        self.request_builder = self.request_builder.query(query);
        self
    }

    /// Sets the optional pagination start value for the filter request.
    #[must_use]
    pub fn start(mut self, start: &str) -> Self {
        self.request_builder = self.request_builder.start(start);
        self
    }

    /// Sets the optional exchange code for the filter request.
    #[must_use]
    pub fn exch_code(mut self, exch_code: ExchCode) -> Self {
        self.request_builder = self.request_builder.exch_code(exch_code);
        self
    }

    /// Sets the optional MIC code for the filter request.
    #[must_use]
    pub fn mic_code(mut self, mic_code: MicCode) -> Self {
        self.request_builder = self.request_builder.mic_code(mic_code);
        self
    }

    /// Sets the optional currency for the filter request.
    #[must_use]
    pub fn currency(mut self, currency: Currency) -> Self {
        self.request_builder = self.request_builder.currency(currency);
        self
    }

    /// Sets the optional market sector description for the filter request.
    ///
    /// Specifies the market sector or asset class for the financial instrument.
    #[must_use]
    pub fn market_sec_des(mut self, market_sec_des: MarketSecDesc) -> Self {
        self.request_builder = self.request_builder.market_sec_des(market_sec_des);
        self
    }

    /// Sets the optional security type for the filter request.
    ///
    /// Specifies the primary security type classification for the instrument.
    #[must_use]
    pub fn security_type(mut self, security_type: SecurityType) -> Self {
        self.request_builder = self.request_builder.security_type(security_type);
        self
    }

    /// Sets the optional secondary security type for the filter request.
    #[must_use]
    pub fn security_type2(mut self, security_type2: SecurityType2) -> Self {
        self.request_builder = self.request_builder.security_type2(security_type2);
        self
    }

    /// Sets the optional flag to include unlisted equities for the filter request.
    #[must_use]
    pub fn include_unlisted_equities(mut self, val: bool) -> Self {
        self.request_builder = self.request_builder.include_unlisted_equities(val);
        self
    }

    /// Sets the optional option type for the filter request.
    #[must_use]
    pub fn option_type(mut self, option_type: OptionType) -> Self {
        self.request_builder = self.request_builder.option_type(option_type);
        self
    }

    /// Sets the optional strike price range for the filter request.
    #[must_use]
    pub fn strike(mut self, strike: [Option<f64>; 2]) -> Self {
        self.request_builder = self.request_builder.strike(strike);
        self
    }

    /// Sets the optional contract size range for the filter request.
    #[must_use]
    pub fn contract_size(mut self, contract_size: [Option<f64>; 2]) -> Self {
        self.request_builder = self.request_builder.contract_size(contract_size);
        self
    }

    /// Sets the optional coupon range for the filter request.
    #[must_use]
    pub fn coupon(mut self, coupon: [Option<f64>; 2]) -> Self {
        self.request_builder = self.request_builder.coupon(coupon);
        self
    }

    /// Sets the optional expiration date range for the filter request.
    #[must_use]
    pub fn expiration(mut self, expiration: [Option<NaiveDate>; 2]) -> Self {
        self.request_builder = self.request_builder.expiration(expiration);
        self
    }

    /// Sets the optional maturity date range for the filter request.
    #[must_use]
    pub fn maturity(mut self, maturity: [Option<NaiveDate>; 2]) -> Self {
        self.request_builder = self.request_builder.maturity(maturity);
        self
    }

    /// Sets the optional state code for the filter request.
    #[must_use]
    pub fn state_code(mut self, state_code: StateCode) -> Self {
        self.request_builder = self.request_builder.state_code(state_code);
        self
    }

    /// Sends the filter request to `/filter` endpoint and returns the raw HTTP response.
    ///
    /// This is useful when you need access to headers, status codes, or want to handle
    /// the response parsing yourself.
    ///
    /// # Errors
    ///
    /// Returns an [`crate::error::OpenFIGIError`] if the filter request is invalid or if the HTTP request fails.
    pub async fn send_raw(self) -> Result<reqwest::Response> {
        let request = self.request_builder.build()?;
        self.client
            .request(DEFAULT_ENDPOINT_FILTER, Method::POST)
            .body(&request)
            .send()
            .await
    }

    /// Sends the filter request to `/filter` endpoint and returns parsed results.
    ///
    /// # Errors
    ///
    /// Returns an [`crate::error::OpenFIGIError`] if the filter request is invalid, if the HTTP request fails,
    /// or if the response cannot be parsed.
    pub async fn send(self) -> Result<FilterData> {
        let client = self.client.clone();
        let raw_response = self.send_raw().await?;

        client.parse_single_response(raw_response).await
    }
}

impl OpenFIGIClient {
    /// Creates a new [`SingleFilterRequestBuilder`] for configuring and executing a single filter request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::client::OpenFIGIClient;
    ///
    /// let client = OpenFIGIClient::new();
    /// let builder = client.filter().query("ibm");
    /// ```
    #[must_use]
    pub fn filter(&self) -> SingleFilterRequestBuilder {
        SingleFilterRequestBuilder {
            client: self.clone(),
            request_builder: FilterRequestBuilder::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::OpenFIGIClient;

    fn create_test_client() -> OpenFIGIClient {
        OpenFIGIClient::new()
    }

    #[test]
    fn test_single_filter_request_builder_creation() {
        let client = create_test_client();
        let builder = client.filter().query("ibm");

        // Builder should be created successfully with correct client reference
        assert_eq!(builder.client.base_url(), client.base_url());
        assert_eq!(builder.client.has_api_key(), client.has_api_key());

        // Test that we can build a valid filter request from the builder
        let request_result = builder.request_builder.build();
        assert!(
            request_result.is_ok(),
            "Builder should create a valid filter request"
        );

        let request = request_result.unwrap();
        assert_eq!(request.query, Some("ibm".to_string()));
    }

    #[test]
    fn test_single_filter_request_builder_chaining() {
        let client = create_test_client();
        let builder = client
            .filter()
            .query("ibm")
            .exch_code(ExchCode::US)
            .currency(Currency::USD)
            .market_sec_des(MarketSecDesc::Equity)
            .security_type(SecurityType::CommonStock)
            .include_unlisted_equities(true);

        // Verify that all chained parameters were properly set
        let request = builder
            .request_builder
            .build()
            .expect("Should build valid filter request");

        // Check all the chained values are correctly set
        assert_eq!(request.query, Some("ibm".to_string()));
        assert_eq!(request.filters.exch_code, Some(ExchCode::US));
        assert_eq!(request.filters.currency, Some(Currency::USD));
        assert_eq!(request.filters.market_sec_des, Some(MarketSecDesc::Equity));
        assert_eq!(
            request.filters.security_type,
            Some(SecurityType::CommonStock)
        );
        assert_eq!(request.filters.include_unlisted_equities, Some(true));

        // Verify client reference is preserved
        assert_eq!(builder.client.base_url(), client.base_url());
    }

    #[test]
    fn test_single_filter_request_builder_option_fields() {
        let client = create_test_client();
        let builder = client
            .filter()
            .query("AAPL")
            .option_type(OptionType::Call)
            .strike([Some(150.0), Some(200.0)])
            .contract_size([Some(100.0), None])
            .coupon([None, Some(5.0)]);

        // Verify that option-specific fields are properly set
        let request = builder
            .request_builder
            .build()
            .expect("Should build valid filter request");

        assert_eq!(request.query, Some("AAPL".to_string()));
        assert_eq!(request.filters.option_type, Some(OptionType::Call));
        assert_eq!(request.filters.strike, Some([Some(150.0), Some(200.0)]));
        assert_eq!(request.filters.contract_size, Some([Some(100.0), None]));
        assert_eq!(request.filters.coupon, Some([None, Some(5.0)]));

        // Verify client reference is preserved
        assert_eq!(builder.client.base_url(), client.base_url());
    }

    #[test]
    fn test_single_filter_request_builder_date_fields() {
        let client = create_test_client();
        let expiration_start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let expiration_end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let maturity_start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();

        let builder = client
            .filter()
            .query("ibm")
            .expiration([Some(expiration_start), Some(expiration_end)])
            .maturity([Some(maturity_start), None])
            .state_code(StateCode::CA);

        // Verify that date and state fields are properly set
        let request = builder
            .request_builder
            .build()
            .expect("Should build valid filter request");

        assert_eq!(request.query, Some("ibm".to_string()));
        assert_eq!(
            request.filters.expiration,
            Some([Some(expiration_start), Some(expiration_end)])
        );
        assert_eq!(request.filters.maturity, Some([Some(maturity_start), None]));
        assert_eq!(request.filters.state_code, Some(StateCode::CA));

        // Verify client reference is preserved
        assert_eq!(builder.client.base_url(), client.base_url());
    }
}
