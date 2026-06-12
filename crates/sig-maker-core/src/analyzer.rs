//! AOB analysis - compare multiple instances and find optimal pattern

use crate::formats::BytePattern;

/// Analyze multiple AOB instances and generate optimized pattern
/// Algorithm: if bytes vary, check for shared nibbles (HighNibble/LowNibble),
/// otherwise use "??". If bytes are identical, keep the byte.
pub fn analyze_aobs(content: &str) -> Vec<BytePattern> {
    let aobs = parse_aobs(content);

    if aobs.is_empty() {
        return vec![];
    }

    // Verify all have same length
    let first_len = aobs[0].bytes.len();
    for (i, aob) in aobs.iter().enumerate() {
        if aob.bytes.len() != first_len {
            panic!(
                "AOB {} has {} bytes, expected {}",
                i + 1,
                aob.bytes.len(),
                first_len
            );
        }
    }

    // Analyze byte-by-byte
    let mut result: Vec<BytePattern> = Vec::with_capacity(first_len);

    for byte_idx in 0..first_len {
        // Collect all byte values at this position
        let values: Vec<u8> = aobs.iter().filter_map(|aob| aob.bytes[byte_idx]).collect();

        if values.is_empty() {
            // All wildcards at this position
            result.push(BytePattern::Wildcard);
        } else {
            // Check if all values are the same
            let first = values[0];
            if values.iter().all(|&v| v == first) {
                result.push(BytePattern::Fixed(first));
            } else {
                // Bytes vary - check for shared nibbles
                let high_nibbles: Vec<u8> = values.iter().map(|&v| v >> 4).collect();
                let low_nibbles: Vec<u8> = values.iter().map(|&v| v & 0x0F).collect();

                let first_high = high_nibbles[0];
                let first_low = low_nibbles[0];

                if high_nibbles.iter().all(|&n| n == first_high) {
                    // All share the same high nibble
                    result.push(BytePattern::HighNibble(first_high));
                } else if low_nibbles.iter().all(|&n| n == first_low) {
                    // All share the same low nibble
                    result.push(BytePattern::LowNibble(first_low));
                } else {
                    // No shared nibbles - use wildcard
                    result.push(BytePattern::Wildcard);
                }
            }
        }
    }

    result
}

/// Parse AOB instances from file content
/// Supports raw hex bytes (one per line)
pub fn parse_aobs(content: &str) -> Vec<AobInstance> {
    let lines: Vec<&str> = content.lines().collect();
    let mut aobs = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
            continue;
        }

        // Remove common prefixes
        let cleaned = if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            &trimmed[2..]
        } else {
            trimmed
        };

        // Parse as simple hex bytes
        let bytes: Vec<Option<u8>> = cleaned
            .split_whitespace()
            .map(|s| u8::from_str_radix(s, 16).ok())
            .collect();

        if !bytes.is_empty() {
            let len = bytes.len();
            aobs.push(AobInstance {
                bytes,
                line_num: i + 1,
                is_expanded: false,
                wildcard_positions: vec![false; len],
            });
        }
    }

    aobs
}

/// AOB instance with source line info
pub struct AobInstance {
    pub bytes: Vec<Option<u8>>,
    pub line_num: usize,
    /// Marks if this instance came from expanding a pattern (vs raw hex)
    pub is_expanded: bool,
    /// For expanded instances, tracks which positions had wildcards in original pattern
    pub wildcard_positions: Vec<bool>,
}

/// Check if an AOB matches a pattern
pub fn aob_matches_pattern(aob: &[Option<u8>], pattern: &[BytePattern]) -> bool {
    if aob.len() != pattern.len() {
        return false;
    }

    for (a, p) in aob.iter().zip(pattern.iter()) {
        if let Some(av) = a {
            if !crate::formats::matches_pattern(*av, *p) {
                return false;
            }
        } else {
            // AOB has wildcard - can only match if pattern also has wildcard
            if !matches!(*p, BytePattern::Wildcard) {
                return false;
            }
        }
    }
    true
}

