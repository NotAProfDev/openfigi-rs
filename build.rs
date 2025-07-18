//! Build script for generating OpenFIGI API enums from live data
//!
//! This script fetches enum values from the OpenFIGI API and generates corresponding
//! Rust enums with proper documentation and serialization attributes. It includes
//! intelligent caching to avoid unnecessary API calls during development.

use serde::Deserialize;
use std::{
    collections::HashMap,
    env,
    fmt::{self, Display, Write},
    fs::{self, File},
    path::{Path, PathBuf},
};

// ============================================================================================
// CONFIGURATION CONSTANTS
// ============================================================================================

/// Environment variable to force rebuilding of all enums regardless of cache status
const FORCE_REBUILD_ENV_VAR: &str = "OPENFIGI_FORCE_REBUILD";

/// Path to the build trigger file that tracks when we last fetched fresh data
const BUILD_TRIGGER_FILE: &str = "target/build_trigger";

/// Maximum age of cached data before forcing a refresh (4 weeks in seconds)
const CACHE_MAX_AGE_SECONDS: u64 = 4 * 7 * 24 * 60 * 60;

/// Base URL for OpenFIGI mapping values API
const OPENFIGI_BASE_URL: &str = "https://api.openfigi.com/v3/mapping/values";

// ============================================================================================
// ERROR HANDLING
// ============================================================================================

/// Custom error type for build script operations
#[derive(Debug)]
enum BuildError {
    Io(std::io::Error),
    Json(serde_json::Error),
    Csv(csv::Error),
    Http(reqwest::Error),
    InvalidData(String),
    EnvVar(env::VarError),
    Fmt(fmt::Error),
}

impl Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildError::Io(e) => write!(f, "I/O error: {e}"),
            BuildError::Json(e) => write!(f, "JSON parsing error: {e}"),
            BuildError::Csv(e) => write!(f, "CSV parsing error: {e}"),
            BuildError::Http(e) => write!(f, "HTTP request error: {e}"),
            BuildError::InvalidData(msg) => write!(f, "Invalid data: {msg}"),
            BuildError::EnvVar(e) => write!(f, "Environment variable error: {e}"),
            BuildError::Fmt(e) => write!(f, "Formatting error: {e}"),
        }
    }
}

impl std::error::Error for BuildError {}

impl From<std::io::Error> for BuildError {
    fn from(error: std::io::Error) -> Self {
        BuildError::Io(error)
    }
}

impl From<serde_json::Error> for BuildError {
    fn from(error: serde_json::Error) -> Self {
        BuildError::Json(error)
    }
}

impl From<csv::Error> for BuildError {
    fn from(error: csv::Error) -> Self {
        BuildError::Csv(error)
    }
}

impl From<reqwest::Error> for BuildError {
    fn from(error: reqwest::Error) -> Self {
        BuildError::Http(error)
    }
}

impl From<env::VarError> for BuildError {
    fn from(error: env::VarError) -> Self {
        BuildError::EnvVar(error)
    }
}

impl From<fmt::Error> for BuildError {
    fn from(error: fmt::Error) -> Self {
        BuildError::Fmt(error)
    }
}

type BuildResult<T> = Result<T, BuildError>;

// ============================================================================================
// CONFIGURATION STRUCTURES
// ============================================================================================

/// Configuration for a single OpenFIGI API endpoint that generates an enum
#[derive(Debug, Clone)]
struct EndpointConfig {
    /// The name of the enum to generate (e.g., "Currency")
    name: &'static str,
    /// The API endpoint path (will be appended to base URL)
    api_endpoint: &'static str,
    /// Path to cache file relative to target directory
    cache_filename: &'static str,
    /// Name of the generated enum file
    enum_filename: &'static str,
    /// Documentation comment for the enum
    doc_comment: &'static str,
    /// Optional path to CSV file containing additional documentation
    docs_csv_path: Option<&'static str>,
}

impl EndpointConfig {
    /// Get the full API URL for this endpoint
    fn api_url(&self) -> String {
        format!("{OPENFIGI_BASE_URL}/{}", self.api_endpoint)
    }

    /// Get the full cache file path
    fn cache_path(&self) -> String {
        format!("target/{}", self.cache_filename)
    }

    /// Get the documentation CSV path if it exists
    fn docs_path(&self, manifest_dir: &str) -> Option<PathBuf> {
        self.docs_csv_path
            .map(|path| Path::new(manifest_dir).join("resources").join(path))
    }
}

/// API response structure for mapping values
#[derive(Deserialize, Debug)]
struct ApiResponse {
    values: Vec<String>,
}

// ============================================================================================
// MAIN BUILD CONFIGURATION
// ============================================================================================

