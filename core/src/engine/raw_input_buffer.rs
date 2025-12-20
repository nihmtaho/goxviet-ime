//! Raw Input Buffer - Fixed-size circular buffer for keystroke history
//!
//! This module implements a memory-efficient circular buffer to store raw keystroke
//! history for ESC restore functionality. It replaces the previous Vec<(u16, bool)>
//! implementation to provide:
//! - Bounded memory usage (no heap reallocation)
//! - Better cache locality
//! - Reduced memory fragmentation
//!
//! Based on reference implementation architecture principles

/// Maximum capacity for raw input buffer
/// This limits the maximum word length that can be restored via ESC
/// 64 keystrokes is sufficient for most Vietnamese words and compound words
const RAW_INPUT_CAPACITY: usize = 64;

/// Fixed-size bounded buffer for raw keystroke history
///
/// Stores (key, caps) pairs representing the original keystrokes before
/// Vietnamese transformation. Used for ESC restore functionality.
///
/// When buffer reaches capacity, oldest elements are discarded (not overwritten).
/// This is appropriate for Vietnamese IME since words are typically short and
/// buffer is cleared on word boundaries (space, punctuation).
///
/// # Memory Layout
/// - Fixed array: 64 * (u16 + bool) = 64 * 3 bytes ≈ 192 bytes
/// - No heap allocation
/// - No reallocation on growth
///
/// # Performance
/// - Push: O(1) amortized (may shift on capacity)
/// - Pop: O(1)
/// - Clear: O(1)
/// - Iteration: O(n) where n = current length
#[derive(Debug, Clone)]
pub struct RawInputBuffer {
    /// Fixed-size array for keystroke data
    /// Each entry stores (key: u16, caps: bool)
    /// Data is stored contiguously from index 0 to len-1
    data: [(u16, bool); RAW_INPUT_CAPACITY],
    /// Current number of elements in buffer
    /// Always <= RAW_INPUT_CAPACITY
    len: usize,
}

impl Default for RawInputBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl RawInputBuffer {
    /// Create a new empty buffer
    #[inline]
    pub fn new() -> Self {
        Self {
            data: [(0, false); RAW_INPUT_CAPACITY],
            len: 0,
        }
    }

    /// Push a new keystroke to the buffer
    ///
    /// If buffer is at capacity, oldest elements are shifted out to make room.
    /// This ensures we always keep the most recent keystrokes.
    ///
    /// # Arguments
    /// * `key` - Key code (e.g., 'a', 's', 'f')
    /// * `caps` - Whether Shift/Caps Lock was active
    #[inline]
    pub fn push(&mut self, key: u16, caps: bool) {
        if self.len < RAW_INPUT_CAPACITY {
            // Buffer not full yet - simple append
            self.data[self.len] = (key, caps);
            self.len += 1;
        } else {
            // Buffer full - shift left and append at end
            // This discards the oldest element
            self.data.copy_within(1..RAW_INPUT_CAPACITY, 0);
            self.data[RAW_INPUT_CAPACITY - 1] = (key, caps);
            // len stays at capacity
        }
    }

    /// Remove the last keystroke from the buffer
    ///
    /// Returns the removed (key, caps) pair, or None if buffer is empty.
    #[inline]
    pub fn pop(&mut self) -> Option<(u16, bool)> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;
        Some(self.data[self.len])
    }

    /// Get the current length of the buffer
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Check if the buffer is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Clear all elements from the buffer
    ///
    /// This is O(1) - we don't need to zero memory, just reset counters
    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Get a slice view of the current buffer contents
    ///
    /// This creates a temporary Vec for compatibility with existing code.
    /// In future optimizations, we could return an iterator instead.
    ///
    /// # Performance Note
    /// This allocates a Vec, so avoid calling in hot paths.
    /// Use `iter()` for zero-allocation iteration.
    #[inline]
    pub fn as_slice(&self) -> Vec<(u16, bool)> {
        if self.len == 0 {
            return Vec::new();
        }

        let mut result = Vec::with_capacity(self.len);
        result.extend_from_slice(&self.data[..self.len]);
        result
    }

    /// Iterate over buffer contents in order (oldest to newest)
    ///
    /// Zero-allocation iterator for read-only access.
    pub fn iter(&self) -> RawInputIterator<'_> {
        RawInputIterator {
            buffer: self,
            index: 0,
        }
    }

    /// Get capacity of the buffer
    #[inline]
    pub fn capacity(&self) -> usize {
        RAW_INPUT_CAPACITY
    }
}

