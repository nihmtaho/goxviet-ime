//! Vietnamese IME Core
//!
//! High-performance Vietnamese input method engine supporting Telex and VNI.
//!
//! # FFI Usage
//!
//! ```c
//! // Initialize once at app start
//! ime_init();
//! ime_method(0);  // 0=Telex, 1=VNI
//!
//! // Process each keystroke
//! ImeResult* r = ime_key(keycode, is_shift, is_ctrl);
//! if (r && r->action == 1) {
//!     // Send r->backspace deletes, then r->chars
//! }
//! ime_free(r);
//!
//! // Clean up on word boundary
//! ime_clear();
//! ```

pub mod data;
pub mod engine;
pub mod engine_v2;
pub mod input;
pub mod updater;
pub mod utils;

use engine::{Engine, Result};
use std::sync::Mutex;

// Global engine instance (thread-safe via Mutex)
static ENGINE: Mutex<Option<Engine>> = Mutex::new(None);

/// Lock the engine mutex, recovering from poisoned state if needed (for tests)
#[inline(always)]
fn lock_engine() -> std::sync::MutexGuard<'static, Option<Engine>> {
    ENGINE.lock().unwrap_or_else(|e| e.into_inner())
}

// ============================================================
// FFI Interface
// ============================================================

/// Initialize the IME engine.
///
/// Must be called exactly once before any other `ime_*` functions.
/// Thread-safe: uses internal mutex.
///
/// # Panics
/// Panics if mutex is poisoned (only if previous call panicked).
#[no_mangle]
pub extern "C" fn ime_init() {
    let mut guard = lock_engine();
    *guard = Some(Engine::new());
}

/// Process a key event and return the result.
///
/// # Arguments
/// * `key` - macOS virtual keycode (0-127 for standard keys)
/// * `caps` - true if CapsLock is pressed (for uppercase letters)
/// * `ctrl` - true if Cmd/Ctrl/Alt is pressed (bypasses IME)
///
/// # Returns
/// * Pointer to `Result` struct (caller must free with `ime_free`)
/// * `null` if engine not initialized
///
/// # Result struct
/// * `action`: 0=None (pass through), 1=Send (replace text), 2=Restore
/// * `backspace`: number of characters to delete
/// * `chars`: UTF-32 codepoints to insert
/// * `count`: number of valid chars
///
/// # Note
/// For VNI mode with Shift+number keys (to type @, #, $ etc.),
/// use `ime_key_ext` with the shift parameter.
#[no_mangle]
#[inline]
pub extern "C" fn ime_key(key: u16, caps: bool, ctrl: bool) -> *mut Result {
    let mut guard = lock_engine();
    match guard.as_mut() {
        Some(e) => {
            let r = e.on_key(key, caps, ctrl);
            Box::into_raw(Box::new(r))
        }
        None => std::ptr::null_mut(),
    }
}

/// Process a key event with extended parameters.
///
/// # Arguments
/// * `key` - macOS virtual keycode (0-127 for standard keys)
/// * `caps` - true if CapsLock is pressed (for uppercase letters)
/// * `ctrl` - true if Cmd/Ctrl/Alt is pressed (bypasses IME)
/// * `shift` - true if Shift key is pressed (for symbols like @, #, $)
///
/// # Returns
/// * Pointer to `Result` struct (caller must free with `ime_free`)
/// * `null` if engine not initialized
///
/// # VNI Shift+number behavior
/// In VNI mode, when `shift=true` and key is a number (0-9), the engine
/// will NOT apply VNI marks/tones. This allows typing symbols:
/// - Shift+2 → @ (not huyền mark)
/// - Shift+3 → # (not hỏi mark)
/// - etc.
#[no_mangle]
#[inline]
pub extern "C" fn ime_key_ext(key: u16, caps: bool, ctrl: bool, shift: bool) -> *mut Result {
    let mut guard = lock_engine();
    match guard.as_mut() {
        Some(e) => {
            let r = e.on_key_ext(key, caps, ctrl, shift);
            Box::into_raw(Box::new(r))
        }
        None => std::ptr::null_mut(),
    }
}

