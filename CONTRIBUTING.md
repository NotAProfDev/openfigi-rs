# Contributing to openfigi-rs

Thank you for your interest in contributing to openfigi-rs! ❤️ Your support and collaboration help make this library better for the entire Rust community.

## 📋 Table of Contents

- [How to Contribute](#-how-to-contribute)
- [Development Setup](#️-development-setup)
- [Development Guidelines](#-development-guidelines)
- [Testing](#-testing)
- [Documentation](#-documentation)
- [Commit Guidelines](#-commit-guidelines)
- [Pull Request Process](#-pull-request-process)
- [Code of Conduct](#code-of-conduct)

## 🤝 How to Contribute

We welcome contributions in various forms:

- 🐛 **Bug fixes** - Help us identify and fix issues
- ✨ **New features** - Add support for new OpenFIGI endpoints or functionality
- 📚 **Documentation** - Improve examples, API docs, or guides
- 🧪 **Tests** - Add test coverage or improve existing tests
- 🔧 **Performance** - Optimize existing code or reduce dependencies
- 🎨 **Code quality** - Refactoring, code cleanup, or style improvements

### Before You Start

- 📝 **Check existing issues** - Browse [open issues](https://github.com/NotAProfDev/openfigi-rs/issues) to see if your idea is already being discussed
- 💬 **Create an issue first** - For major features or breaking changes, please create an issue to discuss the approach
- 📋 **Use issue templates** - Use the provided templates for bug reports and feature requests

## 🛠️ Development Setup

### Prerequisites

- **Rust 1.70+** (2021 edition or later)
- **Git** for version control
- **OpenFIGI API key** (optional, but recommended for testing with higher rate limits)

### Getting Started

1. **Fork and clone the repository**

   ```bash
   git clone https://github.com/YOUR_USERNAME/openfigi-rs.git
   cd openfigi-rs
   ```

2. **Set up environment variables** (optional)

   ```bash
   export OPENFIGI_API_KEY="your-api-key-here"
   ```

3. **Create a feature branch**

   ```bash
   git checkout -b feature/my-awesome-feature
   ```

4. **Build the project**

   ```bash
   cargo build
   ```

5. **Run tests to ensure everything works**

   ```bash
   cargo test
   ```

### Development Container (Recommended)

For a consistent development environment, use the provided dev container:

1. **Install Docker and VS Code with Dev Containers extension**
2. **Open the project in VS Code**
3. **Use Command Palette** (`Ctrl+Shift+P`) → "Dev Containers: Reopen in Container"
4. **The container will automatically set up the environment** with all necessary tools

See `.devcontainer/` directory for configuration details.

## 🔧 Development Guidelines

### Code Style

- Follow **standard Rust formatting** using `cargo fmt`
- Use **meaningful variable and function names**
- Add **comprehensive documentation** for public APIs
- Include **examples in documentation** where helpful
- Follow **existing patterns** in the codebase

### Architecture Principles

- **Type safety first** - Use Rust's type system to prevent errors
- **Async-first design** - All API operations should be async
- **Error handling** - Provide detailed, actionable error messages
- **Performance** - Minimize allocations and optimize for common use cases
- **Modularity** - Keep related functionality organized in logical modules

### Adding New Features

1. **API Endpoints** - Add new endpoint support in `src/endpoint/`
2. **Data Models** - Add request/response models in `src/model/`
3. **Enums** - Add new enums in `src/model/enums/` (use generation scripts when possible)
4. **Error Types** - Extend error handling in `src/error.rs` if needed

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run doctests only
cargo test --doc

# Run integration tests
cargo test --test integration
```

### Test Guidelines

- **Unit tests** - Test individual functions and modules
- **Integration tests** - Test complete workflows and API interactions
- **Documentation tests** - Ensure all examples in docs work correctly
- **Mock responses** - Use test data in `tests/data/` for consistent testing

### Adding Test Data

When adding new test cases:

1. **Use existing test data** from `tests/data/` when possible
2. **Add new test files** following the existing structure
3. **Anonymize sensitive data** in test fixtures
4. **Document test scenarios** clearly

## 📚 Documentation

### Documentation Standards

- **Public APIs** must have comprehensive documentation
- **Include examples** for non-trivial functionality
- **Document error conditions** and how to handle them
- **Keep examples up-to-date** with API changes

### Building Documentation

```bash
# Build and open documentation
cargo doc --no-deps --open

# Check for broken links
cargo doc --no-deps 2>&1 | grep -i warning
```

## 📝 Commit Guidelines

We use [Conventional Commits](https://www.conventionalcommits.org/) for automated changelog generation and semantic versioning.

### Commit Format

```git
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring without functional changes
- `test:` - Adding or modifying tests
- `chore:` - Maintenance tasks, dependency updates

### Examples

```bash
feat: add support for /filter endpoint
fix: handle rate limit responses correctly
docs: add examples for bulk mapping requests
test: add integration tests for search endpoint
```

## 🔄 Pull Request Process

### Before Submitting

1. **Run the CI test suite locally**

   ```bash
   ./scripts/test-ci.sh
   ```

2. **Update documentation** if needed
3. **Add tests** for new functionality
4. **Update CHANGELOG.md** if adding features or fixing bugs
5. **Ensure clean commit history** (squash if necessary)

### PR Guidelines

- **Clear title and description** - Explain what and why
- **Link related issues** - Use "Closes #123" or "Fixes #456"
- **Small, focused changes** - Easier to review and merge
- **Update documentation** - Keep docs in sync with code changes
- **Respond to feedback** - Address review comments promptly

### Review Process

1. **Automated checks** must pass (CI, formatting, linting)
2. **Code review** by maintainers
3. **Testing** - Verify functionality works as expected
4. **Merge** - Typically using "Squash and merge"

## ⚡ Performance Considerations

When contributing, keep these performance guidelines in mind:

- **Minimize allocations** - Reuse buffers where possible
- **Efficient serialization** - Use `serde` efficiently
- **Connection pooling** - Leverage HTTP client pooling
- **Async best practices** - Avoid blocking operations

## 🐛 Debugging

### Useful Commands

```bash
# Enable debug logging
RUST_LOG=debug cargo test

# Run specific test with output
cargo test test_name -- --nocapture

# Check for common issues
cargo clippy -- -D warnings

# Format code
cargo fmt
```

## 🎯 Areas for Contribution

Looking for ways to contribute? Consider these areas:

- **Pagination support** - Add pagination support for the corresponding endpoints
- **Performance optimization** - Profile and optimize hot paths
- **Error handling** - Improve error messages and recovery
- **Documentation** - More examples and better explanations
- **Testing** - Increase coverage and add edge cases
- **Tooling** - Improve development scripts and automation

## Code of Conduct

We are committed to fostering a respectful, inclusive, and welcoming community. By participating in this project, you agree to abide by our [Code of Conduct](./CODE_OF_CONDUCT.md).

## 🆘 Getting Help

- 💬 **Discussions** - [GitHub Discussions](https://github.com/NotAProfDev/openfigi-rs/discussions)
- 🐛 **Issues** - [GitHub Issues](https://github.com/NotAProfDev/openfigi-rs/issues)
- 📧 **Email** - Reach out to maintainers for sensitive topics

---

Thank you for your contributions—your efforts help make openfigi-rs even better! 🚀
