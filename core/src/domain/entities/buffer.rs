//! Buffer Entity - Input Buffer Management
//!
//! Manages the current input buffer for text processing.

use crate::domain::value_objects::char_sequence::CharSequence;
use std::fmt;

/// Maximum buffer size (characters)
const MAX_BUFFER_SIZE: usize = 255;

/// Input buffer entity
///
/// Manages a sequence of characters being typed by the user.
/// Maintains the current word/syllable being composed.
///
/// # Business Rules
/// - Buffer is cleared on word boundaries (space, enter, punctuation)
/// - Maximum size is enforced to prevent memory issues
/// - Operations are fail-safe (gracefully handle overflow)
///
/// # Examples
/// ```
/// # use goxviet_core::domain::entities::buffer::InputBuffer;
/// let mut buffer = InputBuffer::new();
/// buffer.append('h');
/// buffer.append('i');
/// assert_eq!(buffer.current_word(), "hi");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputBuffer {
    /// Current buffer content
    content: CharSequence,
    /// Cursor position (index in characters, not bytes)
    cursor: usize,
}

impl InputBuffer {
    /// Create a new empty buffer
    pub fn new() -> Self {
        Self {
            content: CharSequence::empty(),
            cursor: 0,
        }
    }

    /// Create buffer with initial content
    pub fn with_content(content: impl Into<CharSequence>) -> Self {
        let content = content.into();
        let cursor = content.len();
        Self { content, cursor }
    }

    /// Get current buffer content
    #[inline]
    pub fn content(&self) -> &CharSequence {
        &self.content
    }

    /// Get current word (entire buffer content)
    ///
    /// In IME context, the buffer typically contains one word/syllable.
    #[inline]
    pub fn current_word(&self) -> &str {
        self.content.as_str()
    }

    /// Get buffer length in characters
    #[inline]
    pub fn len(&self) -> usize {
        self.content.len()
    }

    /// Check if buffer is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Check if buffer is at maximum capacity
    #[inline]
    pub fn is_full(&self) -> bool {
        self.len() >= MAX_BUFFER_SIZE
    }

    /// Get cursor position
    #[inline]
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Append a character to the buffer
    ///
    /// Returns false if buffer is full.
    pub fn append(&mut self, ch: char) -> bool {
        if self.is_full() {
            return false;
        }

        self.content = self.content.push(ch);
        self.cursor = self.content.len();
        true
    }

    /// Append a string to the buffer
    ///
    /// Returns the number of characters actually appended.
    pub fn append_str(&mut self, s: &str) -> usize {
        let mut count = 0;
        for ch in s.chars() {
            if !self.append(ch) {
                break;
            }
            count += 1;
        }
        count
    }

    /// Delete the last character
    ///
    /// Returns the deleted character, or None if buffer is empty.
    pub fn delete_last(&mut self) -> Option<char> {
        if self.is_empty() {
            return None;
        }

        let deleted = self.content.last();
        if let Some(new_content) = self.content.pop() {
            self.content = new_content;
            self.cursor = self.content.len();
        }
        deleted
    }

    /// Delete multiple characters from the end
    ///
    /// Returns the number of characters actually deleted.
    pub fn delete_last_n(&mut self, n: usize) -> usize {
        let mut count = 0;
        for _ in 0..n {
            if self.delete_last().is_some() {
                count += 1;
            } else {
                break;
            }
        }
        count
    }

    /// Clear the entire buffer
    pub fn clear(&mut self) {
        self.content = CharSequence::empty();
        self.cursor = 0;
    }

    /// Replace buffer content
    ///
    /// This is a complete replacement operation.
    pub fn replace(&mut self, new_content: impl Into<CharSequence>) {
        self.content = new_content.into();
        self.cursor = self.content.len();
    }

    /// Get the last character without removing it
    pub fn peek_last(&self) -> Option<char> {
        self.content.last()
    }

    /// Get character at specific position
    pub fn char_at(&self, index: usize) -> Option<char> {
        self.content.char_at(index)
    }

    /// Check if buffer starts with a prefix
    pub fn starts_with(&self, prefix: &str) -> bool {
        self.content.starts_with(prefix)
    }

    /// Check if buffer ends with a suffix
    pub fn ends_with(&self, suffix: &str) -> bool {
        self.content.ends_with(suffix)
    }

    /// Check if buffer contains a substring
    pub fn contains(&self, pattern: &str) -> bool {
        self.content.contains(pattern)
    }

    /// Get a substring of the buffer
    ///
    /// Returns None if indices are invalid.
    pub fn substring(&self, start: usize, end: usize) -> Option<CharSequence> {
        self.content.substring(start, end)
    }

    /// Convert buffer to lowercase
    pub fn to_lowercase(&mut self) {
        self.content = self.content.to_lowercase();
    }

    /// Convert buffer to uppercase
    pub fn to_uppercase(&mut self) {
        self.content = self.content.to_uppercase();
    }

    /// Get remaining capacity
    #[inline]
    pub fn remaining_capacity(&self) -> usize {
        MAX_BUFFER_SIZE.saturating_sub(self.len())
    }

    /// Create a snapshot of current buffer state
    ///
    /// Useful for undo/redo operations.
    pub fn snapshot(&self) -> BufferSnapshot {
        BufferSnapshot {
            content: self.content.clone(),
            cursor: self.cursor,
        }
    }

    /// Restore from a snapshot
    pub fn restore(&mut self, snapshot: &BufferSnapshot) {
        self.content = snapshot.content.clone();
        self.cursor = snapshot.cursor;
    }

    /// Check if buffer contains valid UTF-8
    #[inline]
    pub fn is_valid_utf8(&self) -> bool {
        // CharSequence always contains valid UTF-8
        true
    }

