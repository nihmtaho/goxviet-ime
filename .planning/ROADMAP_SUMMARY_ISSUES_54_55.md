# 🎯 Roadmap Summary: Issues #54 & #55

**Created**: 2026-02-05  
**Status**: Planned  
**Target Release**: v2.1.0

---

## 📌 Overview

Two critical bug fix roadmaps have been created to address high-priority issues affecting the GoxViet IME experience:

| Issue | Title | Priority | Timeline | Tasks |
|-------|-------|----------|----------|-------|
| **#55** | Tone Mark Positioning (Character Jump) | 🔴 HIGH | 18 days (Feb 10-28) | 10 tasks |
| **#54** | Character Duplication on Zen Browser | 🔴 HIGH | 15 days (Feb 10-25) | 14 tasks |

---

## 🔴 Issue #55: Tone Mark Positioning - Character Jump Issue

### Problem
When typing tone marks, characters jump unpredictably, creating a non-natural typing experience. Issue occurs on both macOS and Windows platforms.

### Example
- Typing with Telex when applying tone marks → characters shift positions
- Makes extended typing sessions frustrating and error-prone
- Affects latency and perceived responsiveness

### Root Cause (To Be Determined)
- Likely related to tone repositioning algorithm
- Full buffer re-rendering instead of incremental updates
- Inefficient state management during transformation

### Solution Approach
1. **Phase 1** (2 days): Investigation & Root Cause Analysis
   - Review tone repositioning algorithm
   - Trace buffer transformation flow
   - Create reproducible test cases

2. **Phase 2** (8 days): Implementation & Optimization
   - Redesign tone positioning algorithm
   - Optimize buffer state management
   - Add performance benchmarks

3. **Phase 3** (6 days): Review & Release
   - Code review and documentation
   - Pull request creation and merge

### Deliverables
- ✅ Root cause analysis report
- ✅ Refactored tone logic (Rust)
- ✅ Performance benchmarks
- ✅ Merged PR to develop branch

### Budget
- **Total**: $2,300 USD
- Phase 1: $500 | Phase 2: $1,500 | Phase 3: $300

---

## 🔴 Issue #54: Character Duplication on Zen Browser

### Problem
When typing "đẳng cấp" on Zen Browser v2.0.7, it outputs "dđẳng cấp/" instead.
- Character 'd' is duplicated
- Extra '/' appended at the end
- Specific to Zen Browser (Chrome, Safari work fine)

### Root Cause (To Be Determined)
- Browser-specific keyboard input handling differences
- Buffer reset not triggered properly
- Special character 'đ' (d + diacritical mark) handling edge case

### Solution Approach
1. **Phase 1** (1 day): Debugging & Environment Setup
   - Install Zen Browser v2.0.7
   - Reproduce bug consistently
   - Document console errors

2. **Phase 2** (2 days): Root Cause Analysis
   - Analyze 'đ' character processing
   - Trace buffer state changes
   - Compare browser-specific behaviors

3. **Phase 3** (5 days): Implementation & Fix
   - Add buffer validation logic
   - Fix backspace handler for browser input
   - Add Zen Browser-specific test cases

4. **Phase 4** (4 days): PR & Merge
   - Cross-browser verification
   - Documentation update
   - Pull request creation and merge

### Deliverables
- ✅ Bug reproduction steps & video
- ✅ Root cause analysis report
- ✅ Fixed buffer logic (Rust)
- ✅ Zen Browser-specific test cases
- ✅ Verification report
- ✅ Merged PR to develop branch

### Budget
- **Total**: $2,300 USD
- Phase 1: $300 | Phase 2: $600 | Phase 3: $1,200 | Phase 4: $200

---

## 📊 Task Breakdown

### Issue #55: 10 Tasks

**Phase 1 - Investigation (3 tasks)**
- task_001: Review Tone Repositioning Algorithm (4h)
- task_002: Trace Buffer Transformation Flow (3h)
- task_003: Create Test Cases Reproducing Issue (3h)

**Phase 2 - Implementation (6 tasks)**
- task_004: Redesign Tone Repositioning Algorithm (8h)
- task_005: Optimize Buffer State Management (6h)
- task_006: Add Performance Benchmarks (3h)
- task_007: Run Full Test Suite (2h)
- task_008: Manual Testing (4h)

