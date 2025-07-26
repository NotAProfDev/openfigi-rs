//! HTTP client for interacting with the OpenFIGI API.
//!
//! This module provides the main [`crate::client::OpenFIGIClient`] for making requests to the
//! [OpenFIGI API](https://www.openfigi.com/api). The client is designed for both
//! simple usage with sensible defaults and advanced configuration through the builder pattern.
//!
//! ## Key Features
//!
//! - **Simple instantiation** with automatic environment variable detection
//! - **Rate limit handling** with automatic detection and error context
//! - **Connection pooling** and efficient resource management  
//! - **Middleware support** for retries, logging, and observability
//! - **Thread-safe** and optimized for sharing across async tasks
//! - **Comprehensive error handling** with OpenFIGI-specific error messages
//!
//! ## Examples
//!
//! ### Basic Setup
//!
//! ```rust
//! use openfigi_rs::client::OpenFIGIClient;
//!
//! // Default configuration with environment variable API key
//! let client = OpenFIGIClient::new();
//! ```
//!
//! ### Custom HTTP Client
//!
//! ```rust
//! use openfigi_rs::client::OpenFIGIClient;
//! use reqwest::Client;
//! use reqwest_middleware::ClientWithMiddleware;
//! use url::Url;
//!
//! // Example custom HTTP client with middleware
//! let http_client = ClientWithMiddleware::default();
//!
//! let client = OpenFIGIClient::new_with_components(http_client,
//!     Url::parse("https://api.openfigi.com/v3/").expect("Invalid URL"),
//!     Some("api_key".into())
//! );
//! ```
//!
//! ### Fluent Builder API
//!
//! The Fluent Builder API provides a flexible and structured approach to configuring the client.
//! By leveraging the builder pattern, developers can fine-tune settings while maintaining clarity
//! and scalability in their code. For a detailed breakdown of available configuration options,
//! and examples refer to [`crate::client_builder::OpenFIGIClientBuilder`].

use crate::{
    API_KEY, DEFAULT_BASE_URL,
    client_builder::OpenFIGIClientBuilder,
    error::{OpenFIGIError, Result},
    model::response::ResponseResult,
    request_builder::OpenFIGIRequestBuilder,
};
use reqwest_middleware::ClientWithMiddleware;
use serde::de::DeserializeOwned;
use url::Url;

/// HTTP client for making requests to the OpenFIGI API.
///
/// This client provides a high-level interface for interacting with the OpenFIGI service,
/// featuring automatic authentication, error handling, and connection management. The client
/// is designed to be cheaply cloneable and thread-safe for use across async tasks.
///
/// ## Authentication
///
/// The client supports API key authentication, which sets higher rate limits. API keys can be
/// provided explicitly or read from the `OPENFIGI_API_KEY` environment variable.
///
/// ## Performance
///
/// - **Connection Pooling**: Reuses HTTP connections for efficiency
/// - **Cheap Cloning**: Uses `Arc` internally, making clones very lightweight
/// - **Middleware Support**: Configurable retry policies, logging, and observability
///
/// ## Error Handling
///
/// The client provides comprehensive error handling with OpenFIGI-specific context:
/// - Rate limit detection with retry recommendations
/// - Detailed HTTP status code explanations
/// - Request validation errors with suggestions
///
/// # Examples
///
/// ```rust
/// use openfigi_rs::client::OpenFIGIClient;
///
/// // Simple usage with defaults
/// let client = OpenFIGIClient::new();
///
/// // Custom configuration via builder
/// let client = OpenFIGIClient::builder()
///     .api_key("your-api-key")
///     .build()
///     .expect("Failed to build client");
/// ```
#[derive(Clone, Debug)]
pub struct OpenFIGIClient {
    client: ClientWithMiddleware,
    base_url: Url,
    api_key: Option<String>,
}

impl Default for OpenFIGIClient {
    /// Create a new [`crate::client::OpenFIGIClient`] with default settings.
    ///
    /// This implementation constructs the client directly with known-good default values
    /// to ensure it's infallible and doesn't depend on the builder pattern.
    ///
    /// Uses a default `ClientWithMiddleware` and attempts to read the API key
    /// from the `OPENFIGI_API_KEY` environment variable.
    fn default() -> Self {
        let api_key = API_KEY.as_ref().map(std::string::ToString::to_string);
        Self {
            client: ClientWithMiddleware::default(),
            base_url: DEFAULT_BASE_URL.clone(),
            api_key,
        }
    }
}

