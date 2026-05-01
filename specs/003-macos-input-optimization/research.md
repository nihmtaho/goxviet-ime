# Research: macOS Input Pipeline Bug Fixes & Optimization

**Feature**: 003-macos-input-optimization  
**Date**: 2026-04-12

---

## Finding 1: CF Object Lifecycle in Swift Without ARC

**Question**: How should `CFMachPort` (CGEventTap) and `CFRunLoopSource` be released in Swift when ARC does not manage them?

**Decision**: Call `CFRelease()` explicitly in `InputManager.stop()` after removing the source from the run loop and disabling the tap.

**Rationale**: `CGEvent.tapCreate` returns a `CFMachPort` with a +1 retain count. `CFMachPortCreateRunLoopSource` returns a `CFRunLoopSource` with a +1 retain count. Swift's ARC does not bridge these: storing them as `CFMachPort?` / `CFRunLoopSource?` optionals does **not** release them when set to `nil`. Explicit `CFRelease()` is the only correct disposal.

**Code pattern**:
```swift
// In stop():
if let runLoopSource = self.runLoopSource {
    CFRunLoopRemoveSource(CFRunLoopGetCurrent(), runLoopSource, .commonModes)
    CFRelease(runLoopSource)          // ← ADD THIS
    self.runLoopSource = nil
}
if let eventTap = self.eventTap {
    CGEvent.tapEnable(tap: eventTap, enable: false)
    CFRelease(eventTap)               // ← ADD THIS
    self.eventTap = nil
}
```

**Alternatives considered**:
- Wrapping in a `class` with `deinit` calling `CFRelease`: works but adds indirection for objects that are already owned by `InputManager`. Unnecessary complexity.
- Using `Unmanaged<CFMachPort>`: correct but verbose; not needed since ARC manages the Swift reference and we only need to balance the CF retain count on teardown.

---

## Finding 2: Thread-Safe Event Tap Callback Pattern in Swift 6

**Question**: The CGEventTap callback runs on the IOKit thread. `InputManager.handleEvent()` is implicitly `@MainActor` under Swift 6's `SWIFT_DEFAULT_ACTOR_ISOLATION = MainActor`. How should this boundary be handled?

**Decision**: Mark `handleEvent` and all methods it calls synchronously as `nonisolated` (explicitly opt out of MainActor isolation). Dispatch only UI/settings mutations to `MainActor` asynchronously; keep the hot path (Rust FFI call, key decision) synchronous and nonisolated.

**Rationale**: The CGEventTap callback is a C function pointer; it cannot `await` MainActor. The event tap callback must return synchronously with the modified/passthrough event. The Rust engine call via `RustBridgeSafe` is already `nonisolated` (no actor context required). The only MainActor dependencies in the hot path are:
1. Reading `SettingsManager.shared.isEnabled` — safe to read as a cached snapshot
2. Reading `PerAppModeManagerEnhanced.shared.currentMode` — safe to read as an atomic snapshot

The constitution explicitly states: "engine calls are synchronous and MUST NOT be dispatched to an actor." This is consistent with a nonisolated hot path.

**Code pattern**:
```swift
// In InputManager:
nonisolated private func handleEvent(...) -> Unmanaged<CGEvent>? {
    // Direct property reads are safe (read-only snapshots)
    let isEnabled = cachedIsEnabled  // atomic Bool, not UserDefaults
    // ...
    // For UI updates from hot path:
    Task { @MainActor in
        self.updateSomeUIState()
    }
}
```

**Alternatives considered**:
- `MainActor.assumeIsolated`: crashes at runtime if called off MainActor — incorrect for this use case.
- Dispatching entire callback to `DispatchQueue.main.sync`: deadlocks if MainActor is busy; also violates < 5ms callback budget.
- Actor-isolated wrapper type: would require `await` which is incompatible with synchronous C callback.

---

## Finding 3: Removing Semaphore Block from Event Tap Callback

**Question**: `TextInjectionHelper.injectSync()` is called from the event tap callback and uses `semaphore.wait()`. How can rapid-keystroke queuing be achieved without blocking the IOKit thread?

