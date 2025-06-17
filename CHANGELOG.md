# Changelog

All notable changes to the openfigi-rs library will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
