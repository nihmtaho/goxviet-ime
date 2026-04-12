# Tasks: Implement DI Factory Functions for SOLID Architecture

**Input**: Design documents from `specs/002-solid-di-refactor/`
**Prerequisites**: plan.md ✅, spec.md ✅

**Scope**: US1 only — US2 and US3 cancelled per plan.md research findings.  
**Files changed**: `core/src/presentation/di/container.rs` (primary), `core/tests/di_container_test.rs` (new)

---

## Phase 1: Setup

**Purpose**: Confirm current state and baseline before any changes.

- [x] T001 Read and understand `core/src/presentation/di/container.rs` — verify 7 factory functions return `Arc<dyn>`, `create_processor_service` constructs adapters inline, and `#[allow(dead_code)]` annotations are present
- [x] T002 Read `core/src/application/services/processor_service.rs` lines 140–200 to confirm `ProcessorService::new()` accepts `Box<dyn>` for all 6 port parameters and confirm `HistoryTracker` is NOT in the constructor
- [x] T003 Run `cd core && cargo build --release 2>&1 | grep warning` to capture the pre-change warning baseline

---

## Phase 2: Foundational

**Purpose**: Write regression tests that capture current behavior BEFORE modifying production code (Regression-First Testing — Constitution §III).

**⚠️ CRITICAL**: Tests must be written and confirmed runnable before production code is touched.

- [x] T004 Create `core/tests/di_container_test.rs` with a table-driven test that constructs a `Container` with `InputMethodId::Telex` and processes the key `'a'` — assert no FFI panic and result is non-error (behavioral baseline)
- [x] T005 Add a second test case to `core/tests/di_container_test.rs`: construct `Container` with `InputMethodId::Vni` and process the key `'1'` — assert engine wires VNI correctly
- [x] T006 Add a third test case: call `container.update_config()` to swap from Telex to VNI, then process a key — assert the updated input method is used (rewire test)
- [x] T007 Run `cd core && cargo test di_container` — confirm tests compile and pass against the current (pre-refactor) code

**Checkpoint**: Baseline tests pass. Production code changes can now begin.

---

## Phase 3: User Story 1 — Wire DI Factory Functions (Priority: P1) 🎯 MVP

**Goal**: Replace 6 inline adapter constructions in `create_processor_service()` with calls to the named factory functions; delete `create_history_tracker`; remove all `#[allow(dead_code)]` annotations.

**Independent Test**: `cargo test di_container` passes with zero warnings after change; `cargo build --release` produces no warnings.

### Implementation for User Story 1

- [x] T008 [P] [US1] In `core/src/presentation/di/container.rs`: change `create_input_method` return type from `Arc<dyn InputMethod>` to `Box<dyn InputMethod>`, replace `Arc::new(...)` with `Box::new(...)`, remove `#[allow(dead_code)]` and the stale "Kept for potential..." comment
- [x] T009 [P] [US1] In `core/src/presentation/di/container.rs`: change `create_syllable_validator` return type from `Arc<dyn SyllableValidator>` to `Box<dyn SyllableValidator>`, replace `Arc::new(...)` with `Box::new(...)`, remove `#[allow(dead_code)]`
- [x] T010 [P] [US1] In `core/src/presentation/di/container.rs`: change `create_language_detector` return type from `Arc<dyn LanguageDetector>` to `Box<dyn LanguageDetector>`, replace `Arc::new(...)` with `Box::new(...)`, remove `#[allow(dead_code)]`
- [x] T011 [P] [US1] In `core/src/presentation/di/container.rs`: change `create_tone_transformer` return type from `Arc<dyn ToneTransformer>` to `Box<dyn ToneTransformer>`, replace `Arc::new(...)` with `Box::new(...)`, remove `#[allow(dead_code)]`
- [x] T012 [P] [US1] In `core/src/presentation/di/container.rs`: change `create_mark_transformer` return type from `Arc<dyn MarkTransformer>` to `Box<dyn MarkTransformer>`, replace `Arc::new(...)` with `Box::new(...)`, remove `#[allow(dead_code)]`
- [x] T013 [P] [US1] In `core/src/presentation/di/container.rs`: change `create_buffer_manager` return type from `Arc<dyn BufferManager>` to `Box<dyn BufferManager>`, replace `Arc::new(...)` with `Box::new(...)`, remove `#[allow(dead_code)]`
- [x] T014 [US1] In `core/src/presentation/di/container.rs`: delete the entire `create_history_tracker` function (no consumer in `ProcessorService` — confirmed in T002)
- [x] T015 [US1] In `core/src/presentation/di/container.rs`: rewrite `create_processor_service()` to replace all inline adapter constructions with factory calls:
  - `Self::create_input_method(&config)` replaces the inline `match method_id { ... }` block
  - `Self::create_syllable_validator()` replaces `Box::new(SyllableStructureValidator::new())`
  - `Self::create_tone_transformer()` replaces `Box::new(VietnameseToneAdapter::new(...))`
  - `Self::create_mark_transformer()` replaces `Box::new(VietnameseMarkAdapter::new())`
  - `Self::create_buffer_manager()` replaces `Box::new(MemoryBufferAdapter::new())`
  - `Self::create_language_detector()` replaces `Box::new(LanguageDetectorAdapter::new())`
