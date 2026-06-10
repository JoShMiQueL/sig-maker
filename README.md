# Sig-Maker

[![CI](https://github.com/JoShMiQueL/sig-maker/workflows/CI/badge.svg)](https://github.com/JoShMiQueL/sig-maker/actions/workflows/ci.yml)
[![License: CC BY-NC-SA 4.0](https://img.shields.io/badge/License-CC%20BY--NC--SA%204.0-lightgrey.svg)](https://creativecommons.org/licenses/by-nc-sa/4.0/)

Multi-format signature/pattern converter and optimizer. Optimizes wildcard patterns from multiple memory snapshots and converts between Cheat Engine, IDA Pro, Ghidra, x64dbg, and other formats.

## Why

When reverse engineering games or applications, you often capture the same data structure at different memory addresses or game states. The bytes that change are noise - the ones that stay constant are your pattern. This tool finds the optimal wildcard representation automatically.

## Features

- **Pattern optimization**: Compares multiple AOB instances, generates minimal wildcard pattern
- **Nibble-level wildcards**: Detects when only high or low nibble varies (e.g., `0?` vs `??`)
- **8 format support**: Cheat Engine, C++, Rust, Ghidra, IDA Pro, x64dbg, Python, JSON
- **Bidirectional conversion**: Any format to any format
- **Auto-detection**: Input format detected automatically

## Installation

### Pre-built Binaries (Recommended)

Download pre-built binaries for Linux, macOS, and Windows from the [GitHub Releases](https://github.com/JoShMiQueL/sig-maker/releases) page. Installers are available for shell and PowerShell.

### From Source

```bash
cargo build --release
```

Binary: `target/release/sig-maker` (or `sig-maker.exe` on Windows)

### Package Managers (Planned)

The following package managers are planned but not yet available:

- [ ] Homebrew (macOS/Linux)
- [ ] Scoop (Windows)
- [ ] Cargo Binstall
- [ ] AUR (Arch Linux)

See [TODO.md](TODO.md) for progress.

## Usage

### Analyze multiple AOB instances

Create `aobs.txt`:
```
# Comment line
07 00 00 00 01 00 00 00 ED FF FF FF 00
08 00 00 00 01 00 00 00 EC FF FF FF 00
09 00 00 00 01 00 00 00 EB FF FF FF 00
```

Run:
```bash
./sig-maker aobs.txt
```

Output shows all formats + byte-by-byte diff.

> **Note for Windows users:** If you double-click the `.exe` without arguments, it will show the usage message and wait for you to press Enter before closing. This is intentional to prevent the console from disappearing immediately.

### Convert single pattern

```bash
# Auto-detect input, show all output formats
echo "0? 00 00 00 01 00 00 00 E? FF FF FF 00" | ./sig-maker

# Specific output format
./sig-maker pattern.txt --to rust
```

### Format detection

The tool auto-detects input format by content:

| Format | Detection pattern |
|--------|-------------------|
| Cheat Engine | Contains `?` |
| x64dbg | Contains `.` |
| IDA/Ghidra | Contains `[` `]` ranges |
| C++ | `const uint8_t` or `{ 0x... }` |
| Rust | `[u8;` or `static PATTERN` |
| JSON | Starts with `{` `"pattern"` |

## Supported Formats

| Format | Example |
|--------|---------|
| Cheat Engine | `0? 00*3 01 00*3 E? FF*3 00` |
| C++ | `const uint8_t p[] = { 0x00... };` |
| Rust | `static PATTERN: [u8; 13] = [...];` |
| Ghidra | `[00-0F] 00 00 00 [E0-EF] FF` |
| IDA Pro | `[00-0F] 00 00 00 [E0-EF] FF` |
| x64dbg | `0. 00 00 00 01 00 00 00 E.` |
| Python | `re.compile(b'[\x00-\x0F]...')` |
| JSON | `{"pattern":[0,...],"mask":[255,...]}` |

## Wildcard Types

The optimizer detects three wildcard levels:

- **Fixed**: `5F` - byte must match exactly
- **High nibble**: `4?` - high nibble fixed, low varies (40-4F)
- **Low nibble**: `?F` - low nibble fixed, high varies (0F, 1F, ..., FF)
- **Full**: `??` - any byte

Example: Values `07`, `08`, `09` → optimized to `0?`

## Project Structure

```
sig-maker/                    # Cargo workspace
├── Cargo.toml               # Workspace configuration
├── crates/
│   ├── sig-maker-lib/       # Core library
│   │   ├── src/
│   │   │   ├── lib.rs       # Library interface
│   │   │   ├── analyzer.rs  # Pattern optimization engine
│   │   │   ├── converter.rs # Format conversion
│   │   │   ├── cli.rs       # CLI logic
│   │   │   ├── io.rs        # File I/O, format detection
│   │   │   ├── output.rs    # Result formatting
│   │   │   └── formats/     # Format definitions
│   │   │       ├── mod.rs
│   │   │       ├── parser.rs
│   │   │       └── formatter.rs
│   │   ├── benches/         # Benchmarks
│   │   └── tests/           # Unit tests
│   └── sig-maker-cli/       # CLI binary
│       ├── src/
│       │   └── main.rs      # CLI entry point
│       └── tests/           # Integration tests
├── benches/                 # Workspace-level benchmarks
└── .github/workflows/       # CI/CD
```

## Architecture

Sig-Maker is organized as a **Cargo workspace** with two crates:

- **sig-maker-lib**: Core library containing all pattern analysis and conversion logic
- **sig-maker-cli**: CLI binary that uses sig-maker-lib

This modular architecture enables:
- Reuse of the core library in other tools (GUI, MCP servers, etc.)
- Independent versioning and testing
- Clean separation between CLI and core logic

## Performance

Typical analysis of 4-10 AOB instances completes in <1ms. Pattern parsing and format conversion are allocation-light for normal-sized patterns (<1KB).

## Documentation

- [CHANGELOG.md](CHANGELOG.md) — Version history and release notes
- [AGENTS.md](AGENTS.md) — Project context for AI agents and contributors
- [CONTRIBUTING.md](CONTRIBUTING.md) — How to contribute
- [TODO.md](TODO.md) — Planned features and improvements

## License

**CC BY-NC-SA 4.0** (Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International)

This means:
- ✅ **Free for personal/educational use**
- ❌ **No commercial use** (can't make money from it)
- ✅ **Must give credit** to JoShMiQueL
- ✅ **Modifications must use same license**

See [LICENSE](LICENSE) file for full text.

Copyright (c) 2026, JoShMiQueL
