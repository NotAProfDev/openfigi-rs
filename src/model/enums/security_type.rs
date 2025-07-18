#![allow(clippy::allow_attributes)]
//! Enum for all supported security types in [OpenFIGI](https://www.openfigi.com/) mapping requests.
//!
//! This list is based on the [OpenFIGI documentation](https://www.openfigi.com/api/documentation) and may be incomplete or subject to change.
//!
//! Example usage:
//! ```rust
//! use openfigi_rs::model::enums::SecurityType;
//! let security_type = SecurityType::ABSCard; // ABS Card
//! ```
//!
//! For the full list of values, see: <https://api.openfigi.com/v3/mapping/values/securityType>

use serde::{Deserialize, Serialize};

// Enum for all supported security types.
include!(concat!(env!("OUT_DIR"), "/security_type_enum.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_enum_serialization;

    test_enum_serialization!(
        test_serialize_abs_card,
        SecurityType,
        ABSCard,
        "\"ABS Card\""
    );
    test_enum_serialization!(
        test_serialize_adjustable,
        SecurityType,
        ZEROCOUPONOID,
        "\"ZERO COUPON, OID\""
    );
}
