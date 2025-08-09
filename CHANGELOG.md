# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.4](https://github.com/NotAProfDev/openfigi-rs/compare/v0.3.3...v0.3.4) - 2025-08-09

### Added

- add Codecov configuration file for coverage reporting

### Fixed

- update Codecov configuration to ignore specific test utility file and adjust coverage report generation
- remove flags configuration from Codecov YAML, since they only work for PRO versions
- Codecov configuration file for coverage reporting
- update chrono dependency version and add dependency status badge to README
- correct casing of Codecov badge in README
- add missing Code Coverage badge to README
- update CI configuration to include llvm-tools-preview and coverage report generation
- add lcov.info to .gitignore to ignore code coverage files
- ensure cargo-llvm-cov is installed and add coverage report generation to CI script
- resolve collapsible if lint to restore CI stability
- correct date range order in tests for accurate validation
- format deserialization line for improved readability in test macro
- replace unwrap with expect for better error handling in tests and request builders
- streamline serde attributes for share_class_figi and composite_figi in FigiResult
- remove redundant default client test for clarity
- correct placement of api_key in examples for clarity and consitency
- reorder dependencies in Cargo.toml for clarity
- remove spaces around OPENFIGI_API_KEY assignment in .env.example
- update repository clone URL in CONTRIBUTING.md

### Other

- added new mapping table between MIC and FIGI exchange codes
# Changelog

All notable changes to the openfigi-rs library will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.3](https://github.com/NotAProfDev/openfigi-rs/compare/v0.3.2...v0.3.3) - 2025-07-19

### Fixed

- update license link to point to the correct GitHub location
- update rate limit header parsing to match OpenFIGI API changes

## [0.3.2](https://github.com/NotAProfDev/openfigi-rs/compare/v0.3.1...v0.3.2) - 2025-07-19

### Fixed

- update license link

## [0.3.1](https://github.com/NotAProfDev/openfigi-rs/compare/v0.3.0...v0.3.1) - 2025-07-19

### Added

- add new resource cache files for currency, exchange codes, ID types, market security descriptions, MIC codes, security types, and state codes as fallback builds

### Fixed

- reduce API rate limit delay from 10 seconds to 5 seconds

### Other

- streamline CI workflow by consolidating jobs so we do not hit API rate limits through concurrent builds
- Changed cache folder structure and improved build script to work with DOC.RS
- Merge branch 'main' of https://github.com/NotAProfDev/openfigi-rs

## [0.3.0](https://github.com/NotAProfDev/openfigi-rs/compare/v0.2.0...v0.3.0) - 2025-07-19

### Added

- add UnexpectedApiResponse error type for handling unexpected API responses

### Fixed

- add delay to API calls to avoid hitting rate limits in CI
- improve error handling in CSV documentation loading by returning an error instead of printing a warning
- update enum variants to use uppercase constants in MappingRequest validation to match automated enum variants

### Other

- update IdType references from IdIsin to ID_ISIN and similar adjustments across mapping requests and tests
- enhance main function with endpoint validation and improve error handling for empty responses
- update header from 'value' to 'key' in enum documentation files
- enhance build script structure and error handling for OpenFIGI API enums
- streamline endpoint configuration and processing in build script
- replace security type enum definition with generated file
- replace MarketSecDesc enum definition with generated file
- replace IdType enum definition with generated file and update test cases
- add csv as a build dependency
- added new enum builds and resources for documentation
- remove enum generation scripts as part of cleanup
- remove StateCode enum definition and include generated file
- remove SecurityType2 enum definition and include generated file
- remove MicCode enum definition and include generated file
- automated ExchCode enum generation
- remove redundant currency enum definition and include generated file
- add build script to automate the enum generation
- fix emoji in Configuration section for consistency
- update comment for PLAZOSFIJOS to clarify description
- enhanced docs
- improve README and mapping.rs documentation for clarity and consistency
- add anyhow as test dependency for improved error handling and update README structure
- mark internal modules and methods as hidden for cleaner API documentation
- update dependencies in Cargo.toml
- remove unused common.rs file

## [0.2.0](https://github.com/NotAProfDev/openfigi-rs/compare/v0.1.1...v0.2.0) - 2025-07-03

### Other

- enhance documentation for test utilities module
- consolidate test utilities and improve enum serialization tests
- replace impl_filterable_builder macro with impl_filter_builder for consistency across request builders
- introduce macro for standard filter builder methods across request types
- implement filterable builder macros for endpoint request builders
- improve response handling and error reporting in OpenFIGI API
- response types and error handling in OpenFIGI API

## [0.1.1](https://github.com/NotAProfDev/openfigi-rs/compare/v0.1.0...v0.1.1) - 2025-06-17

### Fixed

- update changelog to mark version 0.1.0 release date

## [0.1.0] - 2025-06-17

### Added

- Complete OpenFIGI API client implementation with type-safe interfaces
- Support for all three main API endpoints:
  - `/mapping` - Convert third-party identifiers to FIGIs (supports batching)
  - `/search` - Text-based instrument search with filtering
  - `/filter` - Filter instruments by specific criteria
- Comprehensive request/response data models with validation
- Fluent API with builder patterns for easy request construction
- Environment variable support for API key configuration (`OPENFIGI_API_KEY`)
- Extensive error handling with OpenFIGI-specific context and inspection methods
- Connection pooling and efficient HTTP client management
- Middleware support for retries, logging, and observability
- Rate limit awareness with automatic detection and informative error messages
- Async/await support built on reqwest with full HTTP middleware stack
- Comprehensive documentation with multiple usage examples
- Type-safe enumerations for all API parameters (currencies, exchanges, security types, etc.)
- Production-ready configuration examples with retry policies and timeouts

### Features

- **Client Builder Pattern**: Flexible configuration via `OpenFIGIClientBuilder`
- **Batch Operations**: Support for bulk mapping requests (up to 100 with API key, 5 without)
- **Authentication**: Automatic API key handling with higher rate limits (25 requests/6 seconds vs 25/minute)
- **Error Context**: Detailed error messages with rate limit information and retry suggestions
- **Response Handling**: Unified `ResponseResult` type for handling success/error cases
- **Thread Safety**: Cheap cloning with `Arc` internally for sharing across async tasks
- **Validation**: Automatic request validation with helpful error messages

### Documentation

- Comprehensive API documentation with realistic examples
- Complete module documentation for all public interfaces
- Production configuration patterns for enterprise usage
- OpenFIGI API endpoint overview with capabilities and limitations
- Authentication and rate limiting documentation
- Error handling examples with OpenFIGI-specific context
- Multiple usage patterns from basic to advanced configurations

### Architecture

- **Modular Design**: Organized into clear modules (client, endpoints, models, errors)
- **Type Safety**: Strongly typed request/response models with compile-time validation
- **Builder Patterns**: Fluent APIs for both client configuration and request building
- **Internal Privacy**: Internal modules (`request_builder`) properly encapsulated
- **Performance**: Optimized for minimal allocations and efficient resource usage