/// Set the input method.
///
/// # Arguments
/// * `method` - 0 for Telex, 1 for VNI
///
/// No-op if engine not initialized.
#[no_mangle]
#[inline]
pub extern "C" fn ime_method(method: u8) {
    if let Some(e) = lock_engine().as_mut() {
        e.set_method(method);
    }
}

/// Enable or disable the engine.
///
/// When disabled, `ime_key` returns action=0 (pass through).
/// No-op if engine not initialized.
#[no_mangle]
#[inline]
pub extern "C" fn ime_enabled(enabled: bool) {
    if let Some(e) = lock_engine().as_mut() {
        e.set_enabled(enabled);
    }
}

/// Set whether to skip w→ư shortcut in Telex mode.
///
/// When `skip` is true, typing 'w' at word start stays as 'w'
/// instead of converting to 'ư'.
/// No-op if engine not initialized.
#[no_mangle]
pub extern "C" fn ime_skip_w_shortcut(skip: bool) {
    let mut guard = lock_engine();
    if let Some(ref mut e) = *guard {
        e.set_skip_w_shortcut(skip);
    }
}

/// Set whether ESC key restores raw ASCII input.
///
/// When `enabled` is true (default), pressing ESC restores original keystrokes.
/// When `enabled` is false, ESC key is passed through without restoration.
/// No-op if engine not initialized.
#[no_mangle]
pub extern "C" fn ime_esc_restore(enabled: bool) {
    let mut guard = lock_engine();
    if let Some(ref mut e) = *guard {
        e.set_esc_restore(enabled);
    }
}

/// Set whether to enable free tone placement (skip validation).
///
/// When `enabled` is true, allows placing diacritics anywhere without
/// spelling validation (e.g., "Zìa" is allowed).
/// When `enabled` is false (default), validates Vietnamese spelling rules.
/// No-op if engine not initialized.
#[no_mangle]
pub extern "C" fn ime_free_tone(enabled: bool) {
    let mut guard = lock_engine();
    if let Some(ref mut e) = *guard {
        e.set_free_tone(enabled);
    }
}

/// Set whether to use modern orthography for tone placement.
///
/// When `modern` is true: hoà, thuý (tone on second vowel - new style)
/// When `modern` is false (default): hòa, thúy (tone on first vowel - traditional)
/// No-op if engine not initialized.
#[no_mangle]
pub extern "C" fn ime_modern(modern: bool) {
    let mut guard = lock_engine();
    if let Some(ref mut e) = *guard {
        e.set_modern_tone(modern);
    }
}

/// Set whether to enable instant auto-restore for English words.
///
/// When `enabled` is true (default), restores English words immediately upon detection.
/// When `enabled` is false, auto-restore is disabled.
/// No-op if engine not initialized.
#[no_mangle]
pub extern "C" fn ime_instant_restore(enabled: bool) {
    let mut guard = lock_engine();
    if let Some(ref mut e) = *guard {
        e.set_english_auto_restore(enabled);
    }
}

/// Get the current buffer as a C string.
///
/// # Safety
/// Returns a pointer to a static buffer. Do not free.
#[no_mangle]
pub unsafe extern "C" fn ime_get_buffer() -> *const std::os::raw::c_char {
    static mut BUFFER: [u8; 256] = [0; 256];
    let guard = lock_engine();
    if let Some(ref e) = *guard {
        let s = e.get_buffer();
        let c_str = std::ffi::CString::new(s).unwrap_or_default();
        let bytes = c_str.as_bytes_with_nul();
        let len = bytes.len().min(256);
        #[allow(static_mut_refs)]
        {
            for i in 0..len {
                let ptr = BUFFER.as_mut_ptr();
                *ptr.add(i) = bytes[i];
            }
            BUFFER.as_ptr() as *const std::os::raw::c_char
        }
    } else {
        std::ptr::null()
    }
}

