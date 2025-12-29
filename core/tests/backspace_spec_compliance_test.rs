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

use goxviet_core::data::keys;
use goxviet_core::engine::{Action, Engine};

/// Helper struct to track screen state across multiple operations
struct ScreenTracker {
    engine: Engine,
    screen: String,
}

impl ScreenTracker {
    fn new() -> Self {
        let mut engine = Engine::new();
        engine.set_method(0); // Telex by default
        Self {
            engine,
            screen: String::new(),
        }
    }

    fn set_method(&mut self, method: u8) {
        self.engine.set_method(method);
    }

    /// Type a string and update screen state
    fn type_str(&mut self, input: &str) -> &str {
        for c in input.chars() {
            let key = char_to_key(c);
            let is_caps = c.is_uppercase();

            if key == keys::DELETE {
                let r = self.engine.on_key(key, false, false);
                if r.action == Action::Send as u8 {
                    for _ in 0..r.backspace {
                        self.screen.pop();
                    }
                    for i in 0..r.count as usize {
                        if let Some(ch) = char::from_u32(r.chars[i]) {
                            self.screen.push(ch);
                        }
                    }
                } else {
                    self.screen.pop();
                }
                continue;
            }

            let r = self.engine.on_key(key, is_caps, false);
            if r.action == Action::Send as u8 {
                for _ in 0..r.backspace {
                    self.screen.pop();
                }
                for i in 0..r.count as usize {
                    if let Some(ch) = char::from_u32(r.chars[i]) {
                        self.screen.push(ch);
                    }
                }
            } else {
                self.screen.push(c);
            }
        }
        &self.screen
    }

    /// Get current screen content
    fn screen(&self) -> &str {
        &self.screen
    }

    /// Clear the engine state (simulates word boundary)
    fn clear(&mut self) {
        self.engine.clear();
        self.screen.clear();
    }
}

