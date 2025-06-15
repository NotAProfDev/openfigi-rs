//! # OpenFIGI Rust Client
//!
//! A high-performance Rust client library for the [OpenFIGI API](https://www.openfigi.com/api),
//! providing type-safe access to financial instrument identification and mapping services.
//!
//! OpenFIGI is Bloomberg's open symbology initiative that provides standardized identification
//! for financial instruments across asset classes and markets worldwide.
//!
//! ## Quick Start
//!
//! ### Basic Usage
//!
//! ```rust,no_run
//! use openfigi_rs::client::OpenFIGIClient;
//! use openfigi_rs::model::request::MappingRequest;
//! use openfigi_rs::model::enums::IdType;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a client (uses OPENFIGI_API_KEY env var if available)
//! let client = OpenFIGIClient::new();
//!
//! // Map an ISIN to FIGI
//! let request = MappingRequest::new(IdType::IdIsin, "US4592001014");
//! let mapping_results = client.mapping(IdType::IdIsin, "US4592001014").send().await?;
//!
//! println!("FIGI: {}", mapping_results[0].figi);
//! # Ok(())
//! # }
//! ```
//!
//! ### With API Key
//!
//! ```rust,no_run
//! use openfigi_rs::client::OpenFIGIClient;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Explicit API key configuration
//! let client = OpenFIGIClient::builder()
//!     .api_key("your-api-key")
//!     .build()?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Custom HTTP Configuration
//!
//! ```rust,no_run
//! use openfigi_rs::client::OpenFIGIClient;
//! use reqwest_middleware::ClientBuilder;
//! use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
//! use std::time::Duration;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Configure HTTP client with retry middleware
//! let http_client = reqwest::Client::builder()
//!     .timeout(Duration::from_secs(30))
//!     .build()?;
//!
//! let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
//! let middleware_client = ClientBuilder::new(http_client)
//!     .with(RetryTransientMiddleware::new_with_policy(retry_policy))
//!     .build();
//!
//! let client = OpenFIGIClient::builder()
//!     .middleware_client(middleware_client)
//!     .api_key("your-api-key")
//!     .build()?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Authentication & Rate Limits
//!
//! - **Without API key**: 25 requests per minute, max 5 requests per batch
//! - **With API key**: 10,000 requests per minute, max 100 requests per batch
//! - **API key**: Set via `OPENFIGI_API_KEY` environment variable or explicit configuration
//!
//! ## API Endpoints
//!
//! | Endpoint | Purpose | Batch Support |
//! |----------|---------|---------------|
//! | `/mapping` | Map identifiers to FIGIs | ✓ |
//! | `/search` | Search instruments by text | ✗ |
//! | `/filter` | Filter instruments by criteria | ✗ |
//!
//! ## Features
//!
//! - **Type-safe API**: Strongly typed request/response models with validation
//! - **Async/await**: Built on `reqwest` with full async support and connection pooling
//! - **Middleware support**: Extensible HTTP middleware for retries, logging, and observability
//! - **Comprehensive error handling**: Detailed error types with OpenFIGI-specific context
//! - **Rate limit awareness**: Automatic rate limit detection and informative error messages
//! - **Environment integration**: Automatic API key detection from environment variables
//! - **Production ready**: Connection pooling, timeouts, and efficient resource management

/// HTTP client for OpenFIGI API operations
pub mod client;
/// Client builder with fluent configuration API for custom HTTP settings
pub mod client_builder;
/// API endpoint implementations for mapping, search, and filter operations
pub mod endpoint;
/// Comprehensive error types with OpenFIGI-specific context and inspection methods
pub mod error;
/// Strongly typed request and response data models for all API operations
pub mod model;
/// Internal HTTP request builder utilities (not intended for direct use)
pub(crate) mod request_builder;

use std::sync::LazyLock;
use url::Url;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The default base URL for the OpenFIGI API v3.
///
/// This URL is used by default when creating clients without explicit base URL configuration.
pub static DEFAULT_BASE_URL: LazyLock<Url> = LazyLock::new(|| {
    Url::parse("https://api.openfigi.com/v3/").expect("Built-in default URL should always be valid")
});

/// The default endpoint path for mapping requests.
///
/// Used for converting between different financial identifier types.
pub const DEFAULT_ENDPOINT_MAPPING: &str = "mapping";

/// The default endpoint path for search requests.
///
/// Used for text-based instrument search operations.
pub const DEFAULT_ENDPOINT_SEARCH: &str = "search";

/// The default endpoint path for filter requests.
///
/// Used for filtering instruments by specific criteria.
pub const DEFAULT_ENDPOINT_FILTER: &str = "filter";

/// API key loaded from the `OPENFIGI_API_KEY` environment variable.
///
/// This is automatically loaded at startup and used by default when creating clients.
/// `None` if the environment variable is not set.
static API_KEY: LazyLock<Option<String>> = LazyLock::new(|| std::env::var("OPENFIGI_API_KEY").ok());
