# AI Project Guidelines

Primary Goal: Consistency Analyze the existing codebase and strictly match its style, patterns, naming, and architecture.

## Core Principles

- Idiomatic Rust: Use standard ownership, `Result`/`Option`, iterators, and pattern matching.
- Safety: No `unsafe` without explicit justification.
- Simplicity: Prefer clear, simple code.
- DRY: Refactor to avoid duplication.
- Performance: Avoid premature optimization. Be mindful of allocations (`String`, `Vec`) and `.clone()` in hot paths. Use iterators.

## Specific Rules

- Formatting: Adhere to `rustfmt`.
- Linting: Pass `clippy::pedantic`.
- Error Handling: Use specific `Result<T, E>` types and `Option<T>`. No `unwrap()`/`expect()` on fallible operations.
- Dependencies: Minimize new dependencies and discuss additions.
- Testing: Write unit tests for new logic.
- Documentation: Write `rustdoc` for public APIs and complex logic (`why` and `how`).
- Changelog: For any user-facing change, suggest a `CHANGELOG.md` entry following the "Keep a Changelog" format for the `[Unreleased]` section.
