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

**IMPORTANT: NEVER commit or push without explicit user confirmation.**

Before considering any change complete, run **all** of these:

```bash
# Option 1: Use the pre-commit hook (recommended)
# The hook runs automatically on commit, but you can also run it manually:
sh .githooks/pre-commit

# Option 2: Run checks manually
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

## Devin AI Agent Configuration

This project is configured for optimal use with Devin CLI.

### Configuration Structure

Devin CLI configuration lives in `.devin/` directory:

```
.devin/
├── config.json              # Project config (permissions, MCPs, imports) - committed
├── config.local.json        # Personal overrides (MCP tokens) - gitignored
├── hooks.v1.json            # Lifecycle hooks (optional) - committed
├── scripts/                 # Hook support scripts - committed
│   └── check-git-commit.sh
└── skills/                  # Project-specific skills - committed
    ├── verify-before-commit/
    │   └── SKILL.md
    ├── debug-pattern/
    │   └── SKILL.md
    ├── investigate-code/
    │   └── SKILL.md
    └── test-integration/
        └── SKILL.md
```

**Setup steps:**
1. Copy `.devin/config.local.json.example` to `.devin/config.local.json`
2. Add your API keys for context7 and deepwiki
3. Run `devin mcp login context7` and `devin mcp login deepwiki` if OAuth is required

### Available Skills

Skills are reusable procedures located in `.devin/skills/<skill-name>/SKILL.md`:

- **`verify-before-commit`** - Run full verification (fmt, clippy, build, test) before committing
- **`debug-pattern`** - Debug pattern parsing issues with detailed analysis
- **`investigate-code`** - Read-only code exploration and analysis
- **`test-integration`** - Run integration tests with detailed output

Invoke skills by mentioning them: `@skills:verify-before-commit` or `@skills:debug-pattern <pattern>`

### Available MCPs

Model Context Protocol servers configured in `.devin/config.json`:

- **context7** - Fetch up-to-date Rust/Cargo documentation
- **deepwiki** - Access GitHub repository documentation and wiki

**Note:** These MCPs use pnpm for installation. If authentication is required, create `.devin/config.local.json` with your tokens:

```json
{
  "mcpServers": {
    "context7": {
      "env": {
        "CONTEXT7_API_KEY": "your-api-key"
      }
    }
  }
}
```

Use MCPs by asking Devin to query documentation or access external services.

### Devin Hooks

Lifecycle hooks configured in `.devin/hooks.v1.json`:

- **`SessionStart`** - Runs `.githooks/setup.sh` to ensure git hooks are configured on session start
- **`PreToolUse`** - Warns before `git commit` commands to remind user to run verification

**Purpose:** Hooks provide automated guidance and reminders during development without being intrusive. They respect the "NEVER commit automatically" rule by only providing warnings, not blocking actions.

**Configuration:** Hooks are defined in `.devin/hooks.v1.json` and use the Devin CLI hook format (compatible with Claude Code hooks).

### Permissions

Pre-approved permissions in `.devin/config.json`:
- **Allow:** Read all files, git commands, cargo commands, shell scripts
- **Deny:** sudo, destructive rm -rf, writing to .git/
- **Ask:** git commit, git push, git reset --hard, git clean -fd, git branch -D, git push --force, git rebase, gh repo delete, gh release delete (requires explicit confirmation)

### Devin CLI Workflow

When using Devin CLI locally:

1. **Start a session** in the project directory
2. **Invoke skills** for common tasks:
   - `@skills:verify-before-commit` before committing
   - `@skills:debug-pattern` when patterns fail to parse
   - `@skills:investigate-code <topic>` for code exploration
3. **Use MCPs** for documentation:
   - "Use context7 to check Rust 1.85 documentation for..."
   - "Use deepwiki to check the repo's documentation for..."
4. **Always verify** before committing/pushing (NEVER do this without confirmation)

### Recommended Devin Workflow

1. **Exploration phase:**
   - Use `@skills:investigate-code` to understand the codebase
   - Use deepwiki MCP for repo documentation
   - Use context7 MCP for Rust documentation

2. **Implementation phase:**
   - Make code changes
   - Use `@skills:verify-before-commit` to validate
   - Fix any issues found

3. **Testing phase:**
   - Use `@skills:test-integration` to run tests
   - Debug issues with `@skills:debug-pattern` if needed

4. **Commit phase:**
   - Ask for user confirmation before committing
   - Use conventional commit format
   - Verify with pre-commit hook
