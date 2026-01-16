//! Buffer Rebuild Utilities
//!
//! This module provides utilities for rebuilding the displayed text from
//! the internal buffer state. When Vietnamese transformations are applied
//! or reverted, the displayed text needs to be regenerated from a specific
//! position in the buffer.
//!
//! # Key Functions
//!
//! - `rebuild_from`: Generate replacement text from a buffer position
//! - `rebuild_from_with_backspace`: Rebuild with explicit backspace count
//! - `count_screen_chars`: Count displayed characters for backspace calculation
//! - `find_syllable_boundary`: Find the start of the last syllable for optimization
//!
//! # Performance Considerations
//!
//! These functions are called frequently during Vietnamese typing, so they
//! are optimized for:
//! - Minimal allocations (reuse buffers where possible)
//! - O(1) operations for simple cases
//! - O(syllable) for complex transformations (not O(buffer))
//!
//! # Example
//!
//! ```ignore
//! // After applying a transformation at position 2
//! let result = rebuild_from(&buffer, 2);
//! // result.backspace = number of chars to delete from position 2 to end
//! // result.chars = new characters to insert
//! ```

use super::buffer::{Buffer, Char};
use crate::data::{chars, keys};
use crate::engine::types::Result;

// ============================================================
// Character Rendering
// ============================================================

/// Render a single buffer character to its display form
///
/// Converts the internal representation (key + modifiers) to
/// the actual Unicode character that should be displayed.
///
/// # Arguments
/// * `c` - The buffer character with key, caps, tone, mark, and stroke
///
/// # Returns
/// The Unicode character to display, or `None` if invalid
#[inline]
pub fn render_char(c: &Char) -> Option<char> {
    // Handle đ/Đ (stroked D)
    if c.key == keys::D && c.stroke {
        return Some(chars::get_d(c.caps));
    }

    // Try to get full Vietnamese character with diacritics
    if let Some(ch) = chars::to_char(c.key, c.caps, c.tone, c.mark) {
        return Some(ch);
    }

    // Fallback to basic character conversion
    crate::utils::key_to_char(c.key, c.caps)
}

/// Render buffer contents to a character vector
///
/// Converts the buffer from the given start position to end into
/// displayable characters.
///
/// # Arguments
/// * `buf` - The buffer to render
/// * `start` - Starting position (inclusive)
/// * `end` - Ending position (exclusive)
///
/// # Returns
/// Vector of rendered characters
#[inline]
pub fn render_range(buf: &Buffer, start: usize, end: usize) -> Vec<char> {
    let mut result = Vec::with_capacity(end - start);
    for i in start..end.min(buf.len()) {
        if let Some(c) = buf.get(i) {
            if let Some(ch) = render_char(c) {
                result.push(ch);
            }
        }
    }
    result
}

/// Render entire buffer to a character vector
#[inline]
pub fn render_all(buf: &Buffer) -> Vec<char> {
    render_range(buf, 0, buf.len())
}

// ============================================================
// Screen Character Counting
// ============================================================

/// Count the number of screen characters in a buffer range
///
/// This counts the actual displayed characters, which may differ from
/// buffer positions due to:
/// - Combined characters (base + diacritic rendered as one)
/// - Invalid characters that are skipped
///
/// # Arguments
/// * `buf` - The buffer to count from
/// * `start` - Starting position (inclusive)
/// * `end` - Ending position (exclusive)
///
/// # Returns
/// Number of displayable characters in the range
///
/// # Performance
/// O(n) where n = end - start
#[inline]
pub fn count_screen_chars(buf: &Buffer, start: usize, end: usize) -> usize {
    let mut count = 0;
    for i in start..end.min(buf.len()) {
        if let Some(c) = buf.get(i) {
            // Each buffer position maps to one screen character
            // (Vietnamese diacritics are combined with base characters)
            if render_char(c).is_some() {
                count += 1;
            }
        }
    }
    count
}

// ============================================================
// Syllable Boundary Detection
// ============================================================

