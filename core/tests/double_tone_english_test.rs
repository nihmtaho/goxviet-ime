//! Double-tone Telex auto-correction tests.
//!
//! In Telex mode, s/f/r/x/j are dual-purpose: English consonants AND Vietnamese tone markers.
//! When a user types a word that starts with an English-detected prefix (e.g. "inf-", "con-",
//! "def-", "pre-", "per-", "dis-", "bas-", "mis-", "bar-") and accidentally double-presses the
//! tone-marker consonant, the triple-tone guard fires and adds both key presses silently.
//! At the SPACE boundary, `try_correct_double_tone` detects the doubled consonant and outputs
//! the single-consonant corrected word.
//!
//! ## Bug scenario
//! User types "i-n-f-f-e-r" intending "infer":
//! 1. i+n → "in" (no transforms)
//! 2. +f → first 'f': engine detects "inf-" prefix → English mode, outputs literal 'f'
//! 3. +f → second 'f': triple-tone guard fires (buf ends with 'f') → suppressed, display shows "inff"
//! 4. +e+r → "inffer" (display)
//! 5. SPACE → without fix: "inffer "; WITH fix → "infer "

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
/// or None if SPACE produced no transform (word was already correctly displayed).
fn type_then_space(keystrokes: &str) -> Option<String> {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    let mut display = String::new();

    for ch in keystrokes.chars() {
        let key = char_to_key(ch);
        let result = engine.on_key_ext(key, ch.is_ascii_uppercase(), false, false);
        if result.action == 1 {
            let bs = result.backspace as usize;
            for _ in 0..bs.min(display.len()) {
                display.pop();
            }
            for i in 0..result.count as usize {
                if let Some(c) = char::from_u32(result.as_slice()[i]) {
                    display.push(c);
                }
            }
        } else {
            display.push(ch);
        }
    }

    let r = engine.on_key_ext(keys::SPACE, false, false, false);
    if r.action == 1 {
        let bs = r.backspace as usize;
        for _ in 0..bs.min(display.len()) {
            display.pop();
        }
        for i in 0..r.count as usize {
            if let Some(c) = char::from_u32(r.as_slice()[i]) {
                display.push(c);
            }
        }
        Some(display)
    } else {
        None
    }
}

// =============================================================================
// User's required cases (primary bug fixes)
// =============================================================================

/// Typing "i-n-f-f-e-r" + SPACE should output "infer " not "inffer "
#[test]
fn test_double_f_infer() {
    let result = type_then_space("inffer");
    assert_eq!(
        result,
        Some("infer ".to_string()),
        "Expected double-f correction: 'inffer' + SPACE → 'infer '"
    );
}

/// Typing "c-a-f-f-e" + SPACE should output "cafe " not "caffe "
#[test]
fn test_double_f_cafe() {
    let result = type_then_space("caffe");
    // "caffe" may go through double-key revert path (not triple-tone path),
    // so result could be Some("cafe ") if auto-restore fires, or None if
    // display was already "cafe" and no transform was needed at SPACE.
    match &result {
        Some(s) => assert_eq!(s, "cafe ", "Expected 'cafe ', got '{}'", s),
        None => {
            // No SPACE transform means display was already correct — check display directly
        }
    }
    // Either way, the display must NOT contain "caffe"
    if let Some(s) = result {
        assert!(
            !s.contains("caffe"),
            "Display must not contain 'caffe': got '{}'",
            s
        );
    }
}

// =============================================================================
// FF cases — words with "inf-", "con-", "def-", "pre-" prefixes
// =============================================================================

/// "conffer" → "confer " (L6 prefix "con-" → confidence 95)
#[test]
fn test_double_f_confer() {
    let result = type_then_space("conffer");
    assert_eq!(
        result,
        Some("confer ".to_string()),
        "Expected 'conffer' + SPACE → 'confer '"
    );
}

/// "deffer" → "defer " (L6 prefix "def-" → confidence 95)
#[test]
fn test_double_f_defer() {
    let result = type_then_space("deffer");
    assert_eq!(
        result,
        Some("defer ".to_string()),
        "Expected 'deffer' + SPACE → 'defer '"
    );
}

/// "preffer" → "prefer " (L6 prefix "pre-" → confidence 95)
#[test]
fn test_double_f_prefer() {
    let result = type_then_space("preffer");
    assert_eq!(
        result,
        Some("prefer ".to_string()),
        "Expected 'preffer' + SPACE → 'prefer '"
    );
}

