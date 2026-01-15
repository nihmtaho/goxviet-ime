use goxviet_core::data::keys;
use goxviet_core::engine::Engine;

#[test]
fn test_push_result() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    // Type "push" and check the actual Result
    let r1 = e.on_key_ext(keys::P, false, false, false);
    println!(
        "P: action={}, bs={}, count={}",
        r1.action, r1.backspace, r1.count
    );

    let r2 = e.on_key_ext(keys::U, false, false, false);
    println!(
        "U: action={}, bs={}, count={}",
        r2.action, r2.backspace, r2.count
    );

    let r3 = e.on_key_ext(keys::S, false, false, false);
    println!(
        "S: action={}, bs={}, count={}, buffer='{}'",
        r3.action,
        r3.backspace,
        r3.count,
        e.get_buffer()
    );

    let r4 = e.on_key_ext(keys::H, false, false, false);
    println!(
        "H: action={}, bs={}, count={}, buffer='{}'",
        r4.action,
        r4.backspace,
        r4.count,
        e.get_buffer()
    );

    // Check if final result has restore action
    // action: 0=None, 1=Send, 2=Restore
    assert_eq!(e.get_buffer(), "push");
}

#[test]
fn test_merge_result() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    let r1 = e.on_key_ext(keys::M, false, false, false);
    println!(
        "M: action={}, bs={}, count={}",
        r1.action, r1.backspace, r1.count
    );

    let r2 = e.on_key_ext(keys::E, false, false, false);
    println!(
        "E: action={}, bs={}, count={}",
        r2.action, r2.backspace, r2.count
    );

    let r3 = e.on_key_ext(keys::R, false, false, false);
    println!(
        "R: action={}, bs={}, count={}, buffer='{}'",
        r3.action,
        r3.backspace,
        r3.count,
        e.get_buffer()
    );

    let r4 = e.on_key_ext(keys::G, false, false, false);
    println!(
        "G: action={}, bs={}, count={}, buffer='{}'",
        r4.action,
        r4.backspace,
        r4.count,
        e.get_buffer()
    );

    let r5 = e.on_key_ext(keys::E, false, false, false);
    println!(
        "E: action={}, bs={}, count={}, buffer='{}'",
        r5.action,
        r5.backspace,
        r5.count,
        e.get_buffer()
    );

    assert_eq!(e.get_buffer(), "merge");
}