/// Find the start position of the last syllable in the buffer
///
/// Vietnamese syllables follow the pattern: (C₁)(G)V(C₂)
/// - C₁: Initial consonant (phụ âm đầu)
/// - G: Glide/Medial (âm đệm)
/// - V: Vowel nucleus (nguyên âm chính)
/// - C₂: Final consonant (âm cuối)
///
/// This function scans backwards from the end to find where the
/// last syllable begins. This is used for performance optimization:
/// we only rebuild from the syllable boundary instead of the entire buffer.
///
/// # Arguments
/// * `buf` - The buffer to analyze
///
/// # Returns
/// The starting position of the last syllable (0 if buffer is one syllable)
///
/// # Performance
/// O(n) worst case, but typically O(syllable_size) ≈ O(8)
///
/// # Examples
/// - "việt" → 0 (entire buffer is one syllable)
/// - "việtnam" → 4 (last syllable "nam" starts at position 4)
/// - "a" → 0
#[inline]
pub fn find_syllable_boundary(buf: &Buffer) -> usize {
    let len = buf.len();
    if len <= 1 {
        return 0;
    }

    // For Vietnamese IME, syllable boundary detection is primarily used for
    // word segmentation in multi-word inputs. In practice, the IME clears
    // the buffer on word boundaries (space, punctuation), so most buffers
    // contain a single syllable.
    //
    // This function looks for word breaks (spaces, punctuation) that would
    // indicate where a new "word" starts. For single-word buffers, we return 0.
    //
    // Note: Vietnamese "syllables" within a word (like "việtnam") are not
    // segmented here because the IME processes them as continuous input.

    // Scan backwards looking for word boundary indicators
    for i in (0..len).rev() {
        if let Some(c) = buf.get(i) {
            // Space is a word boundary
            if c.key == keys::SPACE {
                return i + 1;
            }

            // Non-letter characters (punctuation, numbers) are word boundaries
            if !keys::is_letter(c.key) {
                return i + 1;
            }
        }
    }

    // No word boundary found - entire buffer is one word/syllable
    0
}

// ============================================================
// Buffer Rebuild Functions
// ============================================================

/// Rebuild displayed text from a buffer position
///
/// Creates a Result that tells the platform layer how to update
/// the displayed text after a transformation at the given position.
///
/// # Arguments
/// * `buf` - The buffer with updated content
/// * `from_pos` - Position from which to rebuild
///
/// # Returns
/// A `Result` with:
/// - `backspace`: Number of characters to delete (from_pos to old end)
/// - `chars`: New characters to insert (from_pos to new end)
///
/// # Example
/// ```ignore
/// // Buffer: "việt" (4 chars), transformation at position 2
/// // Old screen: "việt", new buffer: "việt"
/// let result = rebuild_from(&buf, 2);
/// // result.backspace = 2 (delete "ệt")
/// // result.chars = ['ệ', 't'] (insert new "ệt")
/// ```
#[inline]
pub fn rebuild_from(buf: &Buffer, from_pos: usize) -> Result {
    // Count how many screen chars from position to end
    let screen_chars = count_screen_chars(buf, from_pos, buf.len());

    // Render the characters from position to end
    let new_chars = render_range(buf, from_pos, buf.len());

    // Backspace count = number of old screen chars at and after position
    // Since we're rebuilding from the same buffer, this equals the new count
    Result::send(screen_chars as u8, &new_chars)
}

/// Rebuild displayed text with explicit backspace count
///
/// This variant is used when we know the old screen length and need
/// to delete a specific number of characters before inserting new ones.
///
/// # Arguments
/// * `buf` - The buffer with updated content
/// * `from_pos` - Position from which to rebuild
/// * `old_screen_length` - Number of screen chars that WERE displayed
///
/// # Returns
/// A `Result` with explicit backspace count
///
/// # Example
/// ```ignore
/// // User deleted a character: old screen had 4 chars, now buffer has 3
/// let old_len = 4;
/// let result = rebuild_from_with_backspace(&buf, 0, old_len);
/// // result.backspace = 4 (delete all old chars)
/// // result.chars = new 3-char content
/// ```
#[inline]
pub fn rebuild_from_with_backspace(
    buf: &Buffer,
    from_pos: usize,
    old_screen_length: usize,
) -> Result {
    let new_chars = render_range(buf, from_pos, buf.len());
    Result::send(old_screen_length as u8, &new_chars)
}

/// Rebuild entire buffer and return the result
///
/// Convenience function for rebuilding the complete buffer content.
///
/// # Arguments
/// * `buf` - The buffer to rebuild
/// * `old_screen_length` - Number of screen chars to delete
#[inline]
pub fn rebuild_all(buf: &Buffer, old_screen_length: usize) -> Result {
    rebuild_from_with_backspace(buf, 0, old_screen_length)
}

// ============================================================
// Vowel Compound Detection
// ============================================================

