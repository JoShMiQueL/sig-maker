//! Sig-Maker - Multi-format signature/pattern converter and optimizer

use sig_maker_core::analyzer;
use sig_maker_core::formats;
use sig_maker_core::io::{self, Input};
use sig_maker_core::parse_aobs;
use std::io::IsTerminal;
use std::path::PathBuf;

mod cli;
mod converter;
mod output;

fn main() {
    // Parse command-line arguments
    let config = cli::Config::parse();

    // Auto-detect if we're in a pipe (not a TTY)
    let is_tty = std::io::stdout().is_terminal();
    let auto_quiet = !is_tty || config.quiet;

    // Check if we should read from stdin (input is "-")
    let use_stdin = config.input_file == "-";
    let input_source = &config.input_file;

    // Read input file or stdin
    let input = match Input::read_with_stdin(input_source, use_stdin) {
        Ok(i) => i,
        Err(e) => cli::error_exit(&e),
    };

    // If input doesn't come from a file (direct pattern), force SimplePattern
    let input_type = if !PathBuf::from(input_source).exists() && !use_stdin {
        io::InputType::SimplePattern
    } else {
        input.detect_type()
    };

    // Check-only mode: validate pattern without converting
    if config.check_only {
        let pattern_str = input
            .extract_pattern()
            .unwrap_or_else(|| input.content.trim().to_string());
        match formats::validate_pattern(&pattern_str) {
            Ok(_) => {
                if !auto_quiet {
                    println!("Pattern is valid: {}", pattern_str);
                }
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Pattern is invalid: {}", e);
                std::process::exit(1);
            }
        }
    }

    // Detect input type and process accordingly
    match input_type {
        io::InputType::MultipleAobs => {
            // Analyze multiple AOB instances
            let aobs = parse_aobs(&input.content);
            if aobs.len() < 2 {
                cli::error_exit("Need at least 2 valid AOB instances");
            }
            let (result, stats) = analyzer::analyze_aobs(&input.content);

            // Print header (only in TTY mode)
            if !auto_quiet {
                println!("Sig-Maker");
                println!();
                println!("Analyzing {} valid AOB instances:", aobs.len());
                for (i, aob) in aobs.iter().enumerate() {
                    println!("  [{}] {} bytes", i + 1, aob.bytes.len());
                }
                println!();
                println!("All AOBs have {} bytes", aobs[0].bytes.len());
                println!();
                println!("Analysis:");
                println!("  Fixed: {} bytes", stats.fixed_bytes());
                println!("  High nibble wildcards: {}", stats.high_nibble_wildcards());
                println!("  Low nibble wildcards: {}", stats.low_nibble_wildcards());
                println!("  Full wildcards: {}", stats.full_wildcards());
                println!();
            }

            // Show diff table (only in TTY mode and CE format)
            if !auto_quiet
                && config.output_file.is_none()
                && (config.to_format.is_none()
                    || config.to_format == Some(formats::Format::CheatEngine))
            {
                output::print_diff_table(&result, &aobs, aobs[0].bytes.len());
            }

            output::print_analysis_results(
                &result,
                &stats,
                config.to_format,
                &aobs,
                config.output_file.as_deref(),
                config.quiet,
                config.verbose,
            );
        }
        io::InputType::CodePattern | io::InputType::SimplePattern => {
            // Convert single pattern
            let pattern = input
                .extract_pattern()
                .unwrap_or_else(|| input.content.trim().to_string());
            converter::convert_pattern(
                &pattern,
                config.to_format,
                config.output_file.as_deref(),
                auto_quiet,
                config.verbose,
            );
        }
    }
}
