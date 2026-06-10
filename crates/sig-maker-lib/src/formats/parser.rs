//! Pattern parsers - convert from various formats to internal representation

use super::BytePattern;

/// Parse a pattern from any format
pub fn parse_pattern(input: &str) -> Option<Vec<BytePattern>> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Try C++ / Rust arrays (check before other formats)
    if trimmed.contains("const uint8_t") || (trimmed.contains("{") && trimmed.contains("0x")) {
        return parse_cpp_array(trimmed);
    }

    if trimmed.contains("static") && trimmed.contains("[u8;") {
        return parse_rust_array(trimmed);
    }

    // Try JSON
    if trimmed.starts_with('{') {
        return parse_json(trimmed);
    }

    // Try x64dbg (uses dots)
    if trimmed.contains('.') && !trimmed.contains('?') && !trimmed.contains('[') {
        return parse_x64dbg(trimmed);
    }

    // Try IDA/Ghidra (uses brackets)
    if trimmed.contains('[') && trimmed.contains('-') {
        return parse_ida_pro(trimmed);
    }

    // Try Cheat Engine (uses ?? and ? nibbles)
    if trimmed.contains('?') {
        return parse_cheat_engine(trimmed);
    }

    // Try raw hex (space-separated)
    if trimmed.len() >= 2
        && trimmed
            .chars()
            .nth(1)
            .map(|c| c.is_ascii_hexdigit())
            .unwrap_or(false)
    {
        return parse_hex_bytes(trimmed);
    }

    None
}

