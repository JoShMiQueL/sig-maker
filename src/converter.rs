//! Pattern conversion - convert single pattern between formats

use crate::formats::{BytePattern, Format, format_pattern, parse_pattern};
use atty;
use colored::Colorize;

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
        None => crate::io::error_exit("Could not parse input pattern"),
    };

    let input_line = content.lines().next().unwrap_or("").trim();

    // Disable colors when writing to file or not in a terminal
    let use_colors = output_file.is_none() && atty::is(atty::Stream::Stdout);

    let output = if let Some(fmt) = to_format {
        // Single format requested
        format_single_format(&pattern, fmt, input_line, quiet, use_colors, verbose)
    } else {
        // Show all formats
        format_all_formats(&pattern, input_line, quiet, use_colors, verbose)
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
    use_colors: bool,
    verbose: bool,
) -> String {
    let mut output = String::new();

    if !quiet {
        if use_colors {
            output.push_str(&format!(
                "{}\n",
                "==============================================".cyan()
            ));
            output.push_str(&format!(
                "  {}\n",
                "Sig-Maker Pattern Converter".cyan().bold()
            ));
            output.push_str(&format!(
                "{}\n",
                "==============================================".cyan()
            ));
        } else {
            output.push_str("==============================================\n");
            output.push_str("  Sig-Maker Pattern Converter\n");
            output.push_str("==============================================\n");
        }
        output.push('\n');
        output.push_str(&format!("Input: {}\n", input_line));
        output.push_str(&format!("Length: {} bytes\n", pattern.len()));
        output.push('\n');
    }

    output.push_str(&format!("Output ({}):\n", fmt.name()));
    output.push_str(&format_pattern(pattern, fmt));
    output.push('\n');

    if verbose {
        let stats = crate::output::PatternStats::from_patterns(pattern);
        output.push_str(&format!("Entropy: {:.3} bits\n", stats.entropy()));
        output.push_str(&format!(
            "Compression ratio: {:.2}%\n",
            stats.compression_ratio() * 100.0
        ));
        output.push('\n');
    }

    if !quiet {
        output.push('\n');
        if use_colors {
            output.push_str(&format!(
                "{}\n",
                "==============================================".cyan()
            ));
        } else {
            output.push_str("==============================================\n");
        }
    }

    output
}

fn format_all_formats(
    pattern: &[BytePattern],
    input_line: &str,
    quiet: bool,
    use_colors: bool,
    verbose: bool,
) -> String {
    let mut output = String::new();

    if !quiet {
        if use_colors {
            output.push_str(&format!(
                "{}\n",
                "==============================================".cyan()
            ));
            output.push_str(&format!(
                "  {}\n",
                "Sig-Maker Pattern Converter".cyan().bold()
            ));
            output.push_str(&format!(
                "{}\n",
                "==============================================".cyan()
            ));
        } else {
            output.push_str("==============================================\n");
            output.push_str("  Sig-Maker Pattern Converter\n");
            output.push_str("==============================================\n");
        }
        output.push('\n');
        if !input_line.is_empty() {
            output.push_str(&format!("Input: {}\n", input_line));
        }
        output.push_str(&format!("Length: {} bytes\n", pattern.len()));
        output.push('\n');
        output.push_str("All Output Formats:\n");
        output.push('\n');
    }

    if verbose {
        let stats = crate::output::PatternStats::from_patterns(pattern);
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
        (Format::IdaPro, "IDA Pro"),
        (Format::X64dbg, "x64dbg"),
        (Format::Python, "Python"),
        (Format::Json, "JSON"),
    ];

    for (fmt, short_name) in formats {
        let pattern_str = format_pattern(pattern, fmt);

        if !pattern_str.contains('\n') && pattern_str.len() < 70 {
            // Single line output
            output.push_str(&format!(
                "{:12} {}\n",
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

    if !quiet {
        output.push('\n');
        if use_colors {
            output.push_str(&format!(
                "{}\n",
                "==============================================".cyan()
            ));
        } else {
            output.push_str("==============================================\n");
        }
    }

    output
}

/// Get pattern in a specific format (for programmatic use)
#[allow(dead_code)]
pub fn convert_to_format(pattern: &[BytePattern], format: Format) -> String {
    format_pattern(pattern, format)
}
