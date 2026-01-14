---
name: "📖 English Word Request"
about: Request an English word to be added to the auto-restore dictionary
title: '[AUTO-RESTORE] Add word: '
labels: 'enhancement, auto-restore, dictionary'
assignees: ''
---

## 🅰️ Word Information

**Word to add:**
<!-- e.g., "mason", "reason", "nurses" -->

**Category:**
- [ ] Common English word
- [ ] Technical/programming term
- [ ] Word with double consonants (`-son`, `-ton`, `-rr-`, etc.)
- [ ] Word with special suffix (`-tion`, `-ment`, `-able`)
- [ ] Other (please specify)

## 🔄 Problem Description

- **Current behavior:** <!-- e.g., "mason" becomes "máon" -->
- **Expected behavior:** <!-- e.g., "mason" stays "mason" -->
- **Typing Mode:** [ ] Telex [ ] VNI [ ] All

---

## 🛠 For Developers

> [!NOTE]
> This section is for maintainers to track implementation details.

1. **Hash calculation:**
   ```bash
   python3 scripts/calc_hash.py <word>
   ```
2. **Target Dictionary:** `is_english_len<N>()`
3. **Tests:**
   - [ ] Added to `english_auto_restore_test.rs`
   - [ ] Verified via `cargo test`
