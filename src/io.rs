//! Input/output operations and content detection

use colored::Colorize;
use std::fs;
use std::path::PathBuf;

/// Input content
pub struct Input {
    pub content: String,
}

/// Type of input detected
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputType {
    /// Multiple AOB instances for analysis
    MultipleAobs,
    /// Single pattern for conversion (code format)
    CodePattern,
    /// Single pattern for conversion (simple format)
    SimplePattern,
}

impl Input {
    /// Read input from file or use as direct input
    pub fn read(source: &str) -> Result<Self, String> {
        let (content, _source_name) = if PathBuf::from(source).exists() {
            let content = fs::read_to_string(source)
                .map_err(|e| format!("Could not read file '{}': {}", source, e))?;
            (content, source.to_string())
        } else {
            // Input might be a direct pattern
            (source.to_string(), "<input>".to_string())
        };

        Ok(Self { content })
    }

    /// Detect the type of input content
    pub fn detect_type(&self) -> InputType {
        let trimmed = self.content.trim();

        // Check if this looks like a code pattern (C++, Rust, JSON, etc.)
        let looks_like_code = trimmed.contains("const uint8_t")
            || trimmed.contains("static PATTERN")
            || trimmed.contains("[u8;")
            || trimmed.starts_with("{ ")
            || trimmed.starts_with("{\"")
            || (trimmed.starts_with('{') && trimmed.contains("pattern"))
            || trimmed.contains("import re")
            || trimmed.contains("idaapi");

        if looks_like_code {
            return InputType::CodePattern;
        }

        // Count valid lines
        let lines: Vec<&str> = trimmed.lines().collect();
        let non_empty_lines: Vec<&str> = lines
            .iter()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty() && !l.starts_with('#') && !l.starts_with("//"))
            .collect();

        if non_empty_lines.len() >= 2 {
            InputType::MultipleAobs
        } else if non_empty_lines.len() == 1 {
            let line = non_empty_lines[0];
            let cleaned = if line.starts_with("- ") || line.starts_with("* ") {
                &line[2..]
            } else {
                line
            };

            // Check if it looks like a pattern
            if cleaned.contains('?')
                || cleaned.contains("0x")
                || cleaned.contains('[')
                || cleaned.contains('.')
            {
                InputType::SimplePattern
            } else {
                // Doesn't look like a pattern, assume it's meant to be a file
                InputType::MultipleAobs
            }
        } else {
            // No valid input
            InputType::MultipleAobs
        }
    }

    /// Extract the first valid pattern from input
    pub fn extract_pattern(&self) -> Option<String> {
        let trimmed = self.content.trim();

        // If it's code, return the whole content
        if trimmed.contains("const uint8_t")
            || trimmed.contains("static PATTERN")
            || trimmed.contains("[u8;")
            || trimmed.starts_with('{')
            || trimmed.contains("import re")
            || trimmed.contains("idaapi")
        {
            return Some(trimmed.to_string());
        }

        // Extract from single line
        let lines: Vec<&str> = trimmed.lines().collect();
        let non_empty_lines: Vec<&str> = lines
            .iter()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty() && !l.starts_with('#') && !l.starts_with("//"))
            .collect();

        if non_empty_lines.is_empty() {
            return None;
        }

        let line = non_empty_lines[0];
        let cleaned = if line.starts_with("- ") || line.starts_with("* ") {
            line[2..].to_string()
        } else {
            line.to_string()
        };

        Some(cleaned)
    }
}

/// Print error and exit
pub fn error_exit(msg: &str) -> ! {
    eprintln!("{}: {}", "ERROR".red().bold(), msg);
    std::process::exit(1);
}
