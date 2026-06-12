//! Pattern conversion - convert single pattern between formats

use crate::formats::{BytePattern, Format, format_pattern, parse_pattern};

/// Convert a single pattern to a specific format
pub fn convert_to_format(pattern: &[BytePattern], format: Format) -> String {
    format_pattern(pattern, format)
}

/// Parse a pattern string into BytePattern array
pub fn parse_single_pattern(content: &str) -> Option<Vec<BytePattern>> {
    parse_pattern(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_to_format_cheat_engine() {
        let pattern = vec![BytePattern::Fixed(0xAB), BytePattern::Wildcard];
        let result = convert_to_format(&pattern, Format::CheatEngine);
        assert_eq!(result, "AB ??");
    }

    #[test]
    fn parse_single_pattern_valid() {
        let result = parse_single_pattern("AB CD EF");
        assert_eq!(
            result,
            Some(vec![
                BytePattern::Fixed(0xAB),
                BytePattern::Fixed(0xCD),
                BytePattern::Fixed(0xEF),
            ])
        );
    }

    #[test]
    fn parse_single_pattern_invalid() {
        let result = parse_single_pattern("invalid");
        assert_eq!(result, None);
    }

    #[test]
    fn parse_single_pattern_empty() {
        let result = parse_single_pattern("");
        assert_eq!(result, None);
    }
}
