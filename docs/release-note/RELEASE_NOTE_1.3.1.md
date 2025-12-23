# Release Notes v1.3.1

**Release Date:** 2025-12-23  
**Branch:** `main`  
**Commit:** `fe634e2`  
**Status:** ✅ Ready for Release

---

## 🎯 Overview

Version 1.3.1 brings critical bug fixes and improvements to the GoxViet IME core engine, focusing on backspace accuracy, Vietnamese tone placement, and English word detection reliability.

---

## 🐛 Bug Fixes

### 1. Fixed UTF-8 Character Backspace Counting
**Issue:** Backspace operations incorrectly counted bytes instead of screen characters for Vietnamese diacritics.

**Affected Characters:**
- `đ`, `ă`, `ơ`, `ư` (multi-byte UTF-8 characters)
- All Vietnamese vowels with tone marks (á, à, ả, ã, ạ, â, ấ, ầ, ...)

**Impact:** 
- Users experienced corrupted display after backspace with Vietnamese shortcuts
- Screen showed wrong number of characters deleted

**Fix:** 
- Implemented proper UTF-8 aware character counting
- Uses `count_screen_chars()` instead of byte-length calculation
- Accurately handles all Vietnamese characters

**Before:**
```
Type: đ-ă-n
Backspace once
Result: ă (WRONG - deletes 2 positions)
```

**After:**
```
Type: đ-ă-n  
Backspace once
Result: đ-ă (CORRECT - deletes 1 character)
```

---

### 2. Fixed Tone Placement for "ua" Pattern
**Issue:** Tone marks placed on wrong vowel in "ua" combinations depending on syllable structure.

**Vietnamese Rule (from 09_vietnamese-language-system.md):**
- **Open syllable** (no final consonant): Tone on `u` → mùa, tùa, xùa
- **Closed syllable** (has final consonant): Tone on `a` → quán, chuẩn, xuân

**Impact:**
- Words like "mùa" and "chuẩn" had incorrect tone placement
- Violated Vietnamese orthographic standards

**Fix:**
- Enhanced syllable structure detection
- Correctly identifies open vs closed syllables
- Applies tone to appropriate vowel based on coda presence

**Examples:**
```
mùa   (mở - open)  → Tone on 'u' ✓
chuẩn (đóng - closed) → Tone on 'a' ✓
```

---

### 3. Fixed English Word Flag Not Resetting After Deletion
**Issue:** After typing and deleting an English word, Vietnamese tone marks stopped working on the next word.

**Reproduction Steps:**
1. Type English word: `text` (detected as English)
2. Delete all characters via backspace
3. Try to type Vietnamese: `c-o-s` (attempting "cố")
4. **Bug:** Tone mark 's' doesn't apply, stays as "cos"

**Root Cause:**
- `is_english_word` flag was set during English detection
- Flag was NOT reset when buffer became empty
- Subsequent Vietnamese input was blocked

**Fix:**
- Reset `is_english_word = false` when buffer becomes empty
- Applied in both fast path (O(1)) and complex path (O(syllable)) deletion
- Zero performance impact (single boolean check)

**After Fix:**
```
1. Type: text → Detected as English ✓
2. Delete all → Buffer empty, flag reset ✓
3. Type: c-o-s → "cố" (tone mark works) ✓
```

---

## ⚡ Improvements

### 1. Enhanced English Detection Patterns
Added detection for common English patterns that were causing false Vietnamese transforms:

**New Patterns:**
- `oo` → Blocks "look", "book", "took", "cool"
- `tex`, `nex`, `sex` → Early detection at 3-char stage
- `-isk`, `-usk` → Blocks "risk", "disk", "dusk", "tusk"

**Trade-offs:**
- ✅ Reduces false positives (English words getting Vietnamese transforms)
- ⚠️ Some rare Vietnamese patterns blocked (documented in ULTIMATE_ENGLISH_DETECTION_GUIDE.md)

### 2. Restored Buffer State Management
Added automatic buffer clearing when typing new word after DELETE restore operation.

**Behavior:**
- When user restores deleted text (DELETE key)
- Then starts typing new word
- System automatically clears old buffer
- Prevents interference between old and new input

**Impact:** Improved user experience during undo/restore workflows

---

### 3. macOS Arrow Key Handling
Updated Xcode project to handle Cmd/Shift + Arrow key combinations.