/// Check if a position is part of a vowel compound (diphthong/triphthong)
///
/// Vietnamese has complex vowel combinations that must be treated as units:
/// - Diphthongs: oa, oe, oo, uô, ươ, etc.
/// - Triphthongs: oai, uôi, ươi, etc.
///
/// When backspacing, if the cursor is in a vowel compound, we need to
/// rebuild the entire compound, not just delete one character.
///
/// # Arguments
/// * `buf` - The buffer to check
/// * `pos` - Position to check
///
/// # Returns
/// `true` if the position is part of a vowel compound
#[inline]
pub fn is_part_of_vowel_compound(buf: &Buffer, pos: usize) -> bool {
    if pos >= buf.len() {
        return false;
    }

    let c = match buf.get(pos) {
        Some(c) => c,
        None => return false,
    };

    // Only vowels can be part of compounds
    if !keys::is_vowel(c.key) {
        return false;
    }

    // Check if adjacent to another vowel
    // Look at previous character
    if pos > 0 {
        if let Some(prev) = buf.get(pos - 1) {
            if keys::is_vowel(prev.key) {
                return true;
            }
        }
    }

    // Look at next character
    if pos + 1 < buf.len() {
        if let Some(next) = buf.get(pos + 1) {
            if keys::is_vowel(next.key) {
                return true;
            }
        }
    }

    // Check if this vowel has a tone/diacritic (might be part of compound)
    if c.tone != 0 || c.mark != 0 {
        // Look for adjacent consonant that might complete a syllable
        if pos > 0 {
            if let Some(prev) = buf.get(pos - 1) {
                if !keys::is_vowel(prev.key) && keys::is_letter(prev.key) {
                    return true; // Like "được" - ươ is a compound
                }
            }
        }
    }

    false
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::chars::{mark, tone};

    /// Helper: Create buffer from string
    fn make_buffer(s: &str) -> Buffer {
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
                _ => continue,
            };
            buf.push(Char::new(key, ch.is_uppercase()));
        }
        buf
    }

    #[test]
    fn test_render_char_basic() {
        let c = Char::new(keys::A, false);
        assert_eq!(render_char(&c), Some('a'));

        let c = Char::new(keys::A, true);
        assert_eq!(render_char(&c), Some('A'));
    }

    #[test]
    fn test_render_char_with_tone() {
        let mut c = Char::new(keys::A, false);
        c.tone = tone::CIRCUMFLEX;
        assert_eq!(render_char(&c), Some('â'));

        c.tone = tone::HORN;
        assert_eq!(render_char(&c), Some('ă'));
    }

    #[test]
    fn test_render_char_with_mark() {
        let mut c = Char::new(keys::A, false);
        c.mark = mark::SAC;
        assert_eq!(render_char(&c), Some('á'));

        c.mark = mark::HUYEN;
        assert_eq!(render_char(&c), Some('à'));
    }

    #[test]
    fn test_render_char_stroked_d() {
        let mut c = Char::new(keys::D, false);
        c.stroke = true;
        assert_eq!(render_char(&c), Some('đ'));

        c.caps = true;
        assert_eq!(render_char(&c), Some('Đ'));
    }

    #[test]
    fn test_render_range() {
        let buf = make_buffer("viet");
        let chars = render_range(&buf, 0, buf.len());
        assert_eq!(chars, vec!['v', 'i', 'e', 't']);
    }

    #[test]
    fn test_render_range_partial() {
        let buf = make_buffer("vietnam");
        let chars = render_range(&buf, 2, 5);
        assert_eq!(chars, vec!['e', 't', 'n']);
    }

    #[test]
    fn test_count_screen_chars() {
        let buf = make_buffer("viet");
        assert_eq!(count_screen_chars(&buf, 0, buf.len()), 4);
        assert_eq!(count_screen_chars(&buf, 2, buf.len()), 2);
        assert_eq!(count_screen_chars(&buf, 0, 2), 2);
    }

    #[test]
    fn test_find_syllable_boundary_single() {
        let buf = make_buffer("viet");
        assert_eq!(find_syllable_boundary(&buf), 0);
    }

    #[test]
    fn test_find_syllable_boundary_empty() {
        let buf = Buffer::new();
        assert_eq!(find_syllable_boundary(&buf), 0);
    }

    #[test]
    fn test_find_syllable_boundary_single_char() {
        let buf = make_buffer("a");
        assert_eq!(find_syllable_boundary(&buf), 0);
    }

    #[test]
    fn test_rebuild_from() {
        let buf = make_buffer("viet");
        let result = rebuild_from(&buf, 2);
        assert!(result.is_send());
        assert_eq!(result.count, 2);
        unsafe {
            assert_eq!(*result.chars.offset(0), 'e' as u32);
            assert_eq!(*result.chars.offset(1), 't' as u32);
        }
    }

    #[test]
    fn test_rebuild_from_with_backspace() {
        let buf = make_buffer("vie");
        let result = rebuild_from_with_backspace(&buf, 0, 4);
        assert!(result.is_send());
        assert_eq!(result.backspace, 4);
        assert_eq!(result.count, 3);
    }

    #[test]
    fn test_rebuild_all() {
        let buf = make_buffer("test");
        let result = rebuild_all(&buf, 5);
        assert!(result.is_send());
        assert_eq!(result.backspace, 5);
        assert_eq!(result.count, 4);
    }

    #[test]
    fn test_is_part_of_vowel_compound() {
        // "oa" - both vowels are part of compound
        let buf = make_buffer("hoa");
        assert!(is_part_of_vowel_compound(&buf, 1)); // o
        assert!(is_part_of_vowel_compound(&buf, 2)); // a

        // Single vowel
        let buf = make_buffer("ha");
        assert!(!is_part_of_vowel_compound(&buf, 1)); // a alone

        // Consonant is not part of compound
        let buf = make_buffer("ban");
        assert!(!is_part_of_vowel_compound(&buf, 0)); // b
        assert!(!is_part_of_vowel_compound(&buf, 2)); // n
    }
}
