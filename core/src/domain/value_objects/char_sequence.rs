//! Character Sequence Value Object
//!
//! Immutable sequence of characters used in Vietnamese text processing.

use std::fmt;
use std::ops::Deref;

/// Immutable character sequence
///
/// Represents a sequence of characters as a value object.
/// Being a value object, it is:
/// - Immutable
/// - Compared by value (not identity)
/// - Can be freely copied/cloned
///
/// # Examples
/// ```
/// # use goxviet_core::domain::value_objects::char_sequence::CharSequence;
/// let seq = CharSequence::from("hello");
/// assert_eq!(seq.len(), 5);
/// assert_eq!(seq.as_str(), "hello");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharSequence {
    chars: String,
}

impl CharSequence {
    /// Create a new character sequence from a string
    pub fn new(s: impl Into<String>) -> Self {
        Self { chars: s.into() }
    }

    /// Create an empty character sequence
    pub fn empty() -> Self {
        Self {
            chars: String::new(),
        }
    }

    /// Create from a single character
    pub fn from_char(ch: char) -> Self {
        Self {
            chars: ch.to_string(),
        }
    }

    /// Get the underlying string slice
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.chars
    }

    /// Get the length in characters (not bytes)
    #[inline]
    pub fn len(&self) -> usize {
        self.chars.chars().count()
    }

    /// Check if the sequence is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }

    /// Get character at index
    ///
    /// Returns None if index is out of bounds.
    /// Note: This is O(n) operation as it needs to iterate UTF-8 characters.
    pub fn char_at(&self, index: usize) -> Option<char> {
        self.chars.chars().nth(index)
    }

    /// Get the first character
    pub fn first(&self) -> Option<char> {
        self.chars.chars().next()
    }

    /// Get the last character
    pub fn last(&self) -> Option<char> {
        self.chars.chars().last()
    }

    /// Create a new sequence with a character appended
    ///
    /// Returns a new CharSequence (immutable operation)
    pub fn push(&self, ch: char) -> Self {
        let mut new_chars = self.chars.clone();
        new_chars.push(ch);
        Self { chars: new_chars }
    }

    /// Create a new sequence with a character removed from the end
    ///
    /// Returns a new CharSequence (immutable operation)
    /// Returns None if sequence is empty.
    pub fn pop(&self) -> Option<Self> {
        if self.is_empty() {
            return None;
        }

        let mut new_chars = self.chars.clone();
        new_chars.pop();
        Some(Self { chars: new_chars })
    }

    /// Create a new sequence with a string appended
    pub fn concat(&self, other: &str) -> Self {
        Self {
            chars: format!("{}{}", self.chars, other),
        }
    }

    /// Get a substring as a new CharSequence
    ///
    /// # Arguments
    /// * `start` - Start index (inclusive)
    /// * `end` - End index (exclusive)
    ///
    /// Returns None if indices are invalid.
    pub fn substring(&self, start: usize, end: usize) -> Option<Self> {
        if start > end || end > self.len() {
            return None;
        }

        let chars: String = self
            .chars
            .chars()
            .skip(start)
            .take(end - start)
            .collect();

        Some(Self { chars })
    }

    /// Check if sequence starts with a given prefix
    pub fn starts_with(&self, prefix: &str) -> bool {
        self.chars.starts_with(prefix)
    }

    /// Check if sequence ends with a given suffix
    pub fn ends_with(&self, suffix: &str) -> bool {
        self.chars.ends_with(suffix)
    }

    /// Check if sequence contains a substring
    pub fn contains(&self, pattern: &str) -> bool {
        self.chars.contains(pattern)
    }

    /// Convert to lowercase
    pub fn to_lowercase(&self) -> Self {
        Self {
            chars: self.chars.to_lowercase(),
        }
    }

    /// Convert to uppercase
    pub fn to_uppercase(&self) -> Self {
        Self {
            chars: self.chars.to_uppercase(),
        }
    }

    /// Iterate over characters
    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.chars.chars()
    }

    /// Convert into owned String
    pub fn into_string(self) -> String {
        self.chars
    }

    /// Check if all characters satisfy a predicate
    pub fn all<F>(&self, f: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        self.chars.chars().all(f)
    }

    /// Check if any character satisfies a predicate
    pub fn any<F>(&self, f: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        self.chars.chars().any(f)
    }
}

impl Default for CharSequence {
    fn default() -> Self {
        Self::empty()
    }
}

impl From<String> for CharSequence {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for CharSequence {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<char> for CharSequence {
    fn from(ch: char) -> Self {
        Self::from_char(ch)
    }
}

impl Deref for CharSequence {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.chars
    }
}

impl fmt::Display for CharSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.chars)
    }
}

impl AsRef<str> for CharSequence {
    fn as_ref(&self) -> &str {
        &self.chars
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_sequence_creation() {
        let seq = CharSequence::new("hello");
        assert_eq!(seq.as_str(), "hello");
        assert_eq!(seq.len(), 5);
        assert!(!seq.is_empty());
    }

    #[test]
    fn test_char_sequence_empty() {
        let seq = CharSequence::empty();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_char_sequence_from_char() {
        let seq = CharSequence::from_char('a');
        assert_eq!(seq.as_str(), "a");
        assert_eq!(seq.len(), 1);
    }

    #[test]
    fn test_char_sequence_char_at() {
        let seq = CharSequence::new("hello");
        assert_eq!(seq.char_at(0), Some('h'));
        assert_eq!(seq.char_at(4), Some('o'));
        assert_eq!(seq.char_at(5), None);
    }

    #[test]
    fn test_char_sequence_first_last() {
        let seq = CharSequence::new("hello");
        assert_eq!(seq.first(), Some('h'));
        assert_eq!(seq.last(), Some('o'));

        let empty = CharSequence::empty();
        assert_eq!(empty.first(), None);
        assert_eq!(empty.last(), None);
    }

    #[test]
    fn test_char_sequence_push() {
        let seq = CharSequence::new("hell");
        let new_seq = seq.push('o');
        assert_eq!(seq.as_str(), "hell"); // Original unchanged
        assert_eq!(new_seq.as_str(), "hello");
    }

    #[test]
    fn test_char_sequence_pop() {
        let seq = CharSequence::new("hello");
        let new_seq = seq.pop().unwrap();
        assert_eq!(seq.as_str(), "hello"); // Original unchanged
        assert_eq!(new_seq.as_str(), "hell");

        let empty = CharSequence::empty();
        assert!(empty.pop().is_none());
    }

    #[test]
    fn test_char_sequence_concat() {
        let seq = CharSequence::new("hello");
        let new_seq = seq.concat(" world");
        assert_eq!(seq.as_str(), "hello"); // Original unchanged
        assert_eq!(new_seq.as_str(), "hello world");
    }

    #[test]
    fn test_char_sequence_substring() {
        let seq = CharSequence::new("hello");
        let sub = seq.substring(1, 4).unwrap();
        assert_eq!(sub.as_str(), "ell");

        assert!(seq.substring(5, 10).is_none()); // Out of bounds
        assert!(seq.substring(3, 2).is_none()); // Invalid range
    }

    #[test]
    fn test_char_sequence_starts_ends_contains() {
        let seq = CharSequence::new("hello world");
        assert!(seq.starts_with("hello"));
        assert!(seq.ends_with("world"));
        assert!(seq.contains("lo wo"));
        assert!(!seq.starts_with("world"));
    }

    #[test]
    fn test_char_sequence_case_conversion() {
        let seq = CharSequence::new("Hello");
        let lower = seq.to_lowercase();
        let upper = seq.to_uppercase();
        
        assert_eq!(seq.as_str(), "Hello"); // Original unchanged
        assert_eq!(lower.as_str(), "hello");
        assert_eq!(upper.as_str(), "HELLO");
    }

    #[test]
    fn test_char_sequence_predicates() {
        let seq = CharSequence::new("abc");
        assert!(seq.all(|c| c.is_alphabetic()));
        assert!(!seq.all(|c| c.is_numeric()));
        assert!(seq.any(|c| c == 'a'));
        assert!(!seq.any(|c| c.is_numeric()));
    }

    #[test]
    fn test_char_sequence_equality() {
        let seq1 = CharSequence::new("hello");
        let seq2 = CharSequence::new("hello");
        let seq3 = CharSequence::new("world");

        assert_eq!(seq1, seq2);
        assert_ne!(seq1, seq3);
    }

    #[test]
    fn test_char_sequence_display() {
        let seq = CharSequence::new("hello");
        assert_eq!(format!("{}", seq), "hello");
    }

    #[test]
    fn test_char_sequence_from_conversions() {
        let from_string = CharSequence::from(String::from("hello"));
        assert_eq!(from_string.as_str(), "hello");

        let from_str = CharSequence::from("hello");
        assert_eq!(from_str.as_str(), "hello");

        let from_char = CharSequence::from('a');
        assert_eq!(from_char.as_str(), "a");
    }

    #[test]
    fn test_char_sequence_vietnamese() {
        let seq = CharSequence::new("xin chào");
        assert_eq!(seq.len(), 8); // Including space
        assert!(seq.contains("chào"));
    }
}
