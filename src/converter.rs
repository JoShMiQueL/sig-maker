//! Pattern conversion - convert single pattern between formats

use crate::formats::{BytePattern, Format, format_pattern, parse_pattern};

/// Convert a single pattern and display results
pub fn convert_pattern(content: &str, to_format: Option<Format>) {
    let pattern = match parse_pattern(content) {
        Some(p) => p,
        None => crate::io::error_exit("Could not parse input pattern"),
    };

    let input_line = content.lines().next().unwrap_or("").trim();

    println!("==============================================");
    println!("  Sig-Maker Pattern Converter");
    println!("==============================================");
    println!();
    println!("Input: {}", input_line);
    println!("Length: {} bytes", pattern.len());
    println!();

    if let Some(fmt) = to_format {
        // Single format requested
        println!("Output ({}):", fmt.name());
        println!("{}", format_pattern(&pattern, fmt));
    } else {
        // Show all formats
        print_all_formats(&pattern);
    }

    println!();
    println!("==============================================");
}

/// Print pattern in all supported formats
fn print_all_formats(pattern: &[BytePattern]) {
    println!("All Output Formats:");
    println!();

    let formats = [
        (Format::CheatEngine, "CE"),
        (Format::Cpp, "C++"),
        (Format::Rust, "Rust"),
        (Format::Ghidra, "Ghidra"),
        (Format::IdaPro, "IDA Pro"),
        (Format::X64dbg, "x64dbg"),
        (Format::Python, "Python"),
        (Format::Json, "JSON"),
    ];

    for (fmt, short_name) in formats {
        let output = format_pattern(pattern, fmt);

        if !output.contains('\n') && output.len() < 70 {
            // Single line output
            println!("{:12} {}", format!("{}:", short_name), output);
        } else {
            // Multi-line output
            println!("{}:", short_name);
            for line in output.lines() {
                println!("  {}", line);
            }
            println!();
        }
    }
}

/// Get pattern in a specific format (for programmatic use)
#[allow(dead_code)]
pub fn convert_to_format(pattern: &[BytePattern], format: Format) -> String {
    format_pattern(pattern, format)
}
