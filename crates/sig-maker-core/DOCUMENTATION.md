# sig-maker-core Documentation

## Overview

**sig-maker-core** is a zero-dependency Rust library for analyzing, optimizing, and converting binary signature/pattern (AOB - Array of Bytes) patterns between multiple formats. It provides the core functionality used by the sig-maker CLI tool.

### Key Features

- **Zero external dependencies** - Pure Rust implementation with no runtime dependencies
- **Multi-format support** - Convert between 8 different pattern formats
- **Pattern optimization** - Analyze multiple AOB instances to generate optimal patterns
- **Nibble-level wildcards** - Support for high/low nibble wildcards for more precise patterns
- **Automatic format detection** - Intelligently detect input format type
- **Pattern statistics** - Calculate entropy and compression ratio for patterns

### Supported Formats

| Format | Description | Wildcard Syntax |
|--------|-------------|-----------------|
| Cheat Engine | Game memory scanner format | `??`, `?X`, `X?` |
| C++ | C/C++ array with mask | `0x00` with mask array |
| Rust | Rust static array with mask | `0x00` with mask array |
| Ghidra | Reverse engineering tool | `.`, `[XX-YY]` |
| IDA Pro | Disassembler format | `[XX-YY]` |
| x64dbg | Windows debugger | `..`, `.X`, `X.` |
| Python | Regex byte pattern | `\xXX`, `[...]` |
| JSON | Structured data | Pattern + mask arrays |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
sig-maker-core = "0.1.0"
```

## Public API

### Core Types

#### `BytePattern`

Represents a single byte in a pattern with varying specificity.

```rust
pub enum BytePattern {
    Fixed(u8),       // Exact byte match (e.g., 0x5F)
    Wildcard,        // Match any byte (??)
    HighNibble(u8),  // Match high nibble only (e.g., 0x5? matches 0x50-0x5F)
    LowNibble(u8),   // Match low nibble only (e.g., ?0F matches 0x0F, 0x1F, ..., 0xFF)
}
```

**Example:**
```rust
use sig_maker_core::BytePattern;

let fixed = BytePattern::Fixed(0x5F);
let wildcard = BytePattern::Wildcard;
let high_nibble = BytePattern::HighNibble(0x05);  // Matches 0x50-0x5F
let low_nibble = BytePattern::LowNibble(0x0F);   // Matches 0x0F, 0x1F, ..., 0xFF
```

#### `Format`

Enum representing supported output formats.

```rust
pub enum Format {
    CheatEngine,
    Cpp,
    Rust,
    Ghidra,
    IdaPro,
    X64dbg,
    Python,
    Json,
}
```

**Methods:**
- `Format::from_string(s: &str) -> Option<Self>` - Parse format name from string
- `Format::name(&self) -> &'static str` - Get human-readable format name
- `Format::all() -> &'static [Format]` - Get all supported formats

**Example:**
```rust
use sig_maker_core::Format;

let format = Format::from_string("ce").unwrap();
assert_eq!(format, Format::CheatEngine);
assert_eq!(format.name(), "Cheat Engine");

for format in Format::all() {
    println!("{}", format.name());
}
```

#### `PatternStats`

Statistics about pattern optimization quality.

```rust
pub struct PatternStats {
    // Private fields, accessed via methods
}
```

**Methods:**
- `PatternStats::from_patterns(patterns: &[BytePattern]) -> Self` - Calculate stats from pattern
- `total_bytes(&self) -> usize` - Total bytes in pattern
- `fixed_bytes(&self) -> usize` - Count of fixed bytes
- `high_nibble_wildcards(&self) -> usize` - Count of high nibble wildcards
- `low_nibble_wildcards(&self) -> usize` - Count of low nibble wildcards
- `full_wildcards(&self) -> usize` - Count of full wildcards
- `entropy(&self) -> f64` - Pattern entropy (measure of randomness)
- `compression_ratio(&self) -> f64` - Fixed bytes / total bytes

**Example:**
```rust
use sig_maker_core::{BytePattern, PatternStats};

let pattern = vec![
    BytePattern::Fixed(0x5F),
    BytePattern::HighNibble(0x05),
    BytePattern::Wildcard,
];

let stats = PatternStats::from_patterns(&pattern);
println!("Total bytes: {}", stats.total_bytes());
println!("Fixed bytes: {}", stats.fixed_bytes());
println!("Entropy: {:.2}", stats.entropy());
println!("Compression ratio: {:.2}", stats.compression_ratio());
```

