//! FFI API Facade
//!
//! C-compatible API using out parameter pattern for cross-platform compatibility.
//! All functions use `catch_unwind` to prevent panics crossing FFI boundary.

use super::conversions::*;
use super::types::*;
use crate::domain::entities::key_event::KeyEvent;
use crate::presentation::di::Container;
use std::ffi::{c_void, CString};
use std::os::raw::{c_char, c_int};
use std::panic;

// ============================================================================
// FFI API v2 - Out Parameter Pattern (Swift-Safe)
// ============================================================================

use crate::presentation::ffi::types::{
    FfiStatusCode, FfiProcessResult_v2, FfiConfig_v2, FfiVersionInfo
};

/// Catch panics and return default value
///
/// Prevents panics from crossing FFI boundary
fn catch_panic<F, R>(default: R, f: F) -> R
where
    F: FnOnce() -> R + panic::UnwindSafe,
{
    match panic::catch_unwind(f) {
        Ok(result) => result,
        Err(_) => default,
    }
}

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
    
    let _ = std::panic::catch_unwind(|| {
        unsafe {
            let _ = Box::from_raw(engine_ptr as *mut Container);
        }
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
        
        let transform_result = match locked.process_key(key_event) {
            Ok(result) => result,
            Err(_) => {
                return FfiStatusCode::ErrorProcessingFailed;
            }
        };
        
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
///
/// # Arguments
/// * `engine_ptr` - Engine instance (must not be NULL)
/// * `out` - Output config (must not be NULL)
///
/// # Returns
/// * 0 on success
/// * <0 on error
#[no_mangle]
pub extern "C" fn ime_get_config_v2(
    engine_ptr: *mut c_void,
    out: *mut FfiConfig_v2,
) -> c_int {
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
pub extern "C" fn ime_set_config_v2(
    engine_ptr: *mut c_void,
    config: *const FfiConfig_v2,
) -> c_int {
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
///
/// Used when user presses Double OPTION key to restore raw typing.
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
