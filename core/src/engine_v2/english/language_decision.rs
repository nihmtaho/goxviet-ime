use crate::engine_v2::english::dictionary::Dictionary;
use crate::engine_v2::english::phonotactic::PhonotacticEngine;

pub struct DecisionResult {
    pub is_english: bool,
    pub confidence: u8,
}

pub struct LanguageDecisionEngine;

impl LanguageDecisionEngine {
    /// O(1) decision making for language detection
    pub fn decide(keys: &[(u16, bool)], has_diacritics: bool) -> DecisionResult {
        if keys.is_empty() {
            return DecisionResult {
                is_english: false,
                confidence: 0,
            };
        }

        // 1. Dictionary Lookup (O(1)) - Highest priority
        let keys_only: Vec<u16> = keys.iter().map(|(k, _)| *k).collect();
        if Dictionary::is_vietnamese(&keys_only) {
            return DecisionResult {
                is_english: false,
                confidence: 100,
            };
        }
        if Dictionary::is_english(&keys_only) {
            return DecisionResult {
                is_english: true,
                confidence: 100,
            };
        }

        // 2. Phonotactic Analysis (O(1))
        let mut phonotactic = PhonotacticEngine::analyze(keys);

        // 3. Diacritics Penalty
        // If the word already contains Vietnamese-specific characters (ê, ư, ơ, diacritics),
        // it is extremely unlikely to be English.
        if has_diacritics {
            phonotactic.english_confidence = phonotactic.english_confidence.saturating_sub(70);
            if phonotactic.english_confidence < 80 {
                phonotactic.is_english = false;
            }
        }

        DecisionResult {
            is_english: phonotactic.is_english,
            confidence: phonotactic.english_confidence,
        }
    }

    /// Early identification based on first 1-2 characters (O(1))
    pub fn identify_early(keys: &[(u16, bool)], has_diacritics: bool) -> Option<DecisionResult> {
        if keys.is_empty() {
            return None;
        }

        // 1. Dictionary Lookup (instant high-confidence)
        let keys_only: Vec<u16> = keys.iter().map(|(k, _)| *k).collect();
        if Dictionary::is_vietnamese(&keys_only) {
            return Some(DecisionResult {
                is_english: false,
                confidence: 100,
            });
        }
        if Dictionary::is_english(&keys_only) {
            return Some(DecisionResult {
                is_english: true,
                confidence: 100,
            });
        }

        // 2. Phonotactic Analysis
        let mut phonotactic = PhonotacticEngine::analyze(keys);

        // Diacritics Penalty (same as decide)
        if has_diacritics {
            // This corresponds to the "has_transforms penalty" in the instruction.
            // The original value was 60, which is now changed to 70 to reflect the 0.4 to 0.7 change.
            phonotactic.english_confidence = phonotactic.english_confidence.saturating_sub(70);
        }

        if phonotactic.english_confidence >= 95 {
            Some(DecisionResult {
                is_english: true,
                confidence: phonotactic.english_confidence,
            })
        } else {
            None
        }
    }
}
