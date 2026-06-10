//! Output formatting and printing

use crate::analyzer::{AobInstance, aob_matches_pattern};
use crate::formats::{BytePattern, Format, format_pattern};

/// Statistics about pattern optimization
pub struct PatternStats {
    fixed: usize,
    high_nibble: usize,
    low_nibble: usize,
    wildcard: usize,
}

impl PatternStats {
    pub fn from_patterns(patterns: &[BytePattern]) -> Self {
        let mut stats = Self {
            fixed: 0,
            high_nibble: 0,
            low_nibble: 0,
            wildcard: 0,
        };

        for p in patterns {
            match p {
                BytePattern::Fixed(_) => stats.fixed += 1,
                BytePattern::Wildcard => stats.wildcard += 1,
                BytePattern::HighNibble(_) => stats.high_nibble += 1,
                BytePattern::LowNibble(_) => stats.low_nibble += 1,
            }
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
}

/// Print the diff table showing byte-by-byte comparison
pub fn print_diff_table(result: &[BytePattern], aobs: &[AobInstance], _first_len: usize) {
    use std::collections::HashSet;

    println!("[2/2] Difference Analysis:");
    println!();
    println!("{:-<90}", "");
    println!("{:<5} {:<15} Values", "Byte", "Pattern");
    println!("{:-<90}", "");

    for (byte_idx, pattern) in result.iter().enumerate() {
        // Show values from each AOB
        let values_str: String = aobs
            .iter()
            .map(|aob| match aob.bytes[byte_idx] {
                Some(v) => format!("{:02X}", v),
                None => "??".to_string(),
            })
            .collect::<Vec<_>>()
            .join(" ");

        // Show marker if optimized
        let marker: String = match *pattern {
            BytePattern::Fixed(_) => String::new(),
            BytePattern::HighNibble(_) => {
                let vals: Vec<String> = aobs
                    .iter()
                    .filter_map(|aob| aob.bytes[byte_idx])
                    .map(|v| format!("{:X}?", v >> 4))
                    .collect();
                let unique: Vec<String> = vals
                    .into_iter()
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .collect();
                if unique.len() == 1 {
                    format!(" <-- optimized to {}", unique[0])
                } else {
                    String::new()
                }
            }
            BytePattern::LowNibble(_) => {
                let vals: Vec<String> = aobs
                    .iter()
                    .filter_map(|aob| aob.bytes[byte_idx])
                    .map(|v| format!("?{:X}", v & 0x0F))
                    .collect();
                let unique: Vec<String> = vals
                    .into_iter()
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .collect();
                if unique.len() == 1 {
                    format!(" <-- optimized to {}", unique[0])
                } else {
                    String::new()
                }
            }
            BytePattern::Wildcard => " <-- CHANGES".to_string(),
        };

        let pattern_str = format_pattern(&[*pattern], Format::CheatEngine);

        println!(
            "{:<5} {:<15} {}{}",
            format!("[{}]", byte_idx),
            pattern_str,
            values_str,
            marker
        );
    }
    println!("{:-<90}", "");
    println!();
}

/// Print final analysis results
pub fn print_analysis_results(
    result: &[BytePattern],
    stats: &PatternStats,
    to_format: Option<Format>,
    aobs: &[AobInstance],
    output_file: Option<&str>,
    quiet: bool,
    verbose: bool,
) {
    let output = if let Some(fmt) = to_format {
        // Single format requested
        format_single_format_result(result, stats, fmt, aobs, quiet, verbose)
    } else {
        // Show all formats
        format_all_formats_result(result, stats, aobs, quiet, verbose)
    };

    if let Some(file) = output_file {
        // Write to file
        if let Err(e) = std::fs::write(file, &output) {
            eprintln!("ERROR: Failed to write to '{}': {}", file, e);
            std::process::exit(1);
        }
        if !quiet {
            println!("Output written to: {}", file);
        }
    } else {
        // Print to stdout
        print!("{}", output);
    }
}

fn format_single_format_result(
    result: &[BytePattern],
    stats: &PatternStats,
    fmt: Format,
    aobs: &[AobInstance],
    quiet: bool,
    verbose: bool,
) -> String {
    let mut output = String::new();

    if !quiet {
        output.push_str("==============================================\n");
        output.push_str(&format!("  ANALYSIS RESULT ({})\n", fmt.name()));
        output.push_str("==============================================\n");
        output.push_str(&format!("Length: {} bytes\n", stats.total_bytes()));
        output.push_str(&format!("Fixed: {} bytes\n", stats.fixed_bytes()));
        output.push_str(&format!(
            "High nibble wildcards: {} bytes\n",
            stats.high_nibble_wildcards()
        ));
        output.push_str(&format!(
            "Low nibble wildcards: {} bytes\n",
            stats.low_nibble_wildcards()
        ));
        output.push_str(&format!(
            "Full wildcards: {} bytes\n",
            stats.full_wildcards()
        ));
        output.push('\n');
    }

    output.push_str("Optimized Pattern:\n");
    output.push_str(&format_pattern(result, fmt));
    output.push('\n');

    if verbose && fmt == Format::CheatEngine {
        output.push_str(&format_verification(aobs, result));
    }

    if !quiet {
        output.push('\n');
        output.push_str("==============================================\n");
        output.push_str("  Analysis Complete!\n");
        output.push_str("==============================================\n");
    }

    output
}

fn format_all_formats_result(
    result: &[BytePattern],
    stats: &PatternStats,
    aobs: &[AobInstance],
    quiet: bool,
    verbose: bool,
) -> String {
    let mut output = String::new();

    if !quiet {
        output.push_str("==============================================\n");
        output.push_str("  ANALYSIS RESULT - All Formats\n");
        output.push_str("==============================================\n");
        output.push_str(&format!("Length: {} bytes\n", stats.total_bytes()));
        output.push_str(&format!("Fixed: {} bytes\n", stats.fixed_bytes()));
        output.push_str(&format!(
            "High nibble wildcards: {} bytes\n",
            stats.high_nibble_wildcards()
        ));
        output.push_str(&format!(
            "Low nibble wildcards: {} bytes\n",
            stats.low_nibble_wildcards()
        ));
        output.push_str(&format!(
            "Full wildcards: {} bytes\n",
            stats.full_wildcards()
        ));
        output.push('\n');
    }

    output.push_str("Optimized Patterns:\n");
    output.push('\n');

    let formats = [
        (Format::CheatEngine, "Cheat Engine"),
        (Format::Cpp, "C++"),
        (Format::Rust, "Rust"),
        (Format::Ghidra, "Ghidra"),
        (Format::IdaPro, "IDA Pro"),
        (Format::X64dbg, "x64dbg"),
        (Format::Python, "Python"),
        (Format::Json, "JSON"),
    ];

    for (fmt, name) in formats {
        let pattern = format_pattern(result, fmt);
        if !pattern.contains('\n') && pattern.len() < 70 {
            output.push_str(&format!("{:15} {}\n", format!("{}:", name), pattern));
        } else {
            output.push_str(&format!("{}:\n", name));
            for line in pattern.lines() {
                output.push_str(&format!("  {}\n", line));
            }
            output.push('\n');
        }
    }

    if verbose {
        output.push_str(&format_verification(aobs, result));
    }

    if !quiet {
        output.push('\n');
        output.push_str("==============================================\n");
        output.push_str("  Analysis Complete!\n");
        output.push_str("==============================================\n");
    }

    output
}

fn format_verification(aobs: &[AobInstance], result: &[BytePattern]) -> String {
    let mut output = String::new();
    output.push('\n');
    output.push_str("Verification:\n");
    for (i, aob) in aobs.iter().enumerate() {
        let matches = aob_matches_pattern(&aob.bytes, result);
        let status = if matches { "✓" } else { "✗" };
        output.push_str(&format!(
            "  {} [{}]: {}\n",
            status,
            i + 1,
            if matches { "matches" } else { "MISMATCH" }
        ));
    }
    output
}