/// Check if an AOB instance matches a pattern
pub fn instance_matches_pattern(instance: &AobInstance, pattern: &[BytePattern]) -> bool {
    if instance.bytes.len() != pattern.len() {
        return false;
    }

    for (a, p) in instance.bytes.iter().zip(pattern.iter()) {
        if let Some(av) = a {
            if !crate::formats::matches_pattern(*av, *p) {
                return false;
            }
        } else {
            // AOB has wildcard - can only match if pattern also has wildcard
            if !matches!(*p, BytePattern::Wildcard) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analyze_aobs_simple() {
        let content = "AB CD EF\nAB CD EF";
        let pattern = analyze_aobs(content);
        assert_eq!(pattern.len(), 3);
        assert_eq!(pattern[0], BytePattern::Fixed(0xAB));
        assert_eq!(pattern[1], BytePattern::Fixed(0xCD));
        assert_eq!(pattern[2], BytePattern::Fixed(0xEF));
    }

    #[test]
    fn analyze_aobs_with_variation() {
        let content = "AB CD EF\nAB 00 EF";
        let pattern = analyze_aobs(content);
        assert_eq!(pattern.len(), 3);
        assert_eq!(pattern[0], BytePattern::Fixed(0xAB));
        assert_eq!(pattern[1], BytePattern::Wildcard); // CD vs 00
        assert_eq!(pattern[2], BytePattern::Fixed(0xEF));
    }

    #[test]
    fn analyze_aobs_all_wildcard() {
        let content = "AB CD EF\n00 11 22\nFF EE DD";
        let pattern = analyze_aobs(content);
        assert_eq!(pattern.len(), 3);
        assert_eq!(pattern[0], BytePattern::Wildcard);
        assert_eq!(pattern[1], BytePattern::Wildcard);
        assert_eq!(pattern[2], BytePattern::Wildcard);
    }

    #[test]
    fn analyze_aobs_shared_high_nibble() {
        let content = "27 00 00\n28 00 00\n2A 00 00";
        let pattern = analyze_aobs(content);
        assert_eq!(pattern.len(), 3);
        assert_eq!(pattern[0], BytePattern::HighNibble(0x2)); // 27, 28, 2A share high nibble 0x2
        assert_eq!(pattern[1], BytePattern::Fixed(0x00));
        assert_eq!(pattern[2], BytePattern::Fixed(0x00));
    }

    #[test]
    fn analyze_aobs_shared_low_nibble() {
        let content = "07 00 00\n27 00 00\n47 00 00";
        let pattern = analyze_aobs(content);
        assert_eq!(pattern.len(), 3);
        assert_eq!(pattern[0], BytePattern::LowNibble(0x7)); // 07, 27, 47 share low nibble 0x7
        assert_eq!(pattern[1], BytePattern::Fixed(0x00));
        assert_eq!(pattern[2], BytePattern::Fixed(0x00));
    }

    #[test]
    fn analyze_aobs_user_example() {
        let content = "07 00 00 00 01 00 00 00 FF FF FF FF 00 00 00 00 00 00 00 00 10 B2 DA 97 B2 02 00 00 A0 6B DA 97 B2 02
08 00 00 00 01 00 00 00 EB FF FF FF 00 00 00 00 00 00 00 00 E0 54 11 15 57 02 00 00 E0 AA 10 15 57 02
09 00 00 00 01 00 00 00 EB FF FF FF 00 00 00 00 00 00 00 00 E0 54 11 15 57 02 00 00 E0 AA 10 15 57 02
09 00 00 00 01 00 00 00 EA FF FF FF 00 00 00 00 00 00 00 00 E0 54 11 15 57 02 00 00 E0 AA 10 15 57 02
FF FF FF FF 01 00 00 00 EA FF FF FF 00 00 00 00 00 00 00 00 E0 54 11 15 57 02 00 00 E0 AA 10 15 57 02
FF FF FF FF 01 00 00 00 01 00 00 00 00 00 00 00 00 00 00 00 E0 54 11 15 57 02 00 00 E0 AA 10 15 57 02
01 00 00 00 01 00 00 00 FF 00 EB 00 00 00 00 00 00 00 00 00 E0 54 11 15 57 02 00 00 E0 AA 10 15 57 02";
        let pattern = analyze_aobs(content);

        // The algorithm should work - just verify it produces a pattern
        assert!(!pattern.is_empty());
        // First byte varies (07, 08, 09, FF, FF, 01) - no shared nibbles
        assert_eq!(pattern[0], BytePattern::Wildcard);
    }

    #[test]
    fn parse_aobs_simple_hex() {
        let content = "AB CD EF\n12 34 56";
        let aobs = parse_aobs(content);
        assert_eq!(aobs.len(), 2);
        assert_eq!(aobs[0].bytes.len(), 3);
        assert_eq!(aobs[1].bytes.len(), 3);
        assert!(!aobs[0].is_expanded);
        assert!(!aobs[1].is_expanded);
    }

    #[test]
    fn parse_aobs_with_comments() {
        let content = "# Comment\nAB CD EF\n// Another comment\n12 34 56";
        let aobs = parse_aobs(content);
        assert_eq!(aobs.len(), 2);
    }

    #[test]
    fn parse_aobs_with_prefixes() {
        let content = "- AB CD EF\n* 12 34 56";
        let aobs = parse_aobs(content);
        assert_eq!(aobs.len(), 2);
    }

    #[test]
    fn parse_aobs_empty_lines() {
        let content = "AB CD EF\n\n\n12 34 56";
        let aobs = parse_aobs(content);
        assert_eq!(aobs.len(), 2);
    }

    #[test]
    fn aob_matches_pattern_exact() {
        let aob = vec![Some(0xAB), Some(0xCD), Some(0xEF)];
        let pattern = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Fixed(0xCD),
            BytePattern::Fixed(0xEF),
        ];
        assert!(aob_matches_pattern(&aob, &pattern));
    }

    #[test]
    fn aob_matches_pattern_with_wildcard() {
        let aob = vec![Some(0xAB), Some(0xCD), Some(0xEF)];
        let pattern = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Wildcard,
            BytePattern::Fixed(0xEF),
        ];
        assert!(aob_matches_pattern(&aob, &pattern));
    }

    #[test]
    fn aob_matches_pattern_mismatch() {
        let aob = vec![Some(0xAB), Some(0xCD), Some(0xEF)];
        let pattern = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Fixed(0x00),
            BytePattern::Fixed(0xEF),
        ];
        assert!(!aob_matches_pattern(&aob, &pattern));
    }

    #[test]
    fn aob_matches_pattern_length_mismatch() {
        let aob = vec![Some(0xAB), Some(0xCD)];
        let pattern = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Fixed(0xCD),
            BytePattern::Fixed(0xEF),
        ];
        assert!(!aob_matches_pattern(&aob, &pattern));
    }

    #[test]
    fn aob_matches_pattern_aob_wildcard() {
        let aob = vec![Some(0xAB), None, Some(0xEF)];
        let pattern = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Wildcard,
            BytePattern::Fixed(0xEF),
        ];
        assert!(aob_matches_pattern(&aob, &pattern));
    }

    #[test]
    fn aob_matches_pattern_aob_wildcard_mismatch() {
        let aob = vec![Some(0xAB), None, Some(0xEF)];
        let pattern = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Fixed(0x00),
            BytePattern::Fixed(0xEF),
        ];
        assert!(!aob_matches_pattern(&aob, &pattern));
    }

    #[test]
    fn instance_matches_pattern_exact() {
        let instance = AobInstance {
            bytes: vec![Some(0xAB), Some(0xCD), Some(0xEF)],
            line_num: 1,
            is_expanded: false,
            wildcard_positions: vec![false; 3],
        };
        let pattern = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Fixed(0xCD),
            BytePattern::Fixed(0xEF),
        ];
        assert!(instance_matches_pattern(&instance, &pattern));
    }

    #[test]
    fn instance_matches_pattern_with_wildcard() {
        let instance = AobInstance {
            bytes: vec![Some(0xAB), Some(0xCD), Some(0xEF)],
            line_num: 1,
            is_expanded: false,
            wildcard_positions: vec![false; 3],
        };
        let pattern = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Wildcard,
            BytePattern::Fixed(0xEF),
        ];
        assert!(instance_matches_pattern(&instance, &pattern));
    }

    #[test]
    fn instance_matches_pattern_mismatch() {
        let instance = AobInstance {
            bytes: vec![Some(0xAB), Some(0xCD), Some(0xEF)],
            line_num: 1,
            is_expanded: false,
            wildcard_positions: vec![false; 3],
        };
        let pattern = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Fixed(0x00),
            BytePattern::Fixed(0xEF),
        ];
        assert!(!instance_matches_pattern(&instance, &pattern));
    }
}
