#![doc(hidden)]
//! Common response types and utilities for OpenFIGI API responses.
//!
//! This module provides the core data structures and traits used across all OpenFIGI API
//! response types. It includes the fundamental [`FigiResult`] struct that represents
//! individual financial instrument data, and the [`ResponseResult`] enum for handling
//! success/error responses.
//!
//! Note: This module is not intended for direct use by consumers of the OpenFIGI API.

use crate::model::enums::{ExchCode, MarketSecDesc, SecurityType, SecurityType2};
use serde::{Deserialize, Serialize};

/// Represents the result of an OpenFIGI API request, which can either succeed with data or fail with an error.
///
/// This enum uses serde's `untagged` attribute to automatically deserialize JSON responses
/// into either success or error variants based on the presence of error fields.
///
/// # Type Parameters
///
/// * `T` - The success response type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseResult<T> {
    /// Successful result containing the response data payload.
    Success(T),
    /// Error result when the API request fails.
    Error(ResponseError),
}

/// Error information returned by the OpenFIGI API when a request fails.
///
/// This structure represents the standard error format used across all OpenFIGI API
/// endpoints when requests cannot be fulfilled successfully.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseError {
    /// Human-readable error message describing why the request failed.
    ///
    /// This message is provided by the OpenFIGI API and typically includes
    /// details about what went wrong with the request (e.g., invalid parameters,
    /// authentication issues, rate limiting, etc.).
    pub error: String,
}

/// Detailed information about a single financial instrument returned by the OpenFIGI API.
///
/// This structure represents the core data about a financial instrument, including its
/// FIGI identifier and various descriptive fields. Not all fields are guaranteed to be
/// present, as different API endpoints and data sources may provide varying levels of detail.
///
/// # Field Descriptions
///
/// - `figi`: The unique FIGI identifier for this specific instrument
/// - `composite_figi`: The Composite Financial Instrument Global Identifier (FIGI) enables users to link
///   multiple FIGIs at the Trading Venue-level within the same country or market in order to obtain an
///   aggregated view for that instrument.
/// - `share_class_figi`: A Share Class level Financial Instrument Global Identifier is assigned to Equities
///   and Funds. This enables users to link multiple Composite FIGIs for the same instrument in order to
///   obtain an aggregated view for that instrument across all countries globally.
/// - `security_type`/`security_type2`: Classification of the instrument type
/// - `market_sector`: Market sector categorization
/// - `ticker`: Trading symbol on the exchange
/// - `name`: Full legal name of the instrument
/// - `exch_code`: Exchange where the instrument is primarily traded
/// - `security_description`: Detailed description of the security
/// - `metadata`: Additional information when other fields are unavailable
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FigiResult {
    /// The Financial Instrument Global Identifier (FIGI) - a unique 12-character identifier.
    ///
    /// An identifier is assigned to instruments of all asset classes, is unique to an individual
    /// instrument and once issued will not change for an instrument. For equity instruments an
    /// identifier is issued per instrument per trading venue.
    ///
    /// This is the primary identifier for the specific financial instrument and is
    /// always present in every FIGI result.
    pub figi: String,

    /// Security type of the instrument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_type: Option<SecurityType>,

    /// Market sector of the instrument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_sector: Option<MarketSecDesc>,

    /// Trading symbol or ticker used on the exchange.
    ///
    /// This is the abbreviated symbol used for trading the instrument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,

    /// Full legal name of the financial instrument.
    ///
    /// The complete, official name of the instrument as registered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Exchange code where the instrument is primarily traded.
    ///
    /// Identifies the specific exchange or trading venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exch_code: Option<ExchCode>,

    /// FIGI identifier for the share class level.
    ///
    /// A Share Class level Financial Instrument Global Identifier is assigned to Equities
    /// and Funds. This enables users to link multiple Composite FIGIs for the same instrument
    /// in order to obtain an aggregated view for that instrument across all countries globally.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shareClassFIGI")]
    pub share_class_figi: Option<String>,

    /// FIGI identifier for the composite level.
    ///
    /// The Composite Financial Instrument Global Identifier (FIGI) enables users to
    /// link multiple FIGIs at the Trading Venue-level within the same country or market
    /// in order to obtain an aggregated view for that instrument.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "compositeFIGI")]
    pub composite_figi: Option<String>,

    /// Alternative security type of the instrument.
    ///
    /// Offers a secondary classification that is less specific than the primary security type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_type2: Option<SecurityType2>,

    /// Detailed textual description of the security.
    ///
    /// Provides additional context and details about the financial instrument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_description: Option<String>,

    /// Additional metadata provided when other fields are unavailable.
    ///
    /// This field may contain supplementary information when the API cannot
    /// populate the standard fields due to data limitations or restrictions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
}

impl FigiResult {
    /// Returns `true` if this result includes a composite FIGI identifier.
    #[must_use]
    pub fn has_composite_figi(&self) -> bool {
        self.composite_figi.is_some()
    }

    /// Returns `true` if this result includes a share class FIGI identifier.
    #[must_use]
    pub fn has_share_class_figi(&self) -> bool {
        self.share_class_figi.is_some()
    }

    /// Returns the most appropriate display name for this financial instrument.
    ///
    /// This method implements a fallback hierarchy:
    /// 1. Full name (if available)
    /// 2. Ticker symbol (if available)  
    /// 3. FIGI identifier (always available)
    ///
    /// This provides a human-readable identifier that is always available.
    #[must_use]
    pub fn display_name(&self) -> &str {
        self.name
            .as_deref()
            .unwrap_or_else(|| self.ticker.as_deref().unwrap_or(&self.figi))
    }
}
