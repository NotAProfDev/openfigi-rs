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
//! The mapping endpoint returns a [`MappingResponse`] which is an array of results, where
//! each element corresponds to one mapping request in the original batch:
//! - [`MappingData`] containing successful FIGI results for valid identifiers
//! - Error information when mapping fails for invalid or unrecognized identifiers
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
//! use openfigi_rs::model::response::MappingResponse;
//! use serde_json;
//!
//! // Single successful mapping result
//! let json = r#"[{
//!     "data": [{"figi": "BBG000BLNNH6", "ticker": "IBM", "name": "INTL BUSINESS MACHINES CORP"}]
//! }]"#;
//! let response: MappingResponse = serde_json::from_str(json).unwrap();
//! assert_eq!(response.len(), 1);
//! assert!(response[0].is_success());
//!
//! // Batch request with mixed success/error results
//! let batch_json = r#"[
//!     {"data": [{"figi": "BBG000BLNNH6", "ticker": "IBM"}]},
//!     {"error": "Invalid ticker symbol"}
//! ]"#;
//! let batch_response: MappingResponse = serde_json::from_str(batch_json).unwrap();
//! assert_eq!(batch_response.len(), 2);
//! assert!(batch_response[0].is_success());
//! assert!(batch_response[1].is_error());
//! ```
//!
//! Note: This module is not intended for direct use by consumers of the OpenFIGI API.

use crate::model::response::common::{FigiData, FigiResult, ResponseResult};
use serde::{Deserialize, Serialize};

/// Response array from the OpenFIGI mapping endpoint (POST /v3/mapping).
///
/// This type represents the complete response from the mapping endpoint, which returns
/// an array of results corresponding to each mapping request submitted in the batch.
/// The mapping endpoint allows converting third-party identifiers (tickers, ISINs, CUSIPs, etc.)
/// into FIGI identifiers.
///
/// # Array Structure
///
/// Each element in the array corresponds to one mapping request from the original batch:
/// - Index 0 contains the result for the first mapping request
/// - Index 1 contains the result for the second mapping request  
/// - And so on...
///
/// # Result Types
///
/// Each array element can be either:
/// - **Success**: Contains [`MappingData`] with FIGI results for the identifier
/// - **Error**: Contains error information when the identifier cannot be mapped
///
/// # Batch Limits
///
/// The OpenFIGI API supports up to 100 mapping requests per batch call (only with an API key).
///
/// # Examples
///
/// ```rust
/// use openfigi_rs::model::response::MappingResponse;
/// use serde_json;
///
/// // Single mapping request result
/// let single_json = r#"[{
///     "data": [
///         {
///             "figi": "BBG000BLNNH6",
///             "ticker": "IBM",
///             "name": "INTL BUSINESS MACHINES CORP"
///         }
///     ]
/// }]"#;
/// let response: MappingResponse = serde_json::from_str(single_json).unwrap();
/// assert_eq!(response.len(), 1);
/// assert!(response[0].is_success());
///
/// // Multiple mapping requests with mixed results
/// let batch_json = r#"[
///     {"data": [{"figi": "BBG000BLNNH6", "ticker": "IBM"}]},
///     {"error": "Invalid ticker symbol"},
///     {"data": [{"figi": "BBG000B9XRY4", "ticker": "AAPL"}]}
/// ]"#;
/// let batch_response: MappingResponse = serde_json::from_str(batch_json).unwrap();
/// assert_eq!(batch_response.len(), 3);
/// assert!(batch_response[0].is_success());
/// assert!(batch_response[1].is_error());
/// assert!(batch_response[2].is_success());
/// ```
pub type MappingResponse = Vec<ResponseResult<MappingData>>;

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

