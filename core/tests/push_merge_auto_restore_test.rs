use goxviet_core::data::keys;
use goxviet_core::engine::Engine;

#[test]
fn test_auto_restore_push() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    // Type "push" - should be detected as English and auto-restored
    e.on_key_ext(keys::P, false, false, false);
    e.on_key_ext(keys::U, false, false, false);
    e.on_key_ext(keys::S, false, false, false);
    e.on_key_ext(keys::H, false, false, false);

    let output = e.get_buffer();
    assert_eq!(output, "push", "push should be auto-restored as English");
}

#[test]
fn test_auto_restore_merge() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    // Type "merge" - should be detected as English and auto-restored
    e.on_key_ext(keys::M, false, false, false);
    e.on_key_ext(keys::E, false, false, false);
    e.on_key_ext(keys::R, false, false, false);
    e.on_key_ext(keys::G, false, false, false);
    e.on_key_ext(keys::E, false, false, false);

    let output = e.get_buffer();
    assert_eq!(output, "merge", "merge should be auto-restored as English");
}
