# GoxViet Core - Legacy Code Cleanup Plan

This document provides a detailed plan for reviewing, marking, and eventually removing legacy code from the GoxViet core engine.

---

## Table of Contents

1. [Overview](#overview)
2. [Legacy Code Inventory](#legacy-code-inventory)
3. [Deprecation Strategy](#deprecation-strategy)
4. [Cleanup Phases](#cleanup-phases)
5. [Safety Checks](#safety-checks)
6. [Risk Assessment](#risk-assessment)

---

## Overview

### Goals

- ✅ Identify all legacy modules that can be removed
- ✅ Mark them with deprecation warnings
- ✅ Provide migration path for any external users
- ✅ Remove after grace period with zero risk

### Status

- **Phase 1-4**: ✅ New architecture complete (415 tests passing)
- **Phase 5**: ✅ Documentation complete
- **Phase 6**: ⏳ Legacy cleanup (this plan)

---

## Legacy Code Inventory

### Modules to Remove

| Module | Size | Status | Replacement | Risk | Priority |
|--------|------|--------|-------------|------|----------|
| `engine/` | ~2000 LOC | ⚠️ Legacy | `domain/entities/` | Low | High |
| `engine_v2/` | ~3000 LOC | ⚠️ Legacy | `domain/entities/` + `infrastructure/` | Low | High |
| `processors/` | ~1500 LOC | ⚠️ Wrapped | `infrastructure/adapters/input/` | Low | Medium |
| `validators/` | ~1000 LOC | ⚠️ Wrapped | `infrastructure/adapters/validation/` | Low | Medium |
| `transformers/` | ~800 LOC | ⚠️ Wrapped | `infrastructure/adapters/transformation/` | Low | Medium |
| `state/` | ~500 LOC | ⚠️ Wrapped | `infrastructure/adapters/state/` | Low | Medium |
| `utils.rs` | ~200 LOC | ⚠️ Legacy | `shared/` | Low | Low |
| `input/` | ~400 LOC | ⚠️ Legacy | `infrastructure/adapters/input/` | Low | Low |

**Total Legacy Code:** ~9,400 LOC (~60% of codebase)

---

### Modules to Keep

| Module | Reason |
|--------|--------|
| `lib.rs` | Public FFI API - must maintain |
| `data/` | Vietnamese language data - migrated to repositories |
| `domain/` | New clean architecture - keep |
| `application/` | New clean architecture - keep |
| `infrastructure/` | New clean architecture - keep |
| `presentation/` | New clean architecture - keep |
| `shared/` | Common utilities - keep |

---

## Deprecation Strategy

### Step 1: Add Deprecation Warnings

Mark all legacy modules with `#[deprecated]` attribute.

**Example:**

```rust
// src/engine/mod.rs
#![deprecated(
    since = "2.0.0",
    note = "This module is deprecated. Use `domain::entities` and `infrastructure::adapters` instead. \
            See MIGRATION_STRATEGY.md for details."
)]

pub mod buffer;
pub mod syllable;
// ... rest of module
```

**Files to mark:**

1. `src/engine/mod.rs`
2. `src/engine_v2/mod.rs`
3. `src/processors/mod.rs`
4. `src/validators/mod.rs`
5. `src/transformers/mod.rs`
6. `src/state/mod.rs`
7. `src/input/mod.rs`
8. `src/utils.rs`

---

### Step 2: Add Migration Guide Comments

Add comments at the top of each deprecated module with migration path.

**Example:**

```rust
// src/processors/telex.rs

//! # DEPRECATED
//!
//! This module is deprecated and will be removed in v3.0.0.
//!
//! ## Migration Path
//!
//! Instead of:
//! ```rust,ignore
//! use goxviet_core::processors::telex::TelexProcessor;
//! let processor = TelexProcessor::new();
//! ```
//!
//! Use:
//! ```rust
//! use goxviet_core::infrastructure::adapters::input::TelexAdapter;
//! use goxviet_core::domain::ports::input::InputMethod;
//!
//! let adapter: Box<dyn InputMethod> = Box::new(TelexAdapter::new());
//! ```
//!
//! See [MIGRATION_STRATEGY.md](../MIGRATION_STRATEGY.md) for full details.

#[deprecated(since = "2.0.0", note = "Use infrastructure::adapters::input::TelexAdapter")]
pub struct TelexProcessor {
    // ... legacy implementation
}
```

---

### Step 3: Update Public Exports

Hide deprecated modules from public API in `lib.rs`.

**Current:**

```rust
// lib.rs
pub mod engine;
pub mod engine_v2;
pub mod processors;
// ... etc
```

**Updated:**

```rust
// lib.rs

// New clean architecture (public API)
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;
pub mod shared;

// Legacy modules (hidden by default, available with feature flag)
#[cfg(feature = "legacy")]
#[deprecated(since = "2.0.0", note = "Use new clean architecture modules")]
pub mod engine;

#[cfg(feature = "legacy")]
#[deprecated(since = "2.0.0", note = "Use infrastructure::adapters")]
pub mod processors;

// ... etc
```

---

## Cleanup Phases

### Phase 6.1: Deprecation Warnings (Week 15)

**Tasks:**

1. ✅ Add `#[deprecated]` to all legacy modules
2. ✅ Add migration guide comments
3. ✅ Update public exports in `lib.rs`
4. ✅ Update `Cargo.toml` with feature flags:

```toml
[features]
default = []
legacy = []  # Enable legacy modules
```

5. ✅ Build and verify warnings appear:

```bash
cargo build --all-features
# Should see deprecation warnings
```

6. ✅ Commit with message: `chore: mark legacy modules as deprecated`

**Deliverables:**
- All legacy code marked deprecated
- Migration guide comments added
- Feature flag `legacy` available

---

### Phase 6.2: Grace Period (Releases 2.0-2.2)

**Duration:** 2-3 stable releases (~2-3 months)

**Goals:**
- Monitor for external usage of legacy code
- Respond to migration questions
- Fix any issues in new architecture

**Tasks:**

1. **Release 2.0** (immediately after Phase 6.1)
   - Include deprecation warnings
   - Update CHANGELOG.md with deprecation notice
   - Announce on GitHub/blog

2. **Release 2.1** (1 month later)
   - Monitor GitHub issues for migration problems
   - Fix any bugs in new architecture
   - Continue deprecation warnings

3. **Release 2.2** (2 months later)
   - Final grace release
   - Announce legacy code removal in 3.0

**Monitoring:**

```bash
# Check for external dependencies
cargo tree | grep goxviet_core

# Check for GitHub issues mentioning legacy
gh issue list --label "legacy" --label "migration"
```

---

### Phase 6.3: Final Removal (Release 3.0)

**Prerequisites:**
- [ ] At least 2 stable releases with deprecation warnings
- [ ] No critical bugs in new architecture
- [ ] No external dependencies on legacy modules
- [ ] Community informed via changelog/blog

**Tasks:**

1. **Delete legacy modules:**

```bash
# Remove legacy directories
rm -rf src/engine/
rm -rf src/engine_v2/
rm -rf src/processors/
rm -rf src/validators/
rm -rf src/transformers/
rm -rf src/state/
rm -rf src/input/
rm src/utils.rs
```

2. **Clean up imports:**

```bash
# Remove legacy feature flag
# Edit Cargo.toml, remove:
# [features]
# legacy = []

# Remove conditional compilation
# Edit lib.rs, remove:
# #[cfg(feature = "legacy")]
# pub mod engine;
# ... etc
```

3. **Update tests:**

```bash
# Remove tests for deleted modules
cargo test
# All should still pass (only clean architecture tests remain)
```

4. **Update documentation:**

```bash
# Update CHANGELOG.md
# Remove legacy references from README.md
# Update ARCHITECTURE.md if needed
```

5. **Verify build:**

```bash
cargo clean
cargo build --release
cargo test --release
cargo clippy
```

6. **Commit and tag:**

```bash
git add -A
git commit -m "chore: remove legacy code (v3.0.0)"
git tag -a v3.0.0 -m "Release 3.0.0: Legacy code removed"
git push origin main --tags
```

**Result:**
- ~9,400 LOC removed
- Codebase ~40% smaller
- Only clean architecture remains

---

## Safety Checks

### Pre-Removal Checklist

Before removing legacy code, verify:

- [ ] **FFI API unchanged**: All FFI functions still work
  ```bash
  nm target/release/libgoxviet_core.a | grep ime_
  # Should see all expected symbols
  ```

- [ ] **All tests pass**: No regressions
  ```bash
  cargo test --release
  # Should see 415+ tests passing
  ```

- [ ] **No external dependencies**: Check cargo tree
  ```bash
  cargo tree --invert goxviet_core
  # Should show no external crates depending on legacy modules
  ```

- [ ] **Platform integration works**: macOS/Windows clients still work
  - [ ] macOS Swift project builds
  - [ ] Windows C# project builds
  - [ ] Manual smoke test on both platforms

- [ ] **Performance unchanged**: Benchmark results similar
  ```bash
  cargo bench
  # Compare with baseline before legacy removal
  ```

- [ ] **Documentation updated**: All refs to legacy removed
  ```bash
  grep -r "engine::" docs/
  grep -r "processors::" docs/
  # Should return no results (or only deprecated notices)
  ```

---

## Risk Assessment

### Low Risk (Safe to Remove)

| Module | Why Safe |
|--------|----------|
| `engine/` | Fully replaced by `domain/entities/` |
| `engine_v2/` | Logic migrated to adapters |
| `processors/` | Wrapped by adapters, not directly called |
| `validators/` | Wrapped by adapters, not directly called |
| `transformers/` | Wrapped by adapters, not directly called |
| `state/` | Wrapped by adapters, not directly called |

**Mitigation:** Keep deprecated modules for 2-3 releases, provide migration guide.

---

### Medium Risk (Requires Caution)

| Module | Risk | Mitigation |
|--------|------|------------|
| `utils.rs` | May be used by external code | Mark deprecated, provide alternatives in `shared/` |
| `input/` | Input method constants may be referenced | Move constants to `domain/`, mark old location deprecated |

**Mitigation:** Longer grace period (3 releases instead of 2).

---

### Zero Risk (No Removal)

| Module | Why Keep |
|--------|----------|
| `lib.rs` | Public FFI API, must maintain backward compatibility |
| `data/` | Vietnamese language data, migrated but files kept |
| `Cargo.toml` | Project metadata |
| `build.rs` | Build script (if present) |

---

## Rollback Plan

If issues discovered during grace period:

### Scenario 1: Bug in New Architecture

**Problem:** New implementation has incorrect behavior

**Solution:**
1. Fix bug in new architecture
2. Add regression test
3. Continue deprecation timeline

**Impact:** No rollback needed, just fix forward

---

### Scenario 2: External Dependency Found

**Problem:** Third-party crate depends on legacy module

**Solution:**
1. Contact crate author
2. Extend grace period
3. Offer to help with migration
4. Consider keeping legacy code longer

**Impact:** Delay removal until external dependency migrated

---

### Scenario 3: Critical Production Issue

**Problem:** New architecture causes crashes/data loss in production

**Solution:**
1. Immediately revert to legacy via feature flag:
   ```toml
   [features]
   default = ["legacy"]
   ```
2. Release hotfix (e.g., 2.0.1)
3. Investigate and fix issue
4. Resume deprecation after fix confirmed

**Impact:** Deprecation timeline reset, start grace period again

---

## Timeline

### Completed

- ✅ **2026-01-15 to 2026-02-11**: Phases 1-5 (architecture + documentation)

### Upcoming

- 📋 **2026-02-12 (Week 15)**: Phase 6.1 - Add deprecation warnings
- 📋 **2026-02-15**: Release 2.0.0 with deprecation warnings
- 📋 **2026-03-15**: Release 2.1.0 (monitor migration)
- 📋 **2026-04-15**: Release 2.2.0 (final grace release)
- 📋 **2026-05-15**: Release 3.0.0 (legacy code removed)

**Total timeline:** ~3 months from deprecation to removal

---

## Implementation Plan

### Week 15: Deprecation Warnings

```bash
# 1. Create feature branch
git checkout -b chore/deprecate-legacy-code

# 2. Mark modules as deprecated (see Step 1 above)
# Edit each legacy module file

# 3. Update Cargo.toml
# Add [features] section

# 4. Update lib.rs
# Hide legacy modules behind feature flag

# 5. Build and verify
cargo build --all-features
cargo test --all-features

# 6. Commit
git add -A
git commit -m "chore: mark legacy modules as deprecated

- Add #[deprecated] attribute to all legacy modules
- Add migration guide comments
- Hide legacy modules behind 'legacy' feature flag
- Update public exports in lib.rs

See LEGACY_CLEANUP.md for details."

# 7. Push and create PR
git push origin chore/deprecate-legacy-code
gh pr create --title "Mark legacy code as deprecated" \
             --body "See LEGACY_CLEANUP.md for deprecation plan"
```

---

### Week 19: Legacy Removal

```bash
# 1. Create feature branch
git checkout -b chore/remove-legacy-code

# 2. Verify prerequisites (see checklist above)

# 3. Delete legacy directories
rm -rf src/engine/ src/engine_v2/ src/processors/ \
       src/validators/ src/transformers/ src/state/ \
       src/input/ src/utils.rs

# 4. Update lib.rs (remove legacy exports)

# 5. Update Cargo.toml (remove legacy feature)

# 6. Build and test
cargo clean
cargo build --release
cargo test --release

# 7. Update documentation
# Edit CHANGELOG.md, README.md, etc.

# 8. Commit
git add -A
git commit -m "chore: remove legacy code (v3.0.0)

- Delete deprecated modules (engine/, processors/, etc.)
- Remove 'legacy' feature flag
- Update documentation

BREAKING CHANGE: Legacy modules removed. Users must migrate
to clean architecture. See MIGRATION_STRATEGY.md."

# 9. Tag release
git tag -a v3.0.0 -m "Release 3.0.0: Legacy code removed"

# 10. Push
git push origin chore/remove-legacy-code --tags
```

---

## Metrics

### Before Cleanup

- **Total LOC**: ~15,000
- **Legacy LOC**: ~9,400 (63%)
- **Clean LOC**: ~5,600 (37%)
- **Test count**: 415
- **Modules**: 50+

### After Cleanup (Projected)

- **Total LOC**: ~6,000 (-60%)
- **Legacy LOC**: 0 (0%)
- **Clean LOC**: ~6,000 (100%)
- **Test count**: 415+ (same or more)
- **Modules**: ~30 (clean architecture only)

**Benefits:**
- 60% smaller codebase
- 100% clean architecture
- Easier to maintain
- Faster builds
- Lower complexity

---

## References

- [Migration Strategy](./MIGRATION_STRATEGY.md)
- [Architecture Documentation](./ARCHITECTURE.md)
- [SOLID Refactoring Progress](./SOLID_REFACTORING_PROGRESS.md)

---

## Support

For questions about legacy cleanup:
- GitHub Issues: https://github.com/goxviet/goxviet/issues
- Discussions: https://github.com/goxviet/goxviet/discussions

---

**Last Updated:** 2026-02-11  
**Version:** 1.0.0  
**Status:** Cleanup plan documented, deprecation warnings pending implementation
