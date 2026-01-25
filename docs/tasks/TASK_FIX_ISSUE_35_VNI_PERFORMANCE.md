# Task: Fix Issue #35 - VNI Performance Optimization

## Overview
Optimize VNI input method performance to match Telex by replacing linear array search with binary search for tone targets.

**Estimated Effort:** 30-60 minutes  
**Priority:** High (bug + performance)  
**Target Completion:** Before next release

---

## Phase 1: Preparation (5-10 minutes)

- [ ] Read root cause analysis: `docs/VNI_PERFORMANCE_ANALYSIS.md`
- [ ] Read implementation guide: `docs/VNI_PERFORMANCE_FIX_GUIDE.md`
- [ ] Locate target arrays in code:
  ```bash
  grep -n "CIRCUMFLEX_TARGETS\|HORN_TARGETS_VNI\|BREVE_TARGETS" core/src/data/*.rs
  ```
- [ ] Confirm arrays are sorted (required for binary search)
- [ ] Locate all `targets.contains()` calls:
  ```bash
  grep -n "targets.contains" core/src/engine/mod.rs
  ```

---

## Phase 2: Implementation (15-20 minutes)

### Subtask 2.1: Modify `try_tone()` method

**File:** `core/src/engine/mod.rs`  
**Lines:** ~930-1100

- [ ] Find all `targets.contains(&c.key)` in `try_tone()` method
- [ ] Replace with `targets.binary_search(&c.key).is_ok()`
- [ ] Verify indentation and formatting correct
- [ ] Add inline comment explaining optimization:
  ```rust
  // OPTIMIZATION: Use binary_search instead of linear search
  // This improves VNI performance from O(14) to O(4) comparisons
  if targets.binary_search(&c.key).is_ok() && ... {
  ```

### Subtask 2.2: Modify `try_mark()` method

**File:** `core/src/engine/mod.rs`  
**Lines:** ~1310-1450

- [ ] Find all `targets.contains(&c.key)` in `try_mark()` method
- [ ] Replace with `targets.binary_search(&c.key).is_ok()`
- [ ] Add same optimization comment

### Subtask 2.3: Modify `find_horn_target_with_switch()` method

**File:** `core/src/engine/mod.rs`  
**Lines:** ~1472+

- [ ] Find all `targets.contains()` calls
- [ ] Replace with `targets.binary_search().is_ok()`
- [ ] Update comments

### Subtask 2.4: Code Verification

- [ ] Run: `grep -n "targets.contains" core/src/engine/mod.rs`
  - **Result should be:** 0 matches (all replaced)
- [ ] Visual inspection of all changes:
  ```bash
  git diff core/src/engine/mod.rs
  ```

---

## Phase 3: Testing (10-15 minutes)

### Subtask 3.1: Build Verification

- [ ] Build debug: `cargo build`
  - **Expected:** 0 errors, 0 warnings
- [ ] Build release: `cargo build --release`
  - **Expected:** 0 errors, 0 warnings
- [ ] Run clippy: `cargo clippy`
  - **Expected:** 0 warnings
- [ ] Check format: `cargo fmt --check`
  - **Expected:** 0 differences

### Subtask 3.2: Unit Tests

- [ ] Run: `cargo test --lib input::vni`
  - **Expected:** all tests pass ✅
- [ ] Run: `cargo test --lib engine::try_tone`
  - **Expected:** all tests pass ✅
- [ ] Run: `cargo test --lib engine::try_mark`
  - **Expected:** all tests pass ✅
- [ ] Run: `cargo test --lib`
  - **Expected:** all tests pass ✅

### Subtask 3.3: VNI Integration Tests

- [ ] Run: `cargo test vni`
  - **Expected:** all VNI tests pass ✅
- [ ] Run: `cargo test telex`
  - **Expected:** Telex tests still pass (regression check) ✅

### Subtask 3.4: Performance Benchmarks

