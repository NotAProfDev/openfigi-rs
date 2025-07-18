//! Enum for all supported idType values in [OpenFIGI](https://www.openfigi.com/) mapping requests.
//!
//! This list is based on the [OpenFIGI documentation](https://www.openfigi.com/api/documentation) and may be incomplete or subject to change.
//!
//! Example usage:
//! ```rust
//! use openfigi_rs::model::enums::IdType;
//! let id_type = IdType::IdIsin; // International Securities Identification Number
//! ```
//!
//! For the full list of values, see: <https://api.openfigi.com/v3/mapping/values/idType>

use serde::{Deserialize, Serialize};

// Enum for all supported idType values.
include!(concat!(env!("OUT_DIR"), "/id_type_enum.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_enum_serialization;

    test_enum_serialization!(test_serialize_id_isin, IdType, ID_ISIN, "\"ID_ISIN\"");
    test_enum_serialization!(
        test_serialize_vendor_index_code,
        IdType,
        VENDOR_INDEX_CODE,
        "\"VENDOR_INDEX_CODE\""
    );
}
