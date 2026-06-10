//! Sig-Maker - Library interface
//!
//! This crate provides signature/pattern analysis and format conversion.

pub mod analyzer;
pub mod cli;
pub mod converter;
pub mod formats;
pub mod io;
pub mod output;

// Re-export commonly used types
pub use analyzer::{AobInstance, analyze_aobs, aob_matches_pattern};
pub use formats::{
    BytePattern, Format, format_pattern, matches_pattern, optimize_byte, parse_pattern,
};
