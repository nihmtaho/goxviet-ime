# Implementation Plan: macOS Input Pipeline Bug Fixes & Optimization

**Branch**: `003-macos-input-optimization` | **Date**: 2026-04-12 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `specs/003-macos-input-optimization/spec.md`

## Summary

Fix 10 confirmed bugs and performance issues in the GoxViet macOS Swift layer: two CF memory leaks in `InputManager.stop()`, a blocking semaphore on the CGEventTap IOKit thread, AX queries executing off-MainActor, synchronous UserDefaults writes on the hot path, a one-shot Spotlight detection flag that fails on re-opens, and a 1.5-second polling timer that wastes battery. All changes are confined to the macOS Swift platform layer; the Rust core and FFI API are unchanged.

## Technical Context

**Language/Version**: Swift 6.2 (`SWIFT_DEFAULT_ACTOR_ISOLATION = MainActor`), Rust stable  
**Primary Dependencies**: AppKit, ApplicationServices (CGEventTap, AXUIElement), CoreFoundation  
**Storage**: UserDefaults (settings persistence; no database)  
**Testing**: XCTest (Swift UI/integration), manual keyboard testing; no XCTest unit harness exists for the event tap path  
**Target Platform**: macOS 11 (Big Sur) minimum deployment target  
**Project Type**: Desktop app (input method engine — keyboard event tap daemon + settings UI)  
**Performance Goals**: < 3ms Rust core, < 16ms end-to-end keystroke latency (from key-down to text appearing)  
**Constraints**: Event tap callback must complete in < 5ms; no macOS 12+ APIs; no new external dependencies  
**Scale/Scope**: Single-user desktop app; keystroke pipeline is a serial hot path, not concurrent

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

| Principle | Gate | Status | Notes |
|-----------|------|--------|-------|
| I. Performance First | Hot-path changes must not regress < 16ms end-to-end latency | ✅ PASS | Fixes remove blocking (semaphore, sync UserDefaults writes) — latency improves, not regresses |
| II. Clean Architecture | All changes stay within Swift layer; `RustBridgeSafe` remains sole FFI boundary | ✅ PASS | No Rust core changes; FFI API unchanged |
| III. Regression-First Testing | Regression test added before each fix | ✅ PASS | Manual keyboard test scripts specified per fix; AX path tested via XCTest mock |
| IV. Zero FFI Panics | `defer { ime_free_string_v2 }` patterns preserved; no new raw FFI calls outside RustBridgeSafe | ✅ PASS | CF memory fixes do not touch Rust FFI boundaries |
| V. Branding Consistency | No branding changes | ✅ PASS | N/A |

**Post-design re-check**: No violations found after Phase 1 design.

## Project Structure

### Documentation (this feature)

```text
specs/003-macos-input-optimization/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output (state transition models)
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   └── event-tap-threading.md
└── tasks.md             # Phase 2 output (/speckit.tasks — NOT created here)
```

### Source Code (files to modify)

```text
platforms/macos/goxviet/goxviet/
├── Managers/
│   ├── Input/
│   │   └── InputManager.swift          # MODIFY: CF release, AX query dispatch, tap re-enable
│   ├── Injection/
│   │   └── TextInjectionHelper.swift   # MODIFY: remove semaphore block from event tap path
│   └── PerAppModeManagerEnhanced.swift # MODIFY: Spotlight re-detection, polling interval
├── Core/
│   └── SettingsManager.swift           # MODIFY: async UserDefaults writes, per-app LRU eviction
└── [no new files required]
```

**Structure Decision**: Single-file modifications in existing platform layer. No new files, no new managers. All fixes are targeted and minimal.

## Complexity Tracking

> No constitution violations — table not required.
