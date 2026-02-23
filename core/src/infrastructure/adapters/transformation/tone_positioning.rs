//! Vietnamese Tone Mark Positioning
//!
//! This module implements the definitive rules for placing tone marks (dấu thanh)
//! on Vietnamese syllables. The position is determined by phonological structure,
//! NOT by input order.
//!
//! ## Core Principle
//! **Tone marks are placed according to Vietnamese orthography rules,
//! independent of Telex/VNI keystroke order.**
//!
//! ## Priority Rules (from highest to lowest)
//!
//! ### Rule 1: Diacritic Priority (HIGHEST)
//! If the vowel cluster contains â/ê/ô/ơ/ư → place mark on that vowel
//! - `viết` → mark on `ê` (not `i`)
//! - `quốc` → mark on `ô` (not `u`)
//! - `lưỡng` → mark on `ơ` (not `u` or `ng`)
//!
//! ### Rule 2: Second Vowel Rule
//! If no diacritics present → place mark on SECOND vowel
//! - `hoá` → mark on `a` (not `o`)
//! - `loé` → mark on `e` (not `o`)
//! - `tuý` → mark on `y` (not `u`)
//!
//! ### Rule 3: Final Consonant Context
//! With final consonants, mark stays on main vowel even with diacritics
//! - `biển` → mark on `ê`
//! - `luyến` → mark on `ê`
//! - `mượn` → mark on `ơ`
//!
//! ## Dynamic Repositioning
//! When vowel structure changes (e.g., `vie` → `viet`), the mark position
//! must be recalculated:
//! ```text
//! v i e + s  → vié  (mark on e, no diacritic)
//! vié + t    → viét (mark stays on e, final consonant added)
//! vie + e    → viê  (e becomes ê)
//! viê + s    → viết (mark moves to ê, Rule 1 applies)
//! ```

use crate::data::{
    keys,
    vowel::{Modifier, Vowel},
};
use crate::shared::buffer::Buffer;

use crate::utils;

/// Find the correct position for tone mark in a vowel cluster
///
/// # Arguments
/// * `vowels` - Vowel cluster with positional information
/// * `has_final_consonant` - Whether syllable has final consonant
///
/// # Returns
/// Buffer position where tone mark should be placed
///
/// # Algorithm
/// 1. Check for diacritic vowels (â/ê/ô/ơ/ư) - Rule 1
/// 2. If none, use second vowel if available - Rule 2
/// 3. Handle special cases with final consonants - Rule 3
pub fn find_mark_position(vowels: &[Vowel], _has_final_consonant: bool) -> usize {
    if vowels.is_empty() {
        return 0;
    }

    // Single vowel - mark goes on it
    if vowels.len() == 1 {
        return vowels[0].pos;
    }

    // ═════════════════════════════════════════════════════════════════════
    // RULE 1: DIACRITIC PRIORITY (HIGHEST)
    // ═════════════════════════════════════════════════════════════════════
    // If cluster contains â/ê/ô/ơ/ư, mark goes there
    // If multiple diacritics exist (e.g., ươi), prioritize by position:
    // - For triphthongs: prefer middle diacritic (ơ in ươi)
    // - For diphthongs: prefer last diacritic (ê in iê)

    let mut last_diacritic_idx: Option<usize> = None;
    let mut middle_diacritic = false;

    for (i, v) in vowels.iter().enumerate() {
        if is_diacritic_vowel(v.key, &v.modifier) {
            last_diacritic_idx = Some(i);
            if vowels.len() == 3 && i == 1 {
                middle_diacritic = true;
            }
        }
    }

    if let Some(idx) = last_diacritic_idx {
        // For triphthongs with multiple diacritics, prefer middle
        if vowels.len() == 3 && middle_diacritic {
            return vowels[1].pos;
        }

        // Default: use last diacritic vowel (works for diphthongs and most cases)
        return vowels[idx].pos;
    }

    // ═════════════════════════════════════════════════════════════════════
    // RULE 2: SECOND VOWEL RULE
    // ═════════════════════════════════════════════════════════════════════
    // No diacritics present → mark on second vowel
    // Applies to: ai, ao, ay, eo, oa, oe, oi, ui, uy, etc.

    if vowels.len() >= 2 {
        // For diphthongs (2 vowels)
        if vowels.len() == 2 {
            return vowels[1].pos;
        }

        // For triphthongs (3+ vowels)
        // Middle vowel gets priority in compound structures
        if vowels.len() == 3 {
            // Check if middle vowel should get mark
            // (e.g., uôi → middle, uyê → last if ê present but already handled by Rule 1)
            return vowels[1].pos;
        }

        // 4+ vowels: rare, use middle
        return vowels[vowels.len() / 2].pos;
    }

    // Default fallback (should not reach here)
    vowels[0].pos
}

