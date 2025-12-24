//! Edge Case Tests for Vietnamese IME
//!
//! This module contains comprehensive test cases for challenging Vietnamese
//! input patterns that require special handling:
//!
//! 1. **ươ Compound Edge Cases**: thuở, chuột, ngươi, etc.
//! 2. **Modern vs Traditional Tone Placement**: Rule 4 patterns (oa, oe, uy)
//! 3. **Complex Vowel Pattern "oeo"**: khoèo, khèo, etc.
//!
//! Target: 95%+ accuracy for all edge cases

#[cfg(test)]
mod tests {
    use crate::engine::Engine;
    use crate::utils::type_word;

    // ═══════════════════════════════════════════════════════════════════
    // 1. ƯỠ COMPOUND EDGE CASES
    // ═══════════════════════════════════════════════════════════════════
    // These words require correct normalization of u+o → ư+ơ compound
    // The challenge: ensure both vowels get horn modifier correctly

    const UO_COMPOUND_BASIC: &[(&str, &str)] = &[
        // Basic ươ patterns
        ("duow", "dươ"),      // du + o + w → dươ
        ("duowc", "dươc"),    // dươ + c
        ("duowcj", "dược"),   // dươc + tone nặng → dược
        ("nguoiw", "ngươi"),  // ngu + o + i + w → ngươi
        ("tuoiw", "tươi"),    // tu + o + i + w → tươi
        ("tuoiwj", "tưới"),   // tươi + tone nặng → tưới
    ];

    const UO_COMPOUND_COMPLEX: &[(&str, &str)] = &[
        // Complex ươ words from requirements
        // Note: "uo" + "w" → both u and o get horn → "ươ"
        ("thuow", "thươ"),       // thu + o + w → thươ (both vowels get horn)
        ("thuowr", "thuở"),      // thươ + tone hỏi → thuở
        ("nguowfi", "người"),    // ngu + o + w + f + i → người (tone huyền on ơ)
        ("muownj", "mượn"),      // mu + o + w + n + j → mượn (tone nặng on ơ)
        ("luowngj", "lượng"),    // lu + o + w + ng + j → lượng (tone nặng on ơ)
        ("ruowuj", "rượu"),      // ru + o + w + u + j → rượu (tone nặng on ơ)
    ];

    const UO_COMPOUND_TONE_POSITIONING: &[(&str, &str)] = &[
        // Test tone mark positioning in ươ compounds
        // With final consonant: tone goes on ơ (second vowel) - Rule 3
        // Without final consonant: tone still goes on ơ (diacritic priority) - Rule 1
        ("duowcj", "dược"),      // du + o + w + c + j → dược (tone on ơ, has final 'c')
        ("duowf", "dườ"),        // du + o + w + f → dườ (tone on ơ, no final)
        ("duowr", "dưở"),        // du + o + w + r → dưở (tone on ơ, no final)
        ("duowx", "dưỡ"),        // du + o + w + x → dưỡ (tone on ơ, no final)
        ("duowj", "dượ"),        // du + o + w + j → dượ (tone on ơ, no final)
        ("nguowfi", "người"),    // ngu + o + w + f + i → người (tone on ơ)
        ("tuowri", "tưởi"),      // tu + o + w + r + i → tưởi (tone on ơ)
    ];

    const UO_COMPOUND_WITH_FINALS: &[(&str, &str)] = &[
        // ươ + final consonants (challenging patterns)
        ("duowc", "dươc"),       // du + o + w + c → dươc
        ("tuowng", "tương"),     // tu + o + w + ng → tương
        ("luown", "lươn"),       // lu + o + w + n → lươn
        ("buowcj", "bước"),      // bu + o + w + c + j → bước (tone nặng)
        ("huowngf", "hường"),    // hu + o + w + ng + f → hường (tone huyền)
    ];

    // ═══════════════════════════════════════════════════════════════════
    // 2. MODERN TONE PLACEMENT (Rule 4)
    // ═══════════════════════════════════════════════════════════════════
    // Target: 95% accuracy
    // Modern style: tone on SECOND vowel in open syllables (oa, oe, uy)

