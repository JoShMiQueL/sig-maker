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
pub use analyzer::{analyze_aobs, aob_matches_pattern, AobInstance};
pub use formats::{optimize_byte, parse_pattern, format_pattern, BytePattern, Format, matches_pattern};
