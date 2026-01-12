use crate::data::keys;
use crate::engine_v2::fsm::tables::{CHAR_PROPS, PROP_VOWEL, VIETNAMESE_BIGRAMS};

pub struct ValidationResult {
    pub is_valid: bool,
    pub confidence: u8,
}

pub struct VietnameseSyllableValidator;

impl VietnameseSyllableValidator {
    /// O(1) validation of Vietnamese syllable structure
    pub fn validate(keys: &[u16]) -> ValidationResult {
        if keys.is_empty() {
            return ValidationResult {
                is_valid: true,
                confidence: 100,
            };
        }

        let len = keys.len();

        // Rule 1: No invalid initials (F, J, W, Z)
        let first = keys[0];
        if first < 128
            && (CHAR_PROPS[first as usize] & crate::engine_v2::fsm::tables::PROP_INITIAL_INVALID)
                != 0
        {
            return ValidationResult {
                is_valid: false,
                confidence: 0,
            };
        }

        // Rule 2 & 6: Bigram validation (O(1) per bigram)
        for i in 0..len.saturating_sub(1) {
            let k1 = keys[i];
            let k2 = keys[i + 1];

            if k1 < 128 && k2 < 128 {
                let allowed_next = VIETNAMESE_BIGRAMS[k1 as usize];
                if (allowed_next & (1 << k2 as u128)) == 0 {
                    // Check if it's a known vowel compound or allowed cluster
                    if !Self::is_allowed_exception(k1, k2) {
                        return ValidationResult {
                            is_valid: false,
                            confidence: 0,
                        };
                    }
                }
            }
        }

        // Rule 5: Coda validation
        let last = keys[len - 1];
        if len > 1
            && (CHAR_PROPS[last as usize] & crate::engine_v2::fsm::tables::PROP_CODA_INVALID) != 0
        {
            // Check for valid compounds like 'ng', 'nh', 'ch'
            let prev = keys[len - 2];
            if !matches!(
                (prev, last),
                (keys::N, keys::G) | (keys::N, keys::H) | (keys::C, keys::H)
            ) {
                return ValidationResult {
                    is_valid: false,
                    confidence: 0,
                };
            }
        }

        ValidationResult {
            is_valid: true,
            confidence: 100,
        }
    }

    #[inline]
    fn is_allowed_exception(k1: u16, k2: u16) -> bool {
        // Handle vowel-vowel sequences and common clusters not in bigram matrix yet
        if k1 >= 128 || k2 >= 128 {
            return true; // Unicode characters (diacritics) are definitely Vietnamese
        }

        let p1 = CHAR_PROPS[k1 as usize];
        let p2 = CHAR_PROPS[k2 as usize];

        if (p1 & PROP_VOWEL) != 0 && (p2 & PROP_VOWEL) != 0 {
            // Exceptions for vowel compounds not easily represented in bigram matrix
            return matches!((k1, k2), (keys::O, keys::O) | (keys::U, keys::U));
        }

        // Consonant-vowel check is already handled by bigram matrix for all consonants

        // Allowed consonant clusters
        matches!(
            (k1, k2),
            (keys::T, keys::R)
                | (keys::T, keys::H)
                | (keys::C, keys::H)
                | (keys::N, keys::H)
                | (keys::N, keys::G)
                | (keys::P, keys::H)
                | (keys::K, keys::H)
                | (keys::G, keys::I)
                | (keys::Q, keys::U)
        )
    }
}
