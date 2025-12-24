//! Benchmark for English Detection Performance
//!
//! Tests the 3-layer detection architecture:
//! - Layer 1: Vietnamese Syllable Validator (~200ns)
//! - Layer 2: Early Pattern Detection (~20ns, HOT PATH)
//! - Layer 3: Multi-Syllable Detection (~150ns)
//!
//! Target: <1ms average latency
//! Expected: ~50ns weighted average
//!
//! Reference: docs/ULTIMATE_ENGLISH_DETECTION_GUIDE.md

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use goxviet_core::engine::Engine;
use goxviet_core::input::InputMethod;

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/// Type a word into engine and return final text
fn type_word(engine: &mut Engine, word: &str) -> String {
    engine.clear();
    let mut result = String::new();
    
    for ch in word.chars() {
        let key = char_to_key(ch);
        let res = engine.on_key(key);
        result.push_str(&res.chars);
    }
    
    result
}

/// Convert char to key code (simplified for benchmarking)
fn char_to_key(ch: char) -> u16 {
    match ch.to_ascii_lowercase() {
        'a' => 0,
        'b' => 11,
        'c' => 8,
        'd' => 2,
        'e' => 14,
        'f' => 3,
        'g' => 5,
        'h' => 4,
        'i' => 34,
        'j' => 38,
        'k' => 40,
        'l' => 37,
        'm' => 46,
        'n' => 45,
        'o' => 31,
        'p' => 35,
        'q' => 12,
        'r' => 15,
        's' => 1,
        't' => 17,
        'u' => 32,
        'v' => 9,
        'w' => 13,
        'x' => 7,
        'y' => 16,
        'z' => 6,
        _ => 49, // SPACE
    }
}

// =============================================================================
// LAYER 2: EARLY PATTERN DETECTION (2-3 chars) - HOT PATH
// =============================================================================

/// Benchmark 2-char "ex" pattern (CRITICAL HOT PATH)
/// Expected: ~15-20ns (single comparison)
/// Coverage: 80% of early detections (export, express, example, etc.)
fn bench_pattern_ex_2char(c: &mut Criterion) {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex);
    
    c.bench_function("english_detection/layer2_2char_ex", |b| {
        b.iter(|| {
            engine.clear();
            engine.on_key(black_box(char_to_key('e')));
            engine.on_key(black_box(char_to_key('x')));
        })
    });
}

/// Benchmark 3-char "tex" pattern
/// Expected: ~50ns (few comparisons)
/// Coverage: Common words like "text", "texture", "context"
fn bench_pattern_tex_3char(c: &mut Criterion) {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex);
    
    c.bench_function("english_detection/layer2_3char_tex", |b| {
        b.iter(|| {
            engine.clear();
            engine.on_key(black_box(char_to_key('t')));
            engine.on_key(black_box(char_to_key('e')));
            engine.on_key(black_box(char_to_key('x')));
        })
    });
}

/// Benchmark 3-char "imp" pattern
/// Expected: ~50ns
/// Coverage: "import", "important", "implement", "impact"
fn bench_pattern_imp_3char(c: &mut Criterion) {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex);
    
    c.bench_function("english_detection/layer2_3char_imp", |b| {
        b.iter(|| {
            engine.clear();
            engine.on_key(black_box(char_to_key('i')));
            engine.on_key(black_box(char_to_key('m')));
            engine.on_key(black_box(char_to_key('p')));
        })
    });
}

/// Benchmark 3-char "com" pattern
/// Expected: ~50ns
/// Coverage: "computer", "complex", "common", "complete"
fn bench_pattern_com_3char(c: &mut Criterion) {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex);
    
    c.bench_function("english_detection/layer2_3char_com", |b| {
        b.iter(|| {
            engine.clear();
            engine.on_key(black_box(char_to_key('c')));
            engine.on_key(black_box(char_to_key('o')));
            engine.on_key(black_box(char_to_key('m')));
        })
    });
}

