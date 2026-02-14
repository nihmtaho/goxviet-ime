//! Simple History Adapter
//!
//! Linear history implementation with capacity limits.

use crate::domain::{
    ports::state::{HistoryEntry, HistoryTracker},
    value_objects::char_sequence::CharSequence,
};

/// Simple linear history adapter
///
/// Implements HistoryTracker with a linear history buffer and capacity limits.
/// When capacity is exceeded, oldest entries are dropped.
///
/// # Behavior
///
/// - Recording new entries truncates forward history (redo stack)
/// - When capacity is reached, oldest entries are dropped
/// - Current position points to the active entry
///
/// # Examples
///
/// ```
/// # use goxviet_core::infrastructure::adapters::state::SimpleHistoryAdapter;
/// # use goxviet_core::domain::ports::state::HistoryTracker;
/// # use goxviet_core::domain::value_objects::char_sequence::CharSequence;
/// let mut adapter = SimpleHistoryAdapter::new(10);
/// 
/// adapter.record(CharSequence::from("h"));
/// adapter.record(CharSequence::from("ho"));
/// adapter.record(CharSequence::from("hoa"));
/// 
/// let prev = adapter.undo();
/// assert_eq!(prev.unwrap().as_str(), "ho");
/// 
/// let next = adapter.redo();
/// assert_eq!(next.unwrap().as_str(), "hoa");
/// ```
#[derive(Debug, Clone)]
pub struct SimpleHistoryAdapter {
    entries: Vec<HistoryEntry>,
    current: usize,
    capacity: usize,
}

impl SimpleHistoryAdapter {
    /// Creates a new history adapter with specified capacity
    ///
    /// # Arguments
    ///
    /// - `capacity`: Maximum number of history entries (must be >= 1)
    ///
    /// # Panics
    ///
    /// Panics if capacity is 0
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::infrastructure::adapters::state::SimpleHistoryAdapter;
    /// let adapter = SimpleHistoryAdapter::new(50);
    /// assert_eq!(adapter.history_size(), 0);
    /// ```
    pub fn new(capacity: usize) -> Self {
        assert!(capacity >= 1, "History capacity must be at least 1");
        Self {
            entries: Vec::new(),
            current: 0,
            capacity,
        }
    }
}

impl HistoryTracker for SimpleHistoryAdapter {
    fn record(&mut self, content: CharSequence) {
        // Special case: first entry
        if self.entries.is_empty() {
            self.entries.push(HistoryEntry::new(content));
            self.current = 0;
            return;
        }

        // Truncate forward history if not at end
        if self.current + 1 < self.entries.len() {
            self.entries.truncate(self.current + 1);
        }

        // Add new entry
        self.entries.push(HistoryEntry::new(content));

        // Handle capacity overflow
        if self.entries.len() > self.capacity {
            self.entries.remove(0);
            if self.current > 0 {
                self.current -= 1;
            }
        }

        // Set current to last index
        self.current = self.entries.len() - 1;
    }

    fn undo(&mut self) -> Option<CharSequence> {
        if self.can_undo() {
            self.current -= 1;
            Some(self.entries[self.current].content.clone())
        } else {
            None
        }
    }

    fn redo(&mut self) -> Option<CharSequence> {
        if self.can_redo() {
            self.current += 1;
            Some(self.entries[self.current].content.clone())
        } else {
            None
        }
    }

    fn can_undo(&self) -> bool {
        self.current > 0 && !self.entries.is_empty()
    }

    fn can_redo(&self) -> bool {
        self.current + 1 < self.entries.len()
    }

    fn clear(&mut self) {
        self.entries.clear();
        self.current = 0;
    }

    fn history_size(&self) -> usize {
        self.entries.len()
    }

    fn current_position(&self) -> usize {
        self.current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let adapter = SimpleHistoryAdapter::new(10);
        assert_eq!(adapter.history_size(), 0);
        assert_eq!(adapter.current_position(), 0);
        assert!(!adapter.can_undo());
        assert!(!adapter.can_redo());
    }

