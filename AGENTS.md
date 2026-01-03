# AGENTS.md - Gõ Việt (GoxViet) Vietnamese IME

> A comprehensive guide for AI coding agents to work effectively on this project.

## Project Overview

**Gõ Việt (GoxViet)** is a cross-platform Vietnamese Input Method Editor (IME) with:
- **Core Engine:** Rust library for high-performance text processing
- **Platforms:** macOS (CGEvent/Accessibility API) and Windows (TSF/WinUI 3)
- **Goal:** < 1ms latency, memory safety, native typing experience

### Brand Names
- **Brand:** "Gõ Việt"
- **Display/App:** "GoxViet"  
- **Code/Repo:** "goxviet"
- **Bundle ID:** `com.goxviet.ime`
- **Log Path:** `~/Library/Logs/GoxViet/`

---

## Project Structure

```
goxviet/
├── core/                     # Rust library (IME engine)
│   ├── src/
│   │   ├── engine/          # Core processing logic (highly modular, see below)
│   │   ├── input/           # Input method handlers (Telex, VNI)
│   │   ├── data/            # Vietnamese data structures (keys, vowels, etc.)
│   │   └── lib.rs
│   ├── tests/               # Unit & integration tests
│   └── Cargo.toml
├── platforms/               # Platform-specific implementations
│   ├── macos/
│   │   └── goxviet/         # Xcode project (CGEvent & Accessibility API)
│   └── windows/
│       └── goxviet/         # Visual Studio solution (TSF)
├── docs/                    # ALL DOCUMENTATION (UPPERCASE names)
├── example-project/         # REFERENCE ONLY - DO NOT MODIFY
│   └── gonhanh.org-main/   # Example implementation for learning
├── .github/
│   ├── copilot-instructions.md
│   └── instructions/        # Detailed guidelines (00-11 numbered files)
└── AGENTS.md                # This file
```

### `core/src/engine/` - Modularized Vietnamese IME Core

- **types.rs**: Core types (`Action`, `Result`, `Transform`)
- **config.rs**: Engine configuration, input method enum
- **buffer.rs**: Typing buffer, character struct
- **history.rs**: Word history ring buffer (backspace-after-space)
- **raw_input_buffer.rs**: Raw keystroke history for ESC restore
- **rebuild.rs**: Buffer rebuild utilities for output generation
- **syllable.rs**: Vietnamese syllable parser
- **tone_positioning.rs**: Diacritic placement rules (old/new style)
- **transform.rs**: Vietnamese transformation logic (Telex/VNI)
- **validation.rs**: Vietnamese spelling validation, foreign pattern detection
- **shortcut.rs**: User-defined text shortcuts
- **vowel_compound.rs**: Utilities for uo/ươ compound handling (modularized, NEW)
- **restore.rs**: Raw ASCII restoration (auto-restore English, ESC restore, NEW)
- **english_detection.rs**: Multi-layer English word detection (pattern, cluster, tech terms)
- **edge_cases_tests.rs**: Edge-case and regression tests
- **mod.rs**: Main Engine struct and orchestration (delegates to modules above)

> **Ghi chú:**  
> - Các chức năng lớn (vowel compound, restore, english detection) đã được tách module riêng để tăng khả năng bảo trì, mở rộng và kiểm thử.
> - Mọi logic xử lý buffer, syllable, transform, validation đều tuân thủ nguyên tắc: KHÔNG sửa trực tiếp chuỗi hiển thị, luôn rebuild từ buffer.
> - Không còn file rỗng/thừa trong core/src.

---

## Setup & Dev Commands

### Rust Core
```bash
cd core
cargo build --release        # Build optimized library
cargo test                   # Run all tests
cargo test -- --nocapture    # Run tests with output
cargo fmt                    # Format code
cargo clippy                 # Lint code
cargo bench                  # Run benchmarks
cargo miri test              # Memory safety checks (requires miri)
```

### macOS Platform
```bash
cd platforms/macos/goxviet
xcodebuild -scheme goxviet -configuration Release build
```

### Performance Testing
```bash
./test-performance.sh        # Run benchmark script
```

---

## Code Style & Conventions

### General Rules
- **Comments:** Code comments in English
- **Explanations:** Technical explanations in Vietnamese (for Vietnamese developers)
- **Documentation:** All docs go in `docs/` with UPPERCASE filenames
- **Tests:** Write test cases BEFORE fixing bugs

### Rust Core
- Use `Result<T, E>` for all fallible functions
- **NO PANIC** in FFI layer - use `std::panic::catch_unwind`
- Prefer `Vec<char>` over `String` for buffer manipulation
- Use table-driven tests for Vietnamese transformations

### FFI Pattern
```rust
#[no_mangle]
pub extern "C" fn ime_process_key(engine_ptr: *mut Engine, key_code: u32) -> *mut c_char {
    if engine_ptr.is_null() { return std::ptr::null_mut(); }
    let engine = unsafe { &mut *engine_ptr };
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        engine.process(key_code)
    }));
    match result {
        Ok(s) => CString::new(s).unwrap_or_default().into_raw(),
        Err(_) => std::ptr::null_mut()
    }
}
```

