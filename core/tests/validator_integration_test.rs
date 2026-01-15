// Integration test for Vietnamese validator enhancements
use goxviet_core::data::keys;
use goxviet_core::engine_v2::vietnamese_validator::VietnameseSyllableValidator;

#[test]
fn test_initial_consonant_validation() {
    // Valid Vietnamese initial consonants
    assert!(
        VietnameseSyllableValidator::validate(&[keys::N, keys::G, keys::H, keys::I, keys::A])
            .is_valid
    ); // nghia
    assert!(VietnameseSyllableValidator::validate(&[keys::P, keys::H, keys::O]).is_valid); // pho
    assert!(VietnameseSyllableValidator::validate(&[keys::T, keys::H, keys::O, keys::I]).is_valid); // thoi
    assert!(VietnameseSyllableValidator::validate(&[keys::T, keys::R, keys::O, keys::I]).is_valid); // troi

    // Invalid English consonant clusters
    assert!(
        !VietnameseSyllableValidator::validate(&[keys::C, keys::L, keys::E, keys::A, keys::N])
            .is_valid
    ); // clean
    assert!(
        !VietnameseSyllableValidator::validate(&[keys::B, keys::R, keys::O, keys::W, keys::N])
            .is_valid
    ); // brown
    assert!(!VietnameseSyllableValidator::validate(&[keys::F, keys::L, keys::Y]).is_valid);
    // fly
}

#[test]
fn test_vowel_combination_validation() {
    // Valid 2-vowel combinations
    assert!(VietnameseSyllableValidator::validate(&[keys::M, keys::A, keys::I]).is_valid); // mai
    assert!(VietnameseSyllableValidator::validate(&[keys::S, keys::A, keys::O]).is_valid); // sao
    assert!(VietnameseSyllableValidator::validate(&[keys::T, keys::O, keys::I]).is_valid); // toi

    // Valid 3-vowel combinations
    assert!(
        VietnameseSyllableValidator::validate(&[keys::K, keys::H, keys::O, keys::A, keys::I])
            .is_valid
    ); // khoai
    assert!(
        VietnameseSyllableValidator::validate(&[keys::Q, keys::U, keys::Y, keys::E, keys::N])
            .is_valid
    ); // quyen
}

#[test]
fn test_vowel_coda_compatibility() {
    // Valid vowel-coda pairs
    assert!(VietnameseSyllableValidator::validate(&[keys::A, keys::C, keys::H]).is_valid); // ach
    assert!(VietnameseSyllableValidator::validate(&[keys::I, keys::C, keys::H]).is_valid); // ich
    assert!(VietnameseSyllableValidator::validate(&[keys::A, keys::N, keys::H]).is_valid); // anh
    assert!(VietnameseSyllableValidator::validate(&[keys::I, keys::N, keys::H]).is_valid); // inh

    // Invalid vowel-coda pairs (o/u before ch/nh)
    assert!(!VietnameseSyllableValidator::validate(&[keys::O, keys::C, keys::H]).is_valid); // och
    assert!(!VietnameseSyllableValidator::validate(&[keys::U, keys::C, keys::H]).is_valid); // uch
    assert!(!VietnameseSyllableValidator::validate(&[keys::O, keys::N, keys::H]).is_valid); // onh
    assert!(!VietnameseSyllableValidator::validate(&[keys::U, keys::N, keys::H]).is_valid); // unh

    // E before -ng is invalid (should use -nh)
    assert!(!VietnameseSyllableValidator::validate(&[keys::E, keys::N, keys::G]).is_valid);
    // eng
}
