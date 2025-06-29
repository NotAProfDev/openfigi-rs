//! # Macros for Filterable Endpoint Builders
//!
//! This module provides reusable macros for implementing common builder methods on endpoint request builders
//! in the OpenFIGI API client. These macros reduce boilerplate and ensure consistency across all endpoints
//! that support filtering by exchange, currency, security type, and other common fields.
//!
//! ## Usage
//!
//! Use the [`impl_filterable_builder!`] macro in your builder struct implementation to automatically generate
//! all standard filter methods. This is especially useful for endpoints like mapping, filter, and search that
//! share a common set of filterable fields.
//!
//! # Example
//!
//! ```rust
//! use openfigi_rs::endpoint::macros::impl_filterable_builder;
//! use openfigi_rs::model::enums::{ExchCode, MicCode, Currency, MarketSecDesc, SecurityType, SecurityType2, OptionType, StateCode};
//! use chrono::NaiveDate;
//!
//! pub struct MyRequestBuilder {
//!     request_builder: SomeInnerBuilder,
//! }
//!
//! impl MyRequestBuilder {
//!     impl_filterable_builder!();
//!     // ... other builder methods ...
//! }
//! ```
//!
//! The macro will generate all standard filter methods (e.g., `.exch_code()`, `.currency()`, etc.)
//! for your builder, delegating to the inner `request_builder` field.
//!
//! Note: This module is not intended for direct use by consumers of the OpenFIGI API.

/// Macro to implement standard filter builder methods for OpenFIGI endpoint request builders.
///
/// This macro generates common filter methods (e.g., `.exch_code()`, `.currency()`, etc.)
/// for builder structs that wrap an inner `request_builder` field, reducing boilerplate
/// and ensuring consistency across filterable endpoints.
#[macro_export]
macro_rules! impl_filterable_builder {
    () => {
        /// Sets the optional exchange code for the request.
        #[must_use]
        pub fn exch_code(mut self, exch_code: ExchCode) -> Self {
            self.request_builder = self.request_builder.exch_code(exch_code);
            self
        }

        /// Sets the optional MIC code for the request.
        #[must_use]
        pub fn mic_code(mut self, mic_code: MicCode) -> Self {
            self.request_builder = self.request_builder.mic_code(mic_code);
            self
        }

        /// Sets the optional currency for the request.
        #[must_use]
        pub fn currency(mut self, currency: Currency) -> Self {
            self.request_builder = self.request_builder.currency(currency);
            self
        }

        /// Sets the optional market sector description for the request.
        #[must_use]
        pub fn market_sec_des(mut self, market_sec_des: MarketSecDesc) -> Self {
            self.request_builder = self.request_builder.market_sec_des(market_sec_des);
            self
        }

        /// Sets the optional security type for the request.
        #[must_use]
        pub fn security_type(mut self, security_type: SecurityType) -> Self {
            self.request_builder = self.request_builder.security_type(security_type);
            self
        }

        /// Sets the optional secondary security type for the request.
        #[must_use]
        pub fn security_type2(mut self, security_type2: SecurityType2) -> Self {
            self.request_builder = self.request_builder.security_type2(security_type2);
            self
        }

        /// Sets the optional flag to include unlisted equities for the request.
        #[must_use]
        pub fn include_unlisted_equities(mut self, val: bool) -> Self {
            self.request_builder = self.request_builder.include_unlisted_equities(val);
            self
        }

        /// Sets the optional option type for the request.
        #[must_use]
        pub fn option_type(mut self, option_type: OptionType) -> Self {
            self.request_builder = self.request_builder.option_type(option_type);
            self
        }

        /// Sets the optional strike price range for the request.
        #[must_use]
        pub fn strike(mut self, strike: [Option<f64>; 2]) -> Self {
            self.request_builder = self.request_builder.strike(strike);
            self
        }

        /// Sets the optional contract size range for the request.
        #[must_use]
        pub fn contract_size(mut self, contract_size: [Option<f64>; 2]) -> Self {
            self.request_builder = self.request_builder.contract_size(contract_size);
            self
        }

        /// Sets the optional coupon range for the request.
        #[must_use]
        pub fn coupon(mut self, coupon: [Option<f64>; 2]) -> Self {
            self.request_builder = self.request_builder.coupon(coupon);
            self
        }

        /// Sets the optional expiration date range for the request.
        #[must_use]
        pub fn expiration(mut self, expiration: [Option<NaiveDate>; 2]) -> Self {
            self.request_builder = self.request_builder.expiration(expiration);
            self
        }

        /// Sets the optional maturity date range for the request.
        #[must_use]
        pub fn maturity(mut self, maturity: [Option<NaiveDate>; 2]) -> Self {
            self.request_builder = self.request_builder.maturity(maturity);
            self
        }

        /// Sets the optional state code for the request.
        #[must_use]
        pub fn state_code(mut self, state_code: StateCode) -> Self {
            self.request_builder = self.request_builder.state_code(state_code);
            self
        }
    };
}
