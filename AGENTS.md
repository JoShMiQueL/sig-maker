# Agents Guide

Project context and rules for AI agents working on this codebase.

## Project Overview

**sig-maker** is a Rust CLI tool for converting and optimizing binary signatures/patterns between multiple formats (Cheat Engine, IDA, Ghidra, x64dbg, C++, Rust, Python, JSON).

- **Language:** Rust (edition 2024, MSRV 1.85)
- **Build system:** Cargo workspace with 3 crates
- **Repository:** https://github.com/JoShMiQueL/sig-maker

## Commands

```bash
# Build workspace
cargo build --workspace

# Build specific binaries
cargo build --bin sig-maker-cli
cargo build --bin sig-maker-gui

# Build in release mode
cargo build --workspace --release

# Run tests
cargo test --workspace

# Lint
cargo clippy --workspace -- -D warnings

# Format check
cargo fmt -- --check

# Format fix
cargo fmt

# Run the CLI
cargo run --bin sig-maker-cli -- <input_file> [--to <format>]

# GUI development (Playwright MCP access)
.\scripts\dev.bat      # Windows
./scripts/dev.sh       # Linux/macOS

# GUI production build
.\scripts\build.bat    # Windows
./scripts/build.sh     # Linux/macOS
```

## GUI Development

**IMPORTANT:** GUI development uses the dev server feature. Never use dev mode for production GUI builds - always use release mode.

### GUI Development Workflow

**Development (browser + Playwright MCP):**
```bash
.\scripts\dev.bat      # Windows
./scripts/dev.sh       # Linux/macOS
```

The dev server:
- Serves the compiled frontend at http://localhost:7331
- Exposes `GET /api/formats` and `POST /api/convert` backed by sig-maker-core
- Allows Playwright MCP to automate the GUI from the browser

**Production:**
```bash
.\scripts\build.bat    # Windows
./scripts/build.sh     # Linux/macOS
```

## Verification

**IMPORTANT: NEVER commit or push without explicit user confirmation.**

Before considering any change complete, run **all** of these:

```bash
cargo fmt -- --check
cargo clippy --workspace -- -D warnings
cargo build --workspace
cargo test --workspace
```

These are the same checks enforced by CI.

### Git Hooks

Pre-commit and commit-msg hooks are configured to enforce quality:

**pre-commit hook:**
- Runs `cargo fmt -- --check`
- Runs `cargo clippy --workspace -- -D warnings`
- Runs `cargo build --workspace`
- Runs `cargo test --workspace`

**commit-msg hook:**
- Enforces Conventional Commits format
- Validates against regex: `^(feat|fix|docs|style|refactor|perf|test|ci|chore|build|revert)(\(.+\))?: .{1,}$`

To skip hooks (not recommended): `git commit --no-verify`

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
│   ├── sig-maker-cli/       # CLI binary
│   │   ├── src/
│   │   │   ├── main.rs      # CLI entry point
│   │   │   ├── cli.rs       # CLI argument parsing
│   │   │   ├── converter.rs # Pattern conversion logic
│   │   │   └── output.rs    # Output formatting
│   │   └── tests/           # Integration tests
│   └── sig-maker-gui/       # GUI binary (Tauri + Astro)
│       ├── src/
│       │   ├── main.rs      # Tauri entry point
│       │   ├── commands.rs  # Tauri commands
│       │   └── http_server.rs # HTTP dev server (feature: dev-server)
│       ├── frontend/         # Astro frontend
│       │   ├── src/
│       │   │   ├── pages/
│       │   │   ├── scripts/
│       │   │   └── styles/
│       │   └── package.json
│       └── tauri.conf.json  # Tauri configuration
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
| `ci.yml` | Push/PR to main | Format check, clippy, tests, build on Linux |

### CI Jobs

The CI workflow has two jobs:

**`test`** (fast, no system dependencies):
- Cache Rust dependencies (Swatinem/rust-cache)
- Format check (cargo fmt -- --check)
- Clippy on core (cargo clippy --package sig-maker-core -- -D warnings)
- Tests on core (cargo test --package sig-maker-core --verbose)
- Build core (cargo build --package sig-maker-core)
- Build CLI (cargo build --package sig-maker-cli)

**`test-gui`** (requires system dependencies):
- Install system dependencies (libdbus, webkit, gtk, etc.)
- Cache Rust dependencies (Swatinem/rust-cache)
- Build GUI (cargo build --package sig-maker-gui)

### Testing CI Locally with act

To test GitHub Actions workflows locally using `act`:

```bash
# Install act (Linux/macOS)
curl -s https://raw.githubusercontent.com/nektos/act/master/install.sh | bash

# Install act (Windows with Chocolatey - requires admin)
choco install act-cli

# Or download binary directly (Windows)
# Download from: https://github.com/nektos/act/releases/latest/download/act_windows_amd64.exe

# Test main job (fast, no system dependencies)
act -W .github/workflows/ci.yml -j test

# Test GUI job (slow, requires system dependencies)
act -W .github/workflows/ci.yml -j test-gui

# Test all jobs
act -W .github/workflows/ci.yml

# Dry run (don't execute)
act -n -W .github/workflows/ci.yml
```

**Note:** For faster local testing, test individual commands locally instead of running the full CI with act. The GUI job requires heavy system dependencies and is slow to run with act.

### Releases

Releases are **fully automated** via GitHub Actions. To release:

