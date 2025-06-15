//! # OpenFIGI Rust Client
//!
//! Rust client library for the [OpenFIGI API](https://www.openfigi.com/api),
//! providing type-safe access to financial instrument mapping services.
//!
//! ## Quick Start
//!
//! ```rust
//! use openfigi_rs::client::OpenFIGIClient;
//! use openfigi_rs::model::request::MappingRequest;
//! use openfigi_rs::model::enums::IdType;
//! use serde_json::json;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = OpenFIGIClient::new();
//! let request = MappingRequest::new(IdType::IdIsin, json!("US4592001014"));
//! # Ok(())
//! # }
//! ```
//!
//! ## Features
//!
//! - **Type-safe API**: Strongly typed request/response models
//! - **Async/await**: Built on `reqwest` with full async support
//! - **Middleware**: Extensible HTTP middleware support
//! - **Error handling**: Comprehensive error types with context

/// HTTP client and configuration
pub mod client;
/// Client builder with fluent configuration API
pub mod client_builder;
/// API endpoint implementations
pub mod endpoint;
/// Error types and result handling
pub mod error;
/// Request and response data models
pub mod model;
/// HTTP request builder utilities
pub mod request_builder;

use std::sync::LazyLock;
use url::Url;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The default base URL for the OpenFIGI API v3.
pub static DEFAULT_BASE_URL: LazyLock<Url> = LazyLock::new(|| {
    Url::parse("https://api.openfigi.com/v3/").expect("Built-in default URL should always be valid")
});
/// The default endpoint for mapping requests.
pub const DEFAULT_ENDPOINT_MAPPING: &str = "mapping";
/// The default endpoint for search requests.
pub const DEFAULT_ENDPOINT_SEARCH: &str = "search";
/// The default endpoint for filter requests.
pub const DEFAULT_ENDPOINT_FILTER: &str = "filter";

/// API key environment variable name
static API_KEY: LazyLock<Option<String>> = LazyLock::new(|| std::env::var("OPENFIGI_API_KEY").ok());
