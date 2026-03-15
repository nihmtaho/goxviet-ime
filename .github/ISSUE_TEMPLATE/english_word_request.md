---
name: "📖 English Word Request"
about: Request an English word to be added to the auto-restore dictionary
title: "[AUTO-RESTORE] Add word: "
labels: [enhancement, auto-restore, dictionary]
assignees: []
---

## Word

**Word:** <!-- e.g., "mason", "reason", "nurses" -->

**Problem:**
- **Current:** <!-- e.g., "mason" → "máon" -->
- **Expected:** <!-- e.g., "mason" stays "mason" -->
- **Input Method:** [ ] Telex [ ] VNI [ ] All

## For Developers

- [ ] Run `python3 scripts/calc_hash.py <word>`
- [ ] Add to `english_auto_restore_test.rs`
- [ ] Verify via `cargo test`
