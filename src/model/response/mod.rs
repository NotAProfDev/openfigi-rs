//! Response data structures for OpenFIGI API endpoints.
//!
//! This module contains all the response types returned by the OpenFIGI API endpoints.
//! Each endpoint has its own dedicated response module with specialized data structures
//! that handle the specific format and semantics of that endpoint's responses.
//!
//! # Available Response Types
//!
//! ## [`FilterData`]
//! Response from the `/v3/filter` endpoint for structured filtering of financial
//! instruments using specific criteria. Returns FIGI results with optional pagination
//! and total count information.
//!
//! ## [`MappingData`]
//! Response from the `/v3/mapping` endpoint for converting third-party identifiers
//! (tickers, ISINs, CUSIPs) into FIGI identifiers. Returns an array of results
//! corresponding to each mapping request in the batch.
//!
//! ## [`SearchData`]
//! Response from the `/v3/search` endpoint for text-based searches of financial
//! instruments. Returns FIGI results ordered by relevance with optional pagination.
//!
//! # Common Patterns
//!
//! All response types follow consistent patterns:
//! - Use the `ResponseResult` enum to handle success/error cases
//! - Support serialization/deserialization with serde
//! - Provide pagination support where applicable

mod common;
pub(crate) use self::common::ResponseResult;

mod mapping_response;
pub use self::mapping_response::MappingData;

mod search_response;
pub use self::search_response::SearchData;

mod filter_response;
pub use self::filter_response::FilterData;
