#![doc(hidden)]
//! # Macros for OpenFIGI API
//!
//! This module provides macros to help reduce boilerplate code in the OpenFIGI API client.
//! These macros are used to generate common methods and patterns for multiple structs within this crate.
//!
//! Note: This module is not intended for direct use by consumers of the OpenFIGI API.

/// Macro to implement standard filter builder methods for OpenFIGI API request builders.
///
/// This macro generates a set of common filter methods (e.g., `.exch_code()`, `.currency()`, etc.)
/// for builder structs that expose a `filters_mut()` method. It is used to reduce boilerplate and
/// ensure consistency across all filterable request builders, both for endpoint and model/request types.
///
/// # Usage
///
/// Add a `filters_mut(&mut self) -> &mut RequestFilters` method to your builder struct, then invoke
/// `impl_filter_builder!();` inside the `impl` block. The macro will generate all standard filter
/// methods, each delegating to the `filters_mut()` helper.
#[macro_export]
macro_rules! impl_filter_builder {
    () => {
        /// Sets the `exch_code` for the desired instrument.
        #[must_use]
        pub fn exch_code(mut self, exch_code: ExchCode) -> Self {
            self.filters_mut().exch_code = Some(exch_code);
            self
        }

        /// Sets the `mic_code` for the desired instrument.
        #[must_use]
        pub fn mic_code(mut self, mic_code: MicCode) -> Self {
            self.filters_mut().mic_code = Some(mic_code);
            self
        }

        /// Sets the `currency` for the desired instrument.
        #[must_use]
        pub fn currency(mut self, currency: Currency) -> Self {
            self.filters_mut().currency = Some(currency);
            self
        }

        /// Sets the `market_sec_des` for the desired instrument.
        #[must_use]
        pub fn market_sec_des(mut self, market_sec_des: MarketSecDesc) -> Self {
            self.filters_mut().market_sec_des = Some(market_sec_des);
            self
        }

        /// Sets the `security_type` for the desired instrument.
        #[must_use]
        pub fn security_type(mut self, security_type: SecurityType) -> Self {
            self.filters_mut().security_type = Some(security_type);
            self
        }

        /// Sets the `security_type2` for the desired instrument.
        #[must_use]
        pub fn security_type2(mut self, security_type2: SecurityType2) -> Self {
            self.filters_mut().security_type2 = Some(security_type2);
            self
        }

        /// Sets whether to include unlisted equities in the filter.
        #[must_use]
        pub fn include_unlisted_equities(mut self, val: bool) -> Self {
            self.filters_mut().include_unlisted_equities = Some(val);
            self
        }

        /// Sets the `option_type` for the desired instrument.
        #[must_use]
        pub fn option_type(mut self, option_type: OptionType) -> Self {
            self.filters_mut().option_type = Some(option_type);
            self
        }

        /// Sets the `strike` price range for the desired instrument.
        #[must_use]
        pub fn strike(mut self, strike: [Option<f64>; 2]) -> Self {
            self.filters_mut().strike = Some(strike);
            self
        }

        /// Sets the `contract_size` range for the desired instrument.
        #[must_use]
        pub fn contract_size(mut self, contract_size: [Option<f64>; 2]) -> Self {
            self.filters_mut().contract_size = Some(contract_size);
            self
        }

        /// Sets the `coupon` range for the desired instrument.
        #[must_use]
        pub fn coupon(mut self, coupon: [Option<f64>; 2]) -> Self {
            self.filters_mut().coupon = Some(coupon);
            self
        }

        /// Sets the `expiration` date range for the desired instrument.
        #[must_use]
        pub fn expiration(mut self, expiration: [Option<NaiveDate>; 2]) -> Self {
            self.filters_mut().expiration = Some(expiration);
            self
        }

        /// Sets the `maturity` date range for the desired instrument.
        #[must_use]
        pub fn maturity(mut self, maturity: [Option<NaiveDate>; 2]) -> Self {
            self.filters_mut().maturity = Some(maturity);
            self
        }

        /// Sets the `state_code` for the desired instrument.
        #[must_use]
        pub fn state_code(mut self, state_code: StateCode) -> Self {
            self.filters_mut().state_code = Some(state_code);
            self
        }
    };
}
