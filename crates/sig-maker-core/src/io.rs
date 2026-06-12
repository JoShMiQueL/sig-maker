//! Input/output operations and content detection

use std::fs;
use std::io::{self, Read};
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
    /// Read input from file, stdin, or use as direct input
    pub fn read(source: &str) -> Result<Self, String> {
        Self::read_with_stdin(source, false)
    }

    /// Read input from file, stdin, or use as direct input
    /// If `use_stdin` is true, reads from stdin when source is "-"
    pub fn read_with_stdin(source: &str, use_stdin: bool) -> Result<Self, String> {
        let (content, _source_name) = if use_stdin && source == "-" {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .map_err(|e| format!("Could not read from stdin: {}", e))?;
            (buffer, "<stdin>".to_string())
        } else if PathBuf::from(source).exists() {
            let content = fs::read_to_string(source)
                .map_err(|e| format!("Could not read file '{}': {}", source, e))?;
            (content, source.to_string())
        } else {
            // Input is a direct pattern (not a file)
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
            // Multiple lines: check if each line is a separate AOB instance
            // or if it's a single pattern split across multiple lines
            let all_lines_look_like_patterns = non_empty_lines.iter().all(|line| {
                let cleaned = if line.starts_with("- ") || line.starts_with("* ") {
                    &line[2..]
                } else {
                    line
                };
                let tokens: Vec<&str> = cleaned.split_whitespace().collect();
                tokens.len() >= 2
                    && tokens.iter().all(|t| {
                        t.len() == 2 && t.chars().all(|c| c.is_ascii_hexdigit() || c == '?')
                    })
            });

            if all_lines_look_like_patterns {
                InputType::MultipleAobs
            } else {
                // Mixed content - treat as single pattern (take first line)
                InputType::SimplePattern
            }
        } else if non_empty_lines.len() == 1 {
            // Single line - always treat as SimplePattern if it looks like a pattern
            // This handles the case where a single pattern has multiple tokens
            let line = non_empty_lines[0];
            let cleaned = if line.starts_with("- ") || line.starts_with("* ") {
                &line[2..]
            } else {
                line
            };

            // Check if it looks like a pattern (wildcards, dots, brackets, or hex bytes)
            let has_wildcards = cleaned.contains('?') || cleaned.contains('.');
            let has_brackets = cleaned.contains('[');

            // More lenient hex detection: allow wildcards mixed with hex
            let tokens: Vec<&str> = cleaned.split_whitespace().collect();
            let has_hex = tokens.len() >= 2
                && tokens.iter().all(|t| {
                    // Allow: "AB" (hex), "A?" (high nibble), "?B" (low nibble), "??" (wildcard)
                    t.len() == 2 && t.chars().all(|c| c.is_ascii_hexdigit() || c == '?')
                });

            if has_wildcards || has_brackets || has_hex {
                InputType::SimplePattern
            } else {
                // Doesn't look like a pattern, assume it's meant to be a file
                InputType::MultipleAobs
            }
        } else {
            // No valid input - treat as simple pattern (empty input is valid for some cases)
            InputType::SimplePattern
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

        // Return the cleaned line - let the parser decide if it's valid
        if cleaned.is_empty() {
            None
        } else {
            Some(cleaned)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_read_direct_string() {
        let input = Input::read("AB CD EF").unwrap();
        assert_eq!(input.content, "AB CD EF");
    }

    #[test]
    fn input_read_with_stdin_false() {
        let input = Input::read_with_stdin("AB CD EF", false).unwrap();
        assert_eq!(input.content, "AB CD EF");
    }

    #[test]
    fn input_read_with_stdin_true_file_not_found() {
        // When use_stdin is true and the file doesn't exist, it treats it as direct input
        let result = Input::read_with_stdin("nonexistent_file.txt", true);
        // It should succeed by treating the string as direct input
        assert!(result.is_ok());
        assert_eq!(result.unwrap().content, "nonexistent_file.txt");
    }

    #[test]
    fn input_read_with_stdin_true_direct_input() {
        // When use_stdin is true and the file doesn't exist, it treats it as direct input
        let result = Input::read_with_stdin("AB CD EF", true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().content, "AB CD EF");
    }

    #[test]
    fn detect_type_code_pattern_cpp() {
        let input = Input {
            content: "const uint8_t pattern[] = { 0xAB, 0xCD };".to_string(),
        };
        assert_eq!(input.detect_type(), InputType::CodePattern);
    }

    #[test]
    fn detect_type_code_pattern_rust() {
        let input = Input {
            content: "static PATTERN: [u8; 2] = [0xAB, 0xCD];".to_string(),
        };
        assert_eq!(input.detect_type(), InputType::CodePattern);
    }

    #[test]
    fn detect_type_code_pattern_json() {
        let input = Input {
            content: "{ \"pattern\": [0xAB, 0xCD] }".to_string(),
        };
        assert_eq!(input.detect_type(), InputType::CodePattern);
    }

    #[test]
    fn detect_type_code_pattern_python() {
        let input = Input {
            content: "import re\npattern = re.compile(b'\\xAB\\xCD')".to_string(),
        };
        assert_eq!(input.detect_type(), InputType::CodePattern);
    }

    #[test]
    fn detect_type_multiple_aobs() {
        let input = Input {
            content: "AB CD EF\n12 34 56".to_string(),
        };
        assert_eq!(input.detect_type(), InputType::MultipleAobs);
    }

    #[test]
    fn detect_type_simple_pattern() {
        let input = Input {
            content: "AB CD EF".to_string(),
        };
        assert_eq!(input.detect_type(), InputType::SimplePattern);
    }

    #[test]
    fn detect_type_simple_pattern_with_wildcards() {
        let input = Input {
            content: "AB ?? CD".to_string(),
        };
        assert_eq!(input.detect_type(), InputType::SimplePattern);
    }

    #[test]
    fn detect_type_simple_pattern_with_dots() {
        let input = Input {
            content: "AB .. CD".to_string(),
        };
        assert_eq!(input.detect_type(), InputType::SimplePattern);
    }

    #[test]
    fn detect_type_simple_pattern_with_brackets() {
        let input = Input {
            content: "AB [A0-AF] CD".to_string(),
        };
        assert_eq!(input.detect_type(), InputType::SimplePattern);
    }

    #[test]
    fn detect_type_empty() {
        let input = Input {
            content: "".to_string(),
        };
        assert_eq!(input.detect_type(), InputType::SimplePattern);
    }

    #[test]
    fn detect_type_with_comments() {
        let input = Input {
            content: "# Comment\nAB CD EF".to_string(),
        };
        assert_eq!(input.detect_type(), InputType::SimplePattern);
    }

    #[test]
    fn detect_type_with_prefixes() {
        let input = Input {
            content: "- AB CD EF\n* 12 34 56".to_string(),
        };
        assert_eq!(input.detect_type(), InputType::MultipleAobs);
    }

    #[test]
    fn detect_type_mixed_content() {
        let input = Input {
            content: "AB CD EF\ninvalid line\n12 34 56".to_string(),
        };
        assert_eq!(input.detect_type(), InputType::SimplePattern);
    }

    #[test]
    fn extract_pattern_code() {
        let input = Input {
            content: "const uint8_t pattern[] = { 0xAB, 0xCD };".to_string(),
        };
        let result = input.extract_pattern();
        assert_eq!(
            result,
            Some("const uint8_t pattern[] = { 0xAB, 0xCD };".to_string())
        );
    }

    #[test]
    fn extract_pattern_simple() {
        let input = Input {
            content: "AB CD EF".to_string(),
        };
        let result = input.extract_pattern();
        assert_eq!(result, Some("AB CD EF".to_string()));
    }

    #[test]
    fn extract_pattern_with_prefix() {
        let input = Input {
            content: "- AB CD EF".to_string(),
        };
        let result = input.extract_pattern();
        assert_eq!(result, Some("AB CD EF".to_string()));
    }

    #[test]
    fn extract_pattern_with_comment() {
        let input = Input {
            content: "# Comment\nAB CD EF".to_string(),
        };
        let result = input.extract_pattern();
        assert_eq!(result, Some("AB CD EF".to_string()));
    }

    #[test]
    fn extract_pattern_empty() {
        let input = Input {
            content: "".to_string(),
        };
        let result = input.extract_pattern();
        assert_eq!(result, None);
    }

    #[test]
    fn extract_pattern_only_comments() {
        let input = Input {
            content: "# Comment\n// Another".to_string(),
        };
        let result = input.extract_pattern();
        assert_eq!(result, None);
    }

    #[test]
    fn extract_pattern_multiline_code() {
        let input = Input {
            content: "const uint8_t pattern[] = { 0xAB,\n0xCD };".to_string(),
        };
        let result = input.extract_pattern();
        assert!(result.is_some());
        assert!(result.unwrap().contains("const uint8_t"));
    }

    #[test]
    fn input_type_equality() {
        assert_eq!(InputType::MultipleAobs, InputType::MultipleAobs);
        assert_eq!(InputType::CodePattern, InputType::CodePattern);
        assert_eq!(InputType::SimplePattern, InputType::SimplePattern);
        assert_ne!(InputType::MultipleAobs, InputType::CodePattern);
        assert_ne!(InputType::CodePattern, InputType::SimplePattern);
    }
}
