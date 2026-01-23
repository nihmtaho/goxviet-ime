use goxviet_core::engine::Engine;
use goxviet_core::data::keys;

#[test]
fn test_console_not_transformed() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    // Type: c-o-n-s-o-l-e
    // Note: 's' after 'o' in Telex normally applies sắc tone (o → ó)
    // But "console" is in the English dictionary, so it should be auto-restored
    e.on_key(keys::C, false, false);
    e.on_key(keys::O, false, false);
    e.on_key(keys::N, false, false);
    e.on_key(keys::S, false, false); // 's' might transform 'o' → 'ó' (creating 'cón')
    e.on_key(keys::O, false, false);
    e.on_key(keys::L, false, false);
    e.on_key(keys::E, false, false); // Auto-restore should fix it here

    let final_buffer = e.get_buffer();

    // Final check: should be exactly "console" (auto-restore should have fixed it)
    assert_eq!(final_buffer, "console", 
        "Expected 'console' but got '{}'", final_buffer);
}
