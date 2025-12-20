use vietnamese_ime_core::engine::Result;
use std::mem::{size_of, offset_of};

#[test]
fn test_result_layout() {
    println!("sizeof(Result) = {}", size_of::<Result>());
    println!("offsetof(action) = {}", offset_of!(Result, action));
    println!("offsetof(backspace) = {}", offset_of!(Result, backspace));
    println!("offsetof(count) = {}", offset_of!(Result, count));
}