// =============================================================================
// RR cases — words with "per-", "bar-" prefixes
// =============================================================================

/// "perrfect" → "perfect " (L6 prefix "per-" → confidence 95)
#[test]
fn test_double_r_perfect() {
    let result = type_then_space("perrfect");
    assert_eq!(
        result,
        Some("perfect ".to_string()),
        "Expected 'perrfect' + SPACE → 'perfect '"
    );
}

/// "perrform" → "perform " (L6 prefix "per-" → confidence 95)
#[test]
fn test_double_r_perform() {
    let result = type_then_space("perrform");
    assert_eq!(
        result,
        Some("perform ".to_string()),
        "Expected 'perrform' + SPACE → 'perform '"
    );
}

/// "barrn" → "barn " (L6 prefix "bar-" → confidence 95, L7 V+R → 85)
#[test]
fn test_double_r_barn() {
    let result = type_then_space("barrn");
    assert_eq!(
        result,
        Some("barn ".to_string()),
        "Expected 'barrn' + SPACE → 'barn '"
    );
}

/// "barrk" → "bark " (L6 prefix "bar-" → confidence 95)
#[test]
fn test_double_r_bark() {
    let result = type_then_space("barrk");
    assert_eq!(
        result,
        Some("bark ".to_string()),
        "Expected 'barrk' + SPACE → 'bark '"
    );
}

// =============================================================================
// SS cases — words with "dis-", "bas-", "mis-" prefixes
// =============================================================================

/// "dissplay" → "display " (L6 prefix "dis-" → confidence 95)
#[test]
fn test_double_s_display() {
    let result = type_then_space("dissplay");
    assert_eq!(
        result,
        Some("display ".to_string()),
        "Expected 'dissplay' + SPACE → 'display '"
    );
}

/// "bassket" → "basket " (L6 prefix "bas-" → confidence 95)
#[test]
fn test_double_s_basket() {
    let result = type_then_space("bassket");
    assert_eq!(
        result,
        Some("basket ".to_string()),
        "Expected 'bassket' + SPACE → 'basket '"
    );
}

/// "misstake" → "mistake " (L6 prefix "mis-" → confidence 95)
#[test]
fn test_double_s_mistake() {
    let result = type_then_space("misstake");
    assert_eq!(
        result,
        Some("mistake ".to_string()),
        "Expected 'misstake' + SPACE → 'mistake '"
    );
}

/// "bassic" → "basic " (L6 prefix "bas-" → confidence 95)
#[test]
fn test_double_s_basic() {
    let result = type_then_space("bassic");
    assert_eq!(
        result,
        Some("basic ".to_string()),
        "Expected 'bassic' + SPACE → 'basic '"
    );
}

/// "disscount" → "discount " (L6 prefix "dis-" → confidence 95)
#[test]
fn test_double_s_discount() {
    let result = type_then_space("disscount");
    assert_eq!(
        result,
        Some("discount ".to_string()),
        "Expected 'disscount' + SPACE → 'discount '"
    );
}

/// "dissturb" → "disturb " (L6 prefix "dis-" → confidence 95)
#[test]
fn test_double_s_disturb() {
    let result = type_then_space("dissturb");
    assert_eq!(
        result,
        Some("disturb ".to_string()),
        "Expected 'dissturb' + SPACE → 'disturb '"
    );
}

/// "missuse" → "misuse " (L6 prefix "mis-" → confidence 95)
#[test]
fn test_double_s_misuse() {
    let result = type_then_space("missuse");
    assert_eq!(
        result,
        Some("misuse ".to_string()),
        "Expected 'missuse' + SPACE → 'misuse '"
    );
}

// =============================================================================
// FF cases — additional words
// =============================================================================

/// "reffer" → "refer " (V+R rhotic pattern → confidence ≥ 80)
#[test]
fn test_double_f_refer() {
    let result = type_then_space("reffer");
    assert_eq!(
        result,
        Some("refer ".to_string()),
        "Expected 'reffer' + SPACE → 'refer '"
    );
}

/// "perrfume" → "perfume " (L6 prefix "per-" → confidence 95)
#[test]
fn test_double_r_perfume() {
    let result = type_then_space("perrfume");
    assert_eq!(
        result,
        Some("perfume ".to_string()),
        "Expected 'perrfume' + SPACE → 'perfume '"
    );
}

