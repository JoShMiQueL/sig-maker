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
        bytes,
        mask,
        pattern.len()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_pattern_cheat_engine() {
        let pattern = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Wildcard,
            BytePattern::HighNibble(0xC),
            BytePattern::LowNibble(0xD),
        ];
        let result = format_pattern(&pattern, Format::CheatEngine);
        assert_eq!(result, "AB ?? C? ?D");
    }

    #[test]
    fn format_pattern_cpp() {
        let pattern = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Wildcard,
            BytePattern::HighNibble(0xC),
            BytePattern::LowNibble(0xD),
        ];
        let result = format_pattern(&pattern, Format::Cpp);
        assert!(result.contains("const uint8_t pattern[]"));
        assert!(result.contains("0xAB"));
        assert!(result.contains("0x00"));
        assert!(result.contains("0xC0"));
        assert!(result.contains("0x0D"));
        assert!(result.contains("const uint8_t mask[]"));
    }

    #[test]
    fn format_pattern_rust() {
        let pattern = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Wildcard,
            BytePattern::HighNibble(0xC),
            BytePattern::LowNibble(0xD),
        ];
        let result = format_pattern(&pattern, Format::Rust);
        assert!(result.contains("static PATTERN: [u8; 4]"));
        assert!(result.contains("0xAB"));
        assert!(result.contains("0x00"));
        assert!(result.contains("0xC0"));
        assert!(result.contains("0x0D"));
        assert!(result.contains("static MASK: [u8; 4]"));
    }

    #[test]
    fn format_pattern_ghidra() {
        let pattern = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Wildcard,
            BytePattern::HighNibble(0xC),
            BytePattern::LowNibble(0xD),
        ];
        let result = format_pattern(&pattern, Format::Ghidra);
        assert!(result.contains("AB"));
        assert!(result.contains("."));
        assert!(result.contains("[C0-CF]"));
        assert!(result.contains("[0D"));
    }

    #[test]
    fn format_pattern_ida_pro() {
        let pattern = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Wildcard,
            BytePattern::HighNibble(0xC),
            BytePattern::LowNibble(0xD),
        ];
        let result = format_pattern(&pattern, Format::IdaPro);
        assert!(result.contains("AB"));
        assert!(result.contains("[0-F0]"));
        assert!(result.contains("[C0-CF]"));
        assert!(result.contains("[0D"));
    }

    #[test]
    fn format_pattern_x64dbg() {
        let pattern = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Wildcard,
            BytePattern::HighNibble(0xC),
            BytePattern::LowNibble(0xD),
        ];
        let result = format_pattern(&pattern, Format::X64dbg);
        assert_eq!(result, "AB .. C. .D");
    }

    #[test]
    fn format_pattern_python() {
        let pattern = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Wildcard,
            BytePattern::HighNibble(0xC),
            BytePattern::LowNibble(0xD),
        ];
        let result = format_pattern(&pattern, Format::Python);
        assert!(result.contains("import re"));
        assert!(result.contains("pattern = re.compile"));
        assert!(result.contains("\\xAB"));
        assert!(result.contains("."));
        assert!(result.contains("[\\xC0-\\xCF]"));
        assert!(result.contains("[\\x0D"));
    }

    #[test]
    fn format_pattern_json() {
        let pattern = vec![
            BytePattern::Fixed(0xAB),
            BytePattern::Wildcard,
            BytePattern::HighNibble(0xC),
            BytePattern::LowNibble(0xD),
        ];
        let result = format_pattern(&pattern, Format::Json);
        assert!(result.contains("\"pattern\""));
        assert!(result.contains("\"mask\""));
        assert!(result.contains("\"length\""));
        assert!(result.contains("4"));
    }

    #[test]
    fn format_cheat_engine_fixed() {
        let pattern = vec![BytePattern::Fixed(0xAB), BytePattern::Fixed(0xCD)];
        let result = format_cheat_engine(&pattern);
        assert_eq!(result, "AB CD");
    }

    #[test]
    fn format_cheat_engine_wildcard() {
        let pattern = vec![BytePattern::Wildcard, BytePattern::Wildcard];
        let result = format_cheat_engine(&pattern);
        assert_eq!(result, "?? ??");
    }

    #[test]
    fn format_cheat_engine_high_nibble() {
        let pattern = vec![BytePattern::HighNibble(0xA), BytePattern::HighNibble(0xC)];
        let result = format_cheat_engine(&pattern);
        assert_eq!(result, "A? C?");
    }

    #[test]
    fn format_cheat_engine_low_nibble() {
        let pattern = vec![BytePattern::LowNibble(0xB), BytePattern::LowNibble(0xD)];
        let result = format_cheat_engine(&pattern);
        assert_eq!(result, "?B ?D");
    }

    #[test]
    fn format_cheat_engine_empty() {
        let pattern = vec![];
        let result = format_cheat_engine(&pattern);
        assert_eq!(result, "");
    }

    #[test]
    fn format_cpp_fixed() {
        let pattern = vec![BytePattern::Fixed(0xAB), BytePattern::Fixed(0xCD)];
        let result = format_cpp(&pattern);
        assert!(result.contains("0xAB, 0xCD"));
        assert!(result.contains("0xFF, 0xFF"));
    }

    #[test]
    fn format_cpp_wildcard() {
        let pattern = vec![BytePattern::Wildcard, BytePattern::Wildcard];
        let result = format_cpp(&pattern);
        assert!(result.contains("0x00, 0x00"));
        assert!(result.contains("0x00, 0x00"));
    }

    #[test]
    fn format_cpp_high_nibble() {
        let pattern = vec![BytePattern::HighNibble(0xA), BytePattern::HighNibble(0xC)];
        let result = format_cpp(&pattern);
        assert!(result.contains("0xA0, 0xC0"));
        assert!(result.contains("0xF0, 0xF0"));
    }

    #[test]
    fn format_cpp_low_nibble() {
        let pattern = vec![BytePattern::LowNibble(0xB), BytePattern::LowNibble(0xD)];
        let result = format_cpp(&pattern);
        assert!(result.contains("0x0B, 0x0D"));
        assert!(result.contains("0x0F, 0x0F"));
    }

    #[test]
    fn format_rust_fixed() {
        let pattern = vec![BytePattern::Fixed(0xAB), BytePattern::Fixed(0xCD)];
        let result = format_rust(&pattern);
        assert!(result.contains("static PATTERN: [u8; 2]"));
        assert!(result.contains("0xAB, 0xCD"));
        assert!(result.contains("static MASK: [u8; 2]"));
        assert!(result.contains("0xFF, 0xFF"));
    }

    #[test]
    fn format_rust_wildcard() {
        let pattern = vec![BytePattern::Wildcard, BytePattern::Wildcard];
        let result = format_rust(&pattern);
        assert!(result.contains("0x00, 0x00"));
        assert!(result.contains("0x00, 0x00"));
    }

    #[test]
    fn format_rust_high_nibble() {
        let pattern = vec![BytePattern::HighNibble(0xA), BytePattern::HighNibble(0xC)];
        let result = format_rust(&pattern);
        assert!(result.contains("0xA0, 0xC0"));
        assert!(result.contains("0xF0, 0xF0"));
    }

    #[test]
    fn format_rust_low_nibble() {
        let pattern = vec![BytePattern::LowNibble(0xB), BytePattern::LowNibble(0xD)];
        let result = format_rust(&pattern);
        assert!(result.contains("0x0B, 0x0D"));
        assert!(result.contains("0x0F, 0x0F"));
    }

    #[test]
    fn format_ghidra_fixed() {
        let pattern = vec![BytePattern::Fixed(0xAB), BytePattern::Fixed(0xCD)];
        let result = format_ghidra(&pattern);
        assert_eq!(result, "AB CD");
    }

    #[test]
    fn format_ghidra_wildcard() {
        let pattern = vec![BytePattern::Wildcard, BytePattern::Wildcard];
        let result = format_ghidra(&pattern);
        assert_eq!(result, ". .");
    }

    #[test]
    fn format_ghidra_high_nibble() {
        let pattern = vec![BytePattern::HighNibble(0xA), BytePattern::HighNibble(0xC)];
        let result = format_ghidra(&pattern);
        assert_eq!(result, "[A0-AF] [C0-CF]");
    }

    #[test]
    fn format_ghidra_low_nibble() {
        let pattern = vec![BytePattern::LowNibble(0xB), BytePattern::LowNibble(0xD)];
        let result = format_ghidra(&pattern);
        assert!(result.contains("[0B"));
        assert!(result.contains("[0D"));
    }

    #[test]
    fn format_ida_pro_fixed() {
        let pattern = vec![BytePattern::Fixed(0xAB), BytePattern::Fixed(0xCD)];
        let result = format_ida_pro(&pattern);
        assert_eq!(result, "AB CD");
    }

    #[test]
    fn format_ida_pro_wildcard() {
        let pattern = vec![BytePattern::Wildcard, BytePattern::Wildcard];
        let result = format_ida_pro(&pattern);
        assert_eq!(result, "[0-F0] [0-F0]");
    }

    #[test]
    fn format_ida_pro_high_nibble() {
        let pattern = vec![BytePattern::HighNibble(0xA), BytePattern::HighNibble(0xC)];
        let result = format_ida_pro(&pattern);
        assert_eq!(result, "[A0-AF] [C0-CF]");
    }

    #[test]
    fn format_ida_pro_low_nibble() {
        let pattern = vec![BytePattern::LowNibble(0xB), BytePattern::LowNibble(0xD)];
        let result = format_ida_pro(&pattern);
        assert!(result.contains("[0B"));
        assert!(result.contains("[0D"));
    }

    #[test]
    fn format_x64dbg_fixed() {
        let pattern = vec![BytePattern::Fixed(0xAB), BytePattern::Fixed(0xCD)];
        let result = format_x64dbg(&pattern);
        assert_eq!(result, "AB CD");
    }

    #[test]
    fn format_x64dbg_wildcard() {
        let pattern = vec![BytePattern::Wildcard, BytePattern::Wildcard];
        let result = format_x64dbg(&pattern);
        assert_eq!(result, ".. ..");
    }

    #[test]
    fn format_x64dbg_high_nibble() {
        let pattern = vec![BytePattern::HighNibble(0xA), BytePattern::HighNibble(0xC)];
        let result = format_x64dbg(&pattern);
        assert_eq!(result, "A. C.");
    }

    #[test]
    fn format_x64dbg_low_nibble() {
        let pattern = vec![BytePattern::LowNibble(0xB), BytePattern::LowNibble(0xD)];
        let result = format_x64dbg(&pattern);
        assert_eq!(result, ".B .D");
    }

    #[test]
    fn format_python_fixed() {
        let pattern = vec![BytePattern::Fixed(0xAB), BytePattern::Fixed(0xCD)];
        let result = format_python(&pattern);
        assert!(result.contains("\\xAB"));
        assert!(result.contains("\\xCD"));
    }

    #[test]
    fn format_python_wildcard() {
        let pattern = vec![BytePattern::Wildcard, BytePattern::Wildcard];
        let result = format_python(&pattern);
        assert!(result.contains(".."));
    }

    #[test]
    fn format_python_high_nibble() {
        let pattern = vec![BytePattern::HighNibble(0xA), BytePattern::HighNibble(0xC)];
        let result = format_python(&pattern);
        assert!(result.contains("[\\xA0-\\xAF]"));
        assert!(result.contains("[\\xC0-\\xCF]"));
    }

    #[test]
    fn format_python_low_nibble() {
        let pattern = vec![BytePattern::LowNibble(0xB), BytePattern::LowNibble(0xD)];
        let result = format_python(&pattern);
        assert!(result.contains("[\\x0B"));
        assert!(result.contains("[\\x0D"));
    }

    #[test]
    fn format_json_fixed() {
        let pattern = vec![BytePattern::Fixed(0xAB), BytePattern::Fixed(0xCD)];
        let result = format_json(&pattern);
        assert!(result.contains("[171, 205]"));
        assert!(result.contains("[255, 255]"));
        assert!(result.contains("\"length\": 2"));
    }

    #[test]
    fn format_json_wildcard() {
        let pattern = vec![BytePattern::Wildcard, BytePattern::Wildcard];
        let result = format_json(&pattern);
        assert!(result.contains("[0, 0]"));
        assert!(result.contains("[0, 0]"));
    }

    #[test]
    fn format_json_high_nibble() {
        let pattern = vec![BytePattern::HighNibble(0xA), BytePattern::HighNibble(0xC)];
        let result = format_json(&pattern);
        assert!(result.contains("[160, 192]"));
        assert!(result.contains("[240, 240]"));
    }

    #[test]
    fn format_json_low_nibble() {
        let pattern = vec![BytePattern::LowNibble(0xB), BytePattern::LowNibble(0xD)];
        let result = format_json(&pattern);
        assert!(result.contains("[11, 13]"));
        assert!(result.contains("[15, 15]"));
    }
}
