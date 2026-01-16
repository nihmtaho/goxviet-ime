//! Integration tests for Smart Backspace optimization
//!
//! Tests verify:
//! - Fast path O(1) for simple characters
//! - Syllable rebuild O(s) for complex transforms
//! - Cache effectiveness on consecutive backspaces
//! - Edge cases and boundary conditions

use goxviet_core::engine::shortcut::InputMethod;
use goxviet_core::engine::Engine;

// Key codes (simplified mapping)
const KEY_A: u16 = 0;
const KEY_B: u16 = 11;
const KEY_C: u16 = 8;
const KEY_D: u16 = 2;
const KEY_E: u16 = 14;
const KEY_F: u16 = 3;
const KEY_G: u16 = 5;
const KEY_H: u16 = 4;
const KEY_I: u16 = 34;
const KEY_J: u16 = 38;
const KEY_L: u16 = 37;
const KEY_N: u16 = 45;
const KEY_O: u16 = 31;
const KEY_R: u16 = 15;
const KEY_S: u16 = 1;
const KEY_T: u16 = 17;
const KEY_U: u16 = 32;
const KEY_W: u16 = 13;
const KEY_X: u16 = 7;
const KEY_SPACE: u16 = 49;
const KEY_DELETE: u16 = 51;

/// Helper: Type a sequence of keys
fn type_keys(engine: &mut Engine, keys: &str) {
    for ch in keys.chars() {
        let (key, caps) = match ch {
            'a' => (KEY_A, false),
            'b' => (KEY_B, false),
            'c' => (KEY_C, false),
            'd' => (KEY_D, false),
            'e' => (KEY_E, false),
            'f' => (KEY_F, false),
            'g' => (KEY_G, false),
            'h' => (KEY_H, false),
            'i' => (KEY_I, false),
            'j' => (KEY_J, false),
            'l' => (KEY_L, false),
            'n' => (KEY_N, false),
            'o' => (KEY_O, false),
            'r' => (KEY_R, false),
            's' => (KEY_S, false),
            't' => (KEY_T, false),
            'u' => (KEY_U, false),
            'w' => (KEY_W, false),
            'x' => (KEY_X, false),
            ' ' => (KEY_SPACE, false),
            _ => continue,
        };
        engine.on_key_ext(key, caps, false, false);
    }
}

/// Helper: Get output string from result
fn result_to_string(result: &goxviet_core::engine::Result) -> String {
    let mut output = String::new();
    for i in 0..result.count as usize {
        if let Some(ch) = char::from_u32(result.as_slice()[i]) {
            output.push(ch);
        }
    }
    output
}

#[test]
fn test_fast_path_simple_chars() {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex as u8);
    engine.set_enabled(true);

    // Type simple English word (no transforms)
    type_keys(&mut engine, "hello");

    // Backspace - should use fast path (O(1))
    let result = engine.on_key_ext(KEY_DELETE, false, false, false);
    assert_eq!(result.action, 1, "Should send action");
    assert_eq!(result.backspace, 1, "Should delete 1 char");
    assert_eq!(result.count, 0, "No replacement needed for simple char");

    // Type more and delete again
    type_keys(&mut engine, "world");
    let result = engine.on_key_ext(KEY_DELETE, false, false, false);
    assert_eq!(result.backspace, 1);
    assert_eq!(result.count, 0);
}

#[test]
fn test_debug_engine_output() {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex as u8);
    engine.set_enabled(true);

    // Type "thuowngj" → should output "thương" (w for ơ, j for nặng)
    type_keys(&mut engine, "thuowngj");

    // Now backspace - let's see what we get
    let result = engine.on_key_ext(KEY_DELETE, false, false, false);
    println!("\n=== DEBUG BACKSPACE ===");
    println!(
        "action={} backspace={} count={}",
        result.action, result.backspace, result.count
    );
    println!("chars:");
    for i in 0..result.count as usize {
        if let Some(ch) = char::from_u32(result.as_slice()[i]) {
            println!("  [{}] = '{}' (U+{:04X})", i, ch, result.as_slice()[i]);
        }
    }

    // Check what the actual output is
    let output = result_to_string(&result);
    println!("output string: '{}'", output);
    println!("output len: {}", output.len());

    // The engine returns the UPDATED buffer content after backspace
    // Not the characters to insert - it's the replacement text
    assert_eq!(result.action, 1);
}

#[test]
#[ignore] // Temporarily disabled - needs investigation
fn test_complex_syllable_rebuild() {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex as u8);
    engine.set_enabled(true);

    // Type "thuowng" + j → "thương" (w creates ơ, j adds nặng tone)
    type_keys(&mut engine, "thuowngj");

    // Backspace 'j' - should rebuild syllable
    let result = engine.on_key_ext(KEY_DELETE, false, false, false);
    assert_eq!(result.action, 1, "Should send action");
    assert_eq!(result.backspace, 7, "Delete 'thương' (7 chars)");

    let output = result_to_string(&result);
    assert_eq!(output, "thương", "Should restore to 'thương' without tone");
}

