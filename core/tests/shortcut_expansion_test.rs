//! Additional shortcut FFI tests
//!
//! LEGACY v1 FFI - DISABLED in v3.0
//! These tests used ime_init/ime_add_shortcut/etc. which were removed.
//! Shortcut functionality is now tested via v2 API in:
//! - application/use_cases/manage_shortcuts.rs (unit tests)
//! - presentation/ffi/api.rs (FFI v2 tests)
//! - platforms/macos/goxvietTests/ (integration tests)

#[ignore = "Legacy v1 FFI removed in v3.0"]
#[test]
fn placeholder_test() {
    // This file contained legacy v1 FFI tests
    // Migrate to v2 API if needed
}

/*
ARCHIVED CONTENT - DO NOT USE:

use goxviet_core::{
    ime_add_shortcut, ime_clear, ime_clear_shortcuts, ime_export_shortcuts_json, ime_free_string,
    ime_import_shortcuts_json, ime_init, ime_method, ime_set_shortcuts_enabled,
    ime_shortcuts_count,
};
use std::ffi::{CStr, CString};

[... rest of file archived ...]
*/
