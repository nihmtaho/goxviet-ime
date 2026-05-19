//! FFI API Facade
//!
//! C-compatible API using out parameter pattern for cross-platform compatibility.
//! All functions use `catch_unwind` to prevent panics crossing FFI boundary.

use super::conversions::*;
use crate::domain::entities::key_event::KeyEvent;
use crate::presentation::di::Container;
use std::ffi::{c_void, CString};
use std::os::raw::{c_char, c_int};

// ============================================================================
// FFI API v2 - Out Parameter Pattern (Swift-Safe)
// ============================================================================

use crate::presentation::ffi::types::{
    FfiConfig_v2, FfiProcessResult_v2, FfiStatusCode, FfiVersionInfo,
};

/// Create engine with optional config (v2 API)
///
/// # Arguments
/// * `config` - Optional configuration (NULL for defaults)
///
/// # Returns
/// * Engine pointer on success
/// * NULL on failure
///
/// # Safety
/// Caller must call `ime_destroy_engine_v2()` to free
#[no_mangle]
pub extern "C" fn ime_create_engine_v2(config: *const FfiConfig_v2) -> *mut c_void {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    let result = catch_unwind(AssertUnwindSafe(|| {
        // Parse config or use default
        let container = if config.is_null() {
            Box::new(Container::new())
        } else {
            let ffi_config = unsafe { &*config };
            let engine_config = to_engine_config_v2(ffi_config);
            Box::new(Container::with_config(engine_config))
        };

        Box::into_raw(container) as *mut c_void
    }));

    match result {
        Ok(ptr) => ptr,
        Err(_) => std::ptr::null_mut(),
    }
}

/// Destroy engine (v2 API)
///
/// # Safety
/// Safe to pass NULL
#[no_mangle]
pub extern "C" fn ime_destroy_engine_v2(engine_ptr: *mut c_void) {
    if engine_ptr.is_null() {
        return;
    }

    let _ = std::panic::catch_unwind(|| unsafe {
        let _ = Box::from_raw(engine_ptr as *mut Container);
    });
}

/// Reset buffer state without destroying engine (preserves shortcuts and config)
///
/// Use this instead of destroy+recreate to clear the typing buffer.
///
/// # Safety
/// - `engine_ptr` must be valid Engine pointer from ime_create_engine_v2
#[no_mangle]
pub extern "C" fn ime_reset_buffer_v2(engine_ptr: *mut c_void) -> FfiStatusCode {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    if engine_ptr.is_null() {
        return FfiStatusCode::ErrorInvalidArgument;
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        let container = unsafe { &*(engine_ptr as *const Container) };
        let processor = container.processor_service();
        let mut locked = processor.lock().unwrap();
        locked.reset_buffer();
        FfiStatusCode::Success
    }));

    match result {
        Ok(status) => status,
        Err(_) => FfiStatusCode::ErrorUnknown,
    }
}

/// Reset all state including word history (preserves shortcuts and config)
///
/// Use this instead of destroy+recreate when cursor moves, app switches, etc.
///
/// # Safety
/// - `engine_ptr` must be valid Engine pointer from ime_create_engine_v2
#[no_mangle]
pub extern "C" fn ime_reset_all_v2(engine_ptr: *mut c_void) -> FfiStatusCode {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    if engine_ptr.is_null() {
        return FfiStatusCode::ErrorInvalidArgument;
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        let container = unsafe { &*(engine_ptr as *const Container) };
        let processor = container.processor_service();
        let mut locked = processor.lock().unwrap();
        locked.reset_all();
        FfiStatusCode::Success
    }));

    match result {
        Ok(status) => status,
        Err(_) => FfiStatusCode::ErrorUnknown,
    }
}

