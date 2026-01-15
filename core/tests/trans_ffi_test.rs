use goxviet_core::data::keys;
use goxviet_core::{ime_free, ime_get_buffer, ime_init, ime_key};
use serial_test::serial;
use std::ffi::CStr;
use std::os::raw::c_char;

#[test]
#[serial]
fn test_trans_via_ffi() {
    // Initialize engine via FFI (same as macOS app)
    ime_init();

    // Type 't'
    let r1 = unsafe { ime_key(keys::T, false, false) };
    assert!(!r1.is_null());
    unsafe { ime_free(r1) };

    // Type 'r'
    let r2 = unsafe { ime_key(keys::R, false, false) };
    assert!(!r2.is_null());
    unsafe { ime_free(r2) };

    // Type 'a'
    let r3 = unsafe { ime_key(keys::A, false, false) };
    assert!(!r3.is_null());
    unsafe { ime_free(r3) };

    // Type 'n'
    let r4 = unsafe { ime_key(keys::N, false, false) };
    assert!(!r4.is_null());
    unsafe { ime_free(r4) };

    // Type 's' - should trigger English detection
    let r5 = unsafe { ime_key(keys::S, false, false) };
    assert!(!r5.is_null());
    unsafe { ime_free(r5) };

    // Get buffer state after 's'
    let buffer_str = unsafe {
        let ptr = ime_get_buffer();
        CStr::from_ptr(ptr as *const c_char).to_str().unwrap()
    };
    println!("After 's': buffer='{}'", buffer_str);
    assert_eq!(buffer_str, "trans", "After 's', buffer should be 'trans'");

    // Type 'f' - should NOT apply tone because is_english_word = true
    let r6 = unsafe { ime_key(keys::F, false, false) };
    assert!(!r6.is_null());
    unsafe {
        assert_eq!(
            (*r6).action,
            0,
            "After 'f' in 'transf', engine should return Action::None (pass through)."
        );
        ime_free(r6);
    };

    // Get final buffer state
    let buffer_str = unsafe {
        let ptr = ime_get_buffer();
        CStr::from_ptr(ptr as *const c_char).to_str().unwrap()
    };
    println!("After 'f': buffer='{}'", buffer_str);

    // CRITICAL ASSERTION
    assert_eq!(
        buffer_str, "transf",
        "FAILED: After 'f', buffer should be 'transf' (not 'tràns').\n\
         This means the FFI layer is not preserving is_english_word state!"
    );

    // PART 2: Test restore_word behavior
    unsafe {
        use std::ffi::CString;
        let word = CString::new("trans").unwrap();
        goxviet_core::ime_restore_word(word.as_ptr());
    }

    // Type 'f' after restoration
    let r7 = unsafe { ime_key(keys::F, false, false) };
    assert!(!r7.is_null());
    unsafe { ime_free(r7) };

    let buffer_str_after_restore = unsafe {
        let ptr = ime_get_buffer();
        CStr::from_ptr(ptr as *const c_char).to_str().unwrap()
    };
    println!(
        "After restore_word('trans') and typing 'f': buffer='{}'",
        buffer_str_after_restore
    );

    assert_eq!(
        buffer_str_after_restore, "transf",
        "FAILED: After restore_word('trans'), the engine forgot it was English!\n\
         This explains why the macOS app still has the regression."
    );
}
