//! Pattern conversion - convert single pattern between formats

use sig_maker_core::PatternStats;
use sig_maker_core::formats::{BytePattern, Format, format_pattern, parse_pattern};

/// Convert a single pattern and display results
pub fn convert_pattern(
    content: &str,
    to_format: Option<Format>,
    output_file: Option<&str>,
    quiet: bool,
    verbose: bool,
) {
    let pattern = match parse_pattern(content) {
        Some(p) => p,
        None => crate::cli::error_exit("Could not parse input pattern"),
    };

    let input_line = content.lines().next().unwrap_or("").trim();

    let output = if let Some(fmt) = to_format {
        // Single format requested
        format_single_format(&pattern, fmt, input_line, quiet, verbose)
    } else {
        // Show all formats
        format_all_formats(&pattern, input_line, quiet, verbose)
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

fn format_single_format(
    pattern: &[BytePattern],
    fmt: Format,
    input_line: &str,
    quiet: bool,
    verbose: bool,
) -> String {
    let mut output = String::new();

    if !quiet {
        output.push_str(&format!("Sig-Maker: {}\n", fmt.name()));
        output.push_str(&format!("Input: {}\n", input_line));
        output.push_str(&format!("Length: {} bytes\n", pattern.len()));
        output.push('\n');
    }

    output.push_str(&format_pattern(pattern, fmt));
    output.push('\n');

    if verbose {
        let stats = PatternStats::from_patterns(pattern);
        output.push_str(&format!("Entropy: {:.3} bits\n", stats.entropy()));
        output.push_str(&format!(
            "Compression ratio: {:.2}%\n",
            stats.compression_ratio() * 100.0
        ));
    }

    output
}

fn format_all_formats(
    pattern: &[BytePattern],
    input_line: &str,
    quiet: bool,
    verbose: bool,
) -> String {
    let mut output = String::new();

    if !quiet {
        output.push_str("Sig-Maker: All Formats\n");
        output.push_str(&format!("Input: {}\n", input_line));
        output.push_str(&format!("Length: {} bytes\n", pattern.len()));
        output.push('\n');
    }

    if verbose {
        let stats = PatternStats::from_patterns(pattern);
        output.push_str(&format!("Entropy: {:.3} bits\n", stats.entropy()));
        output.push_str(&format!(
            "Compression ratio: {:.2}%\n",
            stats.compression_ratio() * 100.0
        ));
        output.push('\n');
    }

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

    for (fmt, short_name) in formats {
        let pattern_str = format_pattern(pattern, fmt);

        if !pattern_str.contains('\n') && pattern_str.len() < 70 {
            // Single line output
            output.push_str(&format!(
                "{:8} {}\n",
                format!("{}:", short_name),
                pattern_str
            ));
        } else {
            // Multi-line output
            output.push_str(&format!("{}:\n", short_name));
            for line in pattern_str.lines() {
                output.push_str(&format!("  {}\n", line));
            }
            output.push('\n');
        }
    }

    output
}
