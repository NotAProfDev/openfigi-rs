//! This build script fetches data from the OpenFIGI API

use serde::Deserialize;
use std::{
    collections::HashMap,
    env,
    error::Error,
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

// A single struct to hold all configuration for an endpoint.
struct EndpointConfig<'a> {
    name: &'a str,
    api_url: &'a str,
    cache_path: &'a str,
    variants_file: &'a str,
    doc_comment: &'a str,
    doc_file_path: Option<PathBuf>,
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Central Configuration
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let endpoints = [
        EndpointConfig {
            name: "Currency",
            api_url: "https://api.openfigi.com/v3/mapping/values/currency",
            cache_path: "target/currency_cache.json",
            variants_file: "currency_enum.rs",
            doc_comment: "/// Enum for all supported currency codes.",
            doc_file_path: None,
        },
        EndpointConfig {
            name: "ExchCode",
            api_url: "https://api.openfigi.com/v3/mapping/values/exchCode",
            cache_path: "target/exch_code_cache.json",
            variants_file: "exch_code_enum.rs",
            doc_comment: "/// Enum for all supported exchange codes.",
            doc_file_path: None,
        },
        EndpointConfig {
            name: "IdType",
            api_url: "https://api.openfigi.com/v3/mapping/values/idType",
            cache_path: "target/id_type_cache.json",
            variants_file: "id_type_enum.rs",
            doc_comment: "/// Enum for all supported ID types.",
            doc_file_path: Some(Path::new(&manifest_dir).join("resources/id_type_enum_docs.csv")),
        },
        EndpointConfig {
            name: "MarketSecDesc",
            api_url: "https://api.openfigi.com/v3/mapping/values/marketSecDes",
            cache_path: "target/market_sec_desc_cache.json",
            variants_file: "market_sec_desc_enum.rs",
            doc_comment: "/// Enum for all supported market sector descriptions.",
            doc_file_path: Some(
                Path::new(&manifest_dir).join("resources/market_sec_desc_enum_docs.csv"),
            ),
        },
        EndpointConfig {
            name: "MicCode",
            api_url: "https://api.openfigi.com/v3/mapping/values/micCode",
            cache_path: "target/mic_code_cache.json",
            variants_file: "mic_code_enum.rs",
            doc_comment: "/// Enum for all supported market identifiers codes.",
            doc_file_path: None,
        },
        EndpointConfig {
            name: "SecurityType",
            api_url: "https://api.openfigi.com/v3/mapping/values/securityType",
            cache_path: "target/security_type_cache.json",
            variants_file: "security_type_enum.rs",
            doc_comment: "/// Enum for all supported security types.",
            doc_file_path: Some(
                Path::new(&manifest_dir).join("resources/security_type_enum_docs.csv"),
            ),
        },
        EndpointConfig {
            name: "SecurityType2",
            api_url: "https://api.openfigi.com/v3/mapping/values/securityType2",
            cache_path: "target/security_type2_cache.json",
            variants_file: "security_type2_enum.rs",
            doc_comment: "/// Enum for all supported security types 2.",
            doc_file_path: None,
        },
        EndpointConfig {
            name: "StateCode",
            api_url: "https://api.openfigi.com/v3/mapping/values/stateCode",
            cache_path: "target/state_code_cache.json",
            variants_file: "state_code_enum.rs",
            doc_comment: "/// Enum for all supported state codes.",
            doc_file_path: None,
        },
    ];

    // --- Automatically add rerun-if-changed for all doc files ---
    for endpoint in &endpoints {
        if let Some(path) = &endpoint.doc_file_path {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    // Check if we should rebuild/refresh the data
    let should_fetch = should_rebuild();
    if should_fetch {
        update_build_trigger();
    }

    for config in &endpoints {
        if let Err(e) = process_endpoint(config, should_fetch) {
            eprintln!("Error processing '{}' data: {}", config.name, e);
        }
    }
}

/// Fetches data from the API and writes it to a cache file.
#[derive(Deserialize, Debug)]
struct MappingValues {
    values: Vec<String>,
}

/// Handles fetching, caching, and generating enum variants for a given endpoint.
fn process_endpoint(config: &EndpointConfig, should_fetch: bool) -> Result<(), Box<dyn Error>> {
    let data = if should_fetch {
        println!("Forcing fresh {} data fetch from API.", config.name);
        fetch_data(config.api_url, config.cache_path)?
    } else {
        println!(
            "Attempting to load {} from cache: {}",
            config.name, config.cache_path
        );
        fetch_cached_data(config.cache_path).or_else(|err| {
            println!(
                "-> Cache for {} failed ('{}'), fetching from API as fallback.",
                config.name, err
            );
            fetch_data(config.api_url, config.cache_path)
        })?
    };

    let docs_map = if let Some(path) = &config.doc_file_path {
        load_docs_from_csv(path)?
    } else {
        HashMap::new()
    };

    let attributes = format!(
        "{}\n{}\n{}\n{}\n{}",
        config.doc_comment,
        r"#[allow(missing_docs)]",
        r"#[allow(non_camel_case_types)]",
        r"#[non_exhaustive]",
        r"#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]"
    );

    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join(config.variants_file);
    generate_enum(config.name, &attributes, &data, &docs_map, &dest_path)?;
    Ok(())
}

/// Function to determine if a rebuild is necessary
fn should_rebuild() -> bool {
    // Check if it's a release build
    if env::var("PROFILE").unwrap_or_default() == "release" {
        println!("Rebuilding because: release build");
        return true;
    }

    // Check if the OPENFIGI_FORCE_REBUILD environment variable is set
    if env::var(FORCE_REBUILD).is_ok() {
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
fn fetch_cached_data(cache_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let data = fs::read_to_string(cache_path)?;
    let data: Vec<String> = serde_json::from_str(&data)?;

    if data.is_empty() {
        return Err("No data found in cache".into());
    }

    Ok(data)
}

/// Fetches data from the API and writes it to a cache file.
fn fetch_data(url: &str, cache_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
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
) -> Result<(), Box<dyn Error>> {
    let mut file_content = String::new();

    writeln!(&mut file_content, "{attributes}")?;
    writeln!(&mut file_content, "pub enum {enum_name} {{")?;

    for item in data {
        // Sanitize the name for the Rust enum variant
        let variant_name = generate_variant_name(item);

        // Look up the documentation for the current item.
        if let Some(doc_comment) = docs_map.get(item) {
            writeln!(&mut file_content, "    /// {doc_comment}")?;
        }
        writeln!(&mut file_content, "    #[serde(rename = \"{item}\")]")?;
        writeln!(&mut file_content, "    {variant_name},")?;
    }

    writeln!(&mut file_content, "}}")?;
    fs::write(path, file_content)?;
    println!("Generated enum '{}' at: {}", enum_name, path.display());
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

/// Loads documentation from a pipe-separated file into a `HashMap`.
fn load_docs_from_csv(path: &PathBuf) -> Result<HashMap<String, String>, Box<dyn Error>> {
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
