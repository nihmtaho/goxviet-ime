//! Test for stroke modifier (dd→đ) issues

use goxviet_core::data::keys;
use goxviet_core::engine::Engine;

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

    // Type 'd' third time - invalid combo ("đd" not valid Vietnamese), should restore to raw "ddd"
    let result = e.on_key(keys::D, false, false);
    let buffer = e.get_buffer();
    println!(
        "After 3rd 'd': buffer='{}', backspace={}, count={}",
        buffer, result.backspace, result.count
    );

    assert_eq!(buffer, "ddd", "Expected 'ddd' (invalid-combo restore) but got '{}'", buffer);
    assert_eq!(
        result.backspace, 1,
        "Should have backspace=1 to replace 'đ' with 'ddd'"
    );
    assert_eq!(result.count, 3, "Should output 3 chars 'ddd'");
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

    // Type 'd' third time - invalid combo restore to raw "ddd"
    // CRITICAL: This should send backspace=1 (delete 'đ'), NOT backspace=2 (which would delete space+'đ')
    let result = e.on_key(keys::D, false, false);
    let buffer = e.get_buffer();
    println!(
        "After 3rd 'd': buffer='{}', backspace={}, count={}",
        buffer, result.backspace, result.count
    );

    // The buffer should show "ddd" (raw restore)
    assert_eq!(buffer, "ddd", "Buffer should be 'ddd', got '{}'", buffer);

    // CRITICAL FIX VERIFICATION: backspace should be 1, NOT 2
    // backspace=2 would delete the space before 'đ', which was the reported bug
    assert_eq!(
        result.backspace, 1,
        "Should backspace 1 char (đ only), not 2 (which would delete preceding space)"
    );
    assert_eq!(result.count, 3, "Should output 3 chars (ddd)");
}

#[test]
fn test_odd_english_word() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    println!("\n=== Test 'odd' English word ===");

    e.on_key(keys::O, false, false);
    e.on_key(keys::D, false, false);
    e.on_key(keys::D, false, false);
    let buffer = e.get_buffer();
    println!("After 'odd': buffer='{}'", buffer);

    assert_eq!(buffer, "odd", "Expected 'odd' but got '{}'", buffer);
}

#[test]
fn test_add_no_intermediate_stroke() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    println!("\n=== Test 'add' — second 'd' must not trigger stroke (backspace=0) ===");

    e.on_key(keys::A, false, false);
    e.on_key(keys::D, false, false);
    let result = e.on_key(keys::D, false, false);
    let buffer = e.get_buffer();
    println!(
        "After 'add': buffer='{}', backspace={}, count={}",
        buffer, result.backspace, result.count
    );

    assert_eq!(buffer, "add", "Expected 'add' but got '{}'", buffer);
    assert_eq!(
        result.backspace, 0,
        "Second 'd' in 'add' must not trigger a revert (backspace=0)"
    );
}

#[test]
fn test_four_d_presses() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    println!("\n=== Test 4 x 'd' presses: d→'d', dd→'đ', ddd→'ddd'(raw restore), dddd→'dddd' ===");
    for i in 1..=4 {
        let result = e.on_key(keys::D, false, false);
        println!(
            "Press {}: buffer='{}', backspace={}, count={}, action={}",
            i,
            e.get_buffer(),
            result.backspace,
            result.count,
            result.action
        );
    }

    let buf = e.get_buffer();
    assert_eq!(
        buf, "dddd",
        "4 d-presses should produce 'dddd', got '{}'",
        buf
    );
}

#[test]
fn test_assign_debug() {
    let mut e = Engine::new();
    e.set_method(0);
    e.set_enabled(true);

    println!("\n=== Test 'assign' (a,s,s,i,g,n) ===");
    for (i, &k) in [keys::A, keys::S, keys::S, keys::I, keys::G, keys::N]
        .iter()
        .enumerate()
    {
        let result = e.on_key(k, false, false);
        println!(
            "Key {}: buf='{}', bs={}, count={}",
            i + 1,
            e.get_buffer(),
            result.backspace,
            result.count
        );
    }
    println!("Final: '{}'", e.get_buffer());
    assert_eq!(
        e.get_buffer(),
        "assign",
        "Expected 'assign', got '{}'",
        e.get_buffer()
    );
}
