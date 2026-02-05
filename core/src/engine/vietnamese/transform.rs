//! Vietnamese Transformation
//!
//! Pattern-based transformation for Vietnamese diacritics.
//! Scans entire buffer instead of case-by-case processing.

use crate::data::{
    chars::{mark, tone},
    keys,
    vowel::Phonology,
};
use crate::engine::buffer::{Buffer, MAX};
use crate::engine::vietnamese::tone_positioning;
use crate::utils;

/// Modifier type detected from key
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModifierType {
    /// Tone diacritic: circumflex (^), horn, breve
    Tone(u8),
    /// Tone mark: sắc, huyền, hỏi, ngã, nặng
    Mark(u8),
    /// Stroke: d → đ
    Stroke,
    /// Remove last diacritic
    Remove,
}

/// Transformation result
#[derive(Debug)]
pub struct TransformResult {
    /// Positions that were modified
    pub modified_positions: Vec<usize>,
    /// Whether transformation was applied
    pub applied: bool,
}

impl TransformResult {
    pub fn none() -> Self {
        Self {
            modified_positions: vec![],
            applied: false,
        }
    }

    pub fn success(positions: Vec<usize>) -> Self {
        Self {
            modified_positions: positions,
            applied: true,
        }
    }

    pub fn earliest_position(&self) -> Option<usize> {
        self.modified_positions.iter().copied().min()
    }
}

/// Apply tone diacritic transformation (^, ơ, ư, ă)
///
/// Pattern-based: scans buffer for matching vowels
pub fn apply_tone(buf: &mut Buffer, key: u16, tone_value: u8, method: u8) -> TransformResult {
    // Find target vowels based on key and method
    let targets = find_tone_targets(buf, key, tone_value, method);

    if targets.is_empty() {
        return TransformResult::none();
    }

    // Apply tone to targets
    let mut positions = vec![];
    for pos in &targets {
        if let Some(c) = buf.get_mut(*pos) {
            if c.tone == tone::NONE {
                c.tone = tone_value;
                positions.push(*pos);
            }
        }
    }

    if positions.is_empty() {
        TransformResult::none()
    } else {
        // After adding tone, reposition mark if needed (Rule 1: diacritic priority)
        if let Some((old_pos, new_pos)) = tone_positioning::reposition_mark(buf) {
            // Mark moved: add both old and new positions to modified list
            // to ensure UI updates both characters
            if !positions.contains(&old_pos) {
                positions.push(old_pos);
            }
            if !positions.contains(&new_pos) {
                positions.push(new_pos);
            }
        }
        TransformResult::success(positions)
    }
}

