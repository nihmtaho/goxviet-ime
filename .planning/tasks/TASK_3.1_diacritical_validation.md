# TASK 3.1: Remove Diacritical Marks After Final Consonants

**Author**: GoxViet Planning  
**Created**: 2026-02-01  
**Phase**: Phase 3 (Quality & Testing)  
**Status**: Planned  
**Estimated Duration**: 4-6 hours

---

## Overview

This task improves input validation to **prevent invalid diacritical mark placement** after Vietnamese final consonants. When a user tries to apply diacritical marks (dấu mũ, dấu móc, dấu trăng) via Telex or VNI after a word already ends with a consonant, the engine should intelligently reject the input instead of applying incorrect marks.

**Problem Example**:
- User types: `s` `o` `n` `g` (from "song" = trí tuệ)
- Then presses Telex `aa` (trying to make â)
- **CURRENT (BUG)**: Might incorrectly transform to "songâ" or corrupt state
- **DESIRED**: Reject the `aa` input gracefully, output remains "song"

---

## Goal

Implement strict validation that prevents diacritical marks from being applied after Vietnamese final consonants, in both Telex and VNI input methods.

---

## Acceptance Criteria

### Functional Requirements
- [ ] System rejects Telex `aa` after any Vietnamese final consonant (-c, -ch, -m, -n, -ng, -nh, -p, -t)
- [ ] System rejects Telex `aw`, `ee`, `oo`, `ow`, `uw` after any final consonant
- [ ] System rejects Telex `dd` (đ transformation) after any final consonant except `d`
- [ ] System rejects VNI `6`, `7`, `8` after any final consonant
- [ ] System gracefully handles rejected input (no crash, no buffer corruption)
- [ ] Valid diacritical placements still work correctly (before consonant, no consonant present)
- [ ] All existing Telex/VNI functionality remains unchanged

### Non-Functional Requirements
- [ ] Performance: tone removal validation < 0.5ms per keystroke (total keystroke <1ms)
- [ ] No memory leaks or buffer overflows
- [ ] No panics across FFI boundary
- [ ] Test coverage: ≥95% of validation logic

### Platform Requirements
- [ ] macOS InputManager respects validation (no ghost characters)
- [ ] Windows TSF respects validation (no ghost characters)
- [ ] Smart Mode doesn't interfere with validation
- [ ] Per-App settings don't bypass validation

---

## Files to Change

### New Files
- **`core/src/engine_v2/diacritical_validator.rs`** (NEW)
  - Core validation logic
  - Phonotactic constraint checking

### Modified Files
- **`core/src/engine_v2/syllable_parser.rs`**
  - Integrate new validator into parsing pipeline
  
- **`core/src/engine_v2/lib.rs`**
  - Export validator module
  
- **`core/tests/diacritical_validation_tests.rs`** (NEW)
  - 50+ unit tests
  - Integration tests with buffer

### Platform Files (Minimal changes expected)
- **`platforms/macos/GoxViet/InputManager.swift`** (READ ONLY)
  - Verify it respects rejection from engine
  
- **`platforms/windows/GoxVietIME/InputManager.cpp`** (READ ONLY)
  - Verify it respects rejection from engine

---

## High-Level Design

### Architecture Overview

```
User Input (Keystroke)
    ↓
InputManager (Platform Layer)
    ↓
ime_process_key() [FFI]
    ↓
Engine::process_key()
    ├─ Parse keystroke
    ├─ Current Syllable State: {initial, vowel, final_c, tone}
    ├─ Check if diacritical key
    │  └─ YES: Call DiacriticalValidator::is_valid_placement()
    │        ├─ Has final_c? → REJECT (return no change)
    │        └─ No final_c? → APPLY
    │  
    └─ Return Result {action, chars, backspace_count}
    ↓
InputManager applies result
    ↓
Screen output
```

### Validator Logic

```rust
// core/src/engine_v2/diacritical_validator.rs

pub struct DiacriticalValidator;

impl DiacriticalValidator {
    /// Check if applying a diacritical mark is valid given current syllable state
    pub fn is_valid_placement(
        vowel: &str,
        final_consonant: Option<&str>,
        diacritical: DiacriticalType,
    ) -> bool {
        // Rule 1: No diacritical after any final consonant
        if final_consonant.is_some() {
            return false;
        }
        
        // Rule 2: Specific vowel constraints for each diacritical
        match diacritical {
            DiacriticalType::Circumflex => {
                // ^ (mũ) only applies to: a, e, o
                matches!(vowel, "a" | "e" | "o")
            }
            DiacriticalType::Breve => {
                // ˘ (trăng) only applies to: a
                vowel == "a"
            }
            DiacriticalType::Horn => {
                // ʼ (móc) only applies to: o, u
                matches!(vowel, "o" | "u")
            }
            DiacriticalType::Stroke => {
                // Đ (gạch) only applies to: d
                // (handled separately in keystroke logic)
                true
            }
        }
    }
}
```

