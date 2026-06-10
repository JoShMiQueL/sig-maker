#![no_main]

use libfuzzer_sys::fuzz_target;
use std::str;

fuzz_target!(|data: &[u8]| {
    // Try to parse the data as a pattern string
    if let Ok(s) = str::from_utf8(data) {
        let _ = sig_maker::formats::parse_pattern(s);
    }
});
