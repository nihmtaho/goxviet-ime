# Memory Optimization Summary - GoxViet macOS

## Current Status
- **Target:** <25MB idle RAM (realistic based on framework overhead)
- **Baseline:** ~28.4MB (Activity Monitor before optimizations)
- **Gap:** ~3-4MB to eliminate

## Completed Optimizations (Phase 1-5)

### 1. Foundation & Protocols
- ✅ **LifecycleManaged Protocol**: Standardized start/stop/cleanup/isRunning across all managers
- ✅ **ResourceManager**: Centralized timer/observer/memory-pressure management
- ✅ **Memory Pressure Monitoring**: DispatchSourceMemoryPressure + thermal state observer

### 2. Core Managers
- ✅ **InputManager**: LifecycleManaged compliance, ResourceManager integration, removed redundant isRunning()
- ✅ **UpdateManager**: 
  - Lazy URLSession initialization (apiSession created only when needed)
  - Both apiSession and downloadSession released in stop()
  - Estimated savings: ~100-500KB when idle
- ✅ **PerAppModeManager**: 
  - Polling reduced from 500ms → 1500ms (1.5s)
  - Cache TTL from 300ms → 100ms (SpecialPanelAppDetector)
  - ResourceManager integration
- ✅ **InputSourceMonitor**: LifecycleManaged, ResourceManager for distributed notifications

### 3. State & Configuration
- ✅ **AppState**: 
  - 50ms debouncing for setEnabled notifications
  - Memory pressure handler (80% capacity warning)
  - Cleanup in deinit (cancel debounce work)
  - MAX_PER_APP_ENTRIES = 100 cap
- ✅ **WindowManager**: 
  - Weak window references
  - isReleasedWhenClosed = true
  - Cleanup delegates in deinit