/// Find which vowel positions should receive the tone modifier
fn find_tone_targets(buf: &Buffer, key: u16, tone_value: u8, method: u8) -> Vec<usize> {
    let mut targets = Vec::with_capacity(buf.len());

    // Build buffer_keys array for phonology checks
    let len = buf.len();
    let mut buffer_keys = [0u16; MAX];
    for i in 0..len {
        buffer_keys[i] = buf.get(i).map(|c| c.key).unwrap_or(0);
    }
    let buffer_keys = &buffer_keys[..len];

    // Find all vowel positions
    let mut vowel_positions = Vec::with_capacity(len);
    for (i, &k) in buffer_keys.iter().enumerate() {
        if keys::is_vowel(k) {
            vowel_positions.push(i);
        }
    }

    if vowel_positions.is_empty() {
        return targets;
    }

    // Telex patterns
    if method == 0 {
        // aa, ee, oo → circumflex (immediate doubling only)
        // The target vowel must be at the LAST position in the buffer
        // This ensures "ee" doubling only works for consecutive presses,
        // not for words like "teacher" where 'e' appears twice non-adjacently
        if tone_value == tone::CIRCUMFLEX && matches!(key, keys::A | keys::E | keys::O) {
            // If ANY vowel in the buffer already has a tone (horn/circumflex/breve),
            // don't trigger same-vowel circumflex. The typed vowel should append as raw letter.
            // Example: "chưa" + "a" → "chưaa" (NOT "chưâ")
            // Also check for marks (sắc/huyền/hỏi/ngã/nặng) - if a vowel already
            // has a mark, the typed vowel should append raw for extended vowel patterns.
            // Example: "quá" + "a" → "quáa" (NOT "quấ")
            let any_vowel_has_tone_or_mark = buf
                .iter()
                .filter(|c| keys::is_vowel(c.key))
                .any(|c| c.has_tone() || c.has_mark());

            if any_vowel_has_tone_or_mark {
                // Skip circumflex, return empty targets to append raw vowel
                return targets;
            }

            // Find matching vowel (same key) - must be at last position
            let last_pos = buf.len().saturating_sub(1);
            for &pos in vowel_positions.iter().rev() {
                if buffer_keys[pos] == key && pos == last_pos {
                    targets.push(pos);
                    break;
                }
            }
        }
        // w → horn/breve
        else if tone_value == tone::HORN && key == keys::W {
            targets = Phonology::find_horn_positions(buffer_keys, &vowel_positions);
        }
    }
    // VNI patterns
    else {
        // 6 → circumflex for a, e, o
        if tone_value == tone::CIRCUMFLEX && key == keys::N6 {
            for &pos in vowel_positions.iter().rev() {
                if matches!(buffer_keys[pos], keys::A | keys::E | keys::O) {
                    targets.push(pos);
                    break;
                }
            }
        }
        // 7 → horn for o, u
        else if tone_value == tone::HORN && key == keys::N7 {
            targets = Phonology::find_horn_positions(buffer_keys, &vowel_positions);
        }
        // 8 → breve for a only
        else if tone_value == tone::HORN && key == keys::N8 {
            for &pos in vowel_positions.iter().rev() {
                if buffer_keys[pos] == keys::A {
                    targets.push(pos);
                    break;
                }
            }
        }
    }

    targets
}

/// Apply mark transformation (sắc, huyền, hỏi, ngã, nặng)
///
/// Uses tone_positioning module for accurate mark placement based on
/// Vietnamese phonology rules (see tone_positioning.rs for details).
pub fn apply_mark(buf: &mut Buffer, mark_value: u8, _modern: bool) -> TransformResult {
    let vowels = utils::collect_vowels(buf);
    if vowels.is_empty() {
        return TransformResult::none();
    }

    // Find position using phonology rules
    // Note: We still use Phonology for complex cases (qu/gi initial, modern/traditional)
    // but tone_positioning handles the core diacritic priority logic
    let last_vowel_pos = vowels.last().map(|v| v.pos).unwrap_or(0);
    let has_final = utils::has_final_consonant(buf, last_vowel_pos);

    // Use simplified positioning that prioritizes diacritics (Rule 1)
    let pos = tone_positioning::find_mark_position(&vowels, has_final);

    // Clear any existing mark first
    for v in &vowels {
        if let Some(c) = buf.get_mut(v.pos) {
            c.mark = mark::NONE;
        }
    }

    // Apply new mark
    if let Some(c) = buf.get_mut(pos) {
        c.mark = mark_value;
        return TransformResult::success(vec![pos]);
    }

    TransformResult::none()
}

/// Apply stroke transformation (d → đ)
///
/// Scans buffer for 'd' at any position
pub fn apply_stroke(buf: &mut Buffer) -> TransformResult {
    // Find first 'd' that hasn't been stroked
    for i in 0..buf.len() {
        if let Some(c) = buf.get_mut(i) {
            if c.key == keys::D && !c.stroke {
                c.stroke = true;
                return TransformResult::success(vec![i]);
            }
        }
    }
    TransformResult::none()
}

