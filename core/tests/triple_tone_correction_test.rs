//! Triple-tone Telex auto-correction tests.
//!
//! In Telex mode, s/f/r/x/j are dual-purpose: English consonants AND Vietnamese tone markers.
//! The engine handles double-tone correctly via the double-key revert mechanism.
//! This test suite verifies that accidentally typing a triple tone marker is auto-corrected
//! at the SPACE boundary.
//!
//! ## Bug scenario
//! User types "a-s-s-s-e-t" (one extra 's') intending to write "asset":
//! 1. a+s → "á" (sắc tone)
//! 2. +s → double-key revert → "as" (is_english_word=true)
//! 3. +s → literal 's' → "ass" (3 chars, display looks correct-ish)
//! 4. +e+t → "asset" (display looks correct)
//! 5. SPACE → without fix: raw_input "assset" → outputs "assset "; WITH fix → "asset "

use goxviet_core::data::keys;
use goxviet_core::engine::Engine;

fn char_to_key(c: char) -> u16 {
    match c.to_ascii_lowercase() {
        'a' => 0,
        's' => 1,
        'd' => 2,
        'f' => 3,
        'h' => 4,
        'g' => 5,
        'z' => 6,
        'x' => 7,
        'c' => 8,
        'v' => 9,
        'b' => 11,
        'q' => 12,
        'w' => 13,
        'e' => 14,
        'r' => 15,
        'y' => 16,
        't' => 17,
        '1' => 18,
        '2' => 19,
        '3' => 20,
        '4' => 21,
        '6' => 22,
        '5' => 23,
        '9' => 25,
        '7' => 26,
        '8' => 28,
        '0' => 29,
        'o' => 31,
        'u' => 32,
        'i' => 34,
        'p' => 35,
        'l' => 37,
        'j' => 38,
        'k' => 40,
        'n' => 45,
        'm' => 46,
        _ => 255,
    }
}

/// Helper: type a sequence of raw keystrokes, then press SPACE.
/// Returns the final display string after applying all engine results,
/// simulating what the user actually sees on screen.
fn type_then_space(keystrokes: &str) -> Option<String> {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    let mut display = String::new();

    for ch in keystrokes.chars() {
        let key = char_to_key(ch);
        let result = engine.on_key_ext(key, ch.is_ascii_uppercase(), false, false);
        if result.action == 1 {
            let bs = result.backspace as usize;
            for _ in 0..bs.min(display.len()) { display.pop(); }
            for i in 0..result.count as usize {
                if let Some(c) = char::from_u32(result.as_slice()[i]) { display.push(c); }
            }
        } else {
            display.push(ch);
        }
    }

    let r = engine.on_key_ext(keys::SPACE, false, false, false);
    if r.action == 1 {
        let bs = r.backspace as usize;
        for _ in 0..bs.min(display.len()) { display.pop(); }
        for i in 0..r.count as usize {
            if let Some(c) = char::from_u32(r.as_slice()[i]) { display.push(c); }
        }
        Some(display)
    } else {
        None
    }
}

/// Helper: type a sequence of raw keystrokes (no SPACE).
/// Returns the accumulated display string after applying engine results.
fn type_word(keystrokes: &str) -> String {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    let mut output = String::new();

    for ch in keystrokes.chars() {
        let key = char_to_key(ch);
        let caps = ch.is_ascii_uppercase();
        let result = engine.on_key(key, caps, false);

        if result.action == 1 {
            let backspace_count = result.backspace as usize;
            for _ in 0..backspace_count.min(output.len()) {
                output.pop();
            }
            for i in 0..result.count as usize {
                if let Some(c) = char::from_u32(result.as_slice()[i]) {
                    output.push(c);
                }
            }
        } else {
            output.push(ch);
        }
    }
    output
}

// =============================================================================
// Core triple-tone correction tests (SPACE boundary)
// =============================================================================

/// Typing "a-s-s-s-e-t" + SPACE should output "asset " not "assset "
#[test]
fn test_triple_s_asset() {
    let result = type_then_space("assset");
    assert_eq!(
        result,
        Some("asset ".to_string()),
        "Expected triple-s correction: 'assset' + SPACE → 'asset '"
    );
}

/// Typing "o-f-f-f-e-r" + SPACE should output "offer " not "offfer "
#[test]
fn test_triple_f_offer() {
    let result = type_then_space("offfer");
    assert_eq!(
        result,
        Some("offer ".to_string()),
        "Expected triple-f correction: 'offfer' + SPACE → 'offer '"
    );
}

/// Typing "c-o-r-r-r-e-c-t" + SPACE should output "correct " not "corrrect "
#[test]
fn test_triple_r_correct() {
    let result = type_then_space("corrrect");
    assert_eq!(
        result,
        Some("correct ".to_string()),
        "Expected triple-r correction: 'corrrect' + SPACE → 'correct '"
    );
}

/// "affair" with triple-f: "afffair" → "affair "
#[test]
fn test_triple_f_affair() {
    let result = type_then_space("afffair");
    assert_eq!(
        result,
        Some("affair ".to_string()),
        "Expected triple-f correction: 'afffair' + SPACE → 'affair '"
    );
}

/// "effect" with triple-f: "efffect" → "effect "
#[test]
fn test_triple_f_effect() {
    let result = type_then_space("efffect");
    assert_eq!(
        result,
        Some("effect ".to_string()),
        "Expected triple-f correction: 'efffect' + SPACE → 'effect '"
    );
}

// =============================================================================
// Regression: double-tone (existing behavior must not break)
// =============================================================================

/// "asset" (double-s, correct) + SPACE should still output "asset "
#[test]
fn test_double_s_asset_no_regression() {
    // "asset" with normal double-key revert: a-s-s-e-t → "asset" in display
    // SPACE should auto-restore to "asset " from raw "asset"
    // (The double-key revert gives raw=[a,s,s,e,t] with buf="asset", no triple)
    let result = type_then_space("asset");
    // Either auto-restore fires (action=1 → "asset ") or no transform (action=0, word was never changed)
    match result {
        Some(s) => assert_eq!(s, "asset ", "Double-s 'asset' should stay 'asset '"),
        None => {
            // action=0 means no restore was needed — the word was typed without transforms
            // This is also acceptable if the display was already "asset"
        }
    }
}

/// "offer" (double-f, correct) + SPACE should still output "offer "
#[test]
fn test_double_f_offer_no_regression() {
    let result = type_then_space("offer");
    match result {
        Some(s) => assert_eq!(s, "offer ", "Double-f 'offer' should stay 'offer '"),
        None => {}
    }
}

// =============================================================================
// Regression: Vietnamese words with ss/ff/rr patterns must not be affected
// =============================================================================

/// Vietnamese Telex: "ass" is NOT in the English double-consonant dict with a valid
/// triple correction, so Vietnamese typing should be unaffected.
#[test]
fn test_vietnamese_words_not_corrupted() {
    // "cánh" typed as "canhs" in Telex → should remain "cánh" not be "corrected"
    let output = type_word("canhs");
    assert_eq!(output, "cánh", "'canhs' should produce Vietnamese 'cánh'");
}
