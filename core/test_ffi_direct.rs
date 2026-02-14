extern crate goxviet_core;

use goxviet_core::presentation::ffi::api::*;
use goxviet_core::ime_free_string;
use std::ffi::CString;

fn main() {
    unsafe {
        // Create engine
        let handle = ime_engine_new();
        println!("Engine created: {:?}", handle);
        
        // Process 'a'
        let key_a = CString::new("a").unwrap();
        let result = ime_process_key(handle, key_a.as_ptr(), 0);
        
        println!("\n=== Process 'a' Result ===");
        println!("success: {}", result.result.success);
        println!("error_code: {}", result.result.error_code);
        println!("consumed: {}", result.consumed);
        println!("backspace_count: {}", result.backspace_count);
        println!("text ptr: {:?}", result.text);
        
        if !result.text.is_null() {
            let text = std::ffi::CStr::from_ptr(result.text);
            println!("text: {:?}", text.to_string_lossy());
            ime_free_string(result.text);
        }
        
        // Free engine
        ime_engine_free(handle);
        println!("\nEngine freed");
    }
}