/// Check if a vowel has diacritic modifier (â/ê/ô/ơ/ư/ă)
///
/// # Arguments
/// * `key` - Base vowel key (A, E, O, U)
/// * `modifier` - Tone modifier applied to base
///
/// # Returns
/// `true` if vowel is â/ê/ô/ơ/ư/ă
fn is_diacritic_vowel(key: u16, modifier: &Modifier) -> bool {
    // No modifier = no diacritic
    if *modifier == Modifier::None {
        return false;
    }

    // Circumflex (^): â, ê, ô
    if *modifier == Modifier::Circumflex {
        return matches!(key, keys::A | keys::E | keys::O);
    }

    // Horn/Breve: ơ, ư, ă
    if *modifier == Modifier::Horn {
        return matches!(key, keys::A | keys::O | keys::U);
    }

    false
}

/// Reposition tone mark after vowel structure change
///
/// This is called when:
/// - A tone modifier is added (e.g., `e` + `e` → `ê`)
/// - A vowel is added/removed
/// - Backspace modifies the cluster
///
/// # Arguments
/// * `buf` - Mutable buffer to update
///
/// # Returns
/// `(old_pos, new_pos)` if mark was moved, `None` if no change
pub fn reposition_mark(buf: &mut Buffer) -> Option<(usize, usize)> {
    // Find current mark position and value
    let mark_info: Option<(usize, u8)> = buf
        .iter()
        .enumerate()
        .find(|(_, c)| c.mark > 0)
        .map(|(i, c)| (i, c.mark));

    let (old_pos, mark_value) = mark_info?;

    // Collect current vowel structure
    let vowels = utils::collect_vowels(buf);
    if vowels.is_empty() {
        return None;
    }

    // Calculate correct position using phonology rules
    let last_vowel_pos = vowels.last().map(|v| v.pos).unwrap_or(0);
    let has_final = utils::has_final_consonant(buf, last_vowel_pos);
    let new_pos = find_mark_position(&vowels, has_final);

    // Move mark if position changed
    if new_pos != old_pos {
        // Clear old position
        if let Some(c) = buf.get_mut(old_pos) {
            c.mark = 0;
        }

        // Set new position
        if let Some(c) = buf.get_mut(new_pos) {
            c.mark = mark_value;
        }

        return Some((old_pos, new_pos));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::chars::mark;
    use crate::shared::buffer::{Buffer, Char};

    /// Helper: Create buffer from string
    fn setup_buffer(s: &str) -> Buffer {
        let mut buf = Buffer::new();
        for ch in s.chars() {
            let key = match ch.to_ascii_lowercase() {
                'a' => keys::A,
                'b' => keys::B,
                'c' => keys::C,
                'd' => keys::D,
                'e' => keys::E,
                'g' => keys::G,
                'h' => keys::H,
                'i' => keys::I,
                'l' => keys::L,
                'n' => keys::N,
                'o' => keys::O,
                'q' => keys::Q,
                't' => keys::T,
                'u' => keys::U,
                'v' => keys::V,
                'w' => keys::W,
                'y' => keys::Y,
                _ => continue,
            };
            buf.push(Char::new(key, ch.is_uppercase()));
        }
        buf
    }

    /// Helper: Create vowel with modifier
    fn vowel_with_modifier(key: u16, modifier_type: u8, pos: usize) -> Vowel {
        use crate::data::chars::tone as tone_const;
        let mod_enum = match modifier_type {
            tone_const::CIRCUMFLEX => Modifier::Circumflex,
            tone_const::HORN => Modifier::Horn,
            _ => Modifier::None,
        };
        Vowel {
            key,
            modifier: mod_enum,
            pos,
        }
    }

    #[test]
    fn test_rule1_diacritic_priority_circumflex() {
        use crate::data::chars::tone;
        // Test: iê cluster → mark on ê (has circumflex)
        let vowels = vec![
            vowel_with_modifier(keys::I, tone::NONE, 1),
            vowel_with_modifier(keys::E, tone::CIRCUMFLEX, 2),
        ];

        let pos = find_mark_position(&vowels, false);
        assert_eq!(pos, 2, "Mark should be on ê (Rule 1: diacritic priority)");
    }

    #[test]
    fn test_rule1_diacritic_priority_horn() {
        use crate::data::chars::tone;
        // Test: uô cluster → mark on ô
        let vowels = vec![
            vowel_with_modifier(keys::U, tone::NONE, 1),
            vowel_with_modifier(keys::O, tone::CIRCUMFLEX, 2),
        ];

        let pos = find_mark_position(&vowels, false);
        assert_eq!(pos, 2, "Mark should be on ô (Rule 1: diacritic priority)");
    }

    #[test]
    fn test_rule1_triphthong_with_diacritic() {
        // Test: ươi cluster → mark on ơ (middle has diacritic, both ư and ơ have horn)
        // When multiple diacritics in triphthong, prefer middle
        use crate::data::chars::tone;
        let vowels = vec![
            vowel_with_modifier(keys::U, tone::HORN, 0), // ư
            vowel_with_modifier(keys::O, tone::HORN, 1), // ơ
            vowel_with_modifier(keys::I, tone::NONE, 2),
        ];

        let pos = find_mark_position(&vowels, false);
        assert_eq!(
            pos, 1,
            "Mark should be on ơ (middle diacritic in triphthong)"
        );
    }

    #[test]
    fn test_rule2_second_vowel_no_diacritic() {
        use crate::data::chars::tone;
        // Test: oa cluster (no diacritics) → mark on a (second)
        let vowels = vec![
            vowel_with_modifier(keys::O, tone::NONE, 1),
            vowel_with_modifier(keys::A, tone::NONE, 2),
        ];

        let pos = find_mark_position(&vowels, false);
        assert_eq!(pos, 2, "Mark should be on a (Rule 2: second vowel)");
    }

    #[test]
    fn test_rule2_ai_cluster() {
        use crate::data::chars::tone;
        // Test: ai cluster → mark on i
        let vowels = vec![
            vowel_with_modifier(keys::A, tone::NONE, 0),
            vowel_with_modifier(keys::I, tone::NONE, 1),
        ];

        let pos = find_mark_position(&vowels, false);
        assert_eq!(pos, 1, "Mark should be on i (Rule 2: second vowel)");
    }

    #[test]
    fn test_rule2_uy_cluster() {
        use crate::data::chars::tone;
        // Test: uy cluster → mark on y
        let vowels = vec![
            vowel_with_modifier(keys::U, tone::NONE, 0),
            vowel_with_modifier(keys::Y, tone::NONE, 1),
        ];

        let pos = find_mark_position(&vowels, false);
        assert_eq!(pos, 1, "Mark should be on y (Rule 2: second vowel)");
    }

    #[test]
    fn test_single_vowel() {
        use crate::data::chars::tone;
        // Test: single vowel → mark on it
        let vowels = vec![vowel_with_modifier(keys::A, tone::NONE, 0)];

        let pos = find_mark_position(&vowels, false);
        assert_eq!(pos, 0, "Mark should be on single vowel");
    }

    #[test]
    fn test_is_diacritic_vowel() {
        // Test circumflex diacritics
        assert!(
            is_diacritic_vowel(keys::A, &Modifier::Circumflex),
            "â is diacritic"
        );
        assert!(
            is_diacritic_vowel(keys::E, &Modifier::Circumflex),
            "ê is diacritic"
        );
        assert!(
            is_diacritic_vowel(keys::O, &Modifier::Circumflex),
            "ô is diacritic"
        );

        // Test horn/breve diacritics
        assert!(
            is_diacritic_vowel(keys::A, &Modifier::Horn),
            "ă is diacritic"
        );
        assert!(
            is_diacritic_vowel(keys::O, &Modifier::Horn),
            "ơ is diacritic"
        );
        assert!(
            is_diacritic_vowel(keys::U, &Modifier::Horn),
            "ư is diacritic"
        );

        // Test non-diacritics
        assert!(
            !is_diacritic_vowel(keys::A, &Modifier::None),
            "a is not diacritic"
        );
        assert!(
            !is_diacritic_vowel(keys::I, &Modifier::None),
            "i is not diacritic"
        );
        assert!(
            !is_diacritic_vowel(keys::I, &Modifier::Circumflex),
            "i with ^ is invalid"
        );
    }

    #[test]
    fn test_reposition_mark_after_tone_change() {
        use crate::data::chars::tone;
        // Simulate: "vie" + s → "vié", then + e → "viê" + s → "viết"
        let mut buf = setup_buffer("vie");

        // Add tone to 'e' to make 'ê'
        if let Some(c) = buf.get_mut(2) {
            c.tone = tone::CIRCUMFLEX; // e → ê
        }

        // Add mark to first vowel 'i' (simulating wrong position)
        if let Some(c) = buf.get_mut(1) {
            c.mark = mark::SAC;
        }

        // Reposition should move mark from 'i' to 'ê'
        let result = reposition_mark(&mut buf);
        assert!(result.is_some(), "Mark should be repositioned");

        let (old_pos, new_pos) = result.unwrap();
        assert_eq!(old_pos, 1, "Mark was on i (pos 1)");
        assert_eq!(new_pos, 2, "Mark moved to ê (pos 2)");

        // Verify final state
        assert_eq!(buf.get(1).unwrap().mark, 0, "i should have no mark");
        assert_eq!(buf.get(2).unwrap().mark, mark::SAC, "ê should have mark");
    }

    #[test]
    fn test_reposition_no_change_needed() {
        // Test: mark already in correct position
        let mut buf = setup_buffer("hoa");

        // Add mark to 'a' (already correct position by Rule 2)
        if let Some(c) = buf.get_mut(2) {
            c.mark = mark::SAC;
        }

        let result = reposition_mark(&mut buf);
        assert!(
            result.is_none(),
            "No repositioning needed when already correct"
        );
    }
}
