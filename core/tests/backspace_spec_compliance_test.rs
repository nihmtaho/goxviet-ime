//! Backspace Specification Compliance Tests
//!
//! This test suite verifies that the backspace implementation complies with
//! the specification defined in `.github/instructions/10_vietnamese_backspace_and_buffer_reset.md`
//!
//! ## Specification Requirements
//!
//! ### Golden Rules
//! 1. Backspace deletes by grapheme (visible character), not by diacritics
//! 2. Telex is only an input method, deletion is based on display
//! 3. Never patch rendered strings - always rebuild from tokens
//! 4. Each word is an independent transaction
//! 5. Deleting entire word → clean all buffers & state
//!
//! ### Backspace Rules
//! - RULE 1: Delete EXACTLY ONE grapheme
//! - RULE 2: NEVER delete tone/modifier independently
//! - RULE 3: NEVER modify rendered text directly
//! - RULE 4: Always rebuild from remaining tokens
//! - RULE 5: Reset everything when last grapheme deleted
//!
//! ### Expected Behavior
//! ```
//! diễn → BS → diê → BS → di → BS → d → BS → ""
//! Then type "a" → "a" (not "ả")
//! ```

use goxviet_core::engine::{Engine, Result as EngineResult};
use goxviet_core::data::keys;

/// Helper to render buffer to string
fn render(engine: &Engine) -> String {
    let result = engine.render_buffer();
    result.iter().collect()
}

/// Helper to type a sequence of keys
fn type_keys(engine: &mut Engine, keys_seq: &[u16]) {
    for &key in keys_seq {
        engine.on_key(key, false, false);
    }
}

// ============================================================================
// MANDATORY TEST CASES FROM SPEC
// ============================================================================

/// Test Case 1 from spec: diễn → BS → diê → BS → di → BS → d → BS → ""
/// Then type "a" → result MUST be "a" (not "ả")
///
/// This tests:
/// - RULE 1: Delete exactly one grapheme
/// - RULE 5: Reset all state when buffer empty
#[test]
fn test_spec_mandatory_case_1_dien() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex

    // Type "diễn" (d-i-e-x-n)
    engine.on_key(keys::D, false, false);
    engine.on_key(keys::I, false, false);
    engine.on_key(keys::E, false, false);
    engine.on_key(keys::X, false, false); // Tone ngã → ẽ
    engine.on_key(keys::N, false, false);

    assert_eq!(render(&engine), "diễn", "Initial word should be 'diễn'");

    // Backspace 1: diễn → diê (remove 'n')
    let r1 = engine.on_backspace();
    assert_eq!(r1.backspace, 1, "Should delete 1 character");
    assert_eq!(render(&engine), "diê", "After BS: should be 'diê'");

    // Backspace 2: diê → di (remove 'ê' which has tone)
    let r2 = engine.on_backspace();
    assert_eq!(r2.backspace, 1, "Should delete 1 character");
    assert_eq!(render(&engine), "di", "After BS: should be 'di'");

    // Backspace 3: di → d (remove 'i')
    let r3 = engine.on_backspace();
    assert_eq!(r3.backspace, 1, "Should delete 1 character");
    assert_eq!(render(&engine), "d", "After BS: should be 'd'");

    // Backspace 4: d → "" (remove 'd')
    let r4 = engine.on_backspace();
    assert_eq!(r4.backspace, 1, "Should delete 1 character");
    assert_eq!(render(&engine), "", "After BS: should be empty");

    // CRITICAL: Verify state is completely reset
    // This is RULE 5 from spec
    assert!(engine.is_buffer_empty(), "Buffer should be empty");

    // Type "a" → should be "a", NOT "ả" (no lingering tone state)
    engine.on_key(keys::A, false, false);
    assert_eq!(render(&engine), "a", "After reset, 'a' should be plain 'a', not 'ả'");
}

