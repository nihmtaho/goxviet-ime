//! Restore Utilities
//!
//! This module provides utilities for restoring buffer content to raw ASCII,
//! used for English word auto-restore and ESC key restoration functionality.
//!
//! ## Use Cases
//!
//! 1. **Auto-restore English words**: When space is pressed after an English word
//!    that was accidentally transformed, restore to raw ASCII + add space.
//!    Example: "telex" → "tễl" (transform) + space → "telex " (restored)
//!
//! 2. **ESC key restore**: When ESC is pressed, restore the entire buffer to
//!    the original keystrokes (undo all Vietnamese transforms).
//!    Example: "tẽt" (from typing "text" in Telex) → "text"

use crate::engine::buffer::Buffer;
use crate::engine::raw_input_buffer::RawInputBuffer;
use crate::engine::types::Result;
use crate::utils;

/// Build raw ASCII output from raw input history
///
/// Converts the raw keystroke history back to ASCII characters.
/// Used by both auto_restore_english and restore_to_raw.
///
/// # Arguments
/// * `raw_input` - Raw keystroke history buffer
///
/// # Returns
/// Vector of ASCII characters, empty if no valid characters
pub fn build_raw_output(raw_input: &RawInputBuffer) -> Vec<char> {
    raw_input
        .iter()
        .filter_map(|(key, caps)| utils::key_to_char(key, caps))
        .collect()
}

/// Build raw ASCII output from a specific position in raw input history
///
/// Used for incremental restore - only rebuilds from the first transform position.
///
/// # Arguments
/// * `raw_input` - Raw keystroke history buffer
/// * `from_pos` - Starting position (0-indexed)
///
/// # Returns
/// Vector of ASCII characters from position to end
pub fn build_raw_output_from(raw_input: &RawInputBuffer, from_pos: usize) -> Vec<char> {
    raw_input
        .iter()
        .skip(from_pos)
        .filter_map(|(key, caps)| utils::key_to_char(key, caps))
        .collect()
}

/// Find the position of the first character with Vietnamese transforms
///
/// Used for incremental restore optimization - instead of rebuilding the entire
/// buffer, we only rebuild from the first transformed character.
///
/// # Arguments
/// * `buf` - Buffer to search
///
/// # Returns
/// Position of first transform, or buffer length if no transforms found
///
/// # Example
/// ```ignore
/// // Buffer: "usẽr" (transform at position 2)
/// let pos = find_first_transform_position(&buf);
/// assert_eq!(pos, 2); // 'ẽ' is at position 2
/// ```
pub fn find_first_transform_position(buf: &Buffer) -> usize {
    buf.iter()
        .position(|c| c.tone != 0 || c.mark != 0 || c.stroke)
        .unwrap_or(buf.len())
}

/// Restore buffer to raw ASCII for English words
///
/// Auto-restore English words when space is pressed AND transforms were applied.
/// Adds a trailing space for better UX.
///
/// # Arguments
/// * `buf` - Current buffer (for backspace count)
/// * `raw_input` - Raw keystroke history
///
/// # Returns
/// Result with backspace count and raw ASCII output + space
///
/// # Example
/// ```ignore
/// // "telex" → "tễl" (transform) + space → "telex " (with auto-space)
/// let result = auto_restore_english(&buf, &raw_input);
/// // result.backspace = 3 (length of "tễl")
/// // result.chars = ['t', 'e', 'l', 'e', 'x', ' ']
/// ```
pub fn auto_restore_english(buf: &Buffer, raw_input: &RawInputBuffer) -> Result {
    if raw_input.is_empty() || buf.is_empty() {
        return Result::none();
    }

    let mut raw_chars = build_raw_output(raw_input);

    if raw_chars.is_empty() {
        return Result::none();
    }

    // Auto-add space after English word restore
    // This provides better UX: user types "telex" + space, gets "telex " ready for next word
    raw_chars.push(' ');

    // Backspace count = current buffer length (displayed chars)
    let backspace = buf.len() as u8;

    Result::send(backspace, &raw_chars)
}

