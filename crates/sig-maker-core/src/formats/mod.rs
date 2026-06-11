//! Format definitions, parsers and formatters for AOB patterns

pub mod formatter;
pub mod parser;

/// Supported AOB pattern formats
#[derive(Debug, Clone, Copy, PartialEq)]
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

impl Format {
    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "ce" | "cheatengine" | "cheat-engine" => Some(Format::CheatEngine),
            "cpp" | "c++" | "c" => Some(Format::Cpp),
            "rust" | "rs" => Some(Format::Rust),
            "ghidra" | "gh" => Some(Format::Ghidra),
            "ida" | "idapro" | "ida-pro" => Some(Format::IdaPro),
            "x64dbg" | "x64" => Some(Format::X64dbg),
            "py" | "python" => Some(Format::Python),
            "json" => Some(Format::Json),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Format::CheatEngine => "Cheat Engine",
            Format::Cpp => "C++",
            Format::Rust => "Rust",
            Format::Ghidra => "Ghidra",
            Format::IdaPro => "IDA Pro",
            Format::X64dbg => "x64dbg",
            Format::Python => "Python",
            Format::Json => "JSON",
        }
    }

    /// All supported formats
    pub fn all() -> &'static [Format] {
        &[
            Format::CheatEngine,
            Format::Cpp,
            Format::Rust,
            Format::Ghidra,
            Format::IdaPro,
            Format::X64dbg,
            Format::Python,
            Format::Json,
        ]
    }
}

/// Byte pattern with wildcard support
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BytePattern {
    Fixed(u8),
    Wildcard,
    HighNibble(u8),
    LowNibble(u8),
}

/// Check if a value matches a pattern
pub fn matches_pattern(value: u8, pattern: BytePattern) -> bool {
    match pattern {
        BytePattern::Fixed(p) => value == p,
        BytePattern::Wildcard => true,
        BytePattern::HighNibble(h) => (value >> 4) == h,
        BytePattern::LowNibble(l) => (value & 0x0F) == l,
    }
}

/// Find the best pattern for a set of byte values
pub fn optimize_byte(values: &[u8]) -> BytePattern {
    if values.is_empty() {
        return BytePattern::Wildcard;
    }

    let first = values[0];

    // Check if all values are the same
    if values.iter().all(|&v| v == first) {
        return BytePattern::Fixed(first);
    }

    // Check if high nibble is the same
    let high_nibble = first >> 4;
    if values.iter().all(|&v| (v >> 4) == high_nibble) {
        return BytePattern::HighNibble(high_nibble);
    }

    // Check if low nibble is the same
    let low_nibble = first & 0x0F;
    if values.iter().all(|&v| (v & 0x0F) == low_nibble) {
        return BytePattern::LowNibble(low_nibble);
    }

    BytePattern::Wildcard
}

// Re-export main functions
pub use formatter::format_pattern;
pub use parser::parse_pattern;

/// Validate a pattern string without converting
pub fn validate_pattern(input: &str) -> Result<(), String> {
    match parse_pattern(input) {
        Some(_) => Ok(()),
        None => Err("Could not parse pattern".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimize_byte_fixed() {
        let values = vec![0x5F, 0x5F, 0x5F];
        assert_eq!(optimize_byte(&values), BytePattern::Fixed(0x5F));
    }

    #[test]
    fn test_optimize_byte_high_nibble() {
        let values = vec![0x40, 0x45, 0x4F];
        assert_eq!(optimize_byte(&values), BytePattern::HighNibble(4));
    }

    #[test]
    fn test_optimize_byte_low_nibble() {
        let values = vec![0x0F, 0x3F, 0xBF];
        assert_eq!(optimize_byte(&values), BytePattern::LowNibble(0x0F));
    }

    #[test]
    fn test_optimize_byte_wildcard() {
        let values = vec![0x00, 0xFF, 0x42];
        assert_eq!(optimize_byte(&values), BytePattern::Wildcard);
    }

    #[test]
    fn test_matches_pattern() {
        assert!(matches_pattern(0x5F, BytePattern::Fixed(0x5F)));
        assert!(!matches_pattern(0x5F, BytePattern::Fixed(0x40)));

        assert!(matches_pattern(0x45, BytePattern::HighNibble(4)));
        assert!(!matches_pattern(0x55, BytePattern::HighNibble(4)));

        assert!(matches_pattern(0x3F, BytePattern::LowNibble(0x0F)));
        assert!(!matches_pattern(0x30, BytePattern::LowNibble(0x0F)));

        assert!(matches_pattern(0x00, BytePattern::Wildcard));
        assert!(matches_pattern(0xFF, BytePattern::Wildcard));
    }

    #[test]
    fn test_format_parse_roundtrip_ce() {
        let input = "0? 00 00 00 01 00 00 00 E? FF FF FF 00";
        let parsed = parse_pattern(input).unwrap();
        let formatted = format_pattern(&parsed, Format::CheatEngine);
        assert_eq!(formatted, input);
    }

    #[test]
    fn test_parse_cheat_engine() {
        let input = "0? 00 ?? 01 ?F F?";
        let result = parse_pattern(input).unwrap();

        assert_eq!(result[0], BytePattern::HighNibble(0));
        assert_eq!(result[1], BytePattern::Fixed(0x00));
        assert_eq!(result[2], BytePattern::Wildcard);
        assert_eq!(result[3], BytePattern::Fixed(0x01));
        assert_eq!(result[4], BytePattern::LowNibble(0x0F));
        assert_eq!(result[5], BytePattern::HighNibble(0x0F));
    }

    #[test]
    fn test_parse_cpp_array() {
        let input = "const uint8_t pattern[] = { 0x00, 0x01, 0xE0 }; const uint8_t mask[] = { 0xF0, 0xFF, 0xF0 };";
        let result = parse_pattern(input).unwrap();

        assert_eq!(result[0], BytePattern::HighNibble(0));
        assert_eq!(result[1], BytePattern::Fixed(0x01));
        assert_eq!(result[2], BytePattern::HighNibble(0x0E));
    }

    #[test]
    fn test_parse_json() {
        let input = r#"{ "pattern": [0, 0, 1, 224], "mask": [240, 255, 255, 240] }"#;
        let result = parse_pattern(input).unwrap();

        assert_eq!(result[0], BytePattern::HighNibble(0));
        assert_eq!(result[1], BytePattern::Fixed(0x00));
        assert_eq!(result[2], BytePattern::Fixed(0x01));
        assert_eq!(result[3], BytePattern::HighNibble(0x0E));
    }

    #[test]
    fn test_format_all_formats() {
        let pattern = vec![
            BytePattern::HighNibble(0),
            BytePattern::Fixed(0x00),
            BytePattern::Fixed(0x01),
            BytePattern::HighNibble(0x0E),
        ];

        for format in Format::all() {
            let output = format_pattern(&pattern, *format);
            assert!(
                !output.is_empty(),
                "Format {:?} produced empty output",
                format
            );
        }
    }
}