/// Clear the input buffer.
///
/// Call on word boundaries (space, punctuation, mouse click, focus change).
/// No-op if engine not initialized.
#[no_mangle]
pub extern "C" fn ime_clear() {
    let mut guard = lock_engine();
    if let Some(ref mut e) = *guard {
        e.clear();
    }
}

/// Clear all state including word history.
///
/// Call when cursor position changes (mouse click, selection-delete, arrow keys).
/// This prevents restoring stale state from history after navigation.
/// No-op if engine not initialized.
#[no_mangle]
pub extern "C" fn ime_clear_all() {
    let mut guard = lock_engine();
    if let Some(ref mut e) = *guard {
        e.clear_all();
    }
}

/// Free a result pointer returned by `ime_key`.
///
/// # Safety
/// * `r` must be a pointer returned by `ime_key`, or null
/// * Must be called exactly once per non-null `ime_key` return
/// * Do not use `r` after calling this function
///
/// # Memory Management
/// This function:
/// 1. Reconstructs the Vec from raw parts (if chars is non-null)
/// 2. Drops the Vec (freeing heap memory)
/// 3. Drops the Box<Result> (freeing Result struct)
#[no_mangle]
pub unsafe extern "C" fn ime_free(r: *mut Result) {
    if r.is_null() {
        return;
    }

    // Take ownership of Result
    let result = Box::from_raw(r);

    // Reconstruct and drop Vec if chars were allocated
    if !result.chars.is_null() && result.capacity > 0 {
        // SAFETY: We created this Vec in Result::send()
        // Reconstruct Vec with exact same parameters
        let _ = Vec::from_raw_parts(result.chars, result.count as usize, result.capacity);
        // Vec drops here, freeing heap memory
    }
    // Box<Result> drops here, freeing Result struct
}

// ============================================================
// Shortcut FFI
// ============================================================

/// Add a shortcut to the engine.
///
/// # Arguments
/// * `trigger` - C string for trigger text
/// * `replacement` - C string for replacement text
///
/// # Returns
/// * `true` if shortcut was added successfully
/// * `false` if capacity limit reached or invalid input
///
/// # Safety
/// Both pointers must be valid null-terminated UTF-8 strings.
#[no_mangle]
pub unsafe extern "C" fn ime_add_shortcut(
    trigger: *const std::os::raw::c_char,
    replacement: *const std::os::raw::c_char,
) -> bool {
    if trigger.is_null() || replacement.is_null() {
        return false;
    }

    let trigger_str = match std::ffi::CStr::from_ptr(trigger).to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };
    let replacement_str = match std::ffi::CStr::from_ptr(replacement).to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    let mut guard = lock_engine();
    if let Some(ref mut e) = *guard {
        e.shortcuts_mut().add(engine::shortcut::Shortcut::new(
            trigger_str,
            replacement_str,
        ))
    } else {
        false
    }
}

/// Remove a shortcut from the engine.
///
/// # Arguments
/// * `trigger` - C string for trigger to remove
///
/// # Safety
/// Pointer must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn ime_remove_shortcut(trigger: *const std::os::raw::c_char) {
    if trigger.is_null() {
        return;
    }

    let trigger_str = match std::ffi::CStr::from_ptr(trigger).to_str() {
        Ok(s) => s,
        Err(_) => return,
    };

    let mut guard = lock_engine();
    if let Some(ref mut e) = *guard {
        e.shortcuts_mut().remove(trigger_str);
    }
}

/// Clear all shortcuts from the engine.
#[no_mangle]
pub extern "C" fn ime_clear_shortcuts() {
    let mut guard = lock_engine();
    if let Some(ref mut e) = *guard {
        e.shortcuts_mut().clear();
    }
}

