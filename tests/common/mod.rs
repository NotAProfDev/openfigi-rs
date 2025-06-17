//! Common utilities for OpenFIGI integration tests
//!
//! This module provides shared functionality used across all integration test modules,
//! including client creation and rate limiting utilities to ensure tests comply with
//! OpenFIGI API limits.

use openfigi_rs::client::OpenFIGIClient;
use std::time::Duration;
use tokio::time::sleep;

/// Creates a new OpenFIGI client instance for testing
///
/// Uses default configuration without authentication for basic API access.
/// This client can be used for testing unauthenticated endpoints with
/// the standard rate limits applied.
pub fn create_test_client() -> OpenFIGIClient {
    OpenFIGIClient::new()
}

/// Adds a delay between API requests to respect rate limits
///
/// OpenFIGI allows 25 requests per minute for unauthenticated requests,
/// which equals approximately 1 request per 2.4 seconds.
///
/// This function should be called after each API request in integration tests
/// to ensure compliance with OpenFIGI's rate limiting policies.
pub async fn rate_limit_delay() {
    sleep(Duration::from_millis(10000)).await;
}
