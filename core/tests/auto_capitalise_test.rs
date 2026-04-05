//! Auto-Capitalise Integration Tests (US4)
//!
//! Regression tests verifying that when `auto_capitalise_enabled` is true:
//! - Typing a letter after `.` + Space capitalises the letter
//! - `!` and `?` sentence-end triggers work (via on_key_ext with shift)
//! - Decimal numbers like `3.14` do NOT trigger capitalisation
//! - Abbreviations like `v.v.`, `tr.`, `tp.` do NOT trigger capitalisation
//!
//! When disabled, no automatic capitalisation occurs.

use goxviet_core::data::keys;
use goxviet_core::engine::Engine;
use goxviet_core::utils::type_word;
use serial_test::serial;

// ── Feature enabled: sentence-end dot triggers capitalisation ────────────────

#[test]
#[serial]
fn test_dot_space_capitalises_next_letter() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_auto_capitalise(true);

    // "a. b" → the 'b' after ". " should be capitalised to 'B'
    let output = type_word(&mut engine, "a. b");
    assert_eq!(
        output, "a. B",
        "Letter after '. ' should be auto-capitalised when feature is enabled"
    );
}

#[test]
#[serial]
fn test_exclamation_space_capitalises_next_letter() {
    // '!' = Shift+1 on keyboard (N1 + shift). Tested via on_key_ext.
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_auto_capitalise(true);

    // Simulate: type 'a', type '!' (N1 with shift), type space, type 'b'
    engine.on_key(keys::A, false, false);
    engine.on_key_ext(keys::N1, false, false, true); // Shift+1 = '!'
    engine.on_key(keys::SPACE, false, false);
    let r = engine.on_key(keys::B, false, false);

    // The 'b' should be capitalised to 'B' (action=Send, char='B')
    use goxviet_core::engine::Action;
    assert_eq!(
        r.action,
        Action::Send as u8,
        "'b' after '! ' should be capitalised"
    );
    let first_char = unsafe { char::from_u32(*r.chars) };
    assert_eq!(
        first_char,
        Some('B'),
        "'b' should become 'B' after exclamation"
    );
}

#[test]
#[serial]
fn test_question_space_capitalises_next_letter() {
    // '?' = Shift+/ on keyboard (SLASH + shift). Tested via on_key_ext.
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_auto_capitalise(true);

    // Simulate: type 'a', type '?' (SLASH with shift), type space, type 'b'
    engine.on_key(keys::A, false, false);
    engine.on_key_ext(keys::SLASH, false, false, true); // Shift+/ = '?'
    engine.on_key(keys::SPACE, false, false);
    let r = engine.on_key(keys::B, false, false);

    use goxviet_core::engine::Action;
    assert_eq!(
        r.action,
        Action::Send as u8,
        "'b' after '? ' should be capitalised"
    );
    let first_char = unsafe { char::from_u32(*r.chars) };
    assert_eq!(
        first_char,
        Some('B'),
        "'b' should become 'B' after question mark"
    );
}

// ── Decimal exclusion ────────────────────────────────────────────────────────

#[test]
#[serial]
fn test_decimal_dot_does_not_capitalise() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_auto_capitalise(true);

    // "3.14 t" → no capitalisation because '.' is a decimal separator
    let output = type_word(&mut engine, "3.14 t");
    assert_eq!(
        output, "3.14 t",
        "Decimal dot '3.14 t' should NOT trigger auto-capitalisation"
    );
}

// ── Abbreviation exclusions ──────────────────────────────────────────────────

#[test]
#[serial]
fn test_abbreviation_vv_does_not_capitalise() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_auto_capitalise(true);

    // "v.v. t" → 'v.v.' is an abbreviation (và vân vân), no capitalisation
    let output = type_word(&mut engine, "v.v. t");
    assert_eq!(
        output, "v.v. t",
        "Abbreviation 'v.v.' should NOT trigger auto-capitalisation"
    );
}

#[test]
#[serial]
fn test_abbreviation_tr_does_not_capitalise() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_auto_capitalise(true);

    // "tr. t" → 'tr.' is an abbreviation (trang), no capitalisation
    let output = type_word(&mut engine, "tr. t");
    assert_eq!(
        output, "tr. t",
        "Abbreviation 'tr.' should NOT trigger auto-capitalisation"
    );
}

#[test]
#[serial]
fn test_abbreviation_tp_does_not_capitalise() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_auto_capitalise(true);

    // "tp. h" → 'tp.' is an abbreviation (thành phố), no capitalisation
    let output = type_word(&mut engine, "tp. h");
    assert_eq!(
        output, "tp. h",
        "Abbreviation 'tp.' should NOT trigger auto-capitalisation"
    );
}

// ── Feature disabled ─────────────────────────────────────────────────────────

#[test]
#[serial]
fn test_no_capitalisation_when_feature_disabled() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    // auto_capitalise defaults to false

    // "a. b" → no capitalisation, 'b' stays lowercase
    let output = type_word(&mut engine, "a. b");
    assert_eq!(
        output, "a. b",
        "No capitalisation should occur when auto_capitalise_enabled is false"
    );
}

#[test]
#[serial]
fn test_only_first_letter_after_boundary_is_capitalised() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_auto_capitalise(true);

    // "a. bc" → only the first 'b' gets capitalised, 'c' stays lowercase
    let output = type_word(&mut engine, "a. bc");
    assert_eq!(
        output, "a. Bc",
        "Only the first letter after a sentence boundary should be auto-capitalised"
    );
}
