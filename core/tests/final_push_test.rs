use goxviet_core::data::keys;
use goxviet_core::engine::Engine;

#[test]
fn test_push_final_verification() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    println!("\n========== PUSH AUTO-RESTORE TEST ==========");

    // P
    let r1 = e.on_key_ext(keys::P, false, false, false);
    println!(
        "1. P: action={}, bs={}, count={}, buf='{}'",
        r1.action,
        r1.backspace,
        r1.count,
        e.get_buffer()
    );
    assert_eq!(e.get_buffer(), "p");
    assert_eq!(r1.action, 0); // None

    // U
    let r2 = e.on_key_ext(keys::U, false, false, false);
    println!(
        "2. U: action={}, bs={}, count={}, buf='{}'",
        r2.action,
        r2.backspace,
        r2.count,
        e.get_buffer()
    );
    assert_eq!(e.get_buffer(), "pu");
    assert_eq!(r2.action, 0); // None

    // S (should apply sắc to U)
    let r3 = e.on_key_ext(keys::S, false, false, false);
    println!(
        "3. S: action={}, bs={}, count={}, buf='{}'",
        r3.action,
        r3.backspace,
        r3.count,
        e.get_buffer()
    );
    assert_eq!(e.get_buffer(), "pú");
    assert_eq!(r3.action, 1); // Send
    assert_eq!(r3.backspace, 1); // Delete 'u'
    assert_eq!(r3.count, 1); // Send 'ú'

    // H (should trigger auto-restore)
    let r4 = e.on_key_ext(keys::H, false, false, false);
    println!(
        "4. H: action={}, bs={}, count={}, buf='{}'",
        r4.action,
        r4.backspace,
        r4.count,
        e.get_buffer()
    );

    // Extract chars from result
    let chars: Vec<char> = (0..r4.count as usize)
        .filter_map(|i| {
            let codepoint = r4.as_slice()[i];
            char::from_u32(codepoint)
        })
        .collect();
    println!("   Result chars: {:?}", chars);

    assert_eq!(e.get_buffer(), "push", "Buffer should be 'push'");
    assert_eq!(r4.action, 1, "Action should be Send (1)");
    assert_eq!(r4.backspace, 2, "Should delete 2 chars ('pú')");
    assert_eq!(r4.count, 4, "Should send 4 chars ('push')");
    assert_eq!(chars, vec!['p', 'u', 's', 'h'], "Chars should be 'push'");

    println!("========== TEST PASSED ==========\n");
}