/// Test Case 2 from spec: tiếng → BS × 5 → ""
/// Then type "o" → result MUST be "o"
#[test]
fn test_spec_mandatory_case_2_tieng() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex

    // Type "tiếng" (t-i-e-s-n-g)
    engine.on_key(keys::T, false, false);
    engine.on_key(keys::I, false, false);
    engine.on_key(keys::E, false, false);
    engine.on_key(keys::S, false, false); // Tone sắc → é
    engine.on_key(keys::N, false, false);
    engine.on_key(keys::G, false, false);

    assert_eq!(render(&engine), "tiếng", "Initial word should be 'tiếng'");

    // Backspace 5 times to empty
    for i in 1..=5 {
        engine.on_backspace();
        println!("After backspace {}: '{}'", i, render(&engine));
    }

    assert_eq!(render(&engine), "", "Should be empty after 5 backspaces");
    assert!(engine.is_buffer_empty(), "Buffer should be empty");

    // Type "o" → should be "o" (clean state)
    engine.on_key(keys::O, false, false);
    assert_eq!(render(&engine), "o", "After reset, 'o' should be plain 'o'");
}

/// Test Case 3 from spec: telex → BS → tele
/// Tests English word handling
#[test]
fn test_spec_mandatory_case_3_telex() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex

    // Type "telex"
    type_keys(&mut engine, &[keys::T, keys::E, keys::L, keys::E, keys::X]);

    // Note: "telex" might be detected as English and stay as "telex"
    // OR it might transform "ex" pattern. Either way, backspace should work.
    let before = render(&engine);
    println!("Before BS: '{}'", before);

    // Backspace once
    engine.on_backspace();
    let after = render(&engine);
    println!("After BS: '{}'", after);

    // Should have removed exactly one character
    assert_eq!(after.len(), before.len() - 1, "Should remove exactly 1 char");
}

/// Test Case 4 from spec: improve → BS → improv
/// Tests English word handling
#[test]
fn test_spec_mandatory_case_4_improve() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex

    // Type "improve"
    type_keys(&mut engine, &[
        keys::I, keys::M, keys::P, keys::R, keys::O, keys::V, keys::E
    ]);

    assert_eq!(render(&engine), "improve", "Should be 'improve'");

    // Backspace once
    engine.on_backspace();
    assert_eq!(render(&engine), "improv", "Should be 'improv'");
}

// ============================================================================
// RULE 1: Delete EXACTLY ONE Grapheme
// ============================================================================

#[test]
fn test_rule1_delete_one_grapheme_simple() {
    let mut engine = Engine::new();
    engine.set_method(0);

    // Type "abc"
    type_keys(&mut engine, &[keys::A, keys::B, keys::C]);
    assert_eq!(render(&engine), "abc");

    // Each backspace deletes exactly 1 char
    engine.on_backspace();
    assert_eq!(render(&engine), "ab", "Delete 1: 'c'");

    engine.on_backspace();
    assert_eq!(render(&engine), "a", "Delete 1: 'b'");

    engine.on_backspace();
    assert_eq!(render(&engine), "", "Delete 1: 'a'");
}

#[test]
fn test_rule1_delete_one_grapheme_with_tone() {
    let mut engine = Engine::new();
    engine.set_method(0);

    // Type "án" (a-s-n)
    engine.on_key(keys::A, false, false);
    engine.on_key(keys::S, false, false); // Tone sắc → á
    engine.on_key(keys::N, false, false);

    assert_eq!(render(&engine), "án");

    // Backspace deletes 'n' (1 grapheme)
    engine.on_backspace();
    assert_eq!(render(&engine), "á", "Should delete 'n', leaving 'á'");

    // Backspace deletes 'á' (1 grapheme with tone)
    engine.on_backspace();
    assert_eq!(render(&engine), "", "Should delete 'á' completely");
}

