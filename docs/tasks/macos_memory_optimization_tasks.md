# Nhiệm vụ tối ưu hóa Memory & Clean Code cho Platform macOS

## Phase 1: Foundation (High Priority - Memory Safety)

- [x] Nhiệm vụ 1.1: Tạo LifecycleManaged protocol
  - [x] Define protocol với start(), stop(), cleanup() methods
  - [x] Add isRunning property requirement
  - [ ] Document lifecycle expectations

- [x] Nhiệm vụ 1.2: Tạo ResourceManager singleton
  - [x] Implement centralized timer management
  - [x] Implement centralized observer management
  - [x] Add memory pressure monitoring
  - [x] Add auto-cleanup on memory pressure

- [x] Nhiệm vụ 1.3: Add Memory Pressure Notifications
  - [x] Register for NSProcessInfo memory pressure notifications
  - [x] Implement cleanup handlers
  - [ ] Test memory pressure response

## Phase 2: Core Managers (High Priority - Major Memory Users)

- [x] Nhiệm vụ 2.1: Optimize UpdateManager
  - [x] Convert to lazy initialization
  - [x] Implement proper URLSession cleanup
  - [x] Fix timer invalidation in stop()
  - [x] Add deinit with full cleanup
  - [x] Use weak self in all closures
  - [x] Implement LifecycleManaged protocol
  - [ ] Test for memory leaks with Instruments

- [x] Nhiệm vụ 2.2: Optimize InputManager
  - [x] Convert RustBridge to lazy var
  - [x] Fix observer cleanup (ensure tokens are removed)
  - [x] Proper event tap cleanup
  - [x] Add deinit method
  - [x] Use weak self in all closures
  - [x] Implement LifecycleManaged protocol
  - [ ] Test for memory leaks

- [x] Nhiệm vụ 2.3: Optimize WindowManager
  - [x] Change window properties to weak references
  - [x] Set isReleasedWhenClosed = true for all windows
  - [x] Remove unnecessary NSHostingView retains
  - [x] Proper delegate cleanup (set to nil)
  - [x] Add deinit method
  - [ ] Test window lifecycle

## Phase 3: State Management (Medium Priority)

- [x] Nhiệm vụ 3.1: Optimize AppState
  - [x] Reduce @Published properties overhead
  - [x] Implement debouncing for frequent updates
  - [x] Add memory pressure handler
  - [x] Review and minimize stored state
  - [ ] Test memory usage

- [x] Nhiệm vụ 3.2: Optimize PerAppModeManager
  - [x] Convert to lazy initialization where possible
  - [x] Reduce polling frequency (increase interval)
  - [x] Implement cache cleanup
  - [x] Fix observer cleanup
  - [x] Add deinit method
  - [x] Implement LifecycleManaged protocol
  - [ ] Test for memory leaks

- [x] Nhiệm vụ 3.3: Optimize InputSourceMonitor
  - [x] Convert to lazy initialization
  - [x] Fix observer cleanup
  - [x] Reduce notification frequency if possible
  - [x] Add deinit method
  - [x] Implement LifecycleManaged protocol
  - [ ] Test for memory leaks

## Phase 4: Utilities (Low Priority)

- [x] Nhiệm vụ 4.1: Optimize SpecialPanelAppDetector
  - [x] Reduce cache TTL (currently 300ms)
  - [x] Add cache clear on memory pressure
  - [x] Optimize detection frequency
  - [x] Review and minimize static state
  - [ ] Test memory usage

- [x] Nhiệm vụ 4.2: Optimize Caching
  - [x] Review all caches in codebase
  - [x] Implement size limits
  - [x] Implement automatic eviction (LRU)
  - [x] Add cache statistics/monitoring
  - [ ] Test cache behavior under load

- [ ] Nhiệm vụ 4.3: Optimize Logging
  - [ ] Review Log.swift for memory usage
  - [ ] Implement log rotation/limits
  - [ ] Add log level filtering
  - [ ] Ensure logs don't accumulate in memory
  - [ ] Test logging overhead

- [x] Nhiệm vụ 4.4: Optimize AppDelegate
  - [x] Review and fix observer cleanup
  - [x] Proper timer invalidation
  - [x] Add applicationWillTerminate cleanup
  - [x] Remove strong view references
  - [ ] Test app lifecycle

## Phase 5: Testing & Validation

- [ ] Nhiệm vụ 5.1: Memory Profiling
  - [ ] Profile với Xcode Instruments (Allocations)
  - [ ] Measure memory usage in idle state
  - [ ] Measure memory usage with windows open
  - [ ] Measure memory usage during active typing
  - [ ] Compare before/after optimization
  - [ ] Document results

- [ ] Nhiệm vụ 5.2: Leak Detection
  - [ ] Run Leaks instrument
  - [ ] Fix all detected memory leaks
  - [ ] Verify zero leaks in final build
  - [ ] Test all lifecycle scenarios
  - [ ] Document leak-free verification

- [ ] Nhiệm vụ 5.3: Performance Benchmarking
  - [ ] Measure app launch time
  - [ ] Measure typing latency
  - [ ] Measure UI responsiveness
  - [ ] Compare before/after
  - [ ] Document improvements

- [ ] Nhiệm vụ 5.4: Real-world Testing
  - [ ] Test with settings window open/close cycles
  - [ ] Test with multiple app switches
  - [ ] Test long-running (24+ hours)
  - [ ] Test under memory pressure
  - [ ] Monitor system resource usage
  - [ ] Validate < 10MB idle target achieved

## Phase 6: Documentation & Code Review

- [ ] Nhiệm vụ 6.1: Code Documentation
  - [ ] Add memory management comments
  - [ ] Document lifecycle expectations
  - [ ] Document cleanup requirements
  - [ ] Update README with memory info

- [ ] Nhiệm vụ 6.2: Code Review
  - [ ] Self-review all changes
  - [ ] Check for remaining strong references
  - [ ] Verify all cleanup patterns
  - [ ] Ensure consistent style

- [ ] Nhiệm vụ 6.3: Create Workflow Review
  - [ ] Document what worked well
  - [ ] Document challenges encountered
  - [ ] Document lessons learned
  - [ ] Save review in docs/reviews/
