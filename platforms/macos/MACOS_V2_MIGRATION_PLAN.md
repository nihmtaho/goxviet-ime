# macOS Platform v2 API Migration Plan

## Current State Analysis

**Problem:** macOS platform code uses deleted v1 FFI API, causing linker errors.

**Root Cause:**
- Core Rust library v1 API deleted (~1,163 LOC) in Phase 8
- macOS Swift code still imports and calls v1 functions
- RustBridgeV2.swift created but not integrated

**Affected Files:**
1. `InputManager.swift` - Direct v1 calls: `ime_key()`, `ime_key_ext()`
2. `RustBridgeSafe.swift` - v1 wrapper with global state
3. v2 Ready: `RustBridgeV2.swift` - Complete v2 bridge (not yet used)

## Migration Strategy

### Phase 1: Add RustBridgeV2 to Xcode (NO code changes)

**Goal:** Make v2 available without breaking existing code.

**Steps:**
1. Add `RustBridgeV2.swift` to Xcode project
2. Verify it compiles alongside RustBridgeSafe
3. No changes to InputManager yet

**Risk:** Low (just adding file)

### Phase 2: Create v2 Wrapper Layer

**Goal:** Allow InputManager to call either v1 or v2 via same interface.

**Approach:**
- Create `RustBridgeAdapter.swift` with protocol
- Implement both v1 and v2 adapters
- InputManager calls adapter, not direct FFI

**Benefits:**
- Gradual migration
- Can test v2 without breaking production
- Easy rollback

### Phase 3: Switch InputManager to v2

**Goal:** Update InputManager to use RustBridgeV2.

**Changes:**
- Replace `ime_key()` with `RustBridgeV2.shared.processKey()`
- Replace `RustBridgeSafe.shared.initialize()` with v2 engine creation
- Update all configuration calls

**Risk:** Medium (behavior changes)

### Phase 4: Remove v1 Code

**Goal:** Delete RustBridgeSafe.swift and adapters.

**Cleanup:**
- Delete `RustBridgeSafe.swift`
- Delete adapter layer
- Update documentation

**Risk:** Low (v2 already working)

## Quick Fix Option (RECOMMENDED)

**For immediate build fix:**

Instead of full migration, temporarily restore v1 API in Rust core.

**Pros:**
- Unblocks builds immediately
- Allows incremental Swift migration
- Can test v2 alongside v1

**Cons:**
- Delays v1 removal
- Maintains dead code temporarily

**Implementation:**
1. Restore `src/lib_v1_backup.rs` → `src/lib.rs`
2. Re-add feature flag: `legacy = []` to Cargo.toml
3. Build works again
4. Migrate Swift code gradually
5. Remove v1 in Phase 8 Task 5 (after Swift migration)

## Detailed Implementation Plan

### Option A: Full Migration (2-3 days)

**Day 1: Preparation**
- [x] Create RustBridgeV2.swift
- [ ] Add to Xcode project
- [ ] Create RustBridgeAdapter protocol
- [ ] Implement v2 adapter

**Day 2: InputManager Migration**
- [ ] Update initialize() to use v2
- [ ] Update processKey() calls
- [ ] Update configuration methods
- [ ] Test keyboard input

**Day 3: Cleanup & Testing**
- [ ] Remove RustBridgeSafe
- [ ] Update all references
- [ ] Full regression testing
- [ ] Update documentation

### Option B: Restore v1 Temporarily (1-2 hours)

**Immediate:**
- [x] Backup created: `src/lib_v1_backup.rs`
- [ ] Restore backup to `src/lib.rs`
- [ ] Add feature flag
- [ ] Rebuild library
- [ ] Verify macOS builds

**Later (Phase 8 Task 5):**
- [ ] Migrate Swift to v2
- [ ] Remove v1 again
- [ ] Complete Phase 8

## Current Status

- ✅ v2 API implemented and tested in Rust
- ✅ v2 API works in standalone C/Swift tests
- ✅ RustBridgeV2.swift created
- ❌ RustBridgeV2 not added to Xcode
- ❌ InputManager not migrated
- ❌ Build broken (linker errors)

## Recommendation

**Choose Option B (Restore v1 Temporarily):**

**Reasons:**
1. Unblocks development immediately
2. Allows proper testing of v2 before migration
3. Can migrate Swift code incrementally
4. Less risk of breaking production code
5. Follows original Phase 7-8 plan (v2 coexists, then remove v1)

**Timeline:**
- Now: Restore v1 (1 hour)
- Phase 8 Task 4-5: Migrate Swift (2-3 days)
- Phase 8 Task 6: Remove v1 final (1 hour)

## Next Steps

### Immediate (Option B):

```bash
# 1. Restore v1 API
cd core
cp src/lib_v1_backup.rs src/lib.rs

# 2. Re-add feature flag
# Edit Cargo.toml, add:
# [features]
# default = ["legacy"]
# legacy = []

# 3. Rebuild
cargo build --release

# 4. Copy to macOS
./scripts/rust_build_lib_universal_for_macos.sh

# 5. Test Xcode build
# Open Xcode, build should work
```

### Later (Phase 8 Task 4-5):

1. Add RustBridgeV2 to Xcode
2. Create adapter layer
3. Migrate InputManager
4. Test thoroughly
5. Remove v1 code

## Conclusion

**Decision:** Restore v1 temporarily to unblock builds.

**Impact:**
- Phase 8 progress: 3/9 → remains 3/9 (Task 4-5 pending Swift migration)
- Can continue other Phase 8 tasks
- Proper v2 migration happens in controlled manner

**Files Modified:**
- `core/src/lib.rs` (restore from backup)
- `core/Cargo.toml` (re-add feature flag)
- `platforms/macos/MACOS_V2_MIGRATION_PLAN.md` (this document)

---

*Created: 2026-02-12*  
*Status: Awaiting decision*
