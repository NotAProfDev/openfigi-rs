//! Enum for all supported state codes in [OpenFIGI](https://www.openfigi.com/) mapping requests.
//!
//! This list is based on the [OpenFIGI documentation](https://www.openfigi.com/api/documentation) and may be incomplete or subject to change.
//!
//! Example usage:
//! ```rust
//! use openfigi_rs::model::enums::StateCode;
//! let mic = StateCode::CA; // California
//! ```
//!
//! For the full list of values, see: <https://api.openfigi.com/v3/mapping/values/stateCode>

use serde::{Deserialize, Serialize};

// Enum for all supported state codes.
include!(concat!(env!("OUT_DIR"), "/state_code_enum.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_enum_serialization;

    test_enum_serialization!(test_serialize_ab, StateCode, CA, "\"CA\"");
    test_enum_serialization!(test_serialize_ac, StateCode, YA, "\"YA\"");
}
