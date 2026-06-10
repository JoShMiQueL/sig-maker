# Sig-Maker

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

### From Source

```bash
cargo build --release
```

Binary: `target/release/sig-maker` (or `sig-maker.exe` on Windows)

### Package Managers (Coming Soon)

```bash
# Homebrew (macOS/Linux)
brew install sig-maker

# Scoop (Windows)
scoop install sig-maker

# Cargo Binstall
cargo binstall sig-maker

# AUR (Arch Linux)
yay -S sig-maker
```

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
src/
├── main.rs        # CLI entry
├── cli.rs         # Argument parsing
├── io.rs          # File I/O, format detection
├── analyzer.rs    # Pattern optimization engine
├── converter.rs   # Format conversion
├── output.rs      # Result formatting
└── formats/       # Format definitions
    ├── mod.rs     # Format enum, BytePattern
    ├── parser.rs  # 8 format parsers
    └── formatter.rs # 8 format formatters
```

## Performance

Typical analysis of 4-10 AOB instances completes in <1ms. Pattern parsing and format conversion are allocation-light for normal-sized patterns (<1KB).

## License

**CC BY-NC-SA 4.0** (Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International)

This means:
- ✅ **Free for personal/educational use**
- ❌ **No commercial use** (can't make money from it)
- ✅ **Must give credit** to JoShMiQueL
- ✅ **Modifications must use same license**

See [LICENSE](LICENSE) file for full text.

Copyright (c) 2024, JoShMiQueL
