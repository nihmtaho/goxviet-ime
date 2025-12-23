# BUGFIX SUMMARY - 2025-12-23

## Issue: Vietnamese Tone Marks Not Working After English Word Deletion

### Problem
When user types English word (e.g., "text", "next"), deletes all characters, then types Vietnamese, tone marks (s, f, r, x, j) don't work.

**Example:**
```
Type: "text" → Delete all → Type "co" + "s"
Expected: "cố" 
Actual: "cos" (tone mark ignored) ❌
```

### Root Cause
`is_english_word` flag persisted after buffer became empty via backspace, blocking all Vietnamese transforms.

### Solution
Reset `is_english_word = false` when buffer becomes empty during DELETE key processing.

**Changes:**
- `core/src/engine/mod.rs`:
  - Line 447-449: Reset flag in fast path (O(1) deletion)
  - Line 476-478: Reset flag in complex path (O(syllable) rebuild)
  - Line 2380-2428: Added test `test_is_english_word_reset_on_empty_buffer`

### Testing
- ✅ New test passes
- ✅ All 104 existing tests pass
- ✅ Zero performance impact
- ✅ No breaking changes

### Files Changed
1. `core/src/engine/mod.rs` - Bug fix + test
2. `docs/BUGFIX_ENGLISH_WORD_FLAG_RESET.md` - Detailed documentation
3. `COMMIT_MESSAGE_ENGLISH_FLAG_FIX.txt` - Commit message

### Commit Command
```bash
git add core/src/engine/mod.rs docs/BUGFIX_ENGLISH_WORD_FLAG_RESET.md
git commit -F COMMIT_MESSAGE_ENGLISH_FLAG_FIX.txt
```

---

**Status:** ✅ FIXED  
**Severity:** Medium  
**Version:** 1.3.1  
**Date:** 2025-12-23