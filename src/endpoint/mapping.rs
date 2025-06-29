//! # OpenFIGI Mapping Endpoint
//!
//! Request builders for interacting with the [/mapping](https://www.openfigi.com/api/documentation#v3-post-mapping) endpoint of the OpenFIGI API.
//! Provides both single and bulk mapping functionality with fluent builder patterns.
//!
//! ## Key Features
//!
//! - **Single Mapping**: Build and send individual mapping requests
//! - **Bulk Mapping**: Batch multiple mapping requests in a single request
//! - **Fluent API**: Chainable method calls for easy configuration
//! - **Validation**: Automatic validation of request limits and API key requirements
//!
//! ## Examples
//!
//! ### Single Mapping Request
//!
//! ```rust
//! use openfigi_rs::client::OpenFIGIClient;
//! use openfigi_rs::model::enums::{IdType, Currency, ExchCode};
//! use serde_json::json;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = OpenFIGIClient::new();
//!
//! let result = client
//!     .mapping(IdType::IdIsin, json!("US4592001014"))
//!     .currency(Currency::USD)
//!     .exch_code(ExchCode::US)
//!     .send()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Bulk Mapping Request
//!
//! ```rust
//! use openfigi_rs::client::OpenFIGIClient;
//! use openfigi_rs::model::request::MappingRequest;
//! use openfigi_rs::model::enums::IdType;
//! use serde_json::json;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = OpenFIGIClient::new();
//!
//! let requests = vec![
//!     MappingRequest::builder()
//!         .id_type(IdType::IdIsin)
//!         .id_value(json!("US4592001014"))
//!         .build()?,
//!     MappingRequest::builder()
//!         .id_type(IdType::Ticker)
//!         .id_value(json!("AAPL"))
//!         .build()?,
//! ];
//!
//! let result = client
//!     .bulk_mapping()
//!     .add_requests(requests)
//!     .send()
//!     .await?;
//! # Ok(())
//! # }
//! ```

use crate::{
    DEFAULT_ENDPOINT_MAPPING,
    client::OpenFIGIClient,
    error::{OpenFIGIError, OtherErrorKind, Result},
    impl_filter_builder,
    model::{
        enums::{
            Currency, ExchCode, IdType, MarketSecDesc, MicCode, OptionType, SecurityType,
            SecurityType2, StateCode,
        },
        request::{MappingRequest, MappingRequestBuilder, RequestFilters},
        response::{MappingData, MappingResponses},
    },
};
use chrono::NaiveDate;
use reqwest::Method;

/// Builder for constructing single mapping requests to the `/mapping` endpoint.
///
/// Provides a fluent API for configuring mapping request parameters and executing requests.
/// Created via [`OpenFIGIClient::mapping`] with required ID type and value parameters.
///
/// # Examples
///
/// ```rust
/// use openfigi_rs::client::OpenFIGIClient;
/// use openfigi_rs::model::enums::{IdType, Currency};
/// use serde_json::json;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = OpenFIGIClient::new();
///
/// let response = client
///     .mapping(IdType::IdIsin, json!("US4592001014"))
///     .currency(Currency::USD)
///     .send()
///     .await?;
/// # Ok(())
/// # }
/// ```
pub struct SingleMappingRequestBuilder {
    client: OpenFIGIClient,
    request_builder: MappingRequestBuilder,
}

impl SingleMappingRequestBuilder {
    /// Sets the required ID type for the mapping request.
    #[must_use]
    pub fn id_type(mut self, id_type: IdType) -> Self {
        self.request_builder = self.request_builder.id_type(id_type);
        self
    }

    /// Sets the required ID value for the mapping request.
    #[must_use]
    pub fn id_value<T: Into<serde_json::Value>>(mut self, id_value: T) -> Self {
        self.request_builder = self.request_builder.id_value(id_value);
        self
    }

    /// Mutable access to the request filters, delegating to the inner `MappingRequestBuilder`.
    pub fn filters_mut(&mut self) -> &mut RequestFilters {
        self.request_builder.filters_mut()
    }

    // Bring in common builder methods for filtering logic
    impl_filter_builder!();