/// Process keystroke (v2 API - OUT PARAMETER)
///
/// # Arguments
/// * `engine_ptr` - Engine instance (must not be NULL)
/// * `key_char` - Character to process
/// * `out` - Output result (must not be NULL)
///
/// # Returns
/// * 0 (FFI_SUCCESS) on success
/// * <0 error code on failure
///
/// # Safety
/// - `engine_ptr` must be valid Engine pointer from ime_create_engine_v2
/// - `out` must be valid writable FfiProcessResult_v2 pointer
/// - Caller must free `out->text` with `ime_free_string_v2()`
#[no_mangle]
pub extern "C" fn ime_process_key_v2(
    engine_ptr: *mut c_void,
    key_char: c_char,
    out: *mut FfiProcessResult_v2,
) -> c_int {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    // Null checks
    if engine_ptr.is_null() {
        return FfiStatusCode::ErrorNullEngine.to_c_int();
    }
    if out.is_null() {
        return FfiStatusCode::ErrorNullOutput.to_c_int();
    }

    // Panic safety
    let result = catch_unwind(AssertUnwindSafe(|| {
        // Cast to Container
        let container = unsafe { &*(engine_ptr as *const Container) };

        // Convert ASCII char to macOS virtual keycode (legacy engine uses keycodes)
        let ascii = key_char as u8;
        let keycode = match crate::data::keys::from_ascii(ascii) {
            Some(kc) => kc,
            None => return FfiStatusCode::ErrorInvalidKey,
        };

        let key_event = KeyEvent::new(keycode, false, false, false, false);

        // Process through processor service (following v1 pattern)
        let processor = container.processor_service();
        let mut locked = processor.lock().unwrap();

        // Process key
        let transform_result = match locked.process_key(key_event) {
            Ok(result) => result,
            Err(_) => {
                return FfiStatusCode::ErrorProcessingFailed;
            }
        };

        // Convert to FFI result (v2)
        let ffi_result = to_ffi_process_result_v2(transform_result);

        // Write to out parameter
        unsafe {
            (*out).text = ffi_result.text;
            (*out).backspace_count = ffi_result.backspace_count;
            (*out).consumed = ffi_result.consumed;
        }

        FfiStatusCode::Success
    }));

    match result {
        Ok(status) => status.to_c_int(),
        Err(_) => FfiStatusCode::ErrorPanic.to_c_int(),
    }
}

/// Process a key with extended modifiers (v2 API)
///
/// Like `ime_process_key_v2` but also passes caps, shift, ctrl modifiers
/// to the engine. Required for Shift+Backspace (delete word), correct
/// letter casing, and modifier-aware processing.
///
/// # Arguments
/// * `engine_ptr` - Engine instance (must not be NULL)
/// * `key_char` - ASCII character code
/// * `caps` - CapsLock active (for letter case)
/// * `shift` - Shift key pressed (for Shift+Backspace, symbol input)
/// * `ctrl` - Ctrl/Cmd/Alt pressed (bypasses IME)
/// * `out` - Output result (must not be NULL)
///
/// # Safety
/// - `engine_ptr` must be valid Engine pointer from ime_create_engine_v2
/// - `out` must be valid writable FfiProcessResult_v2 pointer
/// - Caller must free `out->text` with `ime_free_string_v2()`
#[no_mangle]
pub extern "C" fn ime_process_key_ext_v2(
    engine_ptr: *mut c_void,
    key_char: c_char,
    caps: bool,
    shift: bool,
    ctrl: bool,
    out: *mut FfiProcessResult_v2,
) -> c_int {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    if engine_ptr.is_null() {
        return FfiStatusCode::ErrorNullEngine.to_c_int();
    }
    if out.is_null() {
        return FfiStatusCode::ErrorNullOutput.to_c_int();
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        let container = unsafe { &*(engine_ptr as *const Container) };

        let ascii = key_char as u8;
        let keycode = match crate::data::keys::from_ascii(ascii) {
            Some(kc) => kc,
            None => return FfiStatusCode::ErrorInvalidKey,
        };

        let key_event = KeyEvent::with_caps(keycode, caps, shift, ctrl, false, false);

        let processor = container.processor_service();
        let mut locked = processor.lock().unwrap();

        let (transform_result, key_consumed) = match locked.process_key_ext(key_event) {
            Ok(r) => r,
            Err(_) => return FfiStatusCode::ErrorProcessingFailed,
        };

        let ffi_result = to_ffi_process_result_v2(transform_result);

        unsafe {
            (*out).text = ffi_result.text;
            (*out).backspace_count = ffi_result.backspace_count;
            (*out).consumed = ffi_result.consumed;
            (*out).key_consumed = key_consumed;
        }

        FfiStatusCode::Success
    }));

    match result {
        Ok(status) => status.to_c_int(),
        Err(_) => FfiStatusCode::ErrorPanic.to_c_int(),
    }
}

