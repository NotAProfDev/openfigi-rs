use std::fs;

/// Utility functions to load test data from files
pub fn load_test_data(folder: &str, filename: &str) -> String {
    let path = format!("tests/data/{folder}/{filename}");
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read test file: {path}"))
}

/// Utility macro to test serialization and deserialization of enum variants
#[macro_export]
macro_rules! test_enum_serialization {
    ($name:ident, $enum_type:ty, $variant:ident, $expected:expr) => {
        #[test]
        fn $name() {
            let val = <$enum_type>::$variant;
            let serialized = serde_json::to_string(&val).unwrap();
            assert_eq!(serialized, $expected);

            let deserialized: $enum_type = serde_json::from_str($expected).unwrap();
            assert_eq!(deserialized, val);
        }
    };
}
