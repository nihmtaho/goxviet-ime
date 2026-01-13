---
name: English Word Auto-Restore Request
about: Request an English word to be added to the auto-restore dictionary
title: '[AUTO-RESTORE] Add word: '
labels: 'enhancement, auto-restore, dictionary'
assignees: ''
---

## English Word Request

**Word to add:**
<!-- Enter the English word that should be auto-restored (e.g., "mason", "reason", "nurses") -->


**Word length:**
<!-- Number of letters (e.g., 5, 6, 7) -->


**Word category:**
<!-- Check all that apply -->
- [ ] Common English word
- [ ] Technical/programming term
- [ ] Word with double consonants (-son, -ton, -ron)
- [ ] Word with multiple modifiers (e.g., -es, -s plurals)
- [ ] Word with -tion/-sion suffix
- [ ] Other (please specify below)

## Problem Description

**Current behavior:**
<!-- Describe what happens when you type this word in Telex/VNI mode -->
<!-- Example: "When I type 'mason', it becomes 'máon' with Vietnamese tone marks" -->


**Expected behavior:**
<!-- What should happen instead? -->
<!-- Example: "The word 'mason' should remain as 'mason' without Vietnamese transforms" -->


## Additional Context

**Typing mode:**
<!-- Which mode are you using? -->
- [ ] Telex
- [ ] VNI
- [ ] All

**Frequency of use:**
<!-- How often do you use this word? -->
- [ ] Very frequently (daily)
- [ ] Frequently (weekly)
- [ ] Occasionally (monthly)
- [ ] Rarely

**Similar words:**
<!-- Are there other similar words that should also be added? -->


**Screenshots (optional):**
<!-- If applicable, add screenshots to help explain the problem -->


---

## For Developers

<!-- This section will be filled by maintainers -->

**Hash calculation:**
```bash
python3 scripts/calc_hash.py <word>
```

**Dictionary location:**
- [ ] Length 5: `is_english_len5()`
- [ ] Length 6: `is_english_len6()`
- [ ] Length 7: `is_english_len7()`
- [ ] Length 8: `is_english_len8()`

**Phonotactic pattern:**
- [ ] Requires coda pattern update
- [ ] Requires suffix pattern update
- [ ] Dictionary entry only

**Test case:**
- [ ] Added to `double_consonant_test.rs`
- [ ] Added to `english_auto_restore_test.rs`
