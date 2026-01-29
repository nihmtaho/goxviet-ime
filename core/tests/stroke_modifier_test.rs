//! Test for stroke modifier (dd→đ) issues

use goxviet_core::engine::Engine;
use goxviet_core::data::keys;

#[test]
fn test_add_english_word() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    println!("\n=== Test 'add' English word ===");

    // Type 'a'
    e.on_key(keys::A, false, false);
    println!("After 'a': buffer='{}'", e.get_buffer());

    // Type 'd'
    e.on_key(keys::D, false, false);
    println!("After first 'd': buffer='{}'", e.get_buffer());

    // Type 'd' again - should NOT become 'đ' because context is "ad" + "d" = "add"
    e.on_key(keys::D, false, false);
    let buffer = e.get_buffer();
    println!("After second 'd': buffer='{}'", buffer);
    
    assert_eq!(buffer, "add", "Expected 'add' but got '{}'", buffer);
}

#[test]
fn test_dd_to_stroke() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    println!("\n=== Test 'dd' → 'đ' ===");

    // Type 'd'
    e.on_key(keys::D, false, false);
    println!("After first 'd': buffer='{}'", e.get_buffer());

    // Type 'd' again - should become 'đ'
    e.on_key(keys::D, false, false);
    let buffer = e.get_buffer();
    println!("After second 'd': buffer='{}'", buffer);
    
    assert_eq!(buffer, "đ", "Expected 'đ' but got '{}'", buffer);
}

#[test]
fn test_triple_d_toggle() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    println!("\n=== Test 'ddd' toggle back to 'dd' ===");

    // Type 'd'
    e.on_key(keys::D, false, false);
    println!("After 1st 'd': buffer='{}'", e.get_buffer());

    // Type 'd' again - should become 'đ'
    e.on_key(keys::D, false, false);
    println!("After 2nd 'd': buffer='{}'", e.get_buffer());
    assert_eq!(e.get_buffer(), "đ");

    // Type 'd' third time - should toggle back to 'dd'
    let result = e.on_key(keys::D, false, false);
    let buffer = e.get_buffer();
    println!("After 3rd 'd': buffer='{}', backspace={}, count={}", buffer, result.backspace, result.count);
    
    assert_eq!(buffer, "dd", "Expected 'dd' but got '{}'", buffer);
    assert_eq!(result.backspace, 1, "Should have backspace=1 to replace 'đ' with 'dd'");
    assert_eq!(result.count, 2, "Should output 2 chars 'dd'");
}

#[test]
fn test_ddd_with_space() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    println!("\n=== Test space + 'ddd' - should NOT delete preceding space ===");

    // Type space (commits any previous buffer and starts fresh)
    e.on_key(keys::SPACE, false, false);
    println!("After space: buffer='{}'", e.get_buffer());

    // Type 'd'
    e.on_key(keys::D, false, false);
    println!("After 1st 'd': buffer='{}'", e.get_buffer());

    // Type 'd' again - should become 'đ'
    e.on_key(keys::D, false, false);
    println!("After 2nd 'd': buffer='{}'", e.get_buffer());

    // Type 'd' third time - should toggle back to 'dd'
    // CRITICAL: This should send backspace=1 (delete 'đ'), NOT backspace=2 (which would delete space+'đ')
    let result = e.on_key(keys::D, false, false);
    let buffer = e.get_buffer();
    println!("After 3rd 'd': buffer='{}', backspace={}, count={}", 
        buffer, result.backspace, result.count);
    
    // The buffer should show "dd"
    assert_eq!(buffer, "dd", "Buffer should be 'dd', got '{}'", buffer);
    
    // CRITICAL FIX VERIFICATION: backspace should be 1, NOT 2
    // backspace=2 would delete the space before 'đ', which was the reported bug
    assert_eq!(result.backspace, 1, 
        "Should backspace 1 char (đ only), not 2 (which would delete preceding space)");
    assert_eq!(result.count, 2, "Should output 2 chars (dd)");
}
