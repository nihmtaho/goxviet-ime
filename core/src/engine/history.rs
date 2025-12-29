//! Word History Ring Buffer
//!
//! This module provides a fixed-size ring buffer for storing word history,
//! used for the "backspace-after-space" feature. When a user presses backspace
//! immediately after committing a word with space, we can restore the previous
//! buffer state to allow editing.
//!
//! # Architecture
//!
//! The history stores pairs of (Buffer, RawInputBuffer) representing:
//! - `Buffer`: The displayed Vietnamese characters with diacritics
//! - `RawInputBuffer`: The raw keystrokes that produced the buffer
//!
//! Both are needed for correct restoration when the user continues typing
//! after pressing backspace.
//!
//! # Performance
//!
//! - Stack-allocated ring buffer (no heap allocation)
//! - O(1) push and pop operations
//! - Fixed capacity of 10 words (configurable via HISTORY_CAPACITY)
//!
//! # Example
//!
//! ```ignore
//! let mut history = WordHistory::new();
//!
//! // User types "việt" and presses space
//! history.push(buffer.clone(), raw_input.clone());
//!
//! // User presses backspace - restore previous word
//! if let Some((buf, raw)) = history.pop() {
//!     // Restore editing state
//! }
//! ```

use super::buffer::Buffer;
use super::raw_input_buffer::RawInputBuffer;

/// Ring buffer capacity (stores last N committed words)
///
/// This value is chosen to balance memory usage with practical needs:
/// - 10 words covers most editing scenarios
/// - Total memory: ~10 * (256 + 264) ≈ 5.2KB
/// - Larger values would increase memory without much benefit
pub const HISTORY_CAPACITY: usize = 10;

/// Ring buffer for word history
///
/// Stores pairs of (Buffer, RawInputBuffer) for each committed word.
/// Uses a ring buffer pattern for O(1) push/pop operations.
///
/// # Memory Layout
///
/// ```text
/// ┌─────────────────────────────────────────────┐
/// │ buffers[0..10]     │ ~2560 bytes            │
/// │ raw_inputs[0..10]  │ ~2640 bytes            │
/// │ head: usize        │ 8 bytes                │
/// │ len: usize         │ 8 bytes                │
/// └─────────────────────────────────────────────┘
/// Total: ~5.2KB (stack-allocated)
/// ```
///
/// # Thread Safety
///
/// Not thread-safe. Should be owned by a single Engine instance.
#[derive(Clone)]
pub struct WordHistory {
    /// Ring buffer for displayed buffers
    buffers: [Buffer; HISTORY_CAPACITY],
    /// Ring buffer for raw keystroke history
    raw_inputs: [RawInputBuffer; HISTORY_CAPACITY],
    /// Current head position (next write index)
    head: usize,
    /// Current number of elements (0 to HISTORY_CAPACITY)
    len: usize,
}

impl Default for WordHistory {
    fn default() -> Self {
        Self::new()
    }
}

impl WordHistory {
    /// Create a new empty word history
    ///
    /// # Performance
    /// O(1) - arrays are initialized with default values
    pub fn new() -> Self {
        Self {
            buffers: std::array::from_fn(|_| Buffer::new()),
            raw_inputs: std::array::from_fn(|_| RawInputBuffer::new()),
            head: 0,
            len: 0,
        }
    }

    /// Push buffer and raw_input to history
    ///
    /// If the history is full, the oldest entry is overwritten.
    ///
    /// # Arguments
    /// * `buf` - The displayed buffer state to save
    /// * `raw` - The raw keystroke history to save
    ///
    /// # Performance
    /// O(1) - simple array write and counter update
    #[inline]
    pub fn push(&mut self, buf: Buffer, raw: RawInputBuffer) {
        self.buffers[self.head] = buf;
        self.raw_inputs[self.head] = raw;
        self.head = (self.head + 1) % HISTORY_CAPACITY;
        if self.len < HISTORY_CAPACITY {
            self.len += 1;
        }
    }

    /// Pop most recent buffer and raw_input from history
    ///
    /// Returns `None` if history is empty.
    ///
    /// # Returns
    /// `Some((Buffer, RawInputBuffer))` - The most recently pushed entry
    /// `None` - If history is empty
    ///
    /// # Performance
    /// O(1) - simple array access and counter update
    #[inline]
    pub fn pop(&mut self) -> Option<(Buffer, RawInputBuffer)> {
        if self.len == 0 {
            return None;
        }
        self.head = (self.head + HISTORY_CAPACITY - 1) % HISTORY_CAPACITY;
        self.len -= 1;
        Some((
            self.buffers[self.head].clone(),
            self.raw_inputs[self.head].clone(),
        ))
    }

    /// Peek at the most recent entry without removing it
    ///
    /// # Returns
    /// `Some((&Buffer, &RawInputBuffer))` - Reference to the most recent entry
    /// `None` - If history is empty
    #[inline]
    pub fn peek(&self) -> Option<(&Buffer, &RawInputBuffer)> {
        if self.len == 0 {
            return None;
        }
        let index = (self.head + HISTORY_CAPACITY - 1) % HISTORY_CAPACITY;
        Some((&self.buffers[index], &self.raw_inputs[index]))
    }