1. Bump version in `Cargo.toml` (workspace.package.version)
2. Commit: `chore(release): v0.x.x`
3. Tag: `git tag v0.x.x` (optional: sign with GPG: `git tag -s v0.x.x`)
4. Push: `git push && git push --tags`

The `.github/workflows/release.yml` workflow will automatically:
- Validate tag format and version match
- Build CLI for 5 platforms (Linux x64/ARM64, macOS Intel/ARM, Windows x64)
- Build GUI for 3 platforms (Linux AppImage, macOS DMG, Windows NSIS + Portable ZIP)
- Generate changelog via git-cliff
- Create GitHub Release with all artifacts + install scripts
- Generate SHA-256 checksums

**Note:** fmt, clippy, and tests are NOT re-run in release workflow. These are validated by the existing CI workflow (`.github/workflows/ci.yml`) on every push/PR. Only tag validation is performed in release workflow.

**Artifacts Generated:**
- CLI: 5 binaries (Linux x64/ARM64, macOS Intel/ARM, Windows x64)
- GUI: Linux AppImage, macOS DMG, Windows NSIS installer + Portable ZIP
- Install scripts: `install.sh` (curl | sh), `install.ps1` (irm | iex)
- Total: ~9 artifacts per release

**Dependency pinning:** `Cargo.lock` is committed for reproducible builds.
`tauri-utils` is pinned to `2.8.3` and `time` to `0.3.46` in `Cargo.lock` to avoid
an E0119 coherence bug present in `tauri-utils 2.9.x` + `time 0.3.47+`.
Track: https://github.com/tauri-apps/tauri/issues/15525

**Release Duration:** ~12-15 minutes (with cache)

**Local Testing with act:**
```bash
# Test release workflow locally
act -W .github/workflows/release.yml -j pre-checks
```

**Changelog Format:**
- Uses git-cliff with Conventional Commits
- Follows Keep a Changelog standard
- Auto-generates from commit history
- Commits must follow: `feat:`, `fix:`, `docs:`, etc.

**Breaking Changes:**
- Use `feat!:` prefix or `BREAKING CHANGE:` footer
- Automatically highlighted in changelog

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

## Devin AI Agent Configuration

This project is configured for optimal use with Devin CLI.

### Configuration Structure

Devin CLI configuration lives in `.devin/` directory:

```
.devin/
├── config.json              # Project config (permissions, MCPs, imports) - committed
├── config.local.json        # Personal overrides (MCP tokens) - gitignored
├── hooks.json              # Safety hooks configuration - committed
└── scripts/
    └── safety-check.sh     # Safety hook script - committed
```

**Setup steps:**
1. Copy `.devin/config.local.json.example` to `.devin/config.local.json`
2. Add your API keys for context7 and deepwiki
3. Run `devin mcp login context7` and `devin mcp login deepwiki` if OAuth is required

### Safety Hooks

PreToolUse hooks are configured to block truly dangerous commands that cannot be recovered with git:

**Blocked git commands (affect history irreversibly):**
- `git reset --hard` → Use `git stash` or `git reset --soft`
- `git clean -fd` / `git clean -f` → Use `git clean -n` (dry run)
- `git branch -D` → Use `git branch -d` (only merged branches)
- `git push --force` / `git push -f` → Use `git push --force-with-lease`
- `git rebase` → Consider using merge instead
- `git checkout --` / `git restore --worktree` → Use `git stash` first
- `git stash drop` / `git stash clear` → Warns about permanent loss
- `git reflog expire` → Destroys recovery data
- `git commit --no-verify` / `git commit -n` → Remove flag, fix hooks

**Blocked file deletions (outside project or destroys recovery):**
- `rm -rf /` (system root)
- `rm -rf ~` (home directory)
- `rm -rf ..` (parent directory)
- `rm -rf .git/` (destroys git history)

**Allowed file deletions (within project):**
- Any `rm` command within the project directory is allowed
- Files can be recovered with git if needed
- This enables automated refactoring and cleanup

The hook runs before every `exec` tool call and exits with code 2 to block the action. It provides safer alternatives in the error message.

### Available Skills

No project-specific skills are currently configured. Use standard cargo commands directly for development.

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

### Permissions

Pre-approved permissions in `.devin/config.json`:
- **Allow:** Read all files, git commands, cargo commands, shell scripts
- **Deny:** sudo, destructive rm -rf, writing to .git/
- **Ask:** git commit, git push, git reset --hard, git clean -fd, git branch -D, git push --force, git rebase, gh repo delete, gh release delete (requires explicit confirmation)

### Devin CLI Workflow

When using Devin CLI locally:

1. **Start a session** in the project directory
2. **Use MCPs** for documentation:
   - "Use context7 to check Rust 1.85 documentation for..."
   - "Use deepwiki to check the repo's documentation for..."
3. **Always verify** before committing/pushing (NEVER do this without confirmation)

### Recommended Devin Workflow

1. **Exploration phase:**
   - Use deepwiki MCP for repo documentation
   - Use context7 MCP for Rust documentation
   - Read relevant source files to understand the codebase

2. **Implementation phase:**
   - Make code changes
   - Run verification: `cargo fmt -- --check`, `cargo clippy --workspace -- -D warnings`, `cargo build --workspace`, `cargo test --workspace`
   - Fix any issues found

3. **Testing phase:**
   - Run `cargo test --workspace` to verify all tests pass

4. **Commit phase:**
   - Ask for user confirmation before committing
   - Use conventional commit format
   - Verify with pre-commit hook