    #[test]
    #[should_panic(expected = "History capacity must be at least 1")]
    fn test_new_zero_capacity_panics() {
        SimpleHistoryAdapter::new(0);
    }

    #[test]
    fn test_record_first_entry() {
        let mut adapter = SimpleHistoryAdapter::new(10);
        adapter.record(CharSequence::from("hello"));
        
        assert_eq!(adapter.history_size(), 1);
        assert_eq!(adapter.current_position(), 0);
        assert!(!adapter.can_undo());
        assert!(!adapter.can_redo());
    }

    #[test]
    fn test_record_multiple_entries() {
        let mut adapter = SimpleHistoryAdapter::new(10);
        adapter.record(CharSequence::from("h"));
        adapter.record(CharSequence::from("he"));
        adapter.record(CharSequence::from("hel"));
        
        assert_eq!(adapter.history_size(), 3);
        assert_eq!(adapter.current_position(), 2);
        assert!(adapter.can_undo());
        assert!(!adapter.can_redo());
    }

    #[test]
    fn test_undo() {
        let mut adapter = SimpleHistoryAdapter::new(10);
        adapter.record(CharSequence::from("h"));
        adapter.record(CharSequence::from("he"));
        adapter.record(CharSequence::from("hel"));
        
        let prev = adapter.undo();
        assert_eq!(prev.unwrap().as_str(), "he");
        assert_eq!(adapter.current_position(), 1);
        assert!(adapter.can_undo());
        assert!(adapter.can_redo());
    }

    #[test]
    fn test_undo_to_first_entry() {
        let mut adapter = SimpleHistoryAdapter::new(10);
        adapter.record(CharSequence::from("h"));
        adapter.record(CharSequence::from("he"));
        
        adapter.undo();
        assert_eq!(adapter.current_position(), 0);
        assert!(!adapter.can_undo());
        assert!(adapter.can_redo());
    }

    #[test]
    fn test_undo_on_empty() {
        let mut adapter = SimpleHistoryAdapter::new(10);
        let result = adapter.undo();
        assert!(result.is_none());
    }

    #[test]
    fn test_redo() {
        let mut adapter = SimpleHistoryAdapter::new(10);
        adapter.record(CharSequence::from("h"));
        adapter.record(CharSequence::from("he"));
        adapter.record(CharSequence::from("hel"));
        
        adapter.undo();
        let next = adapter.redo();
        assert_eq!(next.unwrap().as_str(), "hel");
        assert_eq!(adapter.current_position(), 2);
        assert!(adapter.can_undo());
        assert!(!adapter.can_redo());
    }

    #[test]
    fn test_redo_without_undo() {
        let mut adapter = SimpleHistoryAdapter::new(10);
        adapter.record(CharSequence::from("hello"));
        
        let result = adapter.redo();
        assert!(result.is_none());
    }

    #[test]
    fn test_record_after_undo_truncates() {
        let mut adapter = SimpleHistoryAdapter::new(10);
        adapter.record(CharSequence::from("h"));
        adapter.record(CharSequence::from("he"));
        adapter.record(CharSequence::from("hel"));
        
        adapter.undo(); // Back to "he"
        assert!(adapter.can_redo());
        
        adapter.record(CharSequence::from("ho")); // New branch
        assert!(!adapter.can_redo());
        assert_eq!(adapter.history_size(), 3); // ["h", "he", "ho"]
        
        let prev = adapter.undo();
        assert_eq!(prev.unwrap().as_str(), "he");
    }

    #[test]
    fn test_capacity_limit_drops_oldest() {
        let mut adapter = SimpleHistoryAdapter::new(3);
        adapter.record(CharSequence::from("a"));
        adapter.record(CharSequence::from("b"));
        adapter.record(CharSequence::from("c"));
        adapter.record(CharSequence::from("d")); // Should drop "a"
        
        assert_eq!(adapter.history_size(), 3);
        assert_eq!(adapter.current_position(), 2);
        
        // Verify oldest was dropped
        adapter.undo();
        adapter.undo();
        assert_eq!(adapter.entries[adapter.current_position()].content.as_str(), "b");
    }

