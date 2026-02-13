use goxviet_core::data::keys;
use goxviet_core::engine::Engine;

#[test]
fn test_than_s_telex_gives_than_sac() {
    let mut engine = Engine::new();
    // Type "than" + "s" → expect "thán"
    engine.on_key(keys::T, false, false);
    engine.on_key(keys::H, false, false);
    engine.on_key(keys::A, false, false);
    engine.on_key(keys::N, false, false);
    engine.on_key(keys::S, false, false);
    let buf = engine.get_buffer();
    assert_eq!(buf, "thán", "Tone sắc should apply to 'a' after final consonant 'n'");
}

#[test]
fn test_moi_oo_telex_gives_moi_circumflex() {
    let mut engine = Engine::new();
    // Type "moi" + "o" → expect "môi" (circumflex on o)
    engine.on_key(keys::M, false, false);
    engine.on_key(keys::O, false, false);
    engine.on_key(keys::I, false, false);
    engine.on_key(keys::O, false, false);
    let buf = engine.get_buffer();
    assert_eq!(buf, "môi", "Circumflex should apply backward to 'o' in 'oi' diphthong");
}

#[test]
fn test_moiox_telex_gives_moi_nga() {
    let mut engine = Engine::new();
    // Type "moiox" → expect "mỗi"
    engine.on_key(keys::M, false, false);
    engine.on_key(keys::O, false, false);
    engine.on_key(keys::I, false, false);
    engine.on_key(keys::O, false, false);
    engine.on_key(keys::X, false, false);
    let buf = engine.get_buffer();
    assert_eq!(buf, "mỗi", "Should apply circumflex and ngã to get 'mỗi'");
}

#[test]
fn test_moi_6_4_vni_gives_moi_nga() {
    // VNI mode
    let mut engine = Engine::new();
    engine.set_method(1);
    // Type "moi" + "6" + "4" → expect "mỗi"
    engine.on_key(keys::M, false, false);
    engine.on_key(keys::O, false, false);
    engine.on_key(keys::I, false, false);
    engine.on_key(keys::N6, false, false);
    engine.on_key(keys::N4, false, false);
    let buf = engine.get_buffer();
    assert_eq!(buf, "mỗi", "VNI should apply circumflex and ngã to get 'mỗi'");
}

#[test]
fn test_khoe_o_should_not_apply_circumflex() {
    let mut engine = Engine::new();
    // Type "khoe" + "o" → should NOT become "khôe", 'o' should be appended
    engine.on_key(keys::K, false, false);
    engine.on_key(keys::H, false, false);
    engine.on_key(keys::O, false, false);
    engine.on_key(keys::E, false, false);
    engine.on_key(keys::O, false, false);
    let buf = engine.get_buffer();
    // "ôe" is NOT a valid Vietnamese diphthong, so circumflex should NOT apply backward
    assert_ne!(buf, "khôe", "Should NOT apply circumflex backward to form invalid 'ôe' diphthong");
}

#[test]
fn test_dau_a_telex_gives_dau_circumflex() {
    let mut engine = Engine::new();
    // Type "dau" + "a" → expect "dâu" (aa pattern, circumflex on a)
    engine.on_key(keys::D, false, false);
    engine.on_key(keys::A, false, false);
    engine.on_key(keys::U, false, false);
    engine.on_key(keys::A, false, false);
    let buf = engine.get_buffer();
    assert_eq!(buf, "dâu", "Circumflex should apply backward to 'a' in 'au' diphthong");
}

#[test]
fn test_cay_a_telex_gives_cay_circumflex() {
    let mut engine = Engine::new();
    // Type "cay" + "a" → expect "cây" (aa pattern, circumflex on a in ây)
    engine.on_key(keys::C, false, false);
    engine.on_key(keys::A, false, false);
    engine.on_key(keys::Y, false, false);
    engine.on_key(keys::A, false, false);
    let buf = engine.get_buffer();
    assert_eq!(buf, "cây", "Circumflex should apply backward to 'a' in 'ay' → 'ây'");
}

#[test]
fn test_keu_e_telex_gives_keu_circumflex() {
    let mut engine = Engine::new();
    // Type "keu" + "e" → expect "kêu" (ee pattern, circumflex on e)
    engine.on_key(keys::K, false, false);
    engine.on_key(keys::E, false, false);
    engine.on_key(keys::U, false, false);
    engine.on_key(keys::E, false, false);
    let buf = engine.get_buffer();
    assert_eq!(buf, "kêu", "Circumflex should apply backward to 'e' in 'eu' → 'êu'");
}

