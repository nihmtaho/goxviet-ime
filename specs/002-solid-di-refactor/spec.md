# Feature Specification: Implement DI Factory Functions in SOLID Container

**Feature Branch**: `feature/002-solid-di-refactor`  
**Created**: 2026-04-06  
**Status**: Draft  
**Input**: User description: "Implement unused DI factory functions that were cleaned up and use them appropriately in the current SOLID architecture."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - DI Container Wires All Adapters via Factory Functions (Priority: P1)

As the application layer, I need the Dependency Injection Container to compose the engine from clearly-defined factory functions — one per port — so that each component is created, configured, and injected in a single well-bounded location.

Currently, the DI container builds the engine in one monolithic `create_processor_service()` call with inline adapter construction. The factory functions (`create_input_method`, `create_syllable_validator`, `create_language_detector`, `create_tone_transformer`, `create_mark_transformer`, `create_buffer_manager`) exist but are marked `#[allow(dead_code)]` and are not called by `create_processor_service()`.

**Why this priority**: If the engine is assembled from ad-hoc inline code rather than the registered factory functions, the SOLID principle of Single Responsibility is violated. Future adapter swaps (e.g., switching from Telex to VNI) require editing the same monolithic block instead of the relevant factory function.

**Independent Test**: `Container::create_processor_service()` can be tested indirectly by constructing a Container and verifying it produces a correctly-wired engine that processes a simple keypress without error. No UI or FFI required.

**Acceptance Scenarios**:

1. **Given** the Container is instantiated with a valid config, **When** `create_processor_service()` is called, **Then** it internally delegates adapter creation to the named factory functions rather than constructing adapters inline.
2. **Given** a `Container` built with `InputMethodId::Telex`, **When** a keypress is processed, **Then** the engine behaves as a Telex input method.
3. **Given** a `Container` built with `InputMethodId::Vni`, **When** a keypress is processed, **Then** the engine behaves as a VNI input method.
4. **Given** `create_syllable_validator()` is called, **Then** it returns a `Box<dyn SyllableValidator>` wrapping the structure-based validator.
5. **Given** `create_language_detector()` is called, **Then** it returns a `Box<dyn LanguageDetector>`.
6. **Given** each factory function is called, **Then** the returned value satisfies the corresponding port trait.

---

### ~~User Story 2~~ — CANCELLED

**Cancelled**: Research confirmed `find_uo_compound_positions` is already called internally by `has_complete_uo_compound()`. All Engine call sites use the boolean form — none require the position tuple. No action needed.

---

### ~~User Story 3~~ — CANCELLED

**Cancelled**: `engine/mod.rs` line 1458 contains an explicit comment prohibiting this call: *"Do NOT call rebuild_output_from_entire_buffer() because it uses buf.len() as backspace which is wrong after revert."* Reinstating it would introduce a regression.

---

### Edge Cases

- What happens when `create_input_method` receives an unknown `InputMethodId`? → Falls back to Telex with no panic (documented in factory function).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: `Container::create_processor_service()` MUST delegate adapter construction to the dedicated factory functions (`create_input_method`, `create_syllable_validator`, `create_language_detector`, `create_tone_transformer`, `create_mark_transformer`, `create_buffer_manager`) rather than constructing adapters inline.
- **FR-002**: Each factory function MUST return a `Box<dyn PortTrait>` typed to its corresponding domain port (single-owner; `Arc` is not used here).
- **FR-003**: `create_input_method` MUST branch on `InputMethodId` to return the correct adapter (Telex or VNI); unrecognized IDs MUST fall back to Telex with no panic.
- ~~**FR-004**~~: CANCELLED — see US2 cancellation above.
- ~~**FR-005**~~: CANCELLED — see US3 cancellation above.
- **FR-006**: All factory functions MUST have no `#[allow(dead_code)]` annotations — they must be provably called from `create_processor_service()`.
- **FR-007**: No new heap allocations MUST be introduced on the hot path (`process_key` and callees) as a result of this refactor.
- **FR-008**: All changed code MUST pass existing integration tests; new regression tests MUST be added for the factory dispatch scenarios.

### Key Entities

- **Container**: The DI composition root. Responsible for assembling all port adapters and wiring them into the engine.
- **Engine**: The core processing unit. Receives assembled adapters and processes keystrokes.
- **Port Traits** (`InputMethod`, `SyllableValidator`, `LanguageDetector`, `ToneTransformer`, `MarkTransformer`, `BufferManager`): Domain interfaces that adapters implement. (`HistoryTracker` is defined but has no consumer in `ProcessorService` — `create_history_tracker` is deleted.)
- **Factory Functions**: Private (`fn`, no visibility modifier) static methods on `Container` — one per port — returning the correct adapter for the current config. Not directly callable from test code; validated through engine behavior.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All 6 DI factory functions are called from `create_processor_service()` with zero `#[allow(dead_code)]` annotations remaining in `container.rs`.
- **SC-002**: Existing test suite (`cargo test`) passes with zero regressions after the refactor.
- **SC-003**: Build produces zero compiler warnings (`cargo build --release` clean output).
- **SC-004**: New integration tests in `core/tests/di_container_test.rs` cover factory dispatch for Telex, VNI, and config-swap scenarios with table-driven inputs.
- **SC-005**: No hot-path code is changed; `cargo bench` is not required for this refactor (factory functions are construction-time only).
- **SC-006**: Code review confirms no layer violations (no `infrastructure` importing from `presentation`, no `domain` importing from `application`).

## Clarifications

### Session 2026-04-06

- Q: Should `find_uo_compound_positions` be reinstated as an Engine instance method, or should call sites call `vowel_compound::find_uo_compound_positions(&self.buf)` directly? → A: Call sites use `vowel_compound::find_uo_compound_positions(&self.buf)` directly — no new Engine method added.
- Q: When `rebuild_output_from_entire_buffer` is called with a buffer longer than `u8::MAX`, should it silently clamp, log a warning, or return an error? → A: Retain silent clamp to `u8::MAX` — this is a defensive guard for an impossible state in normal use; no logging or error needed.
- Q: Should the 6 DI factory functions have `pub(crate)` visibility for direct unit testing, or remain private (`fn`) with tests validating through engine behavior? → A: Keep private — tests validate correctness through engine output, not by calling factory functions directly.

## Assumptions

- The current `Container::create_processor_service()` implementation constructs adapters inline; the factory functions exist but are bypassed. This has been confirmed via code inspection.
- `create_history_tracker` has no consumer in `ProcessorService` and is deleted (confirmed by grepping `processor_service.rs`).
- Windows platform is out of scope; changes apply to Rust core only.
