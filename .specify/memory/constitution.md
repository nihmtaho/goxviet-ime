<!--
SYNC IMPACT REPORT
==================
Version change: [unfilled template] → 1.0.0
Status: Initial ratification — no prior version to diff.

Modified principles: N/A (first fill)
Added sections: Core Principles (5), Platform & Technology Constraints, Development Workflow, Governance
Removed sections: N/A

Templates reviewed:
  ✅ .specify/templates/plan-template.md — "Constitution Check" section present; gates align with principles below
  ✅ .specify/templates/spec-template.md — scope/requirements structure compatible with all 5 principles
  ✅ .specify/templates/tasks-template.md — task phases compatible; no outdated principle references found
  ✅ .specify/templates/constitution-template.md — source template; no changes required

Follow-up TODOs:
  - No placeholders intentionally deferred; all sections fully populated.
-->

# GoxViet Constitution

## Core Principles

### I. Performance First (NON-NEGOTIABLE)

The engine MUST sustain < 3 ms latency in the Rust core and < 16 ms end-to-end on the keystroke
pipeline. Every function on the hot path (`process_key` and its callees) MUST avoid heap
allocations — use stack arrays or `SmallVec`. All validation lookups MUST be O(1) using FSM tables,
not linear search. Benchmarks (Criterion) MUST be run and reviewed before merging any change that
touches `process_key`, the FSM tables, or the FFI boundary.

**Rationale**: GoxViet is an input method engine that intercepts every keypress. A single slow
keystroke is immediately perceptible to the user and degrades trust in the product. There is no
acceptable trade-off between correctness and latency.

### II. Clean Architecture (Layer Discipline)

The Rust core MUST maintain strict four-layer separation — `domain` → `application` →
`infrastructure` → `presentation/ffi` — with dependencies flowing inward only. No layer may import
from a layer above it. The legacy `engine/` and `engine_v2/` directories no longer exist; all code
lives under the canonical layer paths inside `core/src/`. The Swift layer MUST use only the v2 FFI
API; v1 is removed and MUST NOT be reintroduced. `RustBridgeSafe` is the sole permitted location
for raw FFI calls in Swift; all other Swift code MUST go through that abstraction.

**Rationale**: Clean layering keeps the engine independently testable, enables future Windows and
web targets, and makes onboarding safer by providing clear ownership boundaries.

### III. Regression-First Testing (NON-NEGOTIABLE)

Before fixing any bug, a regression test MUST be written that reproduces the failure and confirmed
to fail. Only then may the fix be implemented (Red → Green). Table-driven tests are preferred over
individual assertions. Integration tests live in `core/tests/`, one file per scenario. Benchmarks
live in `core/benches/` using Criterion. No PR that fixes a bug may land without a corresponding
regression test.

**Rationale**: GoxViet's phonotactic and English-detection logic has many subtle interactions.
Without regression tests, fixes frequently reintroduce previously-resolved bugs under different
input sequences.

### IV. Zero FFI Panics

Every function that crosses the FFI boundary MUST be wrapped in `catch_unwind` + `Result`. Panics
MUST NEVER propagate across FFI — doing so is undefined behaviour in C/Swift contexts and will
crash the host application. On the Swift side, every pointer received from Rust MUST be freed
immediately using `defer { ime_free_string_v2(ptr) }`. UI updates MUST run on `MainActor`; engine
calls are synchronous and MUST NOT be dispatched to an actor.

**Rationale**: A crash in the input method kills the foreground application. Zero-panic FFI is a
hard safety requirement, not a code quality preference.

### V. Branding Consistency

All project artifacts MUST use the canonical brand names and identifiers:

| Context | Value |
|---|---|
| Display / App name | `GoxViet` |
| Vietnamese brand | `Gõ Việt` |
| Repo / code identifiers | `goxviet` |
| Rust crate name | `goxviet-core` |
| macOS Bundle ID | `com.goxviet.ime` |
| Log path | `~/Library/Logs/GoxViet/` |

Names, identifiers, or bundle IDs from the reference implementation in `.uvasx/` MUST NEVER appear
in committed code, build scripts, or release artifacts.

**Rationale**: Brand pollution from the reference implementation has caused confusion in past
releases and creates legal/IP risk.

## Platform & Technology Constraints

- **Rust core** (`core/`): must compile with `cargo build --release`; universal macOS static
  library (`arm64 + x86_64`) produced by `scripts/rust_build_lib_universal_for_macos.sh`.
- **macOS platform** (`platforms/macos/`): Swift + AppKit/SwiftUI; minimum deployment target is
  macOS 11 (Big Sur); Swift 6 concurrency model with `SWIFT_DEFAULT_ACTOR_ISOLATION = MainActor`.
- **Windows platform** (`platforms/windows/`): C# (planned, not yet implemented).
- **Never commit**: `.DS_Store`, `core/target/`, `xcuserdata/`, `*.dmg`, `*.app`,
  `libgoxviet_core.a`, `.temp/`, secrets or API keys.
- **English dictionary** management MUST use `./scripts/manage_dict.py` — direct edits to `.bin`
  files are forbidden; text sources in `.docs/features/core-engine/data/*.txt` are authoritative.

## Development Workflow

### Branch Strategy

| Branch type | Base | Merge target | Strategy |
|---|---|---|---|
| `feature/<name>` | `develop` | `develop` (PR) | squash merge, delete branch |
| `bugfix/<name>` | `develop` | `develop` (PR) | squash merge, delete branch |
| `hotfix/<name>` | `main` | `main` + `develop` | merge commit |
| `release/<version>` | `develop` | `main` + `develop` | merge commit |

Force-pushing to `main`, `develop`, or any production branch is **forbidden**. All branches MUST be
rebased onto their target before merging.

### Commit Messages

Commits MUST follow Conventional Commits:

```
<type>(<scope>): <subject>
```

Valid types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`, `ci`
Valid scopes: `core`, `macos`, `windows`, `ffi`

### Release Process

Releases are created via `./scripts/release.sh <version>` which builds, packages a DMG, notarizes,
and creates a git tag. Version numbers follow semantic versioning. Release notes MUST be placed in
`.release-notes/release_note_<version>.md` before tagging.

## Governance

This constitution supersedes all other project practices and conventions. When CLAUDE.md and this
constitution conflict, this constitution takes precedence for principle-level decisions; CLAUDE.md
governs runtime tooling specifics.

**Amendment procedure**:
1. Propose the change as a PR against `.specify/memory/constitution.md`.
2. Increment `CONSTITUTION_VERSION` according to semantic versioning (MAJOR = backward-incompatible
   governance change; MINOR = new principle/section added; PATCH = clarification or wording fix).
3. Run the `speckit-constitution` skill to validate consistency across all `.specify/templates/`.
4. Amendments take effect on merge to `develop`.

**Compliance**: All PR reviewers MUST verify that the implementation satisfies the principles in
this constitution before approving. The "Constitution Check" gate in `plan.md` MUST be signed off
before Phase 0 research begins and re-checked after Phase 1 design.

**Runtime guidance**: See `CLAUDE.md` for build commands, directory layout, and tooling specifics.

---

**Version**: 1.0.0 | **Ratified**: 2026-04-04 | **Last Amended**: 2026-04-04
