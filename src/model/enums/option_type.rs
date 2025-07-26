//! Enum for all supported option types in [OpenFIGI](https://www.openfigi.com/) mapping requests.
//!
//! This list is based on the [OpenFIGI documentation](https://www.openfigi.com/api/documentation) and may be incomplete or subject to change.
//!
//! Example usage:
//! ```rust
//! use openfigi_rs::model::enums::OptionType;
//! let option_type = OptionType::Call; // Call
//! ```

use serde::{Deserialize, Serialize};

/// Enum for all supported option types.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum OptionType {
    /// Call option.
    Call,
    /// Put option.
    Put,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_enum_serialization;

    test_enum_serialization!(test_serialize_call, OptionType, Call, "\"Call\"");
    test_enum_serialization!(test_serialize_put, OptionType, Put, "\"Put\"");
}
