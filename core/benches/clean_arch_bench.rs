//! Performance benchmarks for clean architecture (internal API)

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use goxviet_core::application::dto::engine_config::EngineConfig;
use goxviet_core::domain::entities::key_event::{Action, KeyEvent};
use goxviet_core::presentation::di::Container;

/// Benchmark keystroke latency
fn bench_keystroke_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("keystroke_latency");
    group.throughput(Throughput::Elements(1));

    let container = Container::new();
    let processor = container.processor_service();

    group.bench_function("char_a", |b| {
        b.iter(|| {
            let key_event = KeyEvent::text(black_box('a'));
            let result = processor.lock().unwrap().process(black_box(key_event));
            black_box(result)
        });
    });

    group.bench_function("tone_s", |b| {
        b.iter(|| {
            let key_event = KeyEvent::text(black_box('s'));
            let result = processor.lock().unwrap().process(black_box(key_event));
            black_box(result)
        });
    });

    group.bench_function("backspace", |b| {
        b.iter(|| {
            let key_event = KeyEvent::new(' ', Action::Backspace);
            let result = processor.lock().unwrap().process(black_box(key_event));
            black_box(result)
        });
    });

    group.finish();
}

/// Benchmark engine creation
fn bench_engine_lifecycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("engine_lifecycle");

    group.bench_function("container_new", |b| {
        b.iter(|| {
            let _container = Container::new();
            black_box(_container)
        });
    });

    group.bench_function("container_with_config", |b| {
        b.iter(|| {
            let config = EngineConfig::default();
            let _container = Container::with_config(black_box(config));
            black_box(_container)
        });
    });

    group.finish();
}

/// Benchmark sustained throughput
fn bench_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput");
    group.sample_size(20);
    group.throughput(Throughput::Elements(6000)); // 1000 * "viets "

    let container = Container::new();
    let processor = container.processor_service();

    group.bench_function("1000x_viets", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                for ch in "viets ".chars() {
                    let key_event = KeyEvent::text(ch);
                    let _ = processor.lock().unwrap().process(key_event);
                }
            }
        });
    });

    group.finish();
}

/// Benchmark memory operations
fn bench_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory");

    group.bench_function("10_containers", |b| {
        b.iter(|| {
            let containers: Vec<_> = (0..10)
                .map(|_| Container::new())
                .collect();
            black_box(containers)
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_keystroke_latency,
    bench_engine_lifecycle,
    bench_throughput,
    bench_memory,
);
criterion_main!(benches);
