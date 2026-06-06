//! Tests for buffer export and word restore FFI functions.

use std::ffi::CString;

#[test]
fn get_buffer_returns_zero_for_empty_buffer() {
    use goxviet_core::{ime_create_engine_v2, ime_destroy_engine_v2, ime_get_buffer_v2, FfiConfig_v2};

    let config = FfiConfig_v2::default();
    let engine = unsafe { ime_create_engine_v2(&config) };
    assert!(!engine.is_null());

    let mut buf = [0u32; 256];
    let count = unsafe { ime_get_buffer_v2(engine, buf.as_mut_ptr(), 256) };
    assert_eq!(count, 0, "Fresh engine should have empty buffer");

    unsafe { ime_destroy_engine_v2(engine) };
}

#[test]
fn get_buffer_null_engine_returns_error() {
    use goxviet_core::ime_get_buffer_v2;

    let mut buf = [0u32; 256];
    let result = unsafe { ime_get_buffer_v2(std::ptr::null_mut(), buf.as_mut_ptr(), 256) };
    assert_eq!(result, -1, "Null engine should return -1");
}

#[test]
fn get_buffer_null_out_returns_error() {
    use goxviet_core::{ime_create_engine_v2, ime_destroy_engine_v2, ime_get_buffer_v2, FfiConfig_v2};

    let config = FfiConfig_v2::default();
    let engine = unsafe { ime_create_engine_v2(&config) };
    assert!(!engine.is_null());

    let result = unsafe { ime_get_buffer_v2(engine, std::ptr::null_mut(), 256) };
    assert_eq!(result, -1, "Null out_buf should return -1");

    unsafe { ime_destroy_engine_v2(engine) };
}

#[test]
fn restore_word_accepts_null_engine_gracefully() {
    use goxviet_core::ime_restore_word_v2;

    let word = CString::new("xin").unwrap();
    let result = unsafe { ime_restore_word_v2(std::ptr::null_mut(), word.as_ptr()) };
    assert_ne!(result, 0, "Null engine should return error code");
}

#[test]
fn restore_word_then_get_buffer() {
    use goxviet_core::{
        ime_create_engine_v2, ime_destroy_engine_v2, ime_get_buffer_v2, ime_restore_word_v2,
        FfiConfig_v2,
    };

    let config = FfiConfig_v2::default();
    let engine = unsafe { ime_create_engine_v2(&config) };
    assert!(!engine.is_null());

    // Restore a simple ASCII word into the buffer
    let word = CString::new("xin").unwrap();
    let status = unsafe { ime_restore_word_v2(engine, word.as_ptr()) };
    assert_eq!(status, 0, "restore_word should succeed");

    // Buffer should now contain the restored chars
    let mut buf = [0u32; 256];
    let count = unsafe { ime_get_buffer_v2(engine, buf.as_mut_ptr(), 256) };
    assert!(count > 0, "Buffer should be non-empty after restore_word");

    unsafe { ime_destroy_engine_v2(engine) };
}

#[test]
fn restore_word_null_word_returns_error() {
    use goxviet_core::{ime_create_engine_v2, ime_destroy_engine_v2, ime_restore_word_v2, FfiConfig_v2};

    let config = FfiConfig_v2::default();
    let engine = unsafe { ime_create_engine_v2(&config) };
    assert!(!engine.is_null());

    let result = unsafe { ime_restore_word_v2(engine, std::ptr::null()) };
    assert_ne!(result, 0, "Null word pointer should return error");

    unsafe { ime_destroy_engine_v2(engine) };
}
