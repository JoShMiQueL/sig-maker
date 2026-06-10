//! Sig-Maker - Multi-format signature/pattern converter and optimizer

use atty::is;
use colored::Colorize;
use sig_maker_core::analyzer;
use sig_maker_core::formats;
use sig_maker_core::io::{self, Input};
use sig_maker_core::parse_aobs;

mod cli;
mod converter;
mod output;

fn main() {
    // Parse command-line arguments
    let config = cli::Config::parse();

    // Read input file
    let input = match Input::read(&config.input_file) {
        Ok(i) => i,
        Err(e) => cli::error_exit(&e),
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
            let aobs = parse_aobs(&input.content);
            if aobs.len() < 2 {
                cli::error_exit("Need at least 2 valid AOB instances");
            }
            let (result, stats) = analyzer::analyze_aobs(&input.content);

            // Print header
            if !config.quiet {
                let use_colors = config.output_file.is_none() && is(atty::Stream::Stdout);
                if use_colors {
                    println!(
                        "{}",
                        "==============================================".cyan()
                    );
                    println!("  {}", "Sig-Maker".cyan().bold());
                    println!(
                        "{}",
                        "==============================================".cyan()
                    );
                } else {
                    println!("==============================================");
                    println!("  Sig-Maker");
                    println!("==============================================");
                }
                println!();
                println!("Analyzing {} valid AOB instances:", aobs.len());
                for (i, aob) in aobs.iter().enumerate() {
                    println!("  [{}] {} bytes", i + 1, aob.bytes.len());
                }
                println!();
                println!("All AOBs have {} bytes", aobs[0].bytes.len());
                println!();
                println!("[1/2] Comparing byte-by-byte...");
                println!();
                println!("    Fixed bytes: {}", stats.fixed_bytes());
                println!(
                    "    High nibble wildcards: {}",
                    stats.high_nibble_wildcards()
                );
                println!("    Low nibble wildcards: {}", stats.low_nibble_wildcards());
                println!("    Full wildcards: {}", stats.full_wildcards());
                println!();
            }

            // Show diff table
            if !config.quiet
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
                .unwrap_or_else(|| cli::error_exit("No valid pattern found"));
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
