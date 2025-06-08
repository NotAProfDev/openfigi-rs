//! # HTTP Request Builder
//!
//! Fluent interface for constructing and sending HTTP requests with automatic
//! authentication, JSON handling, and error processing.
//!
//! ## Key Features
//!
//! - **Fluent API**: Chain method calls for readable request construction
//! - **Auto authentication**: Adds API key headers when available
//! - **JSON handling**: Automatic serialization and deserialization
//! - **Error integration**: Unified error handling with context
//!
//! ## Examples
//!
//! ```rust
//! # use openfigi_rs::request_builder::OpenFIGIRequestBuilder;
//! # use openfigi_rs::client::OpenFIGIClient;
//! # use reqwest::Method;
//! # use serde_json::json;
//! #
//! let client = OpenFIGIClient::new();
//!
//! // POST with JSON body
//! let result: serde_json::Value = OpenFIGIRequestBuilder::new(client, Method::POST, "mapping")
//!     .body(&json!({"idType": "ID_ISIN", "idValue": "US4592001014"}))
//!     .send_json()
//!     .await?;
//! ```

use crate::client::OpenFIGIClient;
use crate::error::{OpenFIGIError, Result};
use reqwest::Method;
use serde::Serialize;

/// HTTP request builder with fluent interface.
///
/// Constructs requests with automatic authentication headers, JSON serialization,
/// and integrated error handling. Optimized for minimal allocations.
///
/// ## Memory Efficiency
///
/// Uses `Box<str>` for string storage to minimize heap allocations and improve
/// memory efficiency compared to `String`.
///
/// # Examples
///
/// ```rust
/// # use openfigi_rs::request_builder::OpenFIGIRequestBuilder;
/// # use openfigi_rs::client::OpenFIGIClient;
/// # use reqwest::Method;
/// # use serde_json::json;
///
/// let client = OpenFIGIClient::new();
/// let builder = OpenFIGIRequestBuilder::new(client, Method::POST, "mapping")
///     .body(&json!({"idType": "ID_ISIN", "idValue": "US4592001014"}));
/// ```
pub struct OpenFIGIRequestBuilder {
    client: OpenFIGIClient,
    method: Method,
    path: Box<str>,
    body: Option<serde_json::Value>,
}

impl OpenFIGIRequestBuilder {
    /// Creates a new request builder.
    ///
    /// # Arguments
    ///
    /// * `client` - Client instance for configuration and execution
    /// * `method` - HTTP method (GET, POST, PUT, DELETE, etc.)
    /// * `path` - API endpoint path relative to base URL
    #[must_use]
    pub fn new(client: OpenFIGIClient, method: Method, path: &str) -> Self {
        Self {
            client,
            method,
            path: path.into(),
            body: None,
        }
    }

    /// Sets request body with fallible JSON serialization.
    ///
    /// # Errors
    ///
    /// Returns error if serialization fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use serde_json::json;
    /// # // Placeholder for actual usage
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let builder = request_builder.try_body(&json!({"key": "value"}))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_body<T: Serialize>(mut self, body: &T) -> Result<Self> {
        self.body = Some(serde_json::to_value(body)?);
        Ok(self)
    }

    /// Sets request body with JSON serialization.
    ///
    /// # Panics
    ///
    /// Panics if serialization fails. Use [`Self::try_body`] for fallible version.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use serde_json::json;
    /// # // Placeholder for actual usage
    /// let builder = request_builder.body(&json!({"key": "value"}));
    /// ```
    #[must_use]
    pub fn body<T: Serialize>(mut self, body: &T) -> Self {
        self.body = Some(serde_json::to_value(body).expect("Failed to serialize body"));
        self
    }

    /// Sets request body from pre-serialized JSON.
    ///
    /// More efficient than [`Self::body`] when you already have a JSON value.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use serde_json::json;
    /// # // Placeholder for actual usage
    /// let json_value = json!({"key": "value"});
    /// let builder = request_builder.json_body(json_value);
    /// ```
    #[must_use]
    pub fn json_body(mut self, body: serde_json::Value) -> Self {
        self.body = Some(body);
        self
    }

    /// Sends the request and returns the raw response.
    ///
    /// Constructs the full URL, adds authentication headers, and executes the request.
    ///
    /// # Errors
    ///
    /// Returns errors for URL construction, network issues, or server errors.
    ///
    /// # Example
    ///
    /// ```rust
    /// # // Placeholder for actual usage
    /// let response = request_builder.send().await?;
    /// println!("Status: {}", response.status());
    /// ```
    pub async fn send(self) -> Result<reqwest::Response> {
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

    /// Sends the request and parses the response as JSON.
    ///
    /// Convenience method combining [`Self::send`] with JSON parsing and error handling.
    ///
    /// # Errors
    ///
    /// Returns errors for network issues, HTTP status errors, or JSON parsing failures.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use serde_json::Value;
    /// # // Placeholder for actual usage
    /// let data: Value = request_builder.send_json().await?;
    /// ```
    pub async fn send_json<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        let client = self.client.clone();
        let response = self.send().await?;
        client.parse_response(response).await
    }

    /// Returns a reference to the client.
    #[must_use]
    pub fn client(&self) -> &OpenFIGIClient {
        &self.client
    }

    /// Returns the HTTP method.
    #[must_use]
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Returns the request path.
    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns `true` if a request body has been set.
    #[must_use]
    pub fn has_body(&self) -> bool {
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
        let builder = OpenFIGIRequestBuilder::new(client, Method::GET, "/test");

        assert_eq!(builder.method(), &Method::GET);
        assert_eq!(builder.path(), "/test");
        assert!(!builder.has_body());
    }

    #[test]
    fn test_request_builder_body() {
        let client = create_test_client();
        let test_data = json!({"key": "value"});

        let builder = OpenFIGIRequestBuilder::new(client, Method::POST, "/test").body(&test_data);

        assert!(builder.has_body());
    }

    #[test]
    fn test_request_builder_try_body_success() {
        let client = create_test_client();
        let test_data = json!({"key": "value"});

        let result =
            OpenFIGIRequestBuilder::new(client, Method::POST, "/test").try_body(&test_data);

        assert!(result.is_ok());
        assert!(result.unwrap().has_body());
    }

    #[test]
    fn test_request_builder_json_body() {
        let client = create_test_client();
        let test_data = json!({"key": "value"});

        let builder =
            OpenFIGIRequestBuilder::new(client, Method::POST, "/test").json_body(test_data);

        assert!(builder.has_body());
    }

    #[test]
    fn test_request_builder_getters() {
        let client = create_test_client();
        let builder = OpenFIGIRequestBuilder::new(client.clone(), Method::PUT, "/api/test");

        assert_eq!(builder.method(), &Method::PUT);
        assert_eq!(builder.path(), "/api/test");
        assert_eq!(builder.client().base_url(), client.base_url());
    }

    #[test]
    fn test_request_builder_chaining() {
        let client = create_test_client();
        let test_data = json!({"test": true});

        let builder = OpenFIGIRequestBuilder::new(client, Method::POST, "/test").body(&test_data);

        assert_eq!(builder.method(), &Method::POST);
        assert_eq!(builder.path(), "/test");
        assert!(builder.has_body());
    }

    #[test]
    fn test_box_str_efficiency() {
        let client = create_test_client();
        let long_path = "/very/long/path/that/would/benefit/from/box/str/optimization";

        let builder = OpenFIGIRequestBuilder::new(client, Method::GET, long_path);
        assert_eq!(builder.path(), long_path);
    }
}
