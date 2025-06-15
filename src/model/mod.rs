//! Data models for OpenFIGI API requests and responses.
//!
//! This module contains all the type definitions and data structures used for
//! interacting with the OpenFIGI API, organized into three main categories:
//!
//! ## Module Organization
//!
//! - [`crate::model::enums`] - Enumeration types for API parameters and response values
//! - [`crate::model::request`] - Request data structures for API endpoints
//! - [`crate::model::response`] - Response data structures from API endpoints
//!
//! These modules provide strongly-typed interfaces for all OpenFIGI API operations,
//! ensuring compile-time validation and ergonomic usage patterns.

pub mod enums;
pub mod request;
pub mod response;
