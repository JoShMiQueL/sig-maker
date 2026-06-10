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
) {
    if let Some(fmt) = to_format {
        // Single format requested
        print_single_format_result(result, stats, fmt, aobs);
    } else {
        // Show all formats
        print_all_formats_result(result, stats, aobs);
    }

    println!();
    println!("==============================================");
    println!("  Analysis Complete!");
    println!("==============================================");
}

fn print_single_format_result(
    result: &[BytePattern],
    stats: &PatternStats,
    fmt: Format,
    aobs: &[AobInstance],
) {
    println!("==============================================");
    println!("  ANALYSIS RESULT ({})", fmt.name());
    println!("==============================================");
    println!("Length: {} bytes", stats.total_bytes());
    println!("Fixed: {} bytes", stats.fixed_bytes());
    println!(
        "High nibble wildcards: {} bytes",
        stats.high_nibble_wildcards()
    );
    println!(
        "Low nibble wildcards: {} bytes",
        stats.low_nibble_wildcards()
    );
    println!("Full wildcards: {} bytes", stats.full_wildcards());
    println!();
    println!("Optimized Pattern:");
    println!("{}", format_pattern(result, fmt));

    if fmt == Format::CheatEngine {
        print_verification(aobs, result);
    }
}

fn print_all_formats_result(result: &[BytePattern], stats: &PatternStats, aobs: &[AobInstance]) {
    println!("==============================================");
    println!("  ANALYSIS RESULT - All Formats");
    println!("==============================================");
    println!("Length: {} bytes", stats.total_bytes());
    println!("Fixed: {} bytes", stats.fixed_bytes());
    println!(
        "High nibble wildcards: {} bytes",
        stats.high_nibble_wildcards()
    );
    println!(
        "Low nibble wildcards: {} bytes",
        stats.low_nibble_wildcards()
    );
    println!("Full wildcards: {} bytes", stats.full_wildcards());
    println!();
    println!("Optimized Patterns:");
    println!();

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
        let output = format_pattern(result, fmt);
        if !output.contains('\n') && output.len() < 70 {
            println!("{:15} {}", format!("{}:", name), output);
        } else {
            println!("{}:", name);
            for line in output.lines() {
                println!("  {}", line);
            }
            println!();
        }
    }

    print_verification(aobs, result);
}

fn print_verification(aobs: &[AobInstance], result: &[BytePattern]) {
    println!();
    println!("Verification:");
    for (i, aob) in aobs.iter().enumerate() {
        let matches = aob_matches_pattern(&aob.bytes, result);
        let status = if matches { "✓" } else { "✗" };
        println!(
            "  {} [{}]: {}",
            status,
            i + 1,
            if matches { "matches" } else { "MISMATCH" }
        );
    }
}
