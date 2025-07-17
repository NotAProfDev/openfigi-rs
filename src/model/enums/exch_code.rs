//! Enum for all supported exchange codes in [OpenFIGI](https://www.openfigi.com/) mapping requests.
//!
//! This list is based on the [OpenFIGI documentation](https://www.openfigi.com/api/documentation) and may be incomplete or subject to change.
//!
//! Example usage:
//! ```rust
//! use openfigi_rs::model::enums::ExchCode;
//! let exch_code = ExchCode::FRANKFURT; // Frankfurt Stock Exchange
//! ```
//!
//! For the full list of values, see: <https://api.openfigi.com/v3/mapping/values/exchCode>

use serde::{Deserialize, Serialize};

// Enum for all supported exchange codes.
include!(concat!(env!("OUT_DIR"), "/exch_code_enum.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_enum_serialization;

    test_enum_serialization!(
        test_serialize_frankfurt,
        ExchCode,
        FRANKFURT,
        "\"FRANKFURT\""
    );
    test_enum_serialization!(test_serialize_bbox, ExchCode, Bbox, "\"bbox\"");
}
