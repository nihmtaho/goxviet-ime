//! Regression tests for three core engine improvements:
//!
//! 1. "coẻ" → "core": invalid Vietnamese compound restored at word boundary
//! 2. "uu" → "ưu" ordering: typing "u-u-w" produces "ưu" (horn on first u)
//! 3. CTRL commit: CTRL+key outputs key literally, commits current buffer as-is

use goxviet_core::{data::keys, engine::Engine, shared::types::Action};

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn char_to_key(c: char) -> u16 {
    goxviet_core::utils::char_to_key(c)
}

/// Simulate typing `input` (with trailing space for boundary commit) and
/// return what appears on screen.  Uses Telex (method=0) by default.
fn type_telex(input: &str) -> String {
    let mut e = Engine::new(); // Telex by default
    type_with(&mut e, input)
}

fn type_vni(input: &str) -> String {
    let mut e = Engine::new();
    e.set_method(1);
    type_with(&mut e, input)
}

fn type_with(e: &mut Engine, input: &str) -> String {
    let mut screen = String::new();
    for c in input.chars() {
        let key = char_to_key(c);
        let caps = c.is_uppercase();
        let r = e.on_key(key, caps, false);
        apply_result(&mut screen, &r, c);
    }
    screen
}

/// Simulate typing where one character is pressed with CTRL held.
/// `ctrl_char_pos` is the 0-based index of the character in `input` that
/// should be sent with ctrl=true.
fn type_with_ctrl(e: &mut Engine, input: &[(&str, bool)]) -> String {
    let mut screen = String::new();
    for &(chars, is_ctrl) in input {
        for c in chars.chars() {
            let key = char_to_key(c);
            let caps = c.is_uppercase();
            let r = e.on_key(key, caps, is_ctrl);
            if r.action == Action::Send as u8 {
                for _ in 0..r.backspace as usize {
                    screen.pop();
                }
                for i in 0..r.count as usize {
                    unsafe {
                        if let Some(ch) = char::from_u32(*r.chars.offset(i as isize)) {
                            screen.push(ch);
                        }
                    }
                }
            } else if !is_ctrl {
                // Pass through only non-ctrl keys (ctrl keys that pass through are OS shortcuts)
                screen.push(c);
            }
        }
    }
    screen
}