/// Restore buffer to raw ASCII for English words (instant - no space)
///
/// Used for immediate restoration during typing when English is detected.
/// OPTIMIZED: Uses incremental restore to only rebuild transformed portion.
///
/// # Performance
/// - Old: Delete entire buffer + retype all characters (O(n))
/// - New: Delete only from first transform + retype from there (O(k) where k = chars after transform)
///
/// # Example
/// ```ignore
/// // Typing "user": u → s → s → e → r
/// // At 'e': buffer = "usẽ" (transform at pos 2)
/// // Old: backspace 3, type "use" (6 operations)
/// // New: backspace 1, type "e" (2 operations) - 3x faster!
/// ```
pub fn instant_restore_english(buf: &Buffer, raw_input: &RawInputBuffer) -> Result {
    if raw_input.is_empty() || buf.is_empty() {
        return Result::none();
    }

    // OPTIMIZATION: Find first transform position for incremental restore
    let first_transform_pos = find_first_transform_position(buf);

    // If no transforms found, nothing to restore
    if first_transform_pos >= buf.len() {
        return Result::none();
    }

    // Build raw output only from the transform position onwards
    let raw_chars_from = build_raw_output_from(raw_input, first_transform_pos);

    if raw_chars_from.is_empty() {
        return Result::none();
    }

    // Count screen characters from transform position to end
    // This is how many backspaces we need
    let backspace = (buf.len() - first_transform_pos) as u8;

    Result::send(backspace, &raw_chars_from)
}

/// Restore buffer to raw ASCII (ESC key handler)
///
/// Replaces transformed output with original keystrokes.
/// Only restores if transforms were actually applied.
///
/// # Arguments
/// * `buf` - Current buffer (for backspace count and transform check)
/// * `raw_input` - Raw keystroke history
///
/// # Returns
/// Result with backspace count and raw ASCII output, or none if no transforms
///
/// # Example
/// ```ignore
/// // "tẽt" (from typing "text" in Telex) → "text"
/// let result = restore_to_raw(&buf, &raw_input);
/// // result.backspace = 3 (length of "tẽt")
/// // result.chars = ['t', 'e', 'x', 't']
/// ```
pub fn restore_to_raw(buf: &Buffer, raw_input: &RawInputBuffer) -> Result {
    if raw_input.is_empty() || buf.is_empty() {
        return Result::none();
    }

    // Check if any transforms were applied
    let has_transforms = buf.iter().any(|c| c.tone > 0 || c.mark > 0 || c.stroke);
    if !has_transforms {
        return Result::none();
    }

    let raw_chars = build_raw_output(raw_input);

    if raw_chars.is_empty() {
        return Result::none();
    }

    // Backspace count = current buffer length (displayed chars)
    let backspace = buf.len() as u8;

    Result::send(backspace, &raw_chars)
}