### Pattern Parsing

#### `parse_pattern`

Parse a pattern string from any supported format into a `BytePattern` array.

```rust
pub fn parse_pattern(input: &str) -> Option<Vec<BytePattern>>
```

**Supported Input Formats:**
- Cheat Engine: `"0? 00 ?? 01 ?F F?"`
- C++: `"const uint8_t pattern[] = { 0x00, 0x01 }; const uint8_t mask[] = { 0xF0, 0xFF };"`
- Rust: `"static PATTERN: [u8; 2] = [0x00, 0x01]; static MASK: [u8; 2] = [0xF0, 0xFF];"`
- JSON: `"{ \"pattern\": [0, 1], \"mask\": [240, 255] }"`
- x64dbg: `"0. 00 .. 01 .F F."`
- IDA Pro: `"00 01 [00-0F]"`
- Raw hex: `"00 01 02 03"`

**Example:**
```rust
use sig_maker_core::parse_pattern;

// Parse Cheat Engine format
let pattern = parse_pattern("0? 00 ?? 01 ?F").unwrap();
assert_eq!(pattern.len(), 5);

// Parse C++ format
let cpp_pattern = parse_pattern(
    "const uint8_t pattern[] = { 0x00, 0x01 }; const uint8_t mask[] = { 0xF0, 0xFF };"
).unwrap();
```

#### `parse_single_pattern`

Wrapper around `parse_pattern` for converting a single pattern.

```rust
pub fn parse_single_pattern(content: &str) -> Option<Vec<BytePattern>>
```

**Example:**
```rust
use sig_maker_core::parse_single_pattern;

let pattern = parse_single_pattern("0? 00 ?? 01").unwrap();
```

### Pattern Formatting

#### `format_pattern`

Format a `BytePattern` array to a specific output format.

```rust
pub fn format_pattern(pattern: &[BytePattern], format: Format) -> String
```

**Example:**
```rust
use sig_maker_core::{BytePattern, format_pattern, Format};

let pattern = vec![
    BytePattern::HighNibble(0),
    BytePattern::Fixed(0x00),
    BytePattern::Wildcard,
];

// Convert to Cheat Engine
let ce = format_pattern(&pattern, Format::CheatEngine);
println!("{}", ce);  // Output: "0? 00 ??"

// Convert to C++
let cpp = format_pattern(&pattern, Format::Cpp);
println!("{}", cpp);
// Output:
// // C++ Pattern + Mask
// const uint8_t pattern[] = { 0x00, 0x00, 0x00 };
// const uint8_t mask[]    = { 0xF0, 0xFF, 0x00 };
```

### Pattern Conversion

#### `convert_to_format`

Convert a pattern to a specific format (wrapper around `format_pattern`).

```rust
pub fn convert_to_format(pattern: &[BytePattern], format: Format) -> String
```

**Example:**
```rust
use sig_maker_core::{convert_to_format, parse_pattern, Format};

let pattern = parse_pattern("0? 00 ??").unwrap();
let cpp_output = convert_to_format(&pattern, Format::Cpp);
println!("{}", cpp_output);
```

### Pattern Optimization

#### `optimize_byte`

Find the best `BytePattern` for a set of byte values.

```rust
pub fn optimize_byte(values: &[u8]) -> BytePattern
```

**Optimization Logic:**
1. If all values are identical → `Fixed(value)`
2. If all values share the same high nibble → `HighNibble(high)`
3. If all values share the same low nibble → `LowNibble(low)`
4. Otherwise → `Wildcard`

**Example:**
```rust
use sig_maker_core::{BytePattern, optimize_byte};

// All same value
let pattern1 = optimize_byte(&[0x5F, 0x5F, 0x5F]);
assert_eq!(pattern1, BytePattern::Fixed(0x5F));

// Same high nibble
let pattern2 = optimize_byte(&[0x40, 0x45, 0x4F]);
assert_eq!(pattern2, BytePattern::HighNibble(4));

// Same low nibble
let pattern3 = optimize_byte(&[0x0F, 0x3F, 0xBF]);
assert_eq!(pattern3, BytePattern::LowNibble(0x0F));

// No commonality
let pattern4 = optimize_byte(&[0x00, 0xFF, 0x42]);
assert_eq!(pattern4, BytePattern::Wildcard);
```

#### `analyze_aobs`

Analyze multiple AOB instances and generate an optimized pattern.

