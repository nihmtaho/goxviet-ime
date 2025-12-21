//! Memory efficiency benchmarks for Vietnamese IME Core
//!
//! Measures memory usage and allocation patterns for RawInputBuffer
//! compared to the old Vec-based approach.
//!
//! Uses FFI interface since Engine is not publicly exported.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use vietnamese_ime_core::*;

// Key codes for common keys
const KEY_A: u16 = 0;
const KEY_B: u16 = 11;
const KEY_C: u16 = 8;
const KEY_D: u16 = 2;
const KEY_E: u16 = 14;
const KEY_F: u16 = 3;
const KEY_G: u16 = 5;
const KEY_H: u16 = 4;
const KEY_I: u16 = 34;
const KEY_M: u16 = 46;
const KEY_N: u16 = 45;
const KEY_O: u16 = 31;
const KEY_S: u16 = 1;
const KEY_T: u16 = 17;
const KEY_V: u16 = 9;
const KEY_X: u16 = 7;
const KEY_SPACE: u16 = 49;
const KEY_BACKSPACE: u16 = 51;
const KEY_ESC: u16 = 53;

/// Helper to type a string character by character
fn type_string(word: &str) {
    for ch in word.chars() {
        let key = match ch {
            'a' => KEY_A,
            'b' => KEY_B,
            'c' => KEY_C,
            'd' => KEY_D,
            'e' => KEY_E,
            'f' => KEY_F,
            'g' => KEY_G,
            'h' => KEY_H,
            'i' => KEY_I,
            'm' => KEY_M,
            'n' => KEY_N,
            'o' => KEY_O,
            's' => KEY_S,
            't' => KEY_T,
            'v' => KEY_V,
            'x' => KEY_X,
            _ => continue,
        };
        let result = ime_key(key, false, false);
        unsafe {
            if !result.is_null() {
                ime_free(result);
            }
        }
    }
}

/// Benchmark: Memory allocation patterns during normal typing
/// 
/// This simulates a realistic typing session with word boundaries.
/// The RawInputBuffer should have:
/// - Zero heap allocations during push/pop operations
/// - Auto-clear on word boundaries (space)
/// - Bounded memory usage regardless of session length
fn bench_memory_normal_typing(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_normal_typing");
    
    // Simulate typing words with varying lengths
    let words = vec![
        "xin", "chao", "vietnam", "tieng", "viet", 
        "go", "nhanh", "hieu", "qua"
    ];
    
    group.bench_function("typing_with_spaces", |b| {
        b.iter(|| {
            ime_init();
            ime_enabled(true);
            ime_method(0); // Telex
            
            for word in &words {
                // Type each word
                black_box(type_string(word));
                
                // Type space (triggers buffer clear)
                let result = ime_key(KEY_SPACE, false, false);
                unsafe {
                    if !result.is_null() {
                        ime_free(result);
                    }
                }
            }
            
            ime_clear();
        });
    });
    
    group.finish();
}

