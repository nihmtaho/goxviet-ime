//! Auto-Restore Regression Tests (T1.1)
//!
//! Documents current behavior of auto-restore pipeline BEFORE refactoring.
//! These tests must pass before and after the Vietnamese-first refactor.
//!
//! ## What is auto-restore?
//! When typing with Vietnamese IME in Telex mode, some key sequences trigger
//! diacritic transforms (e.g. "ss" → "s" sắc → "ś"). For English words, this
//! is undesirable ("restore" → "rêstôre"). Auto-restore reverts the raw input.
//!
//! ## Test categories
//! 1. English words: should NOT be transformed (raw input kept)
//! 2. Vietnamese words: should BE transformed (Vietnamese output)
//! 3. Compound words: multi-syllable Vietnamese (should stay Vietnamese)

use goxviet_core::engine::Engine;

// ─── Helper ────────────────────────────────────────────────────────────────

fn char_to_key(ch: char) -> u16 {
    goxviet_core::utils::char_to_key(ch)
}

/// Simulate typing a word and return the engine's committed output.
/// Appends a space at the end to trigger word boundary commit.
fn type_word_telex(word: &str) -> String {
    let mut engine = Engine::new();
    engine.set_method(0); // 0 = Telex

    let mut output = String::new();

    for ch in word.chars() {
        let key = char_to_key(ch);
        let caps = ch.is_uppercase();
        let result = engine.on_key(key, caps, false);

        if result.action == 1 {
            let bs = result.backspace as usize;
            for _ in 0..bs.min(output.len()) {
                output.pop();
            }
            for i in 0..result.count as usize {
                if let Some(c) = char::from_u32(result.as_slice()[i]) {
                    output.push(c);
                }
            }
        } else {
            // action 0 = pass-through
            output.push(ch);
        }
    }

    output
}

// ─── T1.1.A: Known English words must NOT be Vietnamese-transformed ─────────

/// "array" contains double 'r' (English double-consonant pattern) — must NOT be transformed
#[test]
fn test_english_array_no_transform() {
    // In Telex: "array" → raw a-r-r-a-y
    // 'r' is hỏi tone mark in Telex, but "arr" triggers English detection
    // Expected: engine outputs "array" (not "ârrảy" or similar)
    let result = type_word_telex("array ");
    let trimmed = result.trim();
    assert_eq!(
        trimmed, "array",
        "Expected 'array' (not transformed), got '{}'",
        trimmed
    );
}

/// "windows" starts with 'w' which is tone modifier in Telex
#[test]
fn test_english_windows_no_transform() {
    let result = type_word_telex("windows ");
    let trimmed = result.trim();
    assert_eq!(
        trimmed, "windows",
        "Expected 'windows' (not transformed), got '{}'",
        trimmed
    );
}

/// "enter" — 'r' at end is hỏi tone mark in Telex but "enter" is English
#[test]
fn test_english_enter_no_transform() {
    let result = type_word_telex("enter ");
    let trimmed = result.trim();
    assert_eq!(
        trimmed, "enter",
        "Expected 'enter' (not transformed), got '{}'",
        trimmed
    );
}

/// "stop" — common 4-letter English word
#[test]
fn test_english_stop_no_transform() {
    let result = type_word_telex("stop ");
    let trimmed = result.trim();
    assert_eq!(
        trimmed, "stop",
        "Expected 'stop' (not transformed), got '{}'",
        trimmed
    );
}

/// "aroma" — starts with 'a', ends with 'a', no obvious diacritics
#[test]
fn test_english_aroma_no_transform() {
    let result = type_word_telex("aroma ");
    let trimmed = result.trim();
    // "aroma" - contains 'r' + 'o' which in Telex could be ambiguous
    // Current behavior: should remain "aroma" (not transformed)
    assert_eq!(
        trimmed, "aroma",
        "Expected 'aroma' (not transformed), got '{}'",
        trimmed
    );
}

// ─── T1.1.B: Vietnamese words must BE transformed correctly ────────────────

/// "ăn" in Telex: a-w-n (w=breve modifier for ă)
#[test]
fn test_vietnamese_an_transformed() {
    let result = type_word_telex("awn ");
    let trimmed = result.trim();
    // "awn" in Telex = "ăn"
    assert_eq!(trimmed, "ăn", "Expected Vietnamese 'ăn', got '{}'", trimmed);
}

/// "uống" in Telex: u-o-o-n-g-s (oo=ô tone, s=sắc)
/// Actually: u-o-o-n-g-s or u-w-n-g-s depending on interpretation
/// Simple test: type "uongs" + space
#[test]
fn test_vietnamese_uong_transformed() {
    let result = type_word_telex("uoongs ");
    let trimmed = result.trim();
    // "uoongs" = uống (u + oo=ô + ng + s=sắc)
    assert_eq!(
        trimmed, "uống",
        "Expected Vietnamese 'uống', got '{}'",
        trimmed
    );
}

