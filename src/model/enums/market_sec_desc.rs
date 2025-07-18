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

// Enum for all supported market sector description values.
include!(concat!(env!("OUT_DIR"), "/market_sec_desc_enum.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_enum_serialization;

    test_enum_serialization!(test_serialize_equity, MarketSecDesc, Equity, "\"Equity\"");
    test_enum_serialization!(test_serialize_mmkt, MarketSecDesc, MMkt, "\"M-Mkt\"");
}
