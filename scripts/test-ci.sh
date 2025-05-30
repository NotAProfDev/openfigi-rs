#!/usr/bin/env bash
set -euo pipefail

# Install typos, if not already installed
if ! command -v typos &>/dev/null; then
    cargo install typos-cli --locked
    echo "Installed typos"
fi

# Install and run cargo-audit
if ! command -v cargo-audit &>/dev/null; then
    cargo install cargo-audit --locked
    echo "Installed cargo-audit"
fi

# Check typos
(echo "Checking for typos..." && typos) || exit 1

# Check formatting
(echo "Checking formatting..." && cargo fmt --all -- --check) || exit 1

# Check documentation
(echo "Checking documentation..." && cargo doc --workspace --no-deps --document-private-items) || exit 1

# Lint
(echo "Linting code..." && cargo clippy --all-targets --all-features -- -D warnings) || exit 1

# Build
(echo "Building code..." && cargo build --tests) || exit 1

# Run tests
(echo "Running tests..." && cargo test --all-targets --all-features) || exit 1

# Run doc-tests
(echo "Running doc-tests..." && cargo test --all-features --doc) || exit 1

# Audit dependencies for security vulnerabilities
(echo "Auditing dependencies..." && cargo audit) || exit 1
