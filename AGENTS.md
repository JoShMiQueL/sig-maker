# Agents Guide

Project context and rules for AI agents working on this codebase.

## Project Overview

**sig-maker** is a Rust CLI tool for converting and optimizing binary signatures/patterns between multiple formats (Cheat Engine, IDA, Ghidra, x64dbg, C++, Rust, Python, JSON).

- **Language:** Rust (edition 2024, MSRV 1.85)
- **Build system:** Cargo workspace with 2 crates
- **Repository:** https://github.com/JoShMiQueL/sig-maker

## Commands

```bash
# Build workspace
cargo build --workspace

# Run tests
cargo test --workspace

# Lint
cargo clippy --workspace -- -D warnings

# Format check
cargo fmt -- --check

# Format fix
cargo fmt

# Run the CLI
cargo run --bin sig-maker -- <input_file> [--to <format>]
```

## Verification

Before considering any change complete, run **all** of these:

```bash
# Option 1: Use the pre-commit hook (recommended)
# The hook runs automatically on commit, but you can also run it manually:
sh .githooks/pre-commit

# Option 2: Use the CI validation script (simulates GitHub CI)
# Windows:
powershell -ExecutionPolicy Bypass -File ci-check.ps1
# Unix/Linux/Mac:
sh ci-check.sh

# Option 3: Run checks manually
cargo fmt -- --check
cargo clippy --workspace -- -D warnings
cargo build --workspace
cargo test --workspace
```

These are the same checks enforced by the pre-commit hook and CI.

## Project Structure

```
sig-maker/                    # Cargo workspace
├── Cargo.toml               # Workspace configuration
├── crates/
│   ├── sig-maker-core/      # Core library (zero external dependencies)
│   │   ├── src/
│   │   │   ├── lib.rs       # Library interface
│   │   │   ├── analyzer.rs  # Pattern optimization engine
│   │   │   ├── converter.rs # Format conversion
│   │   │   ├── io.rs        # File I/O, format detection
│   │   │   └── formats/     # Format definitions
│   │   │       ├── mod.rs
│   │   │       ├── parser.rs
│   │   │       └── formatter.rs
│   │   └── tests/           # Unit tests
│   └── sig-maker-cli/       # CLI binary
│       ├── src/
│       │   ├── main.rs      # CLI entry point
│       │   ├── cli.rs       # CLI argument parsing
│       │   ├── converter.rs # Pattern conversion logic
│       │   └── output.rs    # Output formatting
│       └── tests/           # Integration tests
└── .github/workflows/       # CI/CD
```

## Commit Conventions

All commits **must** follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<optional scope>): <description>
```

**Types:** `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `ci`, `chore`, `build`, `revert`

**Examples:**
```
feat(cli): add JSON output format
fix(analyzer): handle empty pattern list
docs: update README examples
ci: upgrade actions to v6
chore!: bump MSRV to 1.85
```

A `commit-msg` git hook validates this format. Non-conforming messages will be rejected.

## Git Hooks

Located in `.githooks/`. Activate with:

```bash
sh .githooks/setup.sh
```

**pre-commit:** runs `cargo fmt --check`, `cargo clippy`, `cargo build`, `cargo test`
**commit-msg:** validates Conventional Commits format

## Branch Protection

- `main` is protected: PRs required with CI passing (`build` check)
- Admin/maintainers can bypass for trivial changes (typos, config)
- External contributors must always use PRs

## Branching & PRs

- **Features/fixes:** Create a branch (`feat/`, `fix/`, `refactor/`, `ci/`, `docs/`) → open PR → squash merge
- **Trivial changes (maintainers only):** Direct push to main is acceptable
- PRs are squash-merged to keep a clean linear history

## CI/CD

### Workflows (`.github/workflows/`)

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| `ci.yml` | Push/PR to main | Tests on Linux, Windows, macOS + clippy + fmt |
| `changelog.yml` | Push to main | Auto-generates `CHANGELOG.md` via git-cliff |
| `release.yml` | Tag `v*` | Builds multi-platform binaries via cargo-dist |

### Releases (cargo-dist)

Releases are automated with `cargo-dist`. To release:

1. Bump version in `Cargo.toml`
2. Commit: `chore(release): v0.x.x`
3. Tag: `git tag v0.x.x`
4. Push: `git push && git push --tags`

cargo-dist builds binaries for: Linux x64/ARM64, macOS x64/ARM64, Windows x64, plus shell and PowerShell installers.

### Changelog (git-cliff)

`CHANGELOG.md` is auto-generated on every push to main. Configuration in `cliff.toml`.

- Groups commits by type (Features, Bug Fixes, CI/CD, etc.)
- Links to commits, PRs, and author profiles
- Shows New Contributors and all Contributors per release
- Filters out bot commits and changelog update commits
- **Do NOT edit `CHANGELOG.md` manually** — it will be overwritten

## Code Style

- Rust edition 2024 idioms
- No external runtime dependencies (zero-dependency binary)
- Compact code: avoid unnecessary nesting, share abstractions
- Use `unsafe extern` blocks (required by edition 2024)
- Do not add/remove comments unless explicitly needed
- Follow existing patterns in neighboring code

## Important Notes

- The CLI automatically detects pipe output (TTY detection via `std::io::IsTerminal`)
- When outputting to a pipe or when `--to <format>` is specified, output is simplified for clean piping
- On Windows, the CLI pauses with "Press Enter to exit..." when launched via double-click (detected via `GetConsoleProcessList` Windows API)
- On Unix, the CLI pauses when stdout is not a TTY
- `dist-workspace.toml` and the release workflow are managed by cargo-dist — do not edit manually. Use `dist init` to reconfigure.
