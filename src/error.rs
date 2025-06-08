//! # Error Types
//!
//! Comprehensive error handling types for API operations.
//!
//! ## Overview
//!
//! [`OpenFIGIError`] is the primary error type, covering:
//!
//! - **Network errors**: Connection failures, timeouts, DNS issues
//! - **HTTP errors**: Status codes (400, 401, 404, 500, etc.)
//! - **Parsing errors**: JSON deserialization failures
//! - **Middleware errors**: Retry policy exhaustion, request building issues
//! - **URL errors**: Invalid URL formation
//! - **IO errors**: File system operations
//!
//! ## Error Inspection
//!
//! Use helper methods to categorize errors without matching variants:
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
//!         eprintln!("Request timed out");
//!     } else if err.is_connect() {
//!         eprintln!("Connection failed");
//!     }
//! }
//! ```

use std::{error, fmt};
use url::Url;

/// Type alias for `Result<T, OpenFIGIError>`.
///
/// Convenience type used throughout the crate for consistent error handling.
pub type Result<T> = std::result::Result<T, OpenFIGIError>;

/// Main error type for all operations.
///
/// Provides unified error handling with convenient inspection methods
/// for categorizing different failure types without variant matching.
#[derive(Debug)]
pub enum OpenFIGIError {
    /// HTTP client error from reqwest
    ReqwestError(reqwest::Error),

    /// Middleware stack error from reqwest-middleware
    ReqwestMiddlewareError(reqwest_middleware::Error),

    /// URL parsing error
    UrlParseError(url::ParseError),

    /// JSON serialization/deserialization error
    SerdeError(serde_json::Error),

    /// File system I/O error
    IoError(std::io::Error),

    /// HTTP response error with status and content
    ResponseError(ResponseContent),

    /// Miscellaneous application errors
    OtherError {
        /// Error classification
        kind: OtherErrorKind,
        /// Error description
        message: Box<str>,
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
    pub message: Option<Box<str>>,
    /// Raw response body content
    pub content: Box<str>,
}

/// Classification for miscellaneous errors.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum OtherErrorKind {
    /// Request validation errors
    Validation,
    /// Unclassified errors
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
    /// Useful for removing sensitive information from URLs.
    #[must_use]
    pub fn url_mut(&mut self) -> Option<&mut Url> {
        match self {
            Self::ReqwestError(inner) => inner.url_mut(),
            Self::ReqwestMiddlewareError(inner) => inner.url_mut(),
            _ => None,
        }
    }

    /// Returns a new error with the specified URL attached.
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

    /// Returns an error with the URL removed for security.
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
    #[must_use]
    pub fn is_middleware(&self) -> bool {
        match self {
            Self::ReqwestMiddlewareError(inner) => inner.is_middleware(),
            // Not applicable for other variants
            _ => false,
        }
    }

    /// Returns true if this error originated from the builder.
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
    pub(crate) fn response_error(
        status: reqwest::StatusCode,
        content: impl Into<Box<str>>,
        message: Option<impl Into<Box<str>>>,
    ) -> Self {
        Self::ResponseError(ResponseContent {
            status,
            content: content.into(),
            message: message.map(Into::into),
        })
    }

    /// Creates a new `OtherError` with the given kind and message.
    pub(crate) fn other_error(kind: OtherErrorKind, message: impl Into<Box<str>>) -> Self {
        Self::OtherError {
            kind,
            message: message.into(),
        }
    }
}