    const MODERN_TONE_OA: &[(&str, &str)] = &[
        // oa pattern - modern: tone on 'a'
        ("hoas", "hoá"),         // hoa + sắc → hoá (tone on a)
        ("hoaf", "hoà"),         // hoa + huyền → hoà
        ("hoar", "hoả"),         // hoa + hỏi → hoả
        ("hoax", "hoã"),         // hoa + ngã → hoã
        ("hoaj", "hoạ"),         // hoa + nặng → hoạ
        ("khoas", "khoá"),       // khoa + sắc → khoá
        ("toans", "toán"),       // toan + sắc → toán (with final consonant)
    ];

    const MODERN_TONE_OE: &[(&str, &str)] = &[
        // oe pattern - modern: tone on 'e'
        ("loes", "loé"),         // loe + sắc → loé
        ("loef", "loè"),         // loe + huyền → loè
        ("loer", "loẻ"),         // loe + hỏi → loẻ
        ("hoef", "hoè"),         // hoe + huyền → hoè
        ("toef", "toè"),         // toe + huyền → toè
    ];

    const MODERN_TONE_UY: &[(&str, &str)] = &[
        // uy pattern (no qu-initial) - modern: tone on 'y'
        ("tuys", "tuý"),         // tuy + sắc → tuý
        ("tuyf", "tuỳ"),         // tuy + huyền → tuỳ
        ("tuyr", "tuỷ"),         // tuy + hỏi → tuỷ
        ("muys", "muý"),         // muy + sắc → muý (rare but valid)
    ];

    const MODERN_TONE_UYE_TRIPHTHONG: &[(&str, &str)] = &[
        // uyê triphthong pattern - tone always on ê (has diacritic)
        // NOT affected by modern/traditional setting
        ("duyeenf", "duyền"),    // duy + e + e + n + f → duyền
        ("duyeens", "duyến"),    // duy + e + e + n + s → duyến
        ("tuyeenf", "tuyền"),    // tuy + e + e + n + f → tuyền
        ("kuyeens", "kuyến"),    // kuy + e + e + n + s → kuyến (rare)
    ];

    const MODERN_TONE_UY_QU_INITIAL: &[(&str, &str)] = &[
        // uy with qu-initial - always on 'y' (qu is consonant cluster)
        // NOT affected by modern/traditional setting
        ("quys", "quý"),         // quy + sắc → quý
        ("quyf", "quỳ"),         // quy + huyền → quỳ
        ("quyr", "quỷ"),         // quy + hỏi → quỷ
        ("quyeenf", "quyền"),    // quy + e + e + n + f → quyền (uyê triphthong, tone on ê)
        ("quyeets", "quyết"),    // quy + e + e + t + s → quyết (uyê triphthong, tone on ê)
    ];

    // ═══════════════════════════════════════════════════════════════════
    // 3. TRADITIONAL TONE PLACEMENT (Rule 4)
    // ═══════════════════════════════════════════════════════════════════
    // Target: 95% accuracy
    // Traditional style: tone on FIRST vowel in open syllables (oa, oe, uy)

    const TRADITIONAL_TONE_OA: &[(&str, &str)] = &[
        // oa pattern - traditional: tone on 'o'
        ("hoas", "hóa"),         // hoa + sắc → hóa (tone on o)
        ("hoaf", "hòa"),         // hoa + huyền → hòa
        ("hoar", "hỏa"),         // hoa + hỏi → hỏa
        ("hoax", "hõa"),         // hoa + ngã → hõa
        ("hoaj", "họa"),         // hoa + nặng → họa
        ("khoas", "khóa"),       // khoa + sắc → khóa
    ];

