//! English Detection Benchmark (DISABLED - Needs Rewrite)
//!
//! This benchmark file is currently disabled because it was written for an older API.
//! The Engine API has changed significantly:
//! - `on_key()` now requires 3 parameters: (key, caps, cmd)
//! - `InputMethod` enum has been refactored
//! - Result structure has changed
//!
//! TODO: Rewrite this benchmark for the current API when performance optimization is needed.
//!
//! Original purpose:
//! - Benchmark English detection performance across 3 layers
//! - Layer 1: Vietnamese Syllable Validator (~200ns)
//! - Layer 2: Early Pattern Detection (~20ns, HOT PATH)
//! - Layer 3: Multi-Syllable Detection (~150ns)
//! - Target: <1ms average latency
//!
//! Reference: docs/ULTIMATE_ENGLISH_DETECTION_GUIDE.md (if exists)

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_placeholder(c: &mut Criterion) {
    c.bench_function("english_detection/placeholder", |b| {
        b.iter(|| {
            // Placeholder - benchmark disabled
            1 + 1
        })
    });
}

criterion_group!(benches, bench_placeholder);
criterion_main!(benches);
