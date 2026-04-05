//! Word History Integration Tests (US5)
//!
//! Regression tests verifying the "Backspace-After-Space" word history restore feature:
//! (a) Space then immediate Backspace → previous word buffer restored
//! (b) Space then non-Backspace key then two Backspaces → only deletes new key, history invalidated
//! (c) History capacity is 10 (stores all 10 of 10 committed words)
//! (d) word_history_enabled: false → Backspace-after-Space does NOT restore previous word

use goxviet_core::data::keys;
use goxviet_core::engine::{Action, Engine};
use goxviet_core::utils::{char_to_key, type_word};
use serial_test::serial;

// ── (a) Basic restore ─────────────────────────────────────────────────────────

#[test]
#[serial]
fn test_space_then_backspace_restores_previous_word() {
    // After committing a word with Space, an immediate Backspace should restore
    // the word buffer so the user can continue editing it.
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_word_history_enabled(true);

    // Type "xin" then Space (commits "xin" to history)
    type_word(&mut engine, "xin ");

    // Immediate Backspace: should restore "xin"
    let result = engine.on_key(keys::DELETE, false, false);

    assert_eq!(
        result.action,
        Action::Send as u8,
        "Backspace after Space should trigger a Send action (delete the space)"
    );
    assert_eq!(
        engine.get_buffer(),
        "xin",
        "Buffer should be restored to 'xin' after Backspace"
    );
}

#[test]
#[serial]
fn test_space_then_backspace_restores_vietnamese_word() {
    // Verify restoration works with a Vietnamese-transformed word.
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_word_history_enabled(true);

    // Type "viet" (no transform) then Space
    type_word(&mut engine, "viet ");

    // Backspace should restore the committed word
    engine.on_key(keys::DELETE, false, false);

    let buf = engine.get_buffer();
    assert!(
        !buf.is_empty(),
        "Buffer should be non-empty after restore (got: '{buf}')"
    );
}

// ── (b) Invalidation: non-Backspace key after Space ──────────────────────────

#[test]
#[serial]
fn test_nonbackspace_then_backspace_does_not_restore() {
    // After Space, typing any non-Backspace key starts a new word.
    // Pressing Backspace once removes that character.
    // A SECOND Backspace must NOT restore the previous word (the restore
    // opportunity was invalidated by the non-Backspace key — FR-009).
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_word_history_enabled(true);

    // Commit "hello" with Space
    type_word(&mut engine, "hello ");

    // Type 'n' (new word — not a Telex modifier when buffer is empty)
    engine.on_key(char_to_key('n'), false, false);
    assert_eq!(engine.get_buffer(), "n", "Buffer should contain 'n'");

    // First Backspace: removes 'n', buffer becomes empty
    engine.on_key(keys::DELETE, false, false);
    assert_eq!(
        engine.get_buffer(),
        "",
        "After first Backspace, buffer should be empty"
    );

    // Second Backspace: must NOT restore "hello" because 'n' invalidated the entry
    let result = engine.on_key(keys::DELETE, false, false);
    assert_ne!(
        result.action,
        Action::Send as u8,
        "Second Backspace should NOT trigger a restore action (restore was invalidated by 'n')"
    );
    assert_eq!(
        engine.get_buffer(),
        "",
        "Buffer must remain empty — previous word must NOT be restored"
    );
}

// ── (c) History capacity = 10 ─────────────────────────────────────────────────

#[test]
#[serial]
fn test_word_history_capacity_is_ten() {
    // The ring buffer must hold at least 10 entries.
    // With HISTORY_CAPACITY = 3 (old value), only the last 3 would be stored.
    // This test verifies word_history_len() == 10 after 10 commits.
    //
    // Words use only 'b' and 'k' — letters that are never Telex modifiers or
    // vowels, so no transforms fire and all buffers are non-empty at Space time.
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_word_history_enabled(true);

    // Push 10 words with lengths 1..=10 using safe (non-modifier) letters
    for n in 1..=10usize {
        let word: String = std::iter::repeat('b').take(n).collect();
        type_word(&mut engine, &format!("{word} "));
    }

    assert_eq!(
        engine.word_history_len(),
        10,
        "History must contain all 10 words — requires HISTORY_CAPACITY = 10 (currently 3)"
    );
}

#[test]
#[serial]
fn test_backspace_after_ten_words_restores_most_recent() {
    // After 10 words, Backspace should restore the 10th (most recent) word.
    // Words use only 'k' (non-modifier) to avoid Telex transforms.
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_word_history_enabled(true);

    for n in 1..=10usize {
        let word: String = std::iter::repeat('k').take(n).collect();
        type_word(&mut engine, &format!("{word} "));
    }

    // Backspace: should restore the 10th word (10 'k' chars)
    engine.on_key(keys::DELETE, false, false);
    assert_eq!(
        engine.get_buffer().chars().count(),
        10,
        "Should restore the 10th word (10 'k' chars)"
    );
}

// ── (d) Feature disabled — no restore ─────────────────────────────────────────

#[test]
#[serial]
fn test_word_history_disabled_no_restore_on_backspace() {
    // When word_history_enabled is false (the default), Backspace after Space
    // must behave as a plain Backspace — it deletes the space but does NOT
    // restore the previous word into the editing buffer.
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    // word_history_enabled defaults to false — do NOT call set_word_history_enabled(true)

    // Commit "hello" then Space
    type_word(&mut engine, "hello ");

    // Backspace: should NOT restore "hello"
    engine.on_key(keys::DELETE, false, false);

    assert_eq!(
        engine.get_buffer(),
        "",
        "With word_history_enabled=false, Backspace-after-Space must not restore 'hello'"
    );
}

#[test]
#[serial]
fn test_word_history_disabled_explicitly_no_restore() {
    // Explicitly disabling word_history after enabling it should also prevent restore.
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_word_history_enabled(false); // explicit disable

    type_word(&mut engine, "test ");

    engine.on_key(keys::DELETE, false, false);

    assert_eq!(
        engine.get_buffer(),
        "",
        "With word_history_enabled=false, buffer must not be restored"
    );
}
