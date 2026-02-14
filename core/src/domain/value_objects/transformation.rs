//! Transformation Value Object
//!
//! Represents the result of text transformation operations.

use crate::domain::entities::key_event::Action;
use crate::domain::value_objects::char_sequence::CharSequence;
use std::fmt;

/// Result of a text transformation operation
///
/// Represents what needs to be done to transform input text.
/// This is immutable and represents the outcome of processing.
///
/// # Examples
/// ```
/// # use goxviet_core::domain::value_objects::transformation::TransformResult;
/// # use goxviet_core::domain::entities::key_event::Action;
/// let result = TransformResult::replace(2, "á");
/// assert_eq!(result.backspace_count(), 2);
/// assert_eq!(result.new_text().as_str(), "á");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformResult {
    /// Action to take
    action: Action,
    /// New text to insert
    new_text: CharSequence,
    /// Whether this transformation changed anything
    modified: bool,
}

impl TransformResult {
    /// Create a new transformation result
    pub fn new(action: Action, new_text: impl Into<CharSequence>) -> Self {
        let new_text = new_text.into();
        let modified = action.is_modifying() || !new_text.is_empty();
        
        Self {
            action,
            new_text,
            modified,
        }
    }

    /// Create a no-op transformation (pass through)
    pub fn none() -> Self {
        Self {
            action: Action::None,
            new_text: CharSequence::empty(),
            modified: false,
        }
    }

    /// Create transformation that inserts text
    pub fn insert(text: impl Into<CharSequence>) -> Self {
        Self::new(Action::Insert, text)
    }

    /// Create transformation that replaces text
    ///
    /// # Arguments
    /// * `backspace_count` - Number of characters to delete
    /// * `new_text` - New text to insert
    pub fn replace(backspace_count: u8, new_text: impl Into<CharSequence>) -> Self {
        Self::new(Action::Replace { backspace_count }, new_text)
    }

    /// Create transformation that clears/commits
    pub fn clear() -> Self {
        Self {
            action: Action::Clear,
            new_text: CharSequence::empty(),
            modified: true,
        }
    }

    /// Create transformation that commits current text
    pub fn commit(text: impl Into<CharSequence>) -> Self {
        Self::new(Action::Commit, text)
    }

    /// Create transformation that undoes last action
    pub fn undo() -> Self {
        Self {
            action: Action::Undo,
            new_text: CharSequence::empty(),
            modified: true,
        }
    }

    /// Get the action to perform
    #[inline]
    pub fn action(&self) -> Action {
        self.action
    }

    /// Get the new text to insert
    #[inline]
    pub fn new_text(&self) -> &CharSequence {
        &self.new_text
    }

    /// Check if this transformation modifies anything
    #[inline]
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Check if this is a no-op
    #[inline]
    pub fn is_none(&self) -> bool {
        matches!(self.action, Action::None) && self.new_text.is_empty()
    }

    /// Get number of backspaces needed
    #[inline]
    pub fn backspace_count(&self) -> u8 {
        self.action.backspace_count()
    }

    /// Check if this transformation requires deletion
    #[inline]
    pub fn requires_deletion(&self) -> bool {
        self.action.requires_deletion()
    }

    /// Convert to owned new text string
    pub fn into_string(self) -> String {
        self.new_text.into_string()
    }

    /// Create a combined transformation from multiple transforms
    ///
    /// Useful for chaining transformations.
    pub fn chain(self, other: Self) -> Self {
        if self.is_none() {
            return other;
        }
        if other.is_none() {
            return self;
        }

        // If both modify, combine them
        let total_backspaces = self.backspace_count() + other.backspace_count();
        let combined_text = self.new_text.concat(other.new_text.as_str());

        Self::replace(total_backspaces, combined_text)
    }
}

impl Default for TransformResult {
    fn default() -> Self {
        Self::none()
    }
}

impl fmt::Display for TransformResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.action {
            Action::None => write!(f, "None"),
            Action::Insert => write!(f, "Insert(\"{}\")", self.new_text),
            Action::Replace { backspace_count } => {
                write!(f, "Replace(bs={}, \"{}\")", backspace_count, self.new_text)
            }
            Action::Clear => write!(f, "Clear"),
            Action::Commit => write!(f, "Commit(\"{}\")", self.new_text),
            Action::Undo => write!(f, "Undo"),
        }
    }
}

/// Builder for TransformResult
///
/// Provides a fluent API for constructing transformation results.
pub struct TransformBuilder {
    action: Action,
    new_text: String,
}

