# Data Model: macOS Input Pipeline Bug Fixes & Optimization

**Feature**: 003-macos-input-optimization  
**Date**: 2026-04-12

This feature does not introduce new data entities. It modifies the lifecycle and threading behavior of existing components. This document captures state transitions and invariants for the components being changed.

---

## Component: InputManager (CF Object Lifecycle)

### State Machine

```
STOPPED ──start()──► RUNNING ──stop()──► STOPPED
                                │
                         [CF objects released]
```

### Invariants

| State | `eventTap` | `runLoopSource` | `isRunning` |
|-------|-----------|----------------|-------------|
| STOPPED | `nil` | `nil` | `false` |
| RUNNING | `CFMachPort` (retained) | `CFRunLoopSource` (retained) | `true` |

### CF Object Lifecycle

```
start():
  CGEvent.tapCreate()            → eventTap (+1 CF retain)
  CFMachPortCreateRunLoopSource() → runLoopSource (+1 CF retain)
  CFRunLoopAddSource()           → runLoop holds +1 ref on runLoopSource

stop():
  CFRunLoopRemoveSource()        → runLoop releases its ref on runLoopSource
  CFRelease(runLoopSource)       → balance our +1 retain → deallocated
  CGEvent.tapEnable(enable:false)
  CFRelease(eventTap)            → balance our +1 retain → deallocated
```

**Validation rule**: After `stop()`, both `eventTap` and `runLoopSource` MUST be `nil` AND their CF retain counts MUST be zero.

---

## Component: TextInjectionHelper (Injection Path)

### Injection Path Classification

| Method | Called from | Thread | Semaphore needed? |
|--------|------------|--------|------------------|
| `syncProxy` | Event tap callback | IOKit thread | NO — inline, synchronous via proxy |
| `fast`, `slow`, `instant`, `charByChar` | Dispatched off callback | Background queue | YES — prevents concurrent event posting |
| `selection`, `emptyCharPrefix` | Dispatched off callback | Background queue | YES |
| `axDirect` | Dispatched off callback | Background queue | YES |

### State: Semaphore Guard

```
IDLE (value=1) ──inject(async path)──► INJECTING (value=0) ──complete──► IDLE
```

**Invariant**: The event tap callback thread (IOKit) MUST never call `semaphore.wait()`. Only background-dispatched injection calls enter the guarded section.

---

## Component: PerAppModeManagerEnhanced (Spotlight Detection)

### Spotlight Cache State

```
STALE (distantPast) ──checkSpotlightOnce()──► FRESH (timestamp)
                                                      │
                                               [TTL expires after 3s]
                                                      │
                                                    STALE
```

**Validation rule**: `lastSpotlightCheckTime` MUST be reset to `.distantPast` on every app switch (in `resetSpotlightCache()`).

### Polling Timer State

| Property | Before Fix | After Fix |
|----------|-----------|-----------|
| Interval | 1.5 seconds | 5.0 seconds |
| Start condition | Always on IME start | Always on IME start |
| Stop condition | `stop()` called | `stop()` called |

---

## Component: SettingsManager (Persistence)

### Write Debounce State

```
for each settings key:

  VALUE_CHANGED ──saveToDefaults()──► PENDING (DispatchWorkItem scheduled)
                                              │
                                       [within 300ms: another change]
                                              │
                                       PENDING (previous cancelled, new item scheduled)
                                              │
                                       [300ms elapsed, no further changes]
                                              │
                                        WRITTEN (userDefaults.set())
```

**Invariant**: The in-memory `@Published` value is ALWAYS the source of truth. The UserDefaults store may lag by up to 300ms. On `applicationWillTerminate`, a synchronous flush is performed.

### Per-App Modes Capacity

| Property | Current | After Fix |
|----------|---------|-----------|
| `MAX_PER_APP_ENTRIES` | 200 | 200 (unchanged) |
| At-capacity behavior | Silent drop | LRU eviction of least-recently-accessed entry |

**LRU key**: bundle ID string. Access time updated on both read (`getPerAppMode`) and write (`setPerAppMode`).