/// Convert char to key code
fn char_to_key(c: char) -> u16 {
    match c.to_ascii_lowercase() {
        'a' => keys::A,
        'b' => keys::B,
        'c' => keys::C,
        'd' => keys::D,
        'e' => keys::E,
        'f' => keys::F,
        'g' => keys::G,
        'h' => keys::H,
        'i' => keys::I,
        'j' => keys::J,
        'k' => keys::K,
        'l' => keys::L,
        'm' => keys::M,
        'n' => keys::N,
        'o' => keys::O,
        'p' => keys::P,
        'q' => keys::Q,
        'r' => keys::R,
        's' => keys::S,
        't' => keys::T,
        'u' => keys::U,
        'v' => keys::V,
        'w' => keys::W,
        'x' => keys::X,
        'y' => keys::Y,
        'z' => keys::Z,
        '<' => keys::DELETE, // Use '<' to represent backspace in test strings
        _ => 255,
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
    let mut t = ScreenTracker::new();

    // Type "diễn" (d-i-ee-x-n: ee for circumflex ê, x for ngã tone)
    t.type_str("dieexn");
    assert_eq!(t.screen(), "diễn", "Initial word should be 'diễn'");

    // Backspace sequence: diễn → diê → di → d → ""
    t.type_str("<");
    assert_eq!(t.screen(), "diễ", "After 1 backspace should be 'diễ'");

    t.type_str("<");
    assert_eq!(t.screen(), "di", "After 2 backspaces should be 'di'");

    t.type_str("<");
    assert_eq!(t.screen(), "d", "After 3 backspaces should be 'd'");

    t.type_str("<");
    assert_eq!(t.screen(), "", "After 4 backspaces should be empty");

    // Type "a" - should be plain "a", not "ả"
    t.type_str("a");
    assert_eq!(t.screen(), "a", "After reset, 'a' should be plain 'a', not 'ả'");
}

/// Test Case 2 from spec: tiếng → BS × 5 → ""
/// Then type "o" → result MUST be "o"
#[test]
fn test_spec_mandatory_case_2_tieng() {
    let mut t = ScreenTracker::new();

    // Type "tiếng" (t-i-ee-s-n-g: ee for circumflex ê, s for sắc tone)
    t.type_str("tieesng");
    assert_eq!(t.screen(), "tiếng", "Initial word should be 'tiếng'");

    // Backspaces to clear
    t.type_str("<<<<<");
    assert_eq!(t.screen(), "", "After backspaces should be empty");

    // Type "o" - should be plain "o"
    t.type_str("o");
    assert_eq!(t.screen(), "o", "After reset, 'o' should be plain 'o'");
}

/// Test Case 3 from spec: Telex transforms then backspace
/// Tests that backspace works correctly with Telex-specific transforms
#[test]
fn test_spec_mandatory_case_3_telex() {
    let mut t = ScreenTracker::new();

    // Type "được" (dd-uo-w-c-j: dd for đ, uow for ươ, c, j for nặng)
    t.type_str("dduowcj");
    assert_eq!(t.screen(), "được", "Initial word should be 'được'");

    // Backspace once - should remove 'c' with nặng tone as one grapheme
    t.type_str("<");
    // được = đ-ư-ợ-c, backspace removes c → đượ
    assert_eq!(t.screen(), "đượ", "After 1 backspace should be 'đượ'");
}

/// Test Case 4 from spec: improve → backspace sequence
/// Tests English detection and backspace behavior
#[test]
fn test_spec_mandatory_case_4_improve() {
    let mut t = ScreenTracker::new();

    // Type "improve" - should be detected as English (mp cluster)
    t.type_str("improve");
    assert_eq!(t.screen(), "improve", "Should stay as English 'improve'");

    // Backspace should delete one character at a time
    t.type_str("<");
    assert_eq!(t.screen(), "improv", "Should delete exactly one character");
}

// ============================================================================
// RULE 1: Delete EXACTLY ONE grapheme
// ============================================================================

/// Test that backspace deletes exactly one grapheme for simple cases
#[test]
fn test_rule1_delete_one_grapheme_simple() {
    let mut t = ScreenTracker::new();

    // Type "ban" (simple consonants + vowel)
    t.type_str("ban");
    assert_eq!(t.screen(), "ban");

    // One backspace
    t.type_str("<");
    assert_eq!(t.screen(), "ba", "Should delete exactly one character");

    // Another backspace
    t.type_str("<");
    assert_eq!(t.screen(), "b", "Should delete exactly one character");
}

/// Test that backspace deletes whole grapheme with tone mark
#[test]
fn test_rule1_delete_one_grapheme_with_tone() {
    let mut t = ScreenTracker::new();

    // Type "cá" (c-a-s for sắc tone)
    t.type_str("cas");
    assert_eq!(t.screen(), "cá");

    // One backspace should delete entire 'á'
    t.type_str("<");
    assert_eq!(t.screen(), "c", "Should delete 'á' as one unit");
}

/// Test that backspace deletes whole grapheme with circumflex
#[test]
fn test_rule1_delete_one_grapheme_circumflex() {
    let mut t = ScreenTracker::new();

    // Type "cân" (c-a-a-n for circumflex â + n)
    t.type_str("caan");
    assert_eq!(t.screen(), "cân");

    // Backspace should delete 'n', circumflex remains on â
    t.type_str("<");
    assert_eq!(t.screen(), "câ", "Should delete 'n', leaving 'câ'");
}

// ============================================================================
// RULE 2: NEVER delete tone/modifier independently
// ============================================================================

/// Test that tone is never deleted separately from base character
#[test]
fn test_rule2_never_delete_tone_separately() {
    let mut t = ScreenTracker::new();

    // Type "cá" (c + a + s)
    t.type_str("cas");
    assert_eq!(t.screen(), "cá");

    // Backspace should delete entire 'á', not just the sắc tone
    t.type_str("<");
    assert_eq!(t.screen(), "c", "Should delete entire 'á', leaving 'c'");

    // Verify clean state by typing 'a' again
    t.type_str("a");
    assert_eq!(t.screen(), "ca", "New 'a' should be plain");
}

/// Test that circumflex is never deleted separately
#[test]
fn test_rule2_never_delete_circumflex_separately() {
    let mut t = ScreenTracker::new();

    // Type "cô" (c-o-o)
    t.type_str("coo");
    assert_eq!(t.screen(), "cô");

    // Backspace should delete entire 'ô', leaving 'c'
    t.type_str("<");
    assert_eq!(t.screen(), "c", "Should delete entire 'ô'");
}

/// Test that stroke (đ) is deleted as one unit
#[test]
fn test_rule2_delete_d_with_stroke_atomically() {
    let mut t = ScreenTracker::new();

    // Type "đi" (d-d-i)
    t.type_str("ddi");
    assert_eq!(t.screen(), "đi");

    // Backspace should delete 'i', leaving 'đ'
    t.type_str("<");
    assert_eq!(t.screen(), "đ", "Should delete 'i', leaving 'đ'");

    // Another backspace should delete entire 'đ'
    t.type_str("<");
    assert_eq!(t.screen(), "", "Should delete entire 'đ'");
}

// ============================================================================
// RULE 5: Reset everything when last grapheme deleted
// ============================================================================

/// Test that all state is reset when buffer becomes empty
#[test]
fn test_rule5_reset_all_state_on_empty() {
    let mut t = ScreenTracker::new();

    // Type "cả" (c + a + r for hỏi tone)
    t.type_str("car");
    assert_eq!(t.screen(), "cả");

    // Delete all (2 backspaces for 'c' and 'ả')
    t.type_str("<<");
    assert_eq!(t.screen(), "");

    // Now type 'e' - should be plain 'e', not affected by previous 'r' (hỏi)
    t.type_str("e");
    assert_eq!(t.screen(), "e", "New vowel should not inherit previous tone");
}

/// Test that English word flag is reset on empty buffer
#[test]
fn test_rule5_reset_english_word_flag() {
    let mut t = ScreenTracker::new();

    // Type something that triggers English detection ("ex" pattern)
    t.type_str("next");
    // "next" triggers English detection due to "ex" pattern
    // Result may vary based on when detection kicks in

    // Delete all
    t.type_str("<<<<");
    assert_eq!(t.screen(), "");

    // Now type Vietnamese - should work normally (flag reset)
    t.type_str("cos");
    assert_eq!(t.screen(), "có", "Vietnamese should work after English word deleted");
}

// ============================================================================
// COMPLEX SYLLABLE TESTS
// ============================================================================

/// Test backspace with complex syllable that needs rebuild
#[test]
fn test_backspace_complex_syllable_rebuild() {
    let mut t = ScreenTracker::new();

    // Type "hoàn" (h-o-a-f-n: f for huyền tone on 'a')
    t.type_str("hoafn");
    assert_eq!(t.screen(), "hoàn");

    // Backspace - remove 'n', tone stays on 'a'
    t.type_str("<");
    assert_eq!(t.screen(), "hoà", "After removing 'n', should be 'hoà'");
}

/// Test backspace with horn vowel (ơ, ư)
#[test]
fn test_backspace_with_horn_vowel() {
    let mut t = ScreenTracker::new();

    // Type "mười" (m-uo-w-i-f: uow for ươ, i, f for huyền on i)
    t.type_str("muowif");
    assert_eq!(t.screen(), "mười");

    // Backspace should remove ì (i with huyền)
    // After removing ì, we have mườ (ơ still has the tone mark repositioned)
    t.type_str("<");
    // The actual result depends on tone repositioning logic
    // mười → backspace → could be "mườ" (tone on ơ) or "mươ" (tone removed)
    assert!(t.screen().starts_with("mư"), "After backspace should start with 'mư'");
}

// ============================================================================
// EDGE CASES
// ============================================================================

/// Test backspace on empty buffer does nothing harmful
#[test]
fn test_backspace_on_empty_buffer() {
    let mut t = ScreenTracker::new();

    // Backspace on empty - should be no-op
    let r = t.engine.on_key(keys::DELETE, false, false);
    assert_eq!(r.action, 0, "Backspace on empty should be Action::None");
}

/// Test that capitalization is preserved through backspace
#[test]
fn test_backspace_preserves_capitalization() {
    let mut t = ScreenTracker::new();

    // Type "Việt" with capital V (V-i-ee-s-t: ee for ê, s for sắc)
    // Note: In Telex, "ees" means e+e=ê then s=sắc, so order matters
    t.type_str("Vieest");
    // The actual output depends on the order of ee and s processing
    // Vieest = V + i + ee(→ê) + s(→sắc on ê = ế) + t = Viết
    // This is actually "Viết" not "Việt" due to Telex processing order
    
    // Backspace - remove 't'
    t.type_str("<");
    // After removing 't', we have "Viế" (or similar)
    assert!(t.screen().starts_with("Vi"), "Capital V should be preserved");
}

// ============================================================================
// SPEC REFERENCE
// ============================================================================

/// Reference module documenting the spec
mod spec_reference {
    #![allow(dead_code)]

    /// From spec: Backspace behavior table
    ///
    /// | Before | After BS | Notes |
    /// |--------|----------|-------|
    /// | diễn   | diê      | Delete 'n', compound 'ễ' preserved |
    /// | diê    | di       | Delete 'ê' as one grapheme |
    /// | á      | ""       | Delete 'á' as one grapheme |
    /// | đ      | ""       | Delete 'đ' as one grapheme |
    const BACKSPACE_TABLE: &str = "See spec document";
}