### Data Flow Example

**Scenario 1: Valid case (no consonant)**
```
Input: h, o, a, a
State after 'a': {initial: "h", vowel: "oa", final_c: None, tone: Ngang}
Validator: final_c is None → VALID
Output: "hoà" ✓
```

**Scenario 2: Invalid case (consonant present)**
```
Input: s, o, n, g, a, a
State after 'g': {initial: "s", vowel: "o", final_c: "ng", tone: Ngang}
Validator: final_c is Some("ng") → INVALID
Output: "song" (rejected `aa`) ✓
```

---

## Implementation Plan

### Step 1: Create DiacriticalValidator Module (1 hour)

**File**: `core/src/engine_v2/diacritical_validator.rs`

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiacriticalType {
    Circumflex, // ^ - for a, e, o → â, ê, ô
    Breve,      // ˘ - for a → ă
    Horn,       // ʼ - for o, u → ơ, ư
    Stroke,     // - - for d → đ
}

pub struct DiacriticalValidator;

impl DiacriticalValidator {
    pub fn is_valid_placement(
        vowel: &str,
        final_consonant: Option<&str>,
        diacritical: DiacriticalType,
    ) -> bool {
        // Implementation as described above
    }
    
    pub fn from_telex_input(ch: char, prev_ch: Option<char>) -> Option<DiacriticalType> {
        // Map Telex keys to diacritical types
        match ch {
            'a' if prev_ch == Some('a') => Some(DiacriticalType::Circumflex),
            'w' if prev_ch == Some('a') => Some(DiacriticalType::Breve),
            'e' if prev_ch == Some('e') => Some(DiacriticalType::Circumflex),
            'o' if prev_ch == Some('o') => Some(DiacriticalType::Circumflex),
            'w' if prev_ch == Some('o') => Some(DiacriticalType::Horn),
            'w' if prev_ch == Some('u') => Some(DiacriticalType::Horn),
            'd' if prev_ch == Some('d') => Some(DiacriticalType::Stroke),
            _ => None,
        }
    }
    
    pub fn from_vni_input(ch: char) -> Option<DiacriticalType> {
        // Map VNI keys to diacritical types
        match ch {
            '6' => Some(DiacriticalType::Circumflex),
            '7' => Some(DiacriticalType::Horn),
            '8' => Some(DiacriticalType::Breve),
            '9' => Some(DiacriticalType::Stroke),
            _ => None,
        }
    }
}
```

### Step 2: Integrate with Syllable Parser (1 hour)

**File**: `core/src/engine_v2/syllable_parser.rs`

Modify the `process_keystroke()` function to use validator:

```rust
pub fn process_keystroke(&mut self, ch: char) -> Result<ProcessResult> {
    // ... existing parsing code ...
    
    // Check if this is a diacritical input
    if let Some(diacritical) = DiacriticalValidator::from_telex_input(ch, self.last_char) {
        let current_state = self.get_current_state();
        
        // VALIDATE before applying
        if !DiacriticalValidator::is_valid_placement(
            &current_state.vowel,
            current_state.final_consonant.as_deref(),
            diacritical,
        ) {
            // REJECT: return no change
            return Ok(ProcessResult::no_change());
        }
        
        // ACCEPT: apply the diacritical
        // ... existing diacritical application code ...
    }
    
    // ... rest of processing ...
}
```

### Step 3: Create Comprehensive Test Suite (2-3 hours)

**File**: `core/tests/diacritical_validation_tests.rs`

**Test Categories**:

#### Category A: Telex `aa` (circumflex on 'a') - Invalid cases
```rust
#[test]
fn test_telex_aa_after_c() {
    // san + aa should reject
    assert_eq!(process("sanaaa"), "san");
}

#[test]
fn test_telex_aa_after_ch() {
    // sach + aa should reject
    assert_eq!(process("sachaaa"), "sach");
}

#[test]
fn test_telex_aa_after_ng() {
    // sang + aa should reject
    assert_eq!(process("sangaaa"), "sang");
}

// ... more tests for m, n, nh, p, t ...
```

#### Category B: Telex `aa` - Valid cases
```rust
#[test]
fn test_telex_aa_no_consonant() {
    // ha + aa should accept → hoà
    assert_eq!(process("haaa"), "hoà");
}

#[test]
fn test_telex_aa_after_vowel_before_consonant() {
    // NOTE: This tests "hoa" + more input
    // hoaa should apply to vowel, result in "hoà"
    assert_eq!(process("hoaa"), "hoà");
}
```

#### Category C: Telex `aw` (breve) - Invalid cases
```rust
#[test]
fn test_telex_aw_after_final_consonant() {
    // cap + aw should reject
    assert_eq!(process("capaw"), "cap");
}
```

#### Category D: VNI equivalents
```rust
#[test]
fn test_vni_6_after_consonant() {
    // san6 should reject (n is final consonant)
    assert_eq!(process_vni("san6"), "san");
}

