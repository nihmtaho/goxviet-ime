//! Restore Utilities
//!
//! This module provides utilities for restoring buffer content to raw ASCII,
//! used for English word auto-restore and ESC key restoration functionality.
//!
//! ## Performance Optimizations (Phase 2)
//! - Pre-allocated String buffers with capacity
//! - Reduced redundant transform checks
//! - Optimized iteration patterns
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

use crate::infrastructure::engine::buffer::Buffer;
use crate::infrastructure::engine::raw_input_buffer::RawInputBuffer;
use crate::infrastructure::engine::core_types::Result;
use crate::utils;

/// Build raw ASCII output from raw input history (OPTIMIZED)
///
/// Converts the raw keystroke history back to ASCII characters.
/// Used by both auto_restore_english and restore_to_raw.
///
/// # Performance
/// Pre-allocates Vec with exact capacity to avoid reallocation.
///
/// # Arguments
/// * `raw_input` - Raw keystroke history buffer
///
/// # Returns
/// Vector of ASCII characters, empty if no valid characters
#[inline]
pub fn build_raw_output(raw_input: &RawInputBuffer) -> Vec<char> {
    let len = raw_input.len();
    if len == 0 {
        return Vec::new();
    }

    // OPTIMIZATION: Pre-allocate with exact capacity
    let mut out = Vec::with_capacity(len);
    for (key, caps) in raw_input.iter() {
        if let Some(ch) = utils::key_to_char(key, caps) {
            out.push(ch);
        }
    }

    out
}

/// Build raw ASCII output from a specific position in raw input history (OPTIMIZED)
///
/// Used for incremental restore - only rebuilds from the first transform position.
///
/// # Performance
/// Pre-allocates Vec with exact capacity for the subset.
///
/// # Arguments
/// * `raw_input` - Raw keystroke history buffer
/// * `from_pos` - Starting position (0-indexed)
///
/// # Returns
/// Vector of ASCII characters from position to end
#[inline]
pub fn build_raw_output_from(raw_input: &RawInputBuffer, from_pos: usize) -> Vec<char> {
    let len = raw_input.len();
    if from_pos >= len {
        return Vec::new();
    }

    // OPTIMIZATION: Pre-allocate exact size needed
    let mut out = Vec::with_capacity(len - from_pos);

    // OPTIMIZATION: Use skip() instead of enumerate + continue
    for (key, caps) in raw_input.iter().skip(from_pos) {
        if let Some(ch) = utils::key_to_char(key, caps) {
            out.push(ch);
        }
    }

    out
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
#[inline]
pub fn find_first_transform_position(buf: &Buffer) -> usize {
    buf.iter()
        .position(|c| c.tone != 0 || c.mark != 0 || c.stroke)
        .unwrap_or(buf.len())
}

/// Check if buffer has any Vietnamese transforms (OPTIMIZED + INLINED)
///
/// Used to distinguish between Vietnamese and English words.
/// Example: "tét" has tone → Vietnamese, "test" no transforms → English
///
/// # Arguments
/// * `buf` - Buffer to check
///
/// # Returns
/// `true` if any character has tone, mark, or stroke
#[inline(always)]
pub fn has_vietnamese_transforms(buf: &Buffer) -> bool {
    // OPTIMIZATION: Early exit on first transform found
    buf.iter().any(|c| c.tone != 0 || c.mark != 0 || c.stroke)
}

/// Restore buffer to raw ASCII for English words (OPTIMIZED)
///
/// Auto-restore English words when space is pressed AND transforms were applied.
/// Adds a trailing space for better UX.
///
/// # Performance
/// - Pre-checks for transforms to avoid unnecessary work
/// - Pre-allocates Vec with capacity for output + space
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
    // Fast path: empty checks
    if raw_input.is_empty() || buf.is_empty() {
        return Result::none();
    }

    // OPTIMIZATION: Pre-allocate with capacity for raw + space
    let mut raw_chars = Vec::with_capacity(raw_input.len() + 1);

    for (key, caps) in raw_input.iter() {
        if let Some(ch) = utils::key_to_char(key, caps) {
            raw_chars.push(ch);
        }
    }

    if raw_chars.is_empty() {
        return Result::none();
    }

    // Auto-add space after English word restore
    // This provides better UX: user types "telex" + space, gets "telex " ready for next word
    raw_chars.push(' ');

    // Backspace count = current buffer length (displayed chars)
    // SAFETY: Clamp to u8::MAX to prevent overflow
    let backspace = buf.len().min(u8::MAX as usize) as u8;

    Result::send(backspace, &raw_chars)
}

