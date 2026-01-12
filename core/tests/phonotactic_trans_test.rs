use goxviet_core::data::keys;
use goxviet_core::engine_v2::english::phonotactic::PhonotacticEngine;

#[test]
fn test_phonotactic_trans() {
    // Test that "trans" returns high confidence
    let keys_trans = vec![
        (keys::T, false),
        (keys::R, false),
        (keys::A, false),
        (keys::N, false),
        (keys::S, false),
    ];

    let result = PhonotacticEngine::analyze(&keys_trans);
    println!(
        "trans: is_english={}, confidence={}",
        result.is_english(),
        result.english_confidence
    );

    assert!(result.is_english(), "trans should be detected as English");
    assert!(
        result.english_confidence >= 95,
        "trans should have confidence >= 95, got {}",
        result.english_confidence
    );
}

#[test]
fn test_phonotactic_transf() {
    // Test that "transf" also returns high confidence
    let keys_transf = vec![
        (keys::T, false),
        (keys::R, false),
        (keys::A, false),
        (keys::N, false),
        (keys::S, false),
        (keys::F, false),
    ];

    let result = PhonotacticEngine::analyze(&keys_transf);
    println!(
        "transf: is_english={}, confidence={}",
        result.is_english(),
        result.english_confidence
    );

    assert!(result.is_english(), "transf should be detected as English");
    assert!(
        result.english_confidence >= 95,
        "transf should have confidence >= 95, got {}",
        result.english_confidence
    );
}
