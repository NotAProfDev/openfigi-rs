#![allow(clippy::allow_attributes)]
//! Enum for all supported currency codes in [OpenFIGI](https://www.openfigi.com/) mapping requests.
//!
//! This list is based on the [OpenFIGI documentation](https://www.openfigi.com/api/documentation) and may be incomplete or subject to change.
//!
//! Example usage:
//! ```rust
//! use openfigi_rs::model::enums::Currency;
//! let currency = Currency::USD; // US Dollar
//! ```
//!
//! For the full list of values, see: <https://api.openfigi.com/v3/mapping/values/currency>

use serde::{Deserialize, Serialize};

// Enum for all supported currency codes.
include!(concat!(env!("OUT_DIR"), "/currency_enum.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_enum_serialization;

    test_enum_serialization!(test_serialize_usd, Currency, USD, "\"USD\"");
    test_enum_serialization!(test_serialize_aud, Currency, AUd, "\"AUd\"");
}