fn apply_result(screen: &mut String, r: &goxviet_core::shared::types::Result, fallback: char) {
    if r.action == Action::Send as u8 {
        for _ in 0..r.backspace as usize {
            screen.pop();
        }
        for i in 0..r.count as usize {
            unsafe {
                if let Some(ch) = char::from_u32(*r.chars.offset(i as isize)) {
                    screen.push(ch);
                }
            }
        }
    } else {
        screen.push(fallback);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// 1. "coẻ" → "core": invalid Vietnamese compound restored at word boundary
// ═════════════════════════════════════════════════════════════════════════════

/// Typing "core " in Telex would naively produce "coẻ " (since 'r'=hỏi absorbed mid-word
/// and 'oe' compound moves tone to 'e').  "coẻ" is not in TuDien → must restore to "core ".
#[test]
fn test_core_restored_at_space() {
    // "core" + space triggers word-boundary English detection
    let result = type_telex("core ");
    assert_eq!(
        result, "core ",
        "typing 'core ' should give 'core ', got '{result}'"
    );
}

/// "score" has the same pattern: 'r' absorbed mid-word, final output "scôẻ" is invalid.
#[test]
fn test_score_restored_at_space() {
    let result = type_telex("score ");
    assert_eq!(
        result, "score ",
        "typing 'score ' should give 'score ', got '{result}'"
    );
}

/// "more" → 'r' absorbed mid-word giving "moẻ" (invalid) → restore to "more ".
#[test]
fn test_more_restored_at_space() {
    let result = type_telex("more ");
    assert_eq!(
        result, "more ",
        "typing 'more ' should give 'more ', got '{result}'"
    );
}

/// "khoẻ" is a REAL Vietnamese word (kh+oe+hỏi) — must NOT be restored.
#[test]
fn test_khoe_kept_as_vietnamese() {
    // khoer = kh+o+e+r(hỏi) → "khoẻ"
    let result = type_telex("khoer ");
    assert_eq!(
        result, "khoẻ ",
        "typing 'khoer ' should give 'khoẻ ' (valid Vietnamese)"
    );
}

/// Regression: "dỉ" (typing "dir") is a valid Vietnamese structure — must NOT be restored.
/// (Single vowel 'i' with hỏi, absorbed 'r' at the END — simple end-tone pattern.)
#[test]
fn test_dir_kept_as_diphthong() {
    let result = type_telex("dir ");
    // "dỉ" is the Vietnamese result of d+i+r(hỏi); not an English restore case
    assert_eq!(
        result, "dỉ ",
        "typing 'dir ' should give 'dỉ ' (valid Vietnamese structure)"
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// 2. "uu" → "ưu": horn applied to FIRST u when typing "u-u-w"
// ═════════════════════════════════════════════════════════════════════════════

/// "uuw" → "ưu": two consecutive u's then 'w' applies horn to the FIRST u.
#[test]
fn test_uuw_gives_uu_with_horn_on_first() {
    use goxviet_core::utils::telex;
    telex(&[("uuw", "ưu")]);
}

/// "luuw" → "lưu": consonant 'l' prefix, then "uu" + 'w' → horn on first u.
#[test]
fn test_luuw_gives_luu_horn() {
    use goxviet_core::utils::telex;
    telex(&[("luuw", "lưu")]);
}

/// "huuw" → "hưu" (deer): same pattern with 'h' prefix.
#[test]
fn test_huuw_gives_huu_horn() {
    use goxviet_core::utils::telex;
    telex(&[("huuw", "hưu")]);
}

/// "nguuw" → "ngưu": multi-char consonant prefix.
#[test]
fn test_nguuw_gives_nguu_horn() {
    use goxviet_core::utils::telex;
    telex(&[("nguuw", "ngưu")]);
}

/// Single "uw" still works: "ư" (one u + w).
#[test]
fn test_uw_gives_single_horn_u() {
    use goxviet_core::utils::telex;
    telex(&[("uw", "ư")]);
}

// ═════════════════════════════════════════════════════════════════════════════
// 3. CTRL commit: CTRL+key outputs key literally, commits current buffer
// ═════════════════════════════════════════════════════════════════════════════

/// "a" + CTRL+s + "k" → "ask"  (Telex: 's' normally = sắc, but CTRL prevents it)
#[test]
fn test_ctrl_prevents_sac_tone_telex() {
    let mut e = Engine::new(); // Telex
                               // &[("chars", is_ctrl)]
    let result = type_with_ctrl(&mut e, &[("a", false), ("s", true), ("k", false)]);
    assert_eq!(
        result, "ask",
        "CTRL+s should output 's' literally, got '{result}'"
    );
}

/// "a" + CTRL+8 → "a8"  (VNI: '8' normally = ngã tone, CTRL prevents it)
#[test]
fn test_ctrl_prevents_nga_tone_vni() {
    let mut e = Engine::new();
    e.set_method(1); // VNI
    let result = type_with_ctrl(&mut e, &[("a", false), ("8", true)]);
    assert_eq!(
        result, "a8",
        "CTRL+8 (VNI) should output '8' literally, got '{result}'"
    );
}

/// "a" + CTRL+f → "af" (Telex: 'f' = huyền, CTRL prevents)
#[test]
fn test_ctrl_prevents_huyen_tone_telex() {
    let mut e = Engine::new();
    let result = type_with_ctrl(&mut e, &[("a", false), ("f", true)]);
    assert_eq!(
        result, "af",
        "CTRL+f should output 'f' literally, got '{result}'"
    );
}

/// Empty buffer + CTRL: no char output (pass through to OS), engine clears.
#[test]
fn test_ctrl_on_empty_buffer_does_nothing() {
    let mut e = Engine::new();
    // No prior typing; CTRL+s on empty buffer should produce nothing visible
    let result = type_with_ctrl(&mut e, &[("s", true)]);
    // ctrl with empty buffer → pass-through → nothing appended to screen
    assert_eq!(
        result, "",
        "CTRL on empty buffer should produce no output, got '{result}'"
    );
}

/// "việt" (typed correctly) + CTRL+s → "việts" (keeps Vietnamese transforms, adds 's')
#[test]
fn test_ctrl_commits_vietnamese_buffer() {
    let mut e = Engine::new(); // Telex
                               // "viet" in Telex = v-i-e-t (no tone, simple structure)
                               // Then CTRL+s should keep "viet" as displayed and add 's'
    let result = type_with_ctrl(&mut e, &[("viet", false), ("s", true)]);
    // "viet" in Telex: v→v, i→i, e→e (no Telex transform for this sequence), t→t → "viet"
    // Then CTRL+s → commit "viet" + 's' → "viets"
    assert_eq!(
        result, "viets",
        "CTRL+s after 'viet' should give 'viets', got '{result}'"
    );
}
