# Sig-Maker TODO

## 🔴 High Priority (Implement Now)

### CLI Features
- ✅ **Output to file** (`-o, --output output.txt`)
- ✅ **Version flag** (`-v, --version`)
- ✅ **Quiet mode** (`-q, --quiet`)
- ✅ **Verbose mode** (`--verbose` - shows entropy and compression ratio)
- ✅ **Terminal colors** (crate: `colored` or `owo-colors`)
- ✅ **Pattern validation** (`-c, --check` - detect if valid without converting)
- ✅ **Detailed statistics** (entropy, compression ratio, etc.)
- ✅ **Pause on double-click** (Windows: shows usage and waits for Enter)

### Testing & Quality
- ✅ **More integration tests** (test complete end-to-end flow)
- ✅ **Fuzzing** (configured with cargo-fuzz, requires nightly Rust to run)

---

## 🟡 Medium Priority (Lower Priority)

### Documentation
- ⏳ **Documentation on docs.rs** (improve rustdocs for all public modules)
- ⏳ **Man page** (`sig-maker.1` for Unix systems)
- ⏳ **Shell completions** (bash, zsh, fish, PowerShell)

### UX/CLI
- ⏳ **Progress bar** for large files (crate: `indicatif`)
- ⏳ **Improved visual diff** with ANSI colors side by side
- ⏳ **Interactive mode** (select format with arrow keys - crate: `dialoguer`)

### DevOps
- ⏳ **Cargo binstall** (fast installation without compiling - publish to crates.io first)
- ⏳ **Homebrew formula** (`brew install sig-maker`)
- ⏳ **Scoop manifest** (for Windows: `scoop install sig-maker`)

---

## 🟢 Low Priority (When There's Time)

### Features
- ⏳ **Batch processing** (process multiple files at once)
- ⏳ **Support for inverse masks** (some formats use 0x00=match, 0xFF=ignore)
- ⏳ **Plugin system** (load external parsers/formatters dynamically)
- ⏳ **Config file** (`.sigmakerrc.toml` with user preferences)

### DevOps
- ⏳ **AUR package** (for Arch Linux - `yay -S sig-maker`)
- ⏳ **Docker image** (for CI/CD usage)
- ⏳ **Snap/Flatpak** (Linux universal packages)

---

## ⚪ Future / Nice to Have

### Advanced Features
- ⏳ **GUI/TUI** (Terminal UI with `ratatui` or desktop with `egui`)
- ⏳ **LSP server** (for pattern autocomplete in editors)
- ⏳ **Pattern database** (save/search known patterns)
- ⏳ **Binary scanner** (scan PE/ELF files directly)
- ⏳ **Signature sharing** (upload/download from a central API)

### Integrations
- ⏳ **VS Code extension** (direct editor integration)
- ⏳ **Cheat Engine plugin** (direct communication with CE)
- ⏳ **IDA Pro plugin** (script to import/export)

---

## Repository Quality

### Community
- ✅ **LICENSE** (CC BY-NC-SA 4.0 - Attribution, Non-Commercial, ShareAlike)
- ✅ **README** (complete with examples)
- ✅ **CONTRIBUTING.md** (contributor guidelines)
- ✅ **CODE_OF_CONDUCT.md** (code of conduct)
- ✅ **Issue templates** (bug report, feature request)
- ✅ **Pull request template** (`.github/PULL_REQUEST_TEMPLATE.md`)
- ✅ **AGENTS.md** (project context for AI agents)

### Automation
- ✅ **CI workflow** (test, build, clippy, fmt on Linux/Windows/macOS)
- ✅ **Release workflow** (generates changelog, then multi-platform builds via cargo-dist)
- ✅ **Changelog generation** (automatic - runs before release via git-cliff)
- ✅ **Git hooks** (pre-commit: fmt/clippy/build/test, commit-msg: Conventional Commits)
- ✅ **Branch protection** (PRs required, CI check required)
- ✅ **Dependabot** (automatic dependency updates for GitHub Actions and Cargo)
- ✅ **Stale bot** (marks stale after 30 days, closes after 7)

---

## Marketing / Distribution

- ⏳ **crates.io publish** (for `cargo install`)
- ✅ **GitHub Releases** (pre-compiled binaries)
- ⏳ **Reddit post** (r/ReverseEngineering, r/cheatengine)
- ⏳ **YouTube demo** (short video showing usage)
- ⏳ **Blog post** (explain the optimization algorithm)

---

## Performance & Optimization

- ⏳ **Benchmark suite** (compare against other tools)
- ⏳ **Memory profiling** (valgrind/massif to optimize allocations)
- ⏳ **SIMD optimizations** (for parallel byte processing)
- ⏳ **Parallel processing** (rayon for multiple files)

---

## Legend
- ✅ Completed
- ⏳ Pending / To do
- 🚧 In Progress

---

*Last updated: 2026*
*Author: JoShMiQueL*
