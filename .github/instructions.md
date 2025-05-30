# AI Project Guidelines

**Core Guidelines:**

1. **Idiomatic Rust:** Prioritize standard Rust idioms (ownership, borrowing, `Result`/`Option`, iterators, traits, pattern matching).
2. **Safety:** Uphold Rust's safety guarantees. Avoid `unsafe` unless absolutely necessary. Provide clear justification and extensive comments when used.
3. **Consistency:** **Crucially, match the style, patterns, naming conventions, and architectural choices of the surrounding code.** Before generating new code, analyze existing modules in the relevant area.
4. **Clarity & Simplicity:** Prefer clear and simple solutions over overly complex ones, balancing with performance needs.
5. **DRY (Don't Repeat Yourself):** Minimize duplication by refactoring common logic into reusable functions, traits, or modules. Avoid redundant calculations, excessive boilerplate, and repetitive patterns. Favor abstraction without sacrificing readability.

**Specific Guidelines:**

- **Formatting:** Strictly adhere to `rustfmt` using the project's `rustfmt.toml` (if present) or default settings.
- **Linting:** Ensure code passes `clippy::pedantic` with minimal justified allowances. Address lints proactively.
- **Error Handling:**
  - Use `Result<T, E>` for recoverable errors. Prefer specific error types (using `thiserror` if used in the project, or custom enums/structs) over generic ones like `Box<dyn Error>`.
  - Use `Option<T>` for optional values.
  - Avoid `panic!` except for unrecoverable states or logic errors during development (which should ideally be caught by tests). Do not use `unwrap()` or `expect()` on `Result`/`Option` without clear justification that a panic is acceptable/intended.
- **Performance:**
  - **Balance:** Strive for a balance between performance, clarity, and maintainability. Avoid premature optimization.
  - **Allocations:** Be mindful of heap allocations (`String`, `Vec`, `Box`) within hot loops or performance-sensitive paths. Prefer borrowing and slices where feasible.
  - **Cloning:** Avoid unnecessary `.clone()` calls, especially on large data structures or within loops.
  - **Data Structures:** Choose appropriate data structures for the task (e.g., `Vec` vs. `HashMap` vs. `BTreeMap`).
  - **Iteration:** Use iterators effectively; they are often well-optimized.
  - **Optimization:** If significant optimization is needed, it should ideally be guided by profiling or benchmarking (`criterion` if used in the project). Discuss complex performance optimizations.
- **Dependencies:**
  - Minimize new dependencies.
  - If adding dependencies, prefer well-maintained, popular crates.
  - Discuss adding dependencies if unsure.
- **Testing:**
  - Write unit tests (`#[test]`) for core logic.
  - Add integration tests where appropriate.
  - Maintain existing test style and structure.
- **Documentation:**
  - Write clear `rustdoc` comments (`///` for items, `//!` for modules/crates) explaining **why** and **how**, especially for public APIs and complex logic.
  - Keep documentation concise and up-to-date.
- **Naming:** Follow Rust API guidelines and match conventions already used in the project (e.g., `snake_case` for variables/functions, `CamelCase` for types/traits).
- **Modularity:** Respect existing module structure. Place new code in logically appropriate locations.
- **Changelog (`CHANGELOG.md`):**
  - **Requirement:** Maintain the `CHANGELOG.md` file for user-facing changes.
  - **Format:** Follow the "Keep a Changelog" format (using sections like `Added`, `Changed`, `Fixed`, `Removed`, `Deprecated`, `Security`).
  - **Contribution:** When contributing features, fixes, breaking changes, or significant refactors, **suggest or draft a corresponding entry** for the `[Unreleased]` section of `CHANGELOG.md`.
  - **Style:** Match the tone and level of detail of existing changelog entries.

**Interaction Style:**

- Provide complete, runnable code snippets where possible.
- Explain complex logic, non-obvious choices, or performance trade-offs briefly.
- If unsure about requirements, consistency, or performance implications, ask clarifying questions.
- Acknowledge limitations if a request is ambiguous or potentially problematic.

**Reference Point:** When in doubt, **refer to the existing codebase** as the primary source of truth for style, patterns, performance expectations, and changelog format.
