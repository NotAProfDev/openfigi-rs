#![doc(hidden)]
//! HTTP request builder for OpenFIGI API operations.
//!
//! This module provides [`crate::request_builder::OpenFIGIRequestBuilder`] for constructing and executing HTTP requests
//! with a fluent interface. The builder handles authentication, JSON serialization, and error
//! processing automatically while maintaining performance through minimal allocations.
//!
//! ## Key Features
//!
//! - **Fluent API**: Chainable method calls for readable request construction
//! - **Automatic authentication**: Adds API key headers when available
//! - **JSON handling**: Automatic serialization and deserialization with error handling
//! - **Error integration**: Unified error handling with detailed context
//! - **Performance optimized**: Minimal allocations and efficient execution
//!
//! Note: This module is not intended for direct use by consumers of the OpenFIGI API.

use crate::client::OpenFIGIClient;
use crate::error::{OpenFIGIError, Result};
use reqwest::Method;
use serde::Serialize;

/// HTTP request builder with fluent interface for OpenFIGI API operations.
///
/// This builder provides a chainable interface for constructing HTTP requests with automatic
/// authentication, JSON serialization, and integrated error handling. It's designed to be
/// both ergonomic and performant, minimizing allocations while providing comprehensive
/// request configuration options.
///
/// ## Design Philosophy
///
/// The builder follows a fluent interface pattern where methods can be chained together
/// to configure the request. Once configured, the request can be executed with
/// [`send()`](Self::send) for raw responses.
///
/// ## Authentication
///
/// The builder automatically adds API key authentication headers when the client
/// has an API key configured. No additional configuration is required.
///
/// ## Performance
///
/// - Uses efficient method chaining to minimize intermediate allocations
/// - Reuses the underlying HTTP client connection pool
/// - Optimizes header construction for common cases
pub(crate) struct OpenFIGIRequestBuilder {
    client: OpenFIGIClient,
    method: Method,
    path: String,
    body: Option<serde_json::Value>,
}

impl OpenFIGIRequestBuilder {
    /// Creates a new request builder for the specified endpoint.
    ///
    /// Initializes the builder with the provided client, HTTP method, and API endpoint path.
    /// The path will be joined with the client's base URL when the request is executed.
    ///
    /// # Arguments
    ///
    /// * `client` - The OpenFIGI client instance containing configuration and HTTP client
    /// * `method` - HTTP method for the request (GET, POST, PUT, DELETE, etc.)
    /// * `path` - API endpoint path relative to the base URL (e.g., "mapping", "search")
    pub(crate) fn new(client: OpenFIGIClient, method: Method, path: impl Into<String>) -> Self {
        Self {
            client,
            method,
            path: path.into(),
            body: None,
        }
    }

    /// Sets the request body using JSON serialization.
    ///
    /// This is a convenience method that serializes the provided data to JSON.
    ///
    /// # Arguments
    ///
    /// * `body` - The data to serialize as the request body
    ///
    /// # Panics
    ///
    /// Panics if JSON serialization fails.
    pub(crate) fn body<T: Serialize>(mut self, body: &T) -> Self {
        self.body = Some(serde_json::to_value(body).expect("Failed to serialize body"));
        self
    }

    /// Executes the HTTP request and returns the raw response.
    ///
    /// This method constructs the full URL by joining the path with the client's base URL,
    /// adds authentication headers if an API key is available, and executes the HTTP request.
    /// The raw response is returned without any processing, allowing full control over
    /// response handling.
    ///
    /// # Process
    ///
    /// 1. Constructs the full URL from base URL and path
    /// 2. Builds the HTTP request with the specified method
    /// 3. Adds JSON body if provided via [`body()`](Self::body)
    /// 4. Adds `X-OPENFIGI-APIKEY` header if API key is configured
    /// 5. Executes the request and returns the response
    ///
    /// # Errors
    ///
    /// Returns errors for:
    /// - URL construction failures (malformed base URL or path)
    /// - Network connectivity issues
    /// - HTTP errors (will not automatically handle status codes)
    /// - Request building failures
    pub(crate) async fn send(self) -> Result<reqwest::Response> {
        // Construct the full URL - this is fallible
        let url = self
            .client
            .base_url()
            .join(&self.path)
            .map_err(OpenFIGIError::from)?;

        // Build the request with optimal method chaining
        let mut request_builder = self.client.client().request(self.method, url);

        // Add JSON body if provided (most efficient path)
        if let Some(body) = self.body {
            request_builder = request_builder.json(&body);
        }

        // Add API key header if available (check once, use efficiently)
        if let Some(api_key) = self.client.api_key() {
            // Use static string for header name to avoid allocation
            request_builder = request_builder.header("X-OPENFIGI-APIKEY", api_key);
        }

        // Execute the request with proper error conversion
        request_builder.send().await.map_err(OpenFIGIError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::OpenFIGIClient;
    use reqwest::Method;
    use serde_json::json;

    impl OpenFIGIRequestBuilder {
        /// Returns the HTTP method configured for this request.
        fn method(&self) -> &Method {
            &self.method
        }

        /// Returns the API endpoint path for this request.
        ///
        /// This is the path relative to the base URL that will be used for the request.
        fn path(&self) -> &str {
            &self.path
        }

        /// Returns `true` if a request body has been configured.
        ///
        /// This method can be used to check whether a body was set.
        fn has_body(&self) -> bool {
            self.body.is_some()
        }
    }

    fn create_test_client() -> OpenFIGIClient {
        OpenFIGIClient::new()
    }

    #[test]
    fn test_request_builder_creation() {
        let client = create_test_client();
        let builder = OpenFIGIRequestBuilder::new(client, Method::GET, "test");

        assert_eq!(builder.method(), &Method::GET);
        assert_eq!(builder.path(), "test");
        assert!(!builder.has_body());
    }

    #[test]
    fn test_request_builder_body() {
        let client = create_test_client();
        let test_data = json!({"key": "value"});

        let builder = OpenFIGIRequestBuilder::new(client, Method::POST, "test").body(&test_data);

        assert!(builder.has_body());
    }

    #[test]
    fn test_request_builder_getters() {
        let client = create_test_client();
        let builder = OpenFIGIRequestBuilder::new(client.clone(), Method::PUT, "api/test");

        assert_eq!(builder.method(), &Method::PUT);
        assert_eq!(builder.path(), "api/test");
    }

    #[test]
    fn test_request_builder_chaining() {
        let client = create_test_client();
        let test_data = json!({"test": true});

        let builder = OpenFIGIRequestBuilder::new(client, Method::POST, "test").body(&test_data);

        assert_eq!(builder.method(), &Method::POST);
        assert_eq!(builder.path(), "test");
        assert!(builder.has_body());
    }
}
