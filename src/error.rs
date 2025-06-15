//! Error handling types for OpenFIGI API operations.
//!
//! This module provides comprehensive error types and utilities for handling failures
//! that can occur during OpenFIGI API interactions. The main error type [`OpenFIGIError`]
//! unifies different error sources into a single, easy-to-handle enum.
//!
//! ## Error Categories
//!
//! [`OpenFIGIError`] covers all possible failure scenarios:
//!
//! - **Network errors**: Connection failures, timeouts, DNS resolution issues
//! - **HTTP errors**: Status codes (400, 401, 404, 429, 500, etc.) with detailed context
//! - **Parsing errors**: JSON deserialization failures and malformed responses
//! - **Middleware errors**: Retry policy exhaustion, request building failures
//! - **URL errors**: Invalid URL formation and parsing issues
//! - **IO errors**: File system operations (for caching, logging, etc.)
//!
//! ## Error Inspection
//!
//! Use convenient inspection methods to categorize errors without pattern matching:
//!
//! ```rust
//! use openfigi_rs::error::OpenFIGIError;
//!
//! fn handle_error(err: OpenFIGIError) {
//!     if err.is_status() {
//!         if let Some(status) = err.status() {
//!             eprintln!("HTTP error: {status}");
//!         }
//!     } else if err.is_timeout() {
//!         eprintln!("Request timed out - consider retry");
//!     } else if err.is_connect() {
//!         eprintln!("Connection failed - check network");
//!     }
//! }
//! ```
//!
//! ## Error Conversion
//!
//! Common error types are automatically converted via `From` implementations:
//!
//! ```compile_fail
//! use openfigi_rs::error::OpenFIGIError;
//!
//! // These conversions happen automatically
//! let reqwest_err: reqwest::Error = /* ... */;
//! let openfigi_err: OpenFIGIError = reqwest_err.into();
//!
//! let json_err: serde_json::Error = /* ... */;
//! let openfigi_err: OpenFIGIError = json_err.into();
//! ```

use std::{error, fmt};
use url::Url;

/// Type alias for `Result<T, OpenFIGIError>`.
///
/// Convenience type used throughout the crate for consistent error handling.
pub type Result<T> = std::result::Result<T, OpenFIGIError>;

/// Main error type for all OpenFIGI API operations.
///
/// This enum unifies all possible error types that can occur during OpenFIGI API
/// interactions, providing a single error type for consistent handling across
/// the entire crate. Each variant wraps a specific error type while maintaining
/// the original error information.
///
/// ## Design Philosophy
///
/// Rather than requiring consumers to handle multiple error types, `OpenFIGIError`
/// provides a unified interface with convenient inspection methods. This allows
/// for both simple error handling (treat all errors the same) and sophisticated
/// error handling (inspect specific error types).
///
/// ## Inspection Methods
///
/// The error type provides numerous `is_*()` methods to check error categories
/// without pattern matching on variants. This makes error handling more ergonomic
/// and future-proof as new error variants can be added without breaking existing code.
///
/// # Examples
///
/// ```rust
/// use openfigi_rs::error::OpenFIGIError;
///
/// async fn handle_request_error(err: OpenFIGIError) {
///     match err.status() {
///         Some(status) if status.is_client_error() => {
///             eprintln!("Client error {}: check request parameters", status);
///         }
///         Some(status) if status.is_server_error() => {
///             eprintln!("Server error {}: retry may help", status);
///         }
///         None if err.is_timeout() => {
///             eprintln!("Request timeout: retry with backoff");
///         }
///         None if err.is_connect() => {
///             eprintln!("Connection error: check network connectivity");
///         }
///         _ => {
///             eprintln!("Other error: {}", err);
///         }
///     }
/// }
/// ```
#[derive(Debug)]
pub enum OpenFIGIError {
    /// HTTP client error from the underlying reqwest library.
    ///
    /// Includes network issues, timeout errors, connection failures,
    /// and other HTTP-level problems.
    ReqwestError(reqwest::Error),

