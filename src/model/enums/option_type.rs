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
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OptionType {
    /// Call option.
    Call,
    /// Put option.
    Put,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_call() {
        let val = OptionType::Call;
        let serialized = serde_json::to_string(&val).unwrap();
        assert_eq!(serialized, "\"Call\"");
    }

    #[test]
    fn test_deserialize_call() {
        let json = "\"Call\"";
        let deserialized: OptionType = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, OptionType::Call);
    }

    #[test]
    fn test_serialize_put() {
        let val = OptionType::Put;
        let serialized = serde_json::to_string(&val).unwrap();
        assert_eq!(serialized, "\"Put\"");
    }

    #[test]
    fn test_deserialize_put() {
        let json = "\"Put\"";
        let deserialized: OptionType = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, OptionType::Put);
    }
}
