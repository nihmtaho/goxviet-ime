# GoxViet Project Structure

This document describes the organization of the GoxViet codebase. It serves as a guide for contributors to understand where files belong and how the project is structured.

## Overview

GoxViet is a high-performance Vietnamese input method editor (IME) with a hybrid architecture:
- **Core Engine (Rust)**: Handles linguistic logic, state management, and text transformation
- **Platform Layers (Native)**: OS-specific implementations (macOS, Windows)

## Directory Structure

```
goxviet/
├── .agent/                    # AI agent skills and automation (consolidated)
├── .github/                   # GitHub configuration (workflows, templates)
├── .planning/                 # Project roadmaps and planning documents
├── .vscode/                   # VSCode settings
├── .temp/                     # Temporary working files (gitignored)
├── core/                      # Rust core engine
├── docs/                      # Public/user-facing documentation
├── .docs/                     # Internal/developer documentation
├── platforms/                 # Platform-specific implementations
│   ├── macos/                 # macOS app (Swift/SwiftUI)
│   └── windows/               # Windows app (placeholder)
├── scripts/                   # Build scripts and utilities
├── CHANGELOG.md               # Release history
├── LICENSE                    # License file
├── README.md                  # Main project readme
└── VERSION                    # Current version
```

## Directory Details

### `.agent/`
**Purpose:** AI agent skills and automation workflows

Contains 20+ skill modules for AI assistants:
- `skills/skill-git/` - Git workflow automation
- `skills/macos-development/` - macOS development guides
- `skills/pr-creator/` - Pull request creation
- `skills/issue-creator/` - Issue management
- `skills/planning/` - Project planning tools
- And more...

**Note:** Previously scattered across `.agent/`, `.claude/`, and `.temp/`. Now consolidated here.

---

### `.github/`
**Purpose:** GitHub-specific configuration

- `workflows/` - GitHub Actions CI/CD workflows
- `instructions/` - Agent instruction templates
- `ISSUE_TEMPLATE/` - Issue templates
- `prompts/` - AI prompt templates
- `commands/` - Command configurations

---

### `.planning/`
**Purpose:** Project planning and roadmaps

Contains:
- Roadmap JSON files
- Planning documents
- `.gitignore` to exclude temporary planning files

---

### `core/`
**Purpose:** Rust core engine - the heart of GoxViet

**Key files:**
- `Cargo.toml` - Rust package configuration
- `src/lib.rs` - Main library entry point
- `src/engine/` - Legacy engine implementation
- `src/engine_v2/` - Modern engine (v2) with FSM validation
- `src/data/` - Static data (constants, vowel mappings)
- `src/input/` - Input method implementations (Telex, VNI)
- `tests/` - Integration tests
- `benches/` - Performance benchmarks

**Note:** `core/target/` is gitignored and should never be committed (contains build artifacts).

---

### `docs/`
**Purpose:** Public/user-facing documentation

- `release-note/` - Version release notes (1.0.1 to 2.0.8)
- `tasks/` - Completed task documentation
- `implementation_plans/` - Feature implementation plans
- `reviews/` - Code review and workflow documents
- Root markdown files - Getting started guides, architecture docs

---

### `.docs/`
**Purpose:** Internal/developer documentation

- `features/core-engine/` - Engine architecture documentation
- `features/platform/macos/` - macOS platform guides
- `guides/` - Feature-specific guides (output encoding, text expansion)
- `templates/` - Pull request templates

**Note:** Hidden directory (dot-prefixed) for internal docs distinct from public `docs/`.

---

### `platforms/`
**Purpose:** Platform-specific application implementations

**macos/** (Active Development)
- `goxviet/` - Main macOS application
  - Xcode project files
  - Swift source files (60+ files)
  - UI components, settings, input management
  - `libgoxviet_core.a` - Compiled Rust library (gitignored)
- Contains GUI, event handling, OS integration

**windows/** (Placeholder)
- Currently minimal - Windows implementation planned

**Note:** Platform-specific build artifacts (`.app`, `.dmg`) are gitignored.

---

### `scripts/`
**Purpose:** Build scripts and development utilities

- `build-release.sh` - Release build script
- `rust_build_lib_universal_for_macos.sh` - Universal binary build
- `release.sh` - Full release workflow
- `notarize.sh` - macOS notarization
- `create-dmg.sh` - DMG creation
- `generate_*.py` - Dictionary generation scripts
- `manage_dict.py` - Dictionary management
- `dict_config/` - Dictionary configuration files

---

### `homebrew/`
**Purpose:** Homebrew formula for distribution

- `goxviet.rb` - Homebrew formula
- `README.md` - Installation instructions

---

### Root Files

- `README.md` - Project overview and quick start
- `CHANGELOG.md` - Detailed release history
- `LICENSE` - Project license
- `VERSION` - Current version number
- `AGENTS.md` → `.rules` (symlink to agent rules)
- `CLAUDE.md` → `.rules` (symlink)
- `GEMINI.md` → `.rules` (symlink)

## File Placement Guidelines

### Where to add new code:

**Core engine changes:**
→ `core/src/` (Rust files)

**macOS UI changes:**
→ `platforms/macos/goxviet/goxviet/` (Swift files)

**Documentation (user-facing):**
→ `docs/` (public) or `.docs/` (internal)

**Build/utility scripts:**
→ `scripts/`

**Agent skills:**
→ `.agent/skills/[skill-name]/SKILL.md`

**CI/CD configuration:**
→ `.github/workflows/`

## Gitignore Rules

The following are **never** committed:
- `.DS_Store` files (macOS system files)
- `core/target/` (Rust build artifacts)
- `xcuserdata/` (Xcode user data)
- `*.dmg`, `*.app` (Distribution packages)
- `example-project/` (Reference implementations)
- `.temp/` (Temporary files)
- Environment files (`.env`, secrets)

## Recent Cleanup (Issue #53)

This structure reflects cleanup completed in Issue #53:
- ✅ Consolidated agent skills from `.agent/`, `.claude/`, `.temp/` into `.agent/`
- ✅ Removed build artifacts (`core/target/`, `.DS_Store`, `xcuserdata`)
- ✅ Deleted empty directories (`.tasks/`, `bindings/`)
- ✅ Moved scripts to canonical location (`scripts/`)
- ✅ Removed duplicate files

## Questions?

See `.rules` for detailed development guidelines and conventions.
