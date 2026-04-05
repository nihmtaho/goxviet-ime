//! Bracket Shortcuts Integration Tests (US2)
//!
//! Regression tests verifying that in Telex mode with bracket_shortcuts_enabled:
//! - `[` (LBRACKET) inserts `ơ`
//! - `]` (RBRACKET) inserts `ư`
//!
//! When disabled or in VNI mode, brackets pass through as literals.

use goxviet_core::data::keys;
use goxviet_core::engine::Engine;
use goxviet_core::utils::type_word;
use serial_test::serial;

// ── Bracket shortcuts enabled (Telex) ────────────────────────────────────────

#[test]
#[serial]
fn test_lbracket_emits_o_horn_in_telex() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_bracket_shortcuts(true);

    let output = type_word(&mut engine, "[");
    assert_eq!(
        output, "ơ",
        "[ should insert ơ when bracket shortcuts are enabled"
    );
}

#[test]
#[serial]
fn test_rbracket_emits_u_horn_in_telex() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_bracket_shortcuts(true);

    let output = type_word(&mut engine, "]");
    assert_eq!(
        output, "ư",
        "] should insert ư when bracket shortcuts are enabled"
    );
}

// ── Bracket shortcuts disabled ────────────────────────────────────────────────

#[test]
#[serial]
fn test_lbracket_passthrough_when_disabled() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    // bracket_shortcuts defaults to false

    let result = engine.on_key(keys::LBRACKET, false, false);
    assert_eq!(
        result.action, 0,
        "[ should not produce a Send result when bracket shortcuts disabled"
    );
}

#[test]
#[serial]
fn test_rbracket_passthrough_when_disabled() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    // bracket_shortcuts defaults to false

    let result = engine.on_key(keys::RBRACKET, false, false);
    assert_eq!(
        result.action, 0,
        "] should not produce a Send result when bracket shortcuts disabled"
    );
}

// ── VNI mode: brackets always pass through ───────────────────────────────────

#[test]
#[serial]
fn test_lbracket_passthrough_in_vni_mode() {
    let mut engine = Engine::new();
    engine.set_method(1); // VNI
    engine.set_enabled(true);
    engine.set_bracket_shortcuts(true); // enabled but VNI mode

    let result = engine.on_key(keys::LBRACKET, false, false);
    assert_eq!(
        result.action, 0,
        "[ should not produce a Send result in VNI mode even if bracket shortcuts enabled"
    );
}

#[test]
#[serial]
fn test_rbracket_passthrough_in_vni_mode() {
    let mut engine = Engine::new();
    engine.set_method(1); // VNI
    engine.set_enabled(true);
    engine.set_bracket_shortcuts(true); // enabled but VNI mode

    let result = engine.on_key(keys::RBRACKET, false, false);
    assert_eq!(
        result.action, 0,
        "] should not produce a Send result in VNI mode even if bracket shortcuts enabled"
    );
}

// ── Double-press escape: [[ → [, ]] → ] ─────────────────────────────────────

#[test]
#[serial]
fn test_double_lbracket_emits_literal_bracket() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_bracket_shortcuts(true);

    // "[[" should produce "[" (second press undoes ơ and inserts literal [)
    let output = type_word(&mut engine, "[[");
    assert_eq!(output, "[", "Double [[ should produce literal [");
}

#[test]
#[serial]
fn test_double_rbracket_emits_literal_bracket() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_bracket_shortcuts(true);

    // "]]" should produce "]" (second press undoes ư and inserts literal ])
    let output = type_word(&mut engine, "]]");
    assert_eq!(output, "]", "Double ]] should produce literal ]");
}

#[test]
#[serial]
fn test_mixed_brackets_no_escape() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_bracket_shortcuts(true);

    // "[]" — different keys, each should produce its own character, no escape
    let output = type_word(&mut engine, "[]");
    assert_eq!(output, "ơư", "Mixed [] should produce ơ then ư");
}

#[test]
#[serial]
fn test_triple_lbracket_restarts_cycle() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_bracket_shortcuts(true);

    // "[[" → "[" (literal), then "[" again → "ơ" (fresh cycle)
    let output = type_word(&mut engine, "[[[");
    assert_eq!(output, "[ơ", "[[[ should produce [ then ơ");
}

// ── Tone marks on bracket-emitted chars ──────────────────────────────────────

#[test]
#[serial]
fn test_lbracket_then_sac_produces_o_sac() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_bracket_shortcuts(true);

    // "[s" → ớ  ([→ ơ in buffer, s applies sắc)
    let output = type_word(&mut engine, "[s");
    assert_eq!(output, "ớ", "[s should produce ớ");
}

#[test]
#[serial]
fn test_rbracket_then_huyen_produces_u_huyen() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_bracket_shortcuts(true);

    // "]f" → ừ  (]→ ư in buffer, f applies huyền)
    let output = type_word(&mut engine, "]f");
    assert_eq!(output, "ừ", "]f should produce ừ");
}

#[test]
#[serial]
fn test_rbracket_lbracket_sac_produces_uong_sac() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_bracket_shortcuts(true);

    // "][s" → ướ  (]→ ư committed, [→ ơ in buffer, s applies sắc)
    let output = type_word(&mut engine, "][s");
    assert_eq!(output, "ướ", "][s should produce ướ");
}
