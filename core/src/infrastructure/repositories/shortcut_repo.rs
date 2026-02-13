//! Shortcut Repository
//!
//! Wrapper around the engine's shortcut table for domain layer access.

use crate::domain::ports::input::input_method::InputMethodId;
use crate::infrastructure::engine::features::shortcut::{InputMethod, Shortcut, ShortcutMatch, ShortcutTable};

/// Repository for shortcut management
pub struct ShortcutRepo {
    table: ShortcutTable,
}

impl ShortcutRepo {
    /// Create a new empty shortcut repository
    pub fn new() -> Self {
        Self {
            table: ShortcutTable::new(),
        }
    }

    /// Create a repository with default Vietnamese shortcuts
    pub fn with_defaults() -> Self {
        Self {
            table: ShortcutTable::with_defaults(),
        }
    }

    /// Add a shortcut to the repository
    pub fn add_shortcut(&mut self, shortcut: Shortcut) -> bool {
        self.table.add(shortcut)
    }

    /// Add a shortcut using trigger and replacement strings
    pub fn add(&mut self, trigger: &str, replacement: &str) -> bool {
        let shortcut = Shortcut::new(trigger, replacement);
        self.add_shortcut(shortcut)
    }

    /// Remove a shortcut by its trigger
    pub fn remove(&mut self, trigger: &str) -> Option<Shortcut> {
        self.table.remove(trigger)
    }

    /// Look up a shortcut for the given buffer
    pub fn lookup(&self, buffer: &str) -> Option<(&str, &Shortcut)> {
        self.table.lookup(buffer)
    }

    /// Look up a shortcut for a specific input method
    pub fn lookup_for_method(
        &self,
        buffer: &str,
        method: InputMethodId,
    ) -> Option<(&str, &Shortcut)> {
        let engine_method = Self::map_input_method(method);
        self.table.lookup_for_method(buffer, engine_method)
    }

    /// Try to match a shortcut in the buffer
    pub fn try_match(
        &self,
        buffer: &str,
        key_char: Option<char>,
        is_word_boundary: bool,
    ) -> Option<ShortcutMatch> {
        self.table.try_match(buffer, key_char, is_word_boundary)
    }

    /// Try to match a shortcut for a specific input method
    pub fn try_match_for_method(
        &self,
        buffer: &str,
        key_char: Option<char>,
        is_word_boundary: bool,
        method: InputMethodId,
    ) -> Option<ShortcutMatch> {
        let engine_method = Self::map_input_method(method);
        self.table
            .try_match_for_method(buffer, key_char, is_word_boundary, engine_method)
    }

    /// Serialize shortcuts to JSON
    pub fn to_json(&self) -> String {
        self.table.to_json()
    }

    /// Deserialize shortcuts from JSON
    pub fn from_json(&mut self, json: &str) -> Result<usize, &'static str> {
        self.table.from_json(json)
    }

    /// Get the number of shortcuts
    pub fn len(&self) -> usize {
        self.table.len()
    }

    /// Check if the repository is empty
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }

    /// Clear all shortcuts
    pub fn clear(&mut self) {
        self.table.clear();
    }

    /// Map InputMethodId to engine InputMethod
    fn map_input_method(method: InputMethodId) -> InputMethod {
        match method {
            InputMethodId::Telex => InputMethod::Telex,
            InputMethodId::Vni => InputMethod::Vni,
            InputMethodId::Plain => InputMethod::All,
        }
    }
}

impl Default for ShortcutRepo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_lookup() {
        let mut repo = ShortcutRepo::new();
        assert!(repo.add("tks", "thanks"));
        
        let result = repo.lookup("tks");
        assert!(result.is_some());
        let (matched, shortcut) = result.unwrap();
        assert_eq!(matched, "tks");
        assert_eq!(shortcut.replacement, "thanks");
    }

    #[test]
    fn test_try_match_for_method_with_telex() {
        let mut repo = ShortcutRepo::new();
        let shortcut = Shortcut::telex("tks", "thanks");
        assert!(repo.add_shortcut(shortcut));
        
        // Should match with Telex
        let result = repo.try_match_for_method("tks", Some(' '), true, InputMethodId::Telex);
        assert!(result.is_some());
        
        // Should not match with VNI (Telex-specific shortcut)
        let result = repo.try_match_for_method("tks", Some(' '), true, InputMethodId::Vni);
        assert!(result.is_none());
    }

    #[test]
    fn test_remove_and_clear() {
        let mut repo = ShortcutRepo::new();
        repo.add("tks", "thanks");
        repo.add("brb", "be right back");
        
        assert_eq!(repo.len(), 2);
        
        let removed = repo.remove("tks");
        assert!(removed.is_some());
        assert_eq!(repo.len(), 1);
        
        repo.clear();
        assert!(repo.is_empty());
        assert_eq!(repo.len(), 0);
    }
}
