//! AOB analysis - compare multiple instances and find optimal pattern

use crate::formats::{BytePattern, optimize_byte, parse_pattern};

/// AOB instance with source line info
pub struct AobInstance {
    pub bytes: Vec<Option<u8>>,
    #[allow(dead_code)]
    pub line_num: usize,
    /// Marks if this instance came from expanding a pattern (vs raw hex)
    pub is_expanded: bool,
    /// For expanded instances, tracks which positions had wildcards in original pattern
    pub wildcard_positions: Vec<bool>,
}

/// Statistics about pattern optimization
#[derive(Debug, Clone)]
pub struct PatternStats {
    fixed: usize,
    high_nibble: usize,
    low_nibble: usize,
    wildcard: usize,
    entropy: f64,
    compression_ratio: f64,
}

impl PatternStats {
    pub fn from_patterns(patterns: &[BytePattern]) -> Self {
        let mut stats = Self {
            fixed: 0,
            high_nibble: 0,
            low_nibble: 0,
            wildcard: 0,
            entropy: 0.0,
            compression_ratio: 0.0,
        };

        for p in patterns {
            match p {
                BytePattern::Fixed(_) => stats.fixed += 1,
                BytePattern::Wildcard => stats.wildcard += 1,
                BytePattern::HighNibble(_) => stats.high_nibble += 1,
                BytePattern::LowNibble(_) => stats.low_nibble += 1,
            }
        }

        // Calculate entropy (simplified: based on pattern type distribution)
        let total = stats.total_bytes();
        if total > 0 {
            let mut entropy = 0.0;
            let counts = [
                stats.fixed,
                stats.high_nibble,
                stats.low_nibble,
                stats.wildcard,
            ];
            for &count in &counts {
                if count > 0 {
                    let p = count as f64 / total as f64;
                    entropy -= p * p.log2();
                }
            }
            stats.entropy = entropy;

            // Compression ratio: fixed bytes / total bytes
            stats.compression_ratio = stats.fixed as f64 / total as f64;
        }

        stats
    }

    pub fn total_bytes(&self) -> usize {
        self.fixed + self.high_nibble + self.low_nibble + self.wildcard
    }

    pub fn fixed_bytes(&self) -> usize {
        self.fixed
    }
    pub fn high_nibble_wildcards(&self) -> usize {
        self.high_nibble
    }
    pub fn low_nibble_wildcards(&self) -> usize {
        self.low_nibble
    }
    pub fn full_wildcards(&self) -> usize {
        self.wildcard
    }
    pub fn entropy(&self) -> f64 {
        self.entropy
    }
    pub fn compression_ratio(&self) -> f64 {
        self.compression_ratio
    }
}

/// Analyze multiple AOB instances and generate optimized pattern
/// Returns the optimized pattern and statistics
pub fn analyze_aobs(content: &str) -> (Vec<BytePattern>, PatternStats) {
    // Parse AOBs from content
    let aobs = parse_aobs(content);

    if aobs.len() < 2 {
        panic!("Need at least 2 valid AOB instances");
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
        // Check if any expanded instance had a wildcard at this position
        let has_original_wildcard = aobs
            .iter()
            .any(|aob| aob.is_expanded && aob.wildcard_positions[byte_idx]);

        if has_original_wildcard {
            // For positions with original wildcards, analyze only non-expanded instances
            // to see if we can refine the wildcard
            let non_expanded_values: Vec<u8> = aobs
                .iter()
                .filter(|aob| !aob.is_expanded)
                .filter_map(|aob| aob.bytes[byte_idx])
                .collect();

            if non_expanded_values.is_empty() {
                // No non-expanded instances, keep wildcard
                result.push(BytePattern::Wildcard);
            } else {
                // Try to optimize based on new AOBs
                let pattern = optimize_byte(&non_expanded_values);
                result.push(pattern);
            }
        } else {
            // No original wildcard, analyze all instances
            let values: Vec<u8> = aobs.iter().filter_map(|aob| aob.bytes[byte_idx]).collect();
            let pattern = optimize_byte(&values);
            result.push(pattern);
        }
    }

    let stats = PatternStats::from_patterns(&result);
    (result, stats)
}

/// Parse AOB instances from file content
/// Supports both raw hex bytes and patterns with wildcards
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

        // Try to parse as pattern with wildcards first
        if let Some(pattern) = parse_pattern(cleaned) {
            // Expand pattern to instances
            let expanded = expand_pattern_to_instances(&pattern, i + 1);
            aobs.extend(expanded);
        } else {
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
    }

    aobs
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

/// Expand a pattern with wildcards to a single representative instance
/// Uses placeholder values (0x00) for wildcards to avoid combinatorial explosion
/// The original wildcard positions are tracked to force wildcards in final result
fn expand_pattern_to_instances(pattern: &[BytePattern], line_num: usize) -> Vec<AobInstance> {
    let mut bytes = Vec::with_capacity(pattern.len());
    let mut wildcard_positions = vec![false; pattern.len()];

    for (idx, &byte_pattern) in pattern.iter().enumerate() {
        match byte_pattern {
            BytePattern::Fixed(byte) => {
                bytes.push(Some(byte));
            }
            BytePattern::Wildcard => {
                wildcard_positions[idx] = true;
                bytes.push(Some(0x00)); // Placeholder
            }
            BytePattern::HighNibble(high) => {
                wildcard_positions[idx] = true;
                bytes.push(Some(high << 4)); // Use 0x00 for low nibble
            }
            BytePattern::LowNibble(low) => {
                wildcard_positions[idx] = true;
                bytes.push(Some(low)); // Use 0x00 for high nibble
            }
        }
    }

    vec![AobInstance {
        bytes,
        line_num,
        is_expanded: true,
        wildcard_positions,
    }]
}