/// Process a key event with an optional Unicode character (v2 API).
///
/// Used for Option-modified keys on macOS where keycode stays the same but
/// the Unicode character differs (e.g. Option+V → √).
///
/// # Arguments
/// * `engine_ptr` - Engine pointer from `ime_create_engine_v2`
/// * `key_char`   - ASCII keycode
/// * `caps`       - CapsLock active
/// * `shift`      - Shift pressed
/// * `ctrl`       - Ctrl/Cmd/Alt pressed
/// * `char_code`  - Actual Unicode codepoint (0 = use keycode mapping only)
/// * `out`        - Output (must not be NULL); free `text` with `ime_free_string_v2`
///
/// # Returns
/// 0 on success, negative on error
///
/// # Safety
/// `engine_ptr` and `out` must be valid non-null pointers.
#[no_mangle]
pub extern "C" fn ime_key_with_char_v2(
    engine_ptr: *mut c_void,
    key_char: c_char,
    caps: bool,
    shift: bool,
    ctrl: bool,
    char_code: u32,
    out: *mut FfiProcessResult_v2,
) -> c_int {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    if engine_ptr.is_null() {
        return FfiStatusCode::ErrorNullEngine.to_c_int();
    }
    if out.is_null() {
        return FfiStatusCode::ErrorNullOutput.to_c_int();
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        let container = unsafe { &*(engine_ptr as *const Container) };

        let ascii = key_char as u8;
        let keycode = match crate::data::keys::from_ascii(ascii) {
            Some(kc) => kc,
            None => return FfiStatusCode::ErrorInvalidKey,
        };

        let ch = if char_code > 0 { char::from_u32(char_code) } else { None };
        let key_event = KeyEvent::with_caps(keycode, caps, shift, ctrl, false, false);

        let processor = container.processor_service();
        let mut locked = processor.lock().unwrap();

        let (transform_result, key_consumed) = match locked.process_key_with_char(key_event, ch) {
            Ok(r) => r,
            Err(_) => return FfiStatusCode::ErrorProcessingFailed,
        };

        let ffi_result = to_ffi_process_result_v2(transform_result);

        unsafe {
            (*out).text = ffi_result.text;
            (*out).backspace_count = ffi_result.backspace_count;
            (*out).consumed = ffi_result.consumed;
            (*out).key_consumed = key_consumed;
        }

        FfiStatusCode::Success
    }));

    match result {
        Ok(status) => status.to_c_int(),
        Err(_) => FfiStatusCode::ErrorPanic.to_c_int(),
    }
}

/// Get the full composed buffer as UTF-32 codepoints (v2 API).
///
/// Used by Select All+Replace injection where the full buffer is needed.
///
/// # Arguments
/// * `engine_ptr` - Engine pointer
/// * `out`        - Caller-allocated u32 buffer (UTF-32 codepoints)
/// * `max_len`    - Size of `out` (number of u32 elements)
///
/// # Returns
/// Number of codepoints written, or -1 on error.
///
/// # Safety
/// `out` must point to valid memory of at least `max_len * 4` bytes.
#[no_mangle]
pub unsafe extern "C" fn ime_get_buffer_v2(
    engine_ptr: *mut c_void,
    out: *mut u32,
    max_len: i64,
) -> i64 {
    if engine_ptr.is_null() || out.is_null() || max_len <= 0 {
        return -1;
    }

    let result = std::panic::catch_unwind(|| {
        let container = &*(engine_ptr as *const Container);
        let processor = container.processor_service();
        let locked = processor.lock().unwrap();
        let s = locked.get_buffer();
        let utf32: Vec<u32> = s.chars().map(|c| c as u32).collect();
        let len = utf32.len().min(max_len as usize);
        std::ptr::copy_nonoverlapping(utf32.as_ptr(), out, len);
        len as i64
    });

    result.unwrap_or(-1)
}

