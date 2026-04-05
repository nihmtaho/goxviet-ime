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

    assert_eq!(
        buffer, "ddd",
        "Expected 'ddd' (invalid-combo restore) but got '{}'",
        buffer
    );
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

#[test]
fn test_na_pac_open_only_vowel_cluster() {
    // Test NA-PAC validation: ơi (NA.5) is open-only and should reject consonants
    // Sequence: v + o + w (creates ơ) + i (creates ơi compound) + c (should restore to raw)
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    println!("\n=== Test NA-PAC: ơi (NA.5 open-only) + 'c' should restore to raw ===");

    // Type 'v' + 'o' + 'w' + 'i' to build 'vơi' (o+w = ơ, forms NA.5)
    e.on_key(keys::V, false, false);
    println!("After 'v': buffer='{}'", e.get_buffer());

    e.on_key(keys::O, false, false);
    println!("After 'o': buffer='{}'", e.get_buffer());

    e.on_key(keys::W, false, false);
    println!("After 'w': buffer='{}' (ơ created)", e.get_buffer());

    e.on_key(keys::I, false, false);
    println!("After 'i': buffer='{}' (ơi compound)", e.get_buffer());

    // Type 'c' - NA.5 (ơi) is open-only, should reject and restore to raw
    let result = e.on_key(keys::C, false, false);
    let buffer = e.get_buffer();
    println!(
        "After 'c': buffer='{}', backspace={}, count={}",
        buffer, result.backspace, result.count
    );

    // CRITICAL: Should restore to raw "vowic" (includes the 'w' used to create ơ) because ơi allows no coda
    assert_eq!(
        buffer, "vowic",
        "NA.5 (ơi) is open-only; 'vowic' should restore to raw, got '{}'",
        buffer
    );
}

#[test]
fn test_na_pac_valid_digraph_coda() {
    // Test NA-PAC validation: ươ (NA.2) allows PAC.1 (ch, ng, nh)
    // Sequence: u + o + w (creates ươ, NA.2) + n + g (creates 'ng' digraph, valid)
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    println!("\n=== Test NA-PAC: ươ (NA.2) + 'ng' should accept digraph ===");

    // Type 'u' + 'o' + 'w' to build 'ư' prefix + 'ô' = 'ươ' (NA.2)
    e.on_key(keys::U, false, false);
    println!("After 'u': buffer='{}'", e.get_buffer());

    e.on_key(keys::O, false, false);
    println!("After 'o': buffer='{}'", e.get_buffer());

    e.on_key(keys::W, false, false);
    println!("After 'w': buffer='{}' (ươ compound)", e.get_buffer());

    e.on_key(keys::N, false, false);
    println!("After 'n': buffer='{}'", e.get_buffer());

    // Type 'g' to complete 'ng' digraph - should be valid for NA.2
    let result = e.on_key(keys::G, false, false);
    let buffer = e.get_buffer();
    println!(
        "After 'g': buffer='{}', backspace={}, count={}",
        buffer, result.backspace, result.count
    );

    // NA.2 (ươ) allows PAC.1 (ng), so 'ương' should be valid Vietnamese
    assert_eq!(
        buffer, "ương",
        "NA.2 (ươ) allows PAC.1 (ng); should produce 'ương', got '{}'",
        buffer
    );
}

#[test]
fn test_digraph_guard_with_vietnamese_transforms() {
    // Test that the just_completed_digraph guard prevents false English restoration
    // when Vietnamese tone transforms are active.
    //
    // Scenario: "rích" (rich with hỏi tone on í)
    // - 'r' (key for hỏi tone) on 'i' creates 'í' with Vietnamese transform
    // - 'c' extends buffer to "rích" (incomplete digraph, last char is consonant)
    // - 'h' completes the 'ch' digraph
    // - Without guard: instant_restore_english() might see "ríh" (3 chars) and falsely restore
    // - With guard: just_completed_digraph detects we just made "ch" digraph, skips restore

    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    println!("\n=== Test digraph guard: Vietnamese tone + digraph completion ===");

    // Type 'i' → 'i' (vowel)
    e.on_key(keys::I, false, false);
    println!("After 'i': buffer='{}'", e.get_buffer());

    // Type 'r' → 'ỉ' (r is grave/huyền tone modifier in Telex, transforms i to ỉ)
    let result = e.on_key(keys::R, false, false);
    println!(
        "After 'r': buffer='{}', has transform={}",
        e.get_buffer(),
        result.backspace > 0
    );
    assert_eq!(e.get_buffer(), "ỉ", "Expected 'ỉ' with huyền tone");

    // Type 'c' → 'íc' (now ends with consonant, incomplete coda)
    e.on_key(keys::C, false, false);
    println!("After 'c': buffer='{}'", e.get_buffer());

    // CRITICAL: Type 'h' to complete 'ch' digraph
    // Without digraph guard: instant_restore_english() might falsely restore
    // because the buffer looks like it could be English (vowel + consonant + 'h').
    // With guard: just_completed_digraph=true prevents the restore since we
    // just completed a Vietnamese digraph coda.
    let result = e.on_key(keys::H, false, false);
    let buffer = e.get_buffer();
    println!(
        "After 'h': buffer='{}', backspace={}, count={}",
        buffer, result.backspace, result.count
    );

    // Should produce "ỉch" (Vietnamese word with huyền tone + ch digraph)
    // NOT restore to raw "ich"
    assert_eq!(
        buffer, "ỉch",
        "Digraph 'ch' completion with Vietnamese tone should produce 'ỉch', not restore to 'ich', got '{}'",
        buffer
    );
    // Verify no raw restore happened
    assert!(
        result.count <= 1,
        "Should not restore to raw when Vietnamese tone + digraph is detected; count should be <= 1, got {}",
        result.count
    );
}