    /// Iterate over characters in the buffer
    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.content.chars()
    }
}

impl Default for InputBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for InputBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl AsRef<str> for InputBuffer {
    fn as_ref(&self) -> &str {
        self.content.as_ref()
    }
}

/// Buffer snapshot for undo/redo operations
///
/// Captures the state of a buffer at a point in time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BufferSnapshot {
    content: CharSequence,
    cursor: usize,
}

impl BufferSnapshot {
    /// Get the content from snapshot
    pub fn content(&self) -> &CharSequence {
        &self.content
    }

    /// Get the cursor position from snapshot
    pub fn cursor(&self) -> usize {
        self.cursor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_creation() {
        let buffer = InputBuffer::new();
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.cursor(), 0);
    }

    #[test]
    fn test_buffer_with_content() {
        let buffer = InputBuffer::with_content("hello");
        assert_eq!(buffer.current_word(), "hello");
        assert_eq!(buffer.len(), 5);
        assert_eq!(buffer.cursor(), 5);
    }

    #[test]
    fn test_buffer_append() {
        let mut buffer = InputBuffer::new();
        assert!(buffer.append('h'));
        assert!(buffer.append('i'));
        assert_eq!(buffer.current_word(), "hi");
        assert_eq!(buffer.len(), 2);
    }

    #[test]
    fn test_buffer_append_str() {
        let mut buffer = InputBuffer::new();
        let count = buffer.append_str("hello");
        assert_eq!(count, 5);
        assert_eq!(buffer.current_word(), "hello");
    }

    #[test]
    fn test_buffer_delete_last() {
        let mut buffer = InputBuffer::with_content("hello");
        let deleted = buffer.delete_last();
        assert_eq!(deleted, Some('o'));
        assert_eq!(buffer.current_word(), "hell");
        assert_eq!(buffer.cursor(), 4);
    }

    #[test]
    fn test_buffer_delete_last_empty() {
        let mut buffer = InputBuffer::new();
        let deleted = buffer.delete_last();
        assert_eq!(deleted, None);
    }

    #[test]
    fn test_buffer_delete_last_n() {
        let mut buffer = InputBuffer::with_content("hello");
        let count = buffer.delete_last_n(3);
        assert_eq!(count, 3);
        assert_eq!(buffer.current_word(), "he");
    }

    #[test]
    fn test_buffer_delete_last_n_overflow() {
        let mut buffer = InputBuffer::with_content("hi");
        let count = buffer.delete_last_n(5);
        assert_eq!(count, 2);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_buffer_clear() {
        let mut buffer = InputBuffer::with_content("hello");
        buffer.clear();
        assert!(buffer.is_empty());
        assert_eq!(buffer.cursor(), 0);
    }

    #[test]
    fn test_buffer_replace() {
        let mut buffer = InputBuffer::with_content("hello");
        buffer.replace("world");
        assert_eq!(buffer.current_word(), "world");
        assert_eq!(buffer.cursor(), 5);
    }

    #[test]
    fn test_buffer_peek_last() {
        let buffer = InputBuffer::with_content("hello");
        assert_eq!(buffer.peek_last(), Some('o'));

        let empty = InputBuffer::new();
        assert_eq!(empty.peek_last(), None);
    }

    #[test]
    fn test_buffer_char_at() {
        let buffer = InputBuffer::with_content("hello");
        assert_eq!(buffer.char_at(0), Some('h'));
        assert_eq!(buffer.char_at(4), Some('o'));
        assert_eq!(buffer.char_at(5), None);
    }

    #[test]
    fn test_buffer_pattern_matching() {
        let buffer = InputBuffer::with_content("hello world");
        assert!(buffer.starts_with("hello"));
        assert!(buffer.ends_with("world"));
        assert!(buffer.contains("lo wo"));
        assert!(!buffer.starts_with("world"));
    }

    #[test]
    fn test_buffer_substring() {
        let buffer = InputBuffer::with_content("hello");
        let sub = buffer.substring(1, 4).unwrap();
        assert_eq!(sub.as_str(), "ell");

        assert!(buffer.substring(10, 20).is_none());
    }

    #[test]
    fn test_buffer_case_conversion() {
        let mut buffer = InputBuffer::with_content("Hello");

        buffer.to_lowercase();
        assert_eq!(buffer.current_word(), "hello");

        buffer.to_uppercase();
        assert_eq!(buffer.current_word(), "HELLO");
    }

    #[test]
    fn test_buffer_remaining_capacity() {
        let buffer = InputBuffer::with_content("hello");
        assert_eq!(buffer.remaining_capacity(), MAX_BUFFER_SIZE - 5);
    }

    #[test]
    fn test_buffer_snapshot_restore() {
        let mut buffer = InputBuffer::with_content("hello");
        let snapshot = buffer.snapshot();

        buffer.append('!');
        assert_eq!(buffer.current_word(), "hello!");

        buffer.restore(&snapshot);
        assert_eq!(buffer.current_word(), "hello");
        assert_eq!(buffer.cursor(), 5);
    }

    #[test]
    fn test_buffer_vietnamese() {
        let mut buffer = InputBuffer::new();
        buffer.append_str("xin chào");
        assert_eq!(buffer.len(), 8);
        assert!(buffer.contains("chào"));
    }

    #[test]
    fn test_buffer_display() {
        let buffer = InputBuffer::with_content("hello");
        assert_eq!(format!("{}", buffer), "hello");
    }

    #[test]
    fn test_buffer_chars_iterator() {
        let buffer = InputBuffer::with_content("abc");
        let chars: Vec<char> = buffer.chars().collect();
        assert_eq!(chars, vec!['a', 'b', 'c']);
    }
}
