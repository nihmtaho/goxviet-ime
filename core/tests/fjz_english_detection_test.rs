use goxviet_core::data::keys;
use goxviet_core::engine::Engine;
use goxviet_core::utils::{telex, vni};

#[test]
fn test_fjz_always_english() {
    // Test that words starting with F, J, Z are always detected as English
    // and never transformed to Vietnamese

    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);

    // Test "facebook"
    for &key in &[
        keys::F,
        keys::A,
        keys::C,
        keys::E,
        keys::B,
        keys::O,
        keys::O,
        keys::K,
    ] {
        engine.on_key_ext(key, false, false, false);
    }
    let output = engine.get_buffer();
    assert_eq!(
        output, "facebook",
        "Words starting with 'f' should remain English"
    );

    // Reset engine
    engine = Engine::new();
    engine.set_method(0);
    engine.set_enabled(true);

    // Test "java"
    for &key in &[keys::J, keys::A, keys::V, keys::A] {
        engine.on_key_ext(key, false, false, false);
    }
    let output = engine.get_buffer();
    assert_eq!(
        output, "java",
        "Words starting with 'j' should remain English"
    );

    // Reset engine
    engine = Engine::new();
    engine.set_method(0);
    engine.set_enabled(true);

    // Test "zoom"
    for &key in &[keys::Z, keys::O, keys::O, keys::M] {
        engine.on_key_ext(key, false, false, false);
    }
    let output = engine.get_buffer();
    assert_eq!(
        output, "zoom",
        "Words starting with 'z' should remain English"
    );
}

// ── Modifier keys must not transform F/J/Z-initial words ──────────────────────
// In Telex, f=huyền, j=nặng, w=horn, s=sắc, etc. These bypass the Layer 0
// English detection block (which requires !_is_modifier). The modifier guard
// catches them before the modifier pipeline runs.

#[test]
fn test_jow_stays_jow() {
    // "jow": j + o + w(horn) → must stay "jow", not transform to "jơ"
    telex(&[("jow", "jow")]);
}

#[test]
fn test_fow_stays_fow() {
    // "fow": f(huyền) + o + w(horn) → must stay "fow"
    telex(&[("fow", "fow")]);
}

#[test]
fn test_fos_stays_fos() {
    // "fos": f + o + s(sắc mark) → must stay "fos", 's' not applied as sắc
    telex(&[("fos", "fos")]);
}

#[test]
fn test_jow_vni_stays_jow7() {
    // VNI: j + o + 7(horn) → must stay "jo7" (7 is literal, not horn)
    vni(&[("jo7", "jo7")]);
}

#[test]
fn test_java_no_transform() {
    // "java": j + a + v + a → no Vietnamese transform
    telex(&[("java", "java")]);
}

#[test]
fn test_json_no_transform() {
    // "json": j + s(nặng attempt, fails on 'j') + o + n → stays "json"
    telex(&[("json", "json")]);
}

// ── W initial: never a Vietnamese consonant ───────────────────────────────────
// Words starting with 'w' are always English. Internal 'w' (like in "window")
// must not apply horn to the preceding vowel.

#[test]
fn test_window_stays_window() {
    // w + i + n + d + o + w(horn attempt on 'o') → must stay "window", not "windơ"
    telex(&[("window", "window")]);
}

#[test]
fn test_work_stays_work() {
    telex(&[("work", "work")]);
}

#[test]
fn test_wow_stays_wow() {
    // w + o + w(horn attempt) → "wow" not "wơ"
    telex(&[("wow", "wow")]);
}

#[test]
fn test_woo_stays_woo() {
    // w + o + o(circumflex attempt) → "woo" not "wô"
    telex(&[("woo", "woo")]);
}
