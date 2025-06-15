//! This module contains enumerations used throughout the openfigi-rs library.
//!
//! Each submodule defines a specific enum type representing codes, types, or descriptors
//! relevant to financial instruments and identifiers, such as exchange codes, currency codes,
//! market security descriptions, and more. These enums are used for type safety and clarity
//! when interacting with the [OpenFIGI](https://www.openfigi.com/api/overview) API and related data models.

mod exch_code;
pub use self::exch_code::ExchCode;
mod id_type;
pub use self::id_type::IdType;
mod mic_code;
pub use self::mic_code::MicCode;
mod currency;
pub use self::currency::Currency;
mod market_sec_desc;
pub use self::market_sec_desc::MarketSecDesc;
mod security_type;
pub use self::security_type::SecurityType;
mod security_type2;
pub use self::security_type2::SecurityType2;
mod state_code;
pub use self::state_code::StateCode;
mod option_type;
pub use self::option_type::OptionType;