/// Benchmark 3-char "ele" pattern
/// Expected: ~50ns
/// Coverage: "element", "delete", "select", "telex", "release"
fn bench_pattern_ele_3char(c: &mut Criterion) {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex);
    
    c.bench_function("english_detection/layer2_3char_ele", |b| {
        b.iter(|| {
            engine.clear();
            engine.on_key(black_box(char_to_key('e')));
            engine.on_key(black_box(char_to_key('l')));
            engine.on_key(black_box(char_to_key('e')));
        })
    });
}

// =============================================================================
// LAYER 3: MULTI-SYLLABLE DETECTION (4+ chars)
// =============================================================================

/// Benchmark C-e-C-e pattern detection (4 chars)
/// Expected: ~100-150ns (single pass scan)
/// Coverage: "tele", "rele", "dele", "sele"
fn bench_pattern_cece_4char(c: &mut Criterion) {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex);
    
    c.bench_function("english_detection/layer3_cece_4char", |b| {
        b.iter(|| {
            engine.clear();
            engine.on_key(black_box(char_to_key('t')));
            engine.on_key(black_box(char_to_key('e')));
            engine.on_key(black_box(char_to_key('l')));
            engine.on_key(black_box(char_to_key('e')));
        })
    });
}

/// Benchmark multiple 'e' detection (7 chars)
/// Expected: ~150-200ns (two pass scan)
/// Coverage: "release" (r-e-l-e-a-s-e, 3 e's)
fn bench_pattern_multiple_e_7char(c: &mut Criterion) {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex);
    
    c.bench_function("english_detection/layer3_multiple_e_7char", |b| {
        b.iter(|| {
            engine.clear();
            engine.on_key(black_box(char_to_key('r')));
            engine.on_key(black_box(char_to_key('e')));
            engine.on_key(black_box(char_to_key('l')));
            engine.on_key(black_box(char_to_key('e')));
            engine.on_key(black_box(char_to_key('a')));
            engine.on_key(black_box(char_to_key('s')));
            engine.on_key(black_box(char_to_key('e')));
        })
    });
}

/// Benchmark multiple 'e' detection (7 chars)
/// Expected: ~150-200ns
/// Coverage: "element" (e-l-e-m-e-n-t, 3 e's)
fn bench_pattern_multiple_e_element(c: &mut Criterion) {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex);
    
    c.bench_function("english_detection/layer3_multiple_e_element", |b| {
        b.iter(|| {
            engine.clear();
            engine.on_key(black_box(char_to_key('e')));
            engine.on_key(black_box(char_to_key('l')));
            engine.on_key(black_box(char_to_key('e')));
            engine.on_key(black_box(char_to_key('m')));
            engine.on_key(black_box(char_to_key('e')));
            engine.on_key(black_box(char_to_key('n')));
            engine.on_key(black_box(char_to_key('t')));
        })
    });
}

// =============================================================================
// INTEGRATED WORD BENCHMARKS (End-to-End)
// =============================================================================

/// Benchmark complete word: "text" (4 chars)
/// Expected: ~20ns average (detected at "tex" 3-char pattern in Layer 2)
fn bench_word_text(c: &mut Criterion) {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex);
    
    c.bench_function("english_detection/word_text", |b| {
        b.iter(|| {
            black_box(type_word(&mut engine, "text"))
        })
    });
}

/// Benchmark complete word: "export" (6 chars)
/// Expected: ~15ns average (detected at "ex" 2-char pattern in Layer 2 - HOT PATH)
fn bench_word_export(c: &mut Criterion) {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex);
    
    c.bench_function("english_detection/word_export", |b| {
        b.iter(|| {
            black_box(type_word(&mut engine, "export"))
        })
    });
}

/// Benchmark complete word: "release" (7 chars)
/// Expected: ~150ns average (detected in Layer 3 with multiple 'e' pattern)
fn bench_word_release(c: &mut Criterion) {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex);
    
    c.bench_function("english_detection/word_release", |b| {
        b.iter(|| {
            black_box(type_word(&mut engine, "release"))
        })
    });
}

