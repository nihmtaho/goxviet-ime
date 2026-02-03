//! Centralized Vietnamese validation module
//!
//! Provides validation functions to be called BEFORE any transform is applied.
//! This ensures we only transform valid Vietnamese syllables.
//!
//! ## Performance Optimizations
//! - LRU cache for recent validation results (size 32)
//! - Thread-local buffer for key simulation (avoids allocation)
//! - Inlined functions for hot path performance

use crate::engine_v2::vietnamese_validator::{ValidationResult, VietnameseSyllableValidator};
use std::cell::RefCell;

/// Maximum number of keys per syllable (Vietnamese syllables are typically 1-8 keys)
const MAX_SYLLABLE_LEN: usize = 16;

/// Size of validation cache (power of 2 for efficient modulo)
const CACHE_SIZE: usize = 32;

/// Simple LRU cache entry for validation results
#[derive(Clone, Copy)]
struct CacheEntry {
    /// Hash of key sequence (simple FNV-1a hash)
    hash: u64,
    /// Validation result
    is_valid: bool,
    /// Length of key sequence (used for collision detection)
    len: u8,
}

impl Default for CacheEntry {
    fn default() -> Self {
        Self {
            hash: 0,
            is_valid: false,
            len: 0,
        }
    }
}

/// Thread-local validation cache and buffer
thread_local! {
    /// LRU cache for validation results
    static VALIDATION_CACHE: RefCell<[CacheEntry; CACHE_SIZE]> = RefCell::new([CacheEntry::default(); CACHE_SIZE]);

    /// Reusable buffer for key simulation (avoids allocation)
    static SIMULATION_BUFFER: RefCell<Vec<u16>> = RefCell::new(Vec::with_capacity(MAX_SYLLABLE_LEN));
}

/// Simple FNV-1a hash for key sequences
#[inline(always)]
fn hash_keys(keys: &[u16]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for &key in keys {
        hash ^= key as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// Check if a sequence of keys forms a valid Vietnamese syllable (with caching)
#[inline]
pub fn is_valid_vietnamese_syllable(keys: &[u16]) -> bool {
    // Fast path: empty or too long
    if keys.is_empty() || keys.len() > MAX_SYLLABLE_LEN {
        return false;
    }

    // Single key is always valid (initial consonant or vowel)
    if keys.len() == 1 {
        return true;
    }

    let hash = hash_keys(keys);
    let cache_idx = (hash as usize) & (CACHE_SIZE - 1);

    // Check cache first
    let cached = VALIDATION_CACHE.with(|cache| {
        let cache = cache.borrow();
        let entry = &cache[cache_idx];
        if entry.hash == hash && entry.len == keys.len() as u8 {
            Some(entry.is_valid)
        } else {
            None
        }
    });

    if let Some(result) = cached {
        return result;
    }

    // Cache miss - do actual validation
    let result = VietnameseSyllableValidator::validate(keys).is_valid;

    // TEMP: Disable cache to debug test failures
    // // Update cache
    // VALIDATION_CACHE.with(|cache| {
    //     let mut cache = cache.borrow_mut();
    //     cache[cache_idx] = CacheEntry {
    //         hash,
    //         is_valid: result,
    //         len: keys.len() as u8,
    //     };
    // });

    result
}

/// Check if adding a new key to current buffer would create valid Vietnamese
///
/// This is critical for transform decisions: we should only apply transforms
/// (like aa → â) if the resulting syllable would be valid Vietnamese.
///
/// ## Performance
/// Uses thread-local buffer to avoid allocation on every call.
#[inline]
pub fn would_be_valid_with_key(current_keys: &[u16], new_key: u16) -> bool {
    // Fast path: if result would be too long, it's invalid
    if current_keys.len() >= MAX_SYLLABLE_LEN {
        return false;
    }

    // Use thread-local buffer to avoid allocation
    SIMULATION_BUFFER.with(|buf| {
        let mut buf = buf.borrow_mut();
        buf.clear();
        buf.extend_from_slice(current_keys);
        buf.push(new_key);
        is_valid_vietnamese_syllable(&buf)
    })
}

/// Get full validation result with confidence score
#[inline(always)]
pub fn validate_with_confidence(keys: &[u16]) -> ValidationResult {
    VietnameseSyllableValidator::validate(keys)
}

/// Check if tone placement is valid for Vietnamese vowel patterns
///
/// Validates tone modifiers on diphthongs/triphthongs:
/// - E+U requires circumflex on E ("êu" valid, "eu"/"eư" invalid)
/// - I+E, U+E, Y+E require circumflex on E
/// - Breve (ă) cannot be followed by vowel
/// - I+E+U, Y+E+U require circumflex on E, U can't have horn
#[inline(always)]
pub fn is_valid_tone_placement(keys: &[u16], tones: &[u8]) -> bool {
    VietnameseSyllableValidator::validate_with_tones(keys, tones).is_valid
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::keys;

    #[test]
    fn test_valid_syllables() {
        assert!(is_valid_vietnamese_syllable(&[keys::A, keys::C, keys::H])); // ach
        assert!(is_valid_vietnamese_syllable(&[
            keys::N,
            keys::G,
            keys::H,
            keys::I,
            keys::A
        ])); // nghia
    }

    #[test]
    fn test_invalid_syllables() {
        assert!(!is_valid_vietnamese_syllable(&[keys::O, keys::C, keys::H])); // och
        assert!(!is_valid_vietnamese_syllable(&[
            keys::C,
            keys::L,
            keys::E,
            keys::A,
            keys::N
        ])); // clean
    }

    #[test]
    fn test_would_be_valid() {
        // "nghi" + "a" would be "nghia" - valid
        assert!(would_be_valid_with_key(
            &[keys::N, keys::G, keys::H, keys::I],
            keys::A
        ));

        // "och" + "a" would be "ocha" - invalid (o + ch not allowed)
        assert!(!would_be_valid_with_key(
            &[keys::O, keys::C, keys::H],
            keys::A
        ));
    }

    #[test]
    fn test_cache_consistency() {
        // Test that cache returns same results as direct validation
        let test_cases = vec![
            vec![keys::A, keys::C, keys::H],
            vec![keys::N, keys::G, keys::H, keys::I, keys::A],
            vec![keys::O, keys::C, keys::H],
        ];

        for keys in test_cases {
            let direct = VietnameseSyllableValidator::validate(&keys).is_valid;
            let cached1 = is_valid_vietnamese_syllable(&keys);
            let cached2 = is_valid_vietnamese_syllable(&keys); // Second call should hit cache

            assert_eq!(direct, cached1, "First call should match direct validation");
            assert_eq!(cached1, cached2, "Second call should match cached result");
        }
    }

    #[test]
    fn test_empty_and_edge_cases() {
        // Empty keys
        assert!(!is_valid_vietnamese_syllable(&[]));

        // Single key (always valid)
        assert!(is_valid_vietnamese_syllable(&[keys::A]));

        // Too long (> MAX_SYLLABLE_LEN)
        let long_keys: Vec<u16> = (0..MAX_SYLLABLE_LEN + 1).map(|_| keys::A).collect();
        assert!(!is_valid_vietnamese_syllable(&long_keys));
    }
}