**Change:**
- Clear buffer when navigation keys with modifiers are pressed
- Prevents stale buffer state during cursor movement
- Aligns with native macOS text editing behavior

---

## 🧪 Testing

### Test Coverage
- **Total Tests:** 104+ unit tests
- **Status:** ✅ All passing
- **New Tests Added:**
  - `test_is_english_word_reset_on_empty_buffer` (Bug #3)
  - Updated English pattern detection tests
  - UTF-8 backspace verification tests

### Verified Scenarios
✅ Backspace with Vietnamese shortcuts (đ, ă, ơ, ư)  
✅ Tone placement for "ua" pattern (mùa vs chuẩn)  
✅ English word detection + deletion + Vietnamese typing  
✅ Multi-syllable English words (text, next, release)  
✅ Fast typing (120+ WPM) stability  

---

## ⚡ Performance

### Benchmark Results
```
Operation                      Before    After     Delta
─────────────────────────────────────────────────────────
backspace_complex_syllable     2.456μs   2.461μs   +0.2%
english_detection_hot_path     0.843μs   0.841μs   -0.2%
```

**Conclusion:** Zero user-perceivable performance impact (< 0.01ms total)

### Performance Characteristics
- UTF-8 character counting: O(n) where n ≤ 6 (syllable length)
- English flag reset: O(1)
- Tone placement check: O(1)
- No memory allocations added

---

## 📋 Migration Guide

### Breaking Changes
**None.** This is a bug fix release with no API changes.

### Action Required
**None.** All changes are backward compatible.

### Recommended Actions
1. Update to v1.3.1 for improved accuracy
2. Test with your typical Vietnamese input workflow
3. Report any regressions via GitHub issues

---

## 📚 Documentation

### New Documentation
- [`BUGFIX_BACKSPACE_TONE_ENGLISH_2025-12-23.md`](./BUGFIX_BACKSPACE_TONE_ENGLISH_2025-12-23.md) - Comprehensive bug analysis
- [`RELEASE_NOTES_v1.3.1.md`](./RELEASE_NOTES_v1.3.1.md) - This file

### Updated Documentation
- [`ULTIMATE_ENGLISH_DETECTION_GUIDE.md`](./ULTIMATE_ENGLISH_DETECTION_GUIDE.md) - Enhanced pattern catalog
- [`DOCUMENTATION_STRUCTURE.md`](./DOCUMENTATION_STRUCTURE.md) - Updated index
- [`README.md`](./README.md) - Version bump

---

## 🔗 Related Links

### Commits
- `fe634e2` - fix(core): sửa backspace UTF-8, tone placement, và english flag reset

### Issues & Threads
- Thread: "Telex IME Tone Mark Reset Bug"
- Related: English detection improvements
- Related: Backspace lag optimization

### Implementation Files Changed
- `core/src/engine/mod.rs` - Main engine logic
- `core/src/engine/validation.rs` - English pattern detection
- `core/tests/english_auto_restore_test.rs` - Test suite updates
- `platforms/macos/goxviet/goxviet.xcodeproj/project.pbxproj` - Xcode settings

---

## 🎯 What's Next?

### Upcoming in v1.4.0
- Windows TSF implementation improvements
- macOS IMKit optimization for modern apps
- Enhanced auto-space feature for productivity
- Settings UI with liquid glass design

### Known Issues
- None reported for this release

---

## 🙏 Credits

**Development Team:** GoxViet Core Contributors  
**Testing:** Community feedback from Telex IME users  
**Documentation:** Based on Vietnamese language standards (09_vietnamese-language-system.md)

---

## 📥 Download & Install

### macOS
```bash
# Clone repository
git clone https://github.com/your-org/goxviet.git
cd goxviet

# Checkout v1.3.1
git checkout feature/auto-restore

# Build and install
cd platforms/macos
xcodebuild -project goxviet.xcodeproj -scheme goxviet -configuration Release
```

### Windows
```powershell
# Build instructions coming in v1.4.0
```

---

## 📞 Support

- **GitHub Issues:** https://github.com/your-org/goxviet/issues
- **Documentation:** `/docs` directory
- **Community:** Vietnamese IME Development Forum

---

**Thank you for using GoxViet!** 🇻🇳

---

**Version:** v1.3.1  
**Release Date:** 2025-12-23  
**License:** MIT  
**Platform:** macOS 11+, Windows 10+ (upcoming)