- [ ] Run: `./test-performance.sh`
  - **Expected outputs:**
    - VNI rapid typing: <16ms/keystroke ✅
    - Telex rapid typing: 7-10ms/keystroke ✅
    - No performance regression ✅
- [ ] Take screenshot/note of before/after metrics

---

## Phase 4: Documentation (5-10 minutes)

### Subtask 4.1: Update CHANGELOG

**File:** `CHANGELOG.md`

- [ ] Add entry under `## [Unreleased]` → `### Fixed`:
  ```markdown
  - **perf**: VNI tone marking now 40-50% faster (#35)
    - Replaced linear search with binary search for tone targets
    - VNI typing latency: 15-18ms → 8-11ms
    - Achieved performance parity with Telex method
  ```

### Subtask 4.2: Document in Code

- [ ] Each change has comment explaining optimization
- [ ] Performance metrics noted in comments
- [ ] Reference to issue #35 included

### Subtask 4.3: Create Implementation Summary

- [ ] Document in: `docs/implementation_plans/issue_35_vni_performance_fix.md`
  ```markdown
  # Issue #35: VNI Performance Fix

  ## Implementation Date: [Date]
  
  ## Changes Made
  - Replaced targets.contains() with targets.binary_search() in 3 methods
  - Added optimization comments
  - Verified all tests pass
  
  ## Performance Results
  - Before: VNI 15-18ms/keystroke
  - After: VNI 8-11ms/keystroke
  - Improvement: 40-50%
  
  ## Status: ✅ COMPLETE
  ```

---

## Phase 5: Git & PR (5 minutes)

### Subtask 5.1: Stage Changes

```bash
cd /Users/nihmtaho/developer/personal-projects/cmlia/goxviet
git status
```

- [ ] Expected changes:
  - `core/src/engine/mod.rs` (modified)
  - `CHANGELOG.md` (modified)
  - `docs/implementation_plans/issue_35_vni_performance_fix.md` (new)

### Subtask 5.2: Commit

```bash
git add core/src/engine/mod.rs CHANGELOG.md docs/implementation_plans/issue_35_vni_performance_fix.md
git commit -m "perf(core): optimize vni tone target lookup using binary search

Replace linear search (O(n)) with binary search (O(log n)) for VNI
tone target matching. This reduces VNI keystroke latency from ~15-18ms
to ~8-11ms, achieving parity with Telex performance.

Changes:
- Replace targets.contains() with targets.binary_search().is_ok()
- Affected methods: try_tone(), try_mark(), find_horn_target_with_switch()
- Performance: 40-50% faster for rapid VNI tone marking
- Tested: All unit tests pass, benchmarks confirm improvement

Fixes: #35"
```

- [ ] Verify commit message follows rules:
  - [ ] Starts with lowercase ✅
  - [ ] No trailing period ✅
  - [ ] Type + scope format ✅
  - [ ] Imperative mood ✅

### Subtask 5.3: Create Pull Request (if applicable)

```bash
git push origin fix/vni-performance-binary-search
```

- [ ] PR title: `perf(core): optimize vni tone target lookup using binary search`
- [ ] PR description:
  ```markdown
  ## Description
  Optimize VNI input method performance by replacing linear array search
  with binary search for tone targets.
  
  ## Problem
  VNI typing is 2-3x slower than Telex due to larger tone target arrays
  (6-15 elements) being searched linearly.
  
  ## Solution
  Replace `targets.contains()` with `targets.binary_search().is_ok()`
  to reduce lookup from O(n) to O(log n).
  
  ## Performance
  - Before: 15-18ms/keystroke (VNI)
  - After: 8-11ms/keystroke (VNI)
  - Improvement: 40-50% faster
  
  ## Testing
  - ✅ All unit tests pass
  - ✅ VNI integration tests pass
  - ✅ Telex regression tests pass
  - ✅ Performance benchmarks confirm improvement
  
  Fixes #35
  ```
- [ ] Add labels: `performance`, `bug-fix`, `core`
- [ ] Request review from code owner

