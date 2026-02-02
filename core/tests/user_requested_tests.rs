/// User-requested test cases for Vietnamese Input Method
/// These tests validate specific user workflows and expectations
use goxviet_core::engine::Engine;

fn type_word(input: &str, method: u8) -> String {
    let mut engine = Engine::new();
    engine.set_method(method);
    engine.set_enabled(true);
    engine.set_modern_tone(false); // Use old-style tone positioning (hòa, not hoà)

    for ch in input.chars() {
        let key_code = match ch {
            'a' => goxviet_core::data::keys::A,
            'b' => goxviet_core::data::keys::B,
            'c' => goxviet_core::data::keys::C,
            'd' => goxviet_core::data::keys::D,
            'e' => goxviet_core::data::keys::E,
            'f' => goxviet_core::data::keys::F,
            'g' => goxviet_core::data::keys::G,
            'h' => goxviet_core::data::keys::H,
            'i' => goxviet_core::data::keys::I,
            'j' => goxviet_core::data::keys::J,
            'k' => goxviet_core::data::keys::K,
            'l' => goxviet_core::data::keys::L,
            'm' => goxviet_core::data::keys::M,
            'n' => goxviet_core::data::keys::N,
            'o' => goxviet_core::data::keys::O,
            'p' => goxviet_core::data::keys::P,
            'q' => goxviet_core::data::keys::Q,
            'r' => goxviet_core::data::keys::R,
            's' => goxviet_core::data::keys::S,
            't' => goxviet_core::data::keys::T,
            'u' => goxviet_core::data::keys::U,
            'v' => goxviet_core::data::keys::V,
            'w' => goxviet_core::data::keys::W,
            'x' => goxviet_core::data::keys::X,
            'y' => goxviet_core::data::keys::Y,
            'z' => goxviet_core::data::keys::Z,
            '0' => goxviet_core::data::keys::N0,
            '1' => goxviet_core::data::keys::N1,
            '2' => goxviet_core::data::keys::N2,
            '3' => goxviet_core::data::keys::N3,
            '4' => goxviet_core::data::keys::N4,
            '5' => goxviet_core::data::keys::N5,
            '6' => goxviet_core::data::keys::N6,
            '7' => goxviet_core::data::keys::N7,
            '8' => goxviet_core::data::keys::N8,
            '9' => goxviet_core::data::keys::N9,
            _ => continue,
        };

        let _result = engine.on_key(key_code, false, false);
    }

    engine.get_buffer()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test case 1: "tan" + "a" → "tân" (Telex)
    // Clarification: Input is "taan" (t-a-a-n)
    // Sequence: t + a + a (circumflex transforms aa→â) + n → "tân"
    #[test]
    fn test_tan_a_becomes_tan_circumflex() {
        let result = type_word("taan", 0); // Telex
        assert_eq!(result, "tân", "taan should become tân");
    }

    // Test case 2: "tien" + "ef" → "tiền" (Telex)
    // Sequence: t + i + e + e (circumflex on e) + n + f (huyền tone)
    #[test]
    fn test_tien_ef_becomes_tien_huyen() {
        let result = type_word("tieenf", 0); // Telex
        assert_eq!(result, "tiền", "tieenf should become tiền");
    }

    // Test case 3: "tieesn" → "tiến" (Telex)
    // Sequence: t + i + e + e (circumflex) + s (sắc) + n
    #[test]
    fn test_tieesn_becomes_tien_sac() {
        let result = type_word("tieesn", 0); // Telex
        assert_eq!(result, "tiến", "tieesn should become tiến");
    }

    // Test case 4: "sanf" → "sàn" (Telex) - corrected from "sans"
    // Note: 's' is sắc (rising tone), 'f' is huyền (falling tone)
    // Sequence: s + a + n + f (huyền)
    #[test]
    fn test_sanf_becomes_san_huyen() {
        let result = type_word("sanf", 0); // Telex
        assert_eq!(result, "sàn", "sanf should become sàn");
    }

    // Test case 5: "tien" + "61" or "16" → "tiến" (VNI)
    // VNI: 6 = circumflex, 1 = sắc
    // Sequence: t + i + e + 6 (circumflex on e) + n + 1 (sắc)
    #[test]
    fn test_tien61_vni_becomes_tien_sac() {
        let result = type_word("tie6n1", 1); // VNI
        assert_eq!(result, "tiến", "tie6n1 (VNI) should become tiến");
    }

    #[test]
    fn test_tien16_vni_becomes_tien_sac() {
        let result = type_word("tie61n", 1); // VNI - circumflex + sắc, then n
        assert_eq!(result, "tiến", "tie61n (VNI) should become tiến");
    }

    // Test case 6: "hoa" + "f" → "hòa" (Telex)
    // Sequence: h + o + a + f (huyền on 'a')
    #[test]
    fn test_hoaf_becomes_hoa_huyen() {
        let result = type_word("hoaf", 0); // Telex
        assert_eq!(result, "hòa", "hoaf should become hòa");
    }

    // Additional test: Verify "taana" doesn't become "tana" or "tân" incorrectly
    #[test]
    fn test_taana_step_by_step() {
        let mut engine = Engine::new();
        engine.set_method(0); // Telex
        engine.set_enabled(true);

        // Step 1: Type 't'
        engine.on_key(goxviet_core::data::keys::T, false, false);
        assert_eq!(engine.get_buffer(), "t");

        // Step 2: Type 'a'
        engine.on_key(goxviet_core::data::keys::A, false, false);
        assert_eq!(engine.get_buffer(), "ta");

        // Step 3: Type 'a' (double-key, should apply circumflex)
        engine.on_key(goxviet_core::data::keys::A, false, false);
        assert_eq!(engine.get_buffer(), "tâ");

        // Step 4: Type 'n'
        engine.on_key(goxviet_core::data::keys::N, false, false);
        assert_eq!(engine.get_buffer(), "tân");

        // Step 5: Type 'a' (rejected - would create invalid syllable 'tâna')
        engine.on_key(goxviet_core::data::keys::A, false, false);
        assert_eq!(
            engine.get_buffer(),
            "tân",
            "Additional 'a' after 'tân' is rejected (invalid syllable)"
        );
    }

    // Test case 9: Verify correct Telex sequence for "câm"
    #[test]
    fn test_cam_sequence() {
        // CORRECT sequence: "caam" (c-a-a-m) → "câm"
        let result_correct = type_word("caam", 0); // Telex
        assert_eq!(result_correct, "câm", "caam should become câm");

        // With NEW FEATURE: "cama" (c-a-m-a) → "câm" (backward diacritical application)
        let result_backward = type_word("cama", 0); // Telex
        assert_eq!(
            result_backward, "câm",
            "cama should now become câm with backward application"
        );
    }

    // NEW FEATURE TEST: Backward diacritical application
    // When typing double vowel after final consonant, apply diacritical to vowel BEFORE consonant

    #[test]
    fn test_cama_becomes_cam_circumflex_step_by_step() {
        let mut engine = Engine::new();
        engine.set_method(0); // Telex
        engine.set_enabled(true);
        engine.set_modern_tone(false);

        // Step 1: Type "c"
        engine.on_key(goxviet_core::data::keys::C, false, false);
        assert_eq!(engine.get_buffer(), "c");

        // Step 2: Type "a"
        engine.on_key(goxviet_core::data::keys::A, false, false);
        assert_eq!(engine.get_buffer(), "ca");

        // Step 3: Type "m" (final consonant)
        engine.on_key(goxviet_core::data::keys::M, false, false);
        assert_eq!(engine.get_buffer(), "cam");

        // Step 4: Type "a" again - should apply circumflex to vowel BEFORE 'm'
        engine.on_key(goxviet_core::data::keys::A, false, false);
        assert_eq!(
            engine.get_buffer(),
            "câm",
            "cama should become câm with backward application"
        );
    }

    #[test]
    fn test_cama_s_becomes_cam_sac() {
        let result = type_word("camas", 0); // Telex: c-a-m-a (→câm) + s (sắc)
        assert_eq!(result, "cấm", "camas should become cấm");
    }

    #[test]
    fn test_tieme_becomes_tiem_circumflex() {
        let result = type_word("tieme", 0); // Telex: t-i-e-m-e (→tiêm)
        assert_eq!(
            result, "tiêm",
            "tieme should become tiêm with backward application"
        );
    }

    #[test]
    fn test_tono_becomes_ton_circumflex() {
        let result = type_word("tono", 0); // Telex: t-o-n-o (→tôn)
        assert_eq!(
            result, "tôn",
            "tono should become tôn with backward application"
        );
    }

    #[test]
    fn test_backward_blocked_if_vowel_has_tone() {
        let mut engine = Engine::new();
        engine.set_method(0); // Telex
        engine.set_enabled(true);

        // Type: c-a-w (makes 'ă')
        engine.on_key(goxviet_core::data::keys::C, false, false);
        engine.on_key(goxviet_core::data::keys::A, false, false);
        engine.on_key(goxviet_core::data::keys::W, false, false); // ă
        assert_eq!(engine.get_buffer(), "că");

        // Type 'm'
        engine.on_key(goxviet_core::data::keys::M, false, false);
        assert_eq!(engine.get_buffer(), "căm");

        // Type 'a' - rejected (would create invalid syllable 'căma')
        // Backward circumflex is also blocked because 'ă' already has breve tone
        engine.on_key(goxviet_core::data::keys::A, false, false);
        assert_eq!(
            engine.get_buffer(),
            "căm",
            "Additional 'a' after 'căm' is rejected (invalid syllable, vowel already has tone)"
        );
    }

    #[test]
    fn test_caam_still_works() {
        // Verify adjacent pattern still works
        let result = type_word("caam", 0);
        assert_eq!(result, "câm", "Adjacent 'aa' should still work");
    }
}