/// Central configuration for all OpenFIGI enums to be generated
const ENDPOINTS: &[EndpointConfig] = &[
    EndpointConfig {
        name: "Currency",
        api_endpoint: "currency",
        cache_filename: "currency_cache.json",
        enum_filename: "currency_enum.rs",
        doc_comment: "/// Enum for all supported currency codes.",
        docs_csv_path: None,
    },
    EndpointConfig {
        name: "ExchCode",
        api_endpoint: "exchCode",
        cache_filename: "exch_code_cache.json",
        enum_filename: "exch_code_enum.rs",
        doc_comment: "/// Enum for all supported exchange codes.",
        docs_csv_path: None,
    },
    EndpointConfig {
        name: "IdType",
        api_endpoint: "idType",
        cache_filename: "id_type_cache.json",
        enum_filename: "id_type_enum.rs",
        doc_comment: "/// Enum for all supported ID types.",
        docs_csv_path: Some("id_type_enum_docs.csv"),
    },
    EndpointConfig {
        name: "MarketSecDesc",
        api_endpoint: "marketSecDes",
        cache_filename: "market_sec_desc_cache.json",
        enum_filename: "market_sec_desc_enum.rs",
        doc_comment: "/// Enum for all supported market sector descriptions.",
        docs_csv_path: Some("market_sec_desc_enum_docs.csv"),
    },
    EndpointConfig {
        name: "MicCode",
        api_endpoint: "micCode",
        cache_filename: "mic_code_cache.json",
        enum_filename: "mic_code_enum.rs",
        doc_comment: "/// Enum for all supported market identifiers codes.",
        docs_csv_path: None,
    },
    EndpointConfig {
        name: "SecurityType",
        api_endpoint: "securityType",
        cache_filename: "security_type_cache.json",
        enum_filename: "security_type_enum.rs",
        doc_comment: "/// Enum for all supported security types.",
        docs_csv_path: Some("security_type_enum_docs.csv"),
    },
    EndpointConfig {
        name: "SecurityType2",
        api_endpoint: "securityType2",
        cache_filename: "security_type2_cache.json",
        enum_filename: "security_type2_enum.rs",
        doc_comment: "/// Enum for all supported security types 2.",
        docs_csv_path: None,
    },
    EndpointConfig {
        name: "StateCode",
        api_endpoint: "stateCode",
        cache_filename: "state_code_cache.json",
        enum_filename: "state_code_enum.rs",
        doc_comment: "/// Enum for all supported state codes.",
        docs_csv_path: None,
    },
];

// ============================================================================================
// MAIN ENTRY POINT
// ============================================================================================

fn main() -> BuildResult<()> {
    println!("cargo:rerun-if-changed=build.rs");

    // Validate endpoint configurations
    validate_endpoint_configs();

    let manifest_dir = match env::var("CARGO_MANIFEST_DIR") {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Failed to get CARGO_MANIFEST_DIR: {e}");
            std::process::exit(1);
        }
    };

    // Register all documentation CSV files for rebuild tracking
    register_doc_files_for_rebuild(&manifest_dir);

    // Determine if we need to fetch fresh data
    let should_fetch_fresh_data = should_rebuild_enums();
    if should_fetch_fresh_data {
        if let Err(e) = update_build_trigger() {
            eprintln!("Warning: Failed to update build trigger: {e}");
        }
    }

    // Process each endpoint configuration
    println!("Processing {} OpenFIGI enums...", ENDPOINTS.len());
    for config in ENDPOINTS {
        process_endpoint_config(config, &manifest_dir, should_fetch_fresh_data)?;
    }

    println!("Successfully generated all OpenFIGI enums");
    Ok(())
}

/// Register documentation files for Cargo rebuild tracking
fn register_doc_files_for_rebuild(manifest_dir: &str) {
    for endpoint in ENDPOINTS {
        if let Some(docs_path) = endpoint.docs_path(manifest_dir) {
            println!("cargo:rerun-if-changed={}", docs_path.display());
        }
    }
}

/// Validates endpoint configurations for consistency
fn validate_endpoint_configs() {
    let mut names = std::collections::HashSet::new();
    let mut cache_files = std::collections::HashSet::new();
    let mut enum_files = std::collections::HashSet::new();

    for config in ENDPOINTS {
        // Check for duplicate names
        assert!(
            names.insert(config.name),
            "Duplicate endpoint name: {}",
            config.name
        );

        // Check for duplicate cache files
        assert!(
            cache_files.insert(config.cache_filename),
            "Duplicate cache filename: {}",
            config.cache_filename
        );

        // Check for duplicate enum files
        assert!(
            enum_files.insert(config.enum_filename),
            "Duplicate enum filename: {}",
            config.enum_filename
        );

        // Validate names are valid Rust identifiers
        assert!(
            !config.name.is_empty() && config.name.chars().next().unwrap().is_ascii_alphabetic(),
            "Invalid enum name: {}",
            config.name
        );
    }

    println!("Validated {} endpoint configurations", ENDPOINTS.len());
}

