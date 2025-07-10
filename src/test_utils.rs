#![doc(hidden)]
//! # Test Utilities
//!
//! Shared testing utilities and macros for the OpenFIGI Rust SDK.
//!
//! This module provides common functionality used across test modules to reduce
//! code duplication and ensure consistent testing patterns throughout the codebase.
//!
//! ## Features
//!
//! - **Test Data Loading**: Functions to load test data files from the `tests/data/` directory
//! - **Enum Serialization Testing**: Macros to test serde serialization/deserialization of enum variants
//! - **Validation Testing**: Macros for testing request validation logic (future enhancement)
//!
//! ## Design Principles
//!
//! - **DRY (Don't Repeat Yourself)**: Eliminate repetitive test code through reusable utilities
//! - **Consistency**: Ensure all tests follow the same patterns and conventions
//! - **Maintainability**: Centralize common testing logic for easier updates
//! - **Type Safety**: Provide compile-time guarantees for test correctness

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