impl FigiData for MappingData {
    /// Returns a slice of the FIGI results contained in this mapping response.
    ///
    /// Provides access to the financial instrument data returned for the mapping request.
    /// This implements the [`FigiData`] trait to allow uniform access to FIGI data across
    /// different response types.
    fn figi_data(&self) -> &[FigiResult] {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    /// Helper function to load test data from the tests/data/mapping directory
    fn load_test_data(filename: &str) -> String {
        let path = format!("tests/data/mapping/{filename}");
        fs::read_to_string(&path).unwrap_or_else(|e| panic!("Failed to read test file {path}: {e}"))
    }

    #[test]
    fn test_deserialize_isin_example() {
        let json_str = load_test_data("isin_example.json");
        let results: MappingResponse = serde_json::from_str(&json_str).unwrap();

        assert_eq!(results.len(), 1);
        let result = &results[0];
        assert!(result.is_success());

        let data = result.data().unwrap();
        assert!(!data.is_empty());

        // Check first IBM entry
        let first_entry = &data[0];
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

    #[test]
    fn test_deserialize_invalid_identifier() {
        let json_str = load_test_data("invalid_identifier.json");
        let results: MappingResponse = serde_json::from_str(&json_str).unwrap();

        assert_eq!(results.len(), 1);
        let result = &results[0];
        assert!(result.is_error());
        assert_eq!(result.error(), Some("Invalid idValue format."));
        assert!(result.data().is_none());
    }

    #[test]
    fn test_deserialize_bulk_request() {
        let json_str = load_test_data("bulk_request.json");
        let results: MappingResponse = serde_json::from_str(&json_str).unwrap();

        assert_eq!(results.len(), 2);

        // First result should be IBM success
        let ibm_result = &results[0];
        assert!(ibm_result.is_success());
        let ibm_data = ibm_result.data().unwrap();
        assert!(!ibm_data.is_empty());
        assert_eq!(ibm_data[0].ticker, Some("IBM".to_string()));

        // Second result should be AAPL success
        let aapl_result = &results[1];
        assert!(aapl_result.is_success());
        let aapl_data = aapl_result.data().unwrap();
        assert!(!aapl_data.is_empty());
        assert_eq!(aapl_data[0].ticker, Some("AAPL".to_string()));
    }

    #[test]
    fn test_deserialize_cusip_with_exchange() {
        let json_str = load_test_data("cusip_with_exchange.json");
        let results: MappingResponse = serde_json::from_str(&json_str).unwrap();

        assert_eq!(results.len(), 1);
        let result = &results[0];
        assert!(result.is_success());

        let data = result.data().unwrap();
        assert!(!data.is_empty());

        // Verify we get valid FIGI results
        for figi_result in data {
            assert!(!figi_result.figi.is_empty());
            assert!(figi_result.ticker.is_some());
        }
    }

    #[test]
    fn test_deserialize_ticker_with_security_type() {
        let json_str = load_test_data("ticker_with_security_type.json");
        let results: MappingResponse = serde_json::from_str(&json_str).unwrap();

        assert_eq!(results.len(), 1);
        let result = &results[0];
        assert!(result.is_success());

        let data = result.data().unwrap();
        assert!(!data.is_empty());

        // Verify security type is properly parsed
        for figi_result in data {
            assert!(figi_result.security_type.is_some());
            assert!(figi_result.market_sector.is_some());
        }
    }

    #[test]
    fn test_deserialize_option_example() {
        let json_str = load_test_data("option_example.json");
        let results: MappingResponse = serde_json::from_str(&json_str).unwrap();

        assert_eq!(results.len(), 1);
        let result = &results[0];

        // This could be either success or error depending on the option data
        if result.is_success() {
            let data = result.data().unwrap();
            for figi_result in data {
                assert!(!figi_result.figi.is_empty());
            }
        } else {
            assert!(result.error().is_some());
        }
    }

    #[test]
    fn test_deserialize_currency_mic_example() {
        let json_str = load_test_data("currency_mic_example.json");
        let results: MappingResponse = serde_json::from_str(&json_str).unwrap();

        assert_eq!(results.len(), 1);
        let result = &results[0];

        // This could be either success or error depending on the currency data
        if result.is_success() {
            let data = result.data().unwrap();
            for figi_result in data {
                assert!(!figi_result.figi.is_empty());
            }
        } else {
            assert!(result.error().is_some());
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