#[test]
fn test_vni_6_no_consonant() {
    // ha6 should accept → hoà
    assert_eq!(process_vni("ha6"), "hoà");
}
```

#### Category E: Edge cases
```rust
#[test]
fn test_empty_buffer_diacritical() {
    // aa with empty buffer should reject gracefully
    assert_eq!(process("aa"), "");
}

#[test]
fn test_double_diacritical() {
    // aa + aa should reject second aa
    assert_eq!(process("aaaa"), "hoà" /* first hoà, second rejects */);
}

#[test]
fn test_diacritical_then_consonant() {
    // hoàng after validation
    assert_eq!(process("haaang"), "hoàng" /* hoà then ng */);
}
```

**Total Test Cases**: 50+

### Step 4: Integration Testing (1 hour)

**File**: `core/tests/buffer_validation_integration.rs`

```rust
#[test]
fn test_buffer_consistency_after_rejected_diacritical() {
    // Buffer state should remain valid after rejection
    let mut buffer = RawBuffer::new();
    buffer.process("sang");
    buffer.process("aa"); // reject
    
    // Verify buffer still valid
    assert_eq!(buffer.len(), 3);
    assert_eq!(buffer.to_string(), "sang");
    
    // Should be able to continue typing
    buffer.process("c");
    assert_eq!(buffer.to_string(), "sangc");
}

#[test]
fn test_undo_after_rejected_diacritical() {
    let mut buffer = RawBuffer::new();
    buffer.process("san");
    buffer.process("aa"); // rejected
    buffer.backspace();   // remove 'n'
    
    assert_eq!(buffer.to_string(), "sa");
}
```

### Step 5: Platform Integration Testing (30 mins)

**File**: `platforms/macos/GoxVietTests/DiacriticalValidationTests.swift`

```swift
func testInputManagerRespectsRejection() {
    // When validator rejects, InputManager should not insert ghost chars
    let manager = InputManager()
    
    // Type "sang"
    manager.handleKey(code: kVK_ANSI_S, ...)
    manager.handleKey(code: kVK_ANSI_A, ...)
    manager.handleKey(code: kVK_ANSI_N, ...)
    manager.handleKey(code: kVK_ANSI_G, ...)
    
    // Try to apply diacritical (should be rejected by engine)
    manager.handleKey(code: kVK_ANSI_A, ...)
    manager.handleKey(code: kVK_ANSI_A, ...)
    
    // Verify output is still "sang"
    XCTAssertEqual(lastOutput, "sang")
}
```

---

## Test Coverage Matrix

| Input Method | Final Consonant | Diacritical | Valid | Test Count |
|--------------|-----------------|-------------|-------|-----------|
| Telex        | c, ch, m, n, ng, nh, p, t | aa, aw, ee, oo, ow, uw | ✗ | 8 × 6 = 48 |
| Telex        | (none) | aa, aw, ee, oo, ow, uw | ✓ | 6 |
| VNI          | c, ch, m, n, ng, nh, p, t | 6, 7, 8 | ✗ | 8 × 3 = 24 |
| VNI          | (none) | 6, 7, 8 | ✓ | 3 |
| Edge Cases   | - | empty buffer, double, etc | - | 5 |
| **TOTAL**    | - | - | - | **95+** |

---

## Dependencies & Prerequisites

- ✅ Core Rust engine already has syllable parsing
- ✅ Telex/VNI input methods already mapped
- ✅ Buffer management in place
- ❌ DiacriticalValidator module doesn't exist (NEW)
- ✅ Test framework ready

---

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|-----------|
| Rejecting valid inputs | Medium | HIGH | Comprehensive test coverage, peer review |
| Performance degradation | Low | HIGH | Profile with cargo flamegraph, optimize validator |
| Platform inconsistency | Low | MEDIUM | Test on both macOS and Windows |
| Buffer state corruption | Low | HIGH | Integration tests verify state after rejection |

---

## Success Criteria

- ✅ All 95+ test cases passing
- ✅ No regressions in existing Telex/VNI functionality
- ✅ Keystroke latency remains <1ms (benchmark)
- ✅ No panics or memory issues
- ✅ Code reviewed and approved by core team
- ✅ Platform tests pass on macOS and Windows
- ✅ Smart Mode doesn't bypass validation

---

## Related Documents

- **Vietnamese Language System**: `.github/instructions/09_vietnamese-language-system.instructions.md` (Sections 4.4, 6.5, 7.6)
- **Backspace & Buffer**: `.github/instructions/07_backspace_buffer.instructions.md` (Buffer management rules)
- **Rust Guidelines**: Custom instruction in project (FFI safety, no panics)
- **Phase 3 Planning**: `.planning/phases/PHASE3_QUALITY_TESTING.md` (Milestone 3.1)

---

**Status**: Ready for implementation planning  
**Next Step**: Schedule Task 3.1 implementation session after Phase 2.9 completes

