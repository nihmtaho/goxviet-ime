//! Foreign Consonants Integration Tests (US3)
//!
//! Regression tests verifying that when `foreign_consonants_enabled` is true:
//! - `w` at word-start is treated as a literal consonant (not ư shortcut)
//! - `z`, `j`, `f` at word-start are allowed as valid word initials
//! - `w` after a vowel (mid-word) still applies the horn modifier
//!
//! When disabled, existing English auto-restore behaviour is unchanged.

use goxviet_core::data::keys;
use goxviet_core::engine::Engine;
use goxviet_core::utils::type_word;
use serial_test::serial;

// ── Feature enabled: w at word-start is a literal consonant ─────────────────

#[test]
#[serial]
fn test_w_at_word_start_is_literal_when_foreign_consonants_enabled() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_foreign_consonants(true);

    // With foreign_consonants_enabled, pressing 'w' at word-start should
    // pass through as literal 'w', NOT emit 'ư' via the horn shortcut.
    let result = engine.on_key(keys::W, false, false);
    assert_eq!(
        result.action, 0,
        "w at word-start should pass through as literal consonant when foreign consonants enabled"
    );
}

#[test]
#[serial]
fn test_wifi_stays_wifi_when_foreign_consonants_enabled() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_foreign_consonants(true);

    let output = type_word(&mut engine, "wifi");
    assert_eq!(
        output, "wifi",
        "wifi should remain wifi when foreign consonants enabled"
    );
}

#[test]
#[serial]
fn test_zoom_stays_zoom_when_foreign_consonants_enabled() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_foreign_consonants(true);

    let output = type_word(&mut engine, "zoom");
    assert_eq!(
        output, "zoom",
        "zoom should remain zoom when foreign consonants enabled"
    );
}

#[test]
#[serial]
fn test_jazz_stays_jazz_when_foreign_consonants_enabled() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_foreign_consonants(true);

    let output = type_word(&mut engine, "jazz");
    assert_eq!(
        output, "jazz",
        "jazz should remain jazz when foreign consonants enabled"
    );
}

// ── Feature enabled: w mid-word still applies horn modifier ─────────────────

#[test]
#[serial]
fn test_w_after_vowel_still_applies_horn_when_foreign_consonants_enabled() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_foreign_consonants(true);

    // 'how': h + o + w → hơ (w applies horn modifier to preceding 'o')
    // Mid-word 'w' must still behave as a horn modifier
    let output = type_word(&mut engine, "how");
    assert_eq!(
        output, "hơ",
        "w after a vowel should still apply horn modifier when foreign consonants enabled"
    );
}

// ── Feature disabled: existing English auto-restore unchanged ────────────────

#[test]
#[serial]
fn test_zoom_still_works_when_foreign_consonants_disabled() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    // foreign_consonants defaults to false

    let output = type_word(&mut engine, "zoom");
    assert_eq!(
        output, "zoom",
        "zoom should still output zoom via English auto-restore when foreign consonants disabled"
    );
}

#[test]
#[serial]
fn test_java_still_works_when_foreign_consonants_disabled() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    // foreign_consonants defaults to false

    let output = type_word(&mut engine, "java");
    assert_eq!(
        output, "java",
        "java should still output java via English auto-restore when foreign consonants disabled"
    );
}

#[test]
#[serial]
fn test_w_passthrough_at_word_start_when_foreign_consonants_disabled() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    // foreign_consonants defaults to false

    // Without the feature, w at word-start passes through as literal 'w' (skip_w_shortcut=true by default)
    let output = type_word(&mut engine, "w");
    assert_eq!(
        output, "w",
        "w at word-start should pass through as literal w (skip_w_shortcut default=true)"
    );
}