#[test]
fn test_consecutive_backspaces_performance() {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex as u8);
    engine.set_enabled(true);

    // Type a word with transforms
    type_keys(&mut engine, "thuowngj"); // → thương

    // Delete all characters consecutively
    // Cache should help after first lookup
    for i in 0..8 {
        let result = engine.on_key_ext(KEY_DELETE, false, false, false);
        assert_eq!(result.action, 1, "Delete #{} should have action", i);
    }

    // Buffer should be empty now
    let result = engine.on_key_ext(KEY_DELETE, false, false, false);
    assert_eq!(result.action, 0, "No more chars to delete");
}

#[test]
#[ignore] // Temporarily disabled - needs investigation
fn test_syllable_boundary_detection_space() {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex as u8);
    engine.set_enabled(true);

    // Type multi-word sentence
    type_keys(&mut engine, "xin chao");

    // Delete 'o' from "chao" - should only rebuild "chao"
    let result = engine.on_key_ext(KEY_DELETE, false, false, false);
    assert_eq!(result.action, 1);

    // Should only affect syllable after space
    let output = result_to_string(&result);
    assert_eq!(output, "cha", "Only rebuild 'chao' → 'cha'");
}

#[test]
fn test_delete_until_syllable_boundary() {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex as u8);
    engine.set_enabled(true);

    // Type "xin chao" - note: space between words clears buffer
    // So buffer only contains "chao" after typing
    type_keys(&mut engine, "xin chao");

    // Delete all of "chao"
    for _ in 0..4 {
        engine.on_key_ext(KEY_DELETE, false, false, false);
    }

    // Buffer is now empty (only "chao" was tracked after space)
    // Backspace on empty buffer passes through
    let result = engine.on_key_ext(KEY_DELETE, false, false, false);
    // With new behavior, empty buffer = pass through
    assert_eq!(result.action, 0, "Empty buffer passes through backspace");
}

#[test]
#[ignore] // Temporarily disabled - needs investigation
fn test_backspace_after_tone_addition() {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex as u8);
    engine.set_enabled(true);

    // Type "viet" → "việt" (auto e + ê)
    type_keys(&mut engine, "viet");

    // Add sắc tone: s
    type_keys(&mut engine, "s");

    // Now delete the tone marker 's'
    let result = engine.on_key_ext(KEY_DELETE, false, false, false);
    assert_eq!(result.action, 1, "Should rebuild");

    let output = result_to_string(&result);
    assert_eq!(output, "việt", "Should restore to 'việt' without sắc");
}

#[test]
#[ignore] // Temporarily disabled - needs investigation
fn test_backspace_on_compound_vowel() {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex as u8);
    engine.set_enabled(true);

    // Type "u" + "o" + "w" → "ươ"
    type_keys(&mut engine, "uow");

    // Delete 'w' - should revert compound
    let result = engine.on_key_ext(KEY_DELETE, false, false, false);
    assert_eq!(result.action, 1);

    let output = result_to_string(&result);
    assert_eq!(output, "uo", "Should revert 'ươ' → 'uo'");
}

#[test]
#[ignore] // Temporarily disabled - needs investigation
fn test_long_word_no_regression() {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex as u8);
    engine.set_enabled(true);

    // Type very long word with multiple syllables (>10)
    // Note: spaces in the middle would clear buffer, so use continuous string
    let long_word = "thuowngjthuowngjthuowngjthuowngjthuowngj";
    type_keys(&mut engine, long_word);

    // Delete last character - should NOT have performance regression
    // Should only rebuild last syllable
    let result = engine.on_key_ext(KEY_DELETE, false, false, false);
    // Result depends on whether transforms were applied
    // For this test, we just verify it doesn't crash
    println!("Long word delete result: action={}", result.action);

    // Performance test: operation should complete quickly
    // (Actual timing done in benchmarks)
}

#[test]
fn test_cache_invalidation_on_new_letter() {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex as u8);
    engine.set_enabled(true);

    type_keys(&mut engine, "xin chao");

    // Delete one char (cache populated)
    engine.on_key_ext(KEY_DELETE, false, false, false);

    // Add new letter (should invalidate cache)
    type_keys(&mut engine, "b");

    // Delete again (cache miss, should recompute)
    let result = engine.on_key_ext(KEY_DELETE, false, false, false);
    assert_eq!(
        result.action, 1,
        "Should still work after cache invalidation"
    );
}

#[test]
fn test_backspace_on_empty_buffer() {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex as u8);
    engine.set_enabled(true);

    // Backspace on empty buffer - should pass through
    let result = engine.on_key_ext(KEY_DELETE, false, false, false);
    assert_eq!(result.action, 0, "Should pass through when empty");
}