/// Iterator over RawInputBuffer contents
///
/// Provides zero-allocation iteration over buffer elements.
pub struct RawInputIterator<'a> {
    buffer: &'a RawInputBuffer,
    index: usize,
}

impl<'a> Iterator for RawInputIterator<'a> {
    type Item = (u16, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.buffer.len {
            return None;
        }

        let result = self.buffer.data[self.index];
        self.index += 1;
        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.buffer.len - self.index;
        (remaining, Some(remaining))
    }
}

impl<'a> ExactSizeIterator for RawInputIterator<'a> {
    fn len(&self) -> usize {
        self.buffer.len - self.index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_buffer_is_empty() {
        let buf = RawInputBuffer::new();
        assert_eq!(buf.len(), 0);
        assert!(buf.is_empty());
    }

    #[test]
    fn test_push_and_pop() {
        let mut buf = RawInputBuffer::new();
        
        buf.push(b'a' as u16, false);
        buf.push(b's' as u16, false);
        buf.push(b'A' as u16, true);
        
        assert_eq!(buf.len(), 3);
        assert_eq!(buf.pop(), Some((b'A' as u16, true)));
        assert_eq!(buf.pop(), Some((b's' as u16, false)));
        assert_eq!(buf.pop(), Some((b'a' as u16, false)));
        assert_eq!(buf.pop(), None);
        assert!(buf.is_empty());
    }

    #[test]
    fn test_clear() {
        let mut buf = RawInputBuffer::new();
        buf.push(b'a' as u16, false);
        buf.push(b's' as u16, false);
        
        buf.clear();
        assert_eq!(buf.len(), 0);
        assert!(buf.is_empty());
    }

    #[test]
    fn test_capacity_overflow() {
        let mut buf = RawInputBuffer::new();
        
        // Fill beyond capacity
        for i in 0..100 {
            buf.push(i as u16, false);
        }
        
        // Should only keep last 64 elements
        assert_eq!(buf.len(), RAW_INPUT_CAPACITY);
        
        // Check that we keep the most recent elements
        // Last element should be 99
        let last = buf.pop();
        assert_eq!(last, Some((99, false)));
        
        // Second to last should be 98
        let second_last = buf.pop();
        assert_eq!(second_last, Some((98, false)));
        
        // After popping 2, we should have 62 elements left
        assert_eq!(buf.len(), RAW_INPUT_CAPACITY - 2);
    }

    #[test]
    fn test_as_slice() {
        let mut buf = RawInputBuffer::new();
        buf.push(b'v' as u16, false);
        buf.push(b'i' as u16, false);
        buf.push(b'e' as u16, false);
        
        let slice = buf.as_slice();
        assert_eq!(slice.len(), 3);
        assert_eq!(slice[0], (b'v' as u16, false));
        assert_eq!(slice[1], (b'i' as u16, false));
        assert_eq!(slice[2], (b'e' as u16, false));
    }

    #[test]
    fn test_iterator() {
        let mut buf = RawInputBuffer::new();
        buf.push(b't' as u16, false);
        buf.push(b'o' as u16, false);
        buf.push(b'n' as u16, false);
        
        let collected: Vec<_> = buf.iter().collect();
        assert_eq!(collected.len(), 3);
        assert_eq!(collected[0], (b't' as u16, false));
        assert_eq!(collected[1], (b'o' as u16, false));
        assert_eq!(collected[2], (b'n' as u16, false));
    }

    #[test]
    fn test_iterator_exact_size() {
        let mut buf = RawInputBuffer::new();
        buf.push(b'a' as u16, false);
        buf.push(b'b' as u16, false);
        
        let mut iter = buf.iter();
        assert_eq!(iter.len(), 2);
        iter.next();
        assert_eq!(iter.len(), 1);
        iter.next();
        assert_eq!(iter.len(), 0);
    }

    #[test]
    fn test_capacity_overflow_iteration() {
        let mut buf = RawInputBuffer::new();
        
        // Fill to capacity
        for i in 0..RAW_INPUT_CAPACITY {
            buf.push(i as u16, false);
        }
        
        // Push beyond capacity - should shift out oldest elements
        buf.push(100, false);
        buf.push(101, false);
        buf.push(102, false);
        
        // Should still have exactly CAPACITY elements
        assert_eq!(buf.len(), RAW_INPUT_CAPACITY);
        
        // First element should be shifted out, so first is now element 3
        let first = buf.iter().next();
        assert_eq!(first, Some((3, false))); // Elements 0,1,2 were shifted out
        
        // Last element should be 102
        let collected: Vec<_> = buf.iter().collect();
        assert_eq!(collected.last(), Some(&(102, false)));
    }
}