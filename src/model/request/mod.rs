//! # Request Types
//!
//! Request structures and builders for OpenFIGI API endpoints.
//! Each request type provides validation and fluent builder patterns for constructing API requests.
//!
//! ## Available Request Types
//!
//! - [`FilterRequest`] - For `/filter` endpoint requests
//! - [`MappingRequest`] - For `/mapping` endpoint requests  
//! - [`SearchRequest`] - For `/search` endpoint requests

mod common;
mod macros;

mod mapping_request;
pub use self::mapping_request::{MappingRequest, MappingRequestBuilder};

mod search_request;
pub use self::search_request::{SearchRequest, SearchRequestBuilder};

mod filter_request;
pub use self::filter_request::{FilterRequest, FilterRequestBuilder};
