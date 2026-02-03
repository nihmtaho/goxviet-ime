//! Test case for stroke edge case: d-a-d-d should produce "dad" not "đad"
//!
//! The issue: When typing "d-a-d-d" in Telex mode, the second 'd' incorrectly
//! applies stroke to the first 'd', resulting in "đad" instead of "dad".
//!
//! Expected behavior:
//! - "d-a-d" → "đa" (stroke applied to second 'd')
//! - "d-a-d-d" → "dad" (NO stroke, English word)

use goxviet_core::data::keys;
use goxviet_core::engine::Engine;

/// Test that "d-a-d" correctly produces "đa"
#[test]
fn test_dad_becomes_da_with_stroke() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);

    // Type "d-a-d"
    engine.on_key(keys::D as u16, false, false);
    engine.on_key(keys::A as u16, false, false);
    engine.on_key(keys::D as u16, false, false);

    let committed = engine.get_buffer();
    assert_eq!(committed, "đa", "d-a-d should produce đa");
}

/// Test that "d-a-d-d" correctly produces "dad" (not "đad")
#[test]
fn test_dadd_becomes_dad_without_stroke() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);

    println!("=== Typing d-a-d-d ===");

    let r1 = engine.on_key(keys::D as u16, false, false);
    println!(
        "After 'd': backspace={}, count={}, buffer={:?}",
        r1.backspace,
        r1.count,
        engine.get_buffer()
    );

    let r2 = engine.on_key(keys::A as u16, false, false);
    println!(
        "After 'a': backspace={}, count={}, buffer={:?}",
        r2.backspace,
        r2.count,
        engine.get_buffer()
    );

    let r3 = engine.on_key(keys::D as u16, false, false);
    println!(
        "After 'd' (3rd): backspace={}, count={}, buffer={:?}",
        r3.backspace,
        r3.count,
        engine.get_buffer()
    );

    let r4 = engine.on_key(keys::D as u16, false, false);
    println!(
        "After 'd' (4th): backspace={}, count={}, buffer={:?}",
        r4.backspace,
        r4.count,
        engine.get_buffer()
    );

    let committed = engine.get_buffer();
    assert_eq!(committed, "dad", "d-a-d-d should produce dad, not đad");
}

/// Test that "d-d" at word start correctly produces "đ"
#[test]
fn test_dd_at_start_becomes_d_with_stroke() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);

    // Type "d-d"
    engine.on_key(keys::D as u16, false, false);
    engine.on_key(keys::D as u16, false, false);

    let committed = engine.get_buffer();
    assert_eq!(committed, "đ", "d-d should produce đ");
}