```rust
pub fn analyze_aobs(content: &str) -> (Vec<BytePattern>, PatternStats)
```

**Input Format:**
- Multiple lines, each containing an AOB instance
- Supports raw hex bytes or patterns with wildcards
- Lines starting with `#` or `//` are comments
- Lines can have `- ` or `* ` prefixes (for list formatting)

**Behavior:**
1. Parses all AOB instances from the input
2. Verifies all instances have the same length
3. Analyzes each byte position across all instances
4. For positions with original wildcards, only analyzes non-expanded instances
5. Returns optimized pattern and statistics

**Example:**
```rust
use sig_maker_core::analyze_aobs;

let input = r#"
48 8B 05 ? ? ? ?
48 8B 05 12 34 56 78
48 8B 05 AB CD EF 00
"#;

let (pattern, stats) = analyze_aobs(input);
println!("Pattern length: {}", pattern.len());
println!("Fixed bytes: {}", stats.fixed_bytes());
println!("Compression ratio: {:.2}", stats.compression_ratio());
```

#### `parse_aobs`

Parse AOB instances from file content.

```rust
pub fn parse_aobs(content: &str) -> Vec<AobInstance>
```

**Returns:**
- Vector of `AobInstance` structs containing:
  - `bytes: Vec<Option<u8>>` - The byte values (None for wildcards)
  - `line_num: usize` - Source line number
  - `is_expanded: bool` - Whether this came from expanding a pattern
  - `wildcard_positions: Vec<bool>` - Tracks original wildcard positions

**Example:**
```rust
use sig_maker_core::parse_aobs;

let input = r#"
48 8B 05 ? ? ? ?
48 8B 05 12 34 56 78
"#;

let aobs = parse_aobs(input);
println!("Parsed {} AOB instances", aobs.len());
```

### Pattern Matching

#### `matches_pattern`

Check if a byte value matches a pattern.

```rust
pub fn matches_pattern(value: u8, pattern: BytePattern) -> bool
```

**Example:**
```rust
use sig_maker_core::{BytePattern, matches_pattern};

assert!(matches_pattern(0x5F, BytePattern::Fixed(0x5F)));
assert!(!matches_pattern(0x5F, BytePattern::Fixed(0x40)));

assert!(matches_pattern(0x45, BytePattern::HighNibble(4)));
assert!(!matches_pattern(0x55, BytePattern::HighNibble(4)));

assert!(matches_pattern(0x3F, BytePattern::LowNibble(0x0F)));
assert!(!matches_pattern(0x30, BytePattern::LowNibble(0x0F)));

assert!(matches_pattern(0x00, BytePattern::Wildcard));
assert!(matches_pattern(0xFF, BytePattern::Wildcard));
```

#### `aob_matches_pattern`

Check if an AOB (array of optional bytes) matches a pattern.

```rust
pub fn aob_matches_pattern(aob: &[Option<u8>], pattern: &[BytePattern]) -> bool
```

**Example:**
```rust
use sig_maker_core::{BytePattern, aob_matches_pattern};

let aob = vec![Some(0x5F), None, Some(0x00)];
let pattern = vec![
    BytePattern::Fixed(0x5F),
    BytePattern::Wildcard,
    BytePattern::Fixed(0x00),
];

assert!(aob_matches_pattern(&aob, &pattern));
```

#### `instance_matches_pattern`

Check if an `AobInstance` matches a pattern.

```rust
pub fn instance_matches_pattern(instance: &AobInstance, pattern: &[BytePattern]) -> bool
```

### Pattern Statistics

#### `get_pattern_stats`

Get statistics for a pattern.

```rust
pub fn get_pattern_stats(pattern: &[BytePattern]) -> PatternStats
```

**Example:**
```rust
use sig_maker_core::{BytePattern, get_pattern_stats};

let pattern = vec![
    BytePattern::Fixed(0x5F),
    BytePattern::HighNibble(0x05),
    BytePattern::Wildcard,
];

let stats = get_pattern_stats(&pattern);
println!("Entropy: {:.2}", stats.entropy());
```

### Input/Output

#### `Input`

Struct for reading and detecting input content.

```rust
pub struct Input {
    pub content: String,
}
```

**Methods:**

##### `Input::read`

Read input from file, stdin, or use as direct input.

```rust
pub fn read(source: &str) -> Result<Self, String>
```

**Behavior:**
- If `source` is `"-"` and stdin is enabled → reads from stdin
- If `source` is an existing file path → reads from file
- Otherwise → treats `source` as direct pattern input