    const TRADITIONAL_TONE_OE: &[(&str, &str)] = &[
        // oe pattern - traditional: tone on 'o'
        ("loes", "lóe"),         // loe + sắc → lóe
        ("loef", "lòe"),         // loe + huyền → lòe
        ("loer", "lỏe"),         // loe + hỏi → lỏe
        ("hoef", "hòe"),         // hoe + huyền → hòe
        ("toef", "tòe"),         // toe + huyền → tòe
    ];

    const TRADITIONAL_TONE_UY: &[(&str, &str)] = &[
        // uy pattern (no qu-initial) - traditional: tone on 'u'
        ("tuys", "túy"),         // tuy + sắc → túy
        ("tuyf", "tùy"),         // tuy + huyền → tùy
        ("tuyr", "tủy"),         // tuy + hỏi → tủy
        ("muys", "múy"),         // muy + sắc → múy
        ("duyf", "dùy"),         // duy + huyền → dùy
    ];

    // ═══════════════════════════════════════════════════════════════════
    // 4. COMPLEX VOWEL PATTERN "OEO"
    // ═══════════════════════════════════════════════════════════════════
    // Triphthong "oeo" requires tone on middle 'e'

    const OEO_PATTERN: &[(&str, &str)] = &[
        // oeo pattern - tone on middle 'e'
        ("khoeo", "khoeo"),      // khoe + o → khoeo
        ("khoeof", "khoèo"),     // khoeo + huyền → khoèo (tone on e)
        ("khoeos", "khoeó"),     // khoeo + sắc → khoeó (tone on e)
        ("kheof", "khèo"),       // kheo + huyền → khèo
        ("kheos", "khéo"),       // kheo + sắc → khéo
        ("ngoeof", "ngoèo"),     // ngoeo + huyền → ngoèo
        ("ngoeos", "ngoeó"),     // ngoeo + sắc → ngoeó
    ];

    const OEO_WITH_FINALS: &[(&str, &str)] = &[
        // oeo + final consonants (if valid)
        ("kheoet", "kheoét"),    // oeo + t (rare pattern)
        ("khoeot", "khoèot"),    // oeo + huyền + t
    ];

    // ═══════════════════════════════════════════════════════════════════
    // TEST FUNCTIONS
    // ═══════════════════════════════════════════════════════════════════

    #[test]
    fn test_uo_compound_basic() {
        for (input, expected) in UO_COMPOUND_BASIC {
            let mut e = Engine::new();
            let result = type_word(&mut e, input);
            assert_eq!(
                result, *expected,
                "[ươ Basic] '{}' should become '{}' but got '{}'",
                input, expected, result
            );
        }
    }

    #[test]
    fn test_uo_compound_complex() {
        for (input, expected) in UO_COMPOUND_COMPLEX {
            let mut e = Engine::new();
            let result = type_word(&mut e, input);
            assert_eq!(
                result, *expected,
                "[ươ Complex] '{}' should become '{}' but got '{}'",
                input, expected, result
            );
        }
    }

    #[test]
    fn test_uo_compound_tone_positioning() {
        for (input, expected) in UO_COMPOUND_TONE_POSITIONING {
            let mut e = Engine::new();
            let result = type_word(&mut e, input);
            assert_eq!(
                result, *expected,
                "[ươ Tone Position] '{}' should become '{}' but got '{}'",
                input, expected, result
            );
        }
    }

    #[test]
    fn test_uo_compound_with_finals() {
        for (input, expected) in UO_COMPOUND_WITH_FINALS {
            let mut e = Engine::new();
            let result = type_word(&mut e, input);
            assert_eq!(
                result, *expected,
                "[ươ + Finals] '{}' should become '{}' but got '{}'",
                input, expected, result
            );
        }
    }

    #[test]
    fn test_modern_tone_oa() {
        for (input, expected) in MODERN_TONE_OA {
            let mut e = Engine::new();
            e.set_modern_tone(true); // Enable modern style
            let result = type_word(&mut e, input);
            assert_eq!(
                result, *expected,
                "[Modern OA] '{}' should become '{}' but got '{}'",
                input, expected, result
            );
        }
    }