/// Restore buffer to raw ASCII for English words (instant - no space) (OPTIMIZED)
///
/// Used for immediate restoration during typing when English is detected.
///
/// # Algorithm
/// When English is detected mid-typing, we need to restore the ENTIRE sequence to raw,
/// not just from the first transform. This is because earlier untransformed characters
/// are still part of the English word.
///
/// Example: typing "r-e-s-t-o" (English "resto")
/// - Key 0-1: "r-e" (no transforms)
/// - Key 2: "s" → triggers tone → buffer becomes "ré" (mark on 'e')
/// - Key 3: "t" → buffer becomes "rét"
/// - Key 4: "o" → English detection triggers
///   - Must restore ENTIRE buffer "rét" → "resto"
///   - Must backspace 3 chars ("rét"), type 5 chars ("resto")
///
/// # Performance Optimizations
/// - Early exit if no transforms
/// - Pre-allocated Vec with exact capacity
/// - Single-pass transform check
pub fn instant_restore_english(buf: &Buffer, raw_input: &RawInputBuffer) -> Result {
    // Fast path: empty checks
    if raw_input.is_empty() || buf.is_empty() {
        return Result::none();
    }

    // OPTIMIZATION: Check for transforms first (early exit)
    if !has_vietnamese_transforms(buf) {
        return Result::none();
    }

    // OPTIMIZATION: Pre-allocate with exact capacity
    let mut raw_chars = Vec::with_capacity(raw_input.len());

    for (key, caps) in raw_input.iter() {
        if let Some(ch) = utils::key_to_char(key, caps) {
            raw_chars.push(ch);
        }
    }

    if raw_chars.is_empty() {
        return Result::none();
    }

    // Backspace count = current buffer length (all displayed chars)
    // CRITICAL FIX: Clamp backspace to raw_input.len() to prevent deleting preceding space
    // This handles edge cases where buf.len() might be erroneously larger than the word
    // (e.g. due to lingering state or compound transforms)
    // English words generally shouldn't expand (like shortcuts), so this is safe.
    // SAFETY: Also clamp to u8::MAX to prevent overflow
    let backspace = buf.len().min(raw_input.len()).min(u8::MAX as usize) as u8;

    Result::send(backspace, &raw_chars)
}

/// Restore buffer to raw ASCII (ESC key handler) (OPTIMIZED)
///
/// Replaces transformed output with original keystrokes.
/// Only restores if transforms were actually applied.
///
/// # Performance
/// - Early exit if no transforms
/// - Pre-allocated Vec with exact capacity
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
    // Fast path: empty checks
    if raw_input.is_empty() || buf.is_empty() {
        return Result::none();
    }

    // OPTIMIZATION: Early exit if no transforms
    if !has_vietnamese_transforms(buf) {
        return Result::none();
    }

    // OPTIMIZATION: Pre-allocate with exact capacity
    let mut raw_chars = Vec::with_capacity(raw_input.len());

    for (key, caps) in raw_input.iter() {
        if let Some(ch) = utils::key_to_char(key, caps) {
            raw_chars.push(ch);
        }
    }

    if raw_chars.is_empty() {
        return Result::none();
    }

    // Backspace count = current buffer length (displayed chars)
    // SAFETY: Clamp to raw_input.len() AND u8::MAX to prevent over-deletion
    let backspace = buf.len().min(raw_input.len()).min(u8::MAX as usize) as u8;

    Result::send(backspace, &raw_chars)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{chars::tone, keys};
    use crate::infrastructure::engine::buffer::Char;

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
                                      // FIX: Must restore ENTIRE buffer to raw (not just from first transform)
                                      // This ensures we don't lose initial characters like in "reto" → "esto" bug
        assert_eq!(result.backspace, 4); // Backspace all 4 chars: "usẽr"
                                         // FIX: Send all 4 raw chars "user"
        assert_eq!(result.count, 4);
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
