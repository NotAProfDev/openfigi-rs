//! Response models for the OpenFIGI search endpoint.
//!
//! This module contains the data structures used to represent responses from the
//! [OpenFIGI search endpoint](https://www.openfigi.com/api/documentation#v3-post-search).
//! The search endpoint allows finding financial instruments using text-based queries
//! and returns either successful FIGI results with optional pagination or error responses.
//!
//! # Response Structure
//!
//! The search endpoint returns either:
//! - [`SearchData`] containing successful FIGI results with optional pagination metadata
//! - [`crate::error::OpenFIGIError`] when the filter request fails
//!
//! # Examples
//!
//! ```rust
//! use openfigi_rs::model::response::SearchData;
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
//! let response: SearchData = serde_json::from_str(json).unwrap();
//! assert_eq!(response.data().len(), 2);
//! assert!(response.next_page().is_some());
//! ```
//!
//! Note: This module is not intended for direct use by consumers of the OpenFIGI API.

use crate::model::response::common::FigiResult;
use serde::{Deserialize, Serialize};

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

impl SearchData {
    /// Returns a slice of the FIGI results contained in this search response.
    ///
    /// Provides access to the financial instrument data returned by the search endpoint.
    #[must_use]
    pub fn data(&self) -> &[FigiResult] {
        &self.data
    }

    /// Returns the pagination token for retrieving the next page of search results.
    ///
    /// Returns `Some(token)` if more search results are available, `None` if this is the last page.
    /// The token can be used in subsequent search requests to continue pagination.
    #[must_use]
    pub fn next_page(&self) -> Option<&str> {
        self.next.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::response::ResponseResult, test_utils::load_test_data};

    /// Type alias for the search response used in tests
    pub type SearchResponse = ResponseResult<SearchData>;

    #[test]
    fn test_deserialize_query_example() {
        let json_str = load_test_data("search", "query_example.json");
        let search_response: SearchResponse =
            serde_json::from_str(&json_str).expect("Failed to deserialize search response");

        let search_data = match search_response {
            ResponseResult::Success(ref data) => data,
            ResponseResult::Error(ref err) => panic!("Expected success, got error: {err:?}"),
        };
        let figi_result = search_data.data();
        assert!(!figi_result.is_empty());

        // Check first IBM entry
        let first_entry = &figi_result[0];
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
        assert!(search_data.next_page().is_some());
    }

    #[test]
    fn test_deserialize_no_data() {
        let json_str = load_test_data("search", "no_data.json");
        let search_response: SearchResponse =
            serde_json::from_str(&json_str).expect("Failed to deserialize search response");

        let figi_result = match search_response {
            ResponseResult::Success(ref data) => data.data(),
            ResponseResult::Error(ref err) => panic!("Expected success, got error: {err:?}"),
        };
        assert!(figi_result.is_empty());
    }
}