///
/// # Arguments
/// * `engine_ptr` - Engine instance (must not be NULL)
/// * `out` - Output config (must not be NULL)
///
/// # Returns
/// * 0 on success
/// * <0 on error
#[no_mangle]
pub extern "C" fn ime_get_config_v2(engine_ptr: *mut c_void, out: *mut FfiConfig_v2) -> c_int {
    // Null checks
    if engine_ptr.is_null() {
        return FfiStatusCode::ErrorNullEngine.to_c_int();
    }
    if out.is_null() {
        return FfiStatusCode::ErrorNullOutput.to_c_int();
    }

    let result = std::panic::catch_unwind(|| {
        let container = unsafe { &*(engine_ptr as *const Container) };

        // Get EngineConfig and convert directly to FfiConfig_v2
        let engine_config = container.get_config();
        let ffi_config_v2 = from_engine_config_v2(&engine_config);

        unsafe {
            *out = ffi_config_v2;
        }

        FfiStatusCode::Success
    });

    match result {
        Ok(status) => status.to_c_int(),
        Err(_) => FfiStatusCode::ErrorPanic.to_c_int(),
    }
}

/// Set engine configuration (v2 API)
///
/// # Arguments
/// * `engine_ptr` - Engine instance (must not be NULL)
/// * `config` - New configuration (must not be NULL)
///
/// # Returns
/// * 0 on success
/// * <0 on error
#[no_mangle]
pub extern "C" fn ime_set_config_v2(engine_ptr: *mut c_void, config: *const FfiConfig_v2) -> c_int {
    // Null checks
    if engine_ptr.is_null() {
        return FfiStatusCode::ErrorNullEngine.to_c_int();
    }
    if config.is_null() {
        return FfiStatusCode::ErrorNullConfig.to_c_int();
    }

    let result = std::panic::catch_unwind(|| {
        let container = unsafe { &mut *(engine_ptr as *mut Container) };
        let ffi_config = unsafe { &*config };

        // Convert v2 config directly to EngineConfig
        let engine_config = to_engine_config_v2(ffi_config);
        container.update_config(engine_config);

        FfiStatusCode::Success
    });

    match result {
        Ok(status) => status.to_c_int(),
        Err(_) => FfiStatusCode::ErrorPanic.to_c_int(),
    }
}

/// Get version information (v2 API)
///
/// # Arguments
/// * `out` - Output version info (must not be NULL)
///
/// # Returns
/// * 0 on success
/// * <0 on error
#[no_mangle]
pub extern "C" fn ime_get_version_v2(out: *mut FfiVersionInfo) -> c_int {
    if out.is_null() {
        return FfiStatusCode::ErrorNullOutput.to_c_int();
    }

    unsafe {
        (*out).major = 2;
        (*out).minor = 0;
        (*out).patch = 0;
        (*out).api_version = 2;
    }

    FfiStatusCode::Success.to_c_int()
}

/// Free string allocated by Rust (v2 API)
///
/// # Safety
/// Safe to pass NULL
#[no_mangle]
pub extern "C" fn ime_free_string_v2(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = std::panic::catch_unwind(|| unsafe {
            let _ = CString::from_raw(ptr);
        });
    }
}

// ============================================================================
// Shortcut Management API (v2)
// ============================================================================

/// Add shortcut (v2 API)
///
/// # Arguments
/// * `engine` - Engine pointer
/// * `trigger` - Trigger text (UTF-8)
/// * `expansion` - Expansion text (UTF-8)
///
/// # Returns
/// * `FFI_STATUS_OK` on success
/// * `FFI_STATUS_INVALID_ARG` if NULL
/// * `FFI_STATUS_ALREADY_EXISTS` if trigger exists
///
/// # Safety
/// `trigger` and `expansion` must be valid UTF-8 C strings
#[no_mangle]
pub extern "C" fn ime_add_shortcut_v2(
    engine: *mut c_void,
    trigger: *const c_char,
    expansion: *const c_char,
) -> FfiStatusCode {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    if engine.is_null() || trigger.is_null() || expansion.is_null() {
        return FfiStatusCode::ErrorInvalidArgument;
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        let container = unsafe { &*(engine as *const Container) };

        let trigger_str = match unsafe { std::ffi::CStr::from_ptr(trigger).to_str() } {
            Ok(s) => s,
            Err(_) => return FfiStatusCode::ErrorInvalidArgument,
        };

        let expansion_str = match unsafe { std::ffi::CStr::from_ptr(expansion).to_str() } {
            Ok(s) => s,
            Err(_) => return FfiStatusCode::ErrorInvalidArgument,
        };

        let processor = container.processor_service();
        let mut locked = processor.lock().unwrap();
        if locked.add_shortcut(trigger_str, expansion_str) {
            FfiStatusCode::Success
        } else {
            FfiStatusCode::ErrorAlreadyExists
        }
    }));

    match result {
        Ok(status) => status,
        Err(_) => FfiStatusCode::ErrorUnknown,
    }
}

