//! Sig-Maker - Multi-format signature/pattern converter and optimizer

use sig_maker::analyzer;
use sig_maker::cli::Config;
use sig_maker::converter;
use sig_maker::io::{self, Input};

fn main() {
    // Parse command-line arguments
    let config = Config::parse();

    // Read input file
    let input = match Input::read(&config.input_file) {
        Ok(i) => i,
        Err(e) => io::error_exit(&e),
    };

    // Detect input type and process accordingly
    match input.detect_type() {
        io::InputType::MultipleAobs => {
            // Analyze multiple AOB instances
            analyzer::analyze_aobs(&input.content, config.to_format);
        }
        io::InputType::CodePattern | io::InputType::SimplePattern => {
            // Convert single pattern
            let pattern = input
                .extract_pattern()
                .unwrap_or_else(|| io::error_exit("No valid pattern found"));
            converter::convert_pattern(&pattern, config.to_format);
        }
    }
}
