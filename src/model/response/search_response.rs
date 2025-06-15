//! Response models for the OpenFIGI search endpoint.
//!
//! This module contains the data structures used to represent responses from the
//! [OpenFIGI search endpoint](https://www.openfigi.com/api/documentation#v3-post-search).
//! The search endpoint allows finding financial instruments using text-based queries
//! and returns either successful FIGI results with optional pagination or error responses.
//!
//! # Response Structure
//!
//! The search endpoint returns a [`SearchResponse`] which wraps either:
//! - [`SearchData`] containing successful FIGI results with optional pagination support
//! - Error information when the search query fails or produces no results
//!
//! # Examples
//!
//! ```rust
//! use openfigi_rs::model::response::SearchResponse;
//! use serde_json;
//!
//! // Successful search response with results
//! let json = r#"{
//!     "data": [
//!         {"figi": "BBG000BLNNH6", "ticker": "AAPL", "name": "Apple Inc"},
//!         {"figi": "BBG000B9XRY4", "ticker": "TSLA", "name": "Tesla Inc"}
//!     ],
//!     "next": "pagination_token_here"
//! }"#;
//! let response: SearchResponse = serde_json::from_str(json).unwrap();
//! assert!(response.is_success());
//! assert_eq!(response.data().unwrap().len(), 2);
//! assert!(response.next_page().is_some());
//!
//! // Error response when search fails
//! let error_json = r#"{"error": "Invalid search query"}"#;
//! let error_response: SearchResponse = serde_json::from_str(error_json).unwrap();
//! assert!(error_response.is_error());
//! ```
//!
//! Note: This module is not intended for direct use by consumers of the OpenFIGI API.

use crate::model::response::common::{FigiData, FigiResult, ResponseResult};
use serde::{Deserialize, Serialize};

/// Response type for the OpenFIGI search endpoint (POST /v3/search).
///
/// This type alias represents the complete response from the search endpoint, which can
/// contain either successful search results or error information. The search endpoint
/// allows finding financial instruments using free-text queries and supports pagination
/// for large result sets.
///
/// # Response Format
///
/// Results are generally sorted by relevance to the search query.
///
/// Successful responses contain:
/// - An array of FIGI results matching the search criteria
/// - Optional pagination token for retrieving additional results
///
/// Error responses contain:
/// - A descriptive error message explaining why the search request failed
///
/// # Examples
///
/// ```rust
/// use openfigi_rs::model::response::SearchResponse;
/// use serde_json;
///
/// // Successful search with multiple results
/// let success_json = r#"{
///     "data": [
///         {
///             "figi": "BBG000BLNNH6",
///             "ticker": "AAPL",
///             "name": "Apple Inc",
///             "marketSector": "Equity"
///         },
///         {
///             "figi": "BBG000BVPV84",
///             "ticker": "AAPL",
///             "name": "Apple Inc",
///             "marketSector": "Equity"
///         }
///     ],
///     "next": "eyJwYWdlIjoxfQ=="
/// }"#;
/// let response: SearchResponse = serde_json::from_str(success_json).unwrap();
/// assert!(response.is_success());
/// assert_eq!(response.data().unwrap().len(), 2);
/// assert!(response.next_page().is_some());
///
/// // Empty search results
/// let empty_json = r#"{"data": []}"#;
/// let empty_response: SearchResponse = serde_json::from_str(empty_json).unwrap();
/// assert!(empty_response.is_success());
/// assert_eq!(empty_response.data().unwrap().len(), 0);
/// assert!(empty_response.next_page().is_none());
///
/// // Search error response
/// let error_json = r#"{"error": "Query too short"}"#;
/// let error_response: SearchResponse = serde_json::from_str(error_json).unwrap();
/// assert!(error_response.is_error());
/// assert_eq!(error_response.error(), Some("Query too short"));
/// ```
pub type SearchResponse = ResponseResult<SearchData>;

/// Successful search result containing FIGI data and optional pagination information.
///
/// This structure represents the payload returned when a search query successfully
/// finds matching financial instruments. The search endpoint can return multiple
/// FIGI results for a single query, especially when the search term matches multiple
/// instruments or variations of the same instrument across different exchanges.
///
/// # Field Descriptions
///
/// - `data`: Array of FIGI results matching the search query, ordered by relevance
/// - `next`: Optional pagination token for retrieving additional search results
///
/// # Pagination
///
/// When the result set is large, the API may return only a subset of results along
/// with a `next` token. This token can be used in subsequent requests to retrieve
/// additional pages of results.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchData {
    /// Array of FIGI results matching the search query.
    ///
    /// Contains financial instruments that match the submitted search criteria.
    /// Results are typically ordered by relevance, with the most likely matches
    /// appearing first. The array may be empty if no instruments match the search query.
    pub data: Vec<FigiResult>,

    /// Pagination token for retrieving the next page of search results.
    ///
    /// This field is present when there are more search results available beyond
    /// the current page. Use this token in subsequent search requests to retrieve
    /// additional results. When `None`, this indicates the last page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

impl FigiData for SearchData {
    /// Returns a slice of the FIGI results contained in this search response.
    ///
    /// Provides access to the financial instrument data returned by the search endpoint.
    /// This implements the [`FigiData`] trait to allow uniform access to FIGI data across
    /// different response types.
    fn figi_data(&self) -> &[FigiResult] {
        &self.data
    }

    /// Returns the pagination token for retrieving the next page of search results.
    ///
    /// Returns `Some(token)` if more search results are available, `None` if this is the last page.
    /// The token can be used in subsequent search requests to continue pagination.
    fn next_page(&self) -> Option<&str> {
        self.next.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    /// Helper function to load test data from the tests/data/search directory
    fn load_test_data(filename: &str) -> String {
        let path = format!("tests/data/search/{filename}");
        fs::read_to_string(&path).unwrap_or_else(|e| panic!("Failed to read test file {path}: {e}"))
    }

    #[test]
    fn test_deserialize_query_example() {
        let json_str = load_test_data("query_example.json");
        let result: SearchResponse = serde_json::from_str(&json_str).unwrap();

        assert!(result.is_success());
        let data = result.data().unwrap();
        assert!(!data.is_empty());

        // Check first IBM entry
        let first_entry = &data[0];
        assert_eq!(first_entry.figi, "BBG0002ZTPP5");
        assert_eq!(first_entry.ticker, Some("IBM 03/20/10 P130".to_string()));
        assert_eq!(first_entry.display_name(), "March 10 Puts on IBM US");

        // Check if composite_figi and share_class_figi exists
        assert!(first_entry.has_composite_figi());
        assert!(!first_entry.has_share_class_figi());

        // Verify actual field values from real data
        assert_eq!(first_entry.composite_figi, Some("BBG0002ZTPP5".to_string()));
        assert_eq!(first_entry.share_class_figi, None);

        // Verify pagination exists
        assert!(result.next_page().is_some());
    }

    #[test]
    fn test_deserialize_no_data() {
        let json_str = load_test_data("no_data.json");
        let result: SearchResponse = serde_json::from_str(&json_str).unwrap();

        assert!(result.is_success());
        let data = result.data().unwrap();
        assert!(data.is_empty());
    }
}
