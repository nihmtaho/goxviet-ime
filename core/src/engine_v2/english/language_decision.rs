use crate::engine_v2::english::dictionary::Dictionary;
use crate::engine_v2::english::phonotactic::PhonotacticEngine;
use crate::engine_v2::vietnamese_validator::ValidationResult;

pub struct DecisionResult {
    pub is_english: bool,
    pub confidence: u8,
}

pub struct LanguageDecisionEngine;

impl LanguageDecisionEngine {
    /// Unified decision making with Vietnamese validator integration
    /// This is the single source of truth for language detection
    pub fn decide_with_validation(
        keys: &[(u16, bool)],
        has_diacritics: bool,
        vietnamese_validator_result: Option<ValidationResult>,
    ) -> DecisionResult {
        if keys.is_empty() {
            return DecisionResult {
                is_english: false,
                confidence: 0,
            };
        }

        // PRIORITY 1: Dictionary Lookup (100% confidence) - Highest priority
        let keys_only: Vec<u16> = keys.iter().map(|(k, _)| *k).collect();
        // if Dictionary::is_vietnamese(&keys_only) {
        //     return DecisionResult {
        //         is_english: false,
        //         confidence: 100,
        //     };
        // }
        if Dictionary::is_english(&keys_only) {
            return DecisionResult {
                is_english: true,
                confidence: 100,
            };
        }

        // PRIORITY 2: Vietnamese Validator - Adjust scores based on validity
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

        // PRIORITY 3: Phonotactic Analysis
        let phonotactic = PhonotacticEngine::analyze(keys);
        english_score += phonotactic.english_confidence as i16;

        // PRIORITY 4: Diacritics Penalty
        // If the word already contains Vietnamese-specific characters (ê, ư, ơ, diacritics),
        // it is extremely unlikely to be English.
        if has_diacritics {
            english_score -= 70;
            vietnamese_score += 70;
        }

        // Final decision based on weighted scores
        let final_english_confidence = english_score.max(0).min(100) as u8;
        let is_english = english_score > vietnamese_score && final_english_confidence >= 80;

        DecisionResult {
            is_english,
            confidence: final_english_confidence,
        }
    }

    /// O(1) decision making for language detection (legacy method)
    /// Delegates to decide_with_validation without validator result
    pub fn decide(keys: &[(u16, bool)], has_diacritics: bool) -> DecisionResult {
        if keys.is_empty() {
            return DecisionResult {
                is_english: false,
                confidence: 0,
            };
        }

        // Delegate to unified decision system without validator result
        Self::decide_with_validation(keys, has_diacritics, None)
    }

    /// Early identification based on first 1-2 characters (O(1))
    /// Uses unified decision system for consistency
    pub fn identify_early(keys: &[(u16, bool)], has_diacritics: bool) -> Option<DecisionResult> {
        if keys.is_empty() {
            return None;
        }

        // Use unified decision system
        let decision = Self::decide_with_validation(keys, has_diacritics, None);

        // Only return early if we have high confidence (95%+)
        if decision.confidence >= 95 {
            Some(decision)
        } else {
            None
        }
    }
}
