//! Buffer Manager Port
//!
//! Defines the abstraction for managing input buffers during Vietnamese typing.
//!
//! # Design Principles
//!
//! - **ISP**: Small, focused interface with essential buffer operations
//! - **DIP**: Domain defines contract, infrastructure implements
//! - **SRP**: Only manages buffer state, not transformation logic
//!
//! # Architecture
//!
//! ```text
//! Domain Layer (this file)
//!     ↓ defines interface
//! Infrastructure Layer
//!     ↓ implements
//! MemoryBufferAdapter, PersistentBufferAdapter
//! ```

use crate::domain::{
    entities::buffer::InputBuffer,
    value_objects::char_sequence::CharSequence,
};

/// Buffer manager port (interface)
///
/// Manages the current input buffer state during typing sessions.
///
/// # Responsibilities
///
/// - Track current buffer content
/// - Handle append/delete/replace operations
/// - Manage buffer lifecycle (create/clear/reset)
/// - Provide buffer snapshots for undo/redo
///
/// # Implementations
///
/// - `MemoryBufferAdapter`: In-memory buffer (fast, volatile)
/// - `PersistentBufferAdapter`: Persistent buffer (survives restarts)
///
/// # Examples
///
/// ```ignore
/// let mut manager: Box<dyn BufferManager> = Box::new(MemoryBufferAdapter::new());
///
/// manager.append("h");
/// manager.append("o");
/// manager.append("a");
/// assert_eq!(manager.current().as_str(), "hoa");
///
/// manager.clear();
/// assert!(manager.is_empty());
/// ```
pub trait BufferManager: Send + Sync {
    /// Gets the current buffer content
    ///
    /// # Returns
    ///
    /// Reference to current `InputBuffer`
    fn current(&self) -> &InputBuffer;

    /// Gets mutable reference to current buffer
    ///
    /// # Returns
    ///
    /// Mutable reference to current `InputBuffer`
    fn current_mut(&mut self) -> &mut InputBuffer;

    /// Appends text to the buffer
    ///
    /// # Arguments
    ///
    /// - `text`: Text to append
    ///
    /// # Returns
    ///
    /// `true` if successfully appended, `false` if buffer full
    ///
    /// # Examples (conceptual)
    ///
    /// ```ignore
    /// manager.append("h");
    /// manager.append("o");
    /// manager.append("a");
    /// // Buffer: "hoa"
    /// ```
    fn append(&mut self, text: &str) -> bool;

    /// Deletes last N characters from buffer
    ///
    /// # Arguments
    ///
    /// - `count`: Number of characters to delete
    ///
    /// # Returns
    ///
    /// Number of characters actually deleted
    ///
    /// # Examples (conceptual)
    ///
    /// ```ignore
    /// manager.append("hoa");
    /// manager.delete(1); // Remove 'a'
    /// // Buffer: "ho"
    /// ```
    fn delete(&mut self, count: usize) -> usize;

    /// Replaces buffer content
    ///
    /// # Arguments
    ///
    /// - `new_content`: New buffer content
    ///
    /// # Examples (conceptual)
    ///
    /// ```ignore
    /// manager.append("hoa");
    /// manager.replace("hoà");
    /// // Buffer: "hoà"
    /// ```
    fn replace(&mut self, new_content: &str);

    /// Clears the buffer
    ///
    /// # Examples (conceptual)
    ///
    /// ```ignore
    /// manager.append("hoa");
    /// manager.clear();
    /// assert!(manager.is_empty());
    /// ```
    fn clear(&mut self);

    /// Checks if buffer is empty
    ///
    /// # Returns
    ///
    /// `true` if buffer has no content
    fn is_empty(&self) -> bool {
        self.current().is_empty()
    }

    /// Gets buffer length
    ///
    /// # Returns
    ///
    /// Number of characters in buffer
    fn len(&self) -> usize {
        self.current().len()
    }

    /// Creates a snapshot of current buffer state
    ///
    /// # Returns
    ///
    /// `CharSequence` copy of current buffer
    ///
    /// # Use Case
    ///
    /// For undo/redo or rollback functionality
    fn snapshot(&self) -> CharSequence {
        self.current().content().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock implementation for testing
    struct MockBufferManager {
        buffer: InputBuffer,
    }

    impl MockBufferManager {
        fn new() -> Self {
            Self {
                buffer: InputBuffer::new(),
            }
        }
    }

    impl BufferManager for MockBufferManager {
        fn current(&self) -> &InputBuffer {
            &self.buffer
        }

        fn current_mut(&mut self) -> &mut InputBuffer {
            &mut self.buffer
        }

        fn append(&mut self, text: &str) -> bool {
            self.buffer.append_str(text) == text.len()
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

    #[test]
    fn test_buffer_manager_append() {
        let mut manager = MockBufferManager::new();
        assert!(manager.append("h"));
        assert!(manager.append("o"));
        assert!(manager.append("a"));
        assert_eq!(manager.current().content().as_str(), "hoa");
    }

    #[test]
    fn test_buffer_manager_delete() {
        let mut manager = MockBufferManager::new();
        manager.append("hoa");
        assert_eq!(manager.delete(1), 1);
        assert_eq!(manager.current().content().as_str(), "ho");
    }

    #[test]
    fn test_buffer_manager_replace() {
        let mut manager = MockBufferManager::new();
        manager.append("hoa");
        manager.replace("hoà");
        assert_eq!(manager.current().content().as_str(), "hoà");
    }

    #[test]
    fn test_buffer_manager_clear() {
        let mut manager = MockBufferManager::new();
        manager.append("hoa");
        manager.clear();
        assert!(manager.is_empty());
    }

    #[test]
    fn test_buffer_manager_is_empty() {
        let mut manager = MockBufferManager::new();
        assert!(manager.is_empty());

        manager.append("a");
        assert!(!manager.is_empty());

        manager.clear();
        assert!(manager.is_empty());
    }

    #[test]
    fn test_buffer_manager_len() {
        let mut manager = MockBufferManager::new();
        assert_eq!(manager.len(), 0);

        manager.append("hoa");
        assert_eq!(manager.len(), 3);

        manager.delete(1);
        assert_eq!(manager.len(), 2);
    }

    #[test]
    fn test_buffer_manager_snapshot() {
        let mut manager = MockBufferManager::new();
        manager.append("hoa");

        let snapshot = manager.snapshot();
        assert_eq!(snapshot.as_str(), "hoa");

        // Modify buffer
        manager.append("n");
        assert_eq!(manager.current().content().as_str(), "hoan");

        // Snapshot unchanged
        assert_eq!(snapshot.as_str(), "hoa");
    }

    #[test]
    fn test_buffer_manager_vietnamese_text() {
        let mut manager = MockBufferManager::new();
        manager.append("tr");
        manager.append("ư");
        manager.append("ờ");
        manager.append("ng");

        assert_eq!(manager.current().content().as_str(), "trường");
        assert_eq!(manager.len(), 6); // 6 characters
    }
}