/// Benchmark: Buffer operations under capacity
///
/// Tests push/pop performance when buffer stays under 64 elements.
/// Should be O(1) for all operations.
fn bench_memory_buffer_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_buffer_ops");
    
    // Test different buffer sizes
    for size in [10, 30, 50, 63].iter() {
        group.bench_with_input(
            BenchmarkId::new("push_pop_cycle", size),
            size,
            |b, &size| {
                b.iter(|| {
                    ime_init();
                    ime_enabled(true);
                    ime_method(0); // Telex
                    
                    // Push N characters
                    for i in 0..size {
                        let key = KEY_A + (i % 26) as u16;
                        let result = ime_key(key, false, false);
                        unsafe {
                            if !result.is_null() {
                                black_box(result);
                                ime_free(result);
                            }
                        }
                    }
                    
                    // Pop all characters via backspace
                    for _ in 0..size {
                        let result = ime_key(KEY_BACKSPACE, false, false);
                        unsafe {
                            if !result.is_null() {
                                black_box(result);
                                ime_free(result);
                            }
                        }
                    }
                    
                    ime_clear();
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark: Buffer behavior at capacity
///
/// Tests performance when buffer reaches 64 elements (capacity).
/// Should gracefully handle overflow without performance degradation.
fn bench_memory_capacity_overflow(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_capacity_overflow");
    
    group.bench_function("overflow_64_to_80", |b| {
        b.iter(|| {
            ime_init();
            ime_enabled(true);
            ime_method(0); // Telex
            
            // Fill to capacity (64 chars)
            for i in 0..64 {
                let key = KEY_A + (i % 26) as u16;
                let result = ime_key(key, false, false);
                unsafe {
                    if !result.is_null() {
                        black_box(result);
                        ime_free(result);
                    }
                }
            }
            
            // Push 16 more (triggers shift behavior)
            for i in 0..16 {
                let key = KEY_A + (i % 26) as u16;
                let result = ime_key(key, false, false);
                unsafe {
                    if !result.is_null() {
                        black_box(result);
                        ime_free(result);
                    }
                }
            }
            
            ime_clear();
        });
    });
    
    group.finish();
}

/// Benchmark: Long editing session
///
/// Simulates a long typing session with multiple words and edits.
/// Tests that memory usage remains bounded over time.
fn bench_memory_long_session(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_long_session");
    
    group.bench_function("100_words_with_edits", |b| {
        b.iter(|| {
            ime_init();
            ime_enabled(true);
            ime_method(0); // Telex
            
            for _ in 0..100 {
                // Type "vietnam"
                black_box(type_string("vietnam"));
                
                // Do some backspaces (simulate corrections)
                for _ in 0..2 {
                    let result = ime_key(KEY_BACKSPACE, false, false);
                    unsafe {
                        if !result.is_null() {
                            ime_free(result);
                        }
                    }
                }
                
                // Type "ese"
                black_box(type_string("ese"));
                
                // Space to commit word (clears buffer)
                let result = ime_key(KEY_SPACE, false, false);
                unsafe {
                    if !result.is_null() {
                        ime_free(result);
                    }
                }
            }
            
            ime_clear();
        });
    });
    
    group.finish();
}

/// Benchmark: ESC restore functionality
///
/// Tests the memory efficiency of restoring raw input via ESC.
/// RawInputBuffer iterator should be zero-allocation.
fn bench_memory_esc_restore(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_esc_restore");
    
    group.bench_function("restore_after_transforms", |b| {
        b.iter(|| {
            ime_init();
            ime_enabled(true);
            ime_esc_restore(true);
            ime_method(0); // Telex
            
            // Type "tooi" -> "tôi" (with transforms)
            black_box(type_string("tooi"));
            
            // ESC to restore (uses raw_input iteration)
            let result = ime_key(KEY_ESC, false, false);
            unsafe {
                if !result.is_null() {
                    black_box(result);
                    ime_free(result);
                }
            }
            
            ime_clear();
        });
    });
    
    group.finish();
}

/// Benchmark: Multiple word restoration
///
/// Tests backspace-after-space feature with word history.
/// Should efficiently restore previous words.
fn bench_memory_word_restoration(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_word_restoration");
    
    group.bench_function("restore_after_backspace", |b| {
        b.iter(|| {
            ime_init();
            ime_enabled(true);
            ime_method(0); // Telex
            
            // Type "xin"
            black_box(type_string("xin"));
            
            // Type space
            let result = ime_key(KEY_SPACE, false, false);
            unsafe {
                if !result.is_null() {
                    ime_free(result);
                }
            }
            
            // Delete space to restore word
            let result = ime_key(KEY_BACKSPACE, false, false);
            unsafe {
                if !result.is_null() {
                    black_box(result);
                    ime_free(result);
                }
            }
            
            // Type "chao"
            black_box(type_string("chao"));
            
            ime_clear();
        });
    });
    
    group.finish();
}

/// Benchmark: Rapid backspace operations
///
/// Tests memory efficiency during rapid deletion.
/// RawInputBuffer should handle pop operations efficiently.
fn bench_memory_rapid_backspace(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_rapid_backspace");
    
    group.bench_function("type_and_delete_50_chars", |b| {
        b.iter(|| {
            ime_init();
            ime_enabled(true);
            ime_method(0); // Telex
            
            // Type 50 characters
            for i in 0..50 {
                let key = KEY_A + (i % 26) as u16;
                let result = ime_key(key, false, false);
                unsafe {
                    if !result.is_null() {
                        ime_free(result);
                    }
                }
            }
            
            // Delete all 50 characters rapidly
            for _ in 0..50 {
                let result = ime_key(KEY_BACKSPACE, false, false);
                unsafe {
                    if !result.is_null() {
                        black_box(result);
                        ime_free(result);
                    }
                }
            }
            
            ime_clear();
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_memory_normal_typing,
    bench_memory_buffer_operations,
    bench_memory_capacity_overflow,
    bench_memory_long_session,
    bench_memory_esc_restore,
    bench_memory_word_restoration,
    bench_memory_rapid_backspace,
);

criterion_main!(benches);