    /// Middleware stack error from reqwest-middleware.
    ///
    /// Occurs when middleware components (retry policies, logging, etc.)
    /// fail or when the middleware stack itself encounters issues.
    ReqwestMiddlewareError(reqwest_middleware::Error),

    /// URL parsing error when constructing request URLs.
    ///
    /// Typically indicates malformed base URLs or invalid URL components.
    UrlParseError(url::ParseError),

    /// JSON serialization or deserialization error.
    ///
    /// Occurs when request payloads cannot be serialized or when
    /// response bodies cannot be parsed as valid JSON.
    SerdeError(serde_json::Error),

    /// File system I/O error for operations like caching or logging.
    ///
    /// May occur during file-based operations if implemented in the future.
    IoError(std::io::Error),

    /// HTTP response error with detailed status and content information.
    ///
    /// Contains structured error information from the OpenFIGI API,
    /// including status codes and response body content.
    ResponseError(ResponseContent),

    /// Miscellaneous application-specific errors.
    ///
    /// Used for validation errors and other issues that don't fit
    /// into the other categories.
    OtherError {
        /// Error classification
        kind: OtherErrorKind,
        /// Error description
        message: String,
    },
}

/// HTTP response error details.
///
/// Contains status code, optional message, and response body content
/// for detailed error analysis and debugging.
#[derive(Debug, Clone)]
pub struct ResponseContent {
    /// HTTP status code
    pub status: reqwest::StatusCode,
    /// Additional error context message
    pub message: Option<String>,
    /// Raw response body content
    pub content: String,
}

/// Classification for miscellaneous errors that don't fit other categories.
///
/// This enum provides additional categorization for application-specific
/// errors that aren't covered by the main error variants.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum OtherErrorKind {
    /// Request validation errors.
    ///
    /// Indicates that request parameters failed validation before
    /// being sent to the API.
    Validation,
    /// Unclassified errors.
    ///
    /// Catch-all category for errors that don't fit other classifications.
    Other,
}

impl fmt::Display for OpenFIGIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Most common errors first for better branch prediction
            Self::ReqwestError(e) => write!(f, "error in reqwest: {e}"),
            Self::ResponseError(e) => {
                write!(f, "error in response: status code {}", e.status)
            }
            Self::SerdeError(e) => write!(f, "error in serde: {e}"),
            Self::ReqwestMiddlewareError(e) => {
                write!(f, "error in reqwest-middleware: {e}")
            }
            Self::UrlParseError(e) => write!(f, "error in url: {e}"),
            Self::IoError(e) => write!(f, "error in IO: {e}"),
            Self::OtherError { kind, message } => {
                write!(f, "error in other: {kind:?}: {message}")
            }
        }
    }
}

impl error::Error for OpenFIGIError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::ReqwestError(e) => Some(e),
            Self::ReqwestMiddlewareError(e) => Some(e),
            Self::SerdeError(e) => Some(e),
            Self::IoError(e) => Some(e),
            Self::UrlParseError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for OpenFIGIError {
    fn from(e: reqwest::Error) -> Self {
        Self::ReqwestError(e)
    }
}

impl From<reqwest_middleware::Error> for OpenFIGIError {
    fn from(e: reqwest_middleware::Error) -> Self {
        Self::ReqwestMiddlewareError(e)
    }
}

impl From<url::ParseError> for OpenFIGIError {
    fn from(e: url::ParseError) -> Self {
        Self::UrlParseError(e)
    }
}

impl From<serde_json::Error> for OpenFIGIError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeError(e)
    }
}

impl From<std::io::Error> for OpenFIGIError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl OpenFIGIError {
    /// Returns the URL associated with this error, if available.
    ///
    /// Provides access to the request URL for errors that occurred during
    /// HTTP operations. Useful for debugging and logging.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::error::OpenFIGIError;
    ///
    /// fn log_error_with_url(err: &OpenFIGIError) {
    ///     if let Some(url) = err.url() {
    ///         eprintln!("Error occurred for URL: {}", url);
    ///     }
    /// }
    /// ```
    #[must_use]
    pub fn url(&self) -> Option<&Url> {
        match self {
            Self::ReqwestError(inner) => inner.url(),
            Self::ReqwestMiddlewareError(inner) => inner.url(),
            _ => None,
        }
    }