**Phase 3 - Review (2 tasks)**
- task_009: Update Documentation (2h)
- task_010: Create Pull Request (1h)

**Total Estimated Effort**: ~36 hours

---

### Issue #54: 14 Tasks

**Phase 1 - Debugging (3 tasks)**
- task_001: Install Zen Browser v2.0.7 (1h)
- task_002: Reproduce Bug Consistently (2h)
- task_003: Document Browser Console Errors (1h)

**Phase 2 - Analysis (3 tasks)**
- task_004: Analyze 'đ' Character Handling (4h)
- task_005: Trace Buffer State for 'đẳng' (3h)
- task_006: Compare Browser Behaviors (3h)

**Phase 3 - Implementation (6 tasks)**
- task_007: Add Buffer Validation Check (6h)
- task_008: Fix Backspace Handler (4h)
- task_009: Add Zen Browser Test Cases (4h)
- task_010: Type Test Sequences (2h)
- task_011: Test Other Vietnamese Words (2h)
- task_012: Cross-browser Testing (3h)

**Phase 4 - PR & Merge (2 tasks)**
- task_013: Update CHANGELOG (1h)
- task_014: Create Pull Request (1h)

**Total Estimated Effort**: ~41 hours

---

## 🎯 Critical Path

### Issue #55 Critical Path
1. task_001 → task_002 → task_003 → task_004 → task_005 → task_006 → task_007 → task_008 → task_009 → task_010

**Critical Duration**: 36 hours (estimated 18 calendar days with other work)

### Issue #54 Critical Path
1. task_001 → task_002 → task_003 → task_004 → task_005 → task_006 → task_007 → task_008 → task_009 → task_010 → task_011 → task_012 → task_013 → task_014

**Critical Duration**: 41 hours (estimated 15 calendar days with other work)

---

## 📅 Timeline

```
WEEK 1 (Feb 10-14)
├─ Issue #55: Phase 1 Investigation (Days 1-2)
├─ Issue #54: Phase 1 Debugging (Day 1)
└─ Issue #54: Phase 2 Analysis (Days 2-3)

WEEK 2 (Feb 17-21)
├─ Issue #55: Phase 2 Implementation (Days 5-8)
├─ Issue #54: Phase 3 Implementation (Days 5-9)
└─ Issue #55: Phase 2 Testing (Days 8-10)

WEEK 3 (Feb 24-28)
├─ Issue #55: Phase 3 Review & Merge (Days 12-14)
└─ Issue #54: Phase 4 PR & Merge (Days 12-15)
```

---

## 🏆 Success Criteria

### Issue #55 Success
- ✅ Character jump eliminated when typing tone marks
- ✅ Tone repositioning latency < 1ms
- ✅ All existing tests pass
- ✅ Manual testing on macOS & Windows successful
- ✅ PR merged to develop branch

### Issue #54 Success
- ✅ Typing "đẳng cấp" produces correct output (no duplication)
- ✅ No character duplication on Zen Browser
- ✅ No regression on Chrome, Safari, Firefox
- ✅ Zen Browser-specific tests added and passing
- ✅ PR merged to develop branch

---

## 📂 File Locations

Both roadmaps are stored in the `.planning/` directory:

```
.planning/
├─ roadmap_issue_55_tone_repositioning_20260205_193900.json
├─ roadmap_issue_54_zen_browser_20260205_193900.json
└─ ROADMAP_SUMMARY_ISSUES_54_55.md (this file)
```

---

## 🚀 Next Steps

1. **This Week**:
   - [ ] Review roadmaps with team
   - [ ] Confirm timeline and resource allocation
   - [ ] Set up development environment

2. **Feb 10 Start**:
   - [ ] Begin Phase 1 investigation on Issue #55
   - [ ] Begin Phase 1 debugging on Issue #54
   - [ ] Create feature branches

3. **Weekly**:
   - [ ] Update task status in roadmap files
   - [ ] Track progress against milestones
   - [ ] Escalate blockers immediately

---

## 📞 Contact & Ownership

- **Owner**: nihmtaho
- **Related Issues**: #55, #54
- **Target Version**: v2.1.0
- **Last Updated**: 2026-02-05

---

**Version**: 1.0.0  
**Status**: 🟡 Planned (Ready for execution)
