//! PAD/NA/PAC syllable structure validator — integration tests
//!
//! Exercises `SyllableStructureValidator` through the public crate API,
//! covering all PAD × NA × PAC combinations that matter for correctness.

use goxviet_core::{
    domain::{
        entities::{syllable::Syllable, tone::ToneType},
        ports::validation::syllable_validator::SyllableValidator,
    },
    infrastructure::adapters::validation::SyllableStructureValidator,
};

fn val() -> SyllableStructureValidator {
    SyllableStructureValidator::new()
}

fn ok(initial: &str, vowel: &str, final_c: &str, tone: ToneType) {
    let s = Syllable::from_parts(initial, vowel, final_c, tone);
    assert!(
        val().validate(&s).is_valid(),
        "expected valid: initial='{}' vowel='{}' final='{}' {:?}",
        initial,
        vowel,
        final_c,
        tone
    );
}

fn bad(initial: &str, vowel: &str, final_c: &str, tone: ToneType) {
    let s = Syllable::from_parts(initial, vowel, final_c, tone);
    assert!(
        val().validate(&s).is_invalid(),
        "expected invalid: initial='{}' vowel='{}' final='{}' {:?}",
        initial,
        vowel,
        final_c,
        tone
    );
}

// ── PAD group coverage ────────────────────────────────────────────────────────

#[test]
fn pad0_representative_consonants() {
    // b d đ g gh m n nh p ph r s t tr v  →  NA.1 (a)
    for init in &["b", "d", "đ", "g", "m", "n", "s", "t", "tr", "v"] {
        ok(init, "a", "", ToneType::Ngang);
    }
}

#[test]
fn pad1_representative_consonants() {
    // c h k kh qu th  →  NA.1 (a), NA.4 (uơ) — PAD.1 allows all NA
    for init in &["c", "h", "k", "kh", "th"] {
        ok(init, "a", "", ToneType::Ngang);
        ok(init, "uơ", "", ToneType::Ngang);
    }
}

#[test]
fn pad2_representative_consonants() {
    // ch gi l ng ngh x  →  NA.1 (a)
    for init in &["ch", "gi", "l", "ng", "ngh", "x"] {
        ok(init, "a", "", ToneType::Ngang);
    }
}

// ── NA group coverage ─────────────────────────────────────────────────────────

#[test]
fn na0_vowels_valid() {
    for vowel in &["ê", "i", "ua", "uê", "uy", "y"] {
        // PAD.1 allows all NA groups, so use "h" as initial
        ok("h", vowel, "", ToneType::Ngang);
    }
}

#[test]
fn na1_vowels_valid() {
    for vowel in &["a", "iê", "oa", "uyê", "yê"] {
        ok("h", vowel, "", ToneType::Ngang);
    }
}

#[test]
fn na2_vowels_valid() {
    for vowel in &["â", "ă", "e", "o", "ô", "ơ", "u", "ư", "uâ", "uô", "ươ"] {
        ok("h", vowel, "", ToneType::Ngang);
    }
}

#[test]
fn na3_vowel_valid() {
    ok("h", "oă", "", ToneType::Ngang);
}

#[test]
fn na4_vowel_valid_open() {
    ok("h", "uơ", "", ToneType::Ngang);
}

#[test]
fn na5_diphthongs_valid_open() {
    for vowel in &["ai", "ao", "oi", "ôi", "ơi", "ui", "ưi", "iêu"] {
        ok("", vowel, "", ToneType::Ngang);
    }
}

// ── PAC group coverage ────────────────────────────────────────────────────────

#[test]
fn pac0_ch_nh_with_na1() {
    // NA.1 (a, iê) allows PAC.0 (ch, nh)
    // "ch" is a stop consonant → requires Sắc or Nặng
    ok("", "a", "ch", ToneType::Sac);    // "ách"
    ok("", "a", "nh", ToneType::Ngang);  // "anh"
    ok("", "iê", "ch", ToneType::Sac);   // "iếch"
    ok("", "iê", "nh", ToneType::Ngang); // "iênh"
}

#[test]
fn pac0_ch_nh_with_na0() {
    // NA.0 (ê, i) allows PAC.0 (ch, nh)
    // "ch" is a stop → Sắc; "nh" is nasal → any tone
    ok("", "ê", "ch", ToneType::Sac);    // "ếch"
    ok("", "i", "nh", ToneType::Ngang);  // "inh"
}

#[test]
fn pac1_c_ng_with_na1() {
    // "c" is a stop consonant → Sắc or Nặng; "ng" is nasal → any tone
    ok("", "a", "c", ToneType::Sac);     // "ác"
    ok("", "a", "ng", ToneType::Ngang);  // "ang"
}

#[test]
fn pac2_mnpt_with_na1() {
    for fc in &["m", "n", "p", "t"] {
        // p and t need Sắc or Nặng tone
        let tone = if *fc == "p" || *fc == "t" { ToneType::Sac } else { ToneType::Ngang };
        ok("", "a", fc, tone);
    }
}

