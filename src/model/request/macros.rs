//! # Macros for `FilterRequestBuilder`
//!
//! This module provides a macro to generate standard filter builder methods for
//! `FilterRequestBuilder` and similar types. It reduces boilerplate and ensures
//! consistency across all filterable request builders in the OpenFIGI API client.
//!
//! ## Usage
//!
//! Use the [`impl_filter_request_builder!`] macro in your builder struct implementation
//! to automatically generate all standard filter methods. This is especially useful
//! for endpoints like filter, mapping, and search that share a common set of filterable fields.
//!
//! # Example
//!
//! ```rust
//! use openfigi_rs::model::request::filter_macros::impl_filter_request_builder;
//! use openfigi_rs::model::enums::{ExchCode, MicCode, Currency, MarketSecDesc, SecurityType, SecurityType2, OptionType, StateCode};
//! use chrono::NaiveDate;
//!
//! pub struct MyFilterBuilder {
//!     filters: RequestFilters,
//! }
//!
//! impl MyFilterBuilder {
//!     impl_filter_request_builder!();
//!     // ... other builder methods ...
//! }
//! ```
//!
//! The macro will generate all standard filter methods (e.g., `.exch_code()`, `.currency()`, etc.)
//! for your builder, delegating to the inner `request_builder` field.
//!
//! Note: This module is not intended for direct use by consumers of the OpenFIGI API.

/// Macro to implement standard filter builder methods for OpenFIGI request builders.
#[macro_export]
macro_rules! impl_filter_request_builder {
    () => {
        /// Sets the `exch_code` for the desired instrument.
        #[must_use]
        pub fn exch_code(mut self, exch_code: ExchCode) -> Self {
            self.filters.exch_code = Some(exch_code);
            self
        }

        /// Sets the `mic_code` for the desired instrument.
        #[must_use]
        pub fn mic_code(mut self, mic_code: MicCode) -> Self {
            self.filters.mic_code = Some(mic_code);
            self
        }

        /// Sets the `currency` for the desired instrument.
        #[must_use]
        pub fn currency(mut self, currency: Currency) -> Self {
            self.filters.currency = Some(currency);
            self
        }

        /// Sets the `market_sec_des` for the desired instrument.
        #[must_use]
        pub fn market_sec_des(mut self, market_sec_des: MarketSecDesc) -> Self {
            self.filters.market_sec_des = Some(market_sec_des);
            self
        }

        /// Sets the `security_type` for the desired instrument.
        #[must_use]
        pub fn security_type(mut self, security_type: SecurityType) -> Self {
            self.filters.security_type = Some(security_type);
            self
        }

        /// Sets the `security_type2` for the desired instrument.
        #[must_use]
        pub fn security_type2(mut self, security_type2: SecurityType2) -> Self {
            self.filters.security_type2 = Some(security_type2);
            self
        }

        /// Sets whether to include unlisted equities in the filter.
        #[must_use]
        pub fn include_unlisted_equities(mut self, val: bool) -> Self {
            self.filters.include_unlisted_equities = Some(val);
            self
        }

        /// Sets the `option_type` for the desired instrument.
        #[must_use]
        pub fn option_type(mut self, option_type: OptionType) -> Self {
            self.filters.option_type = Some(option_type);
            self
        }

        /// Sets the `strike` price range for the desired instrument.
        #[must_use]
        pub fn strike(mut self, strike: [Option<f64>; 2]) -> Self {
            self.filters.strike = Some(strike);
            self
        }

        /// Sets the `contract_size` range for the desired instrument.
        #[must_use]
        pub fn contract_size(mut self, contract_size: [Option<f64>; 2]) -> Self {
            self.filters.contract_size = Some(contract_size);
            self
        }

        /// Sets the `coupon` range for the desired instrument.
        #[must_use]
        pub fn coupon(mut self, coupon: [Option<f64>; 2]) -> Self {
            self.filters.coupon = Some(coupon);
            self
        }

        /// Sets the `expiration` date range for the desired instrument.
        #[must_use]
        pub fn expiration(mut self, expiration: [Option<NaiveDate>; 2]) -> Self {
            self.filters.expiration = Some(expiration);
            self
        }

        /// Sets the `maturity` date range for the desired instrument.
        #[must_use]
        pub fn maturity(mut self, maturity: [Option<NaiveDate>; 2]) -> Self {
            self.filters.maturity = Some(maturity);
            self
        }

        /// Sets the `state_code` for the desired instrument.
        #[must_use]
        pub fn state_code(mut self, state_code: StateCode) -> Self {
            self.filters.state_code = Some(state_code);
            self
        }
    };
}
