//! Regression tests for circumflex blocking when a vowel in the buffer already
//! carries a tone mark or diacritical (Bug fix: V1+tone+V2+V2 pattern).
//!
//! In Telex, doubling a vowel key (aa→â, ee→ê, oo→ô) should NOT apply circumflex
//! when another vowel in the same buffer already has a tone mark or diacritical.
//!
//! ## Bug scenario
//! - "tafoo" (t + a + f[huyền] + o + o): second 'o' should NOT become ô because 'à'
//!   already holds the huyền mark → output must be "tàoo" not "tàô".
//! - Same issue applies with multi-char consonant clusters (ch, tr, ng) and tone-
//!   repositioned sequences (mufaa where the ua diphthong moves the mark to 'a').

use goxviet_core::utils::telex;

// ── Bug 1: simple onset + V1+tone+V2+V2 ──────────────────────────────────────

#[test]
fn test_tafoo_outputs_taoo_huyen() {
    // t + a + f(huyền) + o + o: circumflex on 'o' must be blocked (à has huyền)
    telex(&[("tafoo", "tàoo")]);
}

#[test]
fn test_tasoo_outputs_taoo_sac() {
    // t + a + s(sắc) + o + o: same block for sắc
    telex(&[("tasoo", "táoo")]);
}

// ── Bug 2: multi-char consonant clusters ─────────────────────────────────────

#[test]
fn test_chaofo_outputs_chaoo() {
    // ch (onset) + ao + f(huyền) + o: second 'o' must not become ô
    telex(&[("chaofo", "chàoo")]);
}

#[test]
fn test_trafoo_outputs_traoo() {
    // tr (onset) + a + f(huyền) + oo: circumflex blocked
    telex(&[("trafoo", "tràoo")]);
}

#[test]
fn test_ngaofo_outputs_ngaoo() {
    // ng (onset) + ao + f(huyền) + o: circumflex blocked
    telex(&[("ngaofo", "ngàoo")]);
}

// ── Bug 2: tone repositioning (ua diphthong) ─────────────────────────────────

#[test]
fn test_mufaa_outputs_muaa() {
    // m + u + f(huyền on 'u') + a + a: second 'a' must NOT become â.
    // In Vietnamese, "mùa" keeps the tone on 'u' (correct phonology for the 'ua' vowel
    // in open syllable). The circumflex block fires because 'ù' already has a mark →
    // second 'a' is appended raw → "mùaa" (not "muầ" which was the pre-fix bug).
    telex(&[("mufaa", "mùaa")]);
}

// ── Regression: no prior tone → circumflex still applies ─────────────────────

#[test]
fn test_taa_still_produces_circumflex() {
    // t + a + a: no prior tone/mark → circumflex must still apply → "tâ"
    telex(&[("taa", "tâ")]);
}

#[test]
fn test_oo_alone_produces_circumflex() {
    // o + o: no prior tone → "ô"
    telex(&[("oo", "ô")]);
}

#[test]
fn test_ee_alone_produces_circumflex() {
    telex(&[("ee", "ê")]);
}

// ── Regression: backward circumflex must still work ──────────────────────────

#[test]
fn test_daua_still_becomes_dau_circumflex() {
    // d + a + u + a: backward circumflex from 'a' through 'u' → "dâu"
    telex(&[("daua", "dâu")]);
}

#[test]
fn test_cama_still_becomes_cam_circumflex() {
    // c + a + m + a: backward circumflex through final consonant 'm' → "câm"
    telex(&[("cama", "câm")]);
}

// ── Regression: existing Vietnamese words must not regress ───────────────────

#[test]
fn test_vieets_still_produces_viet() {
    // v + i + ee(→ê) + t + s(sắc) → "viết"
    telex(&[("vieets", "viết")]);
}

#[test]
fn test_hoa_no_transform() {
    // h + o + a: 'a' single press (no doubling) → no circumflex → "hoa"
    telex(&[("hoa", "hoa")]);
}