// ── PAD–NA incompatibility ────────────────────────────────────────────────────

#[test]
fn pad0_cannot_precede_na3_oaw() {
    // PAD.0 (b d đ…) cannot precede NA.3 (oă)
    bad("b", "oă", "", ToneType::Ngang);
    bad("s", "oă", "", ToneType::Ngang);
    bad("tr", "oă", "", ToneType::Ngang);
}

#[test]
fn pad0_cannot_precede_na4_uo() {
    // PAD.0 cannot precede NA.4 (uơ)
    bad("b", "uơ", "", ToneType::Ngang);
    bad("t", "uơ", "", ToneType::Ngang);
}

#[test]
fn pad2_cannot_precede_na4_uo() {
    // PAD.2 (ch gi l ng ngh x) cannot precede NA.4 (uơ)
    bad("ch", "uơ", "", ToneType::Ngang);
    bad("ng", "uơ", "", ToneType::Ngang);
}

// ── NA–PAC incompatibility ────────────────────────────────────────────────────

#[test]
fn na0_cannot_have_pac1_c_ng() {
    // NA.0 (i ê…) + c/ng is invalid (PAC.1 not in NA_PAC.0)
    bad("", "i", "c", ToneType::Ngang);
    bad("", "ê", "ng", ToneType::Ngang);
}

#[test]
fn na2_cannot_have_pac0_ch_nh() {
    // NA.2 (o â ă…) + ch/nh is invalid (PAC.0 not in NA_PAC.2)
    bad("", "o", "ch", ToneType::Ngang);
    bad("", "â", "nh", ToneType::Ngang);
}

#[test]
fn na4_cannot_have_any_final() {
    bad("", "uơ", "n", ToneType::Ngang);
    bad("", "uơ", "ng", ToneType::Ngang);
    bad("", "uơ", "m", ToneType::Ngang);
}

#[test]
fn na5_cannot_have_any_final() {
    // Diphthong vowels are already complete — no final allowed
    bad("", "ai", "n", ToneType::Ngang);
    bad("", "ôi", "t", ToneType::Sac);
    bad("", "iêu", "m", ToneType::Ngang);
}

// ── Tone–final rule ───────────────────────────────────────────────────────────

#[test]
fn stop_finals_require_sac_or_nang() {
    for fc in &["p", "t", "c", "ch"] {
        for tone in &[ToneType::Ngang, ToneType::Huyen, ToneType::Hoi, ToneType::Nga] {
            bad("", "a", fc, *tone);
        }
        ok("", "a", fc, ToneType::Sac);
        ok("", "a", fc, ToneType::Nang);
    }
}

#[test]
fn non_stop_finals_allow_all_tones() {
    for fc in &["m", "n", "ng", "nh"] {
        for tone in &[
            ToneType::Ngang,
            ToneType::Huyen,
            ToneType::Sac,
            ToneType::Hoi,
            ToneType::Nga,
            ToneType::Nang,
        ] {
            ok("", "a", fc, *tone);
        }
    }
}

// ── Unknown / garbage inputs ──────────────────────────────────────────────────

#[test]
fn unknown_initial_rejected() {
    bad("bl", "a", "", ToneType::Ngang);
    bad("str", "a", "", ToneType::Ngang);
    bad("xyz", "a", "", ToneType::Ngang);
}

#[test]
fn unknown_vowel_rejected() {
    bad("b", "xyz", "", ToneType::Ngang);
    bad("", "ae", "", ToneType::Ngang);
}

#[test]
fn unknown_final_rejected() {
    bad("", "a", "b", ToneType::Ngang);
    bad("", "a", "d", ToneType::Ngang);
    bad("", "a", "tr", ToneType::Ngang);
}

#[test]
fn empty_vowel_rejected() {
    let s = Syllable::from_parts("b", "", "", ToneType::Ngang);
    assert!(val().validate(&s).is_invalid());
}

// ── Real Vietnamese word samples ──────────────────────────────────────────────

#[test]
fn real_words_valid() {
    // (initial, vowel, final, tone) for a set of well-known syllables
    let cases: &[(&str, &str, &str, ToneType)] = &[
        ("b", "a", "n", ToneType::Ngang),        // ban
        ("v", "iê", "t", ToneType::Nang),         // việt (stop final + nặng ✓)
        ("n", "ă", "m", ToneType::Ngang),         // năm
        ("th", "ươ", "ng", ToneType::Ngang),      // thương
        ("x", "iê", "ng", ToneType::Ngang),       // xiếng
        ("kh", "o", "ng", ToneType::Ngang),       // không
        ("", "ươi", "", ToneType::Ngang),          // ươi (open NA.5)
        ("qu", "a", "", ToneType::Ngang),          // qua
        ("gi", "a", "", ToneType::Ngang),          // gia
        ("ngh", "e", "", ToneType::Ngang),         // nghe
    ];
    for &(i, v, f, t) in cases {
        ok(i, v, f, t);
    }
}
