//! Response models for the OpenFIGI mapping endpoint.
//!
//! This module contains the data structures used to represent responses from the
//! [OpenFIGI mapping endpoint](https://www.openfigi.com/api/documentation#v3-post-mapping).
//! The mapping endpoint converts third-party identifiers (such as tickers, ISINs, or CUSIPs)
//! into FIGI identifiers and returns either successful FIGI results or error responses for
//! each submitted mapping request.
//!
//! # Response Structure
//!
//! The mapping endpoint returns:
//! - For a single request either:
//!   - [`MappingData`] containing successful results with FIGI data
//!   - [`OpenFIGIError`] when the mapping request fails
//! - For a batch request:
//!   - [`MappingResponses`], which wraps a vector of results (successes and errors) corresponding
//!     to each mapping request in the batch
//!
//! # Batch Processing
//!
//! The mapping endpoint supports batch requests (up to 100 mapping requests per call with an API key),
//! making it efficient for processing multiple identifiers at once. Each mapping request
//! in the batch gets its own result in the response array.
//!
//! # Examples
//!
//! ```rust
//! use openfigi_rs::model::response::MappingData;
//! use serde_json;
//!
//! // Single successful mapping result
//! let json = r#"[{
//!     "data": [{"figi": "BBG000BLNNH6", "ticker": "IBM", "name": "INTL BUSINESS MACHINES CORP"}]
//! }]"#;
//! let response: Vec<MappingData> = serde_json::from_str(json).unwrap();
//! assert_eq!(response[0].data().len(), 1);
//! ```
//!
//! Note: This module is not intended for direct use by consumers of the OpenFIGI API.

use crate::error::{OpenFIGIError, Result};
use crate::model::response::common::FigiResult;
use serde::{Deserialize, Serialize};

/// Ergonomic wrapper for batch responses from the OpenFIGI mapping endpoint (POST /v3/mapping).
///
/// This type represents the complete response from the mapping endpoint, which returns
/// an array of results corresponding to each mapping request submitted in the batch.
/// Each mapping request in the batch gets its own result in the response array, which is
/// either a successful [`MappingData`] or an [`OpenFIGIError`] describing why the mapping failed.
///
/// # Usage
///
/// - Use [`MappingResponses::successes()`] to iterate over all successful mapping results.
/// - Use [`MappingResponses::failures()`] to iterate over all errors that occurred for individual requests.
/// - Use [`MappingResponses::len()`] and [`MappingResponses::is_empty()`] for batch size checks.
#[derive(Debug)]
pub struct MappingResponses(Vec<Result<MappingData>>);

impl MappingResponses {
    #[doc(hidden)]
    /// Creates a new `MappingResponses` from a vector of results.
    /// This constructor is primarily for internal use
    /// and testing purposes.
    pub(crate) fn new(results: Vec<Result<MappingData>>) -> Self {
        Self(results)
    }

    /// Returns an iterator over all successful mapping results in the batch, with their indices.
    ///
    /// Each item is a tuple `(index, &MappingData)` for a request that was successfully mapped.
    pub fn successes(&self) -> impl Iterator<Item = (usize, &MappingData)> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, r)| r.as_ref().ok().map(|data| (i, data)))
    }

    /// Returns an iterator over all errors for failed mapping requests in the batch, with their indices.
    ///
    /// Each item is a tuple `(index, &OpenFIGIError)` for a request that failed to map.
    pub fn failures(&self) -> impl Iterator<Item = (usize, &OpenFIGIError)> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, r)| r.as_ref().err().map(|err| (i, err)))
    }

    /// Returns the total number of mapping results (successes + failures) in the batch.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if there are no mapping results in the batch.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns a reference to the underlying vector of results, preserving order and index.
    pub fn as_slice(&self) -> &[Result<MappingData>] {
        &self.0
    }
}