    /// Sends the mapping request to `/mapping` endpoint and returns the raw HTTP response.
    ///
    /// This is useful when you need access to headers, status codes, or want to handle
    /// the response parsing yourself.
    ///
    /// # Errors
    ///
    /// Returns an [`crate::error::OpenFIGIError`] if the mapping request is invalid or if the HTTP request fails.
    pub async fn send_raw(self) -> Result<reqwest::Response> {
        let request = self.request_builder.build()?;
        let requests = vec![request];
        self.client
            .request(DEFAULT_ENDPOINT_MAPPING, Method::POST)
            .body(&requests)
            .send()
            .await
    }

    /// Sends the mapping request to `/mapping` endpoint and returns parsed results.
    ///
    /// # Errors
    ///
    /// Returns an [`crate::error::OpenFIGIError`] if the mapping request is invalid, if the HTTP request fails,
    /// or if the response cannot be parsed.
    #[expect(clippy::missing_panics_doc)]
    pub async fn send(self) -> Result<MappingData> {
        let client = self.client.clone();
        let raw_response = self.send_raw().await?;

        let mut results = client.parse_list_response(raw_response).await?;

        // Take the first element, ensuring the iterator is consumed and the Vec is empty.
        if results.len() == 1 {
            // The unwrap is safe due to the length check.
            results.pop().unwrap()
        } else {
            Err(OpenFIGIError::response_error(
                // We know the status was OK, otherwise parse_list_response would have failed.
                reqwest::StatusCode::OK,
                format!(
                    "Expected 1 result for single mapping, but got {}",
                    results.len()
                ),
                String::new(),
            ))
        }
    }
}

/// Builder for bulk mapping requests to the `/mapping` endpoint.
///
/// Allows batching multiple mapping requests into a single API request for improved efficiency.
/// Automatically validates request count limits based on API key availability.
///
/// # Limits
///
/// - Without API key: Maximum 5 requests per request
/// - With API key: Maximum 100 requests per request
///
/// # Examples
///
/// ```rust
/// use openfigi_rs::client::OpenFIGIClient;
/// use openfigi_rs::model::request::MappingRequest;
/// use openfigi_rs::model::enums::IdType;
/// use serde_json::json;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = OpenFIGIClient::new();
/// let requests = vec![
///     MappingRequest::new(IdType::IdIsin, json!("US4592001014")),
///     MappingRequest::new(IdType::Ticker, json!("AAPL")),
/// ];
///
/// let result = client
///     .bulk_mapping()
///     .add_requests(requests)
///     .send()
///     .await?;
/// # Ok(())
/// # }
/// ```
pub struct BulkMappingRequestBuilder {
    client: OpenFIGIClient,
    requests: Vec<MappingRequest>,
}

impl BulkMappingRequestBuilder {
    /// Adds a single mapping request to the bulk request.
    #[must_use]
    pub fn add_request(mut self, request: MappingRequest) -> Self {
        self.requests.push(request);
        self
    }

    /// Adds multiple mapping requests to the bulk request.
    #[must_use]
    pub fn add_requests(mut self, requests: Vec<MappingRequest>) -> Self {
        self.requests.extend(requests);
        self
    }

    /// Adds a new, fully configured mapping job to the bulk request using a fluent builder.
    ///
    /// This method provides a closure that receives a `MappingRequestBuilder`,
    /// allowing you to configure a single mapping job with any required filters before
    /// it's added to the bulk request.
    ///
    /// # Errors
    ///
    /// Returns an `OpenFIGIError` if the configured job fails validation (e.g.,
    /// if `id_type` or `id_value` are missing).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use openfigi_rs::client::OpenFIGIClient;
    /// # use openfigi_rs::model::enums::{IdType, Currency, ExchCode};
    /// # use serde_json::json;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = OpenFIGIClient::new();
    /// let result = client
    ///     .bulk_mapping()
    ///     .job(|j| j.id_type(IdType::IdIsin).id_value("US4592001014"))? // Simple job
    ///     .job(|j| { // Complex job with filters
    ///         j.id_type(IdType::Ticker)
    ///             .id_value("IBM")
    ///             .currency(Currency::USD)
    ///             .exch_code(ExchCode::US)
    ///     })?
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn job<F>(mut self, config: F) -> Result<Self>
    where
        F: FnOnce(MappingRequestBuilder) -> MappingRequestBuilder,
    {
        let builder = MappingRequest::builder();
        let configured_builder = config(builder);

        // Build the request and propagate any errors using the `?` operator.
        let request = configured_builder.build()?;

        // If building succeeds, add the request to our list.
        self.requests.push(request);
        Ok(self)
    }

