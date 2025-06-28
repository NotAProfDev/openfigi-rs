//! Integration tests for OpenFIGI API /mapping endpoints
//!
//! These tests validate the functionality of the OpenFIGI mapping API endpoints,
//! which convert various financial instrument identifiers (ISIN, ticker, etc.)
//! to Financial Instrument Global Identifiers (FIGIs).
//!
//! ## Rate Limiting
//!
//! These tests include rate limiting delays to comply with OpenFIGI's API limits:
//! - Unauthenticated: 25 requests per minute
//! - Tests run sequentially with 3-second delays between requests
//!
//! ## Test Coverage
//!
//! - Single identifier mapping requests
//! - Bulk mapping requests with multiple identifiers
//! - Filtered mapping requests with specific criteria
//! - Error handling for invalid identifiers

use openfigi_rs::model::{
    enums::{Currency, ExchCode, IdType, MarketSecDesc, SecurityType},
    request::MappingRequest,
};
use serde_json::json;
use serial_test::serial;

mod common;
use common::{create_test_client, rate_limit_delay};

/// Tests basic single identifier mapping functionality
///
/// Validates that a single ISIN can be successfully mapped to FIGI data
/// and that the response contains the expected structure with non-empty
/// FIGI identifiers.
#[tokio::test]
#[serial]
async fn test_mapping_single_isin_request() {
    let client = create_test_client();

    // Test basic ISIN mapping request
    let mapping_data = client
        .mapping(IdType::IdIsin, "US4592001014")
        .send()
        .await
        .expect("Mapping request should succeed");

    let figi_result = mapping_data.data();
    assert!(
        !figi_result.is_empty(),
        "Expected FIGI data, but received an empty response"
    );

    // Validate that each FIGI result has required fields
    for data in figi_result {
        assert!(!data.figi.is_empty(), "Expected FIGI field to be non-empty");
    }

    // Add delay to avoid rate limiting
    rate_limit_delay().await;
}

/// Tests mapping requests with multiple filter criteria
///
/// Validates that mapping requests can be filtered by:
/// - Currency (USD)
/// - Exchange code (US)
/// - Market security description (Equity)
/// - Security type (Common Stock)
///
/// Ensures that returned results match the specified filter criteria
/// when the fields are present in the response.
#[tokio::test]
#[serial]
async fn test_mapping_with_filters() {
    let client = create_test_client();

    // Test mapping with multiple filters
    let mapping_data = client
        .mapping(IdType::Ticker, "IBM")
        .currency(Currency::USD)
        .exch_code(ExchCode::US)
        .market_sec_des(MarketSecDesc::Equity)
        .security_type(SecurityType::CommonStock)
        .send()
        .await
        .expect("Mapping request with filters should succeed");

    let figi_result = mapping_data.data();
    assert!(
        !figi_result.is_empty(),
        "Expected FIGI data, but received an empty response"
    );

    // Validate that results match our filters when available
    for data in figi_result {
        if let Some(exch_code) = &data.exch_code {
            assert_eq!(exch_code, &ExchCode::US);
        }
        if let Some(security_type) = &data.security_type {
            assert_eq!(security_type, &SecurityType::CommonStock);
        }
    }

    // Add delay to avoid rate limiting
    rate_limit_delay().await;
}

// / Tests bulk mapping functionality with multiple identifiers
// /
// / Validates that multiple mapping requests can be processed in a single
// / bulk API call, testing with:
// / - ISIN identifier
// / - Multiple ticker symbols
// /
// / Ensures that the response contains exactly one result per request
// / and that each result has either data or an error message.
// #[tokio::test]
// #[serial]
// async fn test_mapping_bulk_request() {
//     let client = create_test_client();
//     let requests = vec![
//         MappingRequest::new(IdType::IdIsin, json!("US4592001014")),
//         MappingRequest::new(IdType::Ticker, json!("AAPL")),
//         MappingRequest::new(IdType::Ticker, json!("MSFT")),
//     ];

//     // Test mapping with multiple requests in a single bulk call
//     let mapping_data = client
//         .bulk_mapping()
//         .add_requests(requests)
//         .send()
//         .await
//         .expect("Bulk mapping should succeed");

//     // Should have one result for each request
//     assert_eq!(mapping_data.len(), 3);
//     for result in &mapping_data {
//         // Each result should either have data or an error, but not both
//         assert!(
//             result.data().is_some() || result.error().is_some(),
//             "Each result should have either data or an error"
//         );
//     }

//     // Add delay to avoid rate limiting
//     rate_limit_delay().await;
// }

// /// Tests error handling for invalid identifiers
// ///
// /// Validates that the API properly handles invalid input by:
// /// - Returning appropriate error messages
// /// - Not returning data for invalid requests
// /// - Maintaining proper response structure even with errors
// #[tokio::test]
// #[serial]
// async fn test_mapping_invalid_identifier() {
//     let client = create_test_client();

//     let results = client
//         .mapping(IdType::IdIsin, json!("INVALID_ISIN"))
//         .send()
//         .await
//         .expect("API call should succeed");

//     // Length should be 1 for an invalid request
//     assert_eq!(results.len(), 1);

//     // The response should contain an error
//     let data = &results[0];
//     assert!(data.is_error());
//     assert_eq!(data.error(), Some("Invalid idValue format."));
//     assert!(data.data().is_none());

//     // Add delay to avoid rate limiting
//     rate_limit_delay().await;
// }
