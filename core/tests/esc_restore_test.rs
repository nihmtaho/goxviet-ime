//! ESC Restore Integration Tests (US1)
//!
//! Regression tests verifying that the ESC key restore feature works correctly:
//! - With `esc_restore_enabled: true`, pressing ESC after Vietnamese transformation
//!   reverts the buffer to the original raw ASCII keystrokes.
//! - With `esc_restore_enabled: false`, ESC is not consumed and no restoration occurs.

use goxviet_core::data::keys;
use goxviet_core::engine::{Action, Engine};
use goxviet_core::utils::{char_to_key, type_word};
use serial_test::serial;

// ── ESC restore enabled ──────────────────────────────────────────────────────

#[test]
#[serial]
fn test_esc_restore_reverts_telex_transformation() {
    // "aa" → "â" in Telex; ESC should restore "aa"
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_esc_restore(true);

    let output = type_word(&mut engine, "aa\x1b");
    assert_eq!(output, "aa", "ESC should revert 'â' back to raw 'aa'");
}

#[test]
#[serial]
fn test_esc_restore_reverts_stroke_modifier() {
    // "dd" → "đ" in Telex; ESC should restore "dd"
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_esc_restore(true);

    let output = type_word(&mut engine, "dd\x1b");
    assert_eq!(output, "dd", "ESC should revert 'đ' back to raw 'dd'");
}

#[test]
#[serial]
fn test_esc_restore_reverts_full_word() {
    // "vieejt" → "việt" in Telex; ESC should restore "vieejt"
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_esc_restore(true);

    let output = type_word(&mut engine, "vieejt\x1b");
    assert_eq!(
        output, "vieejt",
        "ESC should revert 'việt' back to raw 'vieejt'"
    );
}

// ── ESC restore disabled ─────────────────────────────────────────────────────

#[test]
#[serial]
fn test_esc_no_restore_when_disabled() {
    // With esc_restore_enabled: false (default), ESC clears but does not emit a Send result.
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    // esc_restore defaults to false — do NOT call set_esc_restore

    engine.on_key(char_to_key('a'), false, false);
    engine.on_key(char_to_key('a'), false, false); // "aa" → "â"

    let result = engine.on_key(keys::ESC, false, false);
    assert_eq!(
        result.action,
        Action::None as u8,
        "ESC should not produce a Send result when esc_restore is disabled"
    );

    let buffer = engine.get_buffer();
    assert_eq!(buffer, "", "Buffer should be empty after ESC");
}

#[test]
#[serial]
fn test_esc_restore_clears_buffer_afterward() {
    // After ESC restore, the buffer should be empty (ready for next word)
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_esc_restore(true);

    engine.on_key(char_to_key('a'), false, false);
    engine.on_key(char_to_key('a'), false, false);
    engine.on_key(keys::ESC, false, false);

    let buffer = engine.get_buffer();
    assert_eq!(buffer, "", "Buffer should be cleared after ESC restore");
}
