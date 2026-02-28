# Sprint Patch: core-refactor-vietnamese-engine

**Feature:** core-refactor-vietnamese-engine  
**Target Milestone:** v2.0.11 (Q1 2026)  
**Branch:** `feature/core-refactor-vietnamese-engine` từ `develop`

---

## Sprint Breakdown

### Sprint A — Foundations (Pre-condition + Dictionaries)
**Goal:** Có regression tests + Vietnamese dictionaries embedded. Không thay đổi behavior.

| ID | Task | Points | Status |
|----|------|--------|--------|
| T1.1 | Regression tests trước khi refactor | 3 | 🔲 pending |
| T2.1 | Setup phf build infrastructure | 3 | 🔲 pending |
| T2.2 | Embed TuDien.txt (~7K entries) | 2 | 🔲 pending |
| T2.3 | Embed TuDienTuGhep.txt (~68K entries) | 3 | 🔲 pending |
| **Total** | | **11** | |

**Definition of Done:**
- `cargo test` pass
- phf sets build thành công
- Build time delta < 30s

---

### Sprint B — Syllable Structure
**Goal:** PAD/NA/PAC validator thay thế FSM. Behavior giữ nguyên.

| ID | Task | Points | Status |
|----|------|--------|--------|
| T3.1 | PAD/NA/PAC lookup tables | 5 | 🔲 pending |
| T3.2 | SyllableStructureValidator implementation | 5 | 🔲 pending |
| T3.3 | Wire validator, xoá FSM | 3 | 🔲 pending |
| **Total** | | **13** | |

**Depends on:** Sprint A hoàn thành  
**Definition of Done:**
- FSM không còn trong production code
- 30+ syllable tests pass
- `cargo test` pass

---

### Sprint C — Auto-Restore Refactor (Core Logic Change)
**Goal:** Vietnamese-first restore decision, English detection bị xoá.

| ID | Task | Points | Status |
|----|------|--------|--------|
| T4.1 | VietnameseFirstDecider trong application layer | 5 | 🔲 pending |
| T4.2 | Xoá English detection pipeline | 5 | 🔲 pending |
| T5.1 | Simplify Char struct | 3 | 🔲 pending |
| T5.2 | Simplify Buffer struct | 3 | 🔲 pending |
| **Total** | | **16** | |

**Depends on:** Sprint A + B hoàn thành  
**Definition of Done:**
- Không còn English detection code
- Regression tests từ T1.1 vẫn pass
- Buffer: zero-allocation confirmed

---

### Sprint D — KieuGo.ini Pattern + Integration
**Goal:** Data-driven input method + integration tests.

| ID | Task | Points | Status |
|----|------|--------|--------|
| T6.1 | InputMethodConfig type trong Rust core | 5 | ✅ done |
| T6.2 | FFI: ime_load_input_config_v2 | 3 | ✅ done |
| T6.3 | Swift binding + InputManager.swift refactor | 5 | ✅ done |
| T7.1 | End-to-end integration tests | 5 | ✅ done |
| T7.2 | Performance benchmarks | 2 | ✅ done |
| T7.3 | Documentation update | 2 | ✅ done |
| **Total** | | **22** | |

**Depends on:** Sprint C hoàn thành  
**Definition of Done:**
- `InputManager.swift` data-driven
- All integration tests pass
- Benchmarks: latency < 3ms
- Docs updated

---

## Total Story Points: 62

## Git Workflow

```bash
# Create feature branch
git checkout develop
git pull --rebase
git checkout -b feature/core-refactor-vietnamese-engine

# Sprint A
git commit -m "test(core): add regression tests for auto-restore behavior"
git commit -m "build(core): add phf build infrastructure for Vietnamese dictionaries"
git commit -m "feat(core): embed TuDien.txt as phf set for syllable validation"
git commit -m "feat(core): embed TuDienTuGhep.txt as phf set for compound words"

# Sprint B
git commit -m "feat(core): add PAD/NA/PAC syllable structure validator"
git commit -m "refactor(core): replace FSM with PAD/NA/PAC SyllableStructureValidator"

# Sprint C
git commit -m "feat(core): implement Vietnamese-first restore decision service"
git commit -m "refactor(core): remove English detection pipeline"
git commit -m "refactor(core): simplify Char and Buffer structs (gonhanh-aligned)"

# Sprint D
git commit -m "feat(core): add InputMethodConfig with Telex/VNI definitions"
git commit -m "feat(ffi): expose ime_load_input_config_v2"
git commit -m "feat(macos): refactor InputManager to data-driven KieuGo pattern"
git commit -m "test(core): add end-to-end integration tests for Vietnamese-first pipeline"
git commit -m "perf(core): benchmark Vietnamese-first pipeline"
git commit -m "docs(core): update documentation for Vietnamese-first architecture"

# Before PR: squash to meaningful commits
git rebase -i origin/develop
# → squash to 4 commits (one per sprint)

# PR: feature/core-refactor-vietnamese-engine → develop
```

---

## Checklist trước khi merge PR

- [ ] `cd core && cargo test` — tất cả tests pass
- [ ] `cd core && cargo fmt && cargo clippy` — clean
- [ ] `cd core && cargo bench` — latency < 3ms confirmed
- [ ] Regression tests (T1.1) vẫn pass
- [ ] Không còn English detection code (`grep -r "english\|phonotactic\|LanguageDecision"`)
- [ ] `InputManager.swift` không còn hardcoded input method logic
- [ ] `CHANGELOG.md` updated
- [ ] macOS app build thành công với thư viện mới
