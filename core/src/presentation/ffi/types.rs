//! FFI Type Definitions
//!
//! C-compatible types for cross-language FFI.
//! All types use `#[repr(C)]` for stable ABI.

use std::os::raw::{c_char, c_int};

/// FFI-safe result type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FfiResult {
    /// Success/failure status
    pub success: bool,
    /// Error code (0 = success)
    pub error_code: c_int,
}

impl FfiResult {
    /// Create success result
    pub const fn ok() -> Self {
        Self {
            success: true,
            error_code: 0,
        }
    }

    /// Create error result
    pub const fn err(code: c_int) -> Self {
        Self {
            success: false,
            error_code: code,
        }
    }
}

/// FFI-safe string pointer (UTF-8, null-terminated)
///
/// # Safety
/// - Caller must free using `ime_free_string()`
/// - Must not be null
/// - Must be valid UTF-8
pub type FfiString = *mut c_char;

/// FFI-safe const string pointer (UTF-8, null-terminated)
///
/// # Safety
/// - Must not be null
/// - Must be valid UTF-8
/// - Read-only, do not modify
pub type FfiConstString = *const c_char;

/// FFI-safe engine handle (opaque pointer)
///
/// # Safety
/// - Created by `ime_engine_new()`
/// - Must be freed by `ime_engine_free()`
pub type FfiEngineHandle = *mut std::ffi::c_void;

/// Input method type (Telex, VNI)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiInputMethod {
    /// Telex input method
    Telex = 0,
    /// VNI input method
    Vni = 1,
}

impl Default for FfiInputMethod {
    fn default() -> Self {
        Self::Telex
    }
}

/// Tone placement style
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiToneStyle {
    /// Old style (hòa, thủy)
    Old = 0,
    /// New style (hoà, thuỷ)
    New = 1,
}

impl Default for FfiToneStyle {
    fn default() -> Self {
        Self::New
    }
}

/// FFI-safe configuration struct
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FfiConfig {
    /// Input method
    pub input_method: FfiInputMethod,
    /// Tone placement style
    pub tone_style: FfiToneStyle,
    /// Enable smart mode
    pub smart_mode: bool,
    /// Enable shortcuts
    pub enable_shortcuts: bool,
}

impl Default for FfiConfig {
    fn default() -> Self {
        Self {
            input_method: FfiInputMethod::Telex,
            tone_style: FfiToneStyle::New,
            smart_mode: true,
            enable_shortcuts: true,
        }
    }
}

// ============================================================
// v2 ONLY - v1 FfiProcessResult removed in v3.0.0
// ============================================================
// Use FfiProcessResult_v2 instead (out parameter pattern for ABI safety)

// End of v1 types (legacy)

// ============================================================================
// FFI API v2 - Out Parameter Pattern (Fixes Swift ABI Issue)
// ============================================================================

/// FFI status codes for v2 API
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiStatusCode {
    /// Operation successful
    Success = 0,
    
    // Input errors
    /// Engine pointer is null
    ErrorNullEngine = -1,
    /// Output pointer is null
    ErrorNullOutput = -2,
    /// Config pointer is null
    ErrorNullConfig = -3,
    /// Invalid key character
    ErrorInvalidKey = -4,
    /// Invalid argument (generic)
    ErrorInvalidArgument = -5,
    
    // Processing errors
    /// Processing failed
    ErrorProcessingFailed = -10,
    /// Invalid UTF-8 encoding
    ErrorInvalidUtf8 = -11,
    
    // Shortcut errors
    /// Shortcut already exists
    ErrorAlreadyExists = -30,
    /// Shortcut not found
    ErrorNotFound = -31,
    
    // System errors
    /// Out of memory
    ErrorOutOfMemory = -20,
    /// Unknown/generic error
    ErrorUnknown = -98,
    /// Rust panic caught
    ErrorPanic = -99,
}

impl FfiStatusCode {
    /// Convert to C int
    pub const fn to_c_int(self) -> c_int {
        self as c_int
    }
    
    /// Check if success
    pub const fn is_success(self) -> bool {
        matches!(self, FfiStatusCode::Success)
    }
}

/// Process result for v2 API (OUT PARAMETER)
///
/// # Safety
/// This struct is written via out parameter to avoid struct-return ABI issues.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FfiProcessResult_v2 {
    /// Result text (UTF-8, null-terminated)
    /// Must be freed by caller using `ime_free_string_v2()`
    pub text: FfiString,
    /// Number of backspaces to perform
    pub backspace_count: u8,
    /// Whether the input was consumed
    pub consumed: bool,
}

impl Default for FfiProcessResult_v2 {
    fn default() -> Self {
        Self {
            text: std::ptr::null_mut(),
            backspace_count: 0,
            consumed: false,
        }
    }
}

/// Configuration for v2 API
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FfiConfig_v2 {
    /// Input method (Telex/VNI)
    pub input_method: FfiInputMethod,
    /// Tone style (Modern/Traditional)
    pub tone_style: FfiToneStyle,
    /// Enable smart mode
    pub smart_mode: bool,
    /// Enable instant auto-restore (restore English when Vietnamese is invalid)
    pub instant_restore_enabled: bool,
    /// Enable ESC key restore (restore original text on ESC)
    pub esc_restore_enabled: bool,
    /// Enable text expansion shortcuts
    pub enable_shortcuts: bool,
}

impl Default for FfiConfig_v2 {
    fn default() -> Self {
        Self {
            input_method: FfiInputMethod::Telex,
            tone_style: FfiToneStyle::New,
            smart_mode: true,
            instant_restore_enabled: true,
            esc_restore_enabled: false,
            enable_shortcuts: true,
        }
    }
}

/// Version information
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FfiVersionInfo {
    /// Major version
    pub major: u32,
    /// Minor version
    pub minor: u32,
    /// Patch version
    pub patch: u32,
    /// API version (2 for v2 API)
    pub api_version: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffi_result_ok() {
        let result = FfiResult::ok();
        assert!(result.success);
        assert_eq!(result.error_code, 0);
    }

    #[test]
    fn test_ffi_result_err() {
        let result = FfiResult::err(42);
        assert!(!result.success);
        assert_eq!(result.error_code, 42);
    }

    #[test]
    fn test_ffi_input_method_default() {
        let method = FfiInputMethod::default();
        assert_eq!(method, FfiInputMethod::Telex);
    }

    #[test]
    fn test_ffi_tone_style_default() {
        let style = FfiToneStyle::default();
        assert_eq!(style, FfiToneStyle::New);
    }

    #[test]
    fn test_ffi_config_default() {
        let config = FfiConfig::default();
        assert_eq!(config.input_method, FfiInputMethod::Telex);
        assert_eq!(config.tone_style, FfiToneStyle::New);
        assert!(config.smart_mode);
        assert!(config.enable_shortcuts);
    }

    #[test]
    fn test_ffi_process_result_v2_default() {
        let result = FfiProcessResult_v2::default();
        assert!(result.text.is_null());
        assert_eq!(result.backspace_count, 0);
        assert!(!result.consumed);
    }

    #[test]
    fn test_ffi_types_are_repr_c() {
        // Ensure all FFI types are properly aligned and sized
        // FfiResult has bool + c_int, with padding
        assert!(std::mem::size_of::<FfiResult>() >= std::mem::size_of::<c_int>());
        assert_eq!(std::mem::size_of::<FfiInputMethod>(), std::mem::size_of::<c_int>());
        assert_eq!(std::mem::size_of::<FfiToneStyle>(), std::mem::size_of::<c_int>());
    }
}
