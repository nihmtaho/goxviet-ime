//! Regression tests for English prefix additions:
//! - "mic-" prefix: "micr" / "micx" / "micf" are invalid Vietnamese → restore at SPACE
//! - "rayc-" prefix: "rayc..." is invalid Vietnamese → restore at SPACE

use goxviet_core::utils::telex;

// ── mic- prefix: micr / micx / micf → restore ─────────────────────────────

/// "micr " → 'r' absorbed as hỏi → "mỉc" (not in TuDien) → restore to "micr "
#[test]
fn test_micr_restores_at_space() {
    telex(&[("micr ", "micr ")]);
}

/// "micx " → 'x' absorbed as ngã → "mĩc" (not in TuDien) → restore to "micx "
#[test]
fn test_micx_restores_at_space() {
    telex(&[("micx ", "micx ")]);
}

/// "micf " → 'f' absorbed as huyền → "mìc" (not in TuDien) → restore to "micf "
#[test]
fn test_micf_restores_at_space() {
    telex(&[("micf ", "micf ")]);
}

/// "microphone " — longer word still starts with mic- prefix → restore
#[test]
fn test_microphone_restores_at_space() {
    telex(&[("microphone ", "microphone ")]);
}

/// "mích " (sắc on 'i', ch coda) IS a real Vietnamese word → must NOT restore
#[test]
fn test_mich_sac_kept_as_vietnamese() {
    telex(&[("michs ", "mích ")]);
}

// ── rayc- prefix: raycasting etc. → restore ───────────────────────────────

/// "raycasts " → 's' absorbed as sắc mid-word → transforms applied
/// → phonotactic finds "rayc" prefix → restore to "raycasts "
#[test]
fn test_raycasts_restores_at_space() {
    telex(&[("raycasts ", "raycasts ")]);
}

/// "raycasting " — longer word with rayc prefix → restore
#[test]
fn test_raycasting_restores_at_space() {
    telex(&[("raycasting ", "raycasting ")]);
}
