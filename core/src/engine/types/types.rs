//! Core types for Vietnamese IME Engine
//!
//! This module contains the fundamental types used throughout the engine:
//! - `Action`: Result action type for FFI responses
//! - `Result`: FFI-compatible result struct for key processing
//! - `Transform`: Internal transform tracking for undo/revert operations
//!
//! These types are extracted from the main engine module for better organization
//! and to enable reuse across different engine components.

use crate::engine::buffer::MAX;

// ============================================================
// FFI Result Types
// ============================================================

/// Engine action result type
///
/// Indicates what action the platform layer should take after processing a key:
/// - `None`: Pass through the key (no IME processing)
/// - `Send`: Send replacement text (delete backspace chars, insert new chars)
/// - `Restore`: Restore raw ASCII input (undo Vietnamese transforms)
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    /// Pass through - don't intercept the key
    None = 0,
    /// Send replacement text to application
    Send = 1,
    /// Restore raw ASCII (legacy, now handled via Send)
    Restore = 2,
}

/// FFI-compatible result struct for key processing
///
/// This struct is returned by `ime_key()` and contains:
/// - `chars`: UTF-32 codepoints to insert (up to MAX characters)
/// - `action`: What action to take (None, Send, Restore)
/// - `backspace`: Number of characters to delete before inserting
/// - `count`: Number of valid characters in `chars` array
///
/// # Memory Layout
/// This struct uses `#[repr(C)]` for stable ABI across FFI boundary.
/// Total size: MAX*4 + 4 = 260 bytes (with MAX=64)
///
/// # Example Usage (C/Swift)
/// ```c
/// ImeResult* r = ime_key(keycode, caps, ctrl);
/// if (r && r->action == 1) {
///     // Delete r->backspace characters
///     for (int i = 0; i < r->backspace; i++) {
///         send_backspace();
///     }
///     // Insert r->chars
///     for (int i = 0; i < r->count; i++) {
///         send_unicode(r->chars[i]);
///     }
/// }
/// ime_free(r);
/// ```
#[repr(C)]
pub struct Result {
    /// UTF-32 codepoints to insert
    /// Only the first `count` elements are valid
    pub chars: [u32; MAX],
    /// Action type: 0=None, 1=Send, 2=Restore
    pub action: u8,
    /// Number of characters to delete (backspace count)
    pub backspace: u8,
    /// Number of valid characters in `chars` array
    pub count: u8,
    /// Padding for alignment (unused)
    pub _pad: u8,
}

impl Result {
    /// Create a "no action" result
    ///
    /// Returns a result that tells the platform layer to pass through
    /// the key without any IME processing.
    #[inline]
    pub fn none() -> Self {
        Self {
            chars: [0; MAX],
            action: Action::None as u8,
            backspace: 0,
            count: 0,
            _pad: 0,
        }
    }

    /// Create a "send" result with backspace count and replacement chars
    ///
    /// # Arguments
    /// * `backspace` - Number of characters to delete before inserting
    /// * `chars` - Characters to insert (will be truncated to MAX)
    ///
    /// # Example
    /// ```ignore
    /// // Replace 2 chars with "việt"
    /// let result = Result::send(2, &['v', 'i', 'ệ', 't']);
    /// ```
    #[inline]
    pub fn send(backspace: u8, chars: &[char]) -> Self {
        let mut result = Self {
            chars: [0; MAX],
            action: Action::Send as u8,
            backspace,
            count: chars.len().min(MAX) as u8,
            _pad: 0,
        };
        for (i, &c) in chars.iter().take(MAX).enumerate() {
            result.chars[i] = c as u32;
        }
        result
    }

    /// Create a result that only deletes characters (no insertion)
    ///
    /// # Arguments
    /// * `backspace` - Number of characters to delete
    #[inline]
    pub fn delete(backspace: u8) -> Self {
        Self {
            chars: [0; MAX],
            action: Action::Send as u8,
            backspace,
            count: 0,
            _pad: 0,
        }
    }

    /// Check if this result requires action (not a pass-through)
    #[inline]
    pub fn requires_action(&self) -> bool {
        self.action != Action::None as u8
    }

    /// Check if this is a "send" action
    #[inline]
    pub fn is_send(&self) -> bool {
        self.action == Action::Send as u8
    }
}

impl Default for Result {
    fn default() -> Self {
        Self::none()
    }
}

// ============================================================
// Internal Transform Tracking
// ============================================================