/// Check if buffer has any Vietnamese transforms (tone, mark, stroke)
///
/// Used to distinguish between Vietnamese and English words.
/// Example: "tét" has tone → Vietnamese, "test" no transforms → English
///
/// # Arguments
/// * `buf` - Buffer to check
///
/// # Returns
/// `true` if any character has tone, mark, or stroke
#[inline]
pub fn has_vietnamese_transforms(buf: &Buffer) -> bool {
    buf.iter().any(|c| c.tone != 0 || c.mark != 0 || c.stroke)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{chars::tone, keys};
    use crate::engine::buffer::Char;

    fn make_buffer(chars: &[(u16, u8, u8, bool)]) -> Buffer {
        let mut buf = Buffer::new();
        for &(key, tone_val, mark_val, stroke) in chars {
            let mut c = Char::new(key, false);
            c.tone = tone_val;
            c.mark = mark_val;
            c.stroke = stroke;
            buf.push(c);
        }
        buf
    }

    fn make_raw_input(keys_list: &[(u16, bool)]) -> RawInputBuffer {
        let mut raw = RawInputBuffer::new();
        for &(key, caps) in keys_list {
            raw.push(key, caps);
        }
        raw
    }

    #[test]
    fn test_build_raw_output() {
        let raw = make_raw_input(&[
            (keys::T, false),
            (keys::E, false),
            (keys::X, false),
            (keys::T, false),
        ]);
        let output = build_raw_output(&raw);
        assert_eq!(output, vec!['t', 'e', 'x', 't']);
    }

    #[test]
    fn test_build_raw_output_with_caps() {
        let raw = make_raw_input(&[(keys::T, true), (keys::E, false), (keys::S, false)]);
        let output = build_raw_output(&raw);
        assert_eq!(output, vec!['T', 'e', 's']);
    }

    #[test]
    fn test_build_raw_output_empty() {
        let raw = RawInputBuffer::new();
        let output = build_raw_output(&raw);
        assert!(output.is_empty());
    }

    #[test]
    fn test_auto_restore_english() {
        // Buffer: "tẽ" (t + e with ngã)
        let buf = make_buffer(&[(keys::T, 0, 0, false), (keys::E, 0, 4, false)]); // mark=4 is ngã
        let raw = make_raw_input(&[(keys::T, false), (keys::E, false), (keys::X, false)]);

        let result = auto_restore_english(&buf, &raw);

        assert_eq!(result.action, 1); // Action::Send
        assert_eq!(result.backspace, 2); // "tẽ" length
        assert_eq!(result.count, 4); // "tex" + space
    }

    #[test]
    fn test_auto_restore_english_empty_buffer() {
        let buf = Buffer::new();
        let raw = make_raw_input(&[(keys::T, false)]);

        let result = auto_restore_english(&buf, &raw);

        assert_eq!(result.action, 0); // Action::None
    }

    #[test]
    fn test_auto_restore_english_empty_raw() {
        let buf = make_buffer(&[(keys::T, 0, 0, false)]);
        let raw = RawInputBuffer::new();

        let result = auto_restore_english(&buf, &raw);

        assert_eq!(result.action, 0); // Action::None
    }

    #[test]
    fn test_restore_to_raw() {
        // Buffer: "đ" (d with stroke)
        let buf = make_buffer(&[(keys::D, 0, 0, true)]);
        let raw = make_raw_input(&[(keys::D, false), (keys::D, false)]);

        let result = restore_to_raw(&buf, &raw);

        assert_eq!(result.action, 1); // Action::Send
        assert_eq!(result.backspace, 1); // "đ" length
        assert_eq!(result.count, 2); // "dd"
    }

    #[test]
    fn test_restore_to_raw_no_transforms() {
        // Buffer: "ab" (no transforms)
        let buf = make_buffer(&[(keys::A, 0, 0, false), (keys::B, 0, 0, false)]);
        let raw = make_raw_input(&[(keys::A, false), (keys::B, false)]);

        let result = restore_to_raw(&buf, &raw);

        assert_eq!(result.action, 0); // Action::None (no transforms to undo)
    }

    #[test]
    fn test_restore_to_raw_empty() {
        let buf = Buffer::new();
        let raw = RawInputBuffer::new();

        let result = restore_to_raw(&buf, &raw);

        assert_eq!(result.action, 0); // Action::None
    }

    #[test]
    fn test_has_vietnamese_transforms_with_tone() {
        let buf = make_buffer(&[(keys::A, tone::HORN, 0, false)]);
        assert!(has_vietnamese_transforms(&buf));
    }

    #[test]
    fn test_has_vietnamese_transforms_with_mark() {
        let buf = make_buffer(&[(keys::A, 0, 1, false)]); // mark=1 is sắc
        assert!(has_vietnamese_transforms(&buf));
    }

    #[test]
    fn test_has_vietnamese_transforms_with_stroke() {
        let buf = make_buffer(&[(keys::D, 0, 0, true)]);
        assert!(has_vietnamese_transforms(&buf));
    }

    #[test]
    fn test_has_vietnamese_transforms_none() {
        let buf = make_buffer(&[(keys::A, 0, 0, false), (keys::B, 0, 0, false)]);
        assert!(!has_vietnamese_transforms(&buf));
    }

    #[test]
    fn test_has_vietnamese_transforms_empty() {
        let buf = Buffer::new();
        assert!(!has_vietnamese_transforms(&buf));
    }

    #[test]
    fn test_find_first_transform_position_with_tone() {
        // Buffer: "usẽr" - transform at position 2
        let buf = make_buffer(&[
            (keys::U, 0, 0, false),
            (keys::S, 0, 0, false),
            (keys::E, 0, 4, false), // mark=4 is ngã
            (keys::R, 0, 0, false),
        ]);
        assert_eq!(find_first_transform_position(&buf), 2);
    }

    #[test]
    fn test_find_first_transform_position_with_stroke() {
        // Buffer: "đa" - transform at position 0
        let buf = make_buffer(&[(keys::D, 0, 0, true), (keys::A, 0, 0, false)]);
        assert_eq!(find_first_transform_position(&buf), 0);
    }

    #[test]
    fn test_find_first_transform_position_no_transforms() {
        // Buffer: "test" - no transforms
        let buf = make_buffer(&[
            (keys::T, 0, 0, false),
            (keys::E, 0, 0, false),
            (keys::S, 0, 0, false),
            (keys::T, 0, 0, false),
        ]);
        assert_eq!(find_first_transform_position(&buf), 4); // Returns buf.len()
    }

    #[test]
    fn test_find_first_transform_position_empty() {
        let buf = Buffer::new();
        assert_eq!(find_first_transform_position(&buf), 0);
    }

    #[test]
    fn test_build_raw_output_from() {
        let raw = make_raw_input(&[
            (keys::U, false),
            (keys::S, false),
            (keys::E, false),
            (keys::R, false),
        ]);
        let output = build_raw_output_from(&raw, 2);
        assert_eq!(output, vec!['e', 'r']); // From position 2 onwards
    }

    #[test]
    fn test_build_raw_output_from_start() {
        let raw = make_raw_input(&[(keys::T, false), (keys::E, false)]);
        let output = build_raw_output_from(&raw, 0);
        assert_eq!(output, vec!['t', 'e']); // All characters
    }

    #[test]
    fn test_build_raw_output_from_beyond_end() {
        let raw = make_raw_input(&[(keys::A, false)]);
        let output = build_raw_output_from(&raw, 10);
        assert!(output.is_empty()); // Beyond end = empty
    }

    #[test]
    fn test_instant_restore_english_incremental() {
        // Buffer: "usẽr" (transform at position 2)
        let buf = make_buffer(&[
            (keys::U, 0, 0, false),
            (keys::S, 0, 0, false),
            (keys::E, 0, 4, false), // mark=4 is ngã
            (keys::R, 0, 0, false),
        ]);
        // Raw input: "user"
        let raw = make_raw_input(&[
            (keys::U, false),
            (keys::S, false),
            (keys::E, false),
            (keys::R, false),
        ]);

        let result = instant_restore_english(&buf, &raw);

        assert_eq!(result.action, 1); // Action::Send
        // OPTIMIZATION: Only backspace from position 2 (2 chars: "ẽr")
        assert_eq!(result.backspace, 2);
        // OPTIMIZATION: Only send "er" (2 chars)
        assert_eq!(result.count, 2);
    }

    #[test]
    fn test_instant_restore_english_no_transforms() {
        // Buffer: "test" (no transforms)
        let buf = make_buffer(&[
            (keys::T, 0, 0, false),
            (keys::E, 0, 0, false),
            (keys::S, 0, 0, false),
            (keys::T, 0, 0, false),
        ]);
        let raw = make_raw_input(&[
            (keys::T, false),
            (keys::E, false),
            (keys::S, false),
            (keys::T, false),
        ]);

        let result = instant_restore_english(&buf, &raw);

        // No transforms = no restore needed
        assert_eq!(result.action, 0); // Action::None
    }

    #[test]
    fn test_instant_restore_english_transform_at_start() {
        // Buffer: "đa" (transform at position 0)
        let buf = make_buffer(&[(keys::D, 0, 0, true), (keys::A, 0, 0, false)]);
        let raw = make_raw_input(&[(keys::D, false), (keys::D, false), (keys::A, false)]);

        let result = instant_restore_english(&buf, &raw);

        assert_eq!(result.action, 1); // Action::Send
        // Backspace entire buffer (transform at start)
        assert_eq!(result.backspace, 2);
        // Send all raw chars
        assert_eq!(result.count, 3); // "dda"
    }
}
