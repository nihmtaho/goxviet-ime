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
