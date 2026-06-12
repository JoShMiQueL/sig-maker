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

/// Process input based on configuration
/// Returns Ok(()) on success, Err(message) on failure
pub fn process_input(config: &cli::Config) -> Result<(), String> {
    // Auto-detect if we're in a pipe (not a TTY)
    let is_tty = std::io::stdout().is_terminal();
    let auto_quiet = !is_tty || config.quiet;

    // Check if we should read from stdin (input is "-")
    let use_stdin = config.input_file == "-";
    let input_source = &config.input_file;

    // Read input file or stdin
    let input = Input::read_with_stdin(input_source, use_stdin)?;

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
                if !config.quiet {
                    println!("Pattern is valid: {}", pattern_str);
                }
                return Ok(());
            }
            Err(e) => {
                return Err(format!("Pattern is invalid: {}", e));
            }
        }
    }

    // Detect input type and process accordingly
    match input_type {
        io::InputType::MultipleAobs => {
            // Analyze multiple AOB instances
            let aobs = parse_aobs(&input.content);
            if aobs.len() < 2 {
                return Err("Need at least 2 valid AOB instances".to_string());
            }
            let result = analyzer::analyze_aobs(&input.content);

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

    Ok(())
}

fn main() {
    // Parse command-line arguments
    let config = cli::Config::parse();

    // Process input
    if let Err(e) = process_input(&config) {
        cli::error_exit(&e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_env() {
        unsafe {
            std::env::set_var("SIG_MAKER_TEST", "1");
        }
        unsafe {
            std::env::set_var("SIG_MAKER_NO_PAUSE", "1");
        }
    }

    fn teardown_test_env() {
        unsafe {
            std::env::remove_var("SIG_MAKER_TEST");
        }
        unsafe {
            std::env::remove_var("SIG_MAKER_NO_PAUSE");
        }
    }

    #[test]
    fn process_input_simple_pattern() {
        setup_test_env();
        let config = cli::Config {
            input_file: "AB ?? CD ?? EF".to_string(),
            to_format: Some(formats::Format::CheatEngine),
            output_file: None,
            verbose: false,
            quiet: true,
            check_only: false,
        };

        let result = process_input(&config);
        teardown_test_env();
        assert!(result.is_ok());
    }

    #[test]
    fn process_input_check_only_valid() {
        setup_test_env();
        let config = cli::Config {
            input_file: "AB ?? CD ?? EF".to_string(),
            to_format: None,
            output_file: None,
            verbose: false,
            quiet: true,
            check_only: true,
        };

        let result = process_input(&config);
        teardown_test_env();
        assert!(result.is_ok());
    }

    #[test]
    fn process_input_check_only_invalid() {
        setup_test_env();
        let config = cli::Config {
            input_file: "invalid pattern".to_string(),
            to_format: None,
            output_file: None,
            verbose: false,
            quiet: true,
            check_only: true,
        };

        let result = process_input(&config);
        teardown_test_env();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid"));
    }

    #[test]
    fn process_input_with_format() {
        setup_test_env();
        let config = cli::Config {
            input_file: "AB ?? CD".to_string(),
            to_format: Some(formats::Format::Rust),
            output_file: None,
            verbose: false,
            quiet: true,
            check_only: false,
        };

        let result = process_input(&config);
        teardown_test_env();
        assert!(result.is_ok());
    }

    #[test]
    fn process_input_verbose() {
        setup_test_env();
        let config = cli::Config {
            input_file: "AB ?? CD ?? EF".to_string(),
            to_format: Some(formats::Format::CheatEngine),
            output_file: None,
            verbose: true,
            quiet: false,
            check_only: false,
        };

        let result = process_input(&config);
        teardown_test_env();
        assert!(result.is_ok());
    }

    #[test]
    fn process_input_quiet() {
        setup_test_env();
        let config = cli::Config {
            input_file: "AB ?? CD ?? EF".to_string(),
            to_format: Some(formats::Format::CheatEngine),
            output_file: None,
            verbose: false,
            quiet: true,
            check_only: false,
        };

        let result = process_input(&config);
        teardown_test_env();
        assert!(result.is_ok());
    }

    #[test]
    fn process_input_empty() {
        setup_test_env();
        let config = cli::Config {
            input_file: "".to_string(),
            to_format: None,
            output_file: None,
            verbose: false,
            quiet: true,
            check_only: false,
        };

        let result = process_input(&config);
        teardown_test_env();
        // In test mode, empty input returns Ok() instead of exiting
        assert!(result.is_ok());
    }
}
