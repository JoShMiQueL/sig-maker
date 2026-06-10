# Contributing to Sig-Maker

Thank you for your interest in contributing to Sig-Maker! This document provides guidelines for contributing to the project.

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the [existing issues](https://github.com/JoShMiQueL/sig-maker/issues) to avoid duplicates. When creating a bug report, please include as many details as possible using the [bug report template](.github/ISSUE_TEMPLATE/bug_report.md).

### Suggesting Enhancements

Enhancement suggestions are tracked as [GitHub issues](https://github.com/JoShMiQueL/sig-maker/issues). Please use the [feature request template](.github/ISSUE_TEMPLATE/feature_request.md) and provide:

- Clear description of the feature
- Why it would be useful
- Possible implementation approach (if you have ideas)

### Pull Requests

All contributions from external contributors **must** go through a Pull Request. The `main` branch is protected and requires the CI `build` check to pass.

**Workflow:**

1. Fork the repository
2. Create a new branch: `git checkout -b feature/my-feature` or `fix/my-bugfix`
3. Make your changes
4. Run tests: `cargo test`
5. Run formatting: `cargo fmt`
6. Run clippy: `cargo clippy -- -D warnings`
7. Commit your changes using [Conventional Commits](#commit-message-format)
8. Push to the branch: `git push origin feature/my-feature`
9. Open a Pull Request against `main`

**Branch naming convention:**
- `feat/description` — New features
- `fix/description` — Bug fixes
- `refactor/description` — Refactoring
- `ci/description` — CI/CD changes
- `docs/description` — Documentation

**Merge strategy:** PRs are squash-merged into `main` to keep a clean linear history.

> **Note for maintainers:** Admins can push directly to `main` for trivial changes (typos, version bumps, config tweaks). Use PRs for anything non-trivial.

## Development Setup

### Prerequisites

- Rust 1.85 or later
- Git

### Git Hooks

This project uses pre-commit hooks to ensure code quality. After cloning, set them up:

```bash
sh .githooks/setup.sh
```

This will run `cargo fmt --check`, `cargo clippy`, `cargo build`, and `cargo test` before every commit. It also validates that commit messages follow [Conventional Commits](https://www.conventionalcommits.org/) format.

### Commit Message Format

All commits must follow the Conventional Commits format:

```
<type>(<optional scope>): <description>
```

**Types:** `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `ci`, `chore`, `build`, `revert`

**Examples:**
```
feat: add JSON output format
fix(cli): handle missing input file
docs: update README examples
ci: migrate to cargo-dist
chore!: bump MSRV to 1.85
```

### Building

```bash
# Clone the repository
git clone https://github.com/JoShMiQueL/sig-maker.git
cd sig-maker

# Build in debug mode
cargo build

# Build in release mode (optimized)
cargo build --release

# Run tests
cargo test

# Run with a pattern file
cargo run -- your_pattern.txt
```

### Project Structure

```
src/
├── main.rs        # CLI entry point
├── lib.rs         # Library exports
├── cli.rs         # Argument parsing
├── io.rs          # File I/O, format detection
├── analyzer.rs    # Pattern optimization engine
├── converter.rs   # Format conversion
├── output.rs      # Result formatting
└── formats/       # Format definitions
    ├── mod.rs     # Format enum, BytePattern
    ├── parser.rs  # Format parsers
    └── formatter.rs # Format formatters
```

## Style Guidelines

### Rust Code Style

- Follow the [Rust Style Guide](https://doc.rust-lang.org/style-guide/)
- Use `cargo fmt` to format code
- Use `cargo clippy` to catch common mistakes
- Write documentation comments for public APIs (`///`)
- Add examples in doc comments where helpful

### Commit Messages

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests liberally after the first line

Example:
```
Add support for x64dbg format

- Parse x64dbg patterns with . wildcards
- Format to x64dbg output format
- Add tests for roundtrip conversion

Fixes #123
```

### Testing

- Write tests for new functionality
- Ensure all tests pass before submitting PR
- Add integration tests for new formats
- Consider edge cases (empty input, invalid patterns, etc.)

## Recognition

Contributors will be recognized in our [CHANGELOG.md](CHANGELOG.md) and release notes.

## Questions?

Feel free to [open a discussion](https://github.com/JoShMiQueL/sig-maker/discussions) if you have questions not covered here.

---

Thank you for contributing! 🎉