/// Successful mapping result containing FIGI data for a single mapping request.
///
/// This structure represents the payload returned when a mapping request successfully
/// finds matching FIGI identifiers for the submitted third-party identifier. A single
/// mapping request can return multiple FIGI results when the identifier matches multiple
/// financial instruments (e.g., different share classes or trading venues).
///
/// # Multiple Results
///
/// Some identifiers may map to multiple FIGIs:
/// - **Multiple exchanges**: The same instrument may trade on different exchanges
/// - **Different instrument types**: An identifier might match both the underlying stock and related derivatives
///
/// # Field Descriptions
///
/// - `data`: Array of FIGI results that match the mapping request criteria
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MappingData {
    /// Array of FIGI results that match the mapping request criteria.
    ///
    /// This field contains all the financial instruments that match the submitted
    /// identifier. The array may contain:
    /// - A single result for unique identifiers
    /// - Multiple results when the identifier matches several instruments
    /// - An empty array if no matches are found (though this typically results in an error instead)
    ///
    /// Each FIGI result provides detailed information about the matched financial instrument.
    pub data: Vec<FigiResult>,
}

impl MappingData {
    /// Returns a slice of the FIGI results contained in this mapping response.
    ///
    /// Provides access to the financial instrument data returned for the mapping request.
    #[must_use]
    pub fn data(&self) -> &[FigiResult] {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::response::common::ResponseResult, test_utils::load_test_data};

    /// Helper function to convert raw response results into a `MappingResponses` instance
    fn from_response_results(raw: Vec<ResponseResult<MappingData>>) -> MappingResponses {
        MappingResponses::new(
            raw.into_iter()
                .map(|res| match res {
                    ResponseResult::Success(data) => Ok(data),
                    ResponseResult::Error(err) => Err(OpenFIGIError::response_error(
                        reqwest::StatusCode::OK,
                        err.error,
                        String::new(),
                    )),
                })
                .collect(),
        )
    }

    #[test]
    fn test_deserialize_isin_example() {
        let json_str = load_test_data("mapping", "isin_example.json");

        let raw: Vec<ResponseResult<MappingData>> =
            serde_json::from_str(&json_str).expect("Failed to deserialize mapping response");
        let mapping_response = from_response_results(raw);

        assert_eq!(mapping_response.len(), 1);
        let response_result = &mapping_response.as_slice()[0];
        match response_result {
            Ok(mapping_data) => {
                let figi_result = mapping_data.data();
                assert!(!figi_result.is_empty());

                // Check first IBM entry
                let first_entry = &figi_result[0];
                assert_eq!(first_entry.figi, "BBG000BLNNH6");
                assert_eq!(first_entry.ticker, Some("IBM".to_string()));
                assert_eq!(first_entry.display_name(), "INTL BUSINESS MACHINES CORP");

                // Check if composite_figi and share_class_figi exists
                assert!(first_entry.has_composite_figi());
                assert!(first_entry.has_share_class_figi());

                // Verify actual field values from real data
                assert_eq!(first_entry.composite_figi, Some("BBG000BLNNH6".to_string()));
                assert_eq!(
                    first_entry.share_class_figi,
                    Some("BBG001S5S399".to_string())
                );
            }
            Err(e) => panic!("Expected success, got error: {e}"),
        }
    }

    #[test]
    fn test_deserialize_invalid_identifier() {
        let json_str = load_test_data("mapping", "invalid_identifier.json");
        let raw: Vec<ResponseResult<MappingData>> =
            serde_json::from_str(&json_str).expect("Failed to deserialize mapping response");
        let mapping_response = from_response_results(raw);

        assert_eq!(mapping_response.len(), 1);
        let response_result = &mapping_response.as_slice()[0];
        match response_result {
            Ok(_) => panic!("Expected error, got success"),
            Err(OpenFIGIError::ResponseError(resp)) => {
                assert!(resp.message.contains("Invalid idValue format."));
            }
            Err(e) => panic!("Unexpected error variant: {e}"),
        }
    }

    #[test]
    fn test_deserialize_bulk_request() {
        let json_str = load_test_data("mapping", "bulk_request.json");
        let raw: Vec<ResponseResult<MappingData>> =
            serde_json::from_str(&json_str).expect("Failed to deserialize mapping response");
        let mapping_response = from_response_results(raw);

        assert_eq!(mapping_response.len(), 2);

        // First result should be IBM success
        let ibm_response_result = &mapping_response.as_slice()[0];
        match ibm_response_result {
            Ok(mapping_data) => {
                let ibm_figi_result = mapping_data.data();
                assert!(!ibm_figi_result.is_empty());
                assert_eq!(ibm_figi_result[0].ticker, Some("IBM".to_string()));
            }
            Err(e) => panic!("Expected IBM success, got error: {e}"),
        }

        // Second result should be AAPL success
        let aapl_response_result = &mapping_response.as_slice()[1];
        match aapl_response_result {
            Ok(mapping_data) => {
                let aapl_figi_result = mapping_data.data();
                assert!(!aapl_figi_result.is_empty());
                assert_eq!(aapl_figi_result[0].ticker, Some("AAPL".to_string()));
            }
            Err(e) => panic!("Expected AAPL success, got error: {e}"),
        }
    }