**Decision**: For the event tap callback path (`syncProxy` injection method), bypass the semaphore entirely and inject synchronously via `CGEventTapPostEvent(proxy)`. For all other injection methods (which post async events), the semaphore guard is still needed to prevent concurrent injection state corruption — but these methods should never be called directly from the event tap callback.

**Rationale**: The existing code has two injection paths:
1. `syncProxy` — posts events synchronously through the live tap proxy. This is already atomic by design (proxy is valid only for the duration of the current event). No semaphore needed.
2. All other methods — post events to `cgSessionEventTap` asynchronously. These are called from a `DispatchQueue` dispatch inside the callback, NOT blocking the IOKit thread.

The fix is: ensure the callback path only calls synchronous proxy injection inline; all async injection is dispatched to a background queue before returning from the callback (the event tap receives `nil` / pass-through while injection is pending).

**Alternatives considered**:
- Replacing `DispatchSemaphore` with `NSLock`: same blocking problem on IOKit thread.
- Using `DispatchQueue.async` for all injections from the callback: valid, but the callback must then synthesize a "swallow" return immediately. Vietnamese composition (which needs to suppress the raw keystroke) already does this via returning `nil` from the tap.

---

## Finding 4: AX Query Timeout on macOS 11

**Question**: AXUIElement queries can hang indefinitely. How can a timeout be enforced on macOS 11 (no `AXUIElementSetMessagingTimeout` is not available pre-macOS 12)?

**Decision**: Use `AXUIElementSetMessagingTimeout()` which is available on macOS 10.5+. Set a 50ms timeout on the system-wide AX element before querying.

**Rationale**: `AXUIElementSetMessagingTimeout(_:_:)` is declared in `ApplicationServices` and has been available since macOS 10.5. It accepts a `Float` timeout in seconds. Setting 0.05 (50ms) ensures the AX server responds within one keystroke cycle.

**Code pattern**:
```swift
private func checkFocusedElementIsTextInput() -> Bool {
    let systemWide = AXUIElementCreateSystemWide()
    AXUIElementSetMessagingTimeout(systemWide, 0.05)  // 50ms timeout
    var focusedRef: CFTypeRef?
    guard AXUIElementCopyAttributeValue(
        systemWide, kAXFocusedUIElementAttribute as CFString, &focusedRef
    ) == .success, let element = focusedRef else {
        return false
    }
    // ...
}
```

Additionally, `checkSpotlightOnce()` and `checkFocusedElementIsTextInput()` must be dispatched to the main thread (not the IOKit event tap thread):
```swift
// Call from event tap callback:
DispatchQueue.main.async {
    self.checkFocusedElementIsTextInput()
}
// Use cached result in callback:
return cachedIsFocusedOnTextInput
```

**Alternatives considered**:
- `DispatchQueue` timeout with `DispatchWorkItem.cancel()`: does not actually cancel in-flight AX IPC calls; the thread still blocks.
- Skipping AX check entirely on the first keystroke: loses the text-field-only-activation feature.

---

## Finding 5: Async UserDefaults Writes (macOS 11 Compatible)

**Question**: `SettingsManager.saveToDefaults()` is called in every `@Published` `didSet` on the MainActor. How to debounce/batch these writes without blocking the keystroke path?

**Decision**: Since all `@Published` `didSet` observers already run on `MainActor`, introduce a `debounce` helper using `DispatchWorkItem` that coalesces writes within a 300ms window. The in-memory `@Published` value is always current; only the persistence layer is debounced.

**Rationale**: `UserDefaults.set(_:forKey:)` is documented to be non-blocking (it writes to an in-memory cache and flushes periodically). However, on some macOS versions it can synchronously flush when the key is first written. Debouncing eliminates any latency spike during rapid setting changes. The `@Published` value (in-memory truth) is never debounced — only the `userDefaults.set()` call.

