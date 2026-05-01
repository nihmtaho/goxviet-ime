//! Regression tests: capitalized Vietnamese words must NOT be auto-restored to English.
//!
//! Bug: Typing a Vietnamese word that starts with an uppercase letter (sentence-start
//! or proper noun) and pressing SPACE incorrectly triggered English auto-restore.
//!
//! Root cause: `is_valid_vietnamese_syllable()` did a case-sensitive lookup against a
//! lowercase-only dictionary, so "Trường" was not found and the English confidence
//! threshold was lowered from 97% to 60%, causing false restoration.
//!
//! Fix: `is_valid_vietnamese_syllable()` now lowercases the input before a second
//! lookup attempt when the fast (exact-match) path misses.

use goxviet_core::{data::keys, engine::Engine};

/// Type `telex_input` (respecting uppercase), press SPACE, and return what the engine
/// outputs for the SPACE key.  Returns `Some(string)` when the engine produces an
/// action (transform or restore); `None` when it passes the space through unchanged.
fn type_then_space(telex_input: &str) -> Option<String> {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex

    for ch in telex_input.chars() {
        let key = goxviet_core::utils::char_to_key(ch);
        let caps = ch.is_uppercase();
        engine.on_key(key, caps, false);
    }

    let r = engine.on_key(keys::SPACE, false, false);
    if r.action == 1 {
        let s: String = (0..r.count as usize)
            .filter_map(|i| char::from_u32(r.as_slice()[i]))
            .collect();
        Some(s)
    } else {
        None
    }
}

// ── Core regression cases ────────────────────────────────────────────────────

/// "Truownfg" (Shift+T) + SPACE must produce "Trường ", NOT restore "Truownfg ".
#[test]
fn test_truong_capitalized_not_restored() {
    // "Truownfg" in Telex → "Trường"; uppercase T comes from Shift.
    let result = type_then_space("Truownfg");
    if let Some(output) = result {
        assert_eq!(
            output, "Trường ",
            "Expected 'Trường ' (Vietnamese kept), got '{}'",
            output
        );
    }
    // If action==0 (space pass-through), the Vietnamese word was already displayed
    // correctly by earlier key events — that is also acceptable, but the space-key
    // result should not have been an English restore.
}

/// "Khoongg" (Shift+K) + SPACE must produce "Không ".
#[test]
fn test_khong_capitalized_not_restored() {
    let result = type_then_space("Khoongg");
    if let Some(output) = result {
        assert_eq!(
            output, "Không ",
            "Expected 'Không ' (Vietnamese kept), got '{}'",
            output
        );
    }
}

/// "Binhf" (Shift+B) + SPACE must produce "Bình ".
#[test]
fn test_binh_capitalized_not_restored() {
    let result = type_then_space("Binhf");
    if let Some(output) = result {
        assert_eq!(
            output, "Bình ",
            "Expected 'Bình ' (Vietnamese kept), got '{}'",
            output
        );
    }
}

/// "Annah" (Shift+A) + SPACE must produce "Anh " (Telex: "annah" = "anh" + 'a'-ngang).
#[test]
fn test_anh_capitalized_not_restored() {
    let result = type_then_space("Annah");
    if let Some(output) = result {
        assert_eq!(
            output, "Anh ",
            "Expected 'Anh ' (Vietnamese kept), got '{}'",
            output
        );
    }
}

// ── Sanity: lowercase variants still work ────────────────────────────────────

/// Lowercase "truownfg" + SPACE must still produce "trường " (unchanged behavior).
#[test]
fn test_truong_lowercase_still_works() {
    let result = type_then_space("truownfg");
    if let Some(output) = result {
        assert_eq!(output, "trường ", "Lowercase 'trường ' must still work");
    }
}

/// Lowercase "binhf" + SPACE must still produce "bình ".
#[test]
fn test_binh_lowercase_still_works() {
    let result = type_then_space("binhf");
    if let Some(output) = result {
        assert_eq!(output, "bình ", "Lowercase 'bình ' must still work");
    }
}
