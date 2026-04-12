# Tasks: macOS Input Pipeline Bug Fixes & Optimization

**Input**: Design documents from `specs/003-macos-input-optimization/`  
**Prerequisites**: plan.md ✅ spec.md ✅ research.md ✅ data-model.md ✅ contracts/ ✅ quickstart.md ✅

**Tests**: No dedicated test tasks — fixes are verified via the manual regression protocol in `quickstart.md`. Test tasks would be included if XCTest harness for the CGEventTap path existed.

**Organization**: Tasks grouped by user story for independent implementation and validation.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files or distinct, non-conflicting code sections)
- **[Story]**: Which user story this task belongs to (US1–US5 from spec.md)
- Exact file paths are relative to `platforms/macos/goxviet/goxviet/`

---

## Phase 1: Setup

**Purpose**: Confirm the build works before any modifications and capture regression baselines.

- [x] T001 Build Rust universal static library via `./scripts/rust_build_lib_universal_for_macos.sh` (required for Xcode compilation)
- [x] T002 [P] Open `platforms/macos/goxviet/goxviet.xcodeproj` in Xcode and confirm clean build with 0 errors, 0 warnings before any changes
- [x] T003 [P] Capture Activity Monitor memory baseline for `goxviet` process (note RSS in MB) for US2 regression verification

**Checkpoint**: Build is green; baselines recorded.

---

## Phase 2: Foundational (Blocking Prerequisite)

**Purpose**: Read and confirm the exact current state of each target function. Required to avoid merging edits with stale line numbers.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 Inspect `InputManager.stop()` in `Managers/Input/InputManager.swift` lines 194–230 and confirm: (a) `CFRelease` is missing for `runLoopSource` and `eventTap`, (b) `checkFocusedElementIsTextInput()` is called from event tap callback without main-thread dispatch
- [x] T005 [P] Inspect `TextInjectionHelper.injectSync()` in `Managers/Injection/TextInjectionHelper.swift` lines 84–108 and confirm `semaphore.wait()` is the first statement regardless of injection method
- [x] T006 [P] Inspect `PerAppModeManagerEnhanced.swift` and confirm: (a) `startPollingTimer()` uses `withTimeInterval: 1.5`, (b) `checkSpotlightOnce()` uses `spotlightChecked: Bool` one-shot flag
- [x] T007 [P] Inspect `SettingsManager.swift` and confirm: (a) `saveToDefaults()` calls `userDefaults.set()` synchronously, (b) `@Published var inputMethod` `didSet` calls `saveToDefaults()` directly

**Checkpoint**: All target functions verified — exact change sites confirmed.

---

## Phase 3: User Story 1 — Stable Input Under Rapid Typing (Priority: P1) 🎯 MVP

**Goal**: Eliminate all event tap callback blocking: AX queries off IOKit thread, semaphore blocking, Swift 6 actor-isolation violations. After this phase, rapid typing (120+ WPM) produces no keyboard freezes.

**Independent Test**: Type at 120+ WPM in a text editor while in Vietnamese mode; no keyboard freeze, no characters dropped. See quickstart.md Fix 2, 3, and 4 regression tests.

### Implementation for User Story 1

- [x] T008 [US1] Mark `handleEvent(_:event:type:proxy:)` as `nonisolated` in `Managers/Input/InputManager.swift` (resolves Swift 6 actor-isolation violation — IOKit callback cannot be MainActor-isolated)
- [x] T009 [US1] Add `cachedIsFocusedOnTextInput: Bool` property (default `false`) and `cachedSpotlightActive: Bool` property (default `false`) as `nonisolated(unsafe)` stored properties to `InputManager` in `Managers/Input/InputManager.swift`
- [x] T010 [US1] Refactor `checkFocusedElementIsTextInput()` in `Managers/Input/InputManager.swift`: add `AXUIElementSetMessagingTimeout(systemWide, 0.05)` before query, dispatch the entire call to `DispatchQueue.main.async`, store result in `cachedIsFocusedOnTextInput`
- [x] T011 [US1] Update all callsites of `checkFocusedElementIsTextInput()` in `Managers/Input/InputManager.swift` to read `cachedIsFocusedOnTextInput` instead of calling inline
- [x] T012 [P] [US1] Add `AXUIElementSetMessagingTimeout(systemWide, 0.05)` before the `AXUIElementCopyAttributeValue` call in `checkSpotlightOnce()` in `Managers/PerAppModeManagerEnhanced.swift`
- [x] T013 [US1] Add a guard in `TextInjectionHelper.injectSync()` in `Managers/Injection/TextInjectionHelper.swift`: move `semaphore.wait()` inside a conditional that skips it for `syncProxy` and `passthrough` methods, preserving semaphore guard only for async injection paths
- [ ] T014 [US1] Verify: build in Xcode (0 errors, 0 warnings); run quickstart.md Fix 3 test (type 120+ WPM, no freeze); run quickstart.md Fix 4 test (semaphore path verified in terminal/slow-injection app)

**Checkpoint**: Rapid typing stable. CGEventTap callback never blocks. Swift 6 concurrency warnings resolved in InputManager.