fn parse_cheat_engine(input: &str) -> Option<Vec<BytePattern>> {
    let mut result = Vec::new();

    for token in input.split_whitespace() {
        let pattern = if token == "??" || token == "?" {
            BytePattern::Wildcard
        } else if token.len() == 2 && token.starts_with('?') {
            let low = u8::from_str_radix(&token[1..2], 16).ok()?;
            BytePattern::LowNibble(low)
        } else if token.len() == 2 && token.ends_with('?') {
            let high = u8::from_str_radix(&token[0..1], 16).ok()?;
            BytePattern::HighNibble(high)
        } else if token.len() == 2 {
            let byte = u8::from_str_radix(token, 16).ok()?;
            BytePattern::Fixed(byte)
        } else {
            continue;
        };
        result.push(pattern);
    }

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

fn parse_x64dbg(input: &str) -> Option<Vec<BytePattern>> {
    let mut result = Vec::new();

    for token in input.split_whitespace() {
        let pattern = if token == "." || token == ".." {
            BytePattern::Wildcard
        } else if token.len() == 2 && token.starts_with('.') {
            let low = u8::from_str_radix(&token[1..2], 16).ok()?;
            BytePattern::LowNibble(low)
        } else if token.len() == 2 && token.ends_with('.') {
            let high = u8::from_str_radix(&token[0..1], 16).ok()?;
            BytePattern::HighNibble(high)
        } else if token.len() == 2 {
            let byte = u8::from_str_radix(token, 16).ok()?;
            BytePattern::Fixed(byte)
        } else {
            continue;
        };
        result.push(pattern);
    }

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

fn parse_ida_pro(input: &str) -> Option<Vec<BytePattern>> {
    let mut result = Vec::new();

    for token in input.split_whitespace() {
        if !token.starts_with('[') {
            let byte = u8::from_str_radix(token, 16).ok()?;
            result.push(BytePattern::Fixed(byte));
        } else {
            let inner = &token[1..token.len() - 1];
            if inner.contains('-') {
                let parts: Vec<&str> = inner.split('-').collect();
                if parts.len() == 2 {
                    let start = u8::from_str_radix(parts[0], 16).ok()?;
                    let end = u8::from_str_radix(parts[1], 16).ok()?;

                    if (start >> 4) == (end >> 4) && (start & 0x0F) == 0 && (end & 0x0F) == 0x0F {
                        result.push(BytePattern::HighNibble(start >> 4));
                    } else if (start & 0x0F) == (end & 0x0F)
                        && (start & 0xF0) == 0
                        && (end & 0xF0) == 0xF0
                    {
                        result.push(BytePattern::LowNibble(start & 0x0F));
                    } else {
                        result.push(BytePattern::Wildcard);
                    }
                }
            }
        }
    }

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

fn parse_cpp_array(input: &str) -> Option<Vec<BytePattern>> {
    let mut values = Vec::new();
    let mut masks = Vec::new();

    if let Some(start) = input.find('{') {
        if let Some(end) = input.find('}') {
            let content = &input[start + 1..end];
            for token in content.split(',') {
                let trimmed = token.trim();
                if let Some(hex) = trimmed.strip_prefix("0x") {
                    let val = u8::from_str_radix(hex, 16).ok()?;
                    values.push(val);
                }
            }
        }
    }

    // Find mask array
    if let Some(mask_start) = input.rfind('{') {
        if let Some(mask_end) = input.rfind('}') {
            if mask_start != input.find('{')? {
                let content = &input[mask_start + 1..mask_end];
                for token in content.split(',') {
                    let trimmed = token.trim();
                    if let Some(hex) = trimmed.strip_prefix("0x") {
                        let val = u8::from_str_radix(hex, 16).ok()?;
                        masks.push(val);
                    }
                }
            }
        }
    }

    Some(apply_mask(&values, &masks))
}

fn parse_rust_array(input: &str) -> Option<Vec<BytePattern>> {
    let mut values = Vec::new();
    let mut masks = Vec::new();

    if let Some(start) = input.find('[') {
        if let Some(end) = input.rfind(']') {
            let content = &input[start + 1..end];
            for token in content.split(',') {
                let trimmed = token.trim();
                if let Some(hex) = trimmed.strip_prefix("0x") {
                    let val = u8::from_str_radix(hex, 16).ok()?;
                    values.push(val);
                }
            }
        }
    }

    // Find mask
    if let Some(mask_start) = input.rfind('[') {
        if let Some(mask_end) = input.rfind(']') {
            if mask_start != input.find('[')? {
                let content = &input[mask_start + 1..mask_end];
                for token in content.split(',') {
                    let trimmed = token.trim();
                    if let Some(hex) = trimmed.strip_prefix("0x") {
                        let val = u8::from_str_radix(hex, 16).ok()?;
                        masks.push(val);
                    }
                }
            }
        }
    }

    Some(apply_mask(&values, &masks))
}

fn parse_json(input: &str) -> Option<Vec<BytePattern>> {
    let pattern_start = input.find("\"pattern\"")?;
    let pattern_arr_start = input[pattern_start..].find('[')? + pattern_start;
    let pattern_arr_end = input[pattern_arr_start..].find(']')? + pattern_arr_start;

    let pattern_content = &input[pattern_arr_start + 1..pattern_arr_end];
    let mut values = Vec::new();
    for token in pattern_content.split(',') {
        let trimmed = token.trim();
        let val = if let Some(hex) = trimmed.strip_prefix("0x") {
            u8::from_str_radix(hex, 16).ok()?
        } else {
            trimmed.parse::<u8>().ok()?
        };
        values.push(val);
    }

    let mut masks = Vec::new();
    if let Some(mask_start) = input.find("\"mask\"") {
        if let Some(mask_arr_start) = input[mask_start..].find('[') {
            let mask_arr_start = mask_arr_start + mask_start;
            if let Some(mask_arr_end) = input[mask_arr_start..].find(']') {
                let mask_arr_end = mask_arr_end + mask_arr_start;
                let mask_content = &input[mask_arr_start + 1..mask_arr_end];
                for token in mask_content.split(',') {
                    let trimmed = token.trim();
                    let val = if let Some(hex) = trimmed.strip_prefix("0x") {
                        u8::from_str_radix(hex, 16).ok()?
                    } else {
                        trimmed.parse::<u8>().ok()?
                    };
                    masks.push(val);
                }
            }
        }
    }

    Some(apply_mask(&values, &masks))
}

fn parse_hex_bytes(input: &str) -> Option<Vec<BytePattern>> {
    let mut result = Vec::new();

    for token in input.split_whitespace() {
        if token.len() == 2 {
            let byte = u8::from_str_radix(token, 16).ok()?;
            result.push(BytePattern::Fixed(byte));
        }
    }

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

fn apply_mask(values: &[u8], masks: &[u8]) -> Vec<BytePattern> {
    values
        .iter()
        .enumerate()
        .map(|(i, &val)| {
            let mask = masks.get(i).copied().unwrap_or(0xFF);
            match mask {
                0xFF => BytePattern::Fixed(val),
                0x00 => BytePattern::Wildcard,
                0xF0 => BytePattern::HighNibble(val >> 4),
                0x0F => BytePattern::LowNibble(val & 0x0F),
                _ => BytePattern::Wildcard,
            }
        })
        .collect()
}
