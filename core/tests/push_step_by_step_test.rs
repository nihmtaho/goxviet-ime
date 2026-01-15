use goxviet_core::data::keys;
use goxviet_core::engine::Engine;

#[test]
fn test_push_step_by_step() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    println!("\n=== Testing PUSH ===");

    let r1 = e.on_key_ext(keys::P, false, false, false);
    println!(
        "After P: buf='{}', action={}, bs={}, count={}",
        e.get_buffer(),
        r1.action,
        r1.backspace,
        r1.count
    );

    let r2 = e.on_key_ext(keys::U, false, false, false);
    println!(
        "After U: buf='{}', action={}, bs={}, count={}",
        e.get_buffer(),
        r2.action,
        r2.backspace,
        r2.count
    );

    let r3 = e.on_key_ext(keys::S, false, false, false);
    println!(
        "After S: buf='{}', action={}, bs={}, count={}",
        e.get_buffer(),
        r3.action,
        r3.backspace,
        r3.count
    );
    println!("  ^ S should apply sắc tone to U");

    let r4 = e.on_key_ext(keys::H, false, false, false);
    println!(
        "After H: buf='{}', action={}, bs={}, count={}",
        e.get_buffer(),
        r4.action,
        r4.backspace,
        r4.count
    );
    println!("  ^ H should trigger auto-restore to 'push'");

    assert_eq!(e.get_buffer(), "push", "Should auto-restore to push");
    assert_eq!(r4.action, 1, "Should send restore action");
    assert_eq!(r4.backspace, 2, "Should backspace 'pú' (2 chars)");
    assert_eq!(r4.count, 4, "Should send 'push' (4 chars)");
}
