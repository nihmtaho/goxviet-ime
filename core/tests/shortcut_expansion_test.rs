//! Additional shortcut FFI tests

use goxviet_core::{
    ime_add_shortcut, ime_clear, ime_clear_shortcuts, ime_export_shortcuts_json, ime_free_string,
    ime_import_shortcuts_json, ime_init, ime_method, ime_set_shortcuts_enabled,
    ime_shortcuts_count,
};
use std::ffi::{CStr, CString};

#[test]
fn test_export_import_json_ffi() {
    ime_init();
    ime_clear_shortcuts();
    ime_method(0); // Telex

    // Add shortcuts
    let trigger = CString::new("vn").unwrap();
    let replacement = CString::new("Việt Nam").unwrap();
    unsafe {
        ime_add_shortcut(trigger.as_ptr(), replacement.as_ptr());
    }

    let trigger2 = CString::new("hcm").unwrap();
    let replacement2 = CString::new("Hồ Chí Minh").unwrap();
    unsafe {
        ime_add_shortcut(trigger2.as_ptr(), replacement2.as_ptr());
    }

    assert_eq!(ime_shortcuts_count(), 2);

    // Export to JSON
    let json_ptr = ime_export_shortcuts_json();
    assert!(!json_ptr.is_null());

    let json_str = unsafe { CStr::from_ptr(json_ptr).to_str().unwrap().to_string() };
    assert!(json_str.contains("\"shortcuts\""));
    assert!(json_str.contains("\"vn\""));
    assert!(json_str.contains("Việt Nam"));

    // Clear and import back
    ime_clear_shortcuts();
    assert_eq!(ime_shortcuts_count(), 0);

    let json_c = CString::new(json_str.clone()).unwrap();
    let imported = unsafe { ime_import_shortcuts_json(json_c.as_ptr()) };
    assert_eq!(imported, 2);
    assert_eq!(ime_shortcuts_count(), 2);

    // Free JSON string
    unsafe {
        ime_free_string(json_ptr);
    }

    ime_clear_shortcuts();
    ime_clear();
}

#[test]
fn test_import_json_invalid() {
    ime_init();
    ime_clear_shortcuts();

    // Import invalid JSON
    let invalid = CString::new("not json").unwrap();
    let result = unsafe { ime_import_shortcuts_json(invalid.as_ptr()) };
    assert_eq!(result, -1);

    // Import null
    let result = unsafe { ime_import_shortcuts_json(std::ptr::null()) };
    assert_eq!(result, -1);

    ime_clear();
}

#[test]
fn test_set_shortcuts_enabled() {
    ime_init();
    ime_clear_shortcuts();

    // Add shortcuts
    let trigger = CString::new("test").unwrap();
    let replacement = CString::new("Test").unwrap();
    unsafe {
        ime_add_shortcut(trigger.as_ptr(), replacement.as_ptr());
    }

    // Disable all shortcuts
    ime_set_shortcuts_enabled(false);

    // Re-enable all shortcuts
    ime_set_shortcuts_enabled(true);

    ime_clear_shortcuts();
    ime_clear();
}