/// Transform type for revert/undo tracking
///
/// When a Vietnamese transformation is applied, we record what type it was
/// so that pressing the same key again can revert it. This enables the
/// "double-tap to undo" behavior in Vietnamese input methods.
///
/// # Examples
/// - Telex: `aa` → `â`, then `a` again → `a` (revert circumflex)
/// - VNI: `a1` → `á`, then `1` again → `a` (revert sắc mark)
/// - Telex: `dd` → `đ`, then `d` again → `d` (revert stroke)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Transform {
    /// Mark transformation (sắc, huyền, hỏi, ngã, nặng)
    /// Stores: (trigger_key, mark_value)
    Mark(u16, u8),

    /// Tone/diacritic transformation (circumflex, horn, breve)
    /// Stores: (trigger_key, tone_value)
    Tone(u16, u8),

    /// Stroke transformation (d → đ)
    /// Stores: trigger_key
    Stroke(u16),

    /// W as vowel ư transformation (Telex mode)
    /// Used for: `w` → `ư`, revert with `ww` → `w`
    WAsVowel,

    /// W shortcut was explicitly skipped
    /// Prevents re-transformation after user typed `ww` → `w`
    WShortcutSkipped,
}

impl Transform {
    /// Get the trigger key for this transform (if applicable)
    pub fn trigger_key(&self) -> Option<u16> {
        match self {
            Transform::Mark(key, _) => Some(*key),
            Transform::Tone(key, _) => Some(*key),
            Transform::Stroke(key) => Some(*key),
            Transform::WAsVowel => None,
            Transform::WShortcutSkipped => None,
        }
    }

    /// Check if this transform can be reverted by the given key
    pub fn can_revert_with(&self, key: u16) -> bool {
        match self {
            Transform::Mark(k, _) => *k == key,
            Transform::Tone(k, _) => *k == key,
            Transform::Stroke(k) => *k == key,
            Transform::WAsVowel => false, // Handled specially
            Transform::WShortcutSkipped => false,
        }
    }

    /// Check if this is a W-related transform
    pub fn is_w_transform(&self) -> bool {
        matches!(self, Transform::WAsVowel | Transform::WShortcutSkipped)
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_none() {
        let r = Result::none();
        assert_eq!(r.action, Action::None as u8);
        assert_eq!(r.backspace, 0);
        assert_eq!(r.count, 0);
        assert!(!r.requires_action());
    }

    #[test]
    fn test_result_send() {
        let r = Result::send(2, &['v', 'i', 'ệ', 't']);
        assert_eq!(r.action, Action::Send as u8);
        assert_eq!(r.backspace, 2);
        assert_eq!(r.count, 4);
        assert!(r.requires_action());
        assert!(r.is_send());
        assert_eq!(r.chars[0], 'v' as u32);
        assert_eq!(r.chars[1], 'i' as u32);
        assert_eq!(r.chars[2], 'ệ' as u32);
        assert_eq!(r.chars[3], 't' as u32);
    }

    #[test]
    fn test_result_delete() {
        let r = Result::delete(3);
        assert_eq!(r.action, Action::Send as u8);
        assert_eq!(r.backspace, 3);
        assert_eq!(r.count, 0);
    }

    #[test]
    fn test_result_default() {
        let r = Result::default();
        assert_eq!(r.action, Action::None as u8);
    }

    #[test]
    fn test_transform_trigger_key() {
        assert_eq!(Transform::Mark(1, 2).trigger_key(), Some(1));
        assert_eq!(Transform::Tone(3, 4).trigger_key(), Some(3));
        assert_eq!(Transform::Stroke(5).trigger_key(), Some(5));
        assert_eq!(Transform::WAsVowel.trigger_key(), None);
        assert_eq!(Transform::WShortcutSkipped.trigger_key(), None);
    }

    #[test]
    fn test_transform_can_revert() {
        let mark = Transform::Mark(1, 2);
        assert!(mark.can_revert_with(1));
        assert!(!mark.can_revert_with(2));

        let tone = Transform::Tone(3, 4);
        assert!(tone.can_revert_with(3));
        assert!(!tone.can_revert_with(4));

        assert!(!Transform::WAsVowel.can_revert_with(13)); // W key
    }

    #[test]
    fn test_transform_is_w() {
        assert!(Transform::WAsVowel.is_w_transform());
        assert!(Transform::WShortcutSkipped.is_w_transform());
        assert!(!Transform::Stroke(2).is_w_transform());
    }
}