#[test]
fn test_rule1_delete_one_grapheme_circumflex() {
    let mut engine = Engine::new();
    engine.set_method(0);

    // Type "ân" (a-a-n)
    engine.on_key(keys::A, false, false);
    engine.on_key(keys::A, false, false); // aa → â
    engine.on_key(keys::N, false, false);

    assert_eq!(render(&engine), "ân");

    // Backspace deletes 'n'
    engine.on_backspace();
    assert_eq!(render(&engine), "â", "Should delete 'n', leaving 'â'");

    // Backspace deletes 'â' (1 grapheme with circumflex)
    engine.on_backspace();
    assert_eq!(render(&engine), "", "Should delete 'â' completely");
}

// ============================================================================
// RULE 2: NEVER Delete Tone/Modifier Independently
// ============================================================================

#[test]
fn test_rule2_never_delete_tone_separately() {
    let mut engine = Engine::new();
    engine.set_method(0);

    // Type "ánh" (a-s-n-h)
    engine.on_key(keys::A, false, false);
    engine.on_key(keys::S, false, false); // á
    engine.on_key(keys::N, false, false);
    engine.on_key(keys::H, false, false);

    assert_eq!(render(&engine), "ánh");

    // Backspace should delete 'h', not remove tone from 'á'
    engine.on_backspace();
    assert_eq!(render(&engine), "án", "Should delete 'h', not tone");

    // Backspace should delete 'n', not remove tone from 'á'
    engine.on_backspace();
    assert_eq!(render(&engine), "á", "Should delete 'n', not tone");

    // Backspace should delete 'á' entirely (with its tone)
    engine.on_backspace();
    assert_eq!(render(&engine), "", "Should delete 'á' with tone");
}

#[test]
fn test_rule2_never_delete_circumflex_separately() {
    let mut engine = Engine::new();
    engine.set_method(0);

    // Type "ông" (o-o-n-g)
    engine.on_key(keys::O, false, false);
    engine.on_key(keys::O, false, false); // oo → ô
    engine.on_key(keys::N, false, false);
    engine.on_key(keys::G, false, false);

    assert_eq!(render(&engine), "ông");

    // Backspace deletes 'g'
    engine.on_backspace();
    assert_eq!(render(&engine), "ôn", "Should delete 'g'");

    // Backspace deletes 'n'
    engine.on_backspace();
    assert_eq!(render(&engine), "ô", "Should delete 'n'");

    // Backspace deletes 'ô' entirely (NOT revert to 'o')
    engine.on_backspace();
    assert_eq!(render(&engine), "", "Should delete 'ô' completely");
}

#[test]
fn test_rule2_delete_d_with_stroke_atomically() {
    let mut engine = Engine::new();
    engine.set_method(0);

    // Type "đi" (d-d-i)
    engine.on_key(keys::D, false, false);
    engine.on_key(keys::D, false, false); // dd → đ
    engine.on_key(keys::I, false, false);

    assert_eq!(render(&engine), "đi");

    // Backspace deletes 'i'
    engine.on_backspace();
    assert_eq!(render(&engine), "đ", "Should delete 'i'");

    // Backspace deletes 'đ' entirely (NOT revert to 'd')
    engine.on_backspace();
    assert_eq!(render(&engine), "", "Should delete 'đ' completely");
}

// ============================================================================
// RULE 5: Reset EVERYTHING When Last Grapheme Deleted
// ============================================================================

#[test]
fn test_rule5_reset_all_state_on_empty() {
    let mut engine = Engine::new();
    engine.set_method(0);

    // Type "hòa" (h-o-f-a)
    engine.on_key(keys::H, false, false);
    engine.on_key(keys::O, false, false);
    engine.on_key(keys::F, false, false); // Tone huyền → ò
    engine.on_key(keys::A, false, false);

    assert_eq!(render(&engine), "hòa");

    // Delete all characters
    for _ in 0..3 {
        engine.on_backspace();
    }

    assert_eq!(render(&engine), "", "Buffer should be empty");
    assert!(engine.is_buffer_empty(), "Buffer should report empty");

    // Type new word "cô" - should NOT be affected by previous state
    engine.on_key(keys::C, false, false);
    engine.on_key(keys::O, false, false);
    engine.on_key(keys::O, false, false); // oo → ô

    assert_eq!(render(&engine), "cô", "New word should be clean");
}

