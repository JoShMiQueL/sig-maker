//! Sig-Maker - Library interface
//!
//! This crate provides signature/pattern analysis and format conversion.

pub mod analyzer;
pub mod converter;
pub mod formats;
pub mod io;

// Re-export commonly used types
pub use analyzer::{AobInstance, analyze_aobs, aob_matches_pattern, parse_aobs};
pub use formats::{BytePattern, Format, format_pattern, matches_pattern, parse_pattern};

// Re-export converter functions
pub use converter::{convert_to_format, parse_single_pattern};