    #[test]
    fn test_capacity_limit_multiple_drops() {
        let mut adapter = SimpleHistoryAdapter::new(2);
        adapter.record(CharSequence::from("a"));
        adapter.record(CharSequence::from("b"));
        adapter.record(CharSequence::from("c"));
        adapter.record(CharSequence::from("d"));
        adapter.record(CharSequence::from("e"));
        
        assert_eq!(adapter.history_size(), 2);
        assert_eq!(adapter.current_position(), 1);
        
        // Should only have "d" and "e"
        adapter.undo();
        assert_eq!(adapter.entries[adapter.current_position()].content.as_str(), "d");
    }

    #[test]
    fn test_clear() {
        let mut adapter = SimpleHistoryAdapter::new(10);
        adapter.record(CharSequence::from("h"));
        adapter.record(CharSequence::from("he"));
        adapter.record(CharSequence::from("hel"));
        
        adapter.clear();
        assert_eq!(adapter.history_size(), 0);
        assert_eq!(adapter.current_position(), 0);
        assert!(!adapter.can_undo());
        assert!(!adapter.can_redo());
    }

    #[test]
    fn test_can_undo_false_when_empty() {
        let adapter = SimpleHistoryAdapter::new(10);
        assert!(!adapter.can_undo());
    }

    #[test]
    fn test_can_undo_false_at_first_entry() {
        let mut adapter = SimpleHistoryAdapter::new(10);
        adapter.record(CharSequence::from("hello"));
        assert!(!adapter.can_undo());
    }

    #[test]
    fn test_can_redo_false_when_empty() {
        let adapter = SimpleHistoryAdapter::new(10);
        assert!(!adapter.can_redo());
    }

    #[test]
    fn test_can_redo_false_at_latest_entry() {
        let mut adapter = SimpleHistoryAdapter::new(10);
        adapter.record(CharSequence::from("h"));
        adapter.record(CharSequence::from("he"));
        assert!(!adapter.can_redo());
    }

    #[test]
    fn test_undo_redo_flow() {
        let mut adapter = SimpleHistoryAdapter::new(10);
        adapter.record(CharSequence::from("h"));
        adapter.record(CharSequence::from("ho"));
        adapter.record(CharSequence::from("hoa"));
        
        // Undo twice
        let prev1 = adapter.undo();
        assert_eq!(prev1.unwrap().as_str(), "ho");
        let prev2 = adapter.undo();
        assert_eq!(prev2.unwrap().as_str(), "h");
        
        assert!(!adapter.can_undo());
        assert!(adapter.can_redo());
        
        // Redo twice
        let next1 = adapter.redo();
        assert_eq!(next1.unwrap().as_str(), "ho");
        let next2 = adapter.redo();
        assert_eq!(next2.unwrap().as_str(), "hoa");
        
        assert!(adapter.can_undo());
        assert!(!adapter.can_redo());
    }

    #[test]
    fn test_vietnamese_text() {
        let mut adapter = SimpleHistoryAdapter::new(10);
        adapter.record(CharSequence::from("h"));
        adapter.record(CharSequence::from("ho"));
        adapter.record(CharSequence::from("hoa"));
        adapter.record(CharSequence::from("hoà"));
        
        let prev = adapter.undo();
        assert_eq!(prev.unwrap().as_str(), "hoa");
    }

    #[test]
    fn test_capacity_one() {
        let mut adapter = SimpleHistoryAdapter::new(1);
        adapter.record(CharSequence::from("a"));
        adapter.record(CharSequence::from("b"));
        adapter.record(CharSequence::from("c"));
        
        assert_eq!(adapter.history_size(), 1);
        assert_eq!(adapter.current_position(), 0);
        assert_eq!(adapter.entries[0].content.as_str(), "c");
    }
}