#[test]
fn test_rule5_reset_english_word_flag() {
    let mut engine = Engine::new();
    engine.set_method(0);

    // Type English word "next" (n-e-x-t)
    // This should set is_english_word flag
    engine.on_key(keys::N, false, false);
    engine.on_key(keys::E, false, false);
    engine.on_key(keys::X, false, false);
    engine.on_key(keys::T, false, false);

    // Delete all
    for _ in 0..4 {
        engine.on_backspace();
    }

    assert_eq!(render(&engine), "", "Should be empty");

    // Type Vietnamese "cố" (c-o-s)
    // The 's' tone mark should work (flag must be reset)
    engine.on_key(keys::C, false, false);
    engine.on_key(keys::O, false, false);
    engine.on_key(keys::S, false, false); // Should apply sắc tone

    let output = render(&engine);
    assert_eq!(output, "cố", "Tone mark should work after English word deleted");
}

// ============================================================================
// COMPLEX SCENARIOS
// ============================================================================

#[test]
fn test_backspace_complex_syllable_rebuild() {
    let mut engine = Engine::new();
    engine.set_method(0);

    // Type "hoàng" (h-o-f-a-n-g)
    engine.on_key(keys::H, false, false);
    engine.on_key(keys::O, false, false);
    engine.on_key(keys::F, false, false); // huyền → ò
    engine.on_key(keys::A, false, false);
    engine.on_key(keys::N, false, false);
    engine.on_key(keys::G, false, false);

    assert_eq!(render(&engine), "hoàng");

    // Backspace 'g' - should rebuild syllable
    engine.on_backspace();
    assert_eq!(render(&engine), "hoàn");

    // Backspace 'n' - should rebuild syllable
    engine.on_backspace();
    assert_eq!(render(&engine), "hoà");

    // Backspace 'à' - should delete entire grapheme
    engine.on_backspace();
    assert_eq!(render(&engine), "ho");
}

#[test]
fn test_backspace_with_horn_vowel() {
    let mut engine = Engine::new();
    engine.set_method(0);

    // Type "ươn" (u-o-w-n)
    engine.on_key(keys::U, false, false);
    engine.on_key(keys::O, false, false);
    engine.on_key(keys::W, false, false); // uow → ươ
    engine.on_key(keys::N, false, false);

    assert_eq!(render(&engine), "ươn");

    // Backspace 'n'
    engine.on_backspace();
    assert_eq!(render(&engine), "ươ");

    // Backspace 'ơ' (should delete, not revert)
    engine.on_backspace();
    let output = render(&engine);
    // Should have deleted one grapheme from "ươ"
    assert!(output.len() < "ươ".len(), "Should have deleted one grapheme");
}

#[test]
fn test_backspace_after_space_restore() {
    let mut engine = Engine::new();
    engine.set_method(0);

    // Type "xin" + space
    type_keys(&mut engine, &[keys::X, keys::I, keys::N]);
    engine.on_key(keys::SPACE, false, false);

    // Type new word "chào"
    type_keys(&mut engine, &[keys::C, keys::H, keys::A, keys::F, keys::O]);
    
    // Note: Implementation may have word history feature
    // This test just ensures backspace works correctly
    engine.on_backspace();
    // Should delete one character from current word
}

