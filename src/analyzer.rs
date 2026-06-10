//! AOB analysis - compare multiple instances and find optimal pattern

use crate::formats::{BytePattern, Format, optimize_byte};
use crate::output::{PatternStats, print_analysis_results, print_diff_table};

/// AOB instance with source line info
pub struct AobInstance {
    pub bytes: Vec<Option<u8>>,
    #[allow(dead_code)]
    pub line_num: usize,
}

/// Analyze multiple AOB instances and generate optimized pattern
pub fn analyze_aobs(content: &str, to_format: Option<Format>) {
    println!("==============================================");
    println!("  Sig-Maker");
    println!("==============================================");
    println!();

    // Parse AOBs from content
    let aobs = parse_aobs(content);

    if aobs.len() < 2 {
        crate::io::error_exit("Need at least 2 valid AOB instances");
    }

    println!("Analyzing {} valid AOB instances:", aobs.len());
    for (i, aob) in aobs.iter().enumerate() {
        println!("  [{}] {} bytes", i + 1, aob.bytes.len());
    }
    println!();

    // Verify all have same length
    let first_len = aobs[0].bytes.len();
    for (i, aob) in aobs.iter().enumerate() {
        if aob.bytes.len() != first_len {
            crate::io::error_exit(&format!(
                "AOB {} has {} bytes, expected {}",
                i + 1,
                aob.bytes.len(),
                first_len
            ));
        }
    }

    println!("All AOBs have {} bytes", first_len);
    println!();

    // Analyze byte-by-byte
    println!("[1/2] Comparing byte-by-byte...");
    let mut result: Vec<BytePattern> = Vec::with_capacity(first_len);

    for byte_idx in 0..first_len {
        let values: Vec<u8> = aobs.iter().filter_map(|aob| aob.bytes[byte_idx]).collect();
        let pattern = optimize_byte(&values);
        result.push(pattern);
    }

    let stats = PatternStats::from_patterns(&result);

    // Print stats
    println!("    Fixed bytes: {}", stats.fixed_bytes());
    println!(
        "    High nibble wildcards: {}",
        stats.high_nibble_wildcards()
    );
    println!("    Low nibble wildcards: {}", stats.low_nibble_wildcards());
    println!("    Full wildcards: {}", stats.full_wildcards());
    println!();

    // Show diff table (only for CE output or when showing all)
    if to_format.is_none() || to_format == Some(Format::CheatEngine) {
        print_diff_table(&result, &aobs, first_len);
    }

    // Print results
    print_analysis_results(&result, &stats, to_format, &aobs);
}

/// Parse AOB instances from file content
fn parse_aobs(content: &str) -> Vec<AobInstance> {
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
            aobs.push(AobInstance {
                bytes,
                line_num: i + 1,
            });
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
