//! # OpenFIGI API Endpoints
//!
//! This module contains implementations for all OpenFIGI API endpoints.
//! Each endpoint is organized into its own submodule with request builders
//! and response handling.
//!
//! ## Available Endpoints
//!
//! - [`crate::endpoint::filter`] - Filter endpoint for finding FIGIs using key words and other filters. The results are listed alphabetically by FIGI and include the number of results.
//! - [`crate::endpoint::mapping`] - Mapping endpoint for converting third party identifiers to FIGIs
//! - [`crate::endpoint::search`] - Search endpoint for finding FIGIs using key words and other filters.
pub mod filter;
mod macros;
pub mod mapping;
pub mod search;
