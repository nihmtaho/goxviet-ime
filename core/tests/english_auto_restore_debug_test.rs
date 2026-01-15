// Test to debug English auto-restore for programming terms: syntax, parse, merge
//
// Issue: Words from dictionary like "syntax", "parse" don't auto-restore correctly,
// but "merge" works fine. This test helps identify why.

use goxviet_core::data::keys;
use goxviet_core::engine_v2::english::dictionary::Dictionary;
use goxviet_core::engine_v2::english::language_decision::LanguageDecisionEngine;
use goxviet_core::engine_v2::english::phonotactic::PhonotacticEngine;
use goxviet_core::engine_v2::vietnamese_validator::VietnameseSyllableValidator;

fn make_key_tuple_from_str(s: &str) -> Vec<(u16, bool)> {
    s.chars()
        .map(|c| match c {
            'a' => (keys::A, false),
            'b' => (keys::B, false),
            'c' => (keys::C, false),
            'd' => (keys::D, false),
            'e' => (keys::E, false),
            'f' => (keys::F, false),
            'g' => (keys::G, false),
            'h' => (keys::H, false),
            'i' => (keys::I, false),
            'j' => (keys::J, false),
            'k' => (keys::K, false),
            'l' => (keys::L, false),
            'm' => (keys::M, false),
            'n' => (keys::N, false),
            'o' => (keys::O, false),
            'p' => (keys::P, false),
            'q' => (keys::Q, false),
            'r' => (keys::R, false),
            's' => (keys::S, false),
            't' => (keys::T, false),
            'u' => (keys::U, false),
            'v' => (keys::V, false),
            'w' => (keys::W, false),
            'x' => (keys::X, false),
            'y' => (keys::Y, false),
            'z' => (keys::Z, false),
            _ => panic!("Unsupported character: {}", c),
        })
        .collect()
}

fn make_key_vec_from_str(s: &str) -> Vec<u16> {
    s.chars()
        .map(|c| match c {
            'a' => keys::A,
            'b' => keys::B,
            'c' => keys::C,
            'd' => keys::D,
            'e' => keys::E,
            'f' => keys::F,
            'g' => keys::G,
            'h' => keys::H,
            'i' => keys::I,
            'j' => keys::J,
            'k' => keys::K,
            'l' => keys::L,
            'm' => keys::M,
            'n' => keys::N,
            'o' => keys::O,
            'p' => keys::P,
            'q' => keys::Q,
            'r' => keys::R,
            's' => keys::S,
            't' => keys::T,
            'u' => keys::U,
            'v' => keys::V,
            'w' => keys::W,
            'x' => keys::X,
            'y' => keys::Y,
            'z' => keys::Z,
            _ => panic!("Unsupported character: {}", c),
        })
        .collect()
}

#[test]
fn test_dict_lookup_syntax() {
    // Test if "syntax" is found in dictionary
    let keys = make_key_vec_from_str("syntax");
    let is_english = Dictionary::is_english(&keys);
    println!("Dictionary lookup 'syntax': {}", is_english);
    assert!(
        is_english,
        "syntax should be found in dictionary as English word"
    );
}

#[test]
fn test_dict_lookup_parse() {
    // Test if "parse" is found in dictionary
    let keys = make_key_vec_from_str("parse");
    let is_english = Dictionary::is_english(&keys);
    println!("Dictionary lookup 'parse': {}", is_english);
    assert!(
        is_english,
        "parse should be found in dictionary as English word"
    );
}

#[test]
fn test_dict_lookup_merge() {
    // Test if "merge" is found in dictionary
    let keys = make_key_vec_from_str("merge");
    let is_english = Dictionary::is_english(&keys);
    println!("Dictionary lookup 'merge': {}", is_english);
    assert!(
        is_english,
        "merge should be found in dictionary as English word"
    );
}

#[test]
fn test_validate_syntax() {
    // Test Vietnamese validation for "syntax"
    let keys = make_key_vec_from_str("syntax");
    let result = VietnameseSyllableValidator::validate(&keys);
    println!(
        "Vietnamese validation 'syntax': valid={}, confidence={}",
        result.is_valid, result.confidence
    );
}

#[test]
fn test_validate_parse() {
    // Test Vietnamese validation for "parse"
    let keys = make_key_vec_from_str("parse");
    let result = VietnameseSyllableValidator::validate(&keys);
    println!(
        "Vietnamese validation 'parse': valid={}, confidence={}",
        result.is_valid, result.confidence
    );
}

