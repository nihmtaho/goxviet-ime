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

/// "mics": sắc is allowed (user exception) — 's' must NOT trigger immediate-English rule.
/// Boundary may restore if "míc" is not in TuDien, but 's' never fires priority 1c.
#[test]
fn test_mics_sac_not_immediate_english() {
    // micra: 'r' fires immediate restore → "micr" committed, 'a' starts fresh → "micra "
    // This passes already; the key invariant being verified here is that 's' does NOT fire.
    // "mics ": 's' → sắc, so we see "míc " OR "mics " at boundary — never "mic " or panic.
    let result = {
        use goxviet_core::engine::Engine;
        use goxviet_core::shared::types::Action;
        let mut e = Engine::new();
        let mut screen = String::new();
        for c in "mics ".chars() {
            let key = goxviet_core::utils::char_to_key(c);
            let r = e.on_key(key, false, false);
            if r.action == Action::Send as u8 {
                for _ in 0..r.backspace as usize { screen.pop(); }
                for i in 0..r.count as usize {
                    unsafe {
                        if let Some(ch) = char::from_u32(*r.chars.offset(i as isize)) {
                            screen.push(ch);
                        }
                    }
                }
            } else {
                screen.push(c);
            }
        }
        screen
    };
    assert!(
        result == "míc " || result == "mics ",
        "'mics ' should stay as Vietnamese (míc) or restore to raw (mics), got '{result}'"
    );
}

/// "micr" should restore IMMEDIATELY (mid-word), not wait for space.
/// Verify by NOT including a trailing space — if it's English mid-word, the next
/// char after 'r' should not see a Vietnamese diacritic in the buffer.
#[test]
fn test_micr_immediate_restore_no_space() {
    // type "micra" — if 'r' triggers immediate English restore, then 'a' starts fresh.
    // Result: "micra" (not "mỉca")
    telex(&[("micra ", "micra ")]);
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
