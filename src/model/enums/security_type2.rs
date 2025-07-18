#![allow(clippy::allow_attributes)]
//! Enum for all supported security types 2 in [OpenFIGI](https://www.openfigi.com/) mapping requests.
//!
//! This list is based on the [OpenFIGI documentation](https://www.openfigi.com/api/documentation) and may be incomplete or subject to change.
//!
//! Example usage:
//! ```rust
//! use openfigi_rs::model::enums::SecurityType2;
//! let mic = SecurityType2::BASISSWAP; // Basis Swap
//! ```
//!
//! For the full list of values, see: <https://api.openfigi.com/v3/mapping/values/securityType2>
use serde::{Deserialize, Serialize};

// Enum for all supported security types 2 values.
include!(concat!(env!("OUT_DIR"), "/security_type2_enum.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_enum_serialization;

    test_enum_serialization!(
        test_serialize_basis_swap,
        SecurityType2,
        BASISSWAP,
        "\"BASIS SWAP\""
    );
    test_enum_serialization!(
        test_serialize_unit_investment_trust,
        SecurityType2,
        UnitInvestmentTrust,
        "\"Unit Investment Trust\""
    );
}
