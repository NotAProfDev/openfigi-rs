pub mod client;
pub mod error;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The default base URL for the OpenFIGI API v3.
pub const DEFAULT_BASE_URL: &str = "https://api.openfigi.com/v3/";
