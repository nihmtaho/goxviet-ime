# Implementation Plan: Implement DI Factory Functions for SOLID Architecture

**Branch**: `feature/002-solid-di-refactor` | **Date**: 2026-04-06 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `specs/002-solid-di-refactor/spec.md`

## Summary

Wire the 7 existing DI factory functions in `Container` into `create_processor_service()` by changing their return types from `Arc<dyn Trait>` to `Box<dyn Trait>`, matching the `ProcessorService::new()` signature. This eliminates all 7 `#[allow(dead_code)]` annotations and completes the SOLID composition boundary. US2 and US3 from the spec are **cancelled** based on research findings (see below).

## Technical Context

**Language/Version**: Rust stable (goxviet-core v2.0.0)  
**Primary Dependencies**: `std::sync::Arc`, `std::sync::Mutex`, domain port traits  
**Storage**: N/A  
**Testing**: `cargo test` (unit tests in `container.rs`, integration tests in `core/tests/`)  
**Target Platform**: Rust core library (macOS arm64 + x86_64 universal)  
**Project Type**: Library (Rust core engine)  
**Performance Goals**: No hot-path changes; factory functions called only at construction time  
**Constraints**: No new heap allocations on hot path; zero compiler warnings  
**Scale/Scope**: Single file change (`container.rs`) + test additions

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

| Principle | Status | Notes |
|---|---|---|
| I. Performance First | ✅ Pass | Factory functions called at container init only — not on `process_key` hot path |
| II. Clean Architecture | ✅ Pass | All changes within `presentation/di/` layer; no layer boundary violations |
| III. Regression-First Testing | ✅ Pass | Test coverage plan included in Phase 1 |
| IV. Zero FFI Panics | ✅ Pass | No FFI boundary changes |
| V. Branding Consistency | ✅ Pass | No naming changes |

**Post-design re-check**: All gates pass. Single-file refactor within the presentation DI layer.

## Research Findings (Phase 0)

### US1 — DI Factory Functions (PROCEED)

The factory functions in `container.rs` return `Arc<dyn Trait>` but `ProcessorService::new()` takes `Box<dyn Trait>`. The fix is to change factory function return types from `Arc<dyn>` to `Box<dyn>` and delegate from `create_processor_service()` to them.

**Decision**: Change 7 factory functions to return `Box<dyn Trait>` and call them from `create_processor_service()`.  
**Rationale**: ProcessorService owns its adapters exclusively (single owner) — `Box` is semantically correct. `Arc` implies shared ownership which is unnecessary here.  
**Alternatives considered**: Changing ProcessorService to take `Arc` — rejected because Box is correct for single-owner injection.

### US2 — `find_uo_compound_positions` (CANCELLED)

**Finding**: `find_uo_compound_positions` in `vowel_compound.rs` is already called by `has_complete_uo_compound()`. All 4 Engine call sites use `has_complete_uo_compound()` (boolean) — none require position data. The removed Engine method was a pure delegate with no callers. There is no compound-vowel transform logic that needs the tuple return value.

**Decision**: US2 is cancelled. No action needed. The `vowel_compound::find_uo_compound_positions` function is alive and used indirectly.

### US3 — `rebuild_output_from_entire_buffer` (CANCELLED)

**Finding**: `engine/mod.rs` line 1458 contains an explicit comment: *"Do NOT call rebuild_output_from_entire_buffer() because it uses buf.len() as backspace which is wrong after revert (e.g., buffer=['d','d'] has len=2 but should backspace=1)"*. The method was removed because it was provably incorrect at the one identified call site, not because it was overlooked.

**Decision**: US3 is cancelled. Reinstating this method would introduce a regression.

## Project Structure

### Documentation (this feature)

```text
specs/002-solid-di-refactor/
├── plan.md              ← this file
├── research.md          ← (inlined in plan — single-file refactor)
├── spec.md
├── tasks.md
└── checklists/
    └── requirements.md
```

### Source Code Changes

```text
core/src/presentation/di/container.rs   ← PRIMARY CHANGE
  - create_processor_service(): delegate to factory functions
  - 7 factory functions: Arc<dyn> → Box<dyn>, remove #[allow(dead_code)]

core/tests/                             ← NEW TEST FILE
  - di_container_test.rs: table-driven integration test
```

