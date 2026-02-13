//! Memory Buffer Adapter
//!
//! In-memory implementation of BufferManager using InputBuffer.

use crate::domain::{
    entities::buffer::InputBuffer,
    ports::state::BufferManager,
};

/// Memory-based buffer adapter
///
/// Simple in-memory implementation of BufferManager using InputBuffer entity.
/// Provides fast, volatile buffer management for typing sessions.
///
/// # Examples
///
/// ```
/// # use goxviet_core::infrastructure::adapters::state::MemoryBufferAdapter;
/// # use goxviet_core::domain::ports::state::BufferManager;
/// let mut adapter = MemoryBufferAdapter::new();
/// assert!(adapter.append("h"));
/// assert!(adapter.append("o"));
/// assert!(adapter.append("a"));
/// assert_eq!(adapter.current().content().as_str(), "hoa");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryBufferAdapter {
    buffer: InputBuffer,
}

impl MemoryBufferAdapter {
    /// Creates a new empty memory buffer adapter
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::infrastructure::adapters::state::MemoryBufferAdapter;
    /// let adapter = MemoryBufferAdapter::new();
    /// assert!(adapter.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            buffer: InputBuffer::new(),
        }
    }
}

impl Default for MemoryBufferAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl BufferManager for MemoryBufferAdapter {
    fn current(&self) -> &InputBuffer {
        &self.buffer
    }

    fn current_mut(&mut self) -> &mut InputBuffer {
        &mut self.buffer
    }

    fn append(&mut self, text: &str) -> bool {
        let appended = self.buffer.append_str(text);
        appended == text.chars().count()
    }

    fn delete(&mut self, count: usize) -> usize {
        self.buffer.delete_last_n(count)
    }

    fn replace(&mut self, new_content: &str) {
        self.buffer.replace(new_content);
    }

    fn clear(&mut self) {
        self.buffer.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let adapter = MemoryBufferAdapter::new();
        assert!(adapter.is_empty());
        assert_eq!(adapter.len(), 0);
    }

    #[test]
    fn test_default() {
        let adapter = MemoryBufferAdapter::default();
        assert!(adapter.is_empty());
    }

    #[test]
    fn test_append() {
        let mut adapter = MemoryBufferAdapter::new();
        assert!(adapter.append("h"));
        assert!(adapter.append("o"));
        assert!(adapter.append("a"));
        assert_eq!(adapter.current().content().as_str(), "hoa");
        assert_eq!(adapter.len(), 3);
    }

    #[test]
    fn test_append_multiple_chars() {
        let mut adapter = MemoryBufferAdapter::new();
        assert!(adapter.append("hello"));
        assert_eq!(adapter.current().content().as_str(), "hello");
        assert_eq!(adapter.len(), 5);
    }

    #[test]
    fn test_append_returns_false_when_full() {
        let mut adapter = MemoryBufferAdapter::new();
        // Fill buffer to capacity (256 chars)
        let large_text = "a".repeat(256);
        assert!(adapter.append(&large_text));
        
        // Try to append more - should return false
        assert!(!adapter.append("b"));
    }

    #[test]
    fn test_delete() {
        let mut adapter = MemoryBufferAdapter::new();
        adapter.append("hello");
        
        let deleted = adapter.delete(2);
        assert_eq!(deleted, 2);
        assert_eq!(adapter.current().content().as_str(), "hel");
    }

    #[test]
    fn test_delete_more_than_available() {
        let mut adapter = MemoryBufferAdapter::new();
        adapter.append("hi");
        
        let deleted = adapter.delete(5);
        assert_eq!(deleted, 2); // Only deleted what was available
        assert!(adapter.is_empty());
    }

    #[test]
    fn test_replace() {
        let mut adapter = MemoryBufferAdapter::new();
        adapter.append("hello");
        adapter.replace("world");
        assert_eq!(adapter.current().content().as_str(), "world");
    }

    #[test]
    fn test_clear() {
        let mut adapter = MemoryBufferAdapter::new();
        adapter.append("hello");
        adapter.clear();
        assert!(adapter.is_empty());
        assert_eq!(adapter.len(), 0);
    }

    #[test]
    fn test_is_empty() {
        let mut adapter = MemoryBufferAdapter::new();
        assert!(adapter.is_empty());
        
        adapter.append("a");
        assert!(!adapter.is_empty());
        
        adapter.clear();
        assert!(adapter.is_empty());
    }

    #[test]
    fn test_len() {
        let mut adapter = MemoryBufferAdapter::new();
        assert_eq!(adapter.len(), 0);
        
        adapter.append("hello");
        assert_eq!(adapter.len(), 5);
        
        adapter.delete(2);
        assert_eq!(adapter.len(), 3);
    }

    #[test]
    fn test_vietnamese_text() {
        let mut adapter = MemoryBufferAdapter::new();
        adapter.append("tr");
        adapter.append("ư");
        adapter.append("ờ");
        adapter.append("ng");
        
        assert_eq!(adapter.current().content().as_str(), "trường");
        assert_eq!(adapter.len(), 6);
    }

    #[test]
    fn test_current_and_current_mut() {
        let mut adapter = MemoryBufferAdapter::new();
        adapter.append("test");
        
        // Test immutable reference
        let buffer_ref = adapter.current();
        assert_eq!(buffer_ref.content().as_str(), "test");
        
        // Test mutable reference
        let buffer_mut = adapter.current_mut();
        buffer_mut.append('!');
        assert_eq!(adapter.current().content().as_str(), "test!");
    }

    #[test]
    fn test_snapshot() {
        let mut adapter = MemoryBufferAdapter::new();
        adapter.append("hello");
        
        let snapshot = adapter.snapshot();
        assert_eq!(snapshot.as_str(), "hello");
        
        adapter.append(" world");
        assert_eq!(adapter.current().content().as_str(), "hello world");
        assert_eq!(snapshot.as_str(), "hello"); // Snapshot unchanged
    }
}
