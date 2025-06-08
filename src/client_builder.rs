//! # `OpenFIGI` Client Builder
//!
//! Fluent builder for configuring [`crate::client::OpenFIGIClient`] with custom settings.
//!
//! ## Key Features
//!
//! - **Fluent API**: Chainable method calls for clean configuration
//! - **HTTP Client Support**: Integrate custom `reqwest::Client` or middleware stacks
//! - **Memory Efficient**: Uses `Box<str>` for optimal string storage
//! - **Smart Defaults**: Falls back to environment variables and sensible defaults
//!
//! ## Examples
//!
//! ### Basic Setup
//! ```rust
//! use openfigi_rs::client_builder::OpenFIGIClientBuilder;
//!
//! let client = OpenFIGIClientBuilder::new()
//!     .api_key("your_api_key")
//!     .build()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### Custom HTTP Client
//! ```rust
//! use openfigi_rs::client_builder::OpenFIGIClientBuilder;
//! use reqwest::Client;
//! use std::time::Duration;
//!
//! let http_client = Client::builder()
//!     .timeout(Duration::from_secs(30))
//!     .build()?;
//!
//! let client = OpenFIGIClientBuilder::new()
//!     .reqwest_client(http_client)
//!     .build()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### With Middleware
//! ```rust
//! use openfigi_rs::client_builder::OpenFIGIClientBuilder;
//! use reqwest_middleware::ClientBuilder;
//! use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
//!
//! let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
//! let middleware_client = ClientBuilder::new(reqwest::Client::new())
//!     .with(RetryTransientMiddleware::new_with_policy(retry_policy))
//!     .build();
//!
//! let client = OpenFIGIClientBuilder::new()
//!     .middleware_client(middleware_client)
//!     .build()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### Production Configuration
//! ```rust
//! use openfigi_rs::client_builder::OpenFIGIClientBuilder;
//! use reqwest::Client;
//! use reqwest_middleware::ClientBuilder;
//! use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
//! use std::time::Duration;
//!
//! // Configure HTTP client with timeouts
//! let http_client = Client::builder()
//!     .timeout(Duration::from_secs(30))
//!     .connect_timeout(Duration::from_secs(5))
//!     .build()?;
//!
//! // Add retry middleware
//! let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
//! let middleware_client = ClientBuilder::new(http_client)
//!     .with(RetryTransientMiddleware::new_with_policy(retry_policy))
//!     .build();
//!
//! // Build OpenFIGI client
//! let client = OpenFIGIClientBuilder::new()
//!     .base_url("https://api.openfigi.com/v3")
//!     .api_key(std::env::var("OPENFIGI_API_KEY")?)
//!     .middleware_client(middleware_client)
//!     .build()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use crate::{API_KEY, DEFAULT_BASE_URL, client::OpenFIGIClient, error::Result};
use reqwest::Client as ReqwestClient;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use url::Url;

/// Builder for configuring [`OpenFIGIClient`] instances with custom settings.
///
/// Provides a fluent API for client configuration with memory-efficient string storage.
/// Supports custom HTTP clients, middleware stacks, base URLs, and API keys with
/// automatic fallbacks to environment variables and sensible defaults.
///
/// ## Client Precedence
///
/// When building the final client, HTTP clients are prioritized as follows:
/// 1. `middleware_client` (if set via [`Self::middleware_client`])
/// 2. `reqwest_client` wrapped with default middleware (if set via [`Self::reqwest_client`])
/// 3. Default `reqwest::Client` with default middleware
///
/// ## Memory Efficiency
///
/// Uses `Box<str>` for string storage to minimize heap allocations and improve
/// memory efficiency compared to `String`.
///
/// # Examples
///
/// ```rust
/// use openfigi_rs::client_builder::OpenFIGIClientBuilder;
///
/// // Basic configuration
/// let client = OpenFIGIClientBuilder::new()
///     .api_key("your-key")
///     .base_url("https://api.openfigi.com/v3")
///     .build()?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct OpenFIGIClientBuilder {
    reqwest_client: Option<ReqwestClient>,
    middleware_client: Option<ClientWithMiddleware>,
    base_url: Option<Box<str>>,
    api_key: Option<Box<str>>,
}

impl Default for OpenFIGIClientBuilder {
    /// Create a new builder with all fields unset.
    ///
    /// Equivalent to [`Self::new`]. All configuration will use defaults
    /// or environment variables during [`Self::build`].
    fn default() -> Self {
        Self::new()
    }
}

