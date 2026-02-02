use goxviet_core::engine::Engine;

/// Test tone target selection for diphthongs
/// "nuawx" should produce "nữa" (tone on 'a'), not "nuẵ" (tone on 'ư')

#[test]
fn test_nuawx_becomes_nua_crosshatch() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_modern_tone(false);

    // Type: n-u-a-w-x → should become "nữa"
    engine.on_key(goxviet_core::data::keys::N, false, false);
    assert_eq!(engine.get_buffer(), "n");

    engine.on_key(goxviet_core::data::keys::U, false, false);
    assert_eq!(engine.get_buffer(), "nu");

    engine.on_key(goxviet_core::data::keys::A, false, false);
    assert_eq!(engine.get_buffer(), "nua");

    // Type 'w' - should apply horn to 'u' → "nưa"
    engine.on_key(goxviet_core::data::keys::W, false, false);
    assert_eq!(engine.get_buffer(), "nưa", "nuaw should produce nưa");

    // Type 'x' (nặng tone) - should apply to 'a' (main vowel), NOT 'ư'
    engine.on_key(goxviet_core::data::keys::X, false, false);
    assert_eq!(
        engine.get_buffer(),
        "nữa",
        "nuawx should produce nữa (tone on 'a'), not nuẵ (tone on 'ư')"
    );
}

#[test]
fn test_cuox_becomes_cuo_crosshatch() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_modern_tone(false);

    // Type: c-u-o-x → should become "cụo" (tone on 'u', which is main vowel in "uo")
    engine.on_key(goxviet_core::data::keys::C, false, false);
    engine.on_key(goxviet_core::data::keys::U, false, false);
    engine.on_key(goxviet_core::data::keys::O, false, false);
    assert_eq!(engine.get_buffer(), "cuo");

    // Type 'x' (ngã tone) - should apply to 'o' (default pos for uo)
    engine.on_key(goxviet_core::data::keys::X, false, false);
    assert_eq!(
        engine.get_buffer(),
        "cuõ",
        "cuox should produce cuõ (tone on 'o')"
    );
}