    /// Returns a mutable reference to the URL for this error.
    ///
    /// Useful for removing sensitive information from URLs before logging
    /// or displaying errors to users.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::error::OpenFIGIError;
    ///
    /// fn sanitize_error_url(mut err: OpenFIGIError) -> OpenFIGIError {
    ///     if let Some(url) = err.url_mut() {
    ///         url.set_query(None); // Remove query parameters
    ///     }
    ///     err
    /// }
    /// ```
    #[must_use]
    pub fn url_mut(&mut self) -> Option<&mut Url> {
        match self {
            Self::ReqwestError(inner) => inner.url_mut(),
            Self::ReqwestMiddlewareError(inner) => inner.url_mut(),
            _ => None,
        }
    }

    /// Returns a new error with the specified URL attached.
    ///
    /// Attaches URL information to errors that support it. Only applies
    /// to reqwest and middleware errors; other error types are returned unchanged.
    #[must_use]
    pub fn with_url(self, url: Url) -> Self {
        match self {
            Self::ReqwestError(inner) => Self::ReqwestError(inner.with_url(url)),
            Self::ReqwestMiddlewareError(inner) => {
                Self::ReqwestMiddlewareError(inner.with_url(url))
            }
            // Not applicable for other variants
            _ => self,
        }
    }

    /// Returns an error with the URL removed for security purposes.
    ///
    /// Removes URL information from errors that contain it. Useful when
    /// URLs might contain sensitive information that shouldn't be logged.
    #[must_use]
    pub fn without_url(self) -> Self {
        match self {
            Self::ReqwestError(inner) => Self::ReqwestError(inner.without_url()),
            Self::ReqwestMiddlewareError(inner) => {
                Self::ReqwestMiddlewareError(inner.without_url())
            }
            // Not applicable for other variants
            _ => self,
        }
    }

    /// Returns true if this error originated from middleware.
    ///
    /// Identifies errors that occurred within the middleware stack,
    /// such as retry policy exhaustion or middleware-specific failures.
    #[must_use]
    pub fn is_middleware(&self) -> bool {
        match self {
            Self::ReqwestMiddlewareError(inner) => inner.is_middleware(),
            // Not applicable for other variants
            _ => false,
        }
    }

    /// Returns true if this error originated from the builder methods.
    #[must_use]
    pub fn is_builder(&self) -> bool {
        match self {
            Self::ReqwestError(inner) => inner.is_builder(),
            Self::ReqwestMiddlewareError(inner) => inner.is_builder(),
            // Not applicable for other variants
            _ => false,
        }
    }

    /// Returns true if this error is a redirect error.
    ///
    /// Identifies errors related to HTTP redirects, such as too many redirects
    /// or redirect loops.
    #[must_use]
    pub fn is_redirect(&self) -> bool {
        match self {
            Self::ReqwestError(inner) => inner.is_redirect(),
            Self::ReqwestMiddlewareError(inner) => inner.is_redirect(),
            // Not applicable for other variants
            _ => false,
        }
    }

    /// Returns true if this error is a status error.
    ///
    /// Indicates errors that contain HTTP status codes, either from reqwest
    /// or from explicit response errors.
    #[must_use]
    pub fn is_status(&self) -> bool {
        match self {
            Self::ReqwestError(inner) => inner.is_status(),
            Self::ReqwestMiddlewareError(inner) => inner.is_status(),
            Self::ResponseError(_) => true,
            // Not applicable for other variants
            _ => false,
        }
    }

    /// Returns true if this error is a timeout error.
    ///
    /// Indicates that the HTTP request exceeded the configured timeout period.
    /// This can help distinguish between connection issues and slow responses.
    #[must_use]
    pub fn is_timeout(&self) -> bool {
        match self {
            Self::ReqwestError(inner) => inner.is_timeout(),
            Self::ReqwestMiddlewareError(inner) => inner.is_timeout(),
            // Not applicable for other variants
            _ => false,
        }
    }

