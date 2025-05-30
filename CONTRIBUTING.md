# Contributing to openfigi-rs

Thank you for your interest in contributing to openfigi-rs! ❤️ Your support and collaboration are greatly appreciated.

## How to Contribute

We welcome contributions in various forms, including bug fixes, new features, documentation improvements, and tests. To ensure a streamlined process, please adhere to the following guidelines:

- Utilize the provided issue templates when submitting new issues.
- Before opening a pull request (PR), consider creating an issue to discuss proposed features or bug fixes.
- Maintain clear and concise documentation and comprehensive test coverage.
- Follow Rust best practices and maintain consistency with the project's code style.
- Commit messages must adhere to the [Conventional Commits](https://www.conventionalcommits.org/) format to facilitate automated changelog generation and release management.
  - Example: `feat: add support for new LEI endpoint`

## Development Guidelines

To contribute effectively, follow these steps:

1. Clone the repository and create a dedicated branch for your changes.
2. Utilize the provided developer container via Docker for a standardized development environment (see `.devcontainer/` for setup instructions).
3. Prior to submitting your PR, run `./scripts/test-ci.sh` to execute the same tests as the continuous integration (CI) pipeline, ensuring efficiency in resource usage.
4. If introducing new features, ensure that relevant documentation and tests accompany your changes.

## Code of Conduct

We are committed to fostering a respectful, inclusive, and welcoming community. By participating in this project, you agree to abide by our [Code of Conduct](./CODE_OF_CONDUCT.md).

---

Thank you for your contributions—your efforts help make openfigi-rs even better! 🚀