// ============================================================================================
// CORE PROCESSING LOGIC
// ============================================================================================

/// Processes a single endpoint configuration to generate its enum
fn process_endpoint_config(
    config: &EndpointConfig,
    manifest_dir: &str,
    should_fetch_fresh: bool,
) -> BuildResult<()> {
    println!("Processing {} enum...", config.name);

    // Fetch the enum values (either from cache or API)
    let enum_values = if should_fetch_fresh {
        println!("  → Fetching fresh {} data from API", config.name);
        fetch_fresh_data_from_api(config)?
    } else {
        println!("  → Attempting to load {} from cache", config.name);
        load_cached_data(config).or_else(|cache_err| {
            println!("  → Cache failed ({cache_err}), falling back to API");
            fetch_fresh_data_from_api(config)
        })?
    };

    // Load documentation if available
    let documentation_map = if let Some(docs_path) = config.docs_path(manifest_dir) {
        println!("  → Loading documentation from {}", docs_path.display());
        load_documentation_from_csv(&docs_path)?
    } else {
        HashMap::new()
    };

    // Generate the enum file
    let output_path = get_enum_output_path(config)?;
    generate_enum_file(config, &enum_values, &documentation_map, &output_path)?;

    println!(
        "  ✓ Generated {} enum with {} variants",
        config.name,
        enum_values.len()
    );
    Ok(())
}

/// Determines if we should rebuild/refresh enum data
fn should_rebuild_enums() -> bool {
    // Check if it's a release build
    if env::var("PROFILE").unwrap_or_default() == "release" {
        println!("Triggering rebuild: release build detected");
        return true;
    }

    // Check for force rebuild environment variable
    if env::var(FORCE_REBUILD_ENV_VAR).is_ok() {
        println!("Triggering rebuild: {FORCE_REBUILD_ENV_VAR} environment variable set");
        return true;
    }

    // Check build trigger file existence and age
    let trigger_path = Path::new(BUILD_TRIGGER_FILE);
    if !trigger_path.exists() {
        println!("Triggering rebuild: build trigger file missing");
        return true;
    }

    // Check the age of the trigger file
    match fs::metadata(trigger_path).and_then(|metadata| metadata.modified()) {
        Ok(modified) => match modified.elapsed() {
            Ok(duration) if duration.as_secs() > CACHE_MAX_AGE_SECONDS => {
                println!("Triggering rebuild: cache older than {CACHE_MAX_AGE_SECONDS} seconds");
                true
            }
            Ok(_) => {
                println!("Using cached data: trigger file is recent enough");
                false
            }
            Err(e) => {
                println!("Triggering rebuild: failed to get elapsed time: {e}");
                true
            }
        },
        Err(e) => {
            println!("Triggering rebuild: failed to read trigger file metadata: {e}");
            true
        }
    }
}

/// Updates the build trigger file to mark when we last fetched fresh data
fn update_build_trigger() -> BuildResult<()> {
    let trigger_path = Path::new(BUILD_TRIGGER_FILE);

    // Ensure the target directory exists
    if let Some(parent) = trigger_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Create/update the trigger file
    File::create(trigger_path)?;
    println!("Updated build trigger file at {}", trigger_path.display());
    Ok(())
}

// ============================================================================================
// DATA FETCHING AND CACHING
// ============================================================================================

/// Loads cached enum values from disk
fn load_cached_data(config: &EndpointConfig) -> BuildResult<Vec<String>> {
    let cache_path = config.cache_path();
    let data = fs::read_to_string(&cache_path)?;
    let values: Vec<String> = serde_json::from_str(&data)?;

    if values.is_empty() || values.len() < 5 {
        return Err(BuildError::InvalidData(
            "Cache contains no values".to_string(),
        ));
    }

    Ok(values)
}

/// Fetches fresh data from the OpenFIGI API and caches it
fn fetch_fresh_data_from_api(config: &EndpointConfig) -> BuildResult<Vec<String>> {
    let url = config.api_url();
    let response = reqwest::blocking::get(&url)?;

    if !response.status().is_success() {
        return Err(BuildError::InvalidData(format!(
            "API request to {} failed with status: {}",
            url,
            response.status()
        )));
    }

    let api_response: ApiResponse = response.json()?;
    let values = api_response.values;

    if values.is_empty() || values.len() < 5 {
        return Err(BuildError::InvalidData(format!(
            "API response from {url} contains no values"
        )));
    }

    // Cache the fetched data
    let cache_path = config.cache_path();
    fs::write(&cache_path, serde_json::to_string(&values)?)?;

    Ok(values)
}

// ============================================================================================
// ENUM GENERATION
// ============================================================================================

