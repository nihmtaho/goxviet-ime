// Direct validator test
use goxviet_core::data::keys;
use goxviet_core::engine_v2::vietnamese_validator::VietnameseSyllableValidator;

#[test]
fn test_och_should_be_invalid() {
    // "och" should be invalid: o + ch is not allowed
    let result = VietnameseSyllableValidator::validate(&[keys::O, keys::C, keys::H]);
    println!(
        "och validation: is_valid={}, confidence={}",
        result.is_valid, result.confidence
    );
    assert!(!result.is_valid, "'och' should be invalid Vietnamese");
}

#[test]
fn test_ach_should_be_valid() {
    // "ach" should be valid: a + ch is allowed
    let result = VietnameseSyllableValidator::validate(&[keys::A, keys::C, keys::H]);
    println!(
        "ach validation: is_valid={}, confidence={}",
        result.is_valid, result.confidence
    );
    assert!(result.is_valid, "'ach' should be valid Vietnamese");
}

#[test]
fn test_uch_should_be_invalid() {
    // "uch" should be invalid: u + ch is not allowed
    let result = VietnameseSyllableValidator::validate(&[keys::U, keys::C, keys::H]);
    println!(
        "uch validation: is_valid={}, confidence={}",
        result.is_valid, result.confidence
    );
    assert!(!result.is_valid, "'uch' should be invalid Vietnamese");
}

#[test]
fn test_eng_with_no_modifier_is_valid() {
    // "eng" (plain e, no circumflex) is valid Vietnamese (e.g., "xà beng")
    use goxviet_core::data::chars::tone;
    let keys_arr = [keys::E, keys::N, keys::G];
    let tones_arr = [tone::NONE, tone::NONE, tone::NONE];
    let result = VietnameseSyllableValidator::validate_with_tones(&keys_arr, &tones_arr);
    assert!(result.is_valid, "'eng' (plain e) should be valid Vietnamese");
}

#[test]
fn test_eng_with_circumflex_is_invalid() {
    // "êng" (e with circumflex → ê) is invalid; should use "-ênh" instead
    use goxviet_core::data::chars::tone;
    let keys_arr = [keys::E, keys::N, keys::G];
    let tones_arr = [tone::CIRCUMFLEX, tone::NONE, tone::NONE]; // ê+ng
    let result = VietnameseSyllableValidator::validate_with_tones(&keys_arr, &tones_arr);
    assert!(!result.is_valid, "'êng' (ê before -ng) should be invalid Vietnamese");
}

#[test]
fn test_anh_is_valid_with_tones() {
    // "anh" is valid, unaffected by e-before-ng check
    use goxviet_core::data::chars::tone;
    let keys_arr = [keys::A, keys::N, keys::H];
    let tones_arr = [tone::NONE, tone::NONE, tone::NONE];
    let result = VietnameseSyllableValidator::validate_with_tones(&keys_arr, &tones_arr);
    assert!(result.is_valid, "'anh' should be valid Vietnamese");
}

#[test]
fn test_ieng_compound_is_valid() {
    // "iêng" (iê compound + -ng) is valid — e.g., tiếng, kiếng, riêng
    use goxviet_core::data::chars::tone;
    // Keys: I, E, N, G; tones: NONE, CIRCUMFLEX (ê), NONE, NONE
    let keys_arr = [keys::I, keys::E, keys::N, keys::G];
    let tones_arr = [tone::NONE, tone::CIRCUMFLEX, tone::NONE, tone::NONE];
    let result = VietnameseSyllableValidator::validate_with_tones(&keys_arr, &tones_arr);
    assert!(result.is_valid, "'iêng' (iê compound) should be valid Vietnamese");
}