**Example:**
```rust
use sig_maker_core::io::Input;

// Read from file
let input = Input::read("patterns.txt").unwrap();

// Read from stdin
let input = Input::read_with_stdin("-", true).unwrap();

// Direct input
let input = Input::read("0? 00 ?? 01").unwrap();
```

##### `Input::read_with_stdin`

Read input with explicit stdin control.

```rust
pub fn read_with_stdin(source: &str, use_stdin: bool) -> Result<Self, String>
```

##### `Input::detect_type`

Detect the type of input content.

```rust
pub fn detect_type(&self) -> InputType
```

**Returns:**
- `InputType::MultipleAobs` - Multiple AOB instances for analysis
- `InputType::CodePattern` - Single pattern in code format (C++, Rust, JSON, etc.)
- `InputType::SimplePattern` - Single pattern in simple format (Cheat Engine, etc.)

**Example:**
```rust
use sig_maker_core::io::{Input, InputType};

let input = Input::read("0? 00 ?? 01").unwrap();
let input_type = input.detect_type();
println!("Detected: {:?}", input_type);
```

##### `Input::extract_pattern`

Extract the first valid pattern from input.

```rust
pub fn extract_pattern(&self) -> Option<String>
```

**Example:**
```rust
use sig_maker_core::io::Input;

let input = Input::read("0? 00 ?? 01").unwrap();
let pattern = input.extract_pattern().unwrap();
println!("Pattern: {}", pattern);
```

#### `InputType`

Enum representing detected input type.

```rust
pub enum InputType {
    MultipleAobs,    // Multiple AOB instances for analysis
    CodePattern,     // Single pattern for conversion (code format)
    SimplePattern,  // Single pattern for conversion (simple format)
}
```

### Validation

#### `validate_pattern`

Validate a pattern string without converting.

```rust
pub fn validate_pattern(input: &str) -> Result<(), String>
```

**Example:**
```rust
use sig_maker_core::validate_pattern;

assert!(validate_pattern("0? 00 ?? 01").is_ok());
assert!(validate_pattern("invalid pattern").is_err());
```

## How Pattern Parsing Works

The parser uses a hierarchical approach to detect and parse different formats:

1. **Check for code formats first** (C++, Rust, JSON) - These have distinctive syntax
2. **Check for x64dbg** - Uses dots (`.`) for wildcards
3. **Check for IDA/Ghidra** - Uses brackets (`[XX-YY]`) for ranges
4. **Check for Cheat Engine** - Uses question marks (`?`) for wildcards
5. **Fallback to raw hex** - Space-separated hex bytes

### Format-Specific Parsing

#### Cheat Engine
- `??` or `?` → Wildcard
- `X?` → High nibble (X is high nibble, low is wildcard)
- `?X` → Low nibble (low is X, high is wildcard)
- `XX` → Fixed byte

#### x64dbg
- `.` or `..` → Wildcard
- `.X` → Low nibble
- `X.` → High nibble
- `XX` → Fixed byte

#### IDA Pro / Ghidra
- `XX` → Fixed byte
- `[XX-YY]` → Range, converted to nibble wildcards if possible
  - `[00-0F]` → Low nibble 0
  - `[00-F0]` → High nibble 0
  - Other ranges → Wildcard

#### C++ / Rust
- Parses `pattern` array and `mask` array
- Applies mask to determine pattern type:
  - `0xFF` → Fixed byte
  - `0x00` → Wildcard
  - `0xF0` → High nibble
  - `0x0F` → Low nibble

#### JSON
- Parses `pattern` and `mask` arrays from JSON
- Applies same mask logic as C++/Rust

## How Pattern Conversion Works

Conversion is a two-step process:

1. **Parse** input format → `BytePattern` array
2. **Format** `BytePattern` array → output format

The `BytePattern` representation is format-agnostic, making conversion between any two formats straightforward.

### Conversion Example

```rust
use sig_maker_core::{parse_pattern, format_pattern, Format};

// Convert Cheat Engine to C++
let ce_input = "0? 00 ?? 01 ?F";
let pattern = parse_pattern(ce_input).unwrap();
let cpp_output = format_pattern(&pattern, Format::Cpp);
println!("{}", cpp_output);
```

## How Pattern Optimization Works

Pattern optimization analyzes multiple AOB instances to find the most specific pattern that matches all instances.

### Optimization Algorithm

For each byte position across all AOB instances:

1. **Check for original wildcards** - If any instance had a wildcard at this position (from pattern expansion), only analyze non-expanded instances
2. **Collect values** - Gather all byte values at this position
3. **Apply optimization logic**:
   - If all values are identical → Use `Fixed(value)`
   - If all values share the same high nibble → Use `HighNibble(high)`
   - If all values share the same low nibble → Use `LowNibble(low)`
   - Otherwise → Use `Wildcard`

### Pattern Expansion

When parsing patterns with wildcards, they are expanded to representative instances:
- Wildcards are replaced with placeholder values (0x00)
- Original wildcard positions are tracked
- During optimization, positions with original wildcards are handled specially to avoid over-constraining the pattern

### Example

Given these AOB instances:
```
48 8B 05 ? ? ? ?
48 8B 05 12 34 56 78
48 8B 05 AB CD EF 00
```

The optimizer analyzes each position:
- Position 0-2: All instances have `48 8B 05` → Fixed bytes
- Position 3-5: First instance has wildcards, others vary → Analyze non-expanded instances only
  - If positions 3-5 in instances 2-3 share commonality → Use nibble wildcards
  - Otherwise → Use wildcards
- Position 6: All instances have different values → Wildcard

Result might be: `48 8B 05 ? ? ? ?` (if no commonality) or `48 8B 05 1? ? ? ?` (if high nibble is common)

## Supported Formats and Their Characteristics

### Cheat Engine
- **Wildcard:** `??` (full), `?X` (low nibble), `X?` (high nibble)
- **Example:** `0? 00 ?? 01 ?F`
- **Use case:** Game memory scanning, cheat tables

### C++
- **Format:** Two arrays (pattern + mask)
- **Wildcard:** Mask byte (0x00 = wildcard, 0xF0 = high nibble, 0x0F = low nibble)
- **Example:**
  ```cpp
  const uint8_t pattern[] = { 0x00, 0x00, 0x00 };
  const uint8_t mask[]    = { 0xF0, 0xFF, 0x00 };
  ```
- **Use case:** C/C++ applications, game hacking

### Rust
- **Format:** Two static arrays (pattern + mask)
- **Wildcard:** Same as C++
- **Example:**
  ```rust
  static PATTERN: [u8; 3] = [0x00, 0x00, 0x00];
  static MASK: [u8; 3]    = [0xF0, 0xFF, 0x00];
  ```
- **Use case:** Rust applications, memory scanning

### Ghidra
- **Wildcard:** `.` (full), `[XX-YY]` (range)
- **Example:** `0? 00 . 01 [00-0F]`
- **Use case:** Reverse engineering, binary analysis

### IDA Pro
- **Wildcard:** `[0-F0]` (full), `[XX-YY]` (range)
- **Example:** `0? 00 [0-F0] 01 [00-0F]`
- **Use case:** Disassembly, binary analysis

### x64dbg
- **Wildcard:** `..` (full), `.X` (low nibble), `X.` (high nibble)
- **Example:** `0. 00 .. 01 .F`
- **Use case:** Windows debugging, reverse engineering

### Python
- **Format:** Regex byte pattern
- **Wildcard:** `.` (full), `[\xXX-\xYY]` (range)
- **Example:**
  ```python
  import re
  pattern = re.compile(b'\x0? \x00 . \x01 [\x00-\x0F]')
  ```
- **Use case:** Python scripts, binary analysis

### JSON
- **Format:** Structured data with pattern, mask, and length
- **Wildcard:** Mask byte (same as C++/Rust)
- **Example:**
  ```json
  { "pattern": [0, 0, 0], "mask": [240, 255, 0], "length": 3 }
  ```
- **Use case:** Data interchange, API responses

## Complete Examples

### Example 1: Simple Pattern Conversion

```rust
use sig_maker_core::{parse_pattern, format_pattern, Format};

fn main() {
    // Parse a Cheat Engine pattern
    let ce_pattern = "0? 00 ?? 01 ?F";
    let parsed = parse_pattern(ce_pattern).unwrap();
    
    // Convert to all formats
    for format in Format::all() {
        let output = format_pattern(&parsed, *format);
        println!("{}:\n{}\n", format.name(), output);
    }
}
```

### Example 2: Pattern Optimization

```rust
use sig_maker_core::analyze_aobs;

fn main() {
    let input = r#"
48 8B 05 ? ? ? ?
48 8B 05 12 34 56 78
48 8B 05 AB CD EF 00
48 8B 05 90 AB CD EF
"#;
    
    let (pattern, stats) = analyze_aobs(input);
    
    println!("Optimized pattern:");
    println!("  Total bytes: {}", stats.total_bytes());
    println!("  Fixed bytes: {}", stats.fixed_bytes());
    println!("  High nibble wildcards: {}", stats.high_nibble_wildcards());
    println!("  Low nibble wildcards: {}", stats.low_nibble_wildcards());
    println!("  Full wildcards: {}", stats.full_wildcards());
    println!("  Entropy: {:.2}", stats.entropy());
    println!("  Compression ratio: {:.2}", stats.compression_ratio());
}
```

### Example 3: Input Detection and Processing

```rust
use sig_maker_core::io::{Input, InputType};
use sig_maker_core::{parse_pattern, format_pattern, Format};

fn main() {
    // Read input (could be file, stdin, or direct pattern)
    let input = Input::read("0? 00 ?? 01").unwrap();
    
    // Detect input type
    match input.detect_type() {
        InputType::MultipleAobs => {
            println!("Detected multiple AOB instances - use analyze_aobs()");
        }
        InputType::CodePattern => {
            println!("Detected code pattern - parsing and converting");
            let pattern_str = input.extract_pattern().unwrap();
            let pattern = parse_pattern(&pattern_str).unwrap();
            let output = format_pattern(&pattern, Format::CheatEngine);
            println!("{}", output);
        }
        InputType::SimplePattern => {
            println!("Detected simple pattern - parsing and converting");
            let pattern_str = input.extract_pattern().unwrap();
            let pattern = parse_pattern(&pattern_str).unwrap();
            let output = format_pattern(&pattern, Format::CheatEngine);
            println!("{}", output);
        }
    }
}
```

### Example 4: Pattern Validation

```rust
use sig_maker_core::validate_pattern;

fn main() {
    let patterns = vec![
        "0? 00 ?? 01",
        "const uint8_t pattern[] = { 0x00, 0x01 };",
        "{ \"pattern\": [0, 1], \"mask\": [255, 255] }",
        "invalid pattern",
    ];
    
    for pattern in patterns {
        match validate_pattern(pattern) {
            Ok(()) => println!("✓ Valid: {}", pattern),
            Err(e) => println!("✗ Invalid: {} - {}", pattern, e),
        }
    }
}
```

### Example 5: Custom Pattern Analysis

```rust
use sig_maker_core::{BytePattern, optimize_byte, matches_pattern};

fn main() {
    // Analyze byte values from memory dumps
    let values = vec![0x40, 0x45, 0x4A, 0x4F];
    let pattern = optimize_byte(&values);
    
    println!("Optimal pattern for {:?}: {:?}", values, pattern);
    
    // Test matching
    for test_val in &[0x40, 0x45, 0x50, 0x55] {
        let matches = matches_pattern(*test_val, pattern);
        println!("  {:02X} matches: {}", test_val, matches);
    }
}
```

## Testing

The crate includes comprehensive unit tests for all major functionality:

```bash
# Run all tests
cargo test --package sig-maker-core

# Run specific test
cargo test --package sig-maker-core test_optimize_byte_fixed
```

## Design Principles

1. **Zero Dependencies** - No external crates for maximum portability
2. **Format Agnostic** - Internal representation is format-independent
3. **Efficient** - Minimal allocations, straightforward algorithms
4. **Explicit** - Clear types and methods, no hidden behavior
5. **Testable** - Pure functions, easy to unit test

## Performance Considerations

- Pattern parsing is O(n) where n is the length of the input string
- Pattern optimization is O(m × k) where m is the number of bytes and k is the number of AOB instances
- Pattern formatting is O(n) where n is the pattern length
- Memory usage is minimal - only stores the pattern and statistics

## Limitations

- Pattern expansion uses placeholder values (0x00) to avoid combinatorial explosion
- Complex ranges in IDA/Ghidra formats may be converted to full wildcards
- No support for named patterns or pattern libraries
- No support for pattern masking beyond nibble-level

## Future Enhancements

Potential areas for expansion:
- Support for additional formats (e.g., Frida, ScyllaHide)
- Pattern library management
- Pattern similarity detection
- Automatic pattern generation from byte differences
- Support for multi-byte patterns (e.g., dword, qword wildcards)

## License

This crate is part of the sig-maker project. See the main project LICENSE file for details.