---

## Phase 4: User Story 2 — No Memory Leaks During Long Sessions (Priority: P1)

**Goal**: Release all Core Foundation objects in `InputManager.stop()` so that repeated enable/disable cycles do not leak CFMachPort or CFRunLoopSource objects.

**Independent Test**: Enable and disable the IME 50 times via the menu bar; Activity Monitor RSS delta is under 5MB vs. the T003 baseline. See quickstart.md Fix 1 regression test.

### Implementation for User Story 2

- [x] T015 [US2] ~~CFRelease calls~~ — confirmed NOT needed: `eventTap: CFMachPort?` and `runLoopSource: CFRunLoopSource?` are typed Swift optionals; ARC releases them when set to `nil`. `CFRelease` is unavailable in Swift by design (prevents double-free). Added clarifying comments to `stop()` instead.
- [x] T016 [US2] No code change required — original `stop()` lifecycle was already correct. US2 finding from codebase analysis was a false positive.
- [ ] T017 [US2] Verify: run quickstart.md Fix 1 test (50 enable/disable cycles, memory delta < 5MB) — validate no regression

**Checkpoint**: CF objects released correctly. No per-cycle memory growth.

---

## Phase 5: User Story 3 — Thread-Safe Settings Updates (Priority: P2)

**Goal**: Debounce UserDefaults writes so that settings changes during active typing do not introduce latency spikes on the MainActor. Flush synchronously on app quit.

**Independent Test**: Change input method 10× in 1 second while typing; verify no perceptible latency spike on keystrokes. See quickstart.md Fix 7 regression test.

### Implementation for User Story 3

- [x] T018 [US3] Add `private var saveWorkItems: [String: DispatchWorkItem] = [:]` to `SettingsManager` in `Core/SettingsManager.swift`
- [x] T019 [US3] Refactor `saveToDefaults<T>(_:value:)` in `Core/SettingsManager.swift` to cancel any existing `DispatchWorkItem` for the key, create a new `DispatchWorkItem` that calls `userDefaults.set(value, forKey: key)`, and schedule it via `DispatchQueue.main.asyncAfter(deadline: .now() + 0.3, execute: item)`
- [x] T020 [US3] In `AppDelegate.applicationWillTerminate(_:)` in `App/AppDelegate.swift`: cancel all pending `saveWorkItems` and call `SettingsManager.shared.userDefaults.synchronize()` to flush any debounced writes before exit
- [ ] T021 [US3] Verify: build (0 errors); run quickstart.md Fix 7 test (10 rapid settings changes, no latency spike)

**Checkpoint**: UserDefaults writes are batched. Hot path never synchronously flushes to disk.

---

## Phase 6: User Story 4 — Accurate Per-App Mode Detection (Priority: P2)

**Goal**: Replace the one-shot Spotlight detection flag with a time-based TTL cache so that Spotlight (and similar panels) are correctly detected on every open, not just the first time per app session.

**Independent Test**: Open Spotlight, close, open again — repeat 5 times. IME mode is correctly applied each time. See quickstart.md Fix 5 regression test.

### Implementation for User Story 4

- [x] T022 [US4] Replace `private var spotlightChecked = false` with `private var lastSpotlightCheckTime: Date = .distantPast` in `Managers/PerAppModeManagerEnhanced.swift`
- [x] T023 [US4] In `checkSpotlightOnce()` in `Managers/PerAppModeManagerEnhanced.swift`: replace `guard !spotlightChecked else { return }` / `spotlightChecked = true` with TTL guard: `guard Date().timeIntervalSince(lastSpotlightCheckTime) > 3.0 else { return }` / `lastSpotlightCheckTime = Date()`
- [x] T024 [US4] Rename `resetSpotlightCheck()` to `resetSpotlightCache()` in `Managers/PerAppModeManagerEnhanced.swift` and update its body to set `lastSpotlightCheckTime = .distantPast`
- [x] T025 [US4] Update all call sites of `resetSpotlightCheck()` to `resetSpotlightCache()` across `Managers/PerAppModeManagerEnhanced.swift` and `Managers/Input/InputManager.swift`
- [ ] T026 [US4] Verify: build (0 errors); run quickstart.md Fix 5 test (5 Spotlight open/close cycles, correct mode each time)

**Checkpoint**: Spotlight detection accurate on repeated opens. TTL cache prevents excessive AX queries.

---

## Phase 7: User Story 5 — Reduced Battery Impact from Background Polling (Priority: P3)

**Goal**: Reduce the special-panel polling timer frequency from 1.5 seconds to 5 seconds, cutting background CPU wake-ups by 70%.

**Independent Test**: Leave GoxViet running with no typing for 10 minutes; CPU usage in Activity Monitor averages below 0.1%. See quickstart.md Fix 6 regression test.

### Implementation for User Story 5

- [x] T027 [US5] In `startPollingTimer()` in `Managers/PerAppModeManagerEnhanced.swift`: change `withTimeInterval: 1.5` to `withTimeInterval: 5.0`
- [ ] T028 [US5] Verify: build (0 errors); run quickstart.md Fix 6 test (10 min idle, CPU < 0.1%)