/// Remove last diacritic (mark first, then tone)
pub fn apply_remove(buf: &mut Buffer) -> TransformResult {
    // Walk backwards to remove mark first, then tone, without allocating vowel list
    for idx in (0..buf.len()).rev() {
        if let Some(c) = buf.get_mut(idx) {
            if keys::is_vowel(c.key) && c.mark > mark::NONE {
                c.mark = mark::NONE;
                return TransformResult::success(vec![idx]);
            }
        }
    }

    for idx in (0..buf.len()).rev() {
        if let Some(c) = buf.get_mut(idx) {
            if keys::is_vowel(c.key) && c.tone > tone::NONE {
                c.tone = tone::NONE;
                return TransformResult::success(vec![idx]);
            }
        }
    }

    TransformResult::none()
}

/// Revert tone transformation
pub fn revert_tone(buf: &mut Buffer, target_key: u16) -> TransformResult {
    for idx in (0..buf.len()).rev() {
        if let Some(c) = buf.get_mut(idx) {
            if c.key == target_key && keys::is_vowel(c.key) && c.tone > tone::NONE {
                c.tone = tone::NONE;
                return TransformResult::success(vec![idx]);
            }
        }
    }

    TransformResult::none()
}

/// Revert mark transformation
pub fn revert_mark(buf: &mut Buffer) -> TransformResult {
    for idx in (0..buf.len()).rev() {
        if let Some(c) = buf.get_mut(idx) {
            if keys::is_vowel(c.key) && c.mark > mark::NONE {
                c.mark = mark::NONE;
                return TransformResult::success(vec![idx]);
            }
        }
    }

    TransformResult::none()
}

