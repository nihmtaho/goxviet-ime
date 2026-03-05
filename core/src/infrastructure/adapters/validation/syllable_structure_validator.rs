//! PAD/NA/PAC Syllable Structure Validator
//!
//! Validates Vietnamese syllable structure using the GhepVan.ini phonotactic model.
//! Three initial-consonant groups (PAD), six vowel groups (NA), and three final-consonant
//! groups (PAC) with explicit compatibility tables extracted from GhepVan.ini.
//!
//! ## Model summary (GhepVan.ini)
//!
//! ```text
//! PAD.0 = b d đ g gh m n nh p ph r s t tr v
//! PAD.1 = c h k kh qu th
//! PAD.2 = ch gi l ng ngh x
//!
//! NA.0 = ê i ua uê uy y
//! NA.1 = a iê oa uyê yê
//! NA.2 = â ă e o oo ô ơ oe u ư uâ uô ươ
//! NA.3 = oă
//! NA.4 = uơ
//! NA.5 = ai ao au âu ay ây eo êu ia iêu iu oai oao oay oeo oi ôi ơi ưa uây ui ưi uôi ươi ươu ưu uya uyu yêu
//!
//! PAC.0 = ch nh
//! PAC.1 = c ng
//! PAC.2 = m n p t
//!
//! PAD_NA.0 = 0 1 2 5        (PAD.0 → NA 0,1,2,5)
//! PAD_NA.1 = 0 1 2 3 4 5    (PAD.1 → all NA)
//! PAD_NA.2 = 0 1 2 3 5      (PAD.2 → NA 0,1,2,3,5; NOT NA.4)
//!
//! NA_PAC.0 = 0 2             (NA.0 → PAC 0,2; or open)
//! NA_PAC.1 = 0 1 2           (NA.1 → all PAC; or open)
//! NA_PAC.2 = 1 2             (NA.2 → PAC 1,2; or open)
//! NA_PAC.3 = 1 2             (NA.3 → PAC 1,2; or open)
//! NA_PAC.4 =                 (NA.4 → open only)
//! NA_PAC.5 =                 (NA.5 → open only; diphthongs)
//! ```

use crate::domain::{
    entities::syllable::Syllable,
    ports::validation::syllable_validator::{quick, SyllableValidator},
    value_objects::validation_result::{ValidationError, ValidationResult},
};

// ── PAD groups (initial consonants) ──────────────────────────────────────────

const PAD_0: &[&str] = &[
    "b", "d", "đ", "g", "gh", "m", "n", "nh", "p", "ph", "r", "s", "t", "tr", "v",
];
const PAD_1: &[&str] = &["c", "h", "k", "kh", "qu", "th"];
const PAD_2: &[&str] = &["ch", "gi", "l", "ng", "ngh", "x"];

const PAD_GROUPS: &[&[&str]] = &[PAD_0, PAD_1, PAD_2];

// ── NA groups (vowel nuclei) ──────────────────────────────────────────────────

const NA_0: &[&str] = &["ê", "i", "ua", "uê", "uy", "y"];
const NA_1: &[&str] = &["a", "iê", "oa", "uyê", "yê"];
const NA_2: &[&str] = &[
    "â", "ă", "e", "o", "oo", "ô", "ơ", "oe", "u", "ư", "uâ", "uô", "ươ",
];
const NA_3: &[&str] = &["oă"];
const NA_4: &[&str] = &["uơ"];
const NA_5: &[&str] = &[
    "ai", "ao", "au", "âu", "ay", "ây", "eo", "êu", "ia", "iêu", "iu", "oai", "oao", "oay",
    "oeo", "oi", "ôi", "ơi", "ưa", "uây", "ui", "ưi", "uôi", "ươi", "ươu", "ưu", "uya", "uyu",
    "yêu",
];

const NA_GROUPS: &[&[&str]] = &[NA_0, NA_1, NA_2, NA_3, NA_4, NA_5];

// ── PAC groups (final consonants) ────────────────────────────────────────────

const PAC_0: &[&str] = &["ch", "nh"];
const PAC_1: &[&str] = &["c", "ng"];
const PAC_2: &[&str] = &["m", "n", "p", "t"];

const PAC_GROUPS: &[&[&str]] = &[PAC_0, PAC_1, PAC_2];

