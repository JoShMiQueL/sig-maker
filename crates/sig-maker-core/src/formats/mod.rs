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
    fn format_from_string_variants() {
        assert_eq!(Format::from_string("ce"), Some(Format::CheatEngine));
        assert_eq!(
            Format::from_string("cheatengine"),
            Some(Format::CheatEngine)
        );
        assert_eq!(
            Format::from_string("cheat-engine"),
            Some(Format::CheatEngine)
        );
        assert_eq!(Format::from_string("cpp"), Some(Format::Cpp));
        assert_eq!(Format::from_string("c++"), Some(Format::Cpp));
        assert_eq!(Format::from_string("c"), Some(Format::Cpp));
        assert_eq!(Format::from_string("rust"), Some(Format::Rust));
        assert_eq!(Format::from_string("rs"), Some(Format::Rust));
        assert_eq!(Format::from_string("ghidra"), Some(Format::Ghidra));
        assert_eq!(Format::from_string("gh"), Some(Format::Ghidra));
        assert_eq!(Format::from_string("ida"), Some(Format::IdaPro));
        assert_eq!(Format::from_string("idapro"), Some(Format::IdaPro));
        assert_eq!(Format::from_string("ida-pro"), Some(Format::IdaPro));
        assert_eq!(Format::from_string("x64dbg"), Some(Format::X64dbg));
        assert_eq!(Format::from_string("x64"), Some(Format::X64dbg));
        assert_eq!(Format::from_string("py"), Some(Format::Python));
        assert_eq!(Format::from_string("python"), Some(Format::Python));
        assert_eq!(Format::from_string("json"), Some(Format::Json));
        assert_eq!(Format::from_string("invalid"), None);
    }

    #[test]
    fn format_from_string_case_insensitive() {
        assert_eq!(Format::from_string("CE"), Some(Format::CheatEngine));
        assert_eq!(Format::from_string("RUST"), Some(Format::Rust));
        assert_eq!(Format::from_string("JSON"), Some(Format::Json));
    }

    #[test]
    fn format_name() {
        assert_eq!(Format::CheatEngine.name(), "Cheat Engine");
        assert_eq!(Format::Cpp.name(), "C++");
        assert_eq!(Format::Rust.name(), "Rust");
        assert_eq!(Format::Ghidra.name(), "Ghidra");
        assert_eq!(Format::IdaPro.name(), "IDA Pro");
        assert_eq!(Format::X64dbg.name(), "x64dbg");
        assert_eq!(Format::Python.name(), "Python");
        assert_eq!(Format::Json.name(), "JSON");
    }

    #[test]
    fn format_all() {
        let all = Format::all();
        assert_eq!(all.len(), 8);
        assert!(all.contains(&Format::CheatEngine));
        assert!(all.contains(&Format::Cpp));
        assert!(all.contains(&Format::Rust));
        assert!(all.contains(&Format::Ghidra));
        assert!(all.contains(&Format::IdaPro));
        assert!(all.contains(&Format::X64dbg));
        assert!(all.contains(&Format::Python));
        assert!(all.contains(&Format::Json));
    }

    #[test]
    fn byte_pattern_fixed() {
        let pattern = BytePattern::Fixed(0xAB);
        assert_eq!(pattern, BytePattern::Fixed(0xAB));
        assert_ne!(pattern, BytePattern::Fixed(0xCD));
    }

    #[test]
    fn byte_pattern_wildcard() {
        let pattern = BytePattern::Wildcard;
        assert_eq!(pattern, BytePattern::Wildcard);
    }

    #[test]
    fn byte_pattern_high_nibble() {
        let pattern = BytePattern::HighNibble(0xA);
        assert_eq!(pattern, BytePattern::HighNibble(0xA));
        assert_ne!(pattern, BytePattern::HighNibble(0xB));
    }

    #[test]
    fn byte_pattern_low_nibble() {
        let pattern = BytePattern::LowNibble(0xB);
        assert_eq!(pattern, BytePattern::LowNibble(0xB));
        assert_ne!(pattern, BytePattern::LowNibble(0xC));
    }

    #[test]
    fn matches_pattern_fixed() {
        assert!(matches_pattern(0xAB, BytePattern::Fixed(0xAB)));
        assert!(!matches_pattern(0xAB, BytePattern::Fixed(0xCD)));
    }

    #[test]
    fn matches_pattern_wildcard() {
        assert!(matches_pattern(0xAB, BytePattern::Wildcard));
        assert!(matches_pattern(0x00, BytePattern::Wildcard));
        assert!(matches_pattern(0xFF, BytePattern::Wildcard));
    }

    #[test]
    fn matches_pattern_high_nibble() {
        assert!(matches_pattern(0xAB, BytePattern::HighNibble(0xA)));
        assert!(matches_pattern(0xAF, BytePattern::HighNibble(0xA)));
        assert!(!matches_pattern(0xAB, BytePattern::HighNibble(0xB)));
        assert!(!matches_pattern(0x0B, BytePattern::HighNibble(0xA)));
    }

    #[test]
    fn matches_pattern_low_nibble() {
        assert!(matches_pattern(0xAB, BytePattern::LowNibble(0xB)));
        assert!(matches_pattern(0x0B, BytePattern::LowNibble(0xB)));
        assert!(!matches_pattern(0xAB, BytePattern::LowNibble(0xC)));
        assert!(!matches_pattern(0xB0, BytePattern::LowNibble(0xB)));
    }

    #[test]
    fn optimize_byte_empty() {
        assert_eq!(optimize_byte(&[]), BytePattern::Wildcard);
    }

    #[test]
    fn optimize_byte_all_same() {
        assert_eq!(optimize_byte(&[0xAB, 0xAB, 0xAB]), BytePattern::Fixed(0xAB));
        assert_eq!(optimize_byte(&[0x00, 0x00]), BytePattern::Fixed(0x00));
    }

    #[test]
    fn optimize_byte_high_nibble_same() {
        assert_eq!(
            optimize_byte(&[0xAB, 0xAC, 0xAD]),
            BytePattern::HighNibble(0xA)
        );
        assert_eq!(
            optimize_byte(&[0x10, 0x1F, 0x15]),
            BytePattern::HighNibble(0x1)
        );
    }

    #[test]
    fn optimize_byte_low_nibble_same() {
        assert_eq!(
            optimize_byte(&[0xAB, 0xCB, 0x0B]),
            BytePattern::LowNibble(0xB)
        );
        assert_eq!(
            optimize_byte(&[0x05, 0xF5, 0xA5]),
            BytePattern::LowNibble(0x5)
        );
    }

    #[test]
    fn optimize_byte_wildcard() {
        assert_eq!(optimize_byte(&[0xAB, 0xCD, 0xEF]), BytePattern::Wildcard);
        assert_eq!(optimize_byte(&[0x00, 0xFF, 0x55]), BytePattern::Wildcard);
    }

    #[test]
    fn optimize_byte_single_value() {
        assert_eq!(optimize_byte(&[0xAB]), BytePattern::Fixed(0xAB));
    }

    #[test]
    fn validate_pattern_valid() {
        assert!(validate_pattern("AB CD EF").is_ok());
        assert!(validate_pattern("AB ? ?F").is_ok());
        assert!(validate_pattern("AB ?? CD").is_ok());
    }

    #[test]
    fn validate_pattern_invalid() {
        assert!(validate_pattern("").is_err());
        assert!(validate_pattern("invalid").is_err());
        assert!(validate_pattern("XYZ").is_err());
    }
}