    /// Clear all entries from history
    ///
    /// # Performance
    /// O(1) - only resets counters, doesn't clear array contents
    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
        self.head = 0;
    }

    /// Get current number of entries in history
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Check if history is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Check if history is at capacity
    #[inline]
    pub fn is_full(&self) -> bool {
        self.len == HISTORY_CAPACITY
    }

    /// Get the capacity of the history
    #[inline]
    pub fn capacity(&self) -> usize {
        HISTORY_CAPACITY
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::keys;
    use crate::engine::buffer::Char;

    /// Helper: Create a simple buffer with given letters
    fn make_buffer(letters: &str) -> Buffer {
        let mut buf = Buffer::new();
        for ch in letters.chars() {
            let key = match ch.to_ascii_lowercase() {
                'a' => keys::A,
                'b' => keys::B,
                'c' => keys::C,
                'd' => keys::D,
                'e' => keys::E,
                'i' => keys::I,
                'o' => keys::O,
                't' => keys::T,
                'u' => keys::U,
                'v' => keys::V,
                _ => continue,
            };
            buf.push(Char::new(key, ch.is_uppercase()));
        }
        buf
    }

    /// Helper: Create a simple raw input buffer
    fn make_raw_input(letters: &str) -> RawInputBuffer {
        let mut raw = RawInputBuffer::new();
        for ch in letters.chars() {
            let key = match ch.to_ascii_lowercase() {
                'a' => keys::A,
                'b' => keys::B,
                'c' => keys::C,
                'd' => keys::D,
                'e' => keys::E,
                'i' => keys::I,
                'o' => keys::O,
                't' => keys::T,
                'u' => keys::U,
                'v' => keys::V,
                _ => continue,
            };
            raw.push(key, ch.is_uppercase());
        }
        raw
    }

    #[test]
    fn test_new_history_is_empty() {
        let history = WordHistory::new();
        assert!(history.is_empty());
        assert_eq!(history.len(), 0);
        assert!(!history.is_full());
    }

    #[test]
    fn test_push_and_pop() {
        let mut history = WordHistory::new();

        let buf1 = make_buffer("viet");
        let raw1 = make_raw_input("viet");
        history.push(buf1.clone(), raw1.clone());

        assert_eq!(history.len(), 1);
        assert!(!history.is_empty());

        let (popped_buf, popped_raw) = history.pop().unwrap();
        assert_eq!(popped_buf.len(), buf1.len());
        assert_eq!(popped_raw.len(), raw1.len());

        assert!(history.is_empty());
    }

    #[test]
    fn test_pop_empty_returns_none() {
        let mut history = WordHistory::new();
        assert!(history.pop().is_none());
    }

    #[test]
    fn test_peek() {
        let mut history = WordHistory::new();

        let buf = make_buffer("test");
        let raw = make_raw_input("test");
        history.push(buf.clone(), raw.clone());

        // Peek should return reference without removing
        let (peeked_buf, peeked_raw) = history.peek().unwrap();
        assert_eq!(peeked_buf.len(), buf.len());
        assert_eq!(peeked_raw.len(), raw.len());

        // Still have 1 element
        assert_eq!(history.len(), 1);

        // Pop should still work
        assert!(history.pop().is_some());
        assert!(history.is_empty());
    }

    #[test]
    fn test_peek_empty_returns_none() {
        let history = WordHistory::new();
        assert!(history.peek().is_none());
    }

    #[test]
    fn test_clear() {
        let mut history = WordHistory::new();

        history.push(make_buffer("a"), make_raw_input("a"));
        history.push(make_buffer("b"), make_raw_input("b"));
        history.push(make_buffer("c"), make_raw_input("c"));

        assert_eq!(history.len(), 3);

        history.clear();
        assert!(history.is_empty());
        assert_eq!(history.len(), 0);
    }

    #[test]
    fn test_lifo_order() {
        let mut history = WordHistory::new();

        // Push in order: a, b, c
        history.push(make_buffer("a"), make_raw_input("a"));
        history.push(make_buffer("bb"), make_raw_input("bb"));
        history.push(make_buffer("ccc"), make_raw_input("ccc"));

        // Pop should return in reverse order: c, b, a
        let (buf, _) = history.pop().unwrap();
        assert_eq!(buf.len(), 3); // "ccc"

        let (buf, _) = history.pop().unwrap();
        assert_eq!(buf.len(), 2); // "bb"

        let (buf, _) = history.pop().unwrap();
        assert_eq!(buf.len(), 1); // "a"

        assert!(history.is_empty());
    }

    #[test]
    fn test_overflow_wraps_around() {
        let mut history = WordHistory::new();

        // Push more than capacity
        for i in 0..15 {
            let s: String = std::iter::repeat('a').take(i + 1).collect();
            history.push(make_buffer(&s), make_raw_input(&s));
        }

        // Should still have exactly CAPACITY elements
        assert_eq!(history.len(), HISTORY_CAPACITY);
        assert!(history.is_full());

        // Most recent should be the last one pushed (15 'a's)
        let (buf, _) = history.peek().unwrap();
        assert_eq!(buf.len(), 15);

        // Pop all and verify LIFO order
        for i in (6..=15).rev() {
            let (buf, _) = history.pop().unwrap();
            assert_eq!(buf.len(), i, "Expected buffer with {} chars", i);
        }

        assert!(history.is_empty());
    }

    #[test]
    fn test_capacity() {
        let history = WordHistory::new();
        assert_eq!(history.capacity(), HISTORY_CAPACITY);
    }

    #[test]
    fn test_is_full() {
        let mut history = WordHistory::new();

        for i in 0..HISTORY_CAPACITY {
            assert!(!history.is_full());
            history.push(make_buffer("a"), make_raw_input("a"));
            if i < HISTORY_CAPACITY - 1 {
                assert!(!history.is_full());
            }
        }

        assert!(history.is_full());
    }

    #[test]
    fn test_default() {
        let history = WordHistory::default();
        assert!(history.is_empty());
        assert_eq!(history.capacity(), HISTORY_CAPACITY);
    }
}