// ── Compatibility tables ──────────────────────────────────────────────────────

/// Which NA groups each PAD group may precede (from PAD_NA.* in GhepVan.ini).
const PAD_NA_COMPAT: &[&[u8]] = &[
    &[0, 1, 2, 5],       // PAD.0
    &[0, 1, 2, 3, 4, 5], // PAD.1
    &[0, 1, 2, 3, 5],    // PAD.2
];

/// Which PAC groups each NA group may be followed by (from NA_PAC.* in GhepVan.ini).
/// An empty slice means only open syllables are allowed (no final consonant).
const NA_PAC_COMPAT: &[&[u8]] = &[
    &[0, 2],    // NA.0
    &[0, 1, 2], // NA.1
    &[1, 2],    // NA.2
    &[1, 2],    // NA.3
    &[],        // NA.4 — open only
    &[],        // NA.5 — open only (diphthongs/triphthongs)
];

// ── Lookup helpers ────────────────────────────────────────────────────────────

fn find_pad_group(initial: &str) -> Option<u8> {
    for (idx, group) in PAD_GROUPS.iter().enumerate() {
        if group.contains(&initial) {
            return Some(idx as u8);
        }
    }
    None
}

fn find_na_group(vowel: &str) -> Option<u8> {
    for (idx, group) in NA_GROUPS.iter().enumerate() {
        if group.contains(&vowel) {
            return Some(idx as u8);
        }
    }
    None
}

fn find_pac_group(final_c: &str) -> Option<u8> {
    for (idx, group) in PAC_GROUPS.iter().enumerate() {
        if group.contains(&final_c) {
            return Some(idx as u8);
        }
    }
    None
}

// ── Public helpers for use in transformation pipeline ────────────────────────

/// Check if a vowel cluster string is a valid Vietnamese vowel nucleus (belongs to any NA group).
pub fn is_valid_vowel_cluster(vowel_str: &str) -> bool {
    find_na_group(vowel_str).is_some()
}

/// Check if vowel cluster + coda combination is phonotactically valid (NA-PAC compatibility).
pub fn is_valid_na_pac_combo(vowel_str: &str, coda_str: &str) -> bool {
    let na_group = match find_na_group(vowel_str) {
        Some(g) => g,
        None => return false,
    };
    let pac_group = match find_pac_group(coda_str) {
        Some(g) => g,
        None => return false,
    };
    let allowed = NA_PAC_COMPAT[na_group as usize];
    !allowed.is_empty() && allowed.contains(&pac_group)
}

// ── Validator ────────────────────────────────────────────────────────────────

/// PAD/NA/PAC syllable structure validator
///
/// Validates Vietnamese syllable structure using the phonotactic model from GhepVan.ini.
/// Replaces the FSM-based validator with an explicit, data-driven approach that is
/// easier to audit, extend, and test.
///
/// # Validation steps
///
/// 1. Vowel nucleus must be present.
/// 2. Initial consonant, if present, must belong to a known PAD group.
/// 3. Vowel must belong to a known NA group.
/// 4. PAD–NA compatibility is checked using the `PAD_NA_COMPAT` table.
/// 5. Final consonant, if present, must belong to a known PAC group.
/// 6. NA–PAC compatibility is checked using the `NA_PAC_COMPAT` table.
/// 7. Tone–final consonant rule: stop finals (p t c ch) only allow Sắc/Nặng.
#[derive(Debug, Clone, Copy, Default)]
pub struct SyllableStructureValidator;

impl SyllableStructureValidator {
    pub fn new() -> Self {
        Self
    }
}

