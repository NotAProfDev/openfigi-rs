//! Enum for all supported market sector description in [OpenFIGI](https://www.openfigi.com/) mapping requests.
//!
//! This list is based on the [OpenFIGI documentation](https://www.openfigi.com/api/documentation) and may be incomplete or subject to change.
//!
//! Example usage:
//! ```rust
//! use openfigi_rs::model::enums::MarketSecDesc;
//! let market_sec_desc = MarketSecDesc::Equity; // Equities
//! ```
//!
//! For the full list of values, see: <https://api.openfigi.com/v3/mapping/values/marketSecDes>

use serde::{Deserialize, Serialize};

/// Enum for all supported market sector description values.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MarketSecDesc {
    /// Equities
    Equity,
    /// Commodities
    Comdty,
    /// Corporate Bonds
    Corp,
    /// Currencies
    Curncy,
    /// Government Bonds
    Govt,
    /// Indices
    Index,
    /// Money Market Instruments
    #[serde(rename = "M-Mkt")]
    MMkt,
    /// Mortgages
    Mtge,
    /// Municipal Bonds
    Muni,
    /// Preferred Stocks
    Pfd,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_equity() {
        let val = MarketSecDesc::Equity;
        let serialized = serde_json::to_string(&val).unwrap();
        assert_eq!(serialized, "\"Equity\"");
    }

    #[test]
    fn test_deserialize_equity() {
        let json = "\"Equity\"";
        let deserialized: MarketSecDesc = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, MarketSecDesc::Equity);
    }

    #[test]
    fn test_serialize_mmkt() {
        let val = MarketSecDesc::MMkt;
        let serialized = serde_json::to_string(&val).unwrap();
        assert_eq!(serialized, "\"M-Mkt\"");
    }

    #[test]
    fn test_deserialize_mmkt() {
        let json = "\"M-Mkt\"";
        let deserialized: MarketSecDesc = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, MarketSecDesc::MMkt);
    }
}