### Swift/macOS
- Use bridging header for Rust FFI
- Main thread for UI only
- Log path: `~/Library/Logs/GoxViet/keyboard.log`

---

## Vietnamese IME Logic

### Input Methods

#### TELEX Mode
| Input | Result | Notes |
|-------|--------|-------|
| `s`, `f`, `r`, `x`, `j` | Tone marks: Sắc, Huyền, Hỏi, Ngã, Nặng | Replace existing tone |
| `z` | Remove tone | Reset to no-tone |
| `aa`, `ee`, `oo` | â, ê, ô | Circumflex |
| `aw`, `ow` | ă, ơ | Breve, horn |
| `w` | Smart transform | `uw`→ư, `ow`→ơ, `uow`→ươ |
| `dd` | đ | Stroke |

#### VNI Mode
| Input | Result |
|-------|--------|
| `1-5` | Tone marks (Sắc to Nặng) |
| `0` | Remove tone |
| `6` | Circumflex (â, ê, ô) |
| `7` | Horn (ư, ơ) |
| `8` | Breve (ă) |
| `9` | Đ |

### Tone Placement Rules

**Priority 1:** If vowel cluster contains â/ê/ô/ơ/ư → place tone on that vowel
```
viết → tone on ê
quốc → tone on ô
```

**Priority 2:** Otherwise → place tone on second vowel
```
hoà → tone on a
tuý → tone on y
```

### Core Data Structure
```rust
pub enum Tone { None, Sac, Huyen, Hoi, Nga, Nang }

pub struct Syllable {
    pub initial: String,   // Initial consonant (k, qu, gh...)
    pub vowel: String,     // Vowel cluster (a, uô, ươ...)
    pub final_c: String,   // Final consonant (n, ng, ch...)
    pub tone: Tone,
}
```

### Edge Cases (MUST Handle)
1. **Smart "ươ":** `u` + `o` + `w` → `ươ` (not `uơ`)
2. **Undo Logic:** `trươ` + Backspace → `truo` (restore pre-transform state)
3. **Non-Vietnamese:** Keep original if invalid pattern (e.g., `w` at word start)

---

## Backspace & Buffer Rules

### Golden Rules
1. **Backspace deletes by grapheme** (displayed character), not by token or Unicode scalar
2. **Never patch rendered string** - always rebuild
3. **Empty word = full reset** (clean all buffers and state)

### Required Buffers
```
TelexTokenBuffer   // Original input tokens
GraphemeBuffer     // Displayed characters + token mapping  
PreeditString      // Rendered output
```

### Backspace Algorithm
```pseudo
function handleBackspace():
    if graphemeBuffer.isEmpty():
        resetAll()
        passBackspaceToOS()
        return
    
    graphemeBuffer.popLast()
    
    if graphemeBuffer.isEmpty():
        resetAll()
        updatePreedit("")
        return
    
    removeAssociatedTokens()
    rebuildFromTokens()
```

### Reset Triggers
- Commit: Space, Enter, Tab, Punctuation
- Cursor move / mouse click
- Invalid syllable detected
- Backspace empties word

---

## Performance Targets

| Metric | Target | Achieved |
|--------|--------|----------|
| Keystroke latency | < 16ms | ~7ms |
| Backspace latency | < 3ms | ~1.2ms |
| Memory usage | < 50MB | ~28MB |
| Batch backspace (10x) | < 30ms | ~10ms |

### Optimization Strategies
1. **Smart Backspace:** O(1) for regular chars, O(s) for syllable rebuild
2. **Zero-allocation hot path:** No heap allocation in `process_key`
3. **App-specific injection:**
   - Modern editors (VSCode, Zed): `.instant` - zero delays
   - Terminals: `.slow` - small delays for stability
   - Browsers: `.selection` - for address bars

### Cargo Release Profile
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = 'abort'
strip = true
```

---

## Testing Requirements

### Test Pyramid
- **70% Unit Tests (Rust):** Vietnamese transformation logic
- **20% Integration Tests (FFI):** Memory allocation/deallocation
- **10% E2E Tests (Native):** UI and cross-app behavior

### Table-Driven Tests (Required)
```rust
#[test]
fn test_telex_transformation() {
    let cases = vec![
        ("duongwf", "đường"),
        ("toans", "toán"),
        ("truowngf", "trường"),
        ("dd", "đ"),
        ("ddd", "dd"), // Escape
    ];
    
    let mut engine = Engine::new();
    for (input, expected) in cases {
        engine.reset();
        let output = engine.process_sequence(input);
        assert_eq!(output, expected, "Failed at input: {}", input);
    }
}
```

### Property-Based Testing (No Panic)
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn doesnt_crash_on_random_input(s in "\\PC*") {
        let mut engine = Engine::new();
        for c in s.chars() {
            let _ = engine.process_key(c);
        }
    }
}
```

### FFI Memory Test
```rust
#[test]
fn test_ffi_memory_cycle() {
    unsafe {
        let engine = ime_create();
        let input = CString::new("a").unwrap();
        let result_ptr = ime_process(engine, input.as_ptr());
        assert!(!result_ptr.is_null());
        ime_free_string(result_ptr);  // MUST free
        ime_destroy(engine);
    }
}
```