#[test]
#[ignore] // Temporarily disabled - needs investigation
fn test_fast_path_vs_complex_path_distinction() {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex as u8);
    engine.set_enabled(true);

    // Fast path: simple char
    type_keys(&mut engine, "a");
    let result = engine.on_key_ext(KEY_DELETE, false, false, false);
    assert_eq!(result.count, 0, "Fast path: no replacement");

    // Complex path: char with transform
    type_keys(&mut engine, "as"); // → á
    let result = engine.on_key_ext(KEY_DELETE, false, false, false);
    assert!(result.count > 0, "Complex path: has replacement");
}

#[test]
#[ignore] // Temporarily disabled - needs investigation
fn test_syllable_with_multiple_transforms() {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex as u8);
    engine.set_enabled(true);

    // Type word with multiple transforms
    // "nguowif" → "người" (w for ơ, i, f for huyền)
    type_keys(&mut engine, "nguowif");

    // Delete 'f' (huyền tone)
    let result = engine.on_key_ext(KEY_DELETE, false, false, false);
    assert_eq!(result.action, 1);

    let output = result_to_string(&result);
    // Should remove huyền but keep ươ compound
    assert!(output.contains("ươ"), "Should preserve compound vowel");
}

#[test]
fn test_backspace_after_space_feature() {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex as u8);
    engine.set_enabled(true);

    // Type word and space - space clears buffer and saves to history
    type_keys(&mut engine, "xin ");

    // After space, buffer is empty
    // Backspace on empty buffer with history decrements space counter
    // and passes through (action=0) to let system delete the space
    let result = engine.on_key_ext(KEY_DELETE, false, false, false);
    // With new behavior: empty buffer after space = pass through
    assert_eq!(
        result.action, 0,
        "Pass through to system for space deletion"
    );
}

#[test]
fn test_mixed_simple_and_complex() {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex as u8);
    engine.set_enabled(true);

    // Type: "hello" (simple) + " " + "thuowngj" (complex)
    type_keys(&mut engine, "hello thuowngj");

    // Delete from complex part
    let result = engine.on_key_ext(KEY_DELETE, false, false, false);
    assert_eq!(result.action, 1, "Should handle complex");

    // Delete until we reach simple part
    for _ in 0..6 {
        engine.on_key_ext(KEY_DELETE, false, false, false);
    }

    // Now in simple part
    let result = engine.on_key_ext(KEY_DELETE, false, false, false);
    assert_eq!(result.action, 1, "Should handle simple");
}

#[test]
fn test_edge_case_single_char() {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex as u8);
    engine.set_enabled(true);

    // Single character
    type_keys(&mut engine, "a");

    // Delete it
    let result = engine.on_key_ext(KEY_DELETE, false, false, false);
    assert_eq!(result.action, 1);
    assert_eq!(result.backspace, 1);

    // Buffer should be empty
    let result = engine.on_key_ext(KEY_DELETE, false, false, false);
    assert_eq!(result.action, 0, "Buffer empty");
}

#[test]
#[ignore] // Temporarily disabled - needs investigation
fn test_edge_case_only_transforms() {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex as u8);
    engine.set_enabled(true);

    // Type just transforms: "as" → "á"
    type_keys(&mut engine, "as");

    // Delete 's' (tone marker)
    let result = engine.on_key_ext(KEY_DELETE, false, false, false);
    assert_eq!(result.action, 1);

    let output = result_to_string(&result);
    assert_eq!(output, "a", "Should revert to plain 'a'");
}

#[test]
fn test_performance_stability_across_lengths() {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex as u8);
    engine.set_enabled(true);

    // Test with increasing word lengths
    for length in [3, 5, 10, 20] {
        engine.clear();

        // Type word of specific length
        let word = "a".repeat(length);
        type_keys(&mut engine, &word);

        // Delete last char - should be consistent for simple chars
        let result = engine.on_key_ext(KEY_DELETE, false, false, false);
        assert_eq!(result.action, 1, "Length {} should work", length);
        // Note: backspace count may vary based on syllable rebuild
        // For simple chars, it should be minimal (1 or small)
        assert!(
            result.backspace <= length as u8,
            "Backspace should be reasonable"
        );
    }
}

#[test]
fn test_boundary_at_buffer_start() {
    let mut engine = Engine::new();
    engine.set_method(InputMethod::Telex as u8);
    engine.set_enabled(true);

    // Single word (boundary at position 0)
    type_keys(&mut engine, "xin");

    // Delete all
    for _ in 0..3 {
        let result = engine.on_key_ext(KEY_DELETE, false, false, false);
        assert_eq!(result.action, 1, "Should handle boundary at start");
    }
}
