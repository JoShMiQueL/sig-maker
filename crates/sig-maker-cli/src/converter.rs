//! Pattern conversion - convert single pattern between formats

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
        None => {
            // In tests, return early instead of exiting
            if std::env::var("SIG_MAKER_TEST").is_ok() {
                eprintln!("ERROR: Could not parse input pattern");
                return;
            }
            crate::cli::error_exit("Could not parse input pattern")
        }
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
    _verbose: bool,
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

    output
}

fn format_all_formats(
    pattern: &[BytePattern],
    input_line: &str,
    quiet: bool,
    _verbose: bool,
) -> String {
    let mut output = String::new();

    if !quiet {
        output.push_str("Sig-Maker: All Formats\n");
        output.push_str(&format!("Input: {}\n", input_line));
        output.push_str(&format!("Length: {} bytes\n", pattern.len()));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_single_format_with_header() {
        let pattern = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Wildcard,
            BytePattern::Fixed(0xCD),
        ];
        let result = format_single_format(&pattern, Format::CheatEngine, "AB ?? CD", false, false);
        assert!(result.contains("Sig-Maker: Cheat Engine"));
        assert!(result.contains("Input: AB ?? CD"));
        assert!(result.contains("Length: 3 bytes"));
        assert!(result.contains("AB ?? CD"));
    }

    #[test]
    fn format_single_format_quiet() {
        let pattern = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Wildcard,
            BytePattern::Fixed(0xCD),
        ];
        let result = format_single_format(&pattern, Format::CheatEngine, "AB ?? CD", true, false);
        assert!(!result.contains("Sig-Maker"));
        assert!(!result.contains("Input:"));
        assert!(!result.contains("Length:"));
        assert!(result.contains("AB ?? CD"));
    }

    #[test]
    fn format_all_formats_with_header() {
        let pattern = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Wildcard,
            BytePattern::Fixed(0xCD),
        ];
        let result = format_all_formats(&pattern, "AB ?? CD", false, false);
        assert!(result.contains("Sig-Maker: All Formats"));
        assert!(result.contains("Input: AB ?? CD"));
        assert!(result.contains("Length: 3 bytes"));
        assert!(result.contains("CE:"));
        assert!(result.contains("C++:"));
        assert!(result.contains("Rust:"));
    }

    #[test]
    fn format_all_formats_quiet() {
        let pattern = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Wildcard,
            BytePattern::Fixed(0xCD),
        ];
        let result = format_all_formats(&pattern, "AB ?? CD", true, false);
        assert!(!result.contains("Sig-Maker"));
        assert!(!result.contains("Input:"));
        assert!(!result.contains("Length:"));
        assert!(result.contains("CE:"));
        assert!(result.contains("C++:"));
    }

    #[test]
    fn format_all_formats_includes_all_formats() {
        let pattern = vec![BytePattern::Fixed(0xAB), BytePattern::Fixed(0xCD)];
        let result = format_all_formats(&pattern, "AB CD", false, false);
        assert!(result.contains("CE:"));
        assert!(result.contains("C++:"));
        assert!(result.contains("Rust:"));
        assert!(result.contains("Ghidra:"));
        assert!(result.contains("IDA:"));
        assert!(result.contains("x64dbg:"));
        assert!(result.contains("Python:"));
        assert!(result.contains("JSON:"));
    }

    #[test]
    fn format_single_format_cpp() {
        let pattern = vec![BytePattern::Fixed(0xAB), BytePattern::Fixed(0xCD)];
        let result = format_single_format(&pattern, Format::Cpp, "AB CD", false, false);
        assert!(result.contains("Sig-Maker: C++"));
        assert!(result.contains("const uint8_t pattern[]"));
    }

    #[test]
    fn format_single_format_rust() {
        let pattern = vec![BytePattern::Fixed(0xAB), BytePattern::Fixed(0xCD)];
        let result = format_single_format(&pattern, Format::Rust, "AB CD", false, false);
        assert!(result.contains("Sig-Maker: Rust"));
        assert!(result.contains("static PATTERN: [u8; 2]"));
    }

    #[test]
    fn format_single_format_json() {
        let pattern = vec![BytePattern::Fixed(0xAB), BytePattern::Fixed(0xCD)];
        let result = format_single_format(&pattern, Format::Json, "AB CD", false, false);
        assert!(result.contains("Sig-Maker: JSON"));
        assert!(result.contains("\"pattern\""));
    }

    #[test]
    fn format_single_format_python() {
        let pattern = vec![BytePattern::Fixed(0xAB), BytePattern::Fixed(0xCD)];
        let result = format_single_format(&pattern, Format::Python, "AB CD", false, false);
        assert!(result.contains("Sig-Maker: Python"));
        assert!(result.contains("import re"));
    }

    #[test]
    fn format_single_format_ghidra() {
        let pattern = vec![BytePattern::Fixed(0xAB), BytePattern::Fixed(0xCD)];
        let result = format_single_format(&pattern, Format::Ghidra, "AB CD", false, false);
        assert!(result.contains("Sig-Maker: Ghidra"));
        assert!(result.contains("AB CD"));
    }

    #[test]
    fn format_single_format_ida() {
        let pattern = vec![BytePattern::Fixed(0xAB), BytePattern::Fixed(0xCD)];
        let result = format_single_format(&pattern, Format::IdaPro, "AB CD", false, false);
        assert!(result.contains("Sig-Maker: IDA Pro"));
        assert!(result.contains("AB CD"));
    }

    #[test]
    fn format_single_format_x64dbg() {
        let pattern = vec![BytePattern::Fixed(0xAB), BytePattern::Fixed(0xCD)];
        let result = format_single_format(&pattern, Format::X64dbg, "AB CD", false, false);
        assert!(result.contains("Sig-Maker: x64dbg"));
        assert!(result.contains("AB CD"));
    }

    #[test]
    fn format_all_formats_empty_pattern() {
        let pattern = vec![];
        let result = format_all_formats(&pattern, "", false, false);
        assert!(result.contains("Length: 0 bytes"));
    }

    #[test]
    fn format_all_formats_long_pattern() {
        let pattern: Vec<BytePattern> = vec![BytePattern::Fixed(0xAB); 20];
        let result = format_all_formats(&pattern, &"AB ".repeat(20), false, false);
        assert!(result.contains("Length: 20 bytes"));
    }
}