impl SyllableValidator for SyllableStructureValidator {
    fn validate(&self, syllable: &Syllable) -> ValidationResult {
        // Rule 1: vowel nucleus is required
        if !quick::has_vowel(syllable) {
            return ValidationResult::invalid(ValidationError::Empty);
        }

        let initial = syllable.initial().as_str();
        let vowel = syllable.vowel().as_str();
        let final_c = syllable.final_consonant().as_str();

        // Rule 3: vowel must be in a known NA group
        let na_group = match find_na_group(vowel) {
            Some(g) => g,
            None => {
                return ValidationResult::invalid(ValidationError::InvalidVowel {
                    vowel: vowel.to_string(),
                    context: "vowel not found in any NA group".to_string(),
                });
            }
        };

        // Rule 2 + 4: initial consonant group and PAD–NA compatibility
        if !initial.is_empty() {
            let pad_group = match find_pad_group(initial) {
                Some(g) => g,
                None => {
                    return ValidationResult::invalid(ValidationError::InvalidConsonant {
                        consonant: initial.to_string(),
                        context: "initial consonant not found in any PAD group".to_string(),
                    });
                }
            };

            if !PAD_NA_COMPAT[pad_group as usize].contains(&na_group) {
                return ValidationResult::invalid(ValidationError::PhonotacticViolation {
                    rule: format!("PAD.{} cannot precede NA.{}", pad_group, na_group),
                    context: format!("initial='{}' vowel='{}'", initial, vowel),
                });
            }
        }

        // Rule 5 + 6: final consonant group and NA–PAC compatibility
        if !final_c.is_empty() {
            let pac_group = match find_pac_group(final_c) {
                Some(g) => g,
                None => {
                    return ValidationResult::invalid(ValidationError::InvalidConsonant {
                        consonant: final_c.to_string(),
                        context: "final consonant not found in any PAC group".to_string(),
                    });
                }
            };

            let allowed = NA_PAC_COMPAT[na_group as usize];
            if allowed.is_empty() {
                return ValidationResult::invalid(ValidationError::PhonotacticViolation {
                    rule: format!("NA.{} does not allow final consonants", na_group),
                    context: format!("vowel='{}' final='{}'", vowel, final_c),
                });
            }
            if !allowed.contains(&pac_group) {
                return ValidationResult::invalid(ValidationError::PhonotacticViolation {
                    rule: format!("NA.{} cannot be followed by PAC.{}", na_group, pac_group),
                    context: format!("vowel='{}' final='{}'", vowel, final_c),
                });
            }
        }

        // Rule 7: stop final consonants only allow Sắc/Nặng tones
        if !quick::is_valid_tone_final(syllable.tone(), final_c) {
            return ValidationResult::invalid(ValidationError::InvalidTonePlacement {
                syllable: syllable.base_form(),
                reason: format!(
                    "stop consonant '{}' cannot have {:?} tone",
                    final_c,
                    syllable.tone()
                ),
            });
        }

        ValidationResult::valid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::tone::ToneType;

    fn v() -> SyllableStructureValidator {
        SyllableStructureValidator::new()
    }

    // ── Valid syllables ───────────────────────────────────────────────────────

    #[test]
    fn valid_truong() {
        // tr (PAD.0) + ươ (NA.2) + ng (PAC.1) — PAD_NA.0 allows NA.2 ✓, NA_PAC.2 allows PAC.1 ✓
        let s = Syllable::from_parts("tr", "ươ", "ng", ToneType::Huyen);
        assert!(v().validate(&s).is_valid(), "trường should be valid");
    }

    #[test]
    fn valid_tieng() {
        // t (PAD.0) + iê (NA.1) + ng (PAC.1) — PAD_NA.0 allows NA.1 ✓, NA_PAC.1 allows PAC.1 ✓
        let s = Syllable::from_parts("t", "iê", "ng", ToneType::Sac);
        assert!(v().validate(&s).is_valid(), "tiếng should be valid");
    }

    #[test]
    fn valid_open_ha() {
        // h (PAD.1) + a (NA.1) + open — PAD_NA.1 allows all NA ✓, open syllable ✓
        let s = Syllable::from_parts("h", "a", "", ToneType::Ngang);
        assert!(v().validate(&s).is_valid(), "ha should be valid");
    }

    #[test]
    fn valid_vowel_initial_an() {
        // no PAD + a (NA.1) + n (PAC.2) — vowel-initial ✓, NA_PAC.1 allows PAC.2 ✓
        let s = Syllable::from_parts("", "a", "n", ToneType::Ngang);
        assert!(v().validate(&s).is_valid(), "an should be valid");
    }

    #[test]
    fn valid_cap_sac() {
        // c (PAD.1) + â (NA.2) + p (PAC.2) — tone Sắc with stop ✓
        let s = Syllable::from_parts("c", "â", "p", ToneType::Sac);
        assert!(v().validate(&s).is_valid(), "cấp should be valid");
    }

    #[test]
    fn valid_cap_nang() {
        let s = Syllable::from_parts("c", "â", "p", ToneType::Nang);
        assert!(v().validate(&s).is_valid(), "cập should be valid");
    }

    #[test]
    fn valid_anh() {
        // no PAD + a (NA.1) + nh (PAC.0) — NA_PAC.1 allows PAC.0 ✓
        let s = Syllable::from_parts("", "a", "nh", ToneType::Ngang);
        assert!(v().validate(&s).is_valid(), "anh should be valid");
    }

    #[test]
    fn valid_hoa() {
        // h (PAD.1) + oa (NA.1) + open — PAD_NA.1 allows NA.1 ✓, open ✓
        let s = Syllable::from_parts("h", "oa", "", ToneType::Ngang);
        assert!(v().validate(&s).is_valid(), "hoa should be valid");
    }

    #[test]
    fn valid_khong() {
        // kh (PAD.1) + o (NA.2) + ng (PAC.1) — PAD_NA.1 all ✓, NA_PAC.2 allows PAC.1 ✓
        let s = Syllable::from_parts("kh", "o", "ng", ToneType::Ngang);
        assert!(v().validate(&s).is_valid(), "không should be valid");
    }

    #[test]
    fn valid_nguoi() {
        // ng (PAD.2) + ươi (NA.5) + open — PAD_NA.2 allows NA.5 ✓, NA_PAC.5 allows open ✓
        let s = Syllable::from_parts("ng", "ươi", "", ToneType::Ngang);
        assert!(v().validate(&s).is_valid(), "người should be valid");
    }

    #[test]
    fn valid_chi() {
        // ch (PAD.2) + i (NA.0) + open — PAD_NA.2 allows NA.0 ✓
        let s = Syllable::from_parts("ch", "i", "", ToneType::Ngang);
        assert!(v().validate(&s).is_valid(), "chi should be valid");
    }

    #[test]
    fn valid_ghe() {
        // gh (PAD.0) + ê (NA.0) + open — PAD_NA.0 allows NA.0 ✓
        let s = Syllable::from_parts("gh", "ê", "", ToneType::Ngang);
        assert!(v().validate(&s).is_valid(), "ghê should be valid");
    }

    #[test]
    fn valid_ngh_e() {
        // ngh (PAD.2) + e (NA.2) + open — PAD_NA.2 allows NA.2 ✓
        let s = Syllable::from_parts("ngh", "e", "", ToneType::Ngang);
        assert!(v().validate(&s).is_valid(), "nghe should be valid");
    }

    #[test]
    fn valid_quit() {
        // qu (PAD.1) + i (NA.0) + t (PAC.2) — PAD_NA.1 all ✓, NA_PAC.0 allows PAC.2 ✓
        let s = Syllable::from_parts("qu", "i", "t", ToneType::Sac);
        assert!(v().validate(&s).is_valid(), "quít should be valid");
    }

    #[test]
    fn valid_na3_oan() {
        // no PAD + oă (NA.3) + n (PAC.2) — NA_PAC.3 allows PAC.2 ✓
        let s = Syllable::from_parts("", "oă", "n", ToneType::Ngang);
        assert!(v().validate(&s).is_valid(), "oăn should be valid");
    }

    #[test]
    fn valid_na4_open() {
        // no PAD + uơ (NA.4) + open — NA_PAC.4 is open only ✓
        let s = Syllable::from_parts("", "uơ", "", ToneType::Ngang);
        assert!(v().validate(&s).is_valid(), "uơ (open) should be valid");
    }

    // ── Invalid: missing vowel ────────────────────────────────────────────────

    #[test]
    fn invalid_no_vowel() {
        let s = Syllable::from_parts("tr", "", "ng", ToneType::Ngang);
        let r = v().validate(&s);
        assert!(r.is_invalid());
        assert_eq!(r.error(), Some(&ValidationError::Empty));
    }

    // ── Invalid: bad initial ──────────────────────────────────────────────────

    #[test]
    fn invalid_initial_bl() {
        // "bl" is not a Vietnamese consonant
        let s = Syllable::from_parts("bl", "a", "", ToneType::Ngang);
        let r = v().validate(&s);
        assert!(r.is_invalid(), "bl is not a valid initial");
    }

    #[test]
    fn invalid_initial_kr() {
        let s = Syllable::from_parts("kr", "a", "", ToneType::Ngang);
        assert!(v().validate(&s).is_invalid());
    }

    // ── Invalid: bad vowel ────────────────────────────────────────────────────

    #[test]
    fn invalid_unknown_vowel() {
        let s = Syllable::from_parts("b", "xyz", "", ToneType::Ngang);
        let r = v().validate(&s);
        assert!(r.is_invalid());
        assert!(matches!(r.error(), Some(ValidationError::InvalidVowel { .. })));
    }

    // ── Invalid: bad final ────────────────────────────────────────────────────

    #[test]
    fn invalid_final_b() {
        // "b" is not a valid Vietnamese final consonant
        let s = Syllable::from_parts("", "a", "b", ToneType::Ngang);
        assert!(v().validate(&s).is_invalid());
    }

    // ── Invalid: PAD–NA incompatibility ──────────────────────────────────────

    #[test]
    fn invalid_pad0_na4() {
        // b (PAD.0) + uơ (NA.4) — PAD_NA.0 does NOT allow NA.4
        let s = Syllable::from_parts("b", "uơ", "", ToneType::Ngang);
        let r = v().validate(&s);
        assert!(r.is_invalid(), "b+uơ should be invalid (PAD.0 cannot precede NA.4)");
        assert!(matches!(r.error(), Some(ValidationError::PhonotacticViolation { .. })));
    }

    #[test]
    fn invalid_pad2_na4() {
        // ch (PAD.2) + uơ (NA.4) — PAD_NA.2 does NOT allow NA.4
        let s = Syllable::from_parts("ch", "uơ", "", ToneType::Ngang);
        let r = v().validate(&s);
        assert!(r.is_invalid(), "ch+uơ should be invalid (PAD.2 cannot precede NA.4)");
    }

    // ── Invalid: NA–PAC incompatibility ──────────────────────────────────────

    #[test]
    fn invalid_na4_with_final() {
        // uơ (NA.4) + n — NA_PAC.4 is open only
        let s = Syllable::from_parts("", "uơ", "n", ToneType::Ngang);
        let r = v().validate(&s);
        assert!(r.is_invalid(), "uơ+n should be invalid (NA.4 allows no PAC)");
    }

    #[test]
    fn invalid_na5_with_final() {
        // ai (NA.5) + n — NA_PAC.5 is open only
        let s = Syllable::from_parts("", "ai", "n", ToneType::Ngang);
        let r = v().validate(&s);
        assert!(r.is_invalid(), "ai+n should be invalid (NA.5 allows no PAC)");
    }

    #[test]
    fn invalid_na0_pac1() {
        // i (NA.0) + c (PAC.1) — NA_PAC.0 allows PAC.0 and PAC.2, NOT PAC.1
        let s = Syllable::from_parts("", "i", "c", ToneType::Ngang);
        let r = v().validate(&s);
        assert!(r.is_invalid(), "i+c should be invalid (NA.0 does not allow PAC.1)");
    }

    #[test]
    fn invalid_na2_pac0() {
        // o (NA.2) + ch (PAC.0) — NA_PAC.2 allows PAC.1 and PAC.2, NOT PAC.0
        let s = Syllable::from_parts("", "o", "ch", ToneType::Ngang);
        let r = v().validate(&s);
        assert!(r.is_invalid(), "o+ch should be invalid (NA.2 does not allow PAC.0)");
    }

    // ── Invalid: tone–final rule ──────────────────────────────────────────────

    #[test]
    fn invalid_stop_final_hoi_tone() {
        // c + â + p (PAC.2) + Hỏi — stop consonant cannot have hỏi
        let s = Syllable::from_parts("c", "â", "p", ToneType::Hoi);
        let r = v().validate(&s);
        assert!(r.is_invalid());
        assert!(matches!(r.error(), Some(ValidationError::InvalidTonePlacement { .. })));
    }

    #[test]
    fn invalid_stop_final_huyen_tone() {
        let s = Syllable::from_parts("t", "a", "t", ToneType::Huyen);
        assert!(v().validate(&s).is_invalid());
    }
}