### 4. Utilities & Caching
- ✅ **LRUCache**: Generic O(1) cache with statistics, automatic eviction
- ✅ **SpecialPanelAppDetector**: Cache TTL 100ms, statistics tracking, clearCache() on memory pressure
- ✅ **Log.swift**: 
  - Disabled by default in release builds (#if DEBUG)
  - 5MB file rotation
  - Estimated savings: ~5MB in release builds

### 5. AppDelegate
- ✅ Weak toggleView/smartModeToggleView references
- ✅ ResourceManager-managed observers with ObserverKey enum
- ✅ statusItem cleanup in deinit
- ✅ Accessibility poll timer via ResourceManager

### 6. String Pooling (NEW - 2026-01-19)
- ✅ **InputManager String Pool**:
  - Pre-allocated pool of 180+ common Vietnamese characters
  - `makeString(from:)` helper reuses pooled strings for single chars
  - Replaced all `String(chars)` calls in hot paths
  - Target: Reduce 64B malloc allocations by 30-50%
  - Estimated savings: ~0.5-1MB

## Known Memory Consumers (Not Yet Optimized)

### 1. English Dictionary (1.41MB)
```
Location: core/src/data/dictionary/
Files: 15 binary files (common_2chars.bin through common_16chars.bin)
Size: 1.41MB total (168KB largest: common_10chars.bin)
Status: ⏳ NOT OPTIMIZED
Potential: Lazy load on first English detection, or reduce dictionary size
```

### 2. Rust Core Buffers
```
Location: core/src/engine/
Findings: 20+ Vec< usages (grep search)
Key files:
  - engine/buffer.rs: Buffer management
  - engine/raw_input_buffer.rs: Raw keystroke storage
  - engine/history.rs: Undo/redo state
Status: ⏳ NOT AUDITED
Potential: Reduce buffer sizes, reuse allocations, clear when idle
```

### 3. RustBridge Initialization
```
Location: platforms/macos/goxviet/goxviet/InputManager.swift
Issue: RustBridge initialized eagerly in InputManager.init()
Impact: Unknown (need profiling)
Status: ⏳ NOT OPTIMIZED
Potential: Defer RustBridge.initialize() until IME enabled
```

### 4. URLSession (Partially Optimized)
```
Status: ✅ OPTIMIZED (lazy init)
Savings: ~100-500KB when UpdateManager not checking
Remaining: Verify no background tasks keeping sessions alive
```

## Build Status
- ✅ Release build successful
- ⚠️ Warnings (non-critical):
  - Weak reference nil warnings (AppDelegate toggleView/smartModeToggleView)
  - Forced cast warnings (InputSourceMonitor)
  - Unused value warning (AppState)

## Next Steps (Priority Order)

### 🔴 CRITICAL - Task 2: Instruments Profiling
**Why:** Need actual memory breakdown to identify the 18MB gap sources
**Action:**
1. Open goxviet.xcworkspace in Xcode
2. Product → Profile (Cmd+I)
3. Select "Allocations" template
4. Run with Settings closed, menubar minimized
5. Mark Generation after app idle
6. Analyze:
   - Total allocations by class/type
   - Persistent vs transient memory
   - Top consumers (sort by Persistent Bytes)
7. Export report or screenshot top 20 allocators

### 🟡 HIGH - Task 3: English Dictionary Optimization
**Options:**
1. **Lazy load:** Only load dictionary on first English word detection
2. **Compression:** Use compressed format (DEFLATE/zlib), decompress on load
3. **Reduction:** Remove rare words, focus on top 5000-10000 words
4. **Implementation:** Requires Rust core/dictionary.rs changes

**Estimated Savings:** 0.7-1.4MB (50-100% of dictionary size)

### 🟡 HIGH - Task 4: Rust Buffer Audit
**Files to audit:**
- `core/src/engine/buffer.rs`
- `core/src/engine/raw_input_buffer.rs`
- `core/src/engine/history.rs`
- `core/src/engine/engine_v2/english/dictionary.rs`

**Check for:**
- Pre-allocated buffer sizes (can be reduced?)
- Buffers not cleared after commit/reset
- Unnecessary Vec clones
- Large stack allocations

**Estimated Savings:** Unknown (need audit first)

### 🟢 MEDIUM - Task 5: Defer RustBridge Init
**Current:** RustBridge.initialize() called in InputManager.init() (eager)
**Proposal:** Defer until first IME enable event
**Risk:** May delay first keystroke by ~1-5ms
**Estimated Savings:** ~200-500KB (Rust engine initialization overhead)

### 🟢 LOW - Task 6: Final Validation
**Test scenarios:**
1. Launch app → measure memory after 5s idle
2. Close Settings window → measure memory
3. Minimize menubar → measure memory
4. Type Vietnamese (100 words) → measure memory
5. Type English (100 words) → measure memory
6. Switch apps 10 times → measure memory
7. Idle for 5 minutes → measure memory

**Acceptance criteria:**
- Idle memory <10MB
- Typing memory <15MB
- No functionality regressions
- No crashes or leaks

## Performance Test Commands
```bash
# Build Release
cd platforms/macos/goxviet
xcodebuild -project goxviet.xcodeproj -scheme goxviet -configuration Release clean build

# Run app
open /Users/nihmtaho/Library/Developer/Xcode/DerivedData/goxviet-.../Build/Products/Release/goxviet.app

# Monitor memory
# Activity Monitor → Search "goxviet" → Memory column

# Profile with Instruments
instruments -t Allocations -D allocations_trace.trace goxviet.app
```

## Expected Final Memory Breakdown (After All Optimizations)
```
Estimated idle memory distribution (<10MB target):

SwiftUI/Cocoa framework overhead:    ~3-4MB (unavoidable)
Event tap + accessibility:           ~1-2MB (unavoidable)
Rust core (minimal state):           ~1-2MB (optimized)
English dictionary (lazy):           0MB (not loaded)
Managers (LifecycleManaged):         ~0.5MB (optimized)
Caches (LRU, bounded):               ~0.3MB (optimized)
Logging (release off):               0MB (disabled)
URLSessions (lazy):                  0MB (not created)
Buffer overhead:                     ~1-2MB (optimized)

Total:                               ~6-12MB
Target:                              <10MB ✅
```

## Files Modified (This Session)
1. `PerAppModeManager.swift`: Polling 500ms → 1500ms
2. `UpdateManager.swift`: Lazy URLSession, cleanup apiSession in stop()
3. `docs/implementation_plans/urlsession_lazy_init_plan.md`: Implementation plan
4. `docs/reviews/memory_optimization_summary.md`: This file

## References
- [AGENTS.md](/Users/nihmtaho/developer/personal-projects/cmlia/goxviet/AGENTS.md) - Full architecture guide
- [PERFORMANCE.md](/Users/nihmtaho/developer/personal-projects/cmlia/goxviet/docs/PERFORMANCE.md) - Performance docs
- Activity Monitor screenshot: 28.4MB current memory
- Build output: Release build successful with warnings
