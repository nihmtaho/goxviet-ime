//! History Tracker Port
//!
//! Defines the abstraction for tracking input history and enabling undo/redo.
//!
//! # Design Principles
//!
//! - **ISP**: Small interface with undo/redo essentials
//! - **DIP**: Domain defines contract, infrastructure implements
//! - **SRP**: Only tracks history, not buffer management
//!
//! # Architecture
//!
//! ```text
//! Domain Layer (this file)
//!     ↓ defines interface
//! Infrastructure Layer
//!     ↓ implements
//! SimpleHistoryAdapter, CircularHistoryAdapter
//! ```

use crate::domain::{entities::buffer::BufferSnapshot, value_objects::char_sequence::CharSequence};

/// History entry representing a state in buffer history
///
/// Each entry captures the buffer state at a specific point in time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryEntry {
    /// Buffer content at this point
    pub content: CharSequence,
    /// Optional metadata (e.g., timestamp, keystroke count)
    pub metadata: Option<String>,
}

impl HistoryEntry {
    /// Creates a new history entry
    ///
    /// # Arguments
    ///
    /// - `content`: Buffer content
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::domain::{
    /// #     ports::state::history_tracker::HistoryEntry,
    /// #     value_objects::char_sequence::CharSequence,
    /// # };
    /// let entry = HistoryEntry::new(CharSequence::from("hoa"));
    /// assert_eq!(entry.content.as_str(), "hoa");
    /// ```
    pub fn new(content: CharSequence) -> Self {
        Self {
            content,
            metadata: None,
        }
    }

    /// Creates entry with metadata
    ///
    /// # Arguments
    ///
    /// - `content`: Buffer content
    /// - `metadata`: Optional metadata string
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::domain::{
    /// #     ports::state::history_tracker::HistoryEntry,
    /// #     value_objects::char_sequence::CharSequence,
    /// # };
    /// let entry = HistoryEntry::with_metadata(
    ///     CharSequence::from("hoa"),
    ///     "Keystroke #5".to_string()
    /// );
    /// assert_eq!(entry.metadata, Some("Keystroke #5".to_string()));
    /// ```
    pub fn with_metadata(content: CharSequence, metadata: String) -> Self {
        Self {
            content,
            metadata: Some(metadata),
        }
    }

    /// Creates from buffer snapshot
    ///
    /// # Arguments
    ///
    /// - `snapshot`: Buffer snapshot
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let snapshot = buffer.create_snapshot();
    /// let entry = HistoryEntry::from_snapshot(&snapshot);
    /// ```
    pub fn from_snapshot(snapshot: &BufferSnapshot) -> Self {
        Self::new(snapshot.content().clone())
    }
}

/// History tracker port (interface)
///
/// Tracks input history and provides undo/redo functionality.
///
/// # Responsibilities
///
/// - Record buffer states as history entries
/// - Support undo (go back to previous state)
/// - Support redo (go forward after undo)
/// - Manage history size limits
///
/// # Implementations
///
/// - `SimpleHistoryAdapter`: Linear history with fixed size
/// - `CircularHistoryAdapter`: Circular buffer (oldest entries dropped)
///
/// # Examples
///
/// ```ignore
/// let mut tracker: Box<dyn HistoryTracker> = Box::new(SimpleHistoryAdapter::new(10));
///
/// tracker.record(CharSequence::from("h"));
/// tracker.record(CharSequence::from("ho"));
/// tracker.record(CharSequence::from("hoa"));
///
/// if let Some(prev) = tracker.undo() {
///     assert_eq!(prev.as_str(), "ho");
/// }
///
/// if let Some(next) = tracker.redo() {
///     assert_eq!(next.as_str(), "hoa");
/// }
/// ```
pub trait HistoryTracker: Send + Sync {
    /// Records a new history entry
    ///
    /// # Arguments
    ///
    /// - `content`: Current buffer content
    ///
    /// # Behavior
    ///
    /// - Adds entry to history
    /// - Clears redo stack (forward history invalidated)
    ///
    /// # Examples (conceptual)
    ///
    /// ```ignore
    /// tracker.record("h");
    /// tracker.record("ho");
    /// tracker.record("hoa");
    /// // History: ["h", "ho", "hoa"] ← current
    /// ```
    fn record(&mut self, content: CharSequence);

    /// Undoes last change
    ///
    /// # Returns
    ///
    /// - `Some(CharSequence)` with previous state
    /// - `None` if no history to undo
    ///
    /// # Examples (conceptual)
    ///
    /// ```ignore
    /// // History: ["h", "ho", "hoa"] ← current
    /// let prev = tracker.undo(); // Returns "ho"
    /// // History: ["h", "ho" ← current, "hoa"]
    /// ```
    fn undo(&mut self) -> Option<CharSequence>;

    /// Redoes previously undone change
    ///
    /// # Returns
    ///
    /// - `Some(CharSequence)` with next state
    /// - `None` if no redo available
    ///
    /// # Examples (conceptual)
    ///
    /// ```ignore
    /// // After undo: ["h", "ho" ← current, "hoa"]
    /// let next = tracker.redo(); // Returns "hoa"
    /// // History: ["h", "ho", "hoa" ← current]
    /// ```
    fn redo(&mut self) -> Option<CharSequence>;

    /// Checks if undo is available
    ///
    /// # Returns
    ///
    /// `true` if there's history to undo
    fn can_undo(&self) -> bool;

    /// Checks if redo is available
    ///
    /// # Returns
    ///
    /// `true` if there's forward history to redo
    fn can_redo(&self) -> bool;

    /// Clears all history
    ///
    /// # Examples (conceptual)
    ///
    /// ```ignore
    /// tracker.record("hoa");
    /// tracker.clear();
    /// assert!(!tracker.can_undo());
    /// ```
    fn clear(&mut self);

    /// Gets number of entries in history
    ///
    /// # Returns
    ///
    /// Total number of history entries
    fn history_size(&self) -> usize;

    /// Gets current position in history
    ///
    /// # Returns
    ///
    /// Index of current state (0-based)
    fn current_position(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_entry_new() {
        let entry = HistoryEntry::new(CharSequence::from("hoa"));
        assert_eq!(entry.content.as_str(), "hoa");
        assert_eq!(entry.metadata, None);
    }

    #[test]
    fn test_history_entry_with_metadata() {
        let entry = HistoryEntry::with_metadata(
            CharSequence::from("hoa"),
            "After Telex 's'".to_string(),
        );
        assert_eq!(entry.content.as_str(), "hoa");
        assert_eq!(entry.metadata, Some("After Telex 's'".to_string()));
    }

    #[test]
    fn test_history_entry_clone() {
        let entry1 = HistoryEntry::new(CharSequence::from("hoa"));
        let entry2 = entry1.clone();
        assert_eq!(entry1, entry2);
    }

    #[test]
    fn test_history_entry_eq() {
        let entry1 = HistoryEntry::new(CharSequence::from("hoa"));
        let entry2 = HistoryEntry::new(CharSequence::from("hoa"));
        let entry3 = HistoryEntry::new(CharSequence::from("hoà"));

        assert_eq!(entry1, entry2);
        assert_ne!(entry1, entry3);
    }

    #[test]
    fn test_history_entry_vietnamese() {
        let entry = HistoryEntry::new(CharSequence::from("trường"));
        assert_eq!(entry.content.as_str(), "trường");
    }

    // Mock implementation for trait testing
    struct MockHistoryTracker {
        history: Vec<CharSequence>,
        position: usize,
    }

    impl MockHistoryTracker {
        fn new() -> Self {
            Self {
                history: Vec::new(),
                position: 0,
            }
        }
    }

    impl HistoryTracker for MockHistoryTracker {
        fn record(&mut self, content: CharSequence) {
            // Truncate forward history (position is 1-based, so truncate at position)
            self.history.truncate(self.position);
            self.history.push(content);
            self.position = self.history.len();
        }

        fn undo(&mut self) -> Option<CharSequence> {
            if self.position > 1 {
                self.position -= 1;
                Some(self.history[self.position - 1].clone())
            } else {
                None
            }
        }

        fn redo(&mut self) -> Option<CharSequence> {
            if self.position < self.history.len() {
                self.position += 1;
                Some(self.history[self.position - 1].clone())
            } else {
                None
            }
        }

        fn can_undo(&self) -> bool {
            self.position > 1
        }

        fn can_redo(&self) -> bool {
            self.position < self.history.len()
        }

        fn clear(&mut self) {
            self.history.clear();
            self.position = 0;
        }

        fn history_size(&self) -> usize {
            self.history.len()
        }

        fn current_position(&self) -> usize {
            self.position
        }
    }

    #[test]
    fn test_history_tracker_record() {
        let mut tracker = MockHistoryTracker::new();
        tracker.record(CharSequence::from("h"));
        tracker.record(CharSequence::from("ho"));
        tracker.record(CharSequence::from("hoa"));

        assert_eq!(tracker.history_size(), 3);
        assert_eq!(tracker.current_position(), 3);
    }

    #[test]
    fn test_history_tracker_undo() {
        let mut tracker = MockHistoryTracker::new();
        tracker.record(CharSequence::from("h"));
        tracker.record(CharSequence::from("ho"));
        tracker.record(CharSequence::from("hoa"));

        let prev = tracker.undo();
        assert_eq!(prev, Some(CharSequence::from("ho")));
        assert_eq!(tracker.current_position(), 2);
    }

    #[test]
    fn test_history_tracker_redo() {
        let mut tracker = MockHistoryTracker::new();
        tracker.record(CharSequence::from("h"));
        tracker.record(CharSequence::from("ho"));
        tracker.record(CharSequence::from("hoa"));

        tracker.undo();
        let next = tracker.redo();
        assert_eq!(next, Some(CharSequence::from("hoa")));
        assert_eq!(tracker.current_position(), 3);
    }

    #[test]
    fn test_history_tracker_can_undo_redo() {
        let mut tracker = MockHistoryTracker::new();
        assert!(!tracker.can_undo());
        assert!(!tracker.can_redo());

        tracker.record(CharSequence::from("h"));
        assert!(!tracker.can_undo()); // Only 1 entry, can't undo yet

        tracker.record(CharSequence::from("ho"));
        assert!(tracker.can_undo());
        assert!(!tracker.can_redo());

        tracker.undo();
        assert!(!tracker.can_undo()); // Back at first entry
        assert!(tracker.can_redo());
    }

    #[test]
    fn test_history_tracker_clear() {
        let mut tracker = MockHistoryTracker::new();
        tracker.record(CharSequence::from("hoa"));
        tracker.clear();

        assert_eq!(tracker.history_size(), 0);
        assert!(!tracker.can_undo());
    }

    #[test]
    fn test_history_tracker_record_clears_redo() {
        let mut tracker = MockHistoryTracker::new();
        tracker.record(CharSequence::from("h"));
        tracker.record(CharSequence::from("ho"));
        tracker.record(CharSequence::from("hoa"));

        tracker.undo(); // Can redo now
        assert!(tracker.can_redo());

        tracker.record(CharSequence::from("hoà")); // New record clears redo
        assert!(!tracker.can_redo());
        assert_eq!(tracker.history_size(), 3); // ["h", "ho", "hoà"]
    }
}