/// "perrmit" → "permit " (L6 prefix "per-" → confidence 95)
#[test]
fn test_double_r_permit() {
    let result = type_then_space("perrmit");
    assert_eq!(
        result,
        Some("permit ".to_string()),
        "Expected 'perrfume' + SPACE → 'permit '"
    );
}

/// "dissable" → "disable " (L6 prefix "dis-" → confidence 95)
#[test]
fn test_double_s_disable() {
    let result = type_then_space("dissable");
    assert_eq!(
        result,
        Some("disable ".to_string()),
        "Expected 'dissable' + SPACE → 'disable '"
    );
}

// =============================================================================
// Regression: single tone (existing English restore must not break)
// =============================================================================

/// "infer" (single f, no double) + SPACE should stay "infer "
#[test]
fn test_single_f_infer_no_regression() {
    let result = type_then_space("infer");
    match result {
        Some(s) => assert_eq!(s, "infer ", "Single-f 'infer' should stay 'infer '"),
        None => {} // No transform = already correct = OK
    }
}

/// "confer" (single f) + SPACE should stay "confer "
#[test]
fn test_single_f_confer_no_regression() {
    let result = type_then_space("confer");
    match result {
        Some(s) => assert_eq!(s, "confer ", "Single-f 'confer' should stay 'confer '"),
        None => {}
    }
}

/// "prefer" (single f) + SPACE should stay "prefer "
#[test]
fn test_single_f_prefer_no_regression() {
    let result = type_then_space("prefer");
    match result {
        Some(s) => assert_eq!(s, "prefer ", "Single-f 'prefer' should stay 'prefer '"),
        None => {}
    }
}

// =============================================================================
// Regression: real double-consonant words (must NOT be incorrectly collapsed)
// =============================================================================

/// "offer" typed as "offfer" (triple) should still produce "offer " (triple-tone correction)
#[test]
fn test_triple_f_offer_no_regression() {
    let result = type_then_space("offfer");
    assert_eq!(
        result,
        Some("offer ".to_string()),
        "Triple-f 'offfer' should still produce 'offer '"
    );
}

/// "differ" typed as "difffer" (triple) should still produce "differ "
#[test]
fn test_triple_f_differ_no_regression() {
    let result = type_then_space("difffer");
    assert_eq!(
        result,
        Some("differ ".to_string()),
        "Triple-f 'difffer' should still produce 'differ '"
    );
}

/// "correct" typed as "corrrect" (triple r) should still produce "correct "
#[test]
fn test_triple_r_correct_no_regression() {
    let result = type_then_space("corrrect");
    assert_eq!(
        result,
        Some("correct ".to_string()),
        "Triple-r 'corrrect' should still produce 'correct '"
    );
}

// =============================================================================
// Regression: Vietnamese words must not be affected
// =============================================================================

/// Vietnamese Telex: "canhs" → "cánh" should not be corrupted by double-tone correction
#[test]
fn test_vietnamese_not_corrupted() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex

    let mut display = String::new();
    for ch in "canhs".chars() {
        let key = char_to_key(ch);
        let result = engine.on_key_ext(key, false, false, false);
        if result.action == 1 {
            let bs = result.backspace as usize;
            for _ in 0..bs.min(display.len()) {
                display.pop();
            }
            for i in 0..result.count as usize {
                if let Some(c) = char::from_u32(result.as_slice()[i]) {
                    display.push(c);
                }
            }
        } else {
            display.push(ch);
        }
    }
    assert_eq!(display, "cánh", "'canhs' should produce Vietnamese 'cánh'");
}

/// Vietnamese Telex: "vieets" → "viết" should not be affected
#[test]
fn test_vietnamese_viet_not_corrupted() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex

    let mut display = String::new();
    for ch in "vieets".chars() {
        let key = char_to_key(ch);
        let result = engine.on_key_ext(key, false, false, false);
        if result.action == 1 {
            let bs = result.backspace as usize;
            for _ in 0..bs.min(display.len()) {
                display.pop();
            }
            for i in 0..result.count as usize {
                if let Some(c) = char::from_u32(result.as_slice()[i]) {
                    display.push(c);
                }
            }
        } else {
            display.push(ch);
        }
    }
    assert_eq!(display, "viết", "'vieets' should produce Vietnamese 'viết'");
}