    /// Sends the bulk mapping request to `/mapping` endpoint and returns the raw HTTP response.
    ///
    /// This is useful when you need access to headers, status codes, or want to handle
    /// the response parsing yourself.
    ///
    /// # Errors
    ///
    /// Returns an [`crate::error::OpenFIGIError`] if the bulk mapping request is invalid or if the HTTP request fails.
    pub async fn send_raw(self) -> Result<reqwest::Response> {
        if self.requests.is_empty() {
            return Err(OpenFIGIError::other_error(
                OtherErrorKind::Validation,
                "No requests to send",
            ));
        } else if !self.client.has_api_key() && self.requests.len() > 5 {
            return Err(OpenFIGIError::other_error(
                OtherErrorKind::Validation,
                "Bulk mapping request cannot exceed 5 requests without an API key",
            ));
        } else if self.requests.len() > 100 {
            return Err(OpenFIGIError::other_error(
                OtherErrorKind::Validation,
                "Bulk mapping request cannot exceed 100 requests",
            ));
        }

        self.client
            .request(DEFAULT_ENDPOINT_MAPPING, Method::POST)
            .body(&self.requests)
            .send()
            .await
    }

    /// Sends the mapping request to `/mapping` endpoint and returns parsed results.
    ///
    /// # Errors
    ///
    /// Returns an [`crate::error::OpenFIGIError`] if the mapping request is invalid, if the HTTP request fails,
    /// or if the response cannot be parsed.
    pub async fn send(self) -> Result<MappingResponses> {
        let client = self.client.clone();
        let raw_response = self.send_raw().await?;

        let results = client.parse_list_response(raw_response).await?;

        Ok(MappingResponses::new(results))
    }
}

impl OpenFIGIClient {
    /// Creates a new [`SingleMappingRequestBuilder`] for configuring and executing a single mapping request.
    ///
    /// # Arguments
    ///
    /// * `id_type` - The type of identifier to map
    /// * `id_value` - The identifier value to map
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::client::OpenFIGIClient;
    /// use openfigi_rs::model::enums::IdType;
    ///
    /// let client = OpenFIGIClient::new();
    /// let builder = client.mapping(IdType::IdIsin, "US4592001014");
    /// ```
    #[must_use]
    pub fn mapping<T: Into<serde_json::Value>>(
        &self,
        id_type: IdType,
        id_value: T,
    ) -> SingleMappingRequestBuilder {
        SingleMappingRequestBuilder {
            client: self.clone(),
            request_builder: MappingRequestBuilder::new()
                .id_type(id_type)
                .id_value(id_value),
        }
    }