    #[test]
    fn test_modern_tone_oe() {
        for (input, expected) in MODERN_TONE_OE {
            let mut e = Engine::new();
            e.set_modern_tone(true);
            let result = type_word(&mut e, input);
            assert_eq!(
                result, *expected,
                "[Modern OE] '{}' should become '{}' but got '{}'",
                input, expected, result
            );
        }
    }

    #[test]
    fn test_modern_tone_uy() {
        for (input, expected) in MODERN_TONE_UY {
            let mut e = Engine::new();
            e.set_modern_tone(true);
            let result = type_word(&mut e, input);
            assert_eq!(
                result, *expected,
                "[Modern UY] '{}' should become '{}' but got '{}'",
                input, expected, result
            );
        }
    }

    #[test]
    fn test_modern_tone_uye_triphthong() {
        for (input, expected) in MODERN_TONE_UYE_TRIPHTHONG {
            let mut e = Engine::new();
            e.set_modern_tone(true);
            let result = type_word(&mut e, input);
            assert_eq!(
                result, *expected,
                "[Modern UYÊ Triphthong] '{}' should become '{}' but got '{}'",
                input, expected, result
            );
        }
    }

    #[test]
    fn test_modern_tone_uy_qu_initial() {
        for (input, expected) in MODERN_TONE_UY_QU_INITIAL {
            let mut e = Engine::new();
            e.set_modern_tone(true); // Should not affect qu-initial uy
            let result = type_word(&mut e, input);
            assert_eq!(
                result, *expected,
                "[Modern UY + QU] '{}' should become '{}' but got '{}'",
                input, expected, result
            );
        }
    }

    #[test]
    fn test_traditional_tone_oa() {
        for (input, expected) in TRADITIONAL_TONE_OA {
            let mut e = Engine::new();
            e.set_modern_tone(false); // Enable traditional style
            let result = type_word(&mut e, input);
            assert_eq!(
                result, *expected,
                "[Traditional OA] '{}' should become '{}' but got '{}'",
                input, expected, result
            );
        }
    }

    #[test]
    fn test_traditional_tone_oe() {
        for (input, expected) in TRADITIONAL_TONE_OE {
            let mut e = Engine::new();
            e.set_modern_tone(false);
            let result = type_word(&mut e, input);
            assert_eq!(
                result, *expected,
                "[Traditional OE] '{}' should become '{}' but got '{}'",
                input, expected, result
            );
        }
    }

    #[test]
    fn test_traditional_tone_uy() {
        for (input, expected) in TRADITIONAL_TONE_UY {
            let mut e = Engine::new();
            e.set_modern_tone(false);
            let result = type_word(&mut e, input);
            assert_eq!(
                result, *expected,
                "[Traditional UY] '{}' should become '{}' but got '{}'",
                input, expected, result
            );
        }
    }

    #[test]
    fn test_oeo_pattern() {
        for (input, expected) in OEO_PATTERN {
            let mut e = Engine::new();
            let result = type_word(&mut e, input);
            assert_eq!(
                result, *expected,
                "[OEO Pattern] '{}' should become '{}' but got '{}'",
                input, expected, result
            );
        }
    }

    #[test]
    fn test_oeo_with_finals() {
        for (input, expected) in OEO_WITH_FINALS {
            let mut e = Engine::new();
            let result = type_word(&mut e, input);
            assert_eq!(
                result, *expected,
                "[OEO + Finals] '{}' should become '{}' but got '{}'",
                input, expected, result
            );
        }
    }

    // ═══════════════════════════════════════════════════════════════════
    // ACCURACY VERIFICATION TESTS
    // ═══════════════════════════════════════════════════════════════════