/// Gets the output path for the generated enum file
fn get_enum_output_path(config: &EndpointConfig) -> BuildResult<PathBuf> {
    let out_dir = env::var("OUT_DIR")?;
    Ok(Path::new(&out_dir).join(config.enum_filename))
}

/// Generates the complete enum file with all variants and documentation
fn generate_enum_file(
    config: &EndpointConfig,
    values: &[String],
    docs_map: &HashMap<String, String>,
    output_path: &Path,
) -> BuildResult<()> {
    let mut content = String::with_capacity(values.len() * 50); // Pre-allocate roughly

    // Write enum declaration with attributes
    writeln!(&mut content, "{}", config.doc_comment)?;
    writeln!(&mut content, "#[allow(missing_docs)]")?;
    writeln!(&mut content, "#[allow(non_camel_case_types)]")?;
    writeln!(&mut content, "#[non_exhaustive]")?;
    writeln!(
        &mut content,
        "#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]"
    )?;
    writeln!(&mut content, "pub enum {} {{", config.name)?;

    // Generate variants
    for value in values {
        let variant_name = sanitize_identifier(value);

        // Add documentation if available
        if let Some(doc_comment) = docs_map.get(value) {
            writeln!(&mut content, "    /// {doc_comment}")?;
        }

        writeln!(&mut content, "    #[serde(rename = \"{value}\")]")?;
        writeln!(&mut content, "    {variant_name},")?;
    }

    writeln!(&mut content, "}}")?;

    // Write the file
    fs::write(output_path, content)?;
    println!("  → Generated enum file: {}", output_path.display());

    Ok(())
}

// ============================================================================================
// IDENTIFIER SANITIZATION
// ============================================================================================

/// Collection of Rust keywords that need to be converted to raw identifiers
const RUST_KEYWORDS: &[&str] = &[
    // Strict keywords (reserved in all editions)
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", // Reserved keywords (may become keywords in future editions)
    "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized",
    "virtual", "yield", // Weak keywords (contextual, but safer to escape)
    "async", "await", "dyn", "try",
];

/// Converts an arbitrary string into a valid Rust identifier
///
/// This function handles several edge cases:
/// - Special symbols are replaced with readable equivalents
/// - Invalid characters are filtered out
/// - Identifiers starting with digits get a prefix
/// - Rust keywords are converted to raw identifiers
/// - Empty results are handled gracefully
fn sanitize_identifier(input: &str) -> String {
    // Handle known special cases first
    if input == "***" {
        return "AAA".to_string();
    }

    // Apply transformations in a pipeline
    let sanitized = input
        .replace('&', "AND") // "R&D" → "RANDD"
        //.replace('+', "PLUS") // "C+" → "CPLUS"
        //.replace(['-', '.', '/'], "_") // "UTF-8" → "UTF_8", "v1.0" → "v1_0", "A/B" → "A_B"
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect::<String>();

    // Handle empty result after sanitization
    if sanitized.is_empty() {
        return "UNKNOWN".to_string();
    }

    // Ensure valid identifier start and capitalization
    let mut chars = sanitized.chars();
    let final_name = match chars.next() {
        Some(first_char) if first_char.is_ascii_digit() => {
            // Prefix with underscore if starts with digit
            format!("_{sanitized}")
        }
        Some(first_char) => {
            // Capitalize first letter
            format!("{}{}", first_char.to_ascii_uppercase(), chars.as_str())
        }
        None => return "UNKNOWN".to_string(),
    };

    // Convert to raw identifier if it's a Rust keyword
    if RUST_KEYWORDS.contains(&final_name.as_str()) {
        format!("r#{final_name}")
    } else {
        final_name
    }
}

// ============================================================================================
// DOCUMENTATION LOADING
// ============================================================================================

/// Loads documentation from a pipe-delimited CSV file into a `HashMap`
///
/// Expected format: `key|description`
///
/// Example:
/// ```
/// USD|United States Dollar
/// EUR|Euro
/// ```
fn load_documentation_from_csv(csv_path: &Path) -> BuildResult<HashMap<String, String>> {
    let mut documentation = HashMap::new();

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(true)
        .from_path(csv_path)?;

    for (line_number, result) in reader.deserialize::<(String, String)>().enumerate() {
        match result {
            Ok((key, description)) => {
                // Skip empty keys or descriptions
                if !key.trim().is_empty() && !description.trim().is_empty() {
                    documentation.insert(key.trim().to_string(), description.trim().to_string());
                }
            }
            Err(e) => {
                return Err(BuildError::InvalidData(format!(
                    "Failed to parse line {} in {}: {}",
                    line_number + 1,
                    csv_path.display(),
                    e
                )));
            }
        }
    }

    println!("  → Loaded {} documentation entries", documentation.len());
    Ok(documentation)
}
