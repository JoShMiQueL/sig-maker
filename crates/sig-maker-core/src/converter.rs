//! Pattern conversion - convert single pattern between formats

use crate::PatternStats;
use crate::formats::{BytePattern, Format, format_pattern, parse_pattern};

/// Convert a single pattern to a specific format
pub fn convert_to_format(pattern: &[BytePattern], format: Format) -> String {
    format_pattern(pattern, format)
}

/// Parse a pattern string into BytePattern array
pub fn parse_single_pattern(content: &str) -> Option<Vec<BytePattern>> {
    parse_pattern(content)
}

/// Get statistics for a pattern
pub fn get_pattern_stats(pattern: &[BytePattern]) -> PatternStats {
    PatternStats::from_patterns(pattern)
}