/// Get current number of shortcuts.
///
/// # Returns
/// Number of shortcuts currently stored
#[no_mangle]
pub extern "C" fn ime_shortcuts_count() -> usize {
    let guard = lock_engine();
    if let Some(ref e) = *guard {
        e.shortcuts().len()
    } else {
        0
    }
}

/// Get maximum shortcuts capacity.
///
/// # Returns
/// Maximum number of shortcuts allowed
#[no_mangle]
pub extern "C" fn ime_shortcuts_capacity() -> usize {
    let guard = lock_engine();
    if let Some(ref e) = *guard {
        e.shortcuts().capacity()
    } else {
        0
    }
}

/// Check if shortcuts table is at capacity.
///
/// # Returns
/// `true` if at capacity, `false` otherwise
#[no_mangle]
pub extern "C" fn ime_shortcuts_is_at_capacity() -> bool {
    let guard = lock_engine();
    if let Some(ref e) = *guard {
        e.shortcuts().is_at_capacity()
    } else {
        false
    }
}

/// Export all shortcuts to JSON string.
///
/// # Returns
/// Pointer to JSON string (caller must free with `ime_free_string`)
/// Returns null if engine not initialized.
///
/// # Safety
/// Caller must free the returned string using `ime_free_string`.
#[no_mangle]
pub extern "C" fn ime_export_shortcuts_json() -> *mut std::os::raw::c_char {
    let guard = lock_engine();
    if let Some(ref e) = *guard {
        let json = e.shortcuts().to_json();
        match std::ffi::CString::new(json) {
            Ok(c_str) => c_str.into_raw(),
            Err(_) => std::ptr::null_mut(),
        }
    } else {
        std::ptr::null_mut()
    }
}

/// Import shortcuts from JSON string.
///
/// # Arguments
/// * `json` - C string containing JSON data
///
/// # Returns
/// Number of shortcuts imported, or -1 on error
///
/// # Safety
/// Pointer must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn ime_import_shortcuts_json(json: *const std::os::raw::c_char) -> i32 {
    if json.is_null() {
        return -1;
    }

    let json_str = match std::ffi::CStr::from_ptr(json).to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    let mut guard = lock_engine();
    if let Some(ref mut e) = *guard {
        match e.shortcuts_mut().from_json(json_str) {
            Ok(count) => count as i32,
            Err(_) => -1,
        }
    } else {
        -1
    }
}

/// Free a string allocated by `ime_export_shortcuts_json`.
///
/// # Safety
/// * `s` must be a pointer returned by `ime_export_shortcuts_json`, or null
/// * Must be called exactly once per non-null pointer
#[no_mangle]
pub unsafe extern "C" fn ime_free_string(s: *mut std::os::raw::c_char) {
    if !s.is_null() {
        // Reconstruct and drop CString
        let _ = std::ffi::CString::from_raw(s);
    }
}

/// Set whether shortcuts are enabled globally.
///
/// When disabled, shortcut expansion is skipped.
/// No-op if engine not initialized.
#[no_mangle]
pub extern "C" fn ime_set_shortcuts_enabled(enabled: bool) {
    let mut guard = lock_engine();
    if let Some(ref mut e) = *guard {
        e.shortcuts_enabled = enabled;
    }
}

// ============================================================
// Encoding FFI
// ============================================================

/// Global encoding converter (thread-safe via Mutex)
static ENCODING: std::sync::Mutex<crate::engine::features::encoding::EncodingConverter> =
    std::sync::Mutex::new(crate::engine::features::encoding::EncodingConverter::new_const());

/// Set the output encoding.
///
/// # Arguments
/// * `encoding` - Encoding type: 0=Unicode (default), 1=TCVN3, 2=VNI, 3=CP1258
///
/// Unicode is the default and requires no conversion.
/// TCVN3, VNI, and CP1258 are legacy Vietnamese encodings.
#[no_mangle]
pub extern "C" fn ime_set_encoding(encoding: u8) {
    use crate::engine::features::encoding::OutputEncoding;
    if let Ok(mut guard) = ENCODING.lock() {
        guard.set_encoding(OutputEncoding::from_u8(encoding));
    }
}

