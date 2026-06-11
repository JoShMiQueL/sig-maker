//! Tauri commands exposed to the frontend

use serde::{Deserialize, Serialize};
use sig_maker_core::formats::Format;
use sig_maker_core::{convert_to_format, get_pattern_stats, parse_single_pattern};

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
    pub specificity: f64,
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
/// Returns an error string if the pattern cannot be parsed.
#[tauri::command]
pub fn convert_pattern(input: String, format_id: String) -> Result<ConversionResult, String> {
    let parsed = parse_single_pattern(&input)
        .ok_or_else(|| "Could not parse pattern — check the input format".to_string())?;

    let format =
        Format::from_string(&format_id).ok_or_else(|| format!("Unknown format: {format_id}"))?;

    let output = convert_to_format(&parsed, format);
    let stats = get_pattern_stats(&parsed);

    Ok(ConversionResult {
        output,
        byte_count: stats.total_bytes(),
        wildcard_count: stats.full_wildcards(),
        specificity: stats.compression_ratio(),
    })
}
