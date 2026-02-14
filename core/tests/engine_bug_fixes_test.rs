//! Test for 4 critical engine bug fixes
//! 
//! Issues from DICTIONARY_TEST_FAILURE_ANALYSIS_V2.md:
//! 1. Smart 'w' Double-Apply Bug: khuow → khươ (should be khuơ)
//! 2. Compound Vowel Over-Aggressive: khoeo → khôe (should stay khoeo)
//! 3. Foreign Word Auto-Restore: tareh → Taẻh (should be Tareh)
//! 4. VNI Compound Mark: thuo73 → thưở (should be thuở)

use goxviet_core::engine::Engine;
use goxviet_core::utils::type_word;

#[test]
fn test_issue_1_smart_w_double_apply_telex() {
    // Issue: khuow should produce khuơ, not khươ
    // Pattern: u + o + w (Smart w handling applies ơ incorrectly twice)
    let mut engine = Engine::new();
    let result = type_word(&mut engine, "khuow");
    println!("Issue #1 (Telex): khuow → {}", result);
    assert_eq!(result, "khuơ", "Smart 'w' should create ơ, not ư+ơ");
}

#[test]
fn test_issue_1_smart_w_double_apply_vni() {
    // Issue: khuo7 should produce khuơ, not khươ
    // In VNI: 7 = horn/móc modifier
    let mut engine = Engine::new();
    engine.set_method(1);  // 1 = VNI method
    let result = type_word(&mut engine, "khuo7");
    println!("Issue #1 (VNI): khuo7 → {}", result);
    assert_eq!(result, "khuơ", "VNI '7' should create ơ, not ư+ơ");
}

#[test]
fn test_issue_2_compound_vowel_oeo_telex() {
    // Issue: khoeo should stay khoeo, not become khôe
    // Pattern: o + e + o (engine incorrectly pairs oe as compound vowel)
    let mut engine = Engine::new();
    
    // Type step by step to debug
    println!("\nStep 1: k");
    engine.on_key(40, false, false); // k
    println!("Step 2: h");
    engine.on_key(4, false, false); // h
    println!("Step 3: o");
    engine.on_key(31, false, false); // o
    println!("Step 4: e");
    engine.on_key(14, false, false); // e
    println!("Step 5: o (critical)");
    let result = engine.on_key(31, false, false); // o
    
    println!("Result: action={}, count={}, backspace={}", 
        result.action, result.count, result.backspace);
    
    // Get full display text from engine buffer
    let display_text = engine.get_buffer();
    println!("Issue #2 (Telex): khoeo → '{}'", display_text);
    
    assert_eq!(display_text, "khoeo", "Pattern 'oeo' should not be transformed");
}

#[test]
fn test_issue_3_foreign_word_tareh() {
    // Issue: tareh should stay as is, not trigger auto-restore to Taẻh
    // Pattern: Word ends with 'eh' (foreign suffix) - auto-restore too aggressive
    let mut engine = Engine::new();
    let result = type_word(&mut engine, "tareh");
    println!("Issue #3 (Telex): tareh → {}", result);
    assert_eq!(result, "tareh", "Foreign word should not trigger unwanted auto-restore");
}

#[test]
fn test_issue_4_vni_compound_mark_thuow() {
    // Issue: thuo73 should produce thuở, not thưở
    // Pattern: u + o + 7(horn) + 3(hỏi) creates unwanted ư
    // In VNI: 7 = móc (horn), 3 = hỏi (interrogative)
    let mut engine = Engine::new();
    engine.set_method(1);  // 1 = VNI method
    let result = type_word(&mut engine, "thuo73");
    println!("Issue #4 (VNI): thuo73 → {}", result);
    assert_eq!(result, "thuở", "VNI compound mark should not create unintended vowel");
}

// Additional tests for variations
#[test]
#[ignore = "Issue #2 variant: Compound Vowel with tone"]
fn test_issue_2_compound_vowel_khoeo_with_tone_telex() {
    // Variant: khoèo should stay khoèo, not become khôef
    let mut engine = Engine::new();
    let result = type_word(&mut engine, "khoeof");
    println!("Issue #2 variant: khoeof → {}", result);
    assert_eq!(result, "khoèo", "Pattern 'oeo' with tone should not be transformed");
}

#[test]
fn test_normal_uo_compound_still_works_telex() {
    // Ensure fix doesn't break normal uo → ương transformation
    // This should STILL work: muowng → mương
    let mut engine = Engine::new();
    let result = type_word(&mut engine, "muowng");
    println!("Normal uo compound: muowng → {}", result);
    assert_eq!(result, "mương", "Normal u+o+w → ương should still work");
}

#[test]
fn test_normal_uo_compound_still_works_vni() {
    // Ensure fix doesn't break normal uo → ương transformation in VNI
    // muo + 7 + ng → mương (with default tone)
    let mut engine = Engine::new();
    engine.set_method(1);  // 1 = VNI method
    let result = type_word(&mut engine, "muo7ng");
    println!("Normal uo compound (VNI): muo7ng → {}", result);
    assert_eq!(result, "mương", "Normal u+o+7 → ương should still work");
}

// ============================================================================
// ISSUE #5: "uyu" triphthong not recognized
// Pattern: khuyu + tone → should become "khuỷu"
// Root cause: Engine doesn't recognize "uyu" as valid triphthong pattern
// ============================================================================

#[test]
fn test_issue_5_uyu_triphthong_telex() {
    // Pattern: khuyur → Expected: khuỷu (u+y+u with hỏi tone on y)
    // Telex: r = hỏi tone
    let mut engine = Engine::new();
    engine.set_method(0);  // 0 = Telex
    let result = type_word(&mut engine, "khuyur");
    println!("Issue #5 Telex: khuyur → {}", result);
    assert_eq!(result, "khuỷu", "uyu triphthong with hỏi tone should work");
}

#[test]
fn test_issue_5_uyu_triphthong_vni() {
    // Pattern: khuyu3 → Expected: khuỷu (u+y+u with hỏi tone on y)
    // VNI: 3 = hỏi tone
    let mut engine = Engine::new();
    engine.set_method(1);  // 1 = VNI
    let result = type_word(&mut engine, "khuyu3");
    println!("Issue #5 VNI: khuyu3 → {}", result);
    assert_eq!(result, "khuỷu", "uyu triphthong with hỏi tone should work");
}