impl OpenFIGIClient {
    /// Create a new [`crate::client::OpenFIGIClient`] with default configuration.
    ///
    /// Uses the official OpenFIGI API base URL (`https://api.openfigi.com/v3/`) and
    /// attempts to read the API key from the `OPENFIGI_API_KEY` environment variable.
    /// If no environment variable is set, the client operates without authentication.
    ///
    /// This is the simplest way to create a client and is suitable for most use cases.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::client::OpenFIGIClient;
    ///
    /// let client = OpenFIGIClient::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`crate::client::OpenFIGIClient`] from individual components.
    ///
    /// Provides maximum control over client configuration. Primarily used
    /// internally by the builder pattern.
    ///
    /// # Arguments
    ///
    /// * `client` - HTTP client for making requests
    /// * `base_url` - Base URL for the OpenFIGI API
    /// * `api_key` - Optional API key for authentication
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::client::OpenFIGIClient;
    /// use reqwest_middleware::ClientWithMiddleware;
    /// use url::Url;
    ///
    /// let client = ClientWithMiddleware::default();
    /// let base_url = Url::parse("https://api.openfigi.com/v3/").unwrap();
    /// let api_key = Some("your-api-key".to_string());
    ///
    /// let openfigi_client = OpenFIGIClient::new_with_components(client, base_url, api_key);
    /// ```
    #[must_use]
    pub fn new_with_components(
        client: ClientWithMiddleware,
        base_url: Url,
        api_key: Option<String>,
    ) -> Self {
        Self {
            client,
            base_url,
            api_key,
        }
    }

    /// Returns a builder for configuring an [`crate::client::OpenFIGIClient`].
    ///
    /// Use the builder pattern when you need custom configuration beyond
    /// the defaults provided by [`crate::client::OpenFIGIClient::new`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::client::OpenFIGIClient;
    ///
    /// let client = OpenFIGIClient::builder()
    ///     .api_key("your-api-key")
    ///     .base_url("https://api.openfigi.com/v3/")
    ///     .build()
    ///     .expect("Failed to build client");
    /// ```
    #[must_use]
    pub fn builder() -> OpenFIGIClientBuilder {
        OpenFIGIClientBuilder::new()
    }

    /// Returns a reference to the underlying HTTP client.
    ///
    /// Provides access to the wrapped `ClientWithMiddleware` for advanced usage
    /// scenarios where direct HTTP client access is needed. Most users should
    /// prefer the high-level API methods instead.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::client::OpenFIGIClient;
    ///
    /// let client = OpenFIGIClient::new();
    /// let http_client = client.client();
    /// // Use http_client for advanced HTTP operations
    /// ```
    #[must_use]
    pub fn client(&self) -> &ClientWithMiddleware {
        &self.client
    }

