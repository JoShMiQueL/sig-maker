//! Pattern formatters - convert from internal representation to various formats

use super::{BytePattern, Format};

/// Format pattern for output
pub fn format_pattern(pattern: &[BytePattern], format: Format) -> String {
    match format {
        Format::CheatEngine => format_cheat_engine(pattern),
        Format::Cpp => format_cpp(pattern),
        Format::Rust => format_rust(pattern),
        Format::Ghidra => format_ghidra(pattern),
        Format::IdaPro => format_ida_pro(pattern),
        Format::X64dbg => format_x64dbg(pattern),
        Format::Python => format_python(pattern),
        Format::Json => format_json(pattern),
    }
}

fn format_cheat_engine(pattern: &[BytePattern]) -> String {
    pattern
        .iter()
        .map(|b| match b {
            BytePattern::Fixed(v) => format!("{:02X}", v),
            BytePattern::Wildcard => "??".to_string(),
            BytePattern::HighNibble(h) => format!("{:X}?", h),
            BytePattern::LowNibble(l) => format!("?{:X}", l),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn format_cpp(pattern: &[BytePattern]) -> String {
    let bytes: Vec<String> = pattern
        .iter()
        .map(|b| match b {
            BytePattern::Fixed(v) => format!("0x{:02X}", v),
            BytePattern::Wildcard => "0x00".to_string(),
            BytePattern::HighNibble(h) => format!("0x{:X}0", h),
            BytePattern::LowNibble(l) => format!("0x0{:X}", l),
        })
        .collect();

    let mask: Vec<String> = pattern
        .iter()
        .map(|b| match b {
            BytePattern::Fixed(_) => "0xFF".to_string(),
            BytePattern::Wildcard => "0x00".to_string(),
            BytePattern::HighNibble(_) => "0xF0".to_string(),
            BytePattern::LowNibble(_) => "0x0F".to_string(),
        })
        .collect();

    format!(
        "// C++ Pattern + Mask\nconst uint8_t pattern[] = {{ {} }};\nconst uint8_t mask[]    = {{ {} }};",
        bytes.join(", "),
        mask.join(", ")
    )
}

fn format_rust(pattern: &[BytePattern]) -> String {
    let bytes: Vec<String> = pattern
        .iter()
        .map(|b| match b {
            BytePattern::Fixed(v) => format!("0x{:02X}", v),
            BytePattern::Wildcard => "0x00".to_string(),
            BytePattern::HighNibble(h) => format!("0x{:X}0", h),
            BytePattern::LowNibble(l) => format!("0x0{:X}", l),
        })
        .collect();

    let mask: Vec<String> = pattern
        .iter()
        .map(|b| match b {
            BytePattern::Fixed(_) => "0xFF".to_string(),
            BytePattern::Wildcard => "0x00".to_string(),
            BytePattern::HighNibble(_) => "0xF0".to_string(),
            BytePattern::LowNibble(_) => "0x0F".to_string(),
        })
        .collect();

    format!(
        "// Rust Pattern + Mask\nstatic PATTERN: [u8; {}] = [{}];\nstatic MASK: [u8; {}]    = [{}];",
        pattern.len(),
        bytes.join(", "),
        pattern.len(),
        mask.join(", ")
    )
}

fn format_ghidra(pattern: &[BytePattern]) -> String {
    pattern
        .iter()
        .map(|b| match b {
            BytePattern::Fixed(v) => format!("{:02X}", v),
            BytePattern::Wildcard => ".".to_string(),
            BytePattern::HighNibble(h) => format!("[{:02X}-{:02X}]", h << 4, (h << 4) | 0x0F),
            BytePattern::LowNibble(l) => format!(
                "[{:02X},{:02X},{:02X},{:02X},{:02X},{:02X},{:02X},{:02X},{:02X},{:02X},{:02X},{:02X},{:02X},{:02X},{:02X},{:02X}]",
                l, l + 0x10, l + 0x20, l + 0x30, l + 0x40, l + 0x50, l + 0x60, l + 0x70,
                l + 0x80, l + 0x90, l + 0xA0, l + 0xB0, l + 0xC0, l + 0xD0, l + 0xE0, l + 0xF0
            ),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn format_ida_pro(pattern: &[BytePattern]) -> String {
    pattern
        .iter()
        .map(|b| match b {
            BytePattern::Fixed(v) => format!("{:02X}", v),
            BytePattern::Wildcard => "[0-F0]".to_string(),
            BytePattern::HighNibble(h) => format!("[{:02X}-{:02X}]", h << 4, (h << 4) | 0x0F),
            BytePattern::LowNibble(l) => format!(
                "[{:02X},{:02X},{:02X},{:02X},{:02X},{:02X},{:02X},{:02X},{:02X},{:02X},{:02X},{:02X},{:02X},{:02X},{:02X},{:02X}]",
                l, l + 0x10, l + 0x20, l + 0x30, l + 0x40, l + 0x50, l + 0x60, l + 0x70,
                l + 0x80, l + 0x90, l + 0xA0, l + 0xB0, l + 0xC0, l + 0xD0, l + 0xE0, l + 0xF0
            ),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn format_x64dbg(pattern: &[BytePattern]) -> String {
    pattern
        .iter()
        .map(|b| match b {
            BytePattern::Fixed(v) => format!("{:02X}", v),
            BytePattern::Wildcard => "..".to_string(),
            BytePattern::HighNibble(h) => format!("{:X}.", h),
            BytePattern::LowNibble(l) => format!(".{:X}", l),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn format_python(pattern: &[BytePattern]) -> String {
    let bytes: String = pattern
        .iter()
        .map(|b| match b {
            BytePattern::Fixed(v) => format!("\\x{:02X}", v),
            BytePattern::Wildcard => ".".to_string(),
            BytePattern::HighNibble(h) => format!("[\\x{:X}0-\\x{:X}F]", h, h),
            BytePattern::LowNibble(l) => format!(
                "[\\x{:02X}\\x{:02X}\\x{:02X}\\x{:02X}\\x{:02X}\\x{:02X}\\x{:02X}\\x{:02X}\\x{:02X}\\x{:02X}\\x{:02X}\\x{:02X}\\x{:02X}\\x{:02X}\\x{:02X}\\x{:02X}]",
                l, l + 0x10, l + 0x20, l + 0x30, l + 0x40, l + 0x50, l + 0x60, l + 0x70,
                l + 0x80, l + 0x90, l + 0xA0, l + 0xB0, l + 0xC0, l + 0xD0, l + 0xE0, l + 0xF0
            ),
        })
        .collect();

    format!("import re\npattern = re.compile(b'{}')", bytes)
}

fn format_json(pattern: &[BytePattern]) -> String {
    let bytes: Vec<u8> = pattern
        .iter()
        .map(|b| match b {
            BytePattern::Fixed(v) => *v,
            BytePattern::Wildcard => 0x00,
            BytePattern::HighNibble(h) => h << 4,
            BytePattern::LowNibble(l) => *l,
        })
        .collect();

    let mask: Vec<u8> = pattern
        .iter()
        .map(|b| match b {
            BytePattern::Fixed(_) => 0xFF,
            BytePattern::Wildcard => 0x00,
            BytePattern::HighNibble(_) => 0xF0,
            BytePattern::LowNibble(_) => 0x0F,
        })
        .collect();

    format!(
        "{{ \"pattern\": {:?}, \"mask\": {:?}, \"length\": {} }}",
        bytes, mask, pattern.len()
    )
}
