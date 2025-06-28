//! Integration tests for OpenFIGI API /search endpoints
//!
//! These tests validate the functionality of the OpenFIGI search API endpoints,
//! which provide text-based search capabilities for financial instruments using
//! keywords and optional filters to find matching FIGIs.
//!
//! ## Rate Limiting
//!
//! These tests include rate limiting delays to comply with OpenFIGI's API limits:
//! - Unauthenticated: 25 requests per minute
//! - Tests run sequentially with 3-second delays between requests
//!
//! ## Test Coverage
//!
//! - Basic text search functionality
//! - Search with filter criteria (currency, market sector, security type)
//! - Empty result handling for non-existent search terms

use openfigi_rs::model::enums::{Currency, MarketSecDesc, SecurityType};
use serial_test::serial;

mod common;
use common::{create_test_client, rate_limit_delay};

/// Tests basic search functionality with a simple text query
///
/// Validates that a text search for "IBM" returns relevant results and that:
/// - Search results are not empty
/// - Each result contains a valid FIGI identifier
/// - Results are related to the search term (IBM, BUSINESS, or MACHINE)
#[tokio::test]
#[serial]
async fn test_search_basic_query() {
    let client = create_test_client();

    let search_data = client
        .search("IBM")
        .send()
        .await
        .expect("Search request should succeed");

    // Check pagination info is present
    assert!(
        search_data.next_page().is_some(),
        "Pagination information should be present"
    );

    let figi_result = search_data.data();
    // Ensure we received some results
    assert!(
        !figi_result.is_empty(),
        "Search results should not be empty"
    );

    // Validate search results structure
    for data in figi_result {
        assert!(!data.figi.is_empty(), "FIGI should not be empty");
        assert!(data.name.is_some(), "Name should be present");

        // Results should be related to IBM
        if let Some(name) = &data.name {
            let name_upper = name.to_uppercase();
            assert!(
                name_upper.contains("IBM")
                    || name_upper.contains("BUSINESS")
                    || name_upper.contains("MACHINE"),
                "Search result should be related to IBM: {name}",
            );
        }
    }

    // Add delay to avoid rate limiting
    rate_limit_delay().await;
}

/// Tests search functionality with multiple filter criteria
///
/// Validates that search requests can be combined with filters for:
/// - Currency (USD)
/// - Market security description (Equity)
/// - Security type (Common Stock)
///
/// Ensures that returned results match the specified filter criteria
/// when the fields are present in the response.
#[tokio::test]
#[serial]
async fn test_search_with_filters() {
    let client = create_test_client();

    let search_data = client
        .search("technology")
        .currency(Currency::USD)
        .market_sec_des(MarketSecDesc::Equity)
        .security_type(SecurityType::CommonStock)
        .send()
        .await
        .expect("Search request with filters should succeed");

    // Check pagination info is present
    assert!(
        search_data.next_page().is_some(),
        "Pagination information should be present"
    );

    let figi_result = search_data.data();
    assert!(
        !figi_result.is_empty(),
        "Search results should not be empty"
    );

    for data in figi_result {
        // Verify filters are applied when data is available
        if let Some(market_sector) = &data.market_sector {
            assert_eq!(market_sector, &MarketSecDesc::Equity);
        }
        if let Some(security_type) = &data.security_type {
            assert_eq!(security_type, &SecurityType::CommonStock);
        }
    }

    // Add delay to avoid rate limiting
    rate_limit_delay().await;
}

/// Tests handling of search queries that return no results
///
/// Validates that the API properly handles searches for non-existent securities by:
/// - Successfully processing the request without errors
/// - Returning an empty result set for impossible search terms
/// - Maintaining proper response structure even with no matches
#[tokio::test]
#[serial]
async fn test_search_empty_results() {
    let client = create_test_client();

    // Search for something very unlikely to exist
    let search_data = client
        .search("xyzneverexistingsecurityname123456")
        .send()
        .await
        .expect("Empty search request should succeed");

    // Check pagination info is absent for empty results
    assert!(
        search_data.next_page().is_none(),
        "Pagination information should be empty for no results"
    );

    // The response should contain an error
    let figi_result = search_data.data();
    assert!(
        figi_result.is_empty(),
        "Search results should be empty for non-existent query"
    );

    // Add delay to avoid rate limiting
    rate_limit_delay().await;
}
