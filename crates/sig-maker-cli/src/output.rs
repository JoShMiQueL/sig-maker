//! Output formatting and printing

use sig_maker_core::analyzer::{AobInstance, aob_matches_pattern};
use sig_maker_core::formats::{BytePattern, Format, format_pattern};
use std::collections::HashSet;

/// Print the diff table showing byte-by-byte comparison
pub fn print_diff_table(result: &[BytePattern], aobs: &[AobInstance], _first_len: usize) {
    println!("Difference Analysis:");
    println!();
    println!("Byte  Pattern         Values");
    println!("----  --------         ------");

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
                    format!(" -> {}", unique[0])
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
                    format!(" -> {}", unique[0])
                } else {
                    String::new()
                }
            }
            BytePattern::Wildcard => "*".to_string(),
        };

        let pattern_str = format_pattern(&[*pattern], Format::CheatEngine);

        println!(
            "[{:2}]  {:15} {}{}",
            byte_idx, pattern_str, values_str, marker
        );
    }
    println!();
}

/// Print final analysis results
pub fn print_analysis_results(
    result: &[BytePattern],
    to_format: Option<Format>,
    aobs: &[AobInstance],
    output_file: Option<&str>,
    quiet: bool,
    verbose: bool,
) {
    let output = if let Some(fmt) = to_format {
        // Single format requested
        format_single_format_result(result, fmt, aobs, quiet, verbose)
    } else {
        // Show all formats
        format_all_formats_result(result, aobs, quiet, verbose)
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
    fmt: Format,
    aobs: &[AobInstance],
    quiet: bool,
    verbose: bool,
) -> String {
    let mut output = String::new();

    if !quiet {
        output.push_str(&format!("Sig-Maker: Analysis ({})\n", fmt.name()));
        output.push_str(&format!("Length: {} bytes\n", result.len()));
        output.push('\n');
    }

    if verbose {
        output.push_str(&format_verification(aobs, result));
    }

    output.push_str("Optimized Pattern:\n");
    output.push_str(&format_pattern(result, fmt));
    output.push('\n');

    output
}

fn format_all_formats_result(
    result: &[BytePattern],
    aobs: &[AobInstance],
    quiet: bool,
    verbose: bool,
) -> String {
    let mut output = String::new();

    if !quiet {
        output.push_str("Sig-Maker: Analysis (All Formats)\n");
        output.push_str(&format!("Length: {} bytes\n", result.len()));
        output.push('\n');
    }

    if verbose {
        output.push_str(&format_verification(aobs, result));
    }

    output.push_str("Optimized Patterns:\n");
    output.push('\n');

    let formats = [
        (Format::CheatEngine, "CE"),
        (Format::Cpp, "C++"),
        (Format::Rust, "Rust"),
        (Format::Ghidra, "Ghidra"),
        (Format::IdaPro, "IDA"),
        (Format::X64dbg, "x64dbg"),
        (Format::Python, "Python"),
        (Format::Json, "JSON"),
    ];

    for (fmt, name) in formats {
        let pattern = format_pattern(result, fmt);
        if !pattern.contains('\n') && pattern.len() < 70 {
            output.push_str(&format!("{:12} {}\n", format!("{}:", name), pattern));
        } else {
            output.push_str(&format!("{}:\n", name));
            for line in pattern.lines() {
                output.push_str(&format!("  {}\n", line));
            }
            output.push('\n');
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_aobs() -> Vec<AobInstance> {
        vec![
            AobInstance {
                bytes: vec![Some(0xAB), Some(0xCD), Some(0xEF)],
                line_num: 1,
                is_expanded: false,
                wildcard_positions: vec![false; 3],
            },
            AobInstance {
                bytes: vec![Some(0xAB), Some(0x00), Some(0xEF)],
                line_num: 2,
                is_expanded: false,
                wildcard_positions: vec![false; 3],
            },
        ]
    }

    #[test]
    fn format_single_format_result_with_header() {
        let result = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Wildcard,
            BytePattern::Fixed(0xEF),
        ];
        let aobs = create_test_aobs();

        let output = format_single_format_result(&result, Format::CheatEngine, &aobs, false, false);
        assert!(output.contains("Sig-Maker: Analysis (Cheat Engine)"));
        assert!(output.contains("Length: 3 bytes"));
        assert!(output.contains("Optimized Pattern:"));
    }

    #[test]
    fn format_single_format_result_quiet() {
        let result = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Wildcard,
            BytePattern::Fixed(0xEF),
        ];
        let aobs = create_test_aobs();

        let output = format_single_format_result(&result, Format::CheatEngine, &aobs, true, false);
        assert!(!output.contains("Sig-Maker"));
        assert!(!output.contains("Length:"));
        assert!(output.contains("Optimized Pattern:"));
    }

    #[test]
    fn format_single_format_result_with_verification() {
        let result = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Wildcard,
            BytePattern::Fixed(0xEF),
        ];
        let aobs = create_test_aobs();

        let output = format_single_format_result(&result, Format::CheatEngine, &aobs, false, true);
        assert!(output.contains("Verification:"));
        assert!(output.contains("[1]:"));
        assert!(output.contains("[2]:"));
    }

    #[test]
    fn format_all_formats_result_with_header() {
        let result = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Wildcard,
            BytePattern::Fixed(0xEF),
        ];
        let aobs = create_test_aobs();

        let output = format_all_formats_result(&result, &aobs, false, false);
        assert!(output.contains("Sig-Maker: Analysis (All Formats)"));
        assert!(output.contains("Length: 3 bytes"));
        assert!(output.contains("Optimized Patterns:"));
    }

    #[test]
    fn format_all_formats_result_quiet() {
        let result = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Wildcard,
            BytePattern::Fixed(0xEF),
        ];
        let aobs = create_test_aobs();

        let output = format_all_formats_result(&result, &aobs, true, false);
        assert!(!output.contains("Sig-Maker"));
        assert!(!output.contains("Length:"));
        assert!(output.contains("Optimized Patterns:"));
    }

    #[test]
    fn format_all_formats_result_verbose() {
        let result = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Wildcard,
            BytePattern::Fixed(0xEF),
        ];
        let aobs = create_test_aobs();

        let output = format_all_formats_result(&result, &aobs, false, true);
        assert!(output.contains("Verification:"));
    }

    #[test]
    fn format_all_formats_result_includes_all_formats() {
        let result = vec![BytePattern::Fixed(0xAB), BytePattern::Fixed(0xCD)];
        let aobs = create_test_aobs();

        let output = format_all_formats_result(&result, &aobs, false, false);
        assert!(output.contains("CE:"));
        assert!(output.contains("C++:"));
        assert!(output.contains("Rust:"));
        assert!(output.contains("Ghidra:"));
        assert!(output.contains("IDA:"));
        assert!(output.contains("x64dbg:"));
        assert!(output.contains("Python:"));
        assert!(output.contains("JSON:"));
    }

    #[test]
    fn format_verification_all_match() {
        let result = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Wildcard,
            BytePattern::Fixed(0xEF),
        ];
        let aobs = create_test_aobs();

        let output = format_verification(&aobs, &result);
        assert!(output.contains("Verification:"));
        assert!(output.contains("✓"));
        assert!(!output.contains("✗"));
    }

    #[test]
    fn format_verification_with_mismatch() {
        let result = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Fixed(0xCD), // Won't match second AOB
            BytePattern::Fixed(0xEF),
        ];
        let aobs = create_test_aobs();

        let output = format_verification(&aobs, &result);
        assert!(output.contains("Verification:"));
        assert!(output.contains("✓"));
        assert!(output.contains("✗"));
        assert!(output.contains("MISMATCH"));
    }

    #[test]
    fn format_verification_empty_aobs() {
        let result = vec![BytePattern::Fixed(0xAB)];
        let aobs: Vec<AobInstance> = vec![];

        let output = format_verification(&aobs, &result);
        assert!(output.contains("Verification:"));
        // Should not have any match indicators
        assert!(!output.contains("✓"));
        assert!(!output.contains("✗"));
    }

    #[test]
    fn format_single_format_result_cpp() {
        let result = vec![BytePattern::Fixed(0xAB), BytePattern::Fixed(0xCD)];
        let aobs = create_test_aobs();

        let output = format_single_format_result(&result, Format::Cpp, &aobs, false, false);
        assert!(output.contains("Sig-Maker: Analysis (C++)"));
        assert!(output.contains("const uint8_t pattern[]"));
    }

    #[test]
    fn format_single_format_result_rust() {
        let result = vec![BytePattern::Fixed(0xAB), BytePattern::Fixed(0xCD)];
        let aobs = create_test_aobs();

        let output = format_single_format_result(&result, Format::Rust, &aobs, false, false);
        assert!(output.contains("Sig-Maker: Analysis (Rust)"));
        assert!(output.contains("static PATTERN: [u8; 2]"));
    }

    #[test]
    fn format_single_format_result_json() {
        let result = vec![BytePattern::Fixed(0xAB), BytePattern::Fixed(0xCD)];
        let aobs = create_test_aobs();

        let output = format_single_format_result(&result, Format::Json, &aobs, false, false);
        assert!(output.contains("Sig-Maker: Analysis (JSON)"));
        assert!(output.contains("\"pattern\""));
    }

    #[test]
    fn format_single_format_result_empty_pattern() {
        let result = vec![];
        let aobs = create_test_aobs();

        let output = format_single_format_result(&result, Format::CheatEngine, &aobs, false, false);
        assert!(output.contains("Length: 0 bytes"));
    }

    #[test]
    fn format_all_formats_result_empty_pattern() {
        let result = vec![];
        let aobs = create_test_aobs();

        let output = format_all_formats_result(&result, &aobs, false, false);
        assert!(output.contains("Length: 0 bytes"));
    }
}
