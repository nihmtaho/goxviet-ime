# Quickstart: Validating US1–US5 Features

**Date**: 2026-04-05

This guide describes how to manually validate each feature after implementation.

---

## Prerequisites

```bash
cd core && cargo build --release
./scripts/rust_build_lib_universal_for_macos.sh
# Open platforms/macos/goxviet/goxviet.xcodeproj in Xcode and build
```

---

## US1: ESC Restore Toggle

1. Open GoxViet **Settings → General → Smart Features**.
2. Verify an "ESC Key Restore" toggle is visible and defaults to OFF.
3. Enable it.
4. Open TextEdit; type `hello` in Telex mode → observe `hẻllō` or any transformed output.
5. Press **ESC** → verify the text reverts to raw `hello`.
6. Disable the toggle; repeat step 4–5 → verify ESC passes through unchanged (TextEdit closes
   dialog or no effect on buffer).

---

## US2: Bracket Shortcuts

1. Open Settings → General; verify a "Bracket Shortcuts" toggle is visible; enable it.
2. Ensure Telex mode is active.
3. Open TextEdit; type `[` → verify `ơ` is inserted.
4. Type `]` → verify `ư` is inserted.
5. Disable the toggle; type `[` → verify literal `[` is inserted.
6. Switch to VNI mode (toggle enabled); type `[` → verify literal `[` is inserted.

---

## US3: Foreign Consonants

1. Open Settings → General; enable "Allow Foreign Consonants (z, w, j, f)".
2. Open TextEdit in Telex mode.
3. Type `wifi` → verify output is `wifi` (not `ưifi` or `wífi`).
4. Type `zoom` → verify output is `zoom`.
5. Type `jazz` → verify output is `jazz`.
6. Type `hoa` → confirm `w` after vowel still acts as horn: `hoaw` → `hoă`.
7. Disable the toggle; type `wifi` → verify English auto-restore fires (raw `wifi` or
   `ưifi` depending on existing behaviour).

---

## US4: Auto-Capitalise

1. Open Settings → General; enable "Auto-Capitalise After Sentence End".
2. Open TextEdit; type `xin chào. ` (include the space) then `t`.
3. Verify `T` is output (capitalised automatically).
4. Type `3.14 ` then `t` → verify `t` is NOT capitalised (decimal exclusion).
5. Type `v.v. ` then `t` → verify `t` is NOT capitalised (abbreviation exclusion).
6. Type `xin chào! ` then `h` → verify `H` is output.
7. Press Enter then type `n` → verify `N` is output.
8. Disable toggle; repeat steps 2–7 → verify no auto-capitalisation.

---

## US5: Word History (Backspace-After-Space)

1. Open Settings → General; enable "Backspace-After-Space Restore" (or equivalent label).
2. Open TextEdit; type `xin` + **Space**.
3. Immediately press **Backspace** → verify buffer restores to `xin` in editable state.
4. Type `chào` + **Space** + `xin` + **Space** + **Backspace** → verify `xin` buffer restores.
5. Type `hello` + **Space** + `w` (new character) + **Backspace** → verify only `w` is deleted
   (history not invoked because a non-Backspace character was typed after Space).
6. Type 10 words separated by spaces; press Backspace → verify stepping back through history.
7. Toggle IME off and on; press Backspace at word-start → verify history is cleared (no restore).

---

## Running Regression Tests

```bash
cd core

# Run all integration tests
cargo test

# Run specific test files related to these features
cargo test --test bracket_shortcuts_test
cargo test --test foreign_consonants_test
cargo test --test auto_capitalise_test
cargo test --test word_history_test
```

Each feature MUST have a failing regression test written before implementation begins
(per Constitution Principle III).