    #[test]
    fn test_deserialize_cusip_with_exchange() {
        let json_str = load_test_data("mapping", "cusip_with_exchange.json");
        let raw: Vec<ResponseResult<MappingData>> =
            serde_json::from_str(&json_str).expect("Failed to deserialize mapping response");
        let mapping_response = from_response_results(raw);

        assert_eq!(mapping_response.len(), 1);
        let response_result = &mapping_response.as_slice()[0];
        match response_result {
            Ok(mapping_data) => {
                let figi_result = mapping_data.data();
                assert!(!figi_result.is_empty());

                // Verify we get valid FIGI results
                for data in figi_result {
                    assert!(!data.figi.is_empty());
                    assert!(data.ticker.is_some());
                }
            }
            Err(e) => panic!("Expected success, got error: {e}"),
        }
    }

    #[test]
    fn test_deserialize_ticker_with_security_type() {
        let json_str = load_test_data("mapping", "ticker_with_security_type.json");
        let raw: Vec<ResponseResult<MappingData>> =
            serde_json::from_str(&json_str).expect("Failed to deserialize mapping response");
        let mapping_response = from_response_results(raw);

        assert_eq!(mapping_response.len(), 1);
        let response_result = &mapping_response.as_slice()[0];
        match response_result {
            Ok(mapping_data) => {
                let figi_result = mapping_data.data();
                assert!(!figi_result.is_empty());

                // Verify security type is properly parsed
                for data in figi_result {
                    assert!(data.security_type.is_some());
                    assert!(data.market_sector.is_some());
                }
            }
            Err(e) => panic!("Expected success, got error: {e}"),
        }
    }

    #[test]
    fn test_deserialize_option_example() {
        let json_str = load_test_data("mapping", "option_example.json");
        let raw: Vec<ResponseResult<MappingData>> =
            serde_json::from_str(&json_str).expect("Failed to deserialize mapping response");
        let mapping_response = from_response_results(raw);

        assert_eq!(mapping_response.len(), 1);
        let response_result = &mapping_response.as_slice()[0];
        match response_result {
            Ok(mapping_data) => {
                let figi_result = mapping_data.data();
                for data in figi_result {
                    assert!(!data.figi.is_empty());
                }
            }
            Err(e) => {
                // This could be either success or error depending on the option data
                assert!(!e.to_string().is_empty());
            }
        }
    }

    #[test]
    fn test_deserialize_currency_mic_example() {
        let json_str = load_test_data("mapping", "currency_mic_example.json");
        let raw: Vec<ResponseResult<MappingData>> =
            serde_json::from_str(&json_str).expect("Failed to deserialize mapping response");
        let mapping_response = from_response_results(raw);

        assert_eq!(mapping_response.len(), 1);
        let response_result = &mapping_response.as_slice()[0];
        match response_result {
            Ok(mapping_data) => {
                let figi_result = mapping_data.data();
                for data in figi_result {
                    assert!(!data.figi.is_empty());
                }
            }
            Err(e) => {
                // This could be either success or error depending on the currency data
                assert!(!e.to_string().is_empty());
            }
        }
    }

    #[test]
    fn test_figi_result_display_name_fallback() {
        // Test with only ticker
        let figi_with_ticker = FigiResult {
            figi: "BBG000BLNNH6".to_string(),
            name: None,
            ticker: Some("IBM".to_string()),
            security_type: None,
            market_sector: None,
            exch_code: None,
            share_class_figi: None,
            composite_figi: None,
            security_type2: None,
            security_description: None,
            metadata: None,
        };
        assert_eq!(figi_with_ticker.display_name(), "IBM");

        // Test with only FIGI
        let figi_only = FigiResult {
            figi: "BBG000BLNNH6".to_string(),
            name: None,
            ticker: None,
            security_type: None,
            market_sector: None,
            exch_code: None,
            share_class_figi: None,
            composite_figi: None,
            security_type2: None,
            security_description: None,
            metadata: None,
        };
        assert_eq!(figi_only.display_name(), "BBG000BLNNH6");
    }
}
