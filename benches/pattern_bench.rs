use criterion::{Criterion, black_box, criterion_group, criterion_main};
use sig_maker::formats::{BytePattern, Format, format_pattern, optimize_byte, parse_pattern};

fn bench_optimize_byte(c: &mut Criterion) {
    c.bench_function("optimize_byte/fixed", |b| {
        let values = vec![0x5F; 10];
        b.iter(|| optimize_byte(black_box(&values)))
    });

    c.bench_function("optimize_byte/high_nibble", |b| {
        let values = vec![0x40, 0x45, 0x4F, 0x42, 0x49];
        b.iter(|| optimize_byte(black_box(&values)))
    });

    c.bench_function("optimize_byte/low_nibble", |b| {
        let values = vec![0x0F, 0x3F, 0xBF, 0x7F, 0xFF];
        b.iter(|| optimize_byte(black_box(&values)))
    });

    c.bench_function("optimize_byte/wildcard", |b| {
        let values = vec![0x00, 0xFF, 0x42, 0x91, 0x23];
        b.iter(|| optimize_byte(black_box(&values)))
    });
}

fn bench_parse_cheat_engine(c: &mut Criterion) {
    let short = "0? 00 00 00 01 00 00 00 E? FF FF FF 00";
    let medium = "0? 00 00 00 01 00 00 00 E? FF FF FF 00 00 00 00 00 00 00 00 10 B2 DA 97 B2 02 00 00 A0 6B DA 97 B2 02";

    c.bench_function("parse/ce_short", |b| {
        b.iter(|| parse_pattern(black_box(short)))
    });

    c.bench_function("parse/ce_medium", |b| {
        b.iter(|| parse_pattern(black_box(medium)))
    });
}

fn bench_parse_cpp(c: &mut Criterion) {
    let input = "const uint8_t pattern[] = { 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0xE0, 0xFF, 0xFF, 0xFF, 0x00 };";

    c.bench_function("parse/cpp", |b| b.iter(|| parse_pattern(black_box(input))));
}

fn bench_format(c: &mut Criterion) {
    let pattern = vec![
        BytePattern::HighNibble(0),
        BytePattern::Fixed(0x00),
        BytePattern::Fixed(0x00),
        BytePattern::Fixed(0x00),
        BytePattern::Fixed(0x01),
        BytePattern::Fixed(0x00),
        BytePattern::Fixed(0x00),
        BytePattern::Fixed(0x00),
        BytePattern::HighNibble(0x0E),
        BytePattern::Fixed(0xFF),
        BytePattern::Fixed(0xFF),
        BytePattern::Fixed(0xFF),
        BytePattern::Fixed(0x00),
    ];

    c.bench_function("format/ce", |b| {
        b.iter(|| format_pattern(black_box(&pattern), Format::CheatEngine))
    });

    c.bench_function("format/cpp", |b| {
        b.iter(|| format_pattern(black_box(&pattern), Format::Cpp))
    });

    c.bench_function("format/rust", |b| {
        b.iter(|| format_pattern(black_box(&pattern), Format::Rust))
    });

    c.bench_function("format/json", |b| {
        b.iter(|| format_pattern(black_box(&pattern), Format::Json))
    });
}

fn bench_full_workflow(c: &mut Criterion) {
    // Simulates full analysis workflow
    let input = r#"
        # Test AOBs
        07 00 00 00 01 00 00 00 ED FF FF FF 00 00 00 00 00 00 00 00 10 B2 DA 97 B2 02
        08 00 00 00 01 00 00 00 EC FF FF FF 00 00 00 00 00 00 00 00 10 B2 DA 97 B2 02
        09 00 00 00 01 00 00 00 EB FF FF FF 00 00 00 00 00 00 00 00 10 B2 DA 97 B2 02
    "#;

    c.bench_function("workflow/parse_and_optimize", |b| {
        b.iter(|| {
            // Parse lines
            let lines: Vec<_> = input
                .lines()
                .map(|l| l.trim())
                .filter(|l| !l.is_empty() && !l.starts_with('#'))
                .collect();

            // Parse each AOB
            let aobs: Vec<Vec<u8>> = lines
                .iter()
                .map(|line| {
                    line.split_whitespace()
                        .filter_map(|s| u8::from_str_radix(s, 16).ok())
                        .collect()
                })
                .collect();

            // Optimize byte-by-byte
            let first_len = aobs[0].len();
            let mut result = Vec::with_capacity(first_len);

            for byte_idx in 0..first_len {
                let values: Vec<u8> = aobs.iter().map(|aob| aob[byte_idx]).collect();
                result.push(optimize_byte(black_box(&values)));
            }

            result
        })
    });
}

criterion_group!(
    benches,
    bench_optimize_byte,
    bench_parse_cheat_engine,
    bench_parse_cpp,
    bench_format,
    bench_full_workflow
);
criterion_main!(benches);
