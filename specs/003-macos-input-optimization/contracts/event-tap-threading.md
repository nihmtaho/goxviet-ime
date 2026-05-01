# Contract: Event Tap Threading Model

**Feature**: 003-macos-input-optimization  
**Date**: 2026-04-12

This contract defines the threading invariants that ALL callers of `InputManager.handleEvent()` and related hot-path functions must uphold after this feature lands.

---

## Threading Zones

```
┌─────────────────────────────────────────────────────────┐
│  IOKit Thread (CGEventTap callback)                      │
│  ─────────────────────────────────                       │
│  • handleEvent(event:type:proxy:)           [nonisolated]│
│  • RustBridgeSafe.processKey()              [nonisolated]│
│  • TextInjector.injectSync(syncProxy path)  [nonisolated]│
│  • Read cached settings snapshots           [nonisolated]│
│  ─────────────────────────────────                       │
│  FORBIDDEN in this zone:                                 │
│    ✗ semaphore.wait()                                    │
│    ✗ DispatchQueue.main.sync { }                         │
│    ✗ AXUIElement queries (unless timeout guarded + async)│
│    ✗ userDefaults.set()                                  │
└─────────────────────────────────────────────────────────┘
                        │
              dispatch async (non-blocking)
                        │
┌─────────────────────────────────────────────────────────┐
│  MainActor (@MainActor)                                  │
│  ─────────────────────                                   │
│  • SettingsManager @Published property updates           │
│  • PerAppModeManagerEnhanced state updates               │
│  • UserDefaults writes (debounced 300ms)                 │
│  • AX queries (with 50ms timeout)                        │
│  • UI updates (SwiftUI / AppKit)                         │
└─────────────────────────────────────────────────────────┘
                        │
              background queue (concurrent)
                        │
┌─────────────────────────────────────────────────────────┐
│  Injection Background Queue                              │
│  ─────────────────────────                               │
│  • TextInjector.injectSync (non-syncProxy methods)       │
│  • semaphore.wait() / signal() — ALLOWED here            │
└─────────────────────────────────────────────────────────┘
```

---

## Cached Settings Snapshot Contract

Settings needed on the hot path MUST be cached as atomic values that are readable from `nonisolated` contexts without locking:

| Setting | Cached As | Updated On |
|---------|-----------|-----------|
| `isEnabled` | `nonisolated(unsafe) var cachedIsEnabled: Bool` | MainActor, on `SettingsManager.isEnabled.didSet` |
| `currentAppMode` | `nonisolated(unsafe) var cachedCurrentAppMode: Bool` | MainActor, on app switch notification |
| `inputMethod` | `nonisolated(unsafe) var cachedInputMethod: Int` | MainActor, on `SettingsManager.inputMethod.didSet` |

**Write protocol**: All cache updates happen on MainActor (single writer). The IOKit callback reads them without a lock (single reader per key). This is safe because:
1. `Bool` and `Int` reads/writes are atomic on ARM64 and x86-64.
2. The worst case (stale read for one keystroke) is acceptable — the next keystroke will see the updated value.

---

## AX Query Contract

- ALL `AXUIElement` queries MUST be dispatched to `DispatchQueue.main`.
- A `AXUIElementSetMessagingTimeout(element, 0.05)` call MUST precede every system-wide AX query.
- Results from AX queries MUST be stored in a cached field (TTL ≥ 1 second for stable values, 3 seconds for Spotlight detection).
- The event tap callback MUST read the cached result, NOT perform an inline AX query.

---

## `TextInjector.injectSync` Call Contract

| Condition | Allowed to call from IOKit thread? |
|-----------|----------------------------------|
| `method == .syncProxy` | YES — inline, no semaphore |
| `method == .passthrough` | YES — no-op |
| Any other method | NO — must dispatch to background queue first |

**Enforcement**: `injectSync` MUST assert that `.syncProxy` and `.passthrough` are the only methods called when `Thread.isMainThread == false && !isBackgroundInjectionQueue`.
