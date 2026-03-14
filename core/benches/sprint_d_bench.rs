//! Sprint D — Performance Benchmarks (T7.2)
//!
//! Verifies that Sprint D changes do not regress existing performance.
//! Target: InputMethodConfig serialization < 1μs, deserialization < 10μs
//!
//! Run with: `cargo bench --bench sprint_d_bench`

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use goxviet_core::domain::entities::input_method_config::InputMethodConfig;
use goxviet_core::presentation::di::Container;

// ─── InputMethodConfig Benchmarks ───────────────────────────────────────────

fn bench_input_method_config(c: &mut Criterion) {
    let mut group = c.benchmark_group("input_method_config");
    group.throughput(Throughput::Elements(1));

    // Telex config construction
    group.bench_function("telex_new", |b| {
        b.iter(|| black_box(InputMethodConfig::telex()))
    });

    // VNI config construction
    group.bench_function("vni_new", |b| {
        b.iter(|| black_box(InputMethodConfig::vni()))
    });

    // JSON serialization
    let telex = InputMethodConfig::telex();
    group.bench_function("telex_to_json", |b| {
        b.iter(|| black_box(telex.to_json().expect("serialize")))
    });

    // JSON deserialization
    let telex_json = InputMethodConfig::telex().to_json().expect("serialize");
    group.bench_function("telex_from_json", |b| {
        b.iter(|| {
            black_box(
                InputMethodConfig::from_json_bytes(black_box(telex_json.as_bytes()))
                    .expect("deserialize"),
            )
        })
    });

    let vni_json = InputMethodConfig::vni().to_json().expect("serialize");
    group.bench_function("vni_from_json", |b| {
        b.iter(|| {
            black_box(
                InputMethodConfig::from_json_bytes(black_box(vni_json.as_bytes()))
                    .expect("deserialize"),
            )
        })
    });

    // method_id lookup (must be O(1))
    let config = InputMethodConfig::telex();
    group.bench_function("method_id_lookup", |b| {
        b.iter(|| black_box(config.method_id()))
    });

    group.finish();
}

// ─── Container load_input_config Benchmark ──────────────────────────────────

fn bench_load_input_config(c: &mut Criterion) {
    let mut group = c.benchmark_group("load_input_config");
    group.throughput(Throughput::Elements(1));

    let telex_json = InputMethodConfig::telex().to_json().expect("serialize");
    let vni_json = InputMethodConfig::vni().to_json().expect("serialize");

    group.bench_function("telex_load", |b| {
        b.iter(|| {
            let mut container = Container::new();
            let config = InputMethodConfig::from_json_bytes(black_box(telex_json.as_bytes()))
                .expect("deserialize");
            container.load_input_config(black_box(config));
        })
    });

    group.bench_function("vni_load", |b| {
        b.iter(|| {
            let mut container = Container::new();
            let config = InputMethodConfig::from_json_bytes(black_box(vni_json.as_bytes()))
                .expect("deserialize");
            container.load_input_config(black_box(config));
        })
    });

    group.finish();
}

// ─── Keystroke Latency with Config ──────────────────────────────────────────

fn bench_keystroke_with_config(c: &mut Criterion) {
    use goxviet_core::domain::entities::key_event::KeyEvent;

    let mut group = c.benchmark_group("keystroke_with_config");
    group.throughput(Throughput::Elements(1));

    // Baseline: keystroke after loading InputMethodConfig
    let telex_config = InputMethodConfig::telex();
    let mut container = Container::new();
    container.load_input_config(telex_config);
    let processor = container.processor_service();

    for key_char in ['a', 's', 'e', 'w', 'd'] {
        group.bench_with_input(BenchmarkId::new("key", key_char), &key_char, |b, &ch| {
            let keycode = goxviet_core::utils::char_to_key(ch) as u16;
            let event = KeyEvent::new(keycode, false, false, false, false);
            b.iter(|| {
                let result = processor.lock().unwrap().process_key(black_box(event));
                black_box(result)
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_input_method_config,
    bench_load_input_config,
    bench_keystroke_with_config,
);
criterion_main!(benches);
