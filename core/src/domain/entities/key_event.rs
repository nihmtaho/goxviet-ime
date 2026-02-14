//! Key Event Entity - User Input Representation
//!
//! Represents keyboard events and actions taken by the IME.

use std::fmt;

/// Key event representing user keyboard input
///
/// Captures a keyboard event with all relevant modifiers.
/// This is the primary input to the IME processing pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyEvent {
    /// The key code (Unicode value or virtual key code)
    pub keycode: u16,
    /// CapsLock is active (for uppercase letters)
    pub caps: bool,
    /// Shift key is pressed
    pub shift: bool,
    /// Control key is pressed
    pub ctrl: bool,
    /// Alt/Option key is pressed
    pub alt: bool,
    /// Command/Windows key is pressed (macOS/Windows specific)
    pub meta: bool,
}

impl KeyEvent {
    /// Create a new key event
    ///
    /// # Examples
    /// ```
    /// # use goxviet_core::domain::entities::key_event::KeyEvent;
    /// let event = KeyEvent::new(97, false, false, false, false); // 'a' key
    /// assert_eq!(event.keycode, 97);
    /// assert!(!event.shift);
    /// ```
    pub fn new(keycode: u16, shift: bool, ctrl: bool, alt: bool, meta: bool) -> Self {
        Self {
            keycode,
            caps: false,
            shift,
            ctrl,
            alt,
            meta,
        }
    }

    /// Create a key event with all modifiers including caps
    pub fn with_caps(keycode: u16, caps: bool, shift: bool, ctrl: bool, alt: bool, meta: bool) -> Self {
        Self {
            keycode,
            caps,
            shift,
            ctrl,
            alt,
            meta,
        }
    }

    /// Create a simple key event without modifiers
    ///
    /// # Examples
    /// ```
    /// # use goxviet_core::domain::entities::key_event::KeyEvent;
    /// let event = KeyEvent::simple(97); // 'a' key
    /// assert_eq!(event.keycode, 97);
    /// assert!(!event.has_modifiers());
    /// ```
    pub fn simple(keycode: u16) -> Self {
        Self {
            keycode,
            caps: false,
            shift: false,
            ctrl: false,
            alt: false,
            meta: false,
        }
    }

    /// Create key event with shift modifier
    pub fn with_shift(keycode: u16) -> Self {
        Self {
            keycode,
            caps: false,
            shift: true,
            ctrl: false,
            alt: false,
            meta: false,
        }
    }

    /// Create key event with control modifier
    pub fn with_ctrl(keycode: u16) -> Self {
        Self {
            keycode,
            caps: false,
            shift: false,
            ctrl: true,
            alt: false,
            meta: false,
        }
    }

    /// Check if any modifier keys are pressed
    #[inline]
    pub fn has_modifiers(&self) -> bool {
        self.shift || self.ctrl || self.alt || self.meta
    }

    /// Check if only shift is pressed (no other modifiers)
    #[inline]
    pub fn is_shift_only(&self) -> bool {
        self.shift && !self.ctrl && !self.alt && !self.meta
    }

    /// Check if control key is pressed (with or without shift)
    #[inline]
    pub fn is_ctrl_pressed(&self) -> bool {
        self.ctrl
    }

    /// Check if alt/option key is pressed
    #[inline]
    pub fn is_alt_pressed(&self) -> bool {
        self.alt
    }

    /// Get the character representation if this is a printable key
    ///
    /// Returns None for non-printable keys (function keys, arrows, etc.)
    pub fn as_char(&self) -> Option<char> {
        char::from_u32(self.keycode as u32)
    }

    /// Check if this is a letter key (a-z, A-Z)
    pub fn is_letter(&self) -> bool {
        matches!(self.keycode, 65..=90 | 97..=122)
    }

    /// Check if this is a digit key (0-9)
    pub fn is_digit(&self) -> bool {
        matches!(self.keycode, 48..=57)
    }

    /// Check if this is a whitespace key (space, tab)
    pub fn is_whitespace(&self) -> bool {
        matches!(self.keycode, 9 | 32) // Tab or Space
    }

    /// Check if this is backspace/delete key
    pub fn is_backspace(&self) -> bool {
        matches!(self.keycode, 8 | 127) // Backspace or Delete
    }

    /// Check if this is enter/return key
    pub fn is_enter(&self) -> bool {
        matches!(self.keycode, 10 | 13) // LF or CR
    }

    /// Check if this is escape key
    pub fn is_escape(&self) -> bool {
        self.keycode == 27
    }

    /// Check if this is a printable character (not a control key)
    pub fn is_printable(&self) -> bool {
        self.keycode >= 32 && self.keycode < 127
    }
}

impl Default for KeyEvent {
    fn default() -> Self {
        Self::simple(0)
    }
}

impl fmt::Display for KeyEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut modifiers = Vec::new();
        if self.meta {
            modifiers.push("Meta");
        }
        if self.ctrl {
            modifiers.push("Ctrl");
        }
        if self.alt {
            modifiers.push("Alt");
        }
        if self.shift {
            modifiers.push("Shift");
        }

        if !modifiers.is_empty() {
            write!(f, "{}+", modifiers.join("+"))?;
        }

        if let Some(ch) = self.as_char() {
            if ch.is_ascii_graphic() {
                write!(f, "'{}'", ch)
            } else {
                write!(f, "U+{:04X}", self.keycode)
            }
        } else {
            write!(f, "Key({})", self.keycode)
        }
    }
}

