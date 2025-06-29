//! Response models for the OpenFIGI filter endpoint.
//!
//! This module contains the data structures used to represent responses from the
//! [OpenFIGI filter endpoint](https://www.openfigi.com/api/documentation#v3-post-filter).
//! The filter endpoint allows searching for financial instruments using various criteria
//! and returns either successful FIGI results with pagination support or error responses.
//!
//! # Response Structure
//!
//! The filter endpoint returns either:
//! - [`FilterData`] containing successful results with FIGI data and pagination metadata
//! - [`crate::error::OpenFIGIError`] when the filter request fails
//!
//! # Examples
//!
//! ```rust
//! use openfigi_rs::model::response::FilterData;
//! use serde_json;
//!
//! // Parsing a successful filter response
//! let json = r#"{
//!     "data": [{"figi": "BBG000BLNNH6", "ticker": "AAPL"}],
//!     "total": 1,
//!     "next": null
//! }"#;
//! let response: FilterData = serde_json::from_str(json).unwrap();
//! assert_eq!(response.data().len(), 1);
//! assert!(response.next_page().is_none());
//! ```
//!
//! Note: This module is not intended for direct use by consumers of the OpenFIGI API.

use crate::model::response::common::FigiResult;
use serde::{Deserialize, Serialize};

/// Successful filter result data containing FIGI results and pagination metadata.
///
/// This structure represents the payload of a successful filter request, containing
/// the actual FIGI data along with pagination information. The filter endpoint can
/// return large result sets, so pagination is supported through the `next` token.
///
/// # Field Descriptions
///
/// - `data`: Array of FIGI results matching the filter criteria
/// - `next`: Optional pagination token for retrieving the next page of results
/// - `total`: Total number of results available (across all pages)
///
/// # Pagination
///
/// When the result set is large, the API may return only a subset of results along
/// with a `next` token. This token can be used in subsequent requests to retrieve
/// additional pages of results.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilterData {
    /// Array of FIGI results matching the filter criteria.
    ///
    /// Each element contains detailed information about a financial instrument
    /// that matches the submitted filter parameters. The array may be empty
    /// if no instruments match the criteria.
    pub data: Vec<FigiResult>,

    /// Pagination token for retrieving the next page of results.
    ///
    /// This field is present when there are more results available beyond
    /// the current page. Use this token in subsequent filter requests to
    /// retrieve additional results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,

    /// Total number of results available across all pages.
    ///
    /// This count represents the complete result set size, not just the
    /// number of results in the current page. Useful for implementing
    /// pagination controls and progress indicators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<usize>,
}

impl FilterData {
    /// Returns a slice of the FIGI results contained in this mapping response.
    ///
    /// Provides access to the financial instrument data returned for the mapping request.
    #[must_use]
    pub fn data(&self) -> &[FigiResult] {
        &self.data
    }

    /// Returns the pagination token for retrieving the next page of results.
    ///
    /// Returns `Some(token)` if more results are available, `None` if this is the last page.
    /// The token can be used in subsequent search requests to continue pagination.
    #[must_use]
    pub fn next_page(&self) -> Option<&str> {
        self.next.as_deref()
    }

    /// Returns the total number of results available across all pages.
    ///
    /// The filter endpoint almost always provides the total count, so this method
    /// returns `Some(total)` rather than `None`.
    #[must_use]
    pub fn total_results(&self) -> Option<&usize> {
        self.total.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::response::common::ResponseResult;
    use std::fs;

    type FilterResponse = ResponseResult<FilterData>;

    /// Helper function to load test data from the tests/data/filter directory
    fn load_test_data(filename: &str) -> String {
        let path = format!("tests/data/filter/{filename}");
        fs::read_to_string(&path).unwrap_or_else(|e| panic!("Failed to read test file {path}: {e}"))
    }

    #[test]
    fn test_deserialize_simple_example() {
        let json_str = load_test_data("simple_example.json");
        let filter_response: FilterResponse = serde_json::from_str(&json_str).unwrap();

        let filter_data = match filter_response {
            ResponseResult::Success(ref data) => data,
            ResponseResult::Error(ref err) => panic!("Expected success, got error: {err:?}"),
        };
        let figi_result = filter_data.data();
        assert!(!figi_result.is_empty());

        // Check first IBM entry
        let first_entry = &figi_result[0];
        assert_eq!(first_entry.figi, "BBG0001RT9P0");
        assert_eq!(first_entry.ticker, Some("A 07/18/09 P12.5".to_string()));
        assert_eq!(first_entry.display_name(), "July 09 Puts on A US");

        // Check if composite_figi and share_class_figi exists
        assert!(first_entry.has_composite_figi());
        assert!(!first_entry.has_share_class_figi());

        // Verify actual field values from real data
        assert_eq!(first_entry.composite_figi, Some("BBG0001RT9P0".to_string()));
        assert_eq!(first_entry.share_class_figi, None);

        // Verify pagination exists
        assert!(filter_data.next_page().is_some());
        assert_eq!(
            filter_data.next_page().unwrap(),
            "QW9Fc1FrSkhNREF3TVZKVVJGY3ogMQ==.wAoXs2FMDgSubHmn4eCvQjx6pvAIM4KU8g7zWH5N0cw="
        );
        assert_eq!(filter_data.total_results(), Some(59_884_674).as_ref());
    }

    #[test]
    fn test_deserialize_no_data() {
        let json_str = load_test_data("no_data.json");
        let filter_response: FilterResponse = serde_json::from_str(&json_str).unwrap();

        let filter_data = match filter_response {
            ResponseResult::Success(ref data) => data,
            ResponseResult::Error(ref err) => panic!("Expected success, got error: {err:?}"),
        };
        let figi_result = filter_data.data();
        assert!(figi_result.is_empty());
        assert!(filter_data.next_page().is_none());
        assert_eq!(filter_data.total_results(), Some(0).as_ref());
    }
}