    /// Creates a new [`BulkMappingRequestBuilder`] for batching multiple mapping requests.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::client::OpenFIGIClient;
    ///
    /// let client = OpenFIGIClient::new();
    /// let builder = client.bulk_mapping();
    /// ```
    #[must_use]
    pub fn bulk_mapping(&self) -> BulkMappingRequestBuilder {
        BulkMappingRequestBuilder {
            client: self.clone(),
            requests: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::OpenFIGIClient;
    use serde_json::json;

    fn create_test_client() -> OpenFIGIClient {
        OpenFIGIClient::new()
    }

    fn create_test_client_with_api_key() -> OpenFIGIClient {
        OpenFIGIClient::builder()
            .api_key("test_key")
            .build()
            .expect("Failed to create test client")
    }

    #[test]
    fn test_single_mapping_request_builder_creation() {
        let client = create_test_client();
        let builder = client.mapping(IdType::IdIsin, json!("US4592001014"));

        // Builder should be created successfully with correct client reference
        assert_eq!(builder.client.base_url(), client.base_url());
        assert_eq!(builder.client.has_api_key(), client.has_api_key());

        // Test that we can build a valid mapping request from the builder
        let request_result = builder.request_builder.build();
        assert!(
            request_result.is_ok(),
            "Builder should create a valid mapping request"
        );

        let request = request_result.unwrap();
        assert_eq!(request.id_type, IdType::IdIsin);
        assert_eq!(request.id_value, json!("US4592001014"));
    }

    #[test]
    fn test_single_mapping_request_builder_chaining() {
        let client = create_test_client();
        let builder = client
            .mapping(IdType::IdIsin, json!("US4592001014"))
            .exch_code(ExchCode::US)
            .currency(Currency::USD)
            .market_sec_des(MarketSecDesc::Equity)
            .security_type(SecurityType::CommonStock)
            .include_unlisted_equities(true);

        // Verify that all chained parameters were properly set
        let request = builder
            .request_builder
            .build()
            .expect("Should build valid mapping request");

        // Check all the chained values are correctly set
        assert_eq!(request.id_type, IdType::IdIsin);
        assert_eq!(request.id_value, json!("US4592001014"));
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
    fn test_single_mapping_request_builder_option_fields() {
        let client = create_test_client();
        let builder = client
            .mapping(IdType::Ticker, json!("AAPL"))
            .option_type(OptionType::Call)
            .strike([Some(150.0), Some(200.0)])
            .contract_size([Some(100.0), None])
            .coupon([None, Some(5.0)]);

        // Verify that option-specific fields are properly set
        let request = builder
            .request_builder
            .build()
            .expect("Should build valid mapping request");

        assert_eq!(request.id_type, IdType::Ticker);
        assert_eq!(request.id_value, json!("AAPL"));
        assert_eq!(request.filters.option_type, Some(OptionType::Call));
        assert_eq!(request.filters.strike, Some([Some(150.0), Some(200.0)]));
        assert_eq!(request.filters.contract_size, Some([Some(100.0), None]));
        assert_eq!(request.filters.coupon, Some([None, Some(5.0)]));

        // Verify client reference is preserved
        assert_eq!(builder.client.base_url(), client.base_url());
    }

    #[test]
    fn test_single_mapping_request_builder_date_fields() {
        let client = create_test_client();
        let expiration_start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let expiration_end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let maturity_start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();

        let builder = client
            .mapping(IdType::IdCusip, json!("037833100"))
            .expiration([Some(expiration_start), Some(expiration_end)])
            .maturity([Some(maturity_start), None])
            .state_code(StateCode::CA);

        // Verify that date and state fields are properly set
        let request = builder
            .request_builder
            .build()
            .expect("Should build valid mapping request");

        assert_eq!(request.id_type, IdType::IdCusip);
        assert_eq!(request.id_value, json!("037833100"));
        assert_eq!(
            request.filters.expiration,
            Some([Some(expiration_start), Some(expiration_end)])
        );
        assert_eq!(request.filters.maturity, Some([Some(maturity_start), None]));
        assert_eq!(request.filters.state_code, Some(StateCode::CA));

        // Verify client reference is preserved
        assert_eq!(builder.client.base_url(), client.base_url());
    }

    #[test]
    fn test_bulk_mapping_request_builder_creation() {
        let client = create_test_client();
        let builder = client.bulk_mapping();

        // Builder should be created with empty requests
        assert_eq!(builder.requests.len(), 0);
        assert_eq!(builder.client.base_url(), client.base_url());
    }

    #[test]
    fn test_bulk_mapping_request_builder_add_request() {
        let client = create_test_client();
        let request = MappingRequest::new(IdType::IdIsin, json!("US4592001014"));

        let builder = client.bulk_mapping().add_request(request);

        // Verify that exactly one request was added
        assert_eq!(builder.requests.len(), 1);

        // Verify that the added request has the correct properties
        let added_request = &builder.requests[0];
        assert_eq!(added_request.id_type, IdType::IdIsin);
        assert_eq!(added_request.id_value, json!("US4592001014"));

        // Verify client reference is preserved
        assert_eq!(builder.client.base_url(), client.base_url());
        assert_eq!(builder.client.has_api_key(), client.has_api_key());
    }

    #[test]
    fn test_bulk_mapping_request_builder_add_requests() {
        let client = create_test_client();
        let requests = vec![
            MappingRequest::new(IdType::IdIsin, json!("US4592001014")),
            MappingRequest::new(IdType::IdIsin, json!("US0378331005")),
            MappingRequest::new(IdType::Ticker, json!("MSFT")),
        ];

        let builder = client.bulk_mapping().add_requests(requests);

        // Verify that exactly three requests were added
        assert_eq!(builder.requests.len(), 3);

        // Verify that the added requests have the correct properties
        assert_eq!(builder.requests[0].id_type, IdType::IdIsin);
        assert_eq!(builder.requests[0].id_value, json!("US4592001014"));

        assert_eq!(builder.requests[1].id_type, IdType::IdIsin);
        assert_eq!(builder.requests[1].id_value, json!("US0378331005"));

        assert_eq!(builder.requests[2].id_type, IdType::Ticker);
        assert_eq!(builder.requests[2].id_value, json!("MSFT"));

        // Verify client reference is preserved
        assert_eq!(builder.client.base_url(), client.base_url());
        assert_eq!(builder.client.has_api_key(), client.has_api_key());
    }

    #[test]
    fn test_bulk_mapping_request_builder_chaining() {
        let client = create_test_client();
        let request1 = MappingRequest::new(IdType::IdIsin, json!("US4592001014"));
        let request2 = MappingRequest::new(IdType::IdIsin, json!("US0378331005"));
        let additional_requests = vec![
            MappingRequest::new(IdType::Ticker, json!("MSFT")),
            MappingRequest::new(IdType::Ticker, json!("GOOGL")),
        ];

        let builder = client
            .bulk_mapping()
            .add_request(request1)
            .add_request(request2)
            .add_requests(additional_requests);

        // Verify that exactly four requests were added through chaining
        assert_eq!(builder.requests.len(), 4);

        // Verify that each request was added in the correct order with correct properties
        assert_eq!(builder.requests[0].id_type, IdType::IdIsin);
        assert_eq!(builder.requests[0].id_value, json!("US4592001014"));

        assert_eq!(builder.requests[1].id_type, IdType::IdIsin);
        assert_eq!(builder.requests[1].id_value, json!("US0378331005"));

        assert_eq!(builder.requests[2].id_type, IdType::Ticker);
        assert_eq!(builder.requests[2].id_value, json!("MSFT"));

        assert_eq!(builder.requests[3].id_type, IdType::Ticker);
        assert_eq!(builder.requests[3].id_value, json!("GOOGL"));

        // Verify client reference is preserved
        assert_eq!(builder.client.base_url(), client.base_url());
        assert_eq!(builder.client.has_api_key(), client.has_api_key());
    }

    #[tokio::test]
    async fn test_bulk_mapping_empty_requests_error() {
        let client = create_test_client();
        let builder = client.bulk_mapping();

        let result = builder.send().await;

        assert!(result.is_err());
        if let Err(OpenFIGIError::OtherError { kind, .. }) = result {
            assert_eq!(kind, OtherErrorKind::Validation);
        } else {
            panic!("Expected validation error for empty requests");
        }
    }

    #[tokio::test]
    async fn test_bulk_mapping_too_many_requests_without_api_key() {
        let client = create_test_client(); // No API key
        let requests = (0..6)
            .map(|i| MappingRequest::new(IdType::Ticker, json!(format!("TEST{}", i))))
            .collect();

        let builder = client.bulk_mapping().add_requests(requests);

        let result = builder.send_raw().await;

        assert!(result.is_err());
        if let Err(OpenFIGIError::OtherError { kind, .. }) = result {
            assert_eq!(kind, OtherErrorKind::Validation);
        } else {
            panic!("Expected validation error for too many requests without API key");
        }
    }

    #[tokio::test]
    async fn test_bulk_mapping_too_many_requests_with_api_key() {
        let client = create_test_client_with_api_key();
        let requests = (0..101)
            .map(|i| MappingRequest::new(IdType::Ticker, json!(format!("TEST{}", i))))
            .collect();

        let builder = client.bulk_mapping().add_requests(requests);

        let result = builder.send_raw().await;

        assert!(result.is_err());
        if let Err(OpenFIGIError::OtherError { kind, .. }) = result {
            assert_eq!(kind, OtherErrorKind::Validation);
        } else {
            panic!("Expected validation error for too many requests even with API key");
        }
    }
}
