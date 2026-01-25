# Memory Optimization Phase 5 - Validation Checklist

## ✅ Completed Optimizations

### 1. String Pooling (InputManager.swift)
- ✅ Pre-allocated pool of 180+ Vietnamese characters
- ✅ Replaced 8+ String(chars) calls with makeString(from:)
- ✅ Targets 19,097 allocations of 64B
- ✅ **Estimated savings:** ~0.5-1MB

### 2. Dictionary Disabled in Release Builds
- ✅ Wrapped dictionary checks with #[cfg(debug_assertions)]
- ✅ Lines ~466 and ~2109 in engine/mod.rs
- ✅ Phonotactic pattern engine remains active
- ✅ **Estimated savings:** ~1.4MB

### 3. Rust Buffer Audit
- ✅ RawInputBuffer: Fixed array, zero heap allocation
- ✅ Buffer: Pre-sized String::with_capacity
- ✅ rebuild.rs: Vec::with_capacity used correctly
- ✅ **No changes needed - already optimal**

### 4. Build Verification
- ✅ Rust core: Release build successful (no warnings)
- ✅ macOS app: Release build successful (clean)
- ✅ Code signing: Completed
- ✅ App registration: Completed

---

## ⏳ Validation Required

### Step 1: Launch Optimized Build
```bash
# Kill any running instances
pkill -9 goxviet

# Launch new optimized build
open /Users/nihmtaho/Library/Developer/Xcode/DerivedData/goxviet-bjjyrpbyvehbqrhjofibwspoygav/Build/Products/Release/goxviet.app
```

**Expected:** App starts normally, no crashes

---

### Step 2: Profile with Instruments
```bash
# 1. Open Instruments
open -a Instruments

# 2. Select "Allocations" template

# 3. Target: goxviet.app (from DerivedData/Release)

# 4. Start recording

# 5. Let app idle for 30 seconds (no menubar, no settings window)

# 6. Stop recording

# 7. Filter: "All Heap & Anonymous VM" → "Persistent" only

# 8. Take screenshot of:
#    - Total persistent memory (expected: ~39-41 MiB, down from 45.61 MiB)
#    - Malloc 64B count (expected: <15K, down from 19K)
#    - Malloc 32B count (should be similar)
#    - Malloc 48B count (should be similar)
```

**Success Criteria:**
- [ ] Total persistent memory: <42 MiB (was 45.61 MiB)
- [ ] Malloc 64B allocations: <15K (was 19,097)
- [ ] No dictionary-related allocations visible
- [ ] Memory stable after 30 seconds

---

### Step 3: Functional Testing

#### Test 3.1: Vietnamese + English Mixed Typing
```
Type: "tôi dùng GitHub mỗi ngày và code trên VSCode"
```
**Expected:**
- [ ] "tôi dùng" - Vietnamese transforms correctly (ô, ù, ỗ, ă, ỗ)
- [ ] "GitHub" - English preserved (phonotactic patterns detect it)
- [ ] "mỗi ngày" - Vietnamese correct
- [ ] "code trên VSCode" - Mixed typing works

#### Test 3.2: ESC Restore (String Pooling)
```
Type: "tuwf" → "từ" → press ESC
```
**Expected:**
- [ ] After ESC, shows "tuwf" (restored from raw buffer)
- [ ] Can continue typing "tuwfng" → "từng"
- [ ] No string allocation issues

#### Test 3.3: Instant Restore
```
1. Enable instant restore in settings
2. Type: "law" → becomes "lă"
3. Type: "w" again
```
**Expected:**
- [ ] Instantly restores to "law" (detects English via phonotactic)
- [ ] No dictionary lookup involved

#### Test 3.4: English Detection Without Dictionary
```
Test these words (should be detected as English without dictionary):
- "law", "saw", "wow" (phonotactic: -aw ending)
- "black", "crack", "street" (phonotactic: bl-, cr-, str- clusters)
- "checking", "thinking" (phonotactic: -ing, -ck)
```
**Expected:**
- [ ] All detected as English correctly
- [ ] No Vietnamese transforms applied
- [ ] Proves dictionary not needed for common patterns