#[test]
fn test_cam_a_telex_gives_cam_circumflex() {
    let mut engine = Engine::new();
    // Type "cam" + "a" → expect "câm" (backward circumflex after final consonant)
    engine.on_key(keys::C, false, false);
    engine.on_key(keys::A, false, false);
    engine.on_key(keys::M, false, false);
    engine.on_key(keys::A, false, false);
    let buf = engine.get_buffer();
    assert_eq!(buf, "câm", "Circumflex should apply backward after final consonant");
}

#[test]
fn test_toiof_telex_gives_toi_huyen() {
    let mut engine = Engine::new();
    // Type "toiof" → "tồi" (circumflex oo + huyền f)
    engine.on_key(keys::T, false, false);
    engine.on_key(keys::O, false, false);
    engine.on_key(keys::I, false, false);
    engine.on_key(keys::O, false, false);
    engine.on_key(keys::F, false, false);
    let buf = engine.get_buffer();
    assert_eq!(buf, "tồi", "Should apply circumflex and huyền to get 'tồi'");
}

#[test]
fn test_backspace_after_space_tone_change() {
    let mut engine = Engine::new();
    // Type "than" + space → commit "than"
    engine.on_key(keys::T, false, false);
    engine.on_key(keys::H, false, false);
    engine.on_key(keys::A, false, false);
    engine.on_key(keys::N, false, false);
    let buf_before_space = engine.get_buffer();
    println!("before space: '{}'", buf_before_space);
    
    // Space commits the word
    engine.on_key(keys::SPACE, false, false);
    let buf_after_space = engine.get_buffer();
    println!("after space: '{}'", buf_after_space);
    
    // Backspace removes the space and restores buffer
    engine.on_key(keys::DELETE, false, false);
    let buf_after_backspace = engine.get_buffer();
    println!("after backspace: '{}'", buf_after_backspace);
    
    // Now type 's' to apply tone sắc
    engine.on_key(keys::S, false, false);
    let buf_final = engine.get_buffer();
    println!("after 's': '{}'", buf_final);
    
    assert_eq!(buf_final, "thán", "After space+backspace, tone 's' should apply to restored 'than' → 'thán'");
}

#[test]
fn test_shortcut_expansion_default() {
    let mut engine = Engine::new();
    engine.shortcuts_enabled = true;
    
    // Type "hcm" + space → should expand to "Hồ Chí Minh"
    engine.on_key(keys::H, false, false);
    engine.on_key(keys::C, false, false);
    engine.on_key(keys::M, false, false);
    let result = engine.on_key(keys::SPACE, false, false);
    
    // Check if result has characters (expansion happened)
    let chars_count = result.count;
    println!("hcm expansion: count={}, backspace={}", chars_count, result.backspace);
    assert!(chars_count > 0 || result.backspace > 0, "Shortcut 'hcm' should trigger expansion");
}

#[test]
fn test_backspace_after_space_replace_tone() {
    // Type "mỗi" (m-o-i-o-x) then space, backspace, "f" → should be "mồi"
    let mut engine = Engine::new();
    // m-o-i-o-x → mỗi
    engine.on_key(keys::M, false, false);
    engine.on_key(keys::O, false, false);
    engine.on_key(keys::I, false, false);
    engine.on_key(keys::O, false, false);
    engine.on_key(keys::X, false, false);
    let before_space = engine.get_buffer();
    eprintln!("before space: '{}'", before_space);
    assert_eq!(before_space, "mỗi", "mỗi should be formed");

    // space commits
    engine.on_key(keys::SPACE, false, false);
    let after_space = engine.get_buffer();
    eprintln!("after space: '{}'", after_space);

    // backspace restores
    engine.on_key(keys::DELETE, false, false);
    let after_bs = engine.get_buffer();
    eprintln!("after backspace: '{}'", after_bs);

    // now type "f" to change ngã→huyền
    engine.on_key(keys::F, false, false);
    let result = engine.get_buffer();
    eprintln!("after 'f': '{}'", result);
    assert_eq!(result, "mồi", "Should replace ngã with huyền → mồi");
}
