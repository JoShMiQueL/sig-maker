//! AOB analysis - compare multiple instances and find optimal pattern

use crate::formats::{BytePattern, Format, optimize_byte, parse_pattern};
use crate::output::{PatternStats, print_analysis_results, print_diff_table};
use atty;
use colored::Colorize;

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

/// Analyze multiple AOB instances and generate optimized pattern
pub fn analyze_aobs(
    content: &str,
    to_format: Option<Format>,
    output_file: Option<&str>,
    verbose: bool,
    quiet: bool,
) {
    // Disable colors when writing to file or not in a terminal
    let use_colors = output_file.is_none() && atty::is(atty::Stream::Stdout);

    if !quiet {
        if use_colors {
            println!(
                "{}",
                "==============================================".cyan()
            );
            println!("  {}", "Sig-Maker".cyan().bold());
            println!(
                "{}",
                "==============================================".cyan()
            );
        } else {
            println!("==============================================");
            println!("  Sig-Maker");
            println!("==============================================");
        }
        println!();
    }

    // Parse AOBs from content
    let aobs = parse_aobs(content);

    if aobs.len() < 2 {
        crate::io::error_exit("Need at least 2 valid AOB instances");
    }

    if !quiet {
        if use_colors {
            println!(
                "Analyzing {} valid AOB instances:",
                aobs.len().to_string().green()
            );
            for (i, aob) in aobs.iter().enumerate() {
                println!(
                    "  [{}] {} bytes",
                    (i + 1).to_string().cyan(),
                    aob.bytes.len()
                );
            }
        } else {
            println!("Analyzing {} valid AOB instances:", aobs.len());
            for (i, aob) in aobs.iter().enumerate() {
                println!("  [{}] {} bytes", i + 1, aob.bytes.len());
            }
        }
        println!();
    }

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

    if !quiet {
        if use_colors {
            println!("All AOBs have {} bytes", first_len.to_string().green());
        } else {
            println!("All AOBs have {} bytes", first_len);
        }
        println!();
    }

    // Analyze byte-by-byte
    if !quiet {
        if use_colors {
            println!("{} Comparing byte-by-byte...", "[1/2]".yellow());
        } else {
            println!("[1/2] Comparing byte-by-byte...");
        }
    }
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

    // Print stats
    if !quiet {
        if use_colors {
            println!(
                "    Fixed bytes: {}",
                stats.fixed_bytes().to_string().green()
            );
            println!(
                "    High nibble wildcards: {}",
                stats.high_nibble_wildcards().to_string().yellow()
            );
            println!(
                "    Low nibble wildcards: {}",
                stats.low_nibble_wildcards().to_string().yellow()
            );
            println!(
                "    Full wildcards: {}",
                stats.full_wildcards().to_string().red()
            );
        } else {
            println!("    Fixed bytes: {}", stats.fixed_bytes());
            println!(
                "    High nibble wildcards: {}",
                stats.high_nibble_wildcards()
            );
            println!("    Low nibble wildcards: {}", stats.low_nibble_wildcards());
            println!("    Full wildcards: {}", stats.full_wildcards());
        }
        println!();
    }

    // Print detailed stats in verbose mode
    if verbose {
        if use_colors {
            println!(
                "    Entropy: {:.3} bits",
                stats.entropy().to_string().cyan()
            );
            println!(
                "    Compression ratio: {:.2}%",
                (stats.compression_ratio() * 100.0).to_string().green()
            );
        } else {
            println!("    Entropy: {:.3} bits", stats.entropy());
            println!(
                "    Compression ratio: {:.2}%",
                stats.compression_ratio() * 100.0
            );
        }
        println!();
    }

    // Show diff table (only for CE output or when showing all, and not writing to file, and not quiet)
    if !quiet
        && output_file.is_none()
        && (to_format.is_none() || to_format == Some(Format::CheatEngine))
    {
        print_diff_table(&result, &aobs, first_len);
    }

    // Print results
    print_analysis_results(
        &result,
        &stats,
        to_format,
        &aobs,
        output_file,
        quiet,
        verbose,
    );
}

/// Parse AOB instances from file content
/// Supports both raw hex bytes and patterns with wildcards
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