**Code pattern**:
```swift
private var saveWorkItems: [String: DispatchWorkItem] = [:]

private func saveToDefaults<T>(_ key: String, value: T) {
    saveWorkItems[key]?.cancel()
    let item = DispatchWorkItem { [weak self] in
        self?.userDefaults.set(value, forKey: key)
    }
    saveWorkItems[key] = item
    DispatchQueue.main.asyncAfter(deadline: .now() + 0.3, execute: item)
}
```

For settings that must persist immediately (e.g., on app quit), call `userDefaults.synchronize()` in `applicationWillTerminate`.

**Alternatives considered**:
- Combine `debounce` operator on a `PassthroughSubject`: clean but requires `Combine` subscription management; adds retention complexity.
- Moving saves to a background `DispatchQueue`: UserDefaults is not thread-safe for concurrent access; all reads/writes must be serialized.

---

## Finding 6: Spotlight Re-Detection Pattern

**Question**: `checkSpotlightOnce()` sets `spotlightChecked = true` and never rechecks. How to detect Spotlight on every re-open?

**Decision**: Remove the `spotlightChecked` one-shot guard from `checkSpotlightOnce()`. Instead, cache the AX query result with a 3-second TTL. Re-query only if the cache has expired.

**Rationale**: The original guard was added to avoid hammering AX on every keystroke. The correct fix is a time-based cache: check once when Spotlight is first detected, cache "Spotlight is active" for 3 seconds. On the next Spotlight open (which resets the TTL because the bundle ID changes), the detection runs fresh.

**Code pattern**:
```swift
private var lastSpotlightCheckTime: Date = .distantPast
private static let spotlightCheckTTL: TimeInterval = 3.0

func checkSpotlightOnce() {
    let now = Date()
    guard now.timeIntervalSince(lastSpotlightCheckTime) > Self.spotlightCheckTTL else { return }
    lastSpotlightCheckTime = now
    // ... perform AX check ...
}

// Reset on app switch (existing resetSpotlightCheck renamed):
func resetSpotlightCache() {
    lastSpotlightCheckTime = .distantPast
}
```

**Alternatives considered**:
- Check on every keystroke with no throttle: too many AX calls; 50-100 µs per call × 120 WPM = ~200ms/min wasted.
- Keep the one-shot flag and reset it differently: requires hooking into system-level Spotlight open/close events, which are not reliably available on macOS 11.

---

## Finding 7: Polling Timer Interval

**Question**: The special-panel polling timer fires every 1.5 seconds. What interval is appropriate?

**Decision**: Increase to 5 seconds.

**Rationale**: Special panels (Spotlight, Raycast, Alfred) are detected via two mechanisms:
1. NSWorkspace `didActivateApplicationNotification` — fires reliably for apps that are proper NSRunningApplications.
2. The polling timer — fallback for panels that don't trigger the notification (e.g., some Spotlight variants on macOS 11).

The polling timer is a fallback. A 5-second interval is a reasonable upper bound on "how long before the user notices the IME mode is wrong in a special panel." Most users type 1-3 characters before the mode is detected and corrected. Five seconds reduces wake-ups from 2400/hr to 720/hr (70% reduction).

**Alternatives considered**:
- 10 seconds: acceptable for battery but unacceptably slow for mode correction in fast-switch scenarios.
- Event-driven only (remove timer): Spotlight on macOS 11 does not reliably emit `didActivateApplicationNotification`; the timer cannot be removed without losing detection entirely.

---

## Summary: All NEEDS CLARIFICATION Resolved

| # | Question | Decision |
|---|----------|----------|
| 1 | CF object release in Swift | Explicit `CFRelease()` in `stop()` |
| 2 | Swift 6 event tap thread model | `nonisolated` hot path, async UI dispatches |
| 3 | Semaphore-free injection | syncProxy path is inline; async paths use background dispatch |
| 4 | AX query timeout macOS 11 | `AXUIElementSetMessagingTimeout()` at 50ms |
| 5 | UserDefaults debounce | 300ms DispatchWorkItem, main queue |
| 6 | Spotlight re-detection | TTL cache (3s) replaces one-shot flag |
| 7 | Polling interval | 5 seconds (from 1.5 seconds) |