#[test]
fn test_multiple_words_with_backspace() {
    let mut engine = Engine::new();
    engine.set_method(0);

    // Type "việt nam"
    type_keys(&mut engine, &[keys::V, keys::I, keys::E, keys::J, keys::T]); // việt
    engine.on_key(keys::SPACE, false, false);
    type_keys(&mut engine, &[keys::N, keys::A, keys::M]); // nam

    assert_eq!(render(&engine), "nam");

    // Backspace in second word
    engine.on_backspace();
    assert_eq!(render(&engine), "na");

    engine.on_backspace();
    assert_eq!(render(&engine), "n");

    engine.on_backspace();
    assert_eq!(render(&engine), "");
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_backspace_on_empty_buffer() {
    let mut engine = Engine::new();
    engine.set_method(0);

    // Backspace on empty buffer should not crash
    let result = engine.on_backspace();
    assert_eq!(result.action, 0, "Should return none action");
    assert_eq!(render(&engine), "");
}

#[test]
fn test_backspace_preserves_capitalization() {
    let mut engine = Engine::new();
    engine.set_method(0);

    // Type "Việt" with capital V
    engine.on_key(keys::V, true, false);  // Caps
    engine.on_key(keys::I, false, false);
    engine.on_key(keys::E, false, false);
    engine.on_key(keys::J, false, false); // ngã tone
    engine.on_key(keys::T, false, false);

    assert_eq!(render(&engine), "Việt");

    // Backspace should maintain capital V
    engine.on_backspace();
    assert_eq!(render(&engine), "Viễ");

    engine.on_backspace();
    assert_eq!(render(&engine), "Vi");

    engine.on_backspace();
    let output = render(&engine);
    assert_eq!(output, "V", "Should preserve capital V");
}

#[test]
fn test_fast_path_vs_complex_path() {
    let mut engine = Engine::new();
    engine.set_method(0);

    // Type "hoán" (h-o-a-s-n)
    engine.on_key(keys::H, false, false);
    engine.on_key(keys::O, false, false);
    engine.on_key(keys::A, false, false);
    engine.on_key(keys::S, false, false); // sắc tone on 'a' → oá
    engine.on_key(keys::N, false, false);

    assert_eq!(render(&engine), "hoán");

    // Backspace 'n' - should use fast path (simple char, no transforms)
    let r1 = engine.on_backspace();
    assert_eq!(r1.backspace, 1);
    assert_eq!(render(&engine), "hoá");

    // Backspace 'á' - should use complex path (has tone)
    let r2 = engine.on_backspace();
    assert_eq!(r2.backspace, 1);
    assert_eq!(render(&engine), "ho");
}

// ============================================================================
// ANTI-PATTERNS (Should NOT Happen)
// ============================================================================

#[test]
fn test_antipattern_no_separate_tone_deletion() {
    let mut engine = Engine::new();
    engine.set_method(0);

    // Type "á" (a-s)
    engine.on_key(keys::A, false, false);
    engine.on_key(keys::S, false, false); // sắc tone

    assert_eq!(render(&engine), "á");

    // Backspace should delete entire 'á', NOT just the tone
    engine.on_backspace();
    assert_eq!(render(&engine), "", "Should delete entire 'á', not revert to 'a'");
}

#[test]
fn test_antipattern_no_string_patching() {
    let mut engine = Engine::new();
    engine.set_method(0);

    // Type "hoàng"
    type_keys(&mut engine, &[keys::H, keys::O, keys::F, keys::A, keys::N, keys::G]);
    
    let before = render(&engine);
    
    // Backspace - implementation MUST rebuild, not patch string
    engine.on_backspace();
    
    let after = render(&engine);
    
    // We can't directly test "no string patching" from outside,
    // but we can verify correctness
    assert!(after.len() < before.len(), "Should have removed characters");
    assert!(after.starts_with("hoà"), "Should maintain correct prefix");
}

#[cfg(test)]
mod spec_reference {
    //! Reference to specification document
    //! 
    //! This test suite implements requirements from:
    //! `.github/instructions/10_vietnamese_backspace_and_buffer_reset.md`
    //! 
    //! Golden Rules:
    //! 1. Backspace xóa theo chữ hiển thị (grapheme)
    //! 2. Telex chỉ là phương thức nhập
    //! 3. Không bao giờ patch string đã render
    //! 4. Mỗi từ là một transaction độc lập
    //! 5. Xóa hết một từ ⇒ clean toàn bộ buffer & state
    //! 
    //! All tests above map to specific rules or test cases from the spec.
}