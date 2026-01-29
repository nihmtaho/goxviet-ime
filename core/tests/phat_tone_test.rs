use goxviet_core::data::keys;
use goxviet_core::engine::Engine;

#[test]
fn test_phat_tone() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    e.on_key(keys::P, false, false);
    e.on_key(keys::H, false, false);
    e.on_key(keys::A, false, false);
    e.on_key(keys::S, false, false); // Should apply acute tone (sắc)
    e.on_key(keys::T, false, false);

    let buffer = e.get_buffer();
    assert_eq!(
        buffer, "phát",
        "Expected 'phát' but got '{}'. Pattern: p + h + a + s + t",
        buffer
    );
}
