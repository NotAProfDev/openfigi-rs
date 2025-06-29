//! # OpenFIGI Search Endpoint
//!
//! Request builders for interacting with the [/search](https://www.openfigi.com/api/documentation#v3-post-search) endpoint of the OpenFIGI API.
//! Provides both single and bulk search functionality with fluent builder patterns.
//!
//! ## Key Features
//!
//! - **Single Search**: Build and send individual search requests
//! - **Fluent API**: Chainable method calls for easy configuration
//! - **Validation**: Automatic validation of request limits and API key requirements
//!
//! ## Examples
//!
//! ### Single Search Request
//!
//! ```rust
//! use openfigi_rs::client::OpenFIGIClient;
//! use openfigi_rs::model::enums::{Currency, ExchCode};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = OpenFIGIClient::new();
//!
//! let result = client
//!     .search("ibm")
//!     .currency(Currency::USD)
//!     .exch_code(ExchCode::US)
//!     .send()
//!     .await?;
//! # Ok(())
//! # }
//! ```

use crate::{
    DEFAULT_ENDPOINT_SEARCH,
    client::OpenFIGIClient,
    error::Result,
    impl_filterable_builder,
    model::{
        enums::{
            Currency, ExchCode, MarketSecDesc, MicCode, OptionType, SecurityType, SecurityType2,
            StateCode,
        },
        request::SearchRequestBuilder,
        response::SearchData,
    },
};
use chrono::NaiveDate;
use reqwest::Method;

/// Builder for constructing single search requests to the `/search` endpoint.
///
/// Provides a fluent API for configuring search request parameters and executing requests.
/// Created via [`OpenFIGIClient::search`] with required query parameter.
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
///     .search("ibm")
///     .currency(Currency::USD)
///     .send()
///     .await?;
/// # Ok(())
/// # }
/// ```
pub struct SingleSearchRequestBuilder {
    client: OpenFIGIClient,
    request_builder: SearchRequestBuilder,
}

impl SingleSearchRequestBuilder {
    /// Sets the required query for the search request.
    #[must_use]
    pub fn query(mut self, query: &str) -> Self {
        self.request_builder = self.request_builder.query(query);
        self
    }

    /// Sets the optional pagination start value for the search request.
    #[must_use]
    pub fn start(mut self, start: &str) -> Self {
        self.request_builder = self.request_builder.start(start);
        self
    }

    // Bring in common builder methods for filtering logic
    impl_filterable_builder!();

    /// Sends the search request to `/search` endpoint and returns the raw HTTP response.
    ///
    /// This is useful when you need access to headers, status codes, or want to handle
    /// the response parsing yourself.
    ///
    /// # Errors
    ///
    /// Returns an [`crate::error::OpenFIGIError`] if the search request is invalid or if the HTTP request fails.
    pub async fn send_raw(self) -> Result<reqwest::Response> {
        let request = self.request_builder.build()?;
        self.client
            .request(DEFAULT_ENDPOINT_SEARCH, Method::POST)
            .body(&request)
            .send()
            .await
    }

    /// Sends the search request to `/search` endpoint and returns parsed results.
    ///
    /// # Errors
    ///
    /// Returns an [`crate::error::OpenFIGIError`] if the search request is invalid, if the HTTP request fails,
    /// or if the response cannot be parsed.
    pub async fn send(self) -> Result<SearchData> {
        let client = self.client.clone();
        let raw_response = self.send_raw().await?;

        client.parse_single_response(raw_response).await
    }
}

impl OpenFIGIClient {
    /// Creates a new [`SingleSearchRequestBuilder`] for configuring and executing a single search request.
    ///
    /// # Arguments
    ///
    /// * `query` - The search query string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::client::OpenFIGIClient;
    ///
    /// let client = OpenFIGIClient::new();
    /// let builder = client.search("ibm");
    /// ```
    #[must_use]
    pub fn search(&self, query: &str) -> SingleSearchRequestBuilder {
        SingleSearchRequestBuilder {
            client: self.clone(),
            request_builder: SearchRequestBuilder::new().query(query),
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
    fn test_single_search_request_builder_creation() {
        let client = create_test_client();
        let builder = client.search("ibm");

        // Builder should be created successfully with correct client reference
        assert_eq!(builder.client.base_url(), client.base_url());
        assert_eq!(builder.client.has_api_key(), client.has_api_key());

        // Test that we can build a valid search request from the builder
        let request_result = builder.request_builder.build();
        assert!(
            request_result.is_ok(),
            "Builder should create a valid search request"
        );

        let request = request_result.unwrap();
        assert_eq!(request.query, "ibm");
    }

    #[test]
    fn test_single_search_request_builder_chaining() {
        let client = create_test_client();
        let builder = client
            .search("ibm")
            .exch_code(ExchCode::US)
            .currency(Currency::USD)
            .market_sec_des(MarketSecDesc::Equity)
            .security_type(SecurityType::CommonStock)
            .include_unlisted_equities(true);

        // Verify that all chained parameters were properly set
        let request = builder
            .request_builder
            .build()
            .expect("Should build valid search request");

        // Check all the chained values are correctly set
        assert_eq!(request.query, "ibm");
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
    fn test_single_search_request_builder_option_fields() {
        let client = create_test_client();
        let builder = client
            .search("AAPL")
            .option_type(OptionType::Call)
            .strike([Some(150.0), Some(200.0)])
            .contract_size([Some(100.0), None])
            .coupon([None, Some(5.0)]);

        // Verify that option-specific fields are properly set
        let request = builder
            .request_builder
            .build()
            .expect("Should build valid search request");

        assert_eq!(request.query, "AAPL");
        assert_eq!(request.filters.option_type, Some(OptionType::Call));
        assert_eq!(request.filters.strike, Some([Some(150.0), Some(200.0)]));
        assert_eq!(request.filters.contract_size, Some([Some(100.0), None]));
        assert_eq!(request.filters.coupon, Some([None, Some(5.0)]));

        // Verify client reference is preserved
        assert_eq!(builder.client.base_url(), client.base_url());
    }

    #[test]
    fn test_single_search_request_builder_date_fields() {
        let client = create_test_client();
        let expiration_start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let expiration_end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let maturity_start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();

        let builder = client
            .search("ibm")
            .expiration([Some(expiration_start), Some(expiration_end)])
            .maturity([Some(maturity_start), None])
            .state_code(StateCode::CA);

        // Verify that date and state fields are properly set
        let request = builder
            .request_builder
            .build()
            .expect("Should build valid search request");

        assert_eq!(request.query, "ibm");
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
