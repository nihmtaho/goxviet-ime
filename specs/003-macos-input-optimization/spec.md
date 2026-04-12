# Feature Specification: macOS Input Pipeline Bug Fixes & Optimization

**Feature Branch**: `003-macos-input-optimization`  
**Created**: 2026-04-12  
**Status**: Draft  
**Input**: User description: "Giúp tôi kiểm tra các lỗi và tối ưu gõ trên nền tảng macOS, hãy kiểm tra codebase platform macOS để tìm các vấn đề chưa được tối ưu."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Stable Input Under Rapid Typing (Priority: P1)

As a fast typist using GoxViet on macOS, I need keystrokes to be processed without dropped characters or input hangs when typing quickly in Vietnamese or switching between apps.

**Why this priority**: Blocking the event tap callback (via semaphore wait or slow AX queries) is a hard crash-class bug that stops all keyboard input. Reproducible on any machine with rapid typing.

**Independent Test**: Open a text editor, type at 120+ WPM sustained, verify no characters are dropped and no keyboard freeze occurs.

**Acceptance Scenarios**:

1. **Given** the user is typing Vietnamese text at 120+ WPM, **When** successive keystrokes arrive within 50ms of each other, **Then** all keystrokes are registered without freezing the keyboard pipeline.
2. **Given** a second keystroke arrives before the first injection completes, **When** the event tap callback receives it, **Then** it is queued and processed without blocking input delivery to the OS.
3. **Given** the CGEventTap is disabled by a system timeout, **When** the tap is re-enabled, **Then** the next keystroke is not lost and the tap resumes correctly within one event cycle.

---

### User Story 2 - No Memory Leaks During Long Sessions (Priority: P1)

As a macOS user running GoxViet for hours, I need the app's memory footprint to remain stable and not grow unboundedly over a long typing session or repeated enable/disable cycles.

**Why this priority**: CFMachPort and CFRunLoopSource leaks are provably reproducible. Each start/stop cycle leaks ~1KB+. Over an 8-hour work session with app switching, this compounds to measurable memory waste.

**Independent Test**: Enable and disable the IME 50 times via the menu bar, then verify memory usage in Activity Monitor does not grow significantly compared to baseline.

**Acceptance Scenarios**:

1. **Given** the IME is enabled and disabled 50 times, **When** memory usage is sampled before and after, **Then** the delta is under 5MB.
2. **Given** the InputManager is started and stopped, **When** it is torn down, **Then** all Core Foundation objects (CGEventTap, CFRunLoopSource) are explicitly released before returning.
3. **Given** the app runs for 8+ hours with normal usage, **When** memory is sampled, **Then** it does not exceed 150% of the initial startup footprint.

---

### User Story 3 - Thread-Safe Settings Updates (Priority: P2)

As a user who changes input method or per-app settings while actively typing, I need configuration changes to be applied safely without causing crashes, deadlocks, or inconsistent engine state.

**Why this priority**: The NSLock double-acquire deadlock is latent but deterministic — it triggers on `getKnownAppsWithStates()` which is called in the settings UI. Combined with the `@Published` race condition, a settings change during active typing can corrupt Rust engine state.

**Independent Test**: Open Settings UI, switch input method from Telex to VNI while typing rapidly in a text editor. Verify no hang, no crash, and the new method is applied consistently within one word boundary.

**Acceptance Scenarios**:

1. **Given** the user changes the input method from Telex to VNI, **When** a keystroke is being processed at the same time, **Then** no deadlock, hang, or engine state corruption occurs.
2. **Given** `getKnownAppsWithStates()` is called from the settings UI, **When** `getPerAppMode()` is invoked inside it, **Then** no deadlock occurs.
3. **Given** multiple settings are changed in rapid succession, **When** the engine config is synchronized, **Then** the final engine state matches all applied changes with no intermediate inconsistent states.

---

### User Story 4 - Accurate Per-App Mode Detection (Priority: P2)

As a user relying on Smart Mode to automatically enable/disable Vietnamese input per app, I need Spotlight, Raycast, and other transient panels to be detected accurately every time they appear — not just the first time.

**Why this priority**: The `spotlightChecked` flag is reset only on app switch, causing Spotlight detection to miss subsequent invocations within the same app session.

**Independent Test**: Open Spotlight 5 times in succession from the same app. Verify the IME mode is correctly detected and set each time Spotlight opens.

**Acceptance Scenarios**:

1. **Given** Spotlight was opened and closed, **When** the user opens Spotlight again within the same app session, **Then** the IME mode is correctly re-evaluated (not assumed from prior detection).
2. **Given** Raycast or another floating search panel opens, **When** the detection timer fires, **Then** the correct per-panel mode is applied within 2 seconds.
3. **Given** a special panel is dismissed, **When** focus returns to the previous app, **Then** the IME mode reverts to that app's configured mode.

---

### User Story 5 - Reduced Battery Impact from Background Polling (Priority: P3)

As a macOS laptop user, I need GoxViet to have minimal background CPU and battery impact when I am not actively typing.

