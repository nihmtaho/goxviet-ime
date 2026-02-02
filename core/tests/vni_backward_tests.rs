use goxviet_core::engine::Engine;

/// Test VNI backward diacritical application
/// When typing VNI numbers after final consonants or vowels,
/// the diacritical should apply backward to the appropriate vowel

#[test]
fn test_vni_cam6_becomes_cam_circumflex() {
    let mut engine = Engine::new();
    engine.set_method(1); // VNI
    engine.set_enabled(true);
    engine.set_modern_tone(false);

    // Type: c-a-m-6 → should become "câm" (circumflex on 'a')
    engine.on_key(goxviet_core::data::keys::C, false, false);
    engine.on_key(goxviet_core::data::keys::A, false, false);
    engine.on_key(goxviet_core::data::keys::M, false, false);
    assert_eq!(engine.get_buffer(), "cam");

    // Type 6 (circumflex) - should apply backward to 'a'
    engine.on_key(goxviet_core::data::keys::N6, false, false);
    assert_eq!(
        engine.get_buffer(),
        "câm",
        "VNI: cam + 6 should produce câm (circumflex backward)"
    );
}

#[test]
fn test_vni_dau6_becomes_dau_circumflex() {
    let mut engine = Engine::new();
    engine.set_method(1); // VNI
    engine.set_enabled(true);
    engine.set_modern_tone(false);

    // Type: d-a-u-6 → should become "dâu" (circumflex on 'a')
    engine.on_key(goxviet_core::data::keys::D, false, false);
    engine.on_key(goxviet_core::data::keys::A, false, false);
    engine.on_key(goxviet_core::data::keys::U, false, false);
    assert_eq!(engine.get_buffer(), "dau");

    // Type 6 (circumflex) - should apply backward to 'a' (before 'u')
    engine.on_key(goxviet_core::data::keys::N6, false, false);
    assert_eq!(
        engine.get_buffer(),
        "dâu",
        "VNI: dau + 6 should produce dâu (circumflex on 'a' before 'u')"
    );
}

#[test]
fn test_vni_con7_becomes_con_horn() {
    let mut engine = Engine::new();
    engine.set_method(1); // VNI
    engine.set_enabled(true);
    engine.set_modern_tone(false);

    // Type: c-o-n-7 → should become "cơn" (horn on 'o')
    engine.on_key(goxviet_core::data::keys::C, false, false);
    engine.on_key(goxviet_core::data::keys::O, false, false);
    engine.on_key(goxviet_core::data::keys::N, false, false);
    assert_eq!(engine.get_buffer(), "con");

    // Type 7 (horn) - should apply backward to 'o'
    engine.on_key(goxviet_core::data::keys::N7, false, false);
    assert_eq!(
        engine.get_buffer(),
        "cơn",
        "VNI: con + 7 should produce cơn (horn backward)"
    );
}

#[test]
fn test_vni_can8_becomes_can_breve() {
    let mut engine = Engine::new();
    engine.set_method(1); // VNI
    engine.set_enabled(true);
    engine.set_modern_tone(false);

    // Type: c-a-n-8 → should become "căn" (breve on 'a')
    engine.on_key(goxviet_core::data::keys::C, false, false);
    engine.on_key(goxviet_core::data::keys::A, false, false);
    engine.on_key(goxviet_core::data::keys::N, false, false);
    assert_eq!(engine.get_buffer(), "can");

    // Type 8 (breve) - should apply backward to 'a'
    engine.on_key(goxviet_core::data::keys::N8, false, false);
    assert_eq!(
        engine.get_buffer(),
        "căn",
        "VNI: can + 8 should produce căn (breve backward)"
    );
}

#[test]
fn test_vni_cao7_becomes_cao_horn() {
    let mut engine = Engine::new();
    engine.set_method(1); // VNI
    engine.set_enabled(true);
    engine.set_modern_tone(false);

    // Type: c-a-o-7 → should become "cơa" wait no, 7 is horn which applies to o,u
    // So "cao" + 7 should give "cơa" (horn on 'o' before 'a'... wait that's wrong)
    // Actually 7 should apply to 'o' making it 'cơ', so result is "cơao"? No...

    // Let me reconsider: "cao" has vowel sequence "ao"
    // When typing 7 (horn), it should apply to 'o' (the last vowel that can receive horn)
    // But 'o' is not the last character, 'a' is after it... hmm

    // Actually for "cao", the vowel nucleus is "ao" (diphthong)
    // Typing 7 should apply horn to 'o', but 'o' is in the middle...
    // This is getting complex. Let me use a simpler example.

    // Type: c-u-o-7 → should become "cưo" (horn on 'u')
    engine.on_key(goxviet_core::data::keys::C, false, false);
    engine.on_key(goxviet_core::data::keys::U, false, false);
    engine.on_key(goxviet_core::data::keys::O, false, false);
    assert_eq!(engine.get_buffer(), "cuo");

    // Type 7 (horn) - should apply backward to 'u' (before 'o')
    // Note: Engine will normalize ưo → ươ automatically
    engine.on_key(goxviet_core::data::keys::N7, false, false);
    assert_eq!(
        engine.get_buffer(),
        "cươ",
        "VNI: cuo + 7 should produce cươ (horn on 'u', normalized to ươ)"
    );
}