/// Get the current output encoding.
///
/// # Returns
/// Encoding type: 0=Unicode, 1=TCVN3, 2=VNI, 3=CP1258
#[no_mangle]
pub extern "C" fn ime_get_encoding() -> u8 {
    if let Ok(guard) = ENCODING.lock() {
        guard.encoding().to_u8()
    } else {
        0 // Default to Unicode on error
    }
}

/// Convert a Unicode string to the current encoding.
///
/// # Arguments
/// * `input` - UTF-8 string to convert
///
/// # Returns
/// Pointer to encoded bytes (caller must free with `ime_free_bytes`)
/// Returns null on error.
///
/// # Safety
/// Caller must free the returned buffer using `ime_free_bytes`.
#[no_mangle]
pub unsafe extern "C" fn ime_convert_encoding(input: *const std::os::raw::c_char) -> *mut u8 {
    if input.is_null() {
        return std::ptr::null_mut();
    }

    let input_str = match std::ffi::CStr::from_ptr(input).to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    if let Ok(guard) = ENCODING.lock() {
        let bytes = guard.convert_string(input_str);
        let mut boxed = bytes.into_boxed_slice();
        let ptr = boxed.as_mut_ptr();
        std::mem::forget(boxed);
        ptr
    } else {
        std::ptr::null_mut()
    }
}

/// Free bytes allocated by ime_convert_encoding.
///
/// # Safety
/// Must be called with pointer from ime_convert_encoding, or null.
#[no_mangle]
pub unsafe extern "C" fn ime_free_bytes(ptr: *mut u8, len: usize) {
    if !ptr.is_null() && len > 0 {
        let _ = Vec::from_raw_parts(ptr, len, len);
    }
}

// ============================================================
// Word Restore FFI
// ============================================================

