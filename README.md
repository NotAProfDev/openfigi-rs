# OpenFIGI Rust Client

[![Crates.io](https://img.shields.io/crates/v/openfigi-rs.svg)](https://crates.io/crates/openfigi-rs)
[![Documentation](https://docs.rs/openfigi-rs/badge.svg)](https://docs.rs/openfigi-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/NotAProfDev/openfigi-rs/workflows/CI/badge.svg)](https://github.com/NotAProfDev/openfigi-rs/actions)
[![Downloads](https://img.shields.io/crates/d/openfigi-rs.svg)](https://crates.io/crates/openfigi-rs)

A high-performance asynchronous Rust client library for the [OpenFIGI API](https://www.openfigi.com/api), providing type-safe access to financial instrument identification and mapping services.

OpenFIGI is Bloomberg's open symbology initiative that provides standardized identification for financial instruments across asset classes and markets worldwide.

## 📖 Table of Contents

- [Features](#-features)
- [Getting Started](#-getting-started)
- [Configuration](#-configuration)
- [API Usage Examples](#-api-usage-examples)
- [Error Handling](#-error-handling)
- [Documentation](#-documentation)
- [Contributing](#-contributing)
- [License](#-license)
- [Acknowledgments](#-acknowledgments)
- [Support](#-support)

## ✨ Features

This library is designed with a focus on ergonomics, correctness, and production readiness.

- **⚖️ Ergonomic:** Provide a simple, intuitive, and fluent builder API.
- **🔒 Type-safe API:** Strongly-typed request and response models prevent invalid data.
- **⚡ Fully Asynchronous:** Built on `tokio` and `reqwest` for high-concurrency applications.
- **🔧 Extensible via Middleware:** Integrates with `reqwest-middleware` for custom logic like retries, logging, and tracing.
- **📊 Ergonomic Error Handling:** Provides distinct error types for network issues, API errors, and invalid requests.
- **🔑 Environment integration:** Automatically detects API keys from environment variables.
- **📈 Built-in Rate Limit Handling:** The client is aware of API rate limits and provides informative errors when they are exceeded.
- **🔄 Batch operations:** First-class support for bulk operations to minimize network round-trips (up to 100 with API key, 5 without).

## 🚀 Getting Started

First, add the crate to your project's dependencies:

```bash
cargo add openfigi-rs
```

### Basic Usage

```rust,no_run
use openfigi_rs::client::OpenFIGIClient;
use openfigi_rs::model::enums::IdType;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a client. It will use the OPENFIGI_API_KEY env var if available.
    let client = OpenFIGIClient::new();

    // Map an ISIN to its corresponding FIGI
    let mapping_results = client
        .mapping(IdType::IdIsin, "US4592001014") // IBM
        .send()
        .await?;

    // The result is a vector of responses. Let's inspect the first one.
    let data = mapping_results.data();
    println!("FIGI: {}", data[0].figi);
    println!("Name: {}", data[0].display_name());

    // You can also pretty-print the full debug output
    // println!("{:#?}", mapping_results);

    Ok(())
}
```

## 🔧 Configuration

### API Key

The client can be configured with an API key to access higher rate limits.

#### 1. Environment Variable (Recommended)

The client automatically detects the `OPENFIGI_API_KEY` environment variable.

```bash
export OPENFIGI_API_KEY="your-secret-key"
```

#### 2. Manual Configuration

You can also provide the key explicitly using the builder pattern.

```rust,no_run
# use openfigi_rs::client::OpenFIGIClient;
# #[tokio::main]
# async fn main() -> anyhow::Result<()> {
#    
let client = OpenFIGIClient::builder()
    .api_key("your-secret-key")
    .build()?;
# Ok(())
# }
```

### Custom HTTP Client & Middleware

For production environments, you'll want to configure timeouts and retry policies. This library is built on `reqwest` and `reqwest-middleware`, making customization easy.

```rust,no_run
use openfigi_rs::client::OpenFIGIClient;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // 1. Create a base reqwest client with timeouts
    let http_client = reqwest::Client::builder()
    .timeout(Duration::from_secs(15))
    .connect_timeout(Duration::from_secs(5))
    .build()?;

    // 2. Configure a retry policy
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);

    // 3. Build the middleware client
    let middleware_client = ClientBuilder::new(http_client)
    .with(RetryTransientMiddleware::new_with_policy(retry_policy))
    .build();

    // 4. Build the OpenFIGI client with the custom middleware client
    let client = OpenFIGIClient::builder()
    .middleware_client(middleware_client)
    .api_key("your-secret-key")
    .build()?;

    Ok(())
}
```

### Rate Limits

| Limitation           | Without API Key | With API Key               |
| -------------------- | --------------- | -------------------------- |
| **Request Rate**     | 25 per minute   | 250 per minute (25 per 6s) |
| **Jobs per Request** | 10 jobs         | 100 jobs                   |

## 📚 API Usage Examples

The client supports all three OpenFIGI API v3 endpoints.

| Endpoint       | Purpose                                         | Batch Support |
| -------------- | ----------------------------------------------- | ------------- |
| **/mapping** | Map third-party identifiers to FIGIs.           | **✓**         |
| **/search**  | Perform a text-based search for instruments.    | ✗             |
| **/filter**  | Search for instruments using specific criteria. | ✗             |

### Mapping Endpoint

Convert third-party identifiers to FIGIs:

```rust,no_run
use openfigi_rs::client::OpenFIGIClient;
use openfigi_rs::model::enums::{IdType, Currency, ExchCode};
use openfigi_rs::model::request::MappingRequest;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = OpenFIGIClient::new();

    // Single mapping request with optional parameters
    let single_result = client
        .mapping(IdType::IdIsin, "US4592001014")
        .currency(Currency::USD)
        .exch_code(ExchCode::US)
        .send()
        .await?;

    // Bulk mapping request for multiple identifiers using a prebuilt vector of requests
    let requests = vec![
        MappingRequest::new(IdType::IdIsin, "US4592001014"),
        MappingRequest::new(IdType::Ticker, "AAPL"),
    ];

    let bulk_results = client
        .bulk_mapping()
        .add_requests(requests)
        .send()
        .await?;

    // Bulk mapping request with inline closures
    let result = client
        .bulk_mapping()
        .add_request_with(|j| {
            // Simple mapping request
            j.id_type(IdType::IdIsin)
                .id_value("US4592001014")
        })?
        .add_request_with(|j| { 
            // Complex mapping request with filters
            j.id_type(IdType::Ticker)
                .id_value("IBM")
                .currency(Currency::USD)
                .exch_code(ExchCode::US)
         })?
         .send()
         .await?;

    Ok(())
}
```

### Search Endpoint

Text-based instrument search:

```rust,no_run
use openfigi_rs::client::OpenFIGIClient;
use openfigi_rs::model::enums::Currency;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = OpenFIGIClient::new();

    let results = client
        .search("apple")
        .currency(Currency::USD)
        .send()
        .await?;

    Ok(())
}
```

### Filter Endpoint

Filter instruments by criteria:

```rust,no_run
use openfigi_rs::client::OpenFIGIClient;
use openfigi_rs::model::enums::SecurityType;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = OpenFIGIClient::new();

    let results = client
        .filter()
        .query("technology")
        .security_type(SecurityType::CommonStock)
        .send()
        .await?;

    Ok(())
}
```

## 🚨 Error Handling

The library provides a comprehensive `OpenFIGIError` enum. A common task is handling responses in a bulk request where some jobs may succeed and others may fail.

The API returns a `200 OK` with a body containing either a `data` array or an `error` message for each job.

```rust,no_run
use openfigi_rs::client::OpenFIGIClient;
use openfigi_rs::error::OpenFIGIError;
use openfigi_rs::model::{enums::IdType, request::MappingRequest};

async fn handle_mapping() -> anyhow::Result<()> {
    let client = OpenFIGIClient::new();

    let requests = vec![
        MappingRequest::new(IdType::IdIsin, "US4592001014"), // Valid
        MappingRequest::new(IdType::IdIsin, "INVALID_ISIN"), // Invalid
    ];

    match client.bulk_mapping().add_requests(requests).send().await {
        Ok(mapping_results) => {
            // Handle successful results
            for (_index, data) in mapping_results.successes() {
                println!("SUCCESS: Found {} instruments.", data.data().len());
            }
            
            // Handle failed results
            for (_index, error) in mapping_results.failures() {
                println!("API ERROR: {}", error);
            }
        }
        // Handle network errors, timeouts, etc.
        Err(e) => {
            eprintln!("An unexpected network error occurred: {}", e);
        }
    }

    Ok(())
}
```

## 📖 Documentation

- [API Documentation](https://docs.rs/openfigi-rs) - Complete technical documentation for this crate.
- [OpenFIGI API Documentation](https://www.openfigi.com/api/documentation) - The upstream API documentation from OpenFIGI.

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to submit patches, report issues, and suggest features.

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [OpenFIGI](https://www.openfigi.com/) for providing the public API.
- [Bloomberg](https://www.bloomberg.com/) for the OpenFIGI initiative.
- [OMG](https://www.omg.org/spec/FIGI) for providing documentation.
- The Rust community for creating amazing libraries like `reqwest`, `reqwest-middleware`, `serde`, and `tokio`.

## 📞 Support

For help with this library, please use the following resources:

- **📚 [Documentation](https://docs.rs/openfigi-rs):** Check the API reference for detailed information.
- **🐛 Issues:** For bugs and feature requests, please use the [GitHub Issue Tracker](https://github.com/NotAProfDev/openfigi-rs/issues).
- **💬 Discussions:** For questions and general discussion, please use the [GitHub Discussions](https://github.com/NotAProfDev/openfigi-rs/discussions).

----------------------------------------------------------------------------------------------------

**Disclaimer**: This library is an independent project and is not officially affiliated with, endorsed by, or sponsored by Bloomberg L.P. or the OpenFIGI project.
