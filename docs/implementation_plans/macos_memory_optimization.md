# Kế hoạch triển khai tối ưu hóa Memory & Clean Code cho Platform macOS

## Mô tả
Tối ưu hóa platform macOS của GoxViet để sử dụng RAM < 10MB khi không mở menubar và settings UI, đồng thời cải thiện clean code, loại bỏ memory leak, và tăng maintainability.

## Problem Issues

### Current Issues
1. **Memory Usage:** App có thể sử dụng > 10MB RAM ngay cả khi idle do:
   - SwiftUI views được giữ trong memory
   - Observers không được cleanup đúng cách
   - Timer không được invalidate
   - URLSession không được cleanup
   - Window instances giữ strong references
   - Cache không được giới hạn

2. **Memory Leaks:**
   - NotificationCenter observers không được remove đúng
   - Timer trong UpdateManager, PerAppModeManager, SpecialPanelAppDetector không được invalidate
   - URLSession trong UpdateManager không được cleanup
   - Strong reference cycles giữa classes
   - Window delegates không được nil

3. **Code Quality:**
   - Duplicate code trong window management
   - Không consistent trong observer cleanup patterns
   - Cache không có size limit rõ ràng
   - Logging có thể tốn memory nếu excessive
   - Không sử dụng lazy initialization cho expensive objects

4. **Maintainability:**
   - Setup/teardown logic không symmetric
   - Lifecycle management không rõ ràng
   - Không có protocol abstraction cho managers
   - Hard to test do tight coupling

### Root Causes
1. **Over-allocation:** Tạo objects ngay từ đầu thay vì lazy initialization
2. **Missing Cleanup:** Không implement deinit hoặc cleanup methods đầy đủ
3. **Strong References:** Không sử dụng [weak self] đúng cách trong closures
4. **Eager Loading:** Load tất cả resources ngay lập tức thay vì on-demand
5. **No Resource Management:** Không có centralized resource manager
6. **Lack of Weak References:** Delegates và observers không weak

## Proposed Changes

### 1. Memory Optimization
- **Lazy Initialization:** Chuyển các managers sang lazy initialization
- **Weak References:** Sử dụng weak references cho all observers và delegates
- **Resource Cleanup:** Implement comprehensive deinit methods
- **Timer Management:** Centralize timer lifecycle management
- **Window Management:** Proper window lifecycle với weak references
- **Cache Limits:** Implement size-limited caches với automatic eviction

### 2. Code Structure Improvements
- **Protocol-Based Design:** Định nghĩa protocols cho Managers
- **Resource Manager:** Tạo centralized ResourceManager
- **Lifecycle Protocol:** Định nghĩa LifecycleManaged protocol
- **Cleanup Patterns:** Standardize cleanup patterns across classes
- **Dependency Injection:** Giảm tight coupling

### 3. Specific File Changes

#### AppDelegate.swift
- Chuyển managers sang lazy var
- Implement proper cleanup trong applicationWillTerminate
- Remove strong references to views
- Proper timer invalidation

#### AppState.swift
- Reduce published properties overhead
- Implement debouncing cho frequent updates
- Add memory pressure notifications handling

#### InputManager.swift
- Lazy initialization cho RustBridge
- Proper cleanup cho event tap
- Weak references cho all observers
- Cache cleanup

#### WindowManager.swift
- Weak window references
- Auto-release windows on close
- Remove unnecessary NSHostingView retains
- Proper delegate cleanup

#### UpdateManager.swift
- Stop URLSession when not needed
- Lazy initialization cho sessions
- Proper timer management
- Clear download cache

#### PerAppModeManager.swift
- Optimize polling frequency
- Lazy workspace notification setup
- Clear app state cache periodically

#### SpecialPanelAppDetector.swift
- Reduce cache TTL
- Clear cache on memory pressure
- Optimize detection frequency

#### InputSourceMonitor.swift
- Lazy initialization
- Proper observer cleanup
- Reduce notification frequency

#### TextInjectionHelper.swift
- No persistent state (already good)
- Ensure semaphore cleanup

## Implementation Order

### Phase 1: Foundation (High Priority - Memory Safety)
1. Define Lifecycle Protocols
2. Implement ResourceManager
3. Add Memory Pressure Monitoring

### Phase 2: Core Managers (High Priority - Major Memory Users)
1. UpdateManager optimization
2. InputManager optimization
3. WindowManager optimization

### Phase 3: State Management (Medium Priority)
1. AppState optimization
2. PerAppModeManager optimization
3. InputSourceMonitor optimization

### Phase 4: Utilities (Low Priority)
1. SpecialPanelAppDetector optimization
2. Cache optimization
3. Logging optimization

### Phase 5: Testing & Validation
1. Memory profiling với Instruments
2. Leak detection với Xcode
3. Performance benchmarking
4. Real-world usage testing

## Thời gian dự kiến

- **Phase 1:** 2-3 hours (Foundation)
- **Phase 2:** 3-4 hours (Core Managers)
- **Phase 3:** 2-3 hours (State Management)
- **Phase 4:** 1-2 hours (Utilities)
- **Phase 5:** 2-3 hours (Testing)

**Total:** 10-15 hours

## Tài nguyên cần thiết

1. **Xcode Instruments:** Memory profiler, Leaks instrument
2. **Testing Device:** macOS device cho real-world testing
3. **Documentation:** Swift memory management best practices
4. **Reference:** Apple's Memory Management Programming Guide

## Expected Outcomes

### Memory Targets
- **Idle State (no windows):** < 10MB
- **With Settings Open:** < 30MB
- **Active Typing:** < 15MB
- **Update Check:** < 20MB

### Code Quality Improvements
- Zero memory leaks detected by Instruments
- All observers properly cleaned up
- All timers properly invalidated
- All sessions properly terminated
- Clear lifecycle management
- Testable architecture

### Performance Improvements
- Faster app launch (lazy initialization)
- Lower CPU usage (optimized polling)
- Better responsiveness (reduced overhead)
- Smoother memory profile (no spikes)
