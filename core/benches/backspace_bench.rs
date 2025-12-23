//! Backspace Performance Benchmarks
//!
//! Tests backspace latency on various scenarios:
//! - Simple characters (target: < 1ms)
//! - Complex syllables with transforms (target: < 3ms)
//! - Long words (>10 syllables) (target: < 5ms)

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use goxviet_core::engine::Engine;
use goxviet_core::engine::shortcut::InputMethod;

/// Helper: Type a sequence and return the engine
fn type_sequence(method: InputMethod, keys: &str) -> Engine {
    let mut engine = Engine::new();
    engine.set_method(method as u8);
    engine.set_enabled(true);
    
    for ch in keys.chars() {
        let key = char_to_key(ch);
        let caps = ch.is_uppercase();
        engine.on_key_ext(key, caps, false, false);
    }
    
    engine
}

/// Convert char to key code (simplified)
fn char_to_key(ch: char) -> u16 {
    match ch.to_ascii_lowercase() {
        'a' => 0, 'b' => 11, 'c' => 8, 'd' => 2, 'e' => 14,
        'f' => 3, 'g' => 5, 'h' => 4, 'i' => 34, 'j' => 38,
        'k' => 40, 'l' => 37, 'm' => 46, 'n' => 45, 'o' => 31,
        'p' => 35, 'q' => 12, 'r' => 15, 's' => 1, 't' => 17,
        'u' => 32, 'v' => 9, 'w' => 13, 'x' => 7, 'y' => 16,
        'z' => 6,
        ' ' => 49,
        _ => 0,
    }
}

const DELETE_KEY: u16 = 51;

/// Benchmark 1: Simple character deletion (no transforms)
/// Target: < 1ms (should be O(1))
fn bench_simple_char_backspace(c: &mut Criterion) {
    let mut group = c.benchmark_group("backspace_simple");
    
    for word_len in [3, 5, 10, 20, 50].iter() {
        group.bench_with_input(
            BenchmarkId::new("simple_chars", word_len),
            word_len,
            |b, &len| {
                b.iter(|| {
                    // Type simple English word
                    let keys = "a".repeat(len);
                    let mut engine = type_sequence(InputMethod::Telex, &keys);
                    
                    // Delete last character
                    black_box(engine.on_key_ext(DELETE_KEY, false, false, false))
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark 2: Complex syllable with transforms
/// Target: < 3ms
fn bench_complex_syllable_backspace(c: &mut Criterion) {
    let mut group = c.benchmark_group("backspace_complex_syllable");
    
    let test_cases = vec![
        ("hoas", "hòa + s = hoás"),           // Tone addition
        ("tuowf", "tuơ + w + f = tươf"),      // Multiple transforms
        ("thuowngj", "thuơng + j = thương"),  // Full syllable
        ("nguowif", "nguơi + f = người"),     // Complex compound
    ];
    
    for (keys, desc) in test_cases {
        group.bench_function(desc, |b| {
            b.iter(|| {
                let mut engine = type_sequence(InputMethod::Telex, keys);
                black_box(engine.on_key_ext(DELETE_KEY, false, false, false))
            });
        });
    }
    
    group.finish();
}

/// Benchmark 3: Backspace in long multi-syllable words
/// Target: < 5ms (this is the REGRESSION case we're fixing)
fn bench_long_word_backspace(c: &mut Criterion) {
    let mut group = c.benchmark_group("backspace_long_words");
    
    // Test words with increasing syllable count
    let test_cases = vec![
        (3, "xin chao ban"),                    // 3 syllables
        (5, "toi dang hoc tieng viet"),         // 5 syllables  
        (10, "hoaf owf uwf tuowf nhuwx oaf troongf coongf tyf"),  // 10+ syllables (complex)
        (15, "hoaf owf uwf tuowf nhuwx oaf troongf coongf tyf vieejt naams"),
    ];
    
    for (syllable_count, keys) in test_cases {
        group.bench_with_input(
            BenchmarkId::new("syllables", syllable_count),
            &keys,
            |b, &input| {
                b.iter(|| {
                    let mut engine = type_sequence(InputMethod::Telex, input);
                    // Delete last character - should only rebuild last syllable
                    black_box(engine.on_key_ext(DELETE_KEY, false, false, false))
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark 4: Multiple consecutive backspaces
/// Target: Consistent performance (no degradation)
fn bench_consecutive_backspaces(c: &mut Criterion) {
    let mut group = c.benchmark_group("backspace_consecutive");
    
    for backspace_count in [1, 5, 10, 20].iter() {
        group.bench_with_input(
            BenchmarkId::new("backspaces", backspace_count),
            backspace_count,
            |b, &count| {
                b.iter(|| {
                    let mut engine = type_sequence(InputMethod::Telex, "thuowngj");
                    
                    // Perform multiple backspaces
                    for _ in 0..count {
                        black_box(engine.on_key_ext(DELETE_KEY, false, false, false));
                    }
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark 5: Backspace after tone/mark changes
/// Tests if last_transform state affects performance
fn bench_backspace_after_transform(c: &mut Criterion) {
    let mut group = c.benchmark_group("backspace_after_transform");
    
    let scenarios = vec![
        ("vieets", "việt + s", "add_tone"),
        ("hoaa", "hôa", "add_mark"),
        ("dd", "đ", "add_stroke"),
        ("uow", "ươ", "compound_vowel"),
    ];
    
    for (keys, result, name) in scenarios {
        group.bench_function(name, |b| {
            b.iter(|| {
                let mut engine = type_sequence(InputMethod::Telex, keys);
                black_box(engine.on_key_ext(DELETE_KEY, false, false, false))
            });
        });
    }
    
    group.finish();
}

/// Benchmark 6: Backspace at syllable boundaries
/// Tests if boundary detection is efficient
fn bench_backspace_at_boundary(c: &mut Criterion) {
    let mut group = c.benchmark_group("backspace_at_boundary");
    
    // Type multi-syllable word, delete at different positions
    group.bench_function("after_space", |b| {
        b.iter(|| {
            let mut engine = type_sequence(InputMethod::Telex, "xin chao ");
            black_box(engine.on_key_ext(DELETE_KEY, false, false, false))
        });
    });
    
    group.bench_function("mid_word", |b| {
        b.iter(|| {
            let mut engine = type_sequence(InputMethod::Telex, "xinchao");
            black_box(engine.on_key_ext(DELETE_KEY, false, false, false))
        });
    });
    
    group.finish();
}

/// Benchmark 7: Worst-case scenario
/// Very long word with many transforms at different positions
fn bench_worst_case(c: &mut Criterion) {
    c.bench_function("backspace_worst_case", |b| {
        b.iter(|| {
            // 50+ characters with transforms scattered throughout
            let keys = "thuowngjthuowngjthuowngjthuowngjthuowngjthuowngjthuowngj";
            let mut engine = type_sequence(InputMethod::Telex, keys);
            black_box(engine.on_key_ext(DELETE_KEY, false, false, false))
        });
    });
}

criterion_group!(
    benches,
    bench_simple_char_backspace,
    bench_complex_syllable_backspace,
    bench_long_word_backspace,
    bench_consecutive_backspaces,
    bench_backspace_after_transform,
    bench_backspace_at_boundary,
    bench_worst_case
);

criterion_main!(benches);