/// Restore buffer from a Vietnamese word string.
///
/// Used when native app detects cursor at word boundary and user
/// wants to continue editing (e.g., backspace into previous word).
/// Parses Vietnamese characters back to buffer components.
///
/// # Arguments
/// * `word` - C string containing the Vietnamese word to restore
///
/// # Safety
/// Pointer must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn ime_restore_word(word: *const std::os::raw::c_char) {
    if word.is_null() {
        return;
    }
    let word_str = match std::ffi::CStr::from_ptr(word).to_str() {
        Ok(s) => s,
        Err(_) => return,
    };
    let mut guard = lock_engine();
    if let Some(ref mut e) = *guard {
        e.restore_word(word_str);
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::keys;
    use serial_test::serial;
    use std::ffi::CString;

    #[test]
    #[serial]
    fn test_ffi_flow() {
        ime_init();
        ime_method(0); // Telex

        // Type 'a' + 's' -> á
        let r1 = ime_key(keys::A, false, false);
        assert!(!r1.is_null());
        unsafe { ime_free(r1) };

        let r2 = ime_key(keys::S, false, false);
        assert!(!r2.is_null());
        unsafe {
            let result = &*r2;
            assert_eq!(result.action, 1);
            assert_eq!(result.count, 1);
            // Access heap-allocated chars via pointer
            assert_eq!(*result.chars.offset(0), 'á' as u32);
            ime_free(r2);
        }

        ime_clear();
    }

    #[test]
    #[serial]
    fn test_shortcut_ffi_add_and_clear() {
        ime_init();
        ime_clear_shortcuts(); // Clear any existing shortcuts
        ime_method(0); // Telex

        // Add a shortcut via FFI
        let trigger = CString::new("vn").unwrap();
        let replacement = CString::new("Việt Nam").unwrap();

        unsafe {
            ime_add_shortcut(trigger.as_ptr(), replacement.as_ptr());
        }

        // Verify shortcut was added by checking engine state
        let guard = lock_engine();
        if let Some(ref e) = *guard {
            assert_eq!(e.shortcuts().len(), 1);
        }
        drop(guard);

        // Clear all shortcuts
        ime_clear_shortcuts();

        // Verify shortcuts cleared
        let guard = lock_engine();
        if let Some(ref e) = *guard {
            assert_eq!(e.shortcuts().len(), 0);
        }
        drop(guard);

        ime_clear();
    }

    #[test]
    #[serial]
    fn test_shortcut_ffi_remove() {
        ime_init();
        ime_clear_shortcuts(); // Clear any existing shortcuts
        ime_method(0); // Telex

        // Add two shortcuts
        let trigger1 = CString::new("hn").unwrap();
        let replacement1 = CString::new("Hà Nội").unwrap();
        let trigger2 = CString::new("hcm").unwrap();
        let replacement2 = CString::new("Hồ Chí Minh").unwrap();

        unsafe {
            ime_add_shortcut(trigger1.as_ptr(), replacement1.as_ptr());
            ime_add_shortcut(trigger2.as_ptr(), replacement2.as_ptr());
        }

        // Verify both added
        let guard = lock_engine();
        if let Some(ref e) = *guard {
            assert_eq!(e.shortcuts().len(), 2);
        }
        drop(guard);

        // Remove one shortcut
        unsafe {
            ime_remove_shortcut(trigger1.as_ptr());
        }

        // Verify only one remains
        let guard = lock_engine();
        if let Some(ref e) = *guard {
            assert_eq!(e.shortcuts().len(), 1);
        }
        drop(guard);

        // Clean up
        ime_clear_shortcuts();
        ime_clear();
    }

    #[test]
    #[serial]
    fn test_shortcut_ffi_null_safety() {
        ime_init();

        // Should not crash with null pointers
        unsafe {
            ime_add_shortcut(std::ptr::null(), std::ptr::null());
            ime_remove_shortcut(std::ptr::null());
        }

        // Engine should still work
        let r = ime_key(keys::A, false, false);
        assert!(!r.is_null());
        unsafe { ime_free(r) };

        ime_clear();
    }

    #[test]
    #[serial]
    fn test_shortcut_ffi_unicode() {
        ime_init();
        ime_clear_shortcuts(); // Clear any existing shortcuts
        ime_method(0);

        // Test with Unicode in both trigger and replacement
        let trigger = CString::new("tphcm").unwrap();
        let replacement = CString::new("Thành phố Hồ Chí Minh").unwrap();

        unsafe {
            ime_add_shortcut(trigger.as_ptr(), replacement.as_ptr());
        }

        // Verify shortcut added with proper UTF-8 handling
        let guard = lock_engine();
        if let Some(ref e) = *guard {
            assert_eq!(e.shortcuts().len(), 1);
        }
        drop(guard);

        ime_clear_shortcuts();
        ime_clear();
    }

    #[test]
    #[serial]
    fn test_restore_word_ffi() {
        ime_init();
        ime_method(0); // Telex

        // Restore a Vietnamese word
        let word = CString::new("việt").unwrap();
        unsafe {
            ime_restore_word(word.as_ptr());
        }

        // Type 's' to add sắc mark - should change ệ to ế
        // Engine returns replacement for changed portion
        let r = ime_key(keys::S, false, false);
        assert!(!r.is_null());
        unsafe {
            assert_eq!((*r).action, 1, "Should send replacement");
            // Engine outputs the modified result
            assert!((*r).count > 0, "Should have output chars");
            ime_free(r);
        }

        ime_clear();
    }

    #[test]
    #[serial]
    fn test_restore_word_ffi_null_safety() {
        ime_init();

        // Should not crash with null pointer
        unsafe {
            ime_restore_word(std::ptr::null());
        }

        // Engine should still work
        let r = ime_key(keys::A, false, false);
        assert!(!r.is_null());
        unsafe { ime_free(r) };

        ime_clear();
    }
}