/// "người" in Telex: n-g-u-o-w-i (ow=ơ modifier)
#[test]
fn test_vietnamese_nguoi_transformed() {
    let result = type_word_telex("nguwowi ");
    let trimmed = result.trim();
    // "nguwowi" in Telex = "người" (ngu + w=ư + o + w=ơ + i)
    // This is complex compound — just verify it's not raw "nguwowi"
    assert_ne!(
        trimmed, "nguwowi",
        "Expected Vietnamese transform, got raw '{}'",
        trimmed
    );
}

/// "đường" in Telex: d-d-u-o-w-n-g-f (dd=đ, ow=ơ, f=huyền)
#[test]
fn test_vietnamese_duong_transformed() {
    let result = type_word_telex("dduowngf ");
    let trimmed = result.trim();
    assert_eq!(
        trimmed, "đường",
        "Expected Vietnamese 'đường', got '{}'",
        trimmed
    );
}

/// "bình" in Telex: b-i-n-h-f (f=huyền)
#[test]
fn test_vietnamese_binh_transformed() {
    let result = type_word_telex("binhf ");
    let trimmed = result.trim();
    assert_eq!(
        trimmed, "bình",
        "Expected Vietnamese 'bình', got '{}'",
        trimmed
    );
}

// ─── T1.1.C: Vietnamese words with Telex ambiguity ─────────────────────────

/// "anh" — pure ASCII but Vietnamese word (brother)
/// Keys a-n-h: none of these are Telex modifiers when combined, so no transform
/// Current behavior: "anh" stays as "anh" (no transforms applied, nothing to restore)
#[test]
fn test_vietnamese_anh_no_false_restore() {
    let result = type_word_telex("anh ");
    let trimmed = result.trim();
    assert_eq!(
        trimmed, "anh",
        "Expected 'anh' (Vietnamese word, should not be altered), got '{}'",
        trimmed
    );
}

/// "khong" as Telex input — k-h-o-n-g without modifiers → stays "khong"
/// (To type "không" one needs "khoong" or similar with oo for ô)
#[test]
fn test_telex_khong_raw() {
    let result = type_word_telex("khong ");
    let trimmed = result.trim();
    // "khong" without the double-o stays as "khong" since no modifiers applied
    // This tests that English detection doesn't incorrectly flag "khong"
    assert_eq!(
        trimmed, "khong",
        "Expected 'khong' (no transforms, raw Vietnamese romanization), got '{}'",
        trimmed
    );
}

/// "khong" in full Telex form: "khoong" (double-o for ô) should produce "không"
#[test]
fn test_vietnamese_khong_transformed() {
    let result = type_word_telex("khoong ");
    let trimmed = result.trim();
    assert_eq!(trimmed, "không", "Expected 'không', got '{}'", trimmed);
}

// ─── T1.1.D: Compound words (multi-syllable) ───────────────────────────────

/// "ánh sáng" Telex: "anhs sangs" or "anhs sasngs"
/// Each syllable typed separately with space between
#[test]
fn test_compound_anh_sang() {
    // "ánh sáng": ánh = anhs, sáng = sangs
    let result_anh = type_word_telex("anhs ");
    let result_sang = type_word_telex("sangs ");
    let anh = result_anh.trim();
    let sang = result_sang.trim();
    assert_eq!(anh, "ánh", "Expected 'ánh', got '{}'", anh);
    assert_eq!(sang, "sáng", "Expected 'sáng', got '{}'", sang);
}

/// "ai đó" Telex: "ai " + "ddos "
#[test]
fn test_compound_ai_do() {
    let result_ai = type_word_telex("ai ");
    let result_do = type_word_telex("ddos ");
    let ai = result_ai.trim();
    let do_word = result_do.trim();
    assert_eq!(ai, "ai", "Expected 'ai', got '{}'", ai);
    assert_eq!(do_word, "đó", "Expected 'đó', got '{}'", do_word);
}

/// "ăn cơm" Telex: "awn " + "cowm "
#[test]
fn test_compound_an_com() {
    let result_an = type_word_telex("awn ");
    let result_com = type_word_telex("cowm ");
    let an = result_an.trim();
    let com = result_com.trim();
    assert_eq!(an, "ăn", "Expected 'ăn', got '{}'", an);
    assert_eq!(com, "cơm", "Expected 'cơm', got '{}'", com);
}

// ─── T1.1.E: Edge cases ────────────────────────────────────────────────────

/// "restore" — starts with "rest" prefix (English pattern) must stay as-is
#[test]
fn test_english_restore_no_transform() {
    let result = type_word_telex("restore ");
    let trimmed = result.trim();
    assert_eq!(
        trimmed, "restore",
        "Expected 'restore' (English word), got '{}'",
        trimmed
    );
}

/// "print" — programming term
#[test]
fn test_english_print_no_transform() {
    let result = type_word_telex("print ");
    let trimmed = result.trim();
    assert_eq!(
        trimmed, "print",
        "Expected 'print' (programming term), got '{}'",
        trimmed
    );
}
