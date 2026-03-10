use crate::data::viet_syllables::is_valid_vietnamese_syllable;
use crate::infrastructure::adapters::validation::english::phonotactic::PhonotacticEngine;
use crate::infrastructure::adapters::validation::vietnamese_validator::ValidationResult;

pub struct DecisionResult {
    pub is_english: bool,
    pub confidence: u8,
}

pub struct LanguageDecisionEngine;

impl LanguageDecisionEngine {
    /// Vietnamese-first language detection.
    ///
    /// Pipeline (high-level, in order):
    /// 1. **Vietnamese dictionary** – if `output_str` is in TuDien → not English.
    /// 2. **Vietnamese validator** – adjust english/vietnamese scores based on syllable validity.
    /// 3. **Phonotactic analysis** – add English confidence from phonotactic signals.
    /// 4. **Diacritics penalty** – heavy penalty for Vietnamese-specific characters.
    ///
    /// Decision: `is_english` only when `english_score > vietnamese_score && confidence >= 80`.
    pub fn decide_with_validation(
        keys: &[(u16, bool)],
        has_diacritics: bool,
        vietnamese_validator_result: Option<ValidationResult>,
        output_str: Option<&str>,
    ) -> DecisionResult {
        if keys.is_empty() {
            return DecisionResult {
                is_english: false,
                confidence: 0,
            };
        }

        // PRIORITY 1: Vietnamese Dictionary Lookup (O(1))
        // If the rendered output is a valid Vietnamese syllable, it is definitely not English.
        if let Some(output) = output_str {
            if is_valid_vietnamese_syllable(output) {
                return DecisionResult {
                    is_english: false,
                    confidence: 0,
                };
            }
        }

        // PRIORITY 2: Vietnamese Validator — adjust scores based on syllable validity
        let mut english_score = 0i16;
        let mut vietnamese_score = 0i16;

        if let Some(validation) = vietnamese_validator_result {
            if validation.is_valid {
                // Valid Vietnamese syllable → boost Vietnamese, penalize English
                vietnamese_score += 30;
                english_score -= 30;
            } else {
                // Invalid Vietnamese syllable → boost English, penalize Vietnamese
                english_score += 20;
                vietnamese_score -= 20;
            }
        }

        // PRIORITY 3: Phonotactic Analysis (keeps PhonotacticEngine for English detection)
        let phonotactic = PhonotacticEngine::analyze(keys);
        english_score += phonotactic.english_confidence as i16;

        // PRIORITY 4: Diacritics Penalty
        // If the word already has Vietnamese-specific characters (ê, ư, ơ, tone marks),
        // it is extremely unlikely to be English.
        if has_diacritics {
            english_score -= 70;
            vietnamese_score += 70;
        }

        // Final decision: English only when clearly dominant and high-confidence
        let final_english_confidence = english_score.max(0).min(100) as u8;
        let is_english = english_score > vietnamese_score && final_english_confidence >= 80;

        DecisionResult {
            is_english,
            confidence: final_english_confidence,
        }
    }

    /// Language detection without rendered output string (legacy/fallback path).
    pub fn decide(keys: &[(u16, bool)], has_diacritics: bool) -> DecisionResult {
        if keys.is_empty() {
            return DecisionResult {
                is_english: false,
                confidence: 0,
            };
        }

        Self::decide_with_validation(keys, has_diacritics, None, None)
    }

    /// Early identification based on first 1-2 characters.
    /// Only returns a result when confidence is ≥ 95%.
    pub fn identify_early(keys: &[(u16, bool)], has_diacritics: bool) -> Option<DecisionResult> {
        if keys.is_empty() {
            return None;
        }

        let decision = Self::decide_with_validation(keys, has_diacritics, None, None);

        if decision.confidence >= 95 {
            Some(decision)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::keys;

    fn k(s: &str) -> Vec<(u16, bool)> {
        s.chars()
            .filter_map(|c| {
                let key = match c {
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
                    _ => return None,
                };
                Some((key, false))
            })
            .collect()
    }

    // ── Priority 1: Vietnamese dictionary ────────────────────────────────────

    #[test]
    fn test_viet_dict_word_not_english() {
        // "trường" in TuDien → never English
        let result = LanguageDecisionEngine::decide_with_validation(
            &k("truong"),
            false,
            None,
            Some("trường"),
        );
        assert!(!result.is_english, "TuDien word should not be English");
        assert_eq!(result.confidence, 0);
    }

    #[test]
    fn test_viet_dict_word_overrides_phonotactics() {
        // "ban" looks somewhat English but is in TuDien
        let result =
            LanguageDecisionEngine::decide_with_validation(&k("ban"), false, None, Some("ban"));
        assert!(
            !result.is_english,
            "TuDien word 'ban' must not be English even if phonotactics suggest it"
        );
    }

    // ── No output_str: falls through to phonotactics ─────────────────────────

    #[test]
    fn test_english_word_without_output() {
        // Without output_str and without the old English dictionary,
        // "syntax" has 0 phonotactic confidence (no strong English phoneme signal).
        // Detection relies on providing output_str for TuDien lookup or phonotactics
        // for words with strong English-only patterns.
        let result =
            LanguageDecisionEngine::decide_with_validation(&k("syntax"), false, None, None);
        // "syntax" does not trigger the phonotactic engine → ambiguous, not English
        assert!(
            !result.is_english,
            "syntax without output_str is ambiguous (not English) under Vietnamese-first policy"
        );
    }

    // ── Diacritics penalty ────────────────────────────────────────────────────

    #[test]
    fn test_diacritics_prevent_english_detection() {
        let result = LanguageDecisionEngine::decide_with_validation(
            &k("truong"),
            true,
            None,
            None, // has_diacritics=true
        );
        assert!(
            !result.is_english,
            "Word with diacritics should not be English"
        );
    }

    // ── Empty / edge cases ───────────────────────────────────────────────────

    #[test]
    fn test_empty_keys_not_english() {
        let result = LanguageDecisionEngine::decide_with_validation(&[], false, None, None);
        assert!(!result.is_english);
        assert_eq!(result.confidence, 0);
    }

    #[test]
    fn test_decide_fallback_delegates_to_decide_with_validation() {
        let a = LanguageDecisionEngine::decide(&k("syntax"), false);
        let b = LanguageDecisionEngine::decide_with_validation(&k("syntax"), false, None, None);
        assert_eq!(a.is_english, b.is_english);
        assert_eq!(a.confidence, b.confidence);
    }
}
