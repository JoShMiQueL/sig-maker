# sig-maker-core Documentation Guidelines

This crate is the core library for signature/pattern conversion and optimization.

## Documentation Standards

When documenting sig-maker-core:
- Focus on the public API: functions, types, enums that are re-exported in lib.rs
- Include code examples for all major functions
- Explain the pattern matching logic (BytePattern variants)
- Document the wildcard syntax for each supported format
- Explain the optimization algorithm (optimize_byte, analyze_aobs)

## Key Files to Document

- `src/lib.rs` - Main API exports
- `src/formats/mod.rs` - Format definitions and pattern types
- `src/formats/parser.rs` - Pattern parsing logic
- `src/formats/formatter.rs` - Pattern formatting logic
- `src/converter.rs` - Conversion between formats
- `src/analyzer.rs` - Pattern optimization and analysis
- `src/io.rs` - Input/Output handling

## API Coverage

Document these public APIs:
- `parse_pattern()` - Parse pattern string to BytePattern array
- `format_pattern()` - Format BytePattern array to specific format
- `convert_to_format()` - Convert pattern to format
- `get_pattern_stats()` - Get statistics about a pattern
- `optimize_byte()` - Find best pattern for byte values
- `analyze_aobs()` - Analyze multiple AOB instances
- `Format::all()` - Get all supported formats
- `Format::from_string()` - Parse format from string

## Format Documentation

For each format, document:
- Wildcard syntax (e.g., `??` for Cheat Engine, `[0-F0]` for IDA Pro)
- Example patterns
- Typical use cases
- Any format-specific quirks

## Code Examples

Include examples for:
- Simple pattern conversion
- Pattern optimization with multiple instances
- Input detection and processing
- Pattern validation
