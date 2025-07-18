//! This build script fetches data from the OpenFIGI API

use serde::Deserialize;
use std::{
    collections::HashMap,
    fmt::Write,
    fs::{self, File},
    path::{Path, PathBuf},
};

// ######################################################################################
// All constants used in the build script
// These constants define API URLs, cache paths, and file names for enum variants.
// ######################################################################################

// Constants for the build script
const FORCE_REBUILD: &str = "OPENFIGI_FORCE_REBUILD";
const BUILD_TRIGGER_PATH: &str = "target/build_trigger";
const TRIGGER_MAX_AGE_SECS: u64 = 4 * 7 * 24 * 60 * 60; // 4 weeks

// Constants for different data sources
const CURRENCY_API_URL: &str = "https://api.openfigi.com/v3/mapping/values/currency";
const CURRENCY_CACHE_PATH: &str = "target/currency_cache.json";
const CURRENCY_VARIANTS_FILE: &str = "currency_enum.rs";
const CURRENCY_DOC: &str = "/// Enum for all supported currency codes.";

const EXCHCODE_API_URL: &str = "https://api.openfigi.com/v3/mapping/values/exchCode";
const EXCHCODE_CACHE_PATH: &str = "target/exch_code_cache.json";
const EXCHCODE_VARIANTS_FILE: &str = "exch_code_enum.rs";
const EXCHCODE_DOC: &str = "/// Enum for all supported exchange codes.";

const IDTYPE_API_URL: &str = "https://api.openfigi.com/v3/mapping/values/idType";
const IDTYPE_CACHE_PATH: &str = "target/id_type_cache.json";
const IDTYPE_VARIANTS_FILE: &str = "id_type_enum.rs";
const IDTYPE_DOC: &str = "/// Enum for all supported ID types.";
const IDTYPE_VARIANTS_DOC_FILE: &str = "resources/id_type_enum_docs.csv";

const MARKETSECDESC_API_URL: &str = "https://api.openfigi.com/v3/mapping/values/marketSecDes";
const MARKETSECDESC_CACHE_PATH: &str = "target/market_sec_desc_cache.json";
const MARKETSECDESC_VARIANTS_FILE: &str = "market_sec_desc_enum.rs";
const MARKETSECDESC_DOC: &str = "/// Enum for all supported market sector descriptions.";
const MARKETSECDESC_VARIANTS_DOC_FILE: &str = "resources/market_sec_desc_enum_docs.csv";

const MICCODE_API_URL: &str = "https://api.openfigi.com/v3/mapping/values/micCode";
const MICCODE_CACHE_PATH: &str = "target/mic_code_cache.json";
const MICCODE_VARIANTS_FILE: &str = "mic_code_enum.rs";
const MICCODE_DOC: &str = "/// Enum for all supported market identifiers codes.";

const SECURITYTYPE_API_URL: &str = "https://api.openfigi.com/v3/mapping/values/securityType";
const SECURITYTYPE_CACHE_PATH: &str = "target/security_type_cache.json";
const SECURITYTYPE_VARIANTS_FILE: &str = "security_type_enum.rs";
const SECURITYTYPE_DOC: &str = "/// Enum for all supported security types.";
const SECURITYTYPE_VARIANTS_DOC_FILE: &str = "resources/security_type_enum_docs.csv";

const SECURITYTYPE2_API_URL: &str = "https://api.openfigi.com/v3/mapping/values/securityType2";
const SECURITYTYPE2_CACHE_PATH: &str = "target/security_type2_cache.json";
const SECURITYTYPE2_VARIANTS_FILE: &str = "security_type2_enum.rs";
const SECURITYTYPE2_DOC: &str = "/// Enum for all supported security types 2.";

const STATECODE_API_URL: &str = "https://api.openfigi.com/v3/mapping/values/stateCode";
const STATECODE_CACHE_PATH: &str = "target/state_code_cache.json";
const STATECODE_VARIANTS_FILE: &str = "state_code_enum.rs";
const STATECODE_DOC: &str = "/// Enum for all supported state codes.";

