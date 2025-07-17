//! Enum for all supported market identifiers codes in [OpenFIGI](https://www.openfigi.com/) mapping requests.
//!
//! This list is based on the [OpenFIGI documentation](https://www.openfigi.com/api/documentation) and may be incomplete or subject to change.
//!
//! Example usage:
//! ```rust
//! use openfigi_rs::model::enums::MicCode;
//! let mic = MicCode::XCME; // Chicago Mercantile Exchange (CME)
//! ```
//!
//! For the full list of values, see: <https://api.openfigi.com/v3/mapping/values/micCode>

use serde::{Deserialize, Serialize};

// Enum for all supported market identifiers codes.
include!(concat!(env!("OUT_DIR"), "/mic_code_enum.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_enum_serialization;

    test_enum_serialization!(test_serialize_xcme, MicCode, XCME, "\"XCME\"");
    test_enum_serialization!(test_serialize_yldx, MicCode, YLDX, "\"YLDX\"");
}