/// Remove shortcut (v2 API)
///
/// # Arguments
/// * `engine` - Engine pointer
/// * `trigger` - Trigger text (UTF-8)
///
/// # Returns
/// * `FFI_STATUS_OK` on success
/// * `FFI_STATUS_INVALID_ARG` if NULL
/// * `FFI_STATUS_NOT_FOUND` if trigger doesn't exist
///
/// # Safety
/// `trigger` must be valid UTF-8 C string
#[no_mangle]
pub extern "C" fn ime_remove_shortcut_v2(
    engine: *mut c_void,
    trigger: *const c_char,
) -> FfiStatusCode {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    if engine.is_null() || trigger.is_null() {
        return FfiStatusCode::ErrorInvalidArgument;
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        let container = unsafe { &*(engine as *const Container) };
        let trigger_str = match unsafe { std::ffi::CStr::from_ptr(trigger).to_str() } {
            Ok(s) => s,
            Err(_) => return FfiStatusCode::ErrorInvalidArgument,
        };

        let processor = container.processor_service();
        let mut locked = processor.lock().unwrap();
        if locked.remove_shortcut(trigger_str) {
            FfiStatusCode::Success
        } else {
            FfiStatusCode::ErrorNotFound
        }
    }));

    match result {
        Ok(status) => status,
        Err(_) => FfiStatusCode::ErrorUnknown,
    }
}

/// Clear all shortcuts (v2 API)
#[no_mangle]
pub extern "C" fn ime_clear_shortcuts_v2(engine: *mut c_void) -> FfiStatusCode {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    if engine.is_null() {
        return FfiStatusCode::ErrorInvalidArgument;
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        let container = unsafe { &*(engine as *const Container) };
        let processor = container.processor_service();
        let mut locked = processor.lock().unwrap();
        locked.clear_shortcuts();
        FfiStatusCode::Success
    }));

    match result {
        Ok(status) => status,
        Err(_) => FfiStatusCode::ErrorUnknown,
    }
}

/// Get shortcut count (v2 API)
#[no_mangle]
pub extern "C" fn ime_shortcuts_count_v2(engine: *mut c_void) -> c_int {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    if engine.is_null() {
        return 0;
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        let container = unsafe { &*(engine as *const Container) };
        let processor = container.processor_service();
        let locked = processor.lock().unwrap();
        locked.shortcuts_count() as c_int
    }));

    match result {
        Ok(count) => count,
        Err(_) => 0,
    }
}

/// Enable/disable shortcuts globally (v2 API)
#[no_mangle]
pub extern "C" fn ime_set_shortcuts_enabled_v2(
    engine: *mut c_void,
    enabled: bool,
) -> FfiStatusCode {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    if engine.is_null() {
        return FfiStatusCode::ErrorInvalidArgument;
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        let container = unsafe { &*(engine as *const Container) };
        let processor = container.processor_service();
        let mut locked = processor.lock().unwrap();
        locked.set_shortcuts_enabled(enabled);
        FfiStatusCode::Success
    }));

    match result {
        Ok(status) => status,
        Err(_) => FfiStatusCode::ErrorUnknown,
    }
}