/// Benchmark complete word: "element" (7 chars)
/// Expected: ~50ns average (detected at "ele" 3-char pattern in Layer 2)
fn bench_word_element(c: &mut Criterion) {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex);
    
    c.bench_function("english_detection/word_element", |b| {
        b.iter(|| {
            black_box(type_word(&mut engine, "element"))
        })
    });
}

/// Benchmark complete word: "importance" (10 chars)
/// Expected: ~50ns average (detected at "imp" 3-char pattern in Layer 2)
fn bench_word_importance(c: &mut Criterion) {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex);
    
    c.bench_function("english_detection/word_importance", |b| {
        b.iter(|| {
            black_box(type_word(&mut engine, "importance"))
        })
    });
}

/// Benchmark complete word: "complex" (7 chars)
/// Expected: ~50ns average (detected at "com" 3-char pattern in Layer 2)
fn bench_word_complex(c: &mut Criterion) {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex);
    
    c.bench_function("english_detection/word_complex", |b| {
        b.iter(|| {
            black_box(type_word(&mut engine, "complex"))
        })
    });
}

// =============================================================================
// VIETNAMESE WORD BENCHMARKS (No Detection, Normal Transform)
// =============================================================================

/// Benchmark Vietnamese word: "tét" (3 chars + tone)
/// Expected: ~100ns (normal Vietnamese transform, no detection overhead)
fn bench_word_vietnamese_tet(c: &mut Criterion) {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex);
    
    c.bench_function("english_detection/word_vietnamese_tet", |b| {
        b.iter(|| {
            engine.clear();
            engine.on_key(black_box(char_to_key('t')));
            engine.on_key(black_box(char_to_key('e')));
            engine.on_key(black_box(char_to_key('t')));
            engine.on_key(black_box(char_to_key('s'))); // tone sắc
        })
    });
}

/// Benchmark Vietnamese word: "việt" (5 chars with transforms)
/// Expected: ~200ns (complex transforms, no detection overhead)
fn bench_word_vietnamese_viet(c: &mut Criterion) {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex);
    
    c.bench_function("english_detection/word_vietnamese_viet", |b| {
        b.iter(|| {
            black_box(type_word(&mut engine, "vieets"))
        })
    });
}

// =============================================================================
// COMPARISON BENCHMARKS (Before/After Detection)
// =============================================================================

/// Benchmark typing "text" WITHOUT detection (old behavior)
/// This would incorrectly transform to "tẽt"
/// Compare with bench_word_text to see detection overhead
fn bench_word_text_no_detection(c: &mut Criterion) {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex);
    engine.set_free_tone(true); // Disable detection to test old behavior
    
    c.bench_function("english_detection/word_text_no_detection", |b| {
        b.iter(|| {
            black_box(type_word(&mut engine, "text"))
        })
    });
}

// =============================================================================
// CRITERION GROUPS
// =============================================================================

criterion_group!(
    layer2_benches,
    bench_pattern_ex_2char,
    bench_pattern_tex_3char,
    bench_pattern_imp_3char,
    bench_pattern_com_3char,
    bench_pattern_ele_3char,
);

criterion_group!(
    layer3_benches,
    bench_pattern_cece_4char,
    bench_pattern_multiple_e_7char,
    bench_pattern_multiple_e_element,
);

criterion_group!(
    word_benches,
    bench_word_text,
    bench_word_export,
    bench_word_release,
    bench_word_element,
    bench_word_importance,
    bench_word_complex,
);

criterion_group!(
    vietnamese_benches,
    bench_word_vietnamese_tet,
    bench_word_vietnamese_viet,
);

criterion_group!(
    comparison_benches,
    bench_word_text_no_detection,
);

criterion_main!(
    layer2_benches,
    layer3_benches,
    word_benches,
    vietnamese_benches,
    comparison_benches,
);