/// Action to be taken by the IME
///
/// Represents the output action after processing a key event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    /// No action needed, pass through to OS
    None,
    /// Replace text: delete N characters and insert new text
    Replace {
        /// Number of characters to delete backwards
        backspace_count: u8,
    },
    /// Insert new text without deletion
    Insert,
    /// Clear current buffer/state
    Clear,
    /// Commit current buffer as-is
    Commit,
    /// Undo last transformation
    Undo,
}

impl Action {
    /// Check if this action modifies text
    #[inline]
    pub fn is_modifying(&self) -> bool {
        !matches!(self, Action::None)
    }

    /// Check if this action requires deletion
    #[inline]
    pub fn requires_deletion(&self) -> bool {
        matches!(self, Action::Replace { .. } | Action::Clear)
    }

    /// Get the number of backspaces needed
    #[inline]
    pub fn backspace_count(&self) -> u8 {
        match self {
            Action::Replace { backspace_count } => *backspace_count,
            _ => 0,
        }
    }
}

impl Default for Action {
    fn default() -> Self {
        Action::None
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::None => write!(f, "None"),
            Action::Replace { backspace_count } => {
                write!(f, "Replace(backspace={})", backspace_count)
            }
            Action::Insert => write!(f, "Insert"),
            Action::Clear => write!(f, "Clear"),
            Action::Commit => write!(f, "Commit"),
            Action::Undo => write!(f, "Undo"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_event_creation() {
        let event = KeyEvent::new(97, true, false, false, false);
        assert_eq!(event.keycode, 97);
        assert!(event.shift);
        assert!(!event.ctrl);
    }

    #[test]
    fn test_key_event_simple() {
        let event = KeyEvent::simple(97);
        assert_eq!(event.keycode, 97);
        assert!(!event.has_modifiers());
    }

    #[test]
    fn test_key_event_with_modifiers() {
        let shift = KeyEvent::with_shift(97);
        assert!(shift.shift);
        assert!(shift.is_shift_only());

        let ctrl = KeyEvent::with_ctrl(97);
        assert!(ctrl.ctrl);
        assert!(ctrl.is_ctrl_pressed());
    }

    #[test]
    fn test_key_event_has_modifiers() {
        let no_mods = KeyEvent::simple(97);
        assert!(!no_mods.has_modifiers());

        let with_shift = KeyEvent::with_shift(97);
        assert!(with_shift.has_modifiers());

        let with_ctrl = KeyEvent::with_ctrl(97);
        assert!(with_ctrl.has_modifiers());
    }

    #[test]
    fn test_key_event_as_char() {
        let event = KeyEvent::simple(97);
        assert_eq!(event.as_char(), Some('a'));

        let event = KeyEvent::simple(65);
        assert_eq!(event.as_char(), Some('A'));
    }

    #[test]
    fn test_key_event_is_letter() {
        assert!(KeyEvent::simple(97).is_letter()); // 'a'
        assert!(KeyEvent::simple(65).is_letter()); // 'A'
        assert!(!KeyEvent::simple(48).is_letter()); // '0'
    }

    #[test]
    fn test_key_event_is_digit() {
        assert!(KeyEvent::simple(48).is_digit()); // '0'
        assert!(KeyEvent::simple(57).is_digit()); // '9'
        assert!(!KeyEvent::simple(97).is_digit()); // 'a'
    }

    #[test]
    fn test_key_event_is_whitespace() {
        assert!(KeyEvent::simple(32).is_whitespace()); // Space
        assert!(KeyEvent::simple(9).is_whitespace()); // Tab
        assert!(!KeyEvent::simple(97).is_whitespace()); // 'a'
    }

    #[test]
    fn test_key_event_special_keys() {
        assert!(KeyEvent::simple(8).is_backspace());
        assert!(KeyEvent::simple(13).is_enter());
        assert!(KeyEvent::simple(27).is_escape());
    }

    #[test]
    fn test_action_is_modifying() {
        assert!(!Action::None.is_modifying());
        assert!(Action::Insert.is_modifying());
        assert!(Action::Replace { backspace_count: 1 }.is_modifying());
        assert!(Action::Clear.is_modifying());
    }

    #[test]
    fn test_action_requires_deletion() {
        assert!(!Action::None.requires_deletion());
        assert!(!Action::Insert.requires_deletion());
        assert!(Action::Replace { backspace_count: 1 }.requires_deletion());
        assert!(Action::Clear.requires_deletion());
    }

    #[test]
    fn test_action_backspace_count() {
        assert_eq!(Action::None.backspace_count(), 0);
        assert_eq!(Action::Insert.backspace_count(), 0);
        assert_eq!(Action::Replace { backspace_count: 5 }.backspace_count(), 5);
    }

    #[test]
    fn test_action_display() {
        assert_eq!(format!("{}", Action::None), "None");
        assert_eq!(format!("{}", Action::Insert), "Insert");
        assert_eq!(
            format!("{}", Action::Replace { backspace_count: 3 }),
            "Replace(backspace=3)"
        );
    }
}