impl TransformBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            action: Action::None,
            new_text: String::new(),
        }
    }

    /// Set action to insert
    pub fn insert(mut self) -> Self {
        self.action = Action::Insert;
        self
    }

    /// Set action to replace with backspace count
    pub fn replace(mut self, backspace_count: u8) -> Self {
        self.action = Action::Replace { backspace_count };
        self
    }

    /// Set action to clear
    pub fn clear(mut self) -> Self {
        self.action = Action::Clear;
        self
    }

    /// Set action to commit
    pub fn commit(mut self) -> Self {
        self.action = Action::Commit;
        self
    }

    /// Set the new text
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.new_text = text.into();
        self
    }

    /// Add text to existing text
    pub fn append_text(mut self, text: &str) -> Self {
        self.new_text.push_str(text);
        self
    }

    /// Build the transformation result
    pub fn build(self) -> TransformResult {
        TransformResult::new(self.action, self.new_text)
    }
}

impl Default for TransformBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_result_none() {
        let result = TransformResult::none();
        assert!(result.is_none());
        assert!(!result.is_modified());
        assert_eq!(result.backspace_count(), 0);
    }

    #[test]
    fn test_transform_result_insert() {
        let result = TransformResult::insert("hello");
        assert!(!result.is_none());
        assert!(result.is_modified());
        assert_eq!(result.new_text().as_str(), "hello");
        assert_eq!(result.action(), Action::Insert);
    }

    #[test]
    fn test_transform_result_replace() {
        let result = TransformResult::replace(3, "world");
        assert!(result.is_modified());
        assert!(result.requires_deletion());
        assert_eq!(result.backspace_count(), 3);
        assert_eq!(result.new_text().as_str(), "world");
    }

    #[test]
    fn test_transform_result_clear() {
        let result = TransformResult::clear();
        assert!(result.is_modified());
        assert!(result.requires_deletion());
        assert_eq!(result.action(), Action::Clear);
    }

    #[test]
    fn test_transform_result_commit() {
        let result = TransformResult::commit("text");
        assert!(result.is_modified());
        assert_eq!(result.new_text().as_str(), "text");
        assert_eq!(result.action(), Action::Commit);
    }

    #[test]
    fn test_transform_result_undo() {
        let result = TransformResult::undo();
        assert!(result.is_modified());
        assert_eq!(result.action(), Action::Undo);
    }

    #[test]
    fn test_transform_result_chain() {
        let t1 = TransformResult::replace(2, "he");
        let t2 = TransformResult::replace(1, "llo");
        let combined = t1.chain(t2);

        assert_eq!(combined.backspace_count(), 3);
        assert_eq!(combined.new_text().as_str(), "hello");
    }

    #[test]
    fn test_transform_result_chain_with_none() {
        let t1 = TransformResult::none();
        let t2 = TransformResult::insert("test");
        
        let combined = t1.chain(t2.clone());
        assert_eq!(combined.new_text().as_str(), "test");

        let combined2 = t2.chain(TransformResult::none());
        assert_eq!(combined2.new_text().as_str(), "test");
    }

    #[test]
    fn test_transform_result_into_string() {
        let result = TransformResult::insert("hello");
        let text = result.into_string();
        assert_eq!(text, "hello");
    }

    #[test]
    fn test_transform_result_display() {
        let none = TransformResult::none();
        assert_eq!(format!("{}", none), "None");

        let insert = TransformResult::insert("hi");
        assert_eq!(format!("{}", insert), "Insert(\"hi\")");

        let replace = TransformResult::replace(2, "test");
        assert!(format!("{}", replace).contains("Replace"));
        assert!(format!("{}", replace).contains("bs=2"));
    }

    #[test]
    fn test_transform_builder() {
        let result = TransformBuilder::new()
            .replace(3)
            .with_text("hello")
            .build();

        assert_eq!(result.backspace_count(), 3);
        assert_eq!(result.new_text().as_str(), "hello");
    }

    #[test]
    fn test_transform_builder_insert() {
        let result = TransformBuilder::new()
            .insert()
            .with_text("world")
            .build();

        assert_eq!(result.action(), Action::Insert);
        assert_eq!(result.new_text().as_str(), "world");
    }

    #[test]
    fn test_transform_builder_append() {
        let result = TransformBuilder::new()
            .insert()
            .with_text("hello")
            .append_text(" world")
            .build();

        assert_eq!(result.new_text().as_str(), "hello world");
    }

    #[test]
    fn test_transform_builder_commit() {
        let result = TransformBuilder::new()
            .commit()
            .with_text("done")
            .build();

        assert_eq!(result.action(), Action::Commit);
        assert_eq!(result.new_text().as_str(), "done");
    }
}