/// Restore current buffer to raw ASCII input (undo all Vietnamese transforms)
/// Returns the raw ASCII text with backspace count to replace current display.
///
/// # Safety
/// - `engine_ptr` must be valid Engine pointer from ime_create_engine_v2
/// - `out` must be valid writable FfiProcessResult_v2 pointer
/// - Caller must free `out->text` with `ime_free_string_v2()`
#[no_mangle]
pub extern "C" fn ime_restore_to_raw_v2(
    engine_ptr: *mut c_void,
    out: *mut FfiProcessResult_v2,
) -> c_int {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    if engine_ptr.is_null() {
        return FfiStatusCode::ErrorNullEngine.to_c_int();
    }
    if out.is_null() {
        return FfiStatusCode::ErrorNullOutput.to_c_int();
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        let container = unsafe { &*(engine_ptr as *const Container) };
        let processor = container.processor_service();
        let mut locked = processor.lock().unwrap();
        let transform_result = locked.restore_to_raw();
        let ffi_result = to_ffi_process_result_v2(transform_result);

        unsafe {
            (*out).text = ffi_result.text;
            (*out).backspace_count = ffi_result.backspace_count;
            (*out).consumed = ffi_result.consumed;
        }

        FfiStatusCode::Success
    }));

    match result {
        Ok(status) => status.to_c_int(),
        Err(_) => FfiStatusCode::ErrorPanic.to_c_int(),
    }
}

// ============================================================================
// Input Method Config API (T6.2)
// ============================================================================

#[cfg(test)]
mod api_tests {
    use super::*;
    use crate::presentation::ffi::types::FfiProcessResult_v2;

    unsafe fn make_engine() -> *mut std::ffi::c_void {
        ime_create_engine_v2(std::ptr::null())
    }

    #[test]
    fn test_ime_key_with_char_v2_plain_char() {
        unsafe {
            let engine = make_engine();
            let mut out = FfiProcessResult_v2::default();
            let status = ime_key_with_char_v2(engine, b'a' as i8, false, false, false, 0, &mut out);
            assert_eq!(status, 0, "should succeed");
            assert!(!out.key_consumed, "plain letter should not consume key");
            ime_destroy_engine_v2(engine);
        }
    }

    #[test]
    fn test_ime_get_buffer_v2_returns_zero_on_empty() {
        unsafe {
            let engine = make_engine();
            let mut buf = [0u32; 64];
            let count = ime_get_buffer_v2(engine, buf.as_mut_ptr(), 64);
            assert_eq!(count, 0, "empty buffer should return 0 codepoints");
            ime_destroy_engine_v2(engine);
        }
    }

    #[test]
    fn test_ime_process_key_ext_v2_key_consumed_false_for_letter() {
        unsafe {
            let engine = make_engine();
            let mut out = FfiProcessResult_v2::default();
            // 'a' key
            let status = ime_process_key_ext_v2(engine, b'a' as i8, false, false, false, &mut out);
            assert_eq!(status, 0);
            assert!(!out.key_consumed, "plain letter should not set key_consumed");
            ime_destroy_engine_v2(engine);
        }
    }
}

/// Load a data-driven InputMethodConfig from JSON (v2 API — Sprint D)
///
/// Accepts a JSON-encoded `InputMethodConfig` and updates the engine's
/// active input method. Based on KieuGo.ini pattern.
///
/// # JSON format
/// ```json
/// {"name":"telex","mappings":{"s":"tone_sac","f":"tone_huyen","dd":"stroke_d"}}
/// ```
///
/// # Safety
/// * `engine_ptr` must be valid and non-NULL
/// * `config_json` must point to at least `len` readable bytes
/// * Does NOT require NUL terminator — use raw pointer + length
#[no_mangle]
pub extern "C" fn ime_load_input_config_v2(
    engine_ptr: *mut c_void,
    config_json: *const u8,
    len: usize,
) -> FfiStatusCode {
    use crate::domain::entities::input_method_config::InputMethodConfig;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    if engine_ptr.is_null() || config_json.is_null() || len == 0 {
        return FfiStatusCode::ErrorInvalidArgument;
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: caller guarantees config_json points to `len` bytes
        let bytes = unsafe { std::slice::from_raw_parts(config_json, len) };

        let config = match InputMethodConfig::from_json_bytes(bytes) {
            Ok(c) => c,
            Err(_) => return FfiStatusCode::ErrorParseError,
        };

        let container = unsafe { &mut *(engine_ptr as *mut Container) };
        container.load_input_config(config);
        FfiStatusCode::Success
    }));

    match result {
        Ok(status) => status,
        Err(_) => FfiStatusCode::ErrorPanic,
    }
}
