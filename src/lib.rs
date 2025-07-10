#![doc = include_str!("../README.md")]

/// HTTP client for OpenFIGI API operations
pub mod client;
/// Client builder with fluent configuration API for custom HTTP settings
pub mod client_builder;
/// API endpoint implementations for mapping, search, and filter operations
pub mod endpoint;
/// Comprehensive error types with OpenFIGI-specific context and inspection methods
pub mod error;
/// Common utilities and macros for OpenFIGI client
pub(crate) mod macros;
/// Strongly typed request and response data models for all API operations
pub mod model;
/// Internal HTTP request builder utilities (not intended for direct use)
pub(crate) mod request_builder;
/// Test utilities for OpenFIGI client
#[cfg(test)]
#[macro_use]
mod test_utils;

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
