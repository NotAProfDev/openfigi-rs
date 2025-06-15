# OpenFIGI Rust Client

[![Crates.io](https://img.shields.io/crates/v/openfigi-rs.svg)](https://crates.io/crates/openfigi-rs)
[![Documentation](https://docs.rs/openfigi-rs/badge.svg)](https://docs.rs/openfigi-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/NotAProfDev/openfigi-rs/workflows/CI/badge.svg)](https://github.com/NotAProfDev/openfigi-rs/actions)

> **ℹ️ API Notice:** The API is subject to change in the future and is not yet stable.

A high-performance Rust client library for the [OpenFIGI API](https://www.openfigi.com/api), providing type-safe access to financial instrument identification and mapping services.

OpenFIGI is Bloomberg's open symbology initiative that provides standardized identification for financial instruments across asset classes and markets worldwide.

## ✨ Features

- **🔒 Type-safe API** - Strongly typed request/response models with compile-time validation
- **⚡ Async/await support** - Built on `reqwest` with full async support and connection pooling
- **🔧 Middleware support** - Extensible HTTP middleware for retries, logging, and observability
- **📊 Comprehensive error handling** - Detailed error types with OpenFIGI-specific context
- **🚀 Production ready** - Connection pooling, timeouts, and efficient resource management
- **🔑 Environment integration** - Automatic API key detection from environment variables
- **📈 Rate limit awareness** - Automatic rate limit detection and informative error messages
- **🔄 Batch operations** - Support for bulk requests (up to 100 with API key, 5 without)

## 🚀 Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
openfigi-rs = "<latest-version>"
```

### Basic Usage

```rust
use openfigi_rs::client::OpenFIGIClient;
use openfigi_rs::model::enums::IdType;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client (uses OPENFIGI_API_KEY env var if available)
    let client = OpenFIGIClient::new();

    // Map an ISIN to FIGI
    let mapping_results = client
        .mapping(IdType::IdIsin, json!("US4592001014"))
        .send()
        .await?;

    // Access the FIGI from the first result
    if let Some(data) = mapping_results[0].data() {
        println!("FIGI: {}", data[0].figi);
        println!("Name: {}", data[0].display_name());
    }

    Ok(())
}
```

### With Custom Configuration

```rust
use openfigi_rs::client::OpenFIGIClient;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure HTTP client with retry middleware
    let http_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let middleware_client = ClientBuilder::new(http_client)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    let client = OpenFIGIClient::builder()
        .middleware_client(middleware_client)
        .api_key("your-api-key")
        .build()?;

    Ok(())
}
```

## 🔑 Authentication & Rate Limits

### API Key Setup

Set your API key as an environment variable:

```bash
export OPENFIGI_API_KEY="your-api-key"
```

Or configure it explicitly:

```rust
let client = OpenFIGIClient::builder()
    .api_key("your-api-key")
    .build()?;
```

### Rate Limits

| Type of Limitation       | Without API Key | With API Key   |
|--------------------------|----------------|---------------|
| Max Amount of Requests  | 25 Per Minute   | 25 Per 6 Seconds |
| Max Jobs Per Request    | 10 Jobs        | 100 Jobs      |

## 📚 API Endpoints

### Mapping Endpoint

Convert third-party identifiers to FIGIs:

```rust
use openfigi_rs::model::enums::{IdType, Currency, ExchCode};

// Single mapping request
let result = client
    .mapping(IdType::IdIsin, "US4592001014")
    .currency(Currency::USD)
    .exch_code(ExchCode::US)
    .send()
    .await?;

// Bulk mapping request
use openfigi_rs::model::request::MappingRequest;

let requests = vec![
    MappingRequest::new(IdType::IdIsin, json!("US4592001014")),
    MappingRequest::new(IdType::Ticker, json!("AAPL")),
];

let results = client
    .bulk_mapping()
    .add_requests(requests)
    .send()
    .await?;
```

### Search Endpoint

Text-based instrument search:

```rust
use openfigi_rs::model::enums::Currency;

let results = client
    .search("apple")
    .currency(Currency::USD)
    .send()
    .await?;
```

### Filter Endpoint

Filter instruments by criteria:

```rust
use openfigi_rs::model::enums::SecurityType;

let results = client
    .filter()
    .query("technology")
    .security_type(SecurityType::CommonStock)
    .send()
    .await?;
```

## 🛠️ Advanced Usage

### Error Handling

```rust
use openfigi_rs::error::OpenFIGIError;

match client.mapping(IdType::IdIsin, json!("INVALID")).send().await {
    Ok(results) => {
        for result in &results {
            match result.data() {
                Some(data) => println!("Found {} FIGIs", data.len()),
                None => println!("Error: {}", result.error().unwrap_or("Unknown")),
            }
        }
    }
    Err(OpenFIGIError::ReqwestError(e)) if e.is_timeout() => {
        println!("Request timed out - consider retry");
    }
    Err(OpenFIGIError::ResponseError(e)) if e.status.as_u16() == 429 => {
        println!("Rate limit exceeded - implement backoff");
    }
    Err(e) => println!("Other error: {}", e),
}
```

### Production Configuration

```rust
use std::time::Duration;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};

let http_client = reqwest::Client::builder()
    .timeout(Duration::from_secs(30))
    .connect_timeout(Duration::from_secs(5))
    .pool_idle_timeout(Duration::from_secs(90))
    .pool_max_idle_per_host(10)
    .build()?;

let retry_policy = ExponentialBackoff::builder()
    .retry_bounds(Duration::from_millis(100), Duration::from_secs(30))
    .build_with_max_retries(3);

let middleware_client = ClientBuilder::new(http_client)
    .with(RetryTransientMiddleware::new_with_policy(retry_policy))
    .build();

let client = OpenFIGIClient::builder()
    .middleware_client(middleware_client)
    .api_key(std::env::var("OPENFIGI_API_KEY")?)
    .build()?;
```

## 📖 Documentation

- [API Documentation](https://docs.rs/openfigi-rs) - Complete API reference
- [OpenFIGI API Documentation](https://www.openfigi.com/api/documentation) - Official API docs

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [OpenFIGI](https://www.openfigi.com/) for providing the API and documentation
- [Bloomberg](https://www.bloomberg.com/) for the OpenFIGI initiative
- The Rust community for excellent HTTP and async libraries

## 📞 Support

- 📚 [Documentation](https://docs.rs/openfigi-rs)
- 🐛 [Issue Tracker](https://github.com/NotAProfDev/openfigi-rs/issues)
- 💬 [Discussions](https://github.com/NotAProfDev/openfigi-rs/discussions)

---

**Note**: This library is not officially affiliated with Bloomberg or OpenFIGI. It's an independent implementation of the OpenFIGI API client for Rust.