    #[test]
    fn verify_uo_compound_accuracy() {
        let all_cases: Vec<(&str, &str)> = UO_COMPOUND_BASIC
            .iter()
            .chain(UO_COMPOUND_COMPLEX.iter())
            .chain(UO_COMPOUND_TONE_POSITIONING.iter())
            .chain(UO_COMPOUND_WITH_FINALS.iter())
            .copied()
            .collect();

        let mut passed = 0;
        let total = all_cases.len();

        for (input, expected) in &all_cases {
            let mut e = Engine::new();
            let result = type_word(&mut e, input);
            if result == *expected {
                passed += 1;
            } else {
                eprintln!(
                    "FAIL [ươ Compound]: '{}' expected '{}' but got '{}'",
                    input, expected, result
                );
            }
        }

        let accuracy = (passed as f64 / total as f64) * 100.0;
        eprintln!("\nươ Compound Accuracy: {:.1}% ({}/{})", accuracy, passed, total);
        assert!(
            accuracy >= 95.0,
            "ươ compound accuracy {:.1}% below target 95%",
            accuracy
        );
    }

    #[test]
    fn verify_modern_tone_accuracy() {
        let all_cases: Vec<(&str, &str)> = MODERN_TONE_OA
            .iter()
            .chain(MODERN_TONE_OE.iter())
            .chain(MODERN_TONE_UY.iter())
            .chain(MODERN_TONE_UYE_TRIPHTHONG.iter())
            .chain(MODERN_TONE_UY_QU_INITIAL.iter())
            .copied()
            .collect();

        let mut passed = 0;
        let total = all_cases.len();

        for (input, expected) in &all_cases {
            let mut e = Engine::new();
            e.set_modern_tone(true);
            let result = type_word(&mut e, input);
            if result == *expected {
                passed += 1;
            } else {
                eprintln!(
                    "FAIL [Modern Tone]: '{}' expected '{}' but got '{}'",
                    input, expected, result
                );
            }
        }

        let accuracy = (passed as f64 / total as f64) * 100.0;
        eprintln!("\nModern Tone Accuracy: {:.1}% ({}/{})", accuracy, passed, total);
        assert!(
            accuracy >= 95.0,
            "Modern tone accuracy {:.1}% below target 95%",
            accuracy
        );
    }

    #[test]
    fn verify_traditional_tone_accuracy() {
        let all_cases: Vec<(&str, &str)> = TRADITIONAL_TONE_OA
            .iter()
            .chain(TRADITIONAL_TONE_OE.iter())
            .chain(TRADITIONAL_TONE_UY.iter())
            .copied()
            .collect();

        let mut passed = 0;
        let total = all_cases.len();

        for (input, expected) in &all_cases {
            let mut e = Engine::new();
            e.set_modern_tone(false);
            let result = type_word(&mut e, input);
            if result == *expected {
                passed += 1;
            } else {
                eprintln!(
                    "FAIL [Traditional Tone]: '{}' expected '{}' but got '{}'",
                    input, expected, result
                );
            }
        }

        let accuracy = (passed as f64 / total as f64) * 100.0;
        eprintln!("\nTraditional Tone Accuracy: {:.1}% ({}/{})", accuracy, passed, total);
        assert!(
            accuracy >= 95.0,
            "Traditional tone accuracy {:.1}% below target 95%",
            accuracy
        );
    }

    #[test]
    fn verify_oeo_pattern_accuracy() {
        let all_cases: Vec<(&str, &str)> = OEO_PATTERN
            .iter()
            .chain(OEO_WITH_FINALS.iter())
            .copied()
            .collect();

        let mut passed = 0;
        let total = all_cases.len();

        for (input, expected) in &all_cases {
            let mut e = Engine::new();
            let result = type_word(&mut e, input);
            if result == *expected {
                passed += 1;
            } else {
                eprintln!(
                    "FAIL [OEO Pattern]: '{}' expected '{}' but got '{}'",
                    input, expected, result
                );
            }
        }

        let accuracy = (passed as f64 / total as f64) * 100.0;
        eprintln!("\nOEO Pattern Accuracy: {:.1}% ({}/{})", accuracy, passed, total);
        assert!(
            accuracy >= 95.0,
            "OEO pattern accuracy {:.1}% below target 95%",
            accuracy
        );
    }
}