fn main() {
    // Check if we should rebuild/refresh the data
    let should_fetch = should_rebuild();

    if should_fetch {
        update_build_trigger();
    } else {
        println!("No rebuild needed, using cached data.");
    }

    // Tell cargo to rerun this script if the build script itself changes.
    println!("cargo:rerun-if-changed=build.rs");

    // Process currency enum
    if let Err(e) = process_endpoint(
        "Currency",
        CURRENCY_API_URL,
        CURRENCY_CACHE_PATH,
        CURRENCY_VARIANTS_FILE,
        CURRENCY_DOC,
        should_fetch,
        None,
    ) {
        eprintln!("Error processing currency data: {e}");
    }

    // Process exchange code enum
    if let Err(e) = process_endpoint(
        "ExchCode",
        EXCHCODE_API_URL,
        EXCHCODE_CACHE_PATH,
        EXCHCODE_VARIANTS_FILE,
        EXCHCODE_DOC,
        should_fetch,
        None,
    ) {
        eprintln!("Error processing exchange code data: {e}");
    }

    // Process exchange code enum
    if let Err(e) = process_endpoint(
        "IdType",
        IDTYPE_API_URL,
        IDTYPE_CACHE_PATH,
        IDTYPE_VARIANTS_FILE,
        IDTYPE_DOC,
        should_fetch,
        Some(IDTYPE_VARIANTS_DOC_FILE),
    ) {
        eprintln!("Error processing exchange code data: {e}");
    }

    // Process market sector description enum
    if let Err(e) = process_endpoint(
        "MarketSecDesc",
        MARKETSECDESC_API_URL,
        MARKETSECDESC_CACHE_PATH,
        MARKETSECDESC_VARIANTS_FILE,
        MARKETSECDESC_DOC,
        should_fetch,
        Some(MARKETSECDESC_VARIANTS_DOC_FILE),
    ) {
        eprintln!("Error processing market sector description data: {e}");
    }

    // Process mic code enum
    if let Err(e) = process_endpoint(
        "MicCode",
        MICCODE_API_URL,
        MICCODE_CACHE_PATH,
        MICCODE_VARIANTS_FILE,
        MICCODE_DOC,
        should_fetch,
        None,
    ) {
        eprintln!("Error processing mic code data: {e}");
    }

    // Process security type enum
    if let Err(e) = process_endpoint(
        "SecurityType",
        SECURITYTYPE_API_URL,
        SECURITYTYPE_CACHE_PATH,
        SECURITYTYPE_VARIANTS_FILE,
        SECURITYTYPE_DOC,
        should_fetch,
        Some(SECURITYTYPE_VARIANTS_DOC_FILE),
    ) {
        eprintln!("Error processing security type data: {e}");
    }

    // Process security type 2 enum
    if let Err(e) = process_endpoint(
        "SecurityType2",
        SECURITYTYPE2_API_URL,
        SECURITYTYPE2_CACHE_PATH,
        SECURITYTYPE2_VARIANTS_FILE,
        SECURITYTYPE2_DOC,
        should_fetch,
        None,
    ) {
        eprintln!("Error processing security type 2 data: {e}");
    }

    // Process state code enum
    if let Err(e) = process_endpoint(
        "StateCode",
        STATECODE_API_URL,
        STATECODE_CACHE_PATH,
        STATECODE_VARIANTS_FILE,
        STATECODE_DOC,
        should_fetch,
        None,
    ) {
        eprintln!("Error processing state code data: {e}");
    }
}

/// Fetches data from the API and writes it to a cache file.
#[derive(Deserialize, Debug)]
struct MappingValues {
    values: Vec<String>,
}

/// Handles fetching, caching, and generating enum variants for a given endpoint.
fn process_endpoint(
    name: &str,
    api_url: &str,
    cache_path: &str,
    variants_file: &str,
    doc: &str,
    should_fetch: bool,
    doc_file: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = if should_fetch {
        println!("Forcing fresh {name} data fetch from API.");
        fetch_data(api_url, cache_path)?
    } else {
        println!("Attempting to load {name} from cache: {cache_path}");
        fetch_cached_data(cache_path).or_else(|err| {
            println!("-> Cache for {name} failed ('{err}'), fetching from API as fallback.");
            fetch_data(api_url, cache_path)
        })?
    };

    let docs_map = if let Some(path) = doc_file {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let path = Path::new(&manifest_dir).join(path);
        println!("Attempting to load documentation from {}", path.display());
        load_docs_from_csv(&path)?
    } else {
        HashMap::new()
    };

    let attributes = format!(
        "{}\n{}\n{}\n{}\n{}",
        doc,
        r"#[expect(missing_docs)]",
        r"#[allow(non_camel_case_types)]",
        r"#[non_exhaustive]",
        r"#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]"
    );

    // Generate the enum variants file in OUT_DIR
    let out_dir = std::env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join(variants_file);
    generate_enum(name, &attributes, &data, &docs_map, &dest_path)?;

    Ok(())
}

/// Function to determine if a rebuild is necessary
fn should_rebuild() -> bool {
    // Check if it's a release build
    if std::env::var("PROFILE").unwrap_or_default() == "release" {
        println!("Rebuilding because: release build");
        return true;
    }

    // Check if the OPENFIGI_FORCE_REBUILD environment variable is set
    if std::env::var(FORCE_REBUILD).is_ok() {
        println!("Rebuilding because: force rebuild requested");
        return true;
    }

    // Check if the timestamp trigger file exists
    let trigger_path = Path::new(BUILD_TRIGGER_PATH);
    if !trigger_path.exists() {
        println!("Rebuilding because: build trigger file does not exist");
        return true;
    }

    // Check the last modified time of the timestamp trigger file
    if let Ok(modified) = fs::metadata(trigger_path).and_then(|m| m.modified()) {
        if modified
            .elapsed()
            .map(|d| d.as_secs() > TRIGGER_MAX_AGE_SECS)
            .unwrap_or(true)
        {
            println!(
                "Rebuilding because: build trigger file is older than {TRIGGER_MAX_AGE_SECS} seconds"
            );
            return true;
        }
    } else {
        // This block runs if fs::metadata or m.modified() returned an Err
        println!("Rebuilding because: could not read build trigger file metadata");
        return true;
    }

    // If all checks passed, no rebuild is needed
    false
}

/// Function to update the build trigger file
fn update_build_trigger() {
    let trigger_path = Path::new(BUILD_TRIGGER_PATH);

    if let Some(parent) = trigger_path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            eprintln!("Failed to create trigger directory: {e}");
            return;
        }
    }

    match File::create(trigger_path) {
        Ok(_) => println!("Updated build trigger file at {trigger_path:?}"),
        Err(e) => eprintln!("Failed to update build trigger file: {e}"),
    }
}

/// Fetches data from the cache file.
fn fetch_cached_data(cache_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(cache_path)?;
    let data: Vec<String> = serde_json::from_str(&data)?;

    if data.is_empty() {
        return Err("No data found in cache".into());
    }

    Ok(data)
}

/// Fetches data from the API and writes it to a cache file.
fn fetch_data(url: &str, cache_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;

    // Check if the response was successful
    if !response.status().is_success() {
        return Err(format!(
            "API request failed to {url} with status: {}",
            response.status()
        )
        .into());
    }

    let api_response: MappingValues = response.json()?;
    let data = api_response.values;

    if data.is_empty() {
        return Err(format!("No data found in API response from {url}").into());
    }

    fs::write(cache_path, serde_json::to_string(&data)?)?;
    Ok(data)
}

/// Generates the Rust code for enum variants and writes it to a file.
fn generate_enum(
    enum_name: &str,
    attributes: &str,
    data: &[String],
    docs_map: &HashMap<String, String>,
    path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file_content = String::new();

    writeln!(&mut file_content, "{attributes}")?;
    writeln!(&mut file_content, "pub enum {enum_name} {{")?;

    for item in data {
        // Sanitize the name for the Rust enum variant
        let variant_name = generate_variant_name(item);

        // Look up the documentation for the current item.
        if let Some(doc_comment) = docs_map.get(item) {
            // Write the doc comment if found.
            writeln!(&mut file_content, "    /// {doc_comment}")?;
        }

        writeln!(&mut file_content, "    #[serde(rename = \"{item}\")]")?;
        writeln!(&mut file_content, "    {variant_name},")?;
    }

    writeln!(&mut file_content, "}}")?;

    fs::write(path, file_content)?;
    println!("Generated enum variants at: {path:?}");
    Ok(())
}

/// Sanitizes a string to be a valid Rust enum variant name.
fn generate_variant_name(name: &str) -> String {
    // Handle special hardcoded cases
    if name == "***" {
        return "AAA".to_string();
    }

    // Sanitize the string in a clear, chained sequence.
    let sanitized: String = name
        // Replace before filtering to handle cases like "R&D"
        .replace('&', "AND")
        .chars()
        // Keep only alphanumeric and underscore characters
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect();

    // Handle the first character logic more concisely.
    let mut chars = sanitized.chars();
    let mut final_name = match chars.next() {
        // If the first char is a digit, prefix the whole string with an underscore.
        Some(c) if c.is_ascii_digit() => format!("_{sanitized}"),
        // Otherwise, capitalize the first letter and append the rest.
        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
        // Handle empty strings after sanitization.
        None => return String::new(),
    };

    // Check if the generated name is a Rust keyword.
    // If it is, convert it to a raw identifier (e.g., "type" -> "r#type").
    // Note: Serde correctly handles serializing r#type as "type".
    match final_name.as_str() {
        "as" | "break" | "const" | "continue" | "crate" | "else" | "enum" | "extern" | "false"
        | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "match" | "mod" | "move"
        | "mut" | "pub" | "ref" | "return" | "self" | "Self" | "static" | "struct" | "super"
        | "trait" | "true" | "type" | "unsafe" | "use" | "where" | "while" | "async" | "await"
        | "dyn" => {
            final_name = format!("r#{final_name}");
        }
        _ => {}
    }

    final_name
}

/// Loads documentation from a pipe-separated file into a HashMap.
fn load_docs_from_csv(
    path: &PathBuf,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();

    // Use a ReaderBuilder to configure the CSV parser.
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'|') // Specify the pipe as the delimiter.
        .from_path(path)?;

    for result in reader.deserialize() {
        let (key, description): (String, String) = result?;
        map.insert(key, description);
    }
    Ok(map)
}