**Checkpoint**: Polling timer fires every 5 seconds. Battery impact reduced.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that span multiple user stories or finalize the feature.

- [x] T029 [P] Implement LRU eviction for per-app modes at capacity in `Core/SettingsManager.swift`: add `private var perAppLastAccess: [String: Date] = [:]`, update `getPerAppMode()` to stamp access time, update `setPerAppMode()` to evict the least-recently-accessed entry instead of silently dropping new entries when `dict.count >= MAX_PER_APP_ENTRIES`
- [ ] T030 [P] Remove commented-out code blocks (lines ~776–801 in `Managers/Input/InputManager.swift` — old backspace coalescing attempt) and consolidate remaining backspace logic  *(deferred — requires careful manual review of backspace logic)*
- [ ] T031 Run the full `quickstart.md` manual validation protocol: all 8 regression tests pass, 0 keyboard freezes, memory stable, Spotlight works 5/5 times
- [ ] T032 Update `CHANGELOG.md` with a `fix(macos)` entry summarizing the 8 fixes and their impact

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on T001 (Rust lib built) — BLOCKS all user stories
- **US1 (Phase 3)**: Depends on Phase 2 completion — no dependency on US2–US5
- **US2 (Phase 4)**: Depends on Phase 2 completion — no dependency on US1, US3–US5; both touch `InputManager.swift` so coordinate with US1 to avoid file conflicts
- **US3 (Phase 5)**: Depends on Phase 2 completion — no dependency on US1–US2
- **US4 (Phase 6)**: Depends on Phase 2 completion — no dependency on other stories
- **US5 (Phase 7)**: Depends on Phase 2 completion — no dependency on other stories
- **Polish (Phase 8)**: Depends on all desired user stories being complete

### User Story Cross-File Matrix

| User Story | Primary File(s) | Conflicts with |
|------------|----------------|---------------|
| US1 | InputManager.swift, TextInjectionHelper.swift, PerAppModeManagerEnhanced.swift (T012) | US2 (both edit InputManager.swift — sequence T008–T014 before T015–T016) |
| US2 | InputManager.swift | US1 (same file — complete US1 first) |
| US3 | SettingsManager.swift, AppDelegate.swift | None |
| US4 | PerAppModeManagerEnhanced.swift | T012 from US1 also edits this file — coordinate |
| US5 | PerAppModeManagerEnhanced.swift | US4 (same file — sequence US4 before US5) |

### Recommended Sequencing for a Single Developer

```
Phase 1 (T001-T003) → Phase 2 (T004-T007) → 
  US1 (T008-T014) → 
  US2 (T015-T017) →     [same file as US1, so sequence after]
  US3 (T018-T021) →     [different file, but order doesn't matter]
  US4 (T022-T026) →     [same file as US1 T012 — apply after US1]
  US5 (T027-T028) →     [same file as US4 — apply after US4]
  Polish (T029-T032)
```

### Parallel Opportunities (Two Developers)

```
After Phase 2:
  Developer A: US1 (T008–T014) → US2 (T015–T017)
  Developer B: US3 (T018–T021) → US4 (T022–T026) → US5 (T027–T028)
Both: Polish (T029–T032) together
```

---

## Parallel Example: User Story 3 (SettingsManager)

```bash
# T018 and T019 are sequential (T019 uses the dict added in T018)
# T020 can be done in parallel with T019 (AppDelegate vs SettingsManager)
Task A: "Refactor saveToDefaults to use DispatchWorkItem in Core/SettingsManager.swift"    # T019
Task B: "Add applicationWillTerminate flush in App/AppDelegate.swift"                     # T020
```

---

## Implementation Strategy

### MVP First (User Stories 1 + 2 Only)

1. Complete Phase 1: Setup (T001–T003)
2. Complete Phase 2: Foundational (T004–T007)
3. Complete Phase 3: User Story 1 (T008–T014)
4. Complete Phase 4: User Story 2 (T015–T017)
5. **STOP and VALIDATE**: Rapid typing is stable; no memory leaks (50 enable/disable cycles)
6. Ship as a hotfix — US3–US5 and Polish can land in a follow-up

### Incremental Delivery

1. Setup + Foundational → Foundation ready
2. US1 + US2 → Core stability fixed → Ship **MVP**
3. US3 → Settings thread safety → Ship
4. US4 + US5 → Per-app accuracy + battery → Ship
5. Polish → LRU eviction + cleanup → Ship

---

## Notes

- **No new files required** — all changes are targeted edits to 4 existing Swift files (+ AppDelegate.swift for T020)
- **[P]** marks tasks on different files or non-conflicting code sections within a file
- Complete US1 before US2 (both edit `InputManager.swift`)
- Complete US4 before US5 (both edit `PerAppModeManagerEnhanced.swift`)
- Constitution Principle III (Regression-First): before each implementation task, manually reproduce the bug per `quickstart.md`, then apply the fix
- Commit after each user story phase with a `fix(macos): <description>` commit message per Conventional Commits