    /// Returns true if this error is a request error.
    ///
    /// Indicates errors that occurred during request processing,
    /// such as malformed request data or invalid parameters.
    #[must_use]
    pub fn is_request(&self) -> bool {
        match self {
            Self::ReqwestError(inner) => inner.is_request(),
            Self::ReqwestMiddlewareError(inner) => inner.is_request(),
            // Not applicable for other variants
            _ => false,
        }
    }

    /// Returns true if this error is a connection error.
    ///
    /// Indicates network-level connection failures, such as DNS resolution
    /// problems, connection refused, or network unreachable errors.
    #[must_use]
    pub fn is_connect(&self) -> bool {
        match self {
            Self::ReqwestError(inner) => inner.is_connect(),
            Self::ReqwestMiddlewareError(inner) => inner.is_connect(),
            // Not applicable for other variants
            _ => false,
        }
    }

    /// Returns true if this error is related to the request or response body.
    ///
    /// Identifies errors that occurred during body processing, such as
    /// reading response bodies or serializing request payloads.
    #[must_use]
    pub fn is_body(&self) -> bool {
        match self {
            Self::ReqwestError(inner) => inner.is_body(),
            Self::ReqwestMiddlewareError(inner) => inner.is_body(),
            // Not applicable for other variants
            _ => false,
        }
    }

    /// Returns true if this error is a decode error.
    ///
    /// Indicates errors that occurred during response deserialization or
    /// other data decoding operations. Includes JSON parsing failures
    /// and format conversion errors.
    #[must_use]
    pub fn is_decode(&self) -> bool {
        match self {
            Self::ReqwestError(inner) => inner.is_decode(),
            Self::ReqwestMiddlewareError(inner) => inner.is_decode(),
            Self::OtherError { .. } => true,
            // Not applicable for other variants
            _ => false,
        }
    }

    /// Returns the HTTP status code associated with this error, if available.
    ///
    /// Extracts the HTTP status code from errors that contain one, such as
    /// reqwest errors with status information or explicit response errors.
    /// Returns `None` for errors that don't have an associated status code.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use openfigi_rs::error::OpenFIGIError;
    ///
    /// fn handle_status_error(err: &OpenFIGIError) {
    ///     if let Some(status) = err.status() {
    ///         match status.as_u16() {
    ///             400 => eprintln!("Bad request - check parameters"),
    ///             401 => eprintln!("Unauthorized - check API key"),
    ///             429 => eprintln!("Rate limited - retry later"),
    ///             500..=599 => eprintln!("Server error - retry may help"),
    ///             _ => eprintln!("HTTP error: {}", status),
    ///         }
    ///     }
    /// }
    /// ```
    #[must_use]
    pub fn status(&self) -> Option<reqwest::StatusCode> {
        match self {
            Self::ReqwestError(inner) => inner.status(),
            Self::ReqwestMiddlewareError(inner) => inner.status(),
            Self::ResponseError(resp) => Some(resp.status),
            // Not applicable for other variants
            _ => None,
        }
    }

    /// Creates a new `ResponseError` with the given parameters.
    ///
    /// This is an internal constructor used by the client to create response errors
    /// with structured information about HTTP failures.
    ///
    /// # Arguments
    ///
    /// * `status` - HTTP status code from the response
    /// * `content` - Raw response body content
    /// * `message` - Optional additional error context message
    pub(crate) fn response_error(
        status: reqwest::StatusCode,
        content: impl Into<String>,
        message: Option<impl Into<String>>,
    ) -> Self {
        Self::ResponseError(ResponseContent {
            status,
            content: content.into(),
            message: message.map(Into::into),
        })
    }

    /// Creates a new `OtherError` with the given kind and message.
    ///
    /// This is an internal constructor for application-specific errors that
    /// don't fit into the other error categories.
    ///
    /// # Arguments
    ///
    /// * `kind` - Classification of the error type
    /// * `message` - Descriptive error message
    pub(crate) fn other_error(kind: OtherErrorKind, message: impl Into<String>) -> Self {
        Self::OtherError {
            kind,
            message: message.into(),
        }
    }
}
