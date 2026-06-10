//! Sig-Maker - Multi-format signature/pattern converter and optimizer

use sig_maker::analyzer;
use sig_maker::cli::Config;
use sig_maker::converter;
use sig_maker::formats;
use sig_maker::io::{self, Input};

fn main() {
    // Parse command-line arguments
    let config = Config::parse();

    // Read input file
    let input = match Input::read(&config.input_file) {
        Ok(i) => i,
        Err(e) => io::error_exit(&e),
    };

    // Check-only mode: validate pattern without converting
    if config.check_only {
        let pattern_str = input.content.lines().next().unwrap_or("").trim();
        match formats::validate_pattern(pattern_str) {
            Ok(_) => {
                if !config.quiet {
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
    match input.detect_type() {
        io::InputType::MultipleAobs => {
            // Analyze multiple AOB instances
            analyzer::analyze_aobs(
                &input.content,
                config.to_format,
                config.output_file.as_deref(),
                config.verbose,
                config.quiet,
            );
        }
        io::InputType::CodePattern | io::InputType::SimplePattern => {
            // Convert single pattern
            let pattern = input
                .extract_pattern()
                .unwrap_or_else(|| io::error_exit("No valid pattern found"));
            converter::convert_pattern(
                &pattern,
                config.to_format,
                config.output_file.as_deref(),
                config.quiet,
                config.verbose,
            );
        }
    }
}
