//! # `OpenFIGI` API Client
//!
//! Main HTTP client for interacting with the [OpenFIGI API](https://www.openfigi.com/api).
//! Provides simple instantiation with sensible defaults or flexible configuration via builder pattern.
//!
//! ## Key Features
//!
//! - Automatic rate limit detection and error context
//! - Connection pooling and efficient resource management  
//! - Middleware support for retries, logging, and observability
//! - Thread-safe and optimized for sharing across async tasks
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
//! and examples refer to [`OpenFIGIClientBuilder`].

use crate::{
    API_KEY, DEFAULT_BASE_URL,
    client_builder::OpenFIGIClientBuilder,
    error::{OpenFIGIError, Result},
    request_builder::OpenFIGIRequestBuilder,
};
use reqwest_middleware::ClientWithMiddleware;
use serde::de::DeserializeOwned;
use url::Url;

/// HTTP client for the `OpenFIGI` API.
///
/// Provides a high-level interface for making requests to the `OpenFIGI` service.
/// Optimized for performance with connection pooling, cheap cloning, and middleware support.
///
/// ## Memory Efficiency
///
/// Uses `Box<str>` for string storage to minimize heap allocations and improve
/// memory efficiency compared to `String`.
///
/// # Examples
///
/// ```rust
/// use openfigi_rs::client::OpenFIGIClient;
///
/// // Simple usage
/// let client = OpenFIGIClient::new();
///
/// // Custom configuration
/// let client = OpenFIGIClient::builder()
///     .api_key("your-api-key")
///     .build()
///     .expect("Failed to build client");
/// ```
#[derive(Clone, Debug)]
pub struct OpenFIGIClient {
    client: ClientWithMiddleware,
    base_url: Url,
    api_key: Option<Box<str>>,
}

impl Default for OpenFIGIClient {
    /// Create a new [`OpenFIGIClient`] with default settings.
    ///
    /// This implementation constructs the client directly with known-good default values
    /// to ensure it's infallible and doesn't depend on the builder pattern.
    ///
    /// Uses a default `ClientWithMiddleware` and attempts to read the API key
    /// from the `OPENFIGI_API_KEY` environment variable.
    fn default() -> Self {
        let api_key = API_KEY.as_ref().cloned();
        Self {
            client: ClientWithMiddleware::default(),
            base_url: DEFAULT_BASE_URL.clone(),
            api_key,
        }
    }
}

impl OpenFIGIClient {
    /// Create a new [`OpenFIGIClient`] with default configuration.
    ///
    /// Uses the official `OpenFIGI` API URL and reads the API key from the
    /// `OPENFIGI_API_KEY` environment variable if available.
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

    /// Create a new [`OpenFIGIClient`] from individual components.
    ///
    /// Provides maximum control over client configuration. Primarily used
    /// internally by the builder pattern.
    ///
    /// # Arguments
    ///
    /// * `client` - HTTP client for making requests
    /// * `base_url` - Base URL for the `OpenFIGI` API
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
            api_key: api_key.map(Into::into),
        }
    }

    /// Returns a builder for configuring an [`OpenFIGIClient`].
    ///
    /// Use the builder pattern when you need custom configuration beyond
    /// the defaults provided by [`OpenFIGIClient::new`].
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
    /// Provides access to the underlying HTTP client for advanced usage scenarios.
    #[must_use]
    pub fn client(&self) -> &ClientWithMiddleware {
        &self.client
    }

    /// Returns the base URL for the API.
    #[must_use]
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// Returns the API key if one is configured.
    #[must_use]
    pub fn api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }

    /// Returns whether an API key is set.
    #[must_use]
    pub fn has_api_key(&self) -> bool {
        self.api_key.is_some()
    }

    /// Returns a request builder for the given endpoint path.
    pub(crate) fn request(&self, path: &str, method: reqwest::Method) -> OpenFIGIRequestBuilder {
        OpenFIGIRequestBuilder::new(self.clone(), method, path)
    }

    /// Parses an HTTP response into the expected type with comprehensive error handling.
    ///
    /// Provides centralized response processing with OpenFIGI-specific error handling,
    /// rate limit detection, and detailed error context.
    ///
    /// ## OpenFIGI-Specific Error Handling
    ///
    /// - **400 Bad Request**: Check request format and required fields
    /// - **401 Unauthorized**: Verify API key is correct  
    /// - **429 Too Many Requests**: Implement exponential backoff
    /// - **413 Payload Too Large**: Reduce mapping jobs (max 100 with API key, 5 without)
    /// - **500+ Server Errors**: Retry with exponential backoff
    ///
    /// Rate limit information is automatically parsed from response headers when available.
    pub(crate) async fn parse_response<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T> {
        let status = response.status();

        // Early return for success case to optimize the common path
        if status == reqwest::StatusCode::OK {
            return response.json::<T>().await.map_err(OpenFIGIError::from);
        }

        // For error cases, we need the URL
        let url = response.url().clone();

        // Extract rate limiting info more efficiently (only for error cases)
        let rate_limit_info = Self::extract_rate_limit_info(response.headers());

        // Get response text for error cases
        let resp_text = response.text().await.map_err(OpenFIGIError::from)?;

        // Handle different HTTP status codes with OpenFIGI-specific context
        let error_message = Self::format_error_message(status, &url, rate_limit_info, &resp_text);

        Err(OpenFIGIError::response_error(
            status,
            resp_text,
            Some(error_message),
        ))
    }

    /// Extract rate limit information from response headers.
    fn extract_rate_limit_info(headers: &reqwest::header::HeaderMap) -> Option<String> {
        let remaining = headers
            .get("X-RateLimit-Remaining")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u32>().ok())?;

        let reset = headers
            .get("X-RateLimit-Reset")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())?;

        Some(format!(
            "Rate limit: {remaining} requests remaining, resets at {reset}"
        ))
    }

    /// Format error message based on status code and context.
    fn format_error_message(
        status: reqwest::StatusCode,
        url: &Url,
        rate_limit_info: Option<String>,
        resp_text: &str,
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
                "Payload too large for {url}: Too many mapping jobs in request (max 100 with API key, 5 without)."
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
            _ => format!(
                "Unexpected HTTP status {} from {url}: {resp_text}",
                status.as_u16()
            ),
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
    fn test_client_default() {
        let client = OpenFIGIClient::default();
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