- [x] T016 [US1] Run `cd core && cargo build --release 2>&1` — verify zero warnings and zero errors; fix any type mismatches before continuing
- [x] T017 [US1] Run `cd core && cargo test di_container` — verify all 3 baseline tests from Phase 2 still pass; fix any regressions before continuing
- [x] T018 [US1] Run `cd core && cargo test` — verify full test suite passes with zero regressions

**Checkpoint**: All factory functions wired, `create_history_tracker` deleted, zero `#[allow(dead_code)]` annotations remain in DI container, all tests pass.

---

## Phase 4: Polish & Cross-Cutting Concerns

- [x] T019 Run `cd core && cargo fmt` to normalize formatting in `core/src/presentation/di/container.rs`
- [x] T020 Run `cd core && cargo clippy` and fix any new lint warnings introduced by the refactor
- [x] T021 [P] Run `./scripts/rust_build_lib_universal_for_macos.sh 2>&1 | tail -5` — verify universal library builds cleanly with no warnings on both architectures

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 completion — BLOCKS Phase 3
- **US1 (Phase 3)**: Depends on Phase 2 (T007 must pass) — T008–T013 are parallelizable among themselves; T014 is independent; T015 depends on T008–T013 completing
- **Polish (Phase 4)**: Depends on Phase 3 completion (T018 must pass)

### Within US1

```
T008 ──┐
T009 ──┤
T010 ──┼──► T015 ──► T016 ──► T017 ──► T018
T011 ──┤
T012 ──┤
T013 ──┘
T014 ──► (independent, can run anytime before T015)
```

### Parallel Opportunities

```bash
# T008–T013 can run in parallel (different factory functions, same file — serialize if single developer):
# Change create_input_method
# Change create_syllable_validator
# Change create_language_detector
# Change create_tone_transformer
# Change create_mark_transformer
# Change create_buffer_manager

# T014 is independent:
# Delete create_history_tracker

# T019 and T021 in Polish phase can run in parallel:
# cargo fmt
# universal library build
```

---

## Implementation Strategy

### MVP (US1 only — this is the full scope)

1. Complete Phase 1: Read and baseline
2. Complete Phase 2: Write tests, confirm they pass pre-change
3. Complete Phase 3: Change factory return types, delete history tracker, wire `create_processor_service`
4. **VALIDATE**: `cargo build --release` — zero warnings; `cargo test` — zero regressions
5. Complete Phase 4: Format + lint + universal build

---

## Notes

- [P] tasks touch different factory functions — safe to parallelize but single-developer work is fastest done sequentially (T008 → T013 in order)
- T015 (rewiring `create_processor_service`) is the highest-risk task — do it last among Phase 3 tasks after all factory types are correct
- Constitution §III requires T004–T007 (regression tests) before T008 (first production change)
- No hot-path changes — performance benchmarks (`cargo bench`) not required for this refactor
