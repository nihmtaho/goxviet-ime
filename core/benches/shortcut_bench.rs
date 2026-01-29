//! Shortcut Expansion Benchmarks
//!
//! Tests shortcut lookup and expansion latency:
//! - Lookup with 10, 50, 100, 200 shortcuts
//! - Target: < 1ms for all cases

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use goxviet_core::engine::shortcut::{Shortcut, ShortcutTable};

/// Benchmark shortcut lookup with varying table sizes
fn bench_shortcut_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("shortcut_lookup");

    for count in [10, 50, 100, 200].iter() {
        let mut table = ShortcutTable::new();

        // Add shortcuts
        for i in 0..*count {
            let trigger = format!("trigger{}", i);
            let replacement = format!("Replacement text for trigger {}", i);
            table.add(Shortcut::new(&trigger, &replacement));
        }

        group.bench_with_input(BenchmarkId::new("shortcuts", count), count, |b, _| {
            b.iter(|| {
                // Lookup existing trigger (worst case: last added)
                black_box(table.lookup(&format!("trigger{}", count - 1)))
            });
        });
    }

    group.finish();
}

/// Benchmark shortcut lookup miss (no match)
fn bench_shortcut_lookup_miss(c: &mut Criterion) {
    let mut group = c.benchmark_group("shortcut_lookup_miss");

    let mut table = ShortcutTable::new();
    for i in 0..200 {
        table.add(Shortcut::new(&format!("t{}", i), &format!("r{}", i)));
    }

    group.bench_function("no_match", |b| {
        b.iter(|| black_box(table.lookup("nonexistent")));
    });

    group.finish();
}

/// Benchmark try_match with word boundary
fn bench_try_match(c: &mut Criterion) {
    let mut group = c.benchmark_group("shortcut_try_match");

    let mut table = ShortcutTable::new();
    table.add(Shortcut::new("vn", "Việt Nam"));
    table.add(Shortcut::new("hcm", "Hồ Chí Minh"));
    table.add(Shortcut::new("hn", "Hà Nội"));

    group.bench_function("match_short", |b| {
        b.iter(|| black_box(table.try_match("vn", Some(' '), true)));
    });

    group.bench_function("match_longer", |b| {
        b.iter(|| black_box(table.try_match("hcm", Some(' '), true)));
    });

    group.bench_function("no_boundary", |b| {
        b.iter(|| black_box(table.try_match("vn", Some('a'), false)));
    });

    group.finish();
}

/// Benchmark JSON export
fn bench_json_export(c: &mut Criterion) {
    let mut group = c.benchmark_group("shortcut_json");

    for count in [10, 50, 100].iter() {
        let mut table = ShortcutTable::new();
        for i in 0..*count {
            table.add(Shortcut::new(
                &format!("t{}", i),
                &format!("Thành phố Hồ Chí Minh {}", i),
            ));
        }

        group.bench_with_input(BenchmarkId::new("export", count), count, |b, _| {
            b.iter(|| black_box(table.to_json()));
        });
    }

    group.finish();
}

/// Benchmark JSON import
fn bench_json_import(c: &mut Criterion) {
    let mut group = c.benchmark_group("shortcut_json_import");

    // Prepare JSON strings
    for count in [10, 50, 100].iter() {
        let mut table = ShortcutTable::new();
        for i in 0..*count {
            table.add(Shortcut::new(
                &format!("t{}", i),
                &format!("Replacement {}", i),
            ));
        }
        let json = table.to_json();

        group.bench_with_input(BenchmarkId::new("import", count), &json, |b, json_str| {
            b.iter(|| {
                let mut new_table = ShortcutTable::new();
                black_box(new_table.from_json(json_str))
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_shortcut_lookup,
    bench_shortcut_lookup_miss,
    bench_try_match,
    bench_json_export,
    bench_json_import,
);

criterion_main!(benches);
