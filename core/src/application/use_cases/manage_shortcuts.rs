//! Manage Shortcuts Use Case
//!
//! Use case for managing text expansion shortcuts.
//!
//! # Responsibilities
//!
//! - Create new shortcuts
//! - Update existing shortcuts
//! - Delete shortcuts
//! - Query shortcuts by trigger
//! - List all shortcuts
//!
//! # Design
//!
//! CRUD operations for shortcut management.
//! This is a simplified placeholder implementation.

use crate::domain::value_objects::char_sequence::CharSequence;
use std::collections::HashMap;

/// Shortcut entity
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shortcut {
    /// Trigger text (what user types)
    pub trigger: CharSequence,
    /// Expansion text (what gets inserted)
    pub expansion: CharSequence,
    /// Enabled flag
    pub enabled: bool,
}

impl Shortcut {
    /// Creates a new shortcut
    pub fn new(trigger: impl Into<CharSequence>, expansion: impl Into<CharSequence>) -> Self {
        Self {
            trigger: trigger.into(),
            expansion: expansion.into(),
            enabled: true,
        }
    }

    /// Disables this shortcut
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
}

/// Shortcut operation result
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShortcutResult {
    /// Operation succeeded
    Success,
    /// Shortcut not found
    NotFound,
    /// Shortcut already exists
    AlreadyExists,
}

/// Manage shortcuts use case
///
/// Handles CRUD operations for text expansion shortcuts.
///
/// # Examples
///
/// ```ignore
/// let mut use_case = ManageShortcutsUseCase::new();
/// let shortcut = Shortcut::new("brb", "be right back");
/// use_case.create(shortcut);
/// 
/// let result = use_case.find("brb");
/// assert_eq!(result.unwrap().expansion.as_str(), "be right back");
/// ```
pub struct ManageShortcutsUseCase {
    /// In-memory storage (simplified)
    shortcuts: HashMap<String, Shortcut>,
}

impl ManageShortcutsUseCase {
    /// Creates a new use case
    pub fn new() -> Self {
        Self {
            shortcuts: HashMap::new(),
        }
    }

    /// Creates a new shortcut
    ///
    /// # Returns
    ///
    /// - `Success` if created
    /// - `AlreadyExists` if trigger already exists
    pub fn create(&mut self, shortcut: Shortcut) -> ShortcutResult {
        let key = shortcut.trigger.as_str().to_string();
        
        if self.shortcuts.contains_key(&key) {
            return ShortcutResult::AlreadyExists;
        }

        self.shortcuts.insert(key, shortcut);
        ShortcutResult::Success
    }

    /// Updates an existing shortcut
    ///
    /// # Returns
    ///
    /// - `Success` if updated
    /// - `NotFound` if trigger doesn't exist
    pub fn update(&mut self, trigger: &str, expansion: impl Into<CharSequence>) -> ShortcutResult {
        if let Some(shortcut) = self.shortcuts.get_mut(trigger) {
            shortcut.expansion = expansion.into();
            ShortcutResult::Success
        } else {
            ShortcutResult::NotFound
        }
    }

    /// Deletes a shortcut
    ///
    /// # Returns
    ///
    /// - `Success` if deleted
    /// - `NotFound` if trigger doesn't exist
    pub fn delete(&mut self, trigger: &str) -> ShortcutResult {
        if self.shortcuts.remove(trigger).is_some() {
            ShortcutResult::Success
        } else {
            ShortcutResult::NotFound
        }
    }

    /// Finds a shortcut by trigger
    pub fn find(&self, trigger: &str) -> Option<&Shortcut> {
        self.shortcuts.get(trigger)
    }

    /// Lists all shortcuts
    pub fn list(&self) -> Vec<&Shortcut> {
        self.shortcuts.values().collect()
    }

    /// Checks if a trigger exists
    pub fn exists(&self, trigger: &str) -> bool {
        self.shortcuts.contains_key(trigger)
    }

    /// Counts total shortcuts
    pub fn count(&self) -> usize {
        self.shortcuts.len()
    }

    /// Clears all shortcuts
    pub fn clear(&mut self) {
        self.shortcuts.clear();
    }
}

impl Default for ManageShortcutsUseCase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_case_creation() {
        let use_case = ManageShortcutsUseCase::new();
        assert_eq!(use_case.count(), 0);
    }

    #[test]
    fn test_create_shortcut() {
        let mut use_case = ManageShortcutsUseCase::new();
        let shortcut = Shortcut::new("brb", "be right back");
        let result = use_case.create(shortcut);
        assert_eq!(result, ShortcutResult::Success);
        assert_eq!(use_case.count(), 1);
    }

    #[test]
    fn test_create_duplicate() {
        let mut use_case = ManageShortcutsUseCase::new();
        let shortcut1 = Shortcut::new("brb", "be right back");
        let shortcut2 = Shortcut::new("brb", "bathroom break");
        
        use_case.create(shortcut1);
        let result = use_case.create(shortcut2);
        
        assert_eq!(result, ShortcutResult::AlreadyExists);
        assert_eq!(use_case.count(), 1);
    }

    #[test]
    fn test_find_shortcut() {
        let mut use_case = ManageShortcutsUseCase::new();
        let shortcut = Shortcut::new("brb", "be right back");
        use_case.create(shortcut);

        let found = use_case.find("brb");
        assert!(found.is_some());
        assert_eq!(found.unwrap().expansion.as_str(), "be right back");
    }

    #[test]
    fn test_find_nonexistent() {
        let use_case = ManageShortcutsUseCase::new();
        let found = use_case.find("xyz");
        assert!(found.is_none());
    }

    #[test]
    fn test_update_shortcut() {
        let mut use_case = ManageShortcutsUseCase::new();
        let shortcut = Shortcut::new("brb", "be right back");
        use_case.create(shortcut);

        let result = use_case.update("brb", "bathroom break");
        assert_eq!(result, ShortcutResult::Success);

        let found = use_case.find("brb").unwrap();
        assert_eq!(found.expansion.as_str(), "bathroom break");
    }

    #[test]
    fn test_update_nonexistent() {
        let mut use_case = ManageShortcutsUseCase::new();
        let result = use_case.update("xyz", "something");
        assert_eq!(result, ShortcutResult::NotFound);
    }

    #[test]
    fn test_delete_shortcut() {
        let mut use_case = ManageShortcutsUseCase::new();
        let shortcut = Shortcut::new("brb", "be right back");
        use_case.create(shortcut);

        let result = use_case.delete("brb");
        assert_eq!(result, ShortcutResult::Success);
        assert_eq!(use_case.count(), 0);
    }

    #[test]
    fn test_delete_nonexistent() {
        let mut use_case = ManageShortcutsUseCase::new();
        let result = use_case.delete("xyz");
        assert_eq!(result, ShortcutResult::NotFound);
    }

    #[test]
    fn test_list_shortcuts() {
        let mut use_case = ManageShortcutsUseCase::new();
        use_case.create(Shortcut::new("brb", "be right back"));
        use_case.create(Shortcut::new("omw", "on my way"));

        let list = use_case.list();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_exists() {
        let mut use_case = ManageShortcutsUseCase::new();
        use_case.create(Shortcut::new("brb", "be right back"));

        assert!(use_case.exists("brb"));
        assert!(!use_case.exists("xyz"));
    }

    #[test]
    fn test_clear() {
        let mut use_case = ManageShortcutsUseCase::new();
        use_case.create(Shortcut::new("brb", "be right back"));
        use_case.create(Shortcut::new("omw", "on my way"));

        use_case.clear();
        assert_eq!(use_case.count(), 0);
    }

    #[test]
    fn test_shortcut_disable() {
        let shortcut = Shortcut::new("brb", "be right back").disable();
        assert!(!shortcut.enabled);
    }
}
