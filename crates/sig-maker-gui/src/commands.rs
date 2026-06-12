//! Tauri commands exposed to the frontend

use serde::{Deserialize, Serialize};
use sig_maker_core::formats::Format;
use sig_maker_core::{analyze_aobs, convert_to_format, parse_aobs, parse_single_pattern};

/// A format entry returned to the frontend
#[derive(Debug, Serialize, Deserialize)]
pub struct FormatInfo {
    pub id: String,
    pub name: String,
}

/// Result of a pattern conversion
#[derive(Debug, Serialize, Deserialize)]
pub struct ConversionResult {
    pub output: String,
    pub byte_count: usize,
    pub wildcard_count: usize,
}

/// Return all supported formats for the UI dropdown
#[tauri::command]
pub fn get_formats() -> Vec<FormatInfo> {
    Format::all()
        .iter()
        .map(|f| FormatInfo {
            id: format!("{:?}", f).to_lowercase(),
            name: f.name().to_string(),
        })
        .collect()
}

/// Convert a pattern string to the requested format
///
/// If the input contains multiple lines (AOB instances), it will analyze them
/// and generate an optimized pattern with wildcards where bytes differ.
/// If the input is a single pattern, it converts directly.
#[tauri::command]
pub fn convert_pattern(input: String, format_id: String) -> Result<ConversionResult, String> {
    let format =
        Format::from_string(&format_id).ok_or_else(|| format!("Unknown format: {format_id}"))?;

    // Check if input has multiple non-empty lines (AOB analysis mode)
    let non_empty_lines: Vec<&str> = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#') && !l.starts_with("//"))
        .collect();

    let parsed = if non_empty_lines.len() >= 2 {
        // Multiple lines: use AOB analysis to find optimal pattern
        let aobs = parse_aobs(&input);
        if aobs.len() < 2 {
            return Err(
                "Could not parse multiple AOB instances — check the input format".to_string(),
            );
        }

        // Verify all have same length
        let first_len = aobs[0].bytes.len();
        for (i, aob) in aobs.iter().enumerate() {
            if aob.bytes.len() != first_len {
                return Err(format!(
                    "Line {} has {} bytes, expected {} (all lines must have the same length)",
                    i + 1,
                    aob.bytes.len(),
                    first_len
                ));
            }
        }

        analyze_aobs(&input)
    } else {
        // Single pattern: parse and convert directly
        parse_single_pattern(&input)
            .ok_or_else(|| "Could not parse pattern — check the input format".to_string())?
    };

    let output = convert_to_format(&parsed, format);

    // Calculate stats from the pattern
    let byte_count = parsed.len();
    let wildcard_count = parsed
        .iter()
        .filter(|b| matches!(b, sig_maker_core::BytePattern::Wildcard))
        .count();

    Ok(ConversionResult {
        output,
        byte_count,
        wildcard_count,
    })
}
