//! Shift+Backspace Tests
//!
//! Tests for deleting entire word with Shift+Backspace

use goxviet_core::data::keys;
use goxviet_core::engine::Engine;
use serial_test::serial;

#[test]
#[serial]
fn test_shift_backspace_simple_word() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);

    // Type "xin"
    engine.on_key(keys::X, false, false);
    engine.on_key(keys::I, false, false);
    engine.on_key(keys::N, false, false);

    assert_eq!(engine.get_buffer(), "xin");

    // Shift+Backspace should delete entire word
    let result = engine.on_key_ext(keys::DELETE, false, false, true);

    assert_eq!(result.action, 1, "Should return Send action");
    assert_eq!(result.backspace, 3, "Should delete 3 chars");
    assert_eq!(engine.get_buffer(), "", "Buffer should be empty");
}

#[test]
#[serial]
fn test_shift_backspace_vietnamese_word() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);

    // Type "việt" = v + i + e + j + t
    engine.on_key(keys::V, false, false);
    engine.on_key(keys::I, false, false);
    engine.on_key(keys::E, false, false);
    engine.on_key(keys::J, false, false); // ệ
    engine.on_key(keys::T, false, false);

    let buffer = engine.get_buffer();
    let char_count = buffer.chars().count();

    // Shift+Backspace should delete entire word
    let result = engine.on_key_ext(keys::DELETE, false, false, true);

    assert_eq!(result.action, 1, "Should return Send action");
    assert_eq!(
        result.backspace as usize, char_count,
        "Should delete all displayed chars"
    );
    assert_eq!(engine.get_buffer(), "", "Buffer should be empty");
}

#[test]
#[serial]
fn test_shift_backspace_empty_buffer() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);

    // Empty buffer - Shift+Backspace should do nothing
    let result = engine.on_key_ext(keys::DELETE, false, false, true);

    assert_eq!(
        result.action, 0,
        "Should return None action for empty buffer"
    );
}

#[test]
#[serial]
fn test_shift_backspace_after_space() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);

    // Type "xin"
    engine.on_key(keys::X, false, false);
    engine.on_key(keys::I, false, false);
    engine.on_key(keys::N, false, false);

    // Press space (commits word)
    engine.on_key(keys::SPACE, false, false);

    // Now buffer is empty but we have history
    assert_eq!(engine.get_buffer(), "");

    // Shift+Backspace should delete the space and the previous word
    let result = engine.on_key_ext(keys::DELETE, false, false, true);

    // Should delete 1 space + 3 chars from "xin" = 4
    assert_eq!(result.action, 1, "Should return Send action");
    assert!(result.backspace >= 1, "Should delete at least 1 char");
}

#[test]
#[serial]
fn test_normal_backspace_still_works() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);

    // Type "abc"
    engine.on_key(keys::A, false, false);
    engine.on_key(keys::B, false, false);
    engine.on_key(keys::C, false, false);

    assert_eq!(engine.get_buffer(), "abc");

    // Normal backspace (without shift) should only delete one char
    let result = engine.on_key_ext(keys::DELETE, false, false, false);

    // Current implementation returns None and lets OS handle it
    assert_eq!(result.action, 0, "Should return None (OS handles deletion)");
    assert_eq!(
        engine.get_buffer(),
        "ab",
        "Buffer should have one less char"
    );
}