#[test]
fn test_validate_merge() {
    // Test Vietnamese validation for "merge"
    let keys = make_key_vec_from_str("merge");
    let result = VietnameseSyllableValidator::validate(&keys);
    println!(
        "Vietnamese validation 'merge': valid={}, confidence={}",
        result.is_valid, result.confidence
    );
}

#[test]
fn test_phonotactic_syntax() {
    // Test phonotactic analysis for "syntax"
    let keys = make_key_tuple_from_str("syntax");
    let phonotactic = PhonotacticEngine::analyze(&keys);
    println!(
        "Phonotactic 'syntax': confidence={}, layers={:08b}",
        phonotactic.english_confidence, phonotactic.matched_layers
    );
    println!(
        "  Layer scores: {:?}",
        phonotactic.layer_scores.iter().take(6).collect::<Vec<_>>()
    );
}

#[test]
fn test_phonotactic_parse() {
    // Test phonotactic analysis for "parse"
    let keys = make_key_tuple_from_str("parse");
    let phonotactic = PhonotacticEngine::analyze(&keys);
    println!(
        "Phonotactic 'parse': confidence={}, layers={:08b}",
        phonotactic.english_confidence, phonotactic.matched_layers
    );
    println!(
        "  Layer scores: {:?}",
        phonotactic.layer_scores.iter().take(6).collect::<Vec<_>>()
    );
}

#[test]
fn test_phonotactic_merge() {
    // Test phonotactic analysis for "merge"
    let keys = make_key_tuple_from_str("merge");
    let phonotactic = PhonotacticEngine::analyze(&keys);
    println!(
        "Phonotactic 'merge': confidence={}, layers={:08b}",
        phonotactic.english_confidence, phonotactic.matched_layers
    );
    println!(
        "  Layer scores: {:?}",
        phonotactic.layer_scores.iter().take(6).collect::<Vec<_>>()
    );
}

#[test]
fn test_language_decision_syntax() {
    // Test unified language decision for "syntax"
    let keys = make_key_tuple_from_str("syntax");
    let vietnamese_validation = VietnameseSyllableValidator::validate(
        &keys
            .iter()
            .map(|(k, _)| *k)
            .collect::<Vec<_>>(),
    );
    let is_valid = vietnamese_validation.is_valid;
    let viet_confidence = vietnamese_validation.confidence;

    let decision = LanguageDecisionEngine::decide_with_validation(&keys, false, Some(vietnamese_validation));
    println!(
        "Language decision 'syntax': is_english={}, confidence={}",
        decision.is_english, decision.confidence
    );
    println!("  Vietnamese validation: valid={}, confidence={}", is_valid, viet_confidence);
}

#[test]
fn test_language_decision_parse() {
    // Test unified language decision for "parse"
    let keys = make_key_tuple_from_str("parse");
    let vietnamese_validation = VietnameseSyllableValidator::validate(
        &keys
            .iter()
            .map(|(k, _)| *k)
            .collect::<Vec<_>>(),
    );
    let is_valid = vietnamese_validation.is_valid;
    let viet_confidence = vietnamese_validation.confidence;

    let decision = LanguageDecisionEngine::decide_with_validation(&keys, false, Some(vietnamese_validation));
    println!(
        "Language decision 'parse': is_english={}, confidence={}",
        decision.is_english, decision.confidence
    );
    println!("  Vietnamese validation: valid={}, confidence={}", is_valid, viet_confidence);
}

#[test]
fn test_language_decision_merge() {
    // Test unified language decision for "merge"
    let keys = make_key_tuple_from_str("merge");
    let vietnamese_validation = VietnameseSyllableValidator::validate(
        &keys
            .iter()
            .map(|(k, _)| *k)
            .collect::<Vec<_>>(),
    );
    let is_valid = vietnamese_validation.is_valid;
    let viet_confidence = vietnamese_validation.confidence;

    let decision = LanguageDecisionEngine::decide_with_validation(&keys, false, Some(vietnamese_validation));
    println!(
        "Language decision 'merge': is_english={}, confidence={}",
        decision.is_english, decision.confidence
    );
    println!("  Vietnamese validation: valid={}, confidence={}", is_valid, viet_confidence);
}
