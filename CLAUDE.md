# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

**Project:** **Gõ Việt (GoxViet)** – A high-performance, cross-platform Vietnamese Input Method Engine (IME).  
**Goals:** Latency < 3ms core / < 16ms end-to-end, zero FFI panics, native macOS/Windows UX.

## Build, Test & Lint

**Rust Core** (`cd core && ...`):
```bash
cargo build --release          # Build
cargo test                     # All tests
cargo test <name>             # Single test by name
cargo test --test trans_test  # Single test file
cargo fmt && cargo clippy     # Lint & format
cargo bench                    # Benchmarks (Criterion)
./scripts/rust_build_lib_universal_for_macos.sh  # macOS arm64 + x86_64 → libgoxviet_core.a
```

**macOS App** (`platforms/macos/goxviet/goxviet.xcodeproj` in Xcode):
```bash
./scripts/build-release.sh <version>      # Build + DMG
./scripts/release.sh <version>            # Full release: build + DMG + notarize + tag
```

## Architecture

**Monorepo Layout:**
- `core/` – Rust engine (crate: `goxviet-core`)
- `platforms/macos/goxviet/` – Swift macOS app (AppKit + SwiftUI, Swift 6.2 with `SWIFT_DEFAULT_ACTOR_ISOLATION = MainActor`)
- `.docs/features/` – Architecture docs (read before major changes)
- `scripts/` – Build, release, dictionary management

**Rust Core (Clean Architecture v3.0.0)** in `core/src/`:
| Layer | Path | Purpose |
|---|---|---|
| **domain** | `domain/` | Entities, value objects, ports |
| **application** | `application/` | Use cases, services |
| **infrastructure** | `infrastructure/` | Telex/VNI engines, validators |
| **presentation** | `presentation/ffi/` | FFI API & DI container |

Supporting: `shared/` (buffer, types), `features/` (shortcuts), `data/` (FSM tables, dicts), `unified_engine.rs` (facade).

**FFI API v2** (only v2, v1 removed):
- Out-parameters, explicit status codes, per-engine config
- Swift uses `RustBridgeSafe` (ONLY place for raw FFI calls)
- Always `defer { ime_free_string_v2(ptr) }` immediately
- No panics across FFI – use `catch_unwind + Result`

**macOS Swift Layout:**
```
FFI/RustBridgeSafe              # ONLY raw FFI calls here
Managers/Input/InputManager     # CGEventTap singleton (HIGH RISK)
Managers/PerAppModeManagerEnhanced  # Smart Mode per-app
Core/AppState                   # Central settings
UI/SettingsRootView             # SwiftUI settings
```

**Processing Pipeline:** `CGEventTap` → `InputManager` → `RustBridgeSafe.processKey()` → backspaces + insert text
- Soft Backspace: undo last transform from token buffer
- Smart Mode: per-app enable/disable in `UserDefaults`
- English Auto-Restore: phonotactic + dictionary analysis restores English sequences

## Conventions

**Branding:** `GoxViet` (app), `Gõ Việt` (Vietnamese), `goxviet` (code), `com.goxviet.ime` (Bundle ID), `~/Library/Logs/GoxViet/` (logs). Never use `.uvasx/` names.

**Commits:** `<type>(<scope>): <subject>` – Types: `feat`, `fix`, `docs`, `refactor`, `perf`, `test`, `chore`. Scopes: `core`, `macos`, `windows`, `ffi`.

**Branches:** `feature/<name>` from develop → PR → squash merge. Never force-push `main`/`develop`. Always rebase before merging.

**Rust Hot Path:** No heap allocs in `process_key` (use `SmallVec`). O(1) validation lookups (FSM tables). Panic-free FFI boundaries.

**Testing:** Integration tests in `core/tests/`, table-driven tests, regression test before each bug fix. Benchmarks use Criterion.

**Dictionary:** Binaries in `core/src/infrastructure/external/data/*.bin`, sources in `.docs/features/core-engine/data/*.txt`.
```bash
./scripts/manage_dict.py add <word>     # Whitelist + rebuild
./scripts/manage_dict.py remove <word>  # Blacklist + rebuild
./scripts/manage_dict.py sync           # Text → binary
```

## Never Commit

`.DS_Store`, `core/target/`, `xcuserdata/`, `*.dmg`, `*.app`, `libgoxviet_core.a`, `.temp/`, secrets.