#### Test 3.5: Shortcuts
```
1. Add shortcut: "addr" → "123 Main St, City, Country"
2. Type: "addr" + space
```
**Expected:**
- [ ] Expands to full address
- [ ] Case modes work (if configured)

#### Test 3.6: Backspace Handling
```
Type: "diễn" → Backspace x4 → should show: "diê", "di", "d", ""
```
**Expected:**
- [ ] Each backspace removes 1 grapheme
- [ ] No string pooling issues

---

### Step 4: Performance Testing
```bash
cd /Users/nihmtaho/developer/personal-projects/cmlia/goxviet
./scripts/test-editor-performance.sh
```

**Expected Results:**
- [ ] Latency <16ms per keystroke (no regression)
- [ ] Backspace <3ms (no regression)
- [ ] Memory stable during typing session
- [ ] No crashes after 1000+ keystrokes

---

### Step 5: Memory Stability Test
```
1. Launch app and let idle for 5 minutes
2. Type Vietnamese paragraph (100+ words)
3. Open menubar, close menubar
4. Open settings, close settings
5. Let idle again for 5 minutes
6. Check memory in Activity Monitor
```

**Expected:**
- [ ] Memory returns to ~25-27MB after activity
- [ ] No memory leaks (memory doesn't keep growing)
- [ ] App remains responsive

---

## 📊 Results Summary Template

After completing validation, fill this out:

### Memory Profiling Results
```
Baseline (Phase 4):           28.4 MB
After Phase 5:                ___ MB
Savings:                      ___ MB

Malloc 64B allocations:
  Before: 19,097
  After:  _______
  Reduction: ____%

Dictionary memory:
  Before: 1.41 MB
  After:  ___ MB (should be ~0 in release)
```

### Functional Testing Results
- [ ] Vietnamese typing: PASS / FAIL
- [ ] English detection: PASS / FAIL
- [ ] ESC restore: PASS / FAIL
- [ ] Instant restore: PASS / FAIL
- [ ] Shortcuts: PASS / FAIL
- [ ] Backspace: PASS / FAIL

### Performance Testing Results
- Avg latency: ___ ms (target: <16ms)
- Backspace latency: ___ ms (target: <3ms)
- Memory stable: YES / NO

### Overall Assessment
- [ ] Target <25MB achieved: YES / NO
- [ ] No regressions: YES / NO
- [ ] Ready for production: YES / NO

---

## 🚀 If Validation Passes

1. Update `docs/PERFORMANCE.md` with Phase 5 results
2. Commit changes with message:
   ```
   perf(macos): phase 5 memory optimizations - string pooling and dictionary optimization
   
   - Added String pooling for Vietnamese characters (~0.7MB savings)
   - Disabled dictionary in release builds (~1.4MB savings)
   - Verified Rust buffers already optimal
   - Total memory: 28.4MB → ~26MB (target <25MB)
   - No functional regressions
   ```
3. Tag release: `git tag v2.0.1-memory-optimized`
4. Build DMG for distribution

---

## ⚠️ If Issues Found

### High Memory (>27MB)
- Re-profile with Instruments Statistics view
- Check for new allocations we missed
- Consider lazy loading URLSession in more places

### Functional Regressions
**If English detection fails:**
- Add problematic words to whitelist manually
- Or re-enable dictionary for that specific case

**If String pooling causes issues:**
- Revert to String(chars) for specific cases
- Keep pool for common cases only

**If crashes occur:**
- Check logs: `tail -f ~/Library/Logs/GoxViet/keyboard.log`
- Debug build with dictionary enabled
- Report stack trace

---

**Created:** 2025-01-20  
**Author:** nihmtaho + Copilot  
**Build:** Release (optimized)  
**Status:** Ready for validation
