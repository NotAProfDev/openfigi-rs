//! Integration tests for OpenFIGI API /filter endpoints
//!
//! These tests validate the functionality of the OpenFIGI filter API endpoints,
//! which allow filtering financial instruments by specific criteria and keywords
//! to find matching FIGIs with advanced filtering capabilities.
//!
//! ## Rate Limiting
//!
//! These tests include rate limiting delays to comply with OpenFIGI's API limits:
//! - Unauthenticated: 25 requests per minute
//! - Tests run sequentially with 3-second delays between requests
//!
//! ## Test Coverage
//!
//! - Basic keyword-based filtering functionality
//! - Multi-criteria filtering with various parameters
//! - Empty result handling for non-existent filter terms
//! - Pagination information validation

use openfigi_rs::model::enums::{Currency, ExchCode, MarketSecDesc, SecurityType};
use serial_test::serial;

mod common;
use common::{create_test_client, rate_limit_delay};

/// Tests basic filter functionality with a simple keyword query
///
/// Validates that a filter request with the keyword "technology" returns relevant results and that:
/// - Filter results are not empty
/// - Each result contains a valid FIGI identifier and name
/// - Pagination information (total results count) is provided
/// - Response structure is valid for basic filtering operations
#[tokio::test]
#[serial]
async fn test_filter_basic_query() {
    let client = create_test_client();

    let result = client
        .filter()
        .query("technology")
        .send()
        .await
        .expect("Filter request should succeed");

    // Check pagination info is present
    assert!(
        result.next_page().is_some(),
        "Pagination information should be present"
    );

    // Check total results count
    assert!(
        result.total_results().is_some(),
        "Total results count should be present"
    );

    if let Some(data) = result.data() {
        assert!(!data.is_empty(), "Filter results should not be empty");

        // Validate filter results structure
        for figi_result in data {
            assert!(!figi_result.figi.is_empty(), "FIGI should not be empty");
            assert!(figi_result.name.is_some(), "Name should be present");
        }

        // Check pagination info
        assert!(
            result.total_results().is_some(),
            "Total count should be present"
        );
        println!(
            "Filter found {} total results",
            result.total_results().unwrap()
        );
    }

    // Add delay to avoid rate limiting
    rate_limit_delay().await;
}

/// Tests filter functionality with multiple filtering criteria
///
/// Validates that filter requests can be combined with multiple criteria:
/// - Keyword query ("energy")
/// - Currency (USD)
/// - Exchange code (US)
/// - Market security description (Equity)
/// - Security type (Common Stock)
/// - Include unlisted equities flag (false)
///
/// Ensures that returned results match the specified filter criteria
/// when the fields are present in the response, and validates pagination information.
#[tokio::test]
#[serial]
async fn test_filter_with_multiple_criteria() {
    let client = create_test_client();
    let result = client
        .filter()
        .query("energy")
        .currency(Currency::USD)
        .exch_code(ExchCode::US)
        .market_sec_des(MarketSecDesc::Equity)
        .security_type(SecurityType::CommonStock)
        .send()
        .await
        .expect("Filter request with multiple criteria should succeed");

    // Check pagination info is present
    assert!(
        result.next_page().is_some(),
        "Pagination information should be present"
    );

    // Check total results count
    assert!(
        result.total_results().is_some(),
        "Total results count should be present"
    );

    if let Some(data) = result.data() {
        for figi_result in data {
            // Verify all filters are applied when data is available
            if let Some(exch_code) = &figi_result.exch_code {
                assert_eq!(exch_code, &ExchCode::US);
            }
            if let Some(market_sector) = &figi_result.market_sector {
                assert_eq!(market_sector, &MarketSecDesc::Equity);
            }
            if let Some(security_type) = &figi_result.security_type {
                assert_eq!(security_type, &SecurityType::CommonStock);
            }
        }
    }

    // Verify pagination information
    assert!(result.total_results().is_some());
    println!(
        "Multiple criteria filter found {} results",
        result.total_results().unwrap()
    );

    // Add delay to avoid rate limiting
    rate_limit_delay().await;
}

/// Tests handling of filter queries that return no results
///
/// Validates that the API properly handles filter requests for non-existent securities by:
/// - Successfully processing the request without errors
/// - Returning an empty result set for impossible filter terms
/// - Maintaining proper response structure even with no matches
/// - Ensuring the response indicates success despite empty results
#[tokio::test]
#[serial]
async fn test_filter_empty_results() {
    let client = create_test_client();

    // Search for something very unlikely to exist
    let result = client
        .filter()
        .query("xyzneverexistingsecurityname123456")
        .send()
        .await
        .expect("Empty filter request should succeed");

    // The response should contain an error
    assert!(result.is_success());
    let data = result.data().unwrap();
    assert!(data.is_empty());

    // Add delay to avoid rate limiting
    rate_limit_delay().await;
}