**Why this priority**: The 1.5-second Spotlight detection timer fires ~2400 times per hour regardless of user activity. On battery, this adds measurable drain.

**Independent Test**: With GoxViet running and no typing activity for 10 minutes, verify CPU usage is under 0.1% average in Activity Monitor.

**Acceptance Scenarios**:

1. **Given** the user has not typed for 60 seconds, **When** the background polling timer fires, **Then** it performs at most one lightweight check per 5+ seconds (not 1.5 seconds).
2. **Given** the user begins typing after idle, **When** the first keystroke is received, **Then** Smart Mode detection responds within 2 seconds without needing the polling timer to catch up.
3. **Given** the app is in background with no active text fields, **When** measured over 10 minutes of no typing, **Then** average CPU usage is below 0.1%.

---

### Edge Cases

- What happens when AX Server is slow or unresponsive and `AXUIElementCopyAttributeValue` hangs indefinitely on the event tap callback thread?
- What happens when `getKnownAppsWithStates()` is called while the per-app dictionary is at maximum capacity and a new app entry is being added?
- How does the system handle the case where `ime_free_string_v2` is called twice on the same pointer (double-free) if the FFI bridge has a defensive guard?
- What happens when the user rapidly enables/disables Smart Mode for an app 100+ times, growing the per-app defaults dictionary unboundedly?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The event tap callback MUST NOT block for more than 5ms at any point; long operations (text injection, AX queries) must be dispatched asynchronously or have enforced timeouts.
- **FR-002**: All Core Foundation objects acquired during `InputManager.start()` MUST be explicitly released in `InputManager.stop()`, including `CGEventTap` (CFMachPort) and `CFRunLoopSource`.
- **FR-003**: The `SettingsManager` lock implementation MUST support recursive acquisition so that calling `getPerAppMode()` inside `getKnownAppsWithStates()` does not deadlock.
- **FR-004**: `@Published` property `didSet` observers in `SettingsManager` that call `syncToCore()` MUST be dispatched so they do not race with concurrent keystroke processing.
- **FR-005**: AX UI element queries MUST have a defined timeout (≤ 50ms) and MUST be executed on the main thread, not the IOKit event tap callback thread.
- **FR-006**: The Spotlight/special-panel detection flag MUST be re-evaluated on each panel appearance, not just the first occurrence per app session.
- **FR-007**: The background special-panel polling interval MUST be at least 5 seconds (up from 1.5 seconds).
- **FR-008**: The TextInjector semaphore MUST NOT block the event tap callback; rapid keystrokes must be queued rather than causing the IOKit thread to wait.
- **FR-009**: `UserDefaults` writes from `SettingsManager` MUST be batched or debounced so they do not execute synchronously on the keystroke processing path.
- **FR-010**: The per-app mode dictionary MUST implement LRU eviction rather than silently dropping new entries when at capacity.

### Key Entities

- **InputManager**: Singleton that owns the CGEventTap lifecycle; must release all CF objects on stop.
- **SettingsManager**: Centralized settings store; must have recursive lock and async persistence writes.
- **RustEngineV2**: Swift wrapper around Rust FFI; must cache config locally to minimize FFI round-trips.
- **PerAppModeManagerEnhanced**: Manages per-app IME mode; must reset Spotlight detection state on each panel appearance.
- **TextInjectionHelper**: Handles text injection into target apps; must not block event tap with semaphore waits.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Zero keyboard freezes (0 events where all keyboard input is blocked) during a 5-minute rapid-typing session at 120+ WPM.
- **SC-002**: Memory usage delta after 50 enable/disable cycles is under 5MB (no CF object leak per cycle).
- **SC-003**: No deadlocks reproduced during 100 consecutive settings changes interleaved with typing.
- **SC-004**: Spotlight detection triggers correctly on 5 out of 5 successive Spotlight open events (100% accuracy, not just first-time).
- **SC-005**: Average CPU usage is below 0.1% measured over 10 minutes of no typing activity.
- **SC-006**: End-to-end keystroke latency (key down → text appears in app) remains under 16ms at the 95th percentile after all fixes applied.
- **SC-007**: Per-app mode dictionary correctly evicts least-recently-used entry when at capacity, with no silent data loss.

## Assumptions

- The investigation is scoped to the Swift macOS platform layer; Rust core changes are out of scope unless an FFI boundary issue is identified.
- The project targets macOS 11 (Big Sur) as the minimum deployment target; solutions must not require macOS 12+ APIs.
- Swift 6 strict concurrency is enabled (`SWIFT_DEFAULT_ACTOR_ISOLATION = MainActor`); fixes must be compatible with this setting.
- No changes will be made to the public FFI API (`ime_process_key_v2`, `ime_free_string_v2`, etc.) as part of this work.
- Performance benchmarks will be measured on Apple Silicon (M-series) hardware; results may differ on Intel Macs.
- The existing TextInjectionHelper dispatch architecture is preserved; the semaphore issue will be resolved within the existing injection model.