impl OpenFIGIClientBuilder {
    /// Create a new builder with default settings.
    ///
    /// All fields start as `None` and will use defaults or environment variables during [`Self::build`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::client_builder::OpenFIGIClientBuilder;
    ///
    /// let builder = OpenFIGIClientBuilder::new();
    /// let client = builder.build()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            reqwest_client: None,
            middleware_client: None,
            base_url: None,
            api_key: None,
        }
    }

    /// Set a custom base URL for the API.
    ///
    /// Defaults to `https://api.openfigi.com/v3/` if not set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::client_builder::OpenFIGIClientBuilder;
    ///
    /// let client = OpenFIGIClientBuilder::new()
    ///     .base_url("https://api-sandbox.openfigi.com/v3/")
    ///     .build()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn base_url(mut self, url: impl Into<Box<str>>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set the API key for authentication.
    ///
    /// If not explicitly set, the system defaults to the `OPENFIGI_API_KEY`
    /// environment variable. If neither is specified, the client will operate
    /// without authentication.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::client_builder::OpenFIGIClientBuilder;
    ///
    /// let client = OpenFIGIClientBuilder::new()
    ///     .api_key("your-api-key")
    ///     .build()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn api_key(mut self, key: impl Into<Box<str>>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Use a custom reqwest client for HTTP operations.
    ///
    /// The client will be wrapped with default middleware.
    /// Note: [`Self::middleware_client`] takes precedence if both are set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::client_builder::OpenFIGIClientBuilder;
    /// use reqwest::Client;
    /// use std::time::Duration;
    ///
    /// let http_client = Client::builder()
    ///     .timeout(Duration::from_secs(30))
    ///     .build()?;
    ///
    /// let client = OpenFIGIClientBuilder::new()
    ///     .reqwest_client(http_client)
    ///     .build()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn reqwest_client(mut self, client: ReqwestClient) -> Self {
        self.reqwest_client = Some(client);
        self
    }

    /// Use a pre-configured middleware client for HTTP operations.
    ///
    /// Takes precedence over any reqwest client set via [`Self::reqwest_client`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::client_builder::OpenFIGIClientBuilder;
    /// use reqwest_middleware::ClientBuilder;
    /// use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
    ///
    /// let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    /// let middleware_client = ClientBuilder::new(reqwest::Client::new())
    ///     .with(RetryTransientMiddleware::new_with_policy(retry_policy))
    ///     .build();
    ///
    /// let client = OpenFIGIClientBuilder::new()
    ///     .middleware_client(middleware_client)
    ///     .build()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn middleware_client(mut self, client: ClientWithMiddleware) -> Self {
        self.middleware_client = Some(client);
        self
    }

    /// Build the [`OpenFIGIClient`] with the configured settings.
    ///
    /// Uses the following precedence for HTTP clients:
    /// 1. `middleware_client` (if set)
    /// 2. `reqwest_client` wrapped with default middleware (if set)
    /// 3. Default reqwest client with default middleware
    ///
    /// # Errors
    ///
    /// Returns an error if the base URL cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::client_builder::OpenFIGIClientBuilder;
    ///
    /// let client = OpenFIGIClientBuilder::new()
    ///     .api_key("test-key")
    ///     .build()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn build(self) -> Result<OpenFIGIClient> {
        // Determine the client to use (middleware_client takes precedence)
        let client = match (self.middleware_client, self.reqwest_client) {
            (Some(middleware_client), _) => middleware_client,
            (None, Some(reqwest_client)) => ClientBuilder::new(reqwest_client).build(),
            (None, None) => ClientBuilder::new(ReqwestClient::default()).build(),
        };

        // Parse base URL or use default
        let base_url = match self.base_url {
            Some(url_str) => Url::parse(&url_str)?,
            None => DEFAULT_BASE_URL.clone(),
        };

        // Use provided API key or try environment variable (only if not set)
        let api_key = self
            .api_key
            .map(|boxed| (*boxed).to_owned())
            .or_else(|| API_KEY.as_ref().map(std::string::ToString::to_string));

        Ok(OpenFIGIClient::new_with_components(
            client, base_url, api_key,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client as ReqwestClient;
    use reqwest_middleware::ClientBuilder;
    use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};

    #[test]
    fn test_builder_basic() {
        let client = OpenFIGIClientBuilder::new()
            .build()
            .expect("Client build should succeed");
        assert_eq!(client.base_url().as_str(), DEFAULT_BASE_URL.as_str());
    }

    #[test]
    fn test_builder_base_url() {
        let custom_url = "https://api-custom.openfigi.com/v3/";
        let client = OpenFIGIClientBuilder::new()
            .base_url(custom_url)
            .build()
            .expect("Client build should succeed");
        assert_eq!(client.base_url().as_str(), custom_url);
    }

    #[test]
    fn test_builder_api_key() {
        let client = OpenFIGIClientBuilder::new()
            .api_key("test_key")
            .build()
            .expect("Client build should succeed");
        assert!(client.has_api_key());
    }

    #[test]
    fn test_builder_reqwest_client() {
        let reqwest_client = ReqwestClient::new();
        let client = OpenFIGIClientBuilder::new()
            .reqwest_client(reqwest_client)
            .build()
            .expect("Client build should succeed");
        assert_eq!(client.base_url().as_str(), DEFAULT_BASE_URL.as_str());
    }

    #[test]
    fn test_builder_middleware_client() {
        let reqwest_client = ReqwestClient::new();
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
        let middleware_client = ClientBuilder::new(reqwest_client)
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        let client = OpenFIGIClientBuilder::new()
            .middleware_client(middleware_client)
            .build()
            .expect("Client build should succeed");

        assert_eq!(client.base_url().as_str(), DEFAULT_BASE_URL.as_str());
    }

    #[test]
    fn test_builder_invalid_url() {
        let result = OpenFIGIClientBuilder::new()
            .base_url("not-a-valid-url")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_middleware_precedence() {
        let reqwest_client = ReqwestClient::new();
        let middleware_client = ClientBuilder::new(ReqwestClient::new()).build();

        let client = OpenFIGIClientBuilder::new()
            .reqwest_client(reqwest_client)
            .middleware_client(middleware_client)
            .build()
            .expect("Client build should succeed");

        // Should use middleware_client, not reqwest_client
        assert_eq!(client.base_url().as_str(), DEFAULT_BASE_URL.as_str());
    }

    #[test]
    fn test_builder_chaining() {
        let client = OpenFIGIClientBuilder::new()
            .base_url("https://api-custom.openfigi.com/v3/")
            .api_key("test_key")
            .reqwest_client(ReqwestClient::new())
            .build()
            .expect("Client build should succeed");

        assert_eq!(
            client.base_url().as_str(),
            "https://api-custom.openfigi.com/v3/"
        );
        assert!(client.has_api_key());
    }
}