---

## Git Workflow

### Branch Naming
```
<type>/<short-description>
feat/telex-tone
fix/windows-tsf-crash
chore/ci-cache
docs/ime-guide
```

### Commit Messages (Conventional Commits)
```
<type>(scope): <subject>

feat(core): add smart uow transformation
fix(macos): handle null text_ptr in bridge
perf(core): reduce allocs in process_key
docs(instructions): add interop strategy
```

### Types
`feat`, `fix`, `chore`, `refactor`, `perf`, `docs`, `test`, `build`, `ci`

### PR Checklist
- [ ] Run `cargo test` and platform tests
- [ ] Run `cargo fmt` and `cargo clippy`
- [ ] Update docs if changing public API/FFI
- [ ] Describe before/after behavior
- [ ] Note performance impact if applicable

### Merge Strategy
- Squash & merge (single commit per PR)
- Rebase with `main` before merge
- Use `--force-with-lease` for force push

---

## Critical Rules

### ❌ NEVER DO
1. **Do NOT use names from example project:**
   - ❌ "GoNhanh", "gonhanh", "go-nhanh"
   - ✅ Use "GoxViet", "goxviet", "Gõ Việt"

2. **Do NOT modify example project:**
   - ❌ Edit files in `example-project/gonhanh.org-main/`
   - ✅ Only read and reference

3. **Do NOT create docs outside structure:**
   - ❌ Documentation at project root
   - ✅ All docs in `docs/` with UPPERCASE names

4. **Do NOT panic in FFI layer:**
   - ❌ `unwrap()` without handling
   - ✅ `Result<T, E>` + `catch_unwind`

### ✅ ALWAYS DO
1. Use correct brand names and paths
2. Write tests before fixing bugs
3. Follow monorepo structure
4. Add credit: `// Based on reference implementation`
5. Keep latency < 16ms for any keystroke

---

## Documentation Reference

### Quick Start
- `docs/getting-started/QUICK_START.md`
- `docs/GETTING_STARTED.md`

### Performance
- `docs/PERFORMANCE.md` - Main performance doc
- `docs/performance/` - Detailed guides and benchmarks

### Fixes & Troubleshooting  
- `docs/fixes/` - Bug fixes by category
- `docs/TROUBLESHOOTING_VIESET_BUG.md`

### Architecture
- `.github/instructions/01_architecture.md`
- `.github/instructions/02_rust_core.md`
- `.github/instructions/03_macos_swift.md`
- `.github/instructions/04_windows_winui.md`

### Vietnamese Logic
- `.github/instructions/06_vietnamese_logic.md`
- `.github/instructions/09_vietnamese-language-system.md`
- `.github/instructions/10_vietnamese_backspace_and_buffer_reset.md`
- `.github/instructions/11_vietnamese_telext_tone_repositioning.md`

---

## Memory Safety Contract (FFI)

1. **Ownership:** Allocator must deallocate
   - Rust allocates → Rust provides `ime_free_string`
   - Native calls `ime_create` → Native calls `ime_destroy`

2. **Lifetimes:** Never return references across FFI
   - Always copy data or transfer ownership (`into_raw`)

3. **Thread Safety:** `ImeEngine` is NOT thread-safe
   - Call from single thread only (Main/Input thread)

4. **Pointer Validity:** 
   - Return from `ime_process_key` valid until next call
   - Native must copy immediately if storing

---

## Platform-Specific Notes

### macOS
- **Framework:** CGEvent + Accessibility API (not IMKit)
- **Permission:** Requires Accessibility access
- **Injection Methods:** `.instant`, `.slow`, `.selection` based on app
- **Build:** Static library (`libcore.a`)

### Windows  
- **Framework:** TSF (Text Services Framework)
- **Architecture:** Two-process model (DLL + WinUI 3 EXE)
- **IPC:** Named Pipes for DLL ↔ UI communication
- **Encoding:** UTF-16 for Windows API

---

## Troubleshooting

### Typing Lag
1. Verify release build: `cargo build --release`
2. Check for competing input methods
3. Review logs: `~/Library/Logs/GoxViet/keyboard.log`

### Backspace Issues
1. Verify syllable boundary detection
2. Ensure not rebuilding entire buffer for single char
3. Run benchmark: `./test-performance.sh`

### Memory Leaks
1. Run Instruments (Leaks, Allocations) on macOS
2. Check Rust allocations in hot path
3. Verify no retain cycles in Swift/FFI

### Browser Address Bar
1. Use `.selection` injection method
2. Check skip-placeholder logic

---

## Quick Reference

```bash
# Build & Test
cd core && cargo build --release && cargo test

# Format & Lint  
cargo fmt && cargo clippy

# Benchmark
./test-performance.sh

# View Logs (macOS)
tail -f ~/Library/Logs/GoxViet/keyboard.log
```

---

*Last updated: 2025*
*For detailed instructions, see `.github/instructions/` directory*