---

## Phase 6: Issue Closure (2 minutes)

### Subtask 6.1: Update Issue #35

**GitHub Issue:** https://github.com/nihmtaho/goxviet-ime/issues/35

```markdown
## Resolution: Fixed ✅

VNI performance issue has been optimized using binary search.

### Summary
Replaced linear array search with binary search for tone target matching,
reducing VNI keystroke latency from 15-18ms to 8-11ms.

### Performance Comparison
| Method | Before | After | Improvement |
|--------|--------|-------|-------------|
| Telex | 7-10ms | 7-10ms | No change |
| VNI | 15-18ms | 8-11ms | 40-50% ✅ |

### Technical Details
- Changed: `targets.contains()` → `targets.binary_search().is_ok()`
- Affected methods: try_tone(), try_mark(), find_horn_target_with_switch()
- Complexity: O(14) comparisons → O(4) comparisons

### Testing
- ✅ All unit tests pass
- ✅ VNI integration tests pass
- ✅ Performance benchmarks meet target <16ms/keystroke
- ✅ Telex unaffected (regression check)

### PR
#[PR_NUMBER] - [Link to PR]

### Notes
Binary search optimization was sufficient. If further improvement needed,
consider bitmask optimization (see docs/VNI_PERFORMANCE_ANALYSIS.md).
```

- [ ] Close issue: "Fixed by PR #[NUMBER]"
- [ ] Add `fixed` label
- [ ] Link to PR

---

## Quality Checklist (Final Verification)

- [ ] **Code Quality**
  - [ ] No compiler warnings ✅
  - [ ] Clippy passes ✅
  - [ ] Format correct ✅
  - [ ] Comments added ✅

- [ ] **Testing**
  - [ ] Unit tests pass ✅
  - [ ] Integration tests pass ✅
  - [ ] Benchmarks pass ✅
  - [ ] Regression check pass ✅

- [ ] **Documentation**
  - [ ] CHANGELOG updated ✅
  - [ ] Implementation plan created ✅
  - [ ] Code comments added ✅
  - [ ] Issue updated ✅

- [ ] **Git**
  - [ ] Commit message follows rules ✅
  - [ ] PR created and linked ✅
  - [ ] Branch name correct ✅

- [ ] **Performance**
  - [ ] VNI improved 40-50% ✅
  - [ ] Telex unaffected ✅
  - [ ] All keystroke latencies <16ms ✅

---

## Time Tracking

| Phase | Subtask | Time | Status |
|-------|---------|------|--------|
| 1 | Preparation | 5-10m | ⏳ |
| 2 | Implementation | 15-20m | ⏳ |
| 3 | Testing | 10-15m | ⏳ |
| 4 | Documentation | 5-10m | ⏳ |
| 5 | Git & PR | 5m | ⏳ |
| 6 | Issue Closure | 2m | ⏳ |
| **TOTAL** | | **45-62 min** | ⏳ |

---

## Notes

- Keep terminal output from `./test-performance.sh` for reference
- Take screenshot of performance metrics before/after
- Be prepared to explain optimization choice (binary search vs bitmask)
- If tests fail, check `docs/VNI_PERFORMANCE_ANALYSIS.md` troubleshooting section

---

## Success Criteria (Definition of Done)

- ✅ VNI keystroke latency: **8-11ms** (was 15-18ms)
- ✅ Telex latency: **7-10ms** (unchanged)
- ✅ All unit tests: **PASS**
- ✅ All integration tests: **PASS**
- ✅ Clippy warnings: **0**
- ✅ Code format: **Correct**
- ✅ Issue #35: **CLOSED**
- ✅ CHANGELOG: **UPDATED**
- ✅ PR: **CREATED & LINKED**

---

*Created: 2025*  
*Task Type: Bug Fix + Performance Optimization*  
*Difficulty: Medium (3/5)*  
*Effort: 45-62 minutes*
