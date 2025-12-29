//! Vowel Compound Utilities
//!
//! This module provides utilities for handling Vietnamese vowel compounds,
//! particularly the uo/ươ compound which requires special handling.
//!
//! In Vietnamese phonology:
//! - "ưo" (u with horn + plain o) is NEVER valid
//! - It should always be "ươ" (both with horn)
//! - The uo/ou compound can be transformed to ươ or uô depending on context

use crate::data::{chars::tone, keys};
use crate::engine::buffer::Buffer;

/// Find positions of U+O or O+U compound (adjacent vowels)
/// Returns Some((first_pos, second_pos)) if found, None otherwise
pub fn find_uo_compound_positions(buf: &Buffer) -> Option<(usize, usize)> {
    for i in 0..buf.len().saturating_sub(1) {
        if let (Some(c1), Some(c2)) = (buf.get(i), buf.get(i + 1)) {
            let is_uo = c1.key == keys::U && c2.key == keys::O;
            let is_ou = c1.key == keys::O && c2.key == keys::U;
            if is_uo || is_ou {
                return Some((i, i + 1));
            }
        }
    }
    None
}

/// Check for uo compound in buffer (any tone state)
#[inline]
pub fn has_uo_compound(buf: &Buffer) -> bool {
    find_uo_compound_positions(buf).is_some()
}

/// Check for complete ươ compound (both u and o have horn)
pub fn has_complete_uo_compound(buf: &Buffer) -> bool {
    if let Some((pos1, pos2)) = find_uo_compound_positions(buf) {
        if let (Some(c1), Some(c2)) = (buf.get(pos1), buf.get(pos2)) {
            // Check ư + ơ pattern (both with horn)
            let is_u_horn = c1.key == keys::U && c1.tone == tone::HORN;
            let is_o_horn = c2.key == keys::O && c2.tone == tone::HORN;
            return is_u_horn && is_o_horn;
        }
    }
    false
}

/// Normalize ưo → ươ compound
///
/// In Vietnamese, "ưo" (u with horn + plain o) is NEVER valid.
/// It should always be "ươ" (both with horn).
/// This function finds and fixes this pattern anywhere in the buffer.
///
/// Returns Some(position) of the 'o' that was modified, None if no change.
pub fn normalize_uo_compound(buf: &mut Buffer) -> Option<usize> {
    // Look for pattern: U with horn + O without horn (anywhere in buffer)
    for i in 0..buf.len().saturating_sub(1) {
        let c1 = buf.get(i)?;
        let c2 = buf.get(i + 1)?;

        // Check: U with horn + O plain → always normalize to ươ
        let is_u_with_horn = c1.key == keys::U && c1.tone == tone::HORN;
        let is_o_plain = c2.key == keys::O && c2.tone == tone::NONE;

        if is_u_with_horn && is_o_plain {
            // Apply horn to O to form the ươ compound
            if let Some(c) = buf.get_mut(i + 1) {
                c.tone = tone::HORN;
                return Some(i + 1);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::buffer::Char;

    fn make_buffer(chars: &[(u16, u8)]) -> Buffer {
        let mut buf = Buffer::new();
        for &(key, tone_val) in chars {
            let mut c = Char::new(key, false);
            c.tone = tone_val;
            buf.push(c);
        }
        buf
    }

    #[test]
    fn test_find_uo_compound() {
        // Test u+o pattern
        let buf = make_buffer(&[(keys::U, 0), (keys::O, 0)]);
        assert_eq!(find_uo_compound_positions(&buf), Some((0, 1)));

        // Test o+u pattern
        let buf = make_buffer(&[(keys::O, 0), (keys::U, 0)]);
        assert_eq!(find_uo_compound_positions(&buf), Some((0, 1)));

        // Test no compound
        let buf = make_buffer(&[(keys::A, 0), (keys::I, 0)]);
        assert_eq!(find_uo_compound_positions(&buf), None);

        // Test compound with consonants before
        let buf = make_buffer(&[(keys::T, 0), (keys::U, 0), (keys::O, 0)]);
        assert_eq!(find_uo_compound_positions(&buf), Some((1, 2)));
    }

    #[test]
    fn test_has_uo_compound() {
        let buf = make_buffer(&[(keys::U, 0), (keys::O, 0)]);
        assert!(has_uo_compound(&buf));

        let buf = make_buffer(&[(keys::A, 0), (keys::I, 0)]);
        assert!(!has_uo_compound(&buf));
    }

    #[test]
    fn test_has_complete_uo_compound() {
        // Complete ươ compound (both with horn)
        let buf = make_buffer(&[(keys::U, tone::HORN), (keys::O, tone::HORN)]);
        assert!(has_complete_uo_compound(&buf));

        // Incomplete: only u has horn
        let buf = make_buffer(&[(keys::U, tone::HORN), (keys::O, 0)]);
        assert!(!has_complete_uo_compound(&buf));

        // Incomplete: only o has horn
        let buf = make_buffer(&[(keys::U, 0), (keys::O, tone::HORN)]);
        assert!(!has_complete_uo_compound(&buf));

        // No horn on either
        let buf = make_buffer(&[(keys::U, 0), (keys::O, 0)]);
        assert!(!has_complete_uo_compound(&buf));
    }

    #[test]
    fn test_normalize_uo_compound() {
        // ưo → ươ
        let mut buf = make_buffer(&[(keys::U, tone::HORN), (keys::O, 0)]);
        let result = normalize_uo_compound(&mut buf);
        assert_eq!(result, Some(1));
        assert_eq!(buf.get(1).unwrap().tone, tone::HORN);

        // Already normalized (ươ) - no change
        let mut buf = make_buffer(&[(keys::U, tone::HORN), (keys::O, tone::HORN)]);
        let result = normalize_uo_compound(&mut buf);
        assert_eq!(result, None);

        // Plain uo - no change (u doesn't have horn)
        let mut buf = make_buffer(&[(keys::U, 0), (keys::O, 0)]);
        let result = normalize_uo_compound(&mut buf);
        assert_eq!(result, None);
    }

    #[test]
    fn test_normalize_with_consonants() {
        // "dưo" → "dươ"
        let mut buf = make_buffer(&[(keys::D, 0), (keys::U, tone::HORN), (keys::O, 0)]);
        let result = normalize_uo_compound(&mut buf);
        assert_eq!(result, Some(2));
        assert_eq!(buf.get(2).unwrap().tone, tone::HORN);
    }
}