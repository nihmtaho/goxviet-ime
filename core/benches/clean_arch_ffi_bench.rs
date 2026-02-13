//! Performance benchmarks for clean architecture FFI API

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use std::ffi::CString;

// Import FFI functions
extern "C" {
    fn ime_engine_new() -> *mut std::ffi::c_void;
    fn ime_engine_free(handle: *mut std::ffi::c_void);
    fn ime_process_key(
        handle: *mut std::ffi::c_void,
        key: *const std::os::raw::c_char,
        action: std::os::raw::c_int,
    ) -> FfiProcessResult;
    fn ime_free_string(ptr: *mut std::os::raw::c_char);
}

#[repr(C)]
struct FfiResult {
    success: bool,
    error_code: std::os::raw::c_int,
}

#[repr(C)]
struct FfiProcessResult {
    text: *mut std::os::raw::c_char,
    backspace_count: std::os::raw::c_int,
    consumed: bool,
    result: FfiResult,
}

/// Benchmark single keystroke latency
fn bench_keystroke_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("keystroke_latency");
    group.throughput(Throughput::Elements(1));

    let engine = unsafe { ime_engine_new() };

    group.bench_function("single_char_a", |b| {
        b.iter(|| {
            let key = CString::new("a").unwrap();
            let result = unsafe { 
                ime_process_key(black_box(engine), key.as_ptr(), 0) 
            };
            if !result.text.is_null() {
                unsafe { ime_free_string(result.text) };
            }
            black_box(result)
        });
    });

    group.bench_function("tone_mark_s", |b| {
        b.iter(|| {
            let key = CString::new("s").unwrap();
            let result = unsafe { 
                ime_process_key(black_box(engine), key.as_ptr(), 0) 
            };
            if !result.text.is_null() {
                unsafe { ime_free_string(result.text) };
            }
            black_box(result)
        });
    });

    group.finish();
    unsafe { ime_engine_free(engine) };
}

/// Benchmark engine lifecycle
fn bench_engine_lifecycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("engine_lifecycle");

    group.bench_function("create_and_destroy", |b| {
        b.iter(|| {
            let engine = unsafe { ime_engine_new() };
            unsafe { ime_engine_free(black_box(engine)) };
        });
    });

    group.finish();
}

/// Benchmark throughput
fn bench_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput");
    group.sample_size(20);
    group.throughput(Throughput::Elements(6000)); // 1000 * "viets "

    let engine = unsafe { ime_engine_new() };

    group.bench_function("1000x_viets", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                for ch in "viets ".chars() {
                    let key = CString::new(&ch.to_string()[..]).unwrap();
                    let result = unsafe {
                        ime_process_key(engine, key.as_ptr(), 0)
                    };
                    if !result.text.is_null() {
                        unsafe { ime_free_string(result.text) };
                    }
                }
            }
        });
    });

    group.finish();
    unsafe { ime_engine_free(engine) };
}

criterion_group!(
    benches,
    bench_keystroke_latency,
    bench_engine_lifecycle,
    bench_throughput,
);
criterion_main!(benches);