**Structure Decision**: Single-file change in `presentation/di/`. No new modules, no new files except one test.

## Phase 1: Design & Contracts

### Data Model

No new entities. The relevant types already exist:

| Type | Location | Change |
|---|---|---|
| `Container` | `presentation/di/container.rs` | `create_processor_service` delegates to factories |
| 7 factory functions | same file | Return `Box<dyn Trait>` instead of `Arc<dyn Trait>` |
| `ProcessorService` | `application/services/processor_service.rs` | No change — already takes `Box<dyn Trait>` |

### Interface Contracts

This is an internal library refactor. No public FFI interface changes. The `ProcessorService` internal interface contract is unchanged — it still takes `Box<dyn>` for all 6 port types.

**Factory function signatures after change:**

```
fn create_input_method(config: &Arc<Mutex<EngineConfig>>) -> Box<dyn InputMethod>
fn create_syllable_validator() -> Box<dyn SyllableValidator>
fn create_language_detector() -> Box<dyn LanguageDetector>
fn create_tone_transformer() -> Box<dyn ToneTransformer>
fn create_mark_transformer() -> Box<dyn MarkTransformer>
fn create_buffer_manager() -> Box<dyn BufferManager>
// create_history_tracker() → DELETED (no consumer in ProcessorService)
```

Note: `create_history_tracker` has no consumer — `ProcessorService::new()` has no `HistoryTracker` port and no other Container method uses it. **It must be deleted** (confirmed by grepping `processor_service.rs`).

### Implementation Steps

**Step 1 — Change factory return types** (`container.rs`):

For each of the 7 factory functions:
- Remove `#[allow(dead_code)]` annotation
- Remove the "Kept for potential alternative injection patterns" comment
- Change `Arc::new(...)` to `Box::new(...)` in the return expression
- Change return type from `Arc<dyn Trait>` to `Box<dyn Trait>`

**Step 2 — Wire `create_processor_service` to factories**:

Replace inline adapter construction in `create_processor_service()` with factory calls:

```rust
// Before (inline):
let input_method_box: Box<dyn InputMethod> = match method_id { ... };
ProcessorService::new(
    input_method_box,
    Box::new(SyllableStructureValidator::new()),
    Box::new(VietnameseToneAdapter::new(...)),
    Box::new(VietnameseMarkAdapter::new()),
    Box::new(MemoryBufferAdapter::new()),
    Box::new(LanguageDetectorAdapter::new()),
    &config_snapshot,
)

// After (factory delegation):
ProcessorService::new(
    Self::create_input_method(&config),
    Self::create_syllable_validator(),
    Self::create_tone_transformer(),
    Self::create_mark_transformer(),
    Self::create_buffer_manager(),
    Self::create_language_detector(),
    &config_snapshot,
)
```

**Step 3 — Remove `create_history_tracker`**:

`ProcessorService::new()` has no `HistoryTracker` port (confirmed). No other Container method consumes it. Delete the function entirely — keeping it even with `#[allow(dead_code)]` is dead scaffolding.

**Step 4 — Verify build is clean**:

```bash
cd core && cargo build --release
```

Zero warnings required.

**Step 5 — Add integration test**:

Create `core/tests/di_container_test.rs` with table-driven tests:
- Container wires TelexAdapter when config is Telex
- Container wires VniAdapter when config is Vni
- Engine processes a simple keypress after construction (behavioral proof)
- `update_config()` rewires correctly after VNI → Telex swap

### Confirmed: `create_history_tracker` → Delete

`ProcessorService` has no `HistoryTracker` in its constructor or fields. The function is pure dead scaffolding. Remove it in Step 3.

## Complexity Tracking

No constitution violations. No complexity justification needed.

## Spec Amendment Required

Update `spec.md` to reflect cancelled user stories before committing:

- US2 (`find_uo_compound_positions`): Mark as **Cancelled** — already live via `has_complete_uo_compound()` internal call chain
- US3 (`rebuild_output_from_entire_buffer`): Mark as **Cancelled** — explicitly prohibited by existing engine comment; reinstatement would introduce regression
- FR-004, FR-005: Mark as cancelled / removed
- SC-004: Update to cover factory dispatch tests only (not compound-position or full-buffer rebuild)