/// Revert stroke transformation
pub fn revert_stroke(buf: &mut Buffer) -> TransformResult {
    // Find stroked 'd' and un-stroke it
    for i in 0..buf.len() {
        if let Some(c) = buf.get_mut(i) {
            if c.key == keys::D && c.stroke {
                c.stroke = false;
                return TransformResult::success(vec![i]);
            }
        }
    }
    TransformResult::none()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::buffer::Char;

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

    #[test]
    fn test_apply_stroke() {
        let mut buf = setup_buffer("do");
        let result = apply_stroke(&mut buf);
        assert!(result.applied);
        assert!(buf.get(0).unwrap().stroke);
    }

    #[test]
    fn test_apply_stroke_anywhere() {
        // "dod" should stroke the first 'd'
        let mut buf = setup_buffer("dod");
        let result = apply_stroke(&mut buf);
        assert!(result.applied);
        assert!(buf.get(0).unwrap().stroke); // First d is stroked
    }

    #[test]
    fn test_apply_mark() {
        let mut buf = setup_buffer("an");
        let result = apply_mark(&mut buf, mark::SAC, true);
        assert!(result.applied);
        assert_eq!(buf.get(0).unwrap().mark, mark::SAC);
    }

    #[test]
    fn test_uo_compound() {
        let mut buf = setup_buffer("duoc");
        let result = apply_tone(&mut buf, keys::W, tone::HORN, 0);
        assert!(result.applied);
        // Both u and o should have horn
        assert_eq!(buf.get(1).unwrap().tone, tone::HORN); // u
        assert_eq!(buf.get(2).unwrap().tone, tone::HORN); // o
    }

    #[test]
    fn test_mark_repositioning_after_tone_added() {
        // Test case: ie cluster (setup_buffer only includes vowels)
        // Buffer positions: i(0), e(1)
        // Note: setup_buffer skips consonants, so "vie" becomes just [i, e]
        let mut buf = setup_buffer("ie");

        // Apply mark - should go on 'e' at position 1 (second vowel, Rule 2)
        let result = apply_mark(&mut buf, mark::SAC, true);
        assert!(result.applied);

        // Find which vowel has the mark
        let marked_pos = buf
            .iter()
            .enumerate()
            .find(|(_, c)| c.mark > 0)
            .map(|(i, _)| i);

        assert!(marked_pos.is_some(), "A vowel should have the mark");
        // For "ie": i(0) and e(1) are vowels, mark should be on e(1) by Rule 2
        assert_eq!(
            marked_pos.unwrap(),
            1,
            "Mark should be on 'e' (second vowel)"
        );
    }

    #[test]
    fn test_mark_on_second_vowel_no_diacritic() {
        // Test: hoa + s → hoá (mark on 'a', Rule 2)
        let mut buf = setup_buffer("hoa");
        let result = apply_mark(&mut buf, mark::SAC, true);
        assert!(result.applied);
        assert_eq!(
            buf.get(2).unwrap().mark,
            mark::SAC,
            "Mark on 'a' (second vowel)"
        );
    }

    #[test]
    fn test_mark_on_diacritic_in_compound() {
        // Test: uo compound vowel
        // Buffer: u(0), o(1)
        let mut buf = setup_buffer("uo");

        // Add circumflex to 'o' to make 'ô'
        if let Some(c) = buf.get_mut(1) {
            c.tone = tone::CIRCUMFLEX; // o → ô
        }

        // Apply mark - should go on ô (position 1) by Rule 1
        let result = apply_mark(&mut buf, mark::SAC, true);
        assert!(result.applied);

        // Find which position has the mark
        let marked_pos = buf
            .iter()
            .enumerate()
            .find(|(_, c)| c.mark > 0)
            .map(|(i, _)| i);

        assert_eq!(
            marked_pos,
            Some(1),
            "Mark should be on ô (diacritic priority)"
        );
    }

    #[test]
    fn test_mark_repositioning_when_adding_circumflex() {
        // Updated: With safety check (Issue #211), double-vowel circumflex BLOCKED when mark exists
        // Scenario: ie + mark → ié (mark on e)
        //        ié + e → blocked (NOT iê - circumflex blocked)
        let mut buf = setup_buffer("ie");

        // Step 1: Apply mark - should go on 'e' (position 1)
        let result = apply_mark(&mut buf, mark::SAC, true);
        assert!(result.applied, "Mark should be applied");
        assert_eq!(buf.get(1).unwrap().mark, mark::SAC, "Mark on 'e'");

        // Step 2: Try circumflex - BLOCKED by safety check (mark exists)
        let result = apply_tone(&mut buf, keys::E, tone::CIRCUMFLEX, 0);
        assert!(!result.applied, "Circumflex blocked");
        assert_eq!(buf.get(1).unwrap().tone, tone::NONE, "No circumflex");
        assert_eq!(buf.get(1).unwrap().mark, mark::SAC, "Mark remains");
    }

    #[test]
    fn test_mark_moves_to_diacritic_in_compound() {
        // Test: uoi → mark on second vowel (o)
        //       uoi + w → ươi (u→ư, o→ơ) → mark should move to ơ (middle diacritic)
        let mut buf = setup_buffer("uoi");

        // Apply mark first - should go on 'o' (position 1, second vowel)
        let result = apply_mark(&mut buf, mark::SAC, true);
        assert!(result.applied);
        let marked_before = buf
            .iter()
            .enumerate()
            .find(|(_, c)| c.mark > 0)
            .map(|(i, _)| i);
        assert_eq!(marked_before, Some(1), "Mark should be on 'o' initially");

        // Add horn (w) - both u and o should get horn
        let result = apply_tone(&mut buf, keys::W, tone::HORN, 0);
        assert!(result.applied);

        // Verify both u and o have horn
        assert_eq!(buf.get(0).unwrap().tone, tone::HORN, "'u' should have horn");
        assert_eq!(buf.get(1).unwrap().tone, tone::HORN, "'o' should have horn");

        // Verify mark is still on position 1 (now 'ơ')
        // Since both ư and ơ have diacritics, and ơ is in middle, it keeps the mark
        assert_eq!(
            buf.get(1).unwrap().mark,
            mark::SAC,
            "Mark should stay on 'ơ'"
        );
    }
}
