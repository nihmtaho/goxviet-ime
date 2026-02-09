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
/// - `chars`: Heap-allocated UTF-32 codepoints (up to MAX characters)
/// - `action`: What action to take (None, Send, Restore)
/// - `backspace`: Number of characters to delete before inserting
/// - `count`: Number of valid characters in `chars` array
///
/// # Memory Layout
/// This struct uses `#[repr(C)]` for stable ABI across FFI boundary.
/// Uses heap allocation to support large MAX values without stack overflow.
/// Total size: ~24 bytes (pointer + metadata)
///
/// # Memory Management
/// - `chars` is heap-allocated via Vec
/// - Caller MUST call `ime_free()` to avoid memory leaks
/// - `ime_free()` reconstructs Vec to properly free memory
///
/// # Example Usage (C/Swift)
/// ```c
/// ImeResult* r = ime_key(keycode, caps, ctrl);
/// if (r && r->action == 1) {
///     // Delete r->backspace characters
///     for (int i = 0; i < r->backspace; i++) {
///         send_backspace();
///     }
///     // Insert r->chars (pointer arithmetic works)
///     for (int i = 0; i < r->count; i++) {
///         send_unicode(r->chars[i]);
///     }
/// }
/// ime_free(r);  // CRITICAL: Must free to avoid leak
/// ```
#[repr(C)]
pub struct Result {
    /// Heap-allocated UTF-32 codepoints
    /// Null if action == None
    pub chars: *mut u32,
    /// Allocated capacity (for proper Vec reconstruction)
    pub capacity: usize,
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
    ///
    /// Uses null pointer since no chars are needed.
    #[inline]
    pub fn none() -> Self {
        Self {
            chars: std::ptr::null_mut(),
            capacity: 0,
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
    /// # Memory
    /// Allocates heap memory via Vec. Caller must call `ime_free()` to avoid leak.
    ///
    /// # Example
    /// ```ignore
    /// // Replace 2 chars with "việt"
    /// let result = Result::send(2, &['v', 'i', 'ệ', 't']);
    /// ```
    #[inline]
    pub fn send(backspace: u8, chars: &[char]) -> Self {
        let count = chars.len().min(MAX);

        if count == 0 {
            // No chars to send, use null pointer
            return Self {
                chars: std::ptr::null_mut(),
                capacity: 0,
                action: Action::Send as u8,
                backspace,
                count: 0,
                _pad: 0,
            };
        }

        // Allocate Vec on heap
        let mut vec: Vec<u32> = Vec::with_capacity(count);
        for &c in chars.iter().take(count) {
            vec.push(c as u32);
        }

        // Extract raw parts and forget Vec (we'll manage memory manually)
        let ptr = vec.as_mut_ptr();
        let capacity = vec.capacity();
        std::mem::forget(vec);

        Self {
            chars: ptr,
            capacity,
            action: Action::Send as u8,
            backspace,
            count: count as u8,
            _pad: 0,
        }
    }

    /// Create a result that only deletes characters (no insertion)
    ///
    /// # Arguments
    /// * `backspace` - Number of characters to delete
    ///
    /// Uses null pointer since no chars are inserted.
    #[inline]
    pub fn delete(backspace: u8) -> Self {
        Self {
            chars: std::ptr::null_mut(),
            capacity: 0,
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

    /// Get chars as a slice for iteration
    ///
    /// # Safety
    /// Returns a slice view of the heap-allocated chars array.
    /// Safe to use as long as Result is valid.
    #[inline]
    pub fn as_slice(&self) -> &[u32] {
        if self.chars.is_null() || self.count == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(self.chars, self.count as usize) }
        }
    }
}

impl Default for Result {
    fn default() -> Self {
        Self::none()
    }
}

impl Drop for Result {
    /// Automatically free heap-allocated memory when Result goes out of scope.
    ///
    /// This ensures memory is freed in Rust test utilities. When using FFI,
    /// the Result is boxed and returned as a pointer, so Drop is not called
    /// until ime_free() reconstructs and drops the Box.
    fn drop(&mut self) {
        if !self.chars.is_null() && self.capacity > 0 {
            // Reconstruct and drop the Vec to free heap memory
            unsafe {
                let _ = Vec::from_raw_parts(self.chars, self.count as usize, self.capacity);
            }
            // Vec is dropped here, freeing the memory
        }
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
        // Access heap-allocated chars via unsafe pointer
        unsafe {
            assert_eq!(*r.chars.offset(0), 'v' as u32);
            assert_eq!(*r.chars.offset(1), 'i' as u32);
            assert_eq!(*r.chars.offset(2), 'ệ' as u32);
            assert_eq!(*r.chars.offset(3), 't' as u32);
        }
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
