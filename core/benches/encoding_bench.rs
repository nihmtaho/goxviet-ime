//! Encoding Conversion Performance Benchmarks
//!
//! Tests the latency of converting a standard UTF-8 string to legacy
//! Vietnamese encodings.
//! - TCVN3 (ABC)
//! - VNI
//! - CP1258
//! Target: < 1ms for typical sentences.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use goxviet_core::engine::features::encoding::{convert_to_encoding, Encoding};

const SAMPLE_TEXT: &str = "Trăm năm trong cõi người ta, chữ tài chữ mệnh khéo là ghét nhau.";

fn bench_encoding_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("encoding_conversion");

    let encodings = vec![
        ("TCVN3", Encoding::TCVN3),
        ("VNI", Encoding::VNI),
        ("CP1258", Encoding::CP1258),
    ];

    for (name, encoding) in encodings {
        group.bench_function(name, |b| {
            b.iter(|| {
                black_box(convert_to_encoding(SAMPLE_TEXT, encoding));
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench_encoding_conversion);
criterion_main!(benches);
