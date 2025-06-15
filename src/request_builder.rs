//! HTTP request builder for OpenFIGI API operations.
//!
//! This module provides [`OpenFIGIRequestBuilder`] for constructing and executing HTTP requests
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
/// to configure the request. Once configured, the request can be executed with either
/// [`send()`](Self::send) for raw responses or [`send_json()`](Self::send_json) for
/// automatic JSON deserialization.
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

    /// Sets the request body using fallible JSON serialization.
    ///
    /// This method attempts to serialize the provided data to JSON and returns an error
    /// if serialization fails. Use this when you need to handle serialization errors
    /// explicitly, especially with complex data structures that might fail to serialize.
    ///
    /// # Arguments
    ///
    /// * `body` - The data to serialize as the request body
    ///
    /// # Errors
    ///
    /// Returns [`OpenFIGIError::SerdeError`] if JSON serialization fails.
    pub(crate) fn try_body<T: Serialize>(mut self, body: &T) -> Result<Self> {
        self.body = Some(serde_json::to_value(body)?);
        Ok(self)
    }

    /// Sets the request body using JSON serialization.
    ///
    /// This is a convenience method that serializes the provided data to JSON.
    /// It's more ergonomic than [`try_body()`](Self::try_body) but will panic
    /// if serialization fails.
    ///
    /// # Arguments
    ///
    /// * `body` - The data to serialize as the request body
    ///
    /// # Panics
    ///
    /// Panics if JSON serialization fails. Use [`try_body()`](Self::try_body)
    /// for error handling instead of panicking.
    pub(crate) fn body<T: Serialize>(mut self, body: &T) -> Self {
        self.body = Some(serde_json::to_value(body).expect("Failed to serialize body"));
        self
    }

    /// Sets the request body from a pre-serialized JSON value.
    ///
    /// This method accepts a `serde_json::Value` directly, avoiding the serialization
    /// step. It's more efficient than [`body()`](Self::body) when you already have
    /// a JSON value, such as when building dynamic JSON structures.
    ///
    /// # Arguments
    ///
    /// * `body` - A pre-serialized JSON value to use as the request body
    pub(crate) fn json_body(mut self, body: serde_json::Value) -> Self {
        self.body = Some(body);
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
    /// 3. Adds JSON body if provided via [`body()`](Self::body) or [`json_body()`](Self::json_body)
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

    /// Executes the HTTP request and parses the response as JSON.
    ///
    /// This is a convenience method that combines [`send()`](Self::send) with automatic
    /// JSON parsing and comprehensive error handling. It handles HTTP status codes,
    /// provides detailed error context, and deserializes successful responses.
    ///
    /// # Process
    ///
    /// 1. Executes the HTTP request via [`send()`](Self::send)
    /// 2. Checks the HTTP status code and handles errors
    /// 3. Parses response body as JSON into the specified type
    /// 4. Provides detailed error context for failures
    ///
    /// # Type Parameter
    ///
    /// * `T` - The type to deserialize the JSON response into. Must implement `DeserializeOwned`.
    ///
    /// # Errors
    ///
    /// Returns errors for:
    /// - All errors from [`send()`](Self::send) (network, URL construction, etc.)
    /// - HTTP status errors (4xx, 5xx) with OpenFIGI-specific context
    /// - JSON parsing failures when response body is malformed
    /// - Response body reading errors
    pub(crate) async fn send_json<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        let client = self.client.clone();
        let response = self.send().await?;
        client.parse_response(response).await
    }

    /// Returns a reference to the underlying OpenFIGI client.
    ///
    /// Provides access to the client instance used for configuration and execution.
    /// Useful for accessing client configuration or for advanced usage scenarios.
    pub(crate) fn client(&self) -> &OpenFIGIClient {
        &self.client
    }

    /// Returns the HTTP method configured for this request.
    #[must_use]
    pub(crate) fn method(&self) -> &Method {
        &self.method
    }

    /// Returns the API endpoint path for this request.
    ///
    /// This is the path relative to the base URL that will be used for the request.
    #[must_use]
    pub(crate) fn path(&self) -> &str {
        &self.path
    }

    /// Returns `true` if a request body has been configured.
    ///
    /// This method can be used to check whether a body was set via [`body()`](Self::body),
    /// [`try_body()`](Self::try_body), or [`json_body()`](Self::json_body).
    pub(crate) fn has_body(&self) -> bool {
        self.body.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::OpenFIGIClient;
    use reqwest::Method;
    use serde_json::json;

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
    fn test_request_builder_try_body_success() {
        let client = create_test_client();
        let test_data = json!({"key": "value"});

        let result = OpenFIGIRequestBuilder::new(client, Method::POST, "test").try_body(&test_data);

        assert!(result.is_ok());
        assert!(result.unwrap().has_body());
    }

    #[test]
    fn test_request_builder_json_body() {
        let client = create_test_client();
        let test_data = json!({"key": "value"});

        let builder =
            OpenFIGIRequestBuilder::new(client, Method::POST, "test").json_body(test_data);

        assert!(builder.has_body());
    }

    #[test]
    fn test_request_builder_getters() {
        let client = create_test_client();
        let builder = OpenFIGIRequestBuilder::new(client.clone(), Method::PUT, "api/test");

        assert_eq!(builder.method(), &Method::PUT);
        assert_eq!(builder.path(), "api/test");
        assert_eq!(builder.client().base_url(), client.base_url());
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