    /// Returns the base URL configured for this client.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::client::OpenFIGIClient;
    ///
    /// let client = OpenFIGIClient::new();
    /// println!("Base URL: {}", client.base_url());
    /// ```
    #[must_use]
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// Returns the API key if one is configured.
    ///
    /// Returns `Some(key)` if an API key was provided during client creation,
    /// either explicitly or via the `OPENFIGI_API_KEY` environment variable.
    /// Returns `None` if no API key is configured.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::client::OpenFIGIClient;
    ///
    /// let client = OpenFIGIClient::builder()
    ///     .api_key("your-api-key")
    ///     .build()
    ///     .unwrap();
    ///     
    /// assert_eq!(client.api_key(), Some("your-api-key"));
    /// ```
    #[must_use]
    pub fn api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }

    /// Returns whether an API key is configured for this client.
    ///
    /// This is a convenience method equivalent to `self.api_key().is_some()`.
    /// Useful for checking authentication status before making requests.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::client::OpenFIGIClient;
    ///
    /// let client = OpenFIGIClient::new();
    /// if client.has_api_key() {
    ///     println!("Client is authenticated");
    /// } else {
    ///     println!("Client will use public rate limits");
    /// }
    /// ```
    #[must_use]
    pub fn has_api_key(&self) -> bool {
        self.api_key.is_some()
    }

    #[doc(hidden)]
    /// Creates a request builder for the specified endpoint path and HTTP method.
    ///
    /// This is an internal method used by the endpoint-specific request methods.
    /// It handles URL construction, authentication header injection, and request
    /// configuration setup.
    ///
    /// # Arguments
    ///
    /// * `path` - The API endpoint path (e.g., "mapping", "search")
    /// * `method` - The HTTP method to use for the request
    ///
    /// # Returns
    ///
    /// Returns an [`OpenFIGIRequestBuilder`] configured for this client and endpoint.
    pub(crate) fn request(&self, path: &str, method: reqwest::Method) -> OpenFIGIRequestBuilder {
        OpenFIGIRequestBuilder::new(self.clone(), method, path)
    }

    #[doc(hidden)]
    /// Parses single HTTP responses with comprehensive OpenFIGI-specific error handling.
    ///
    /// This internal method processes HTTP responses from the OpenFIGI API, providing
    /// detailed error context and automatic rate limit detection. It converts successful
    /// responses to the expected type and transforms error responses into meaningful
    /// [`crate::error::OpenFIGIError`] instances.
    ///
    /// ## Error Handling by Status Code
    ///
    /// - **200 OK**: Successfully deserializes response to type `T`
    /// - **400 Bad Request**: Invalid request format or missing required fields
    /// - **401 Unauthorized**: Missing or invalid API key
    /// - **404 Not Found**: Requested resource not found
    /// - **405 Method Not Allowed**: HTTP method not supported for endpoint
    /// - **406 Not Acceptable**: Unsupported Accept header
    /// - **413 Payload Too Large**: Too many requests in batch (max 100 with API key, 5 without)
    /// - **429 Too Many Requests**: Rate limit exceeded, includes retry timing
    /// - **500 Internal Server Error**: Resend the request later with an exponential backoff strategy
    /// - **503**: Service temporarily unavailable
    ///
    /// ## Rate Limiting
    ///
    /// When rate limit headers are present (`X-RateLimit-Remaining`, `X-RateLimit-Reset`),
    /// they are automatically parsed and included in error messages to help with retry logic.
    ///
    /// # Arguments
    ///
    /// * `response` - The HTTP response from the OpenFIGI API
    ///
    /// # Returns
    ///
    /// Returns `Ok(T)` for successful responses or `Err(OpenFIGIError)` with detailed context.
    pub(crate) async fn parse_single_response<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T> {
        let status = response.status();

        // Early return for success case to optimize the common path
        if status.is_success() {
            // Deserialize the response body into the expected type `T`
            let parsed_response: ResponseResult<T> =
                response.json().await.map_err(OpenFIGIError::from)?;

            // Check if the response contains an error
            match parsed_response {
                ResponseResult::Success(data) => return Ok(data),
                ResponseResult::Error(err) => {
                    // If the response contains an error, convert it to OpenFIGIError
                    return Err(OpenFIGIError::response_error(
                        status,
                        format!("OpenFIGI API error: {}", err.error),
                        String::new(),
                    ));
                }
            }
        }

        return Err(self.handle_error_response(response).await);
    }

    #[doc(hidden)]
    /// Parses a list of HTTP responses from the OpenFIGI API, handling both successes and errors for batch mapping requests.
    ///
    /// This internal method processes HTTP responses that return a list of results (such as batch mapping requests).
    /// Each item in the returned vector is a `Result<T, OpenFIGIError>`, where:
    /// - `Ok(T)` contains a successfully parsed mapping result
    /// - `Err(OpenFIGIError)` contains a detailed error for a failed mapping request
    ///
    /// # Arguments
    ///
    /// * `response` - The HTTP response from the OpenFIGI API
    ///
    /// # Returns
    ///
    /// Returns `Ok(Vec<Result<T, OpenFIGIError>>)` for successful HTTP responses, where each item is either a success or error for the corresponding batch request.
    /// Returns `Err(OpenFIGIError)` if the HTTP response itself is an error (e.g., network failure, invalid status code).
    ///
    /// # Error Handling
    ///
    /// - Each batch item is parsed and mapped to either a success or error variant.
    /// - If the HTTP response is not successful, a single `OpenFIGIError` is returned for the entire batch.
    pub(crate) async fn parse_list_response<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<Vec<Result<T>>> {
        let status = response.status();

        // Early return for success case to optimize the common path
        if response.status().is_success() {
            // Deserialize the response body into the expected type `T`
            let parsed_list: Vec<ResponseResult<T>> =
                response.json().await.map_err(OpenFIGIError::from)?;

            // Transform the parsed list into a `Result<T, OpenFIGIError>`.
            let results: Vec<Result<T>> = parsed_list
                .into_iter()
                .map(|item| match item {
                    ResponseResult::Success(data) => Ok(data),
                    ResponseResult::Error(err) => Err(OpenFIGIError::response_error(
                        status,
                        format!("OpenFIGI API error: {}", err.error),
                        String::new(),
                    )),
                })
                .collect();

            return Ok(results);
        }

        return Err(self.handle_error_response(response).await);
    }

    /// Handles non-successful HTTP responses by creating a detailed `OpenFIGIError`.
    async fn handle_error_response(&self, response: reqwest::Response) -> OpenFIGIError {
        let status = response.status();
        let url = response.url().clone();

        // Rate-Limit-Info nur bei 429 extrahieren
        let rate_limit_info = if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            Self::extract_rate_limit_info(response.headers())
        } else {
            None
        };

        // Handle different HTTP status codes with OpenFIGI-specific context
        let error_message = Self::format_error_message(status, &url, rate_limit_info);

        // Use `unwrap_or_default` to avoid panics if text cannot be read
        let resp_text = response.text().await.unwrap_or_default();

        OpenFIGIError::response_error(status, error_message, resp_text)
    }

    /// Extracts rate limit information from HTTP response headers.
    ///
    /// Parses the `ratelimit-policy` and `ratelimit-reset` / `retry-after` headers commonly
    /// used by the OpenFIGI API to communicate rate limiting status. This information
    /// is used to provide helpful error messages when rate limits are exceeded.
    ///
    /// # Arguments
    ///
    /// * `headers` - HTTP response headers to parse
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` with formatted rate limit information if both headers
    /// are present and valid, `None` otherwise.
    fn extract_rate_limit_info(headers: &reqwest::header::HeaderMap) -> Option<String> {
        let policy = headers
            .get("ratelimit-policy")
            .and_then(|v| v.to_str().ok());

        let reset = headers
            .get("ratelimit-reset")
            .and_then(|v| v.to_str().ok())
            .or_else(|| headers.get("retry-after").and_then(|v| v.to_str().ok()));

        match (policy, reset) {
            (Some(policy), Some(reset)) => Some(format!(
                "Rate limit policy: {policy}, resets in {reset} seconds"
            )),
            (None, Some(reset)) => Some(format!("Rate limit reset in {reset} seconds")),
            _ => None,
        }
    }

    /// Formats comprehensive error messages based on HTTP status codes and context.
    ///
    /// Creates user-friendly error messages with specific guidance for different
    /// types of failures. Includes rate limit information when available and provides
    /// actionable suggestions for resolving common issues.
    ///
    /// # Arguments
    ///
    /// * `status` - HTTP status code from the response
    /// * `url` - The URL that was requested
    /// * `rate_limit_info` - Optional rate limit information from headers
    /// * `resp_text` - Raw response body text
    ///
    /// # Returns
    ///
    /// Returns a formatted error message string with context and suggestions.
    fn format_error_message(
        status: reqwest::StatusCode,
        url: &Url,
        rate_limit_info: Option<String>,
    ) -> String {
        match status {
            reqwest::StatusCode::BAD_REQUEST => {
                format!("Bad request to {url}: Invalid request body or parameters.")
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                format!("Unauthorized access to {url}: API key is missing or invalid.")
            }
            reqwest::StatusCode::NOT_FOUND => {
                format!("Not found error from {url}: The requested resource could not be found.")
            }
            reqwest::StatusCode::METHOD_NOT_ALLOWED => format!(
                "Method not allowed for {url}: The requested method is not supported for this endpoint."
            ),
            reqwest::StatusCode::NOT_ACCEPTABLE => {
                format!("Not acceptable request to {url}: Unsupported Accept header type.")
            }
            reqwest::StatusCode::PAYLOAD_TOO_LARGE => format!(
                "Payload too large for {url}: Too many mapping requests in request (max 100 with API key, 5 without)."
            ),
            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                let rate_msg = rate_limit_info.unwrap_or_else(|| "Rate limit exceeded".to_string());
                format!("{rate_msg} for {url}. Please retry later.")
            }
            reqwest::StatusCode::INTERNAL_SERVER_ERROR => format!(
                "Internal server error from {url}: OpenFIGI service is experiencing issues."
            ),
            reqwest::StatusCode::BAD_GATEWAY
            | reqwest::StatusCode::SERVICE_UNAVAILABLE
            | reqwest::StatusCode::GATEWAY_TIMEOUT => format!(
                "Service unavailable from {url}: OpenFIGI service is temporarily unavailable. Please retry later."
            ),
            _ => format!("Unexpected HTTP status {} from {url}", status.as_u16()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_new() {
        let client = OpenFIGIClient::new();
        assert_eq!(client.base_url(), &*DEFAULT_BASE_URL);
    }

    #[test]
    fn test_client_with_components() {
        let client = ClientWithMiddleware::default();
        let base_url = DEFAULT_BASE_URL.clone();
        let api_key = Some("test_key".to_string());

        let openfigi_client =
            OpenFIGIClient::new_with_components(client, base_url.clone(), api_key.clone());

        assert_eq!(openfigi_client.base_url(), &base_url);
        assert_eq!(openfigi_client.api_key(), api_key.as_deref());
        assert!(openfigi_client.has_api_key());
    }
}
