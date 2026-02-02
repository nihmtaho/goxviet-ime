# TASK 3.2: Improve Tone Mark Removal (Telex `z` & VNI `0`)

**Author**: GoxViet Planning  
**Created**: 2026-02-01  
**Phase**: Phase 3 (Quality & Testing)  
**Status**: Planned  
**Estimated Duration**: 5-7 hours

---

## Overview

This task improves the **tone mark removal** feature in both Telex and VNI input methods. Currently, the system may not correctly handle tone removal when a tone mark is placed after a final consonant (e.g., "hoát" where tone is after 't'). Users should be able to reliably remove any tone mark by pressing Telex `z` or VNI `0`, regardless of where the tone was placed.

**Problem Example**:
- User types: Telex `h` `o` `a` `t` `s` → "hoát" (tone after vowel, before consonant visually)
- User presses: Telex `z` (remove tone)
- **CURRENT (BUG)**: May not correctly identify and remove the tone mark
- **DESIRED**: Return "hoat" (tone completely removed)

**Another Example**:
- User types: VNI `h` `o` `a` `1` → "hoá" (sắc tone on vowel)
- User presses: VNI `0` (remove tone)
- **CURRENT**: May work but not optimized
- **DESIRED**: Return "hoa" with guaranteed success

---

## Goal

Implement **robust tone mark removal** that correctly identifies and removes tone marks regardless of placement in the syllable, supporting both Telex and VNI methods, with proper undo/redo and state management.

---

## Acceptance Criteria

### Functional Requirements
- [ ] Telex `z` removes any tone mark currently in buffer (sắc, huyền, hỏi, ngã, nặng)
- [ ] VNI `0` removes any tone mark currently in buffer
- [ ] Tone removal works regardless of tone placement (after vowel or after consonant)
- [ ] Removed tone becomes "ngang" (no tone mark)
- [ ] Pressing `z`/`0` multiple times when no tone exists is safe (no-op)
- [ ] Undo/Redo works correctly after tone removal
- [ ] Backspace after tone removal works correctly
- [ ] Re-adding tone after removal works (e.g., `z` then `s` for sắc)
- [ ] All 6 tones can be removed: sắc, huyền, hỏi, ngã, nặng
- [ ] Smart Mode doesn't interfere with tone removal
- [ ] Per-App settings don't interfere with tone removal

### Non-Functional Requirements
- [ ] Performance: tone removal validation <0.5ms per keystroke (total <1ms)
- [ ] No memory leaks or buffer corruption during tone removal
- [ ] No panics across FFI boundary
- [ ] Test coverage: ≥95% of tone removal logic
- [ ] Consistent behavior across Telex and VNI

### Platform Requirements
- [ ] macOS InputManager correctly handles tone removal (no ghost chars)
- [ ] Windows TSF correctly handles tone removal (no ghost chars)
- [ ] Per-App Smart Mode respects tone removal
- [ ] Settings persistence works with tone removal

---

## Files to Change

### New Files
- **`core/src/engine_v2/tone_removal_processor.rs`** (NEW)
  - Core tone removal logic
  - Tone detection and extraction
  
- **`core/tests/tone_removal_tests.rs`** (NEW)
  - 60+ unit tests covering all scenarios

### Modified Files
- **`core/src/engine_v2/syllable_buffer.rs`**
  - Add `tone_position: Option<usize>` field to track where tone was placed
  - Add `find_and_remove_tone() -> bool` method

- **`core/src/engine_v2/syllable_parser.rs`**
  - Integrate tone removal processor
  - Handle Telex `z` and VNI `0` specially

- **`core/src/engine_v2/lib.rs`**
  - Export tone removal module

- **`core/tests/tone_removal_integration_tests.rs`** (NEW)
  - Integration tests with undo/redo, backspace

### Platform Files (Verification only)
- **`platforms/macos/GoxViet/InputManager.swift`** (READ verification)
- **`platforms/windows/GoxVietIME/InputManager.cpp`** (READ verification)

---

## High-Level Design

### Architecture Overview

```
User Input: Telex 'z' or VNI '0'
    ↓
InputManager (Platform Layer)
    ↓
ime_process_key() [FFI]
    ↓
Engine::process_key()
    ├─ Detect: is this a tone removal key?
    │  (Telex: 'z', VNI: '0')
    │
    ├─ Call ToneRemovalProcessor::remove_tone(&mut buffer)
    │  ├─ Search buffer for any tone mark (using tone_position field)
    │  ├─ If found:
    │  │  └─ Remove from buffer
    │  │  └─ Update syllable to ngang tone
    │  │  └─ Return: {modified: true, chars_to_output}
    │  └─ If not found:
    │     └─ Return: {modified: false, no_change}
    │
    └─ Return Result {action, chars, backspace_count}
    ↓
InputManager applies result
    ↓
Screen output
```

### Buffer Enhancement

**Current RawBuffer**:
```rust
pub struct RawBuffer {
    buffer: Vec<char>,
    // ... other fields ...
}
```

**Enhanced RawBuffer**:
```rust
pub struct RawBuffer {
    buffer: Vec<char>,
    tone_position: Option<usize>, // Index where tone mark is stored
    tone_type: Option<Tone>,       // Which tone (for recovery)
}
```

### Tone Removal Processor

```rust
// core/src/engine_v2/tone_removal_processor.rs

#[derive(Debug, Clone, Copy)]
pub struct ToneRemovalResult {
    pub removed: bool,
    pub tone_type: Option<Tone>, // What was removed
}

pub struct ToneRemovalProcessor;

impl ToneRemovalProcessor {
    /// Find and remove any tone mark in the current syllable state
    pub fn remove_tone(buffer: &mut RawBuffer) -> ToneRemovalResult {
        // Strategy: Check all known tone mark positions in buffer
        // 1. Scan buffer for Unicode tone characters (U+0300-U+036F range + Vietnamese specific)
        // 2. When found, determine which tone it is
        // 3. Remove from buffer
        // 4. Return syllable with ngang tone
        
        if let Some(tone_position) = buffer.tone_position {
            // Direct removal using tracked position
            if tone_position < buffer.buffer.len() {
                buffer.buffer.remove(tone_position);
                buffer.tone_position = None;
                
                return ToneRemovalResult {
                    removed: true,
                    tone_type: buffer.tone_type,
                };
            }
        }
        
        // Fallback: Search for tone marks
        for (i, &ch) in buffer.buffer.iter().enumerate() {
            if Self::is_tone_mark(ch) {
                buffer.buffer.remove(i);
                buffer.tone_position = None;
                
                return ToneRemovalResult {
                    removed: true,
                    tone_type: Some(Self::identify_tone(ch)),
                };
            }
        }
        
        // No tone found
        ToneRemovalResult {
            removed: false,
            tone_type: None,
        }
    }
    
    fn is_tone_mark(ch: char) -> bool {
        // Detect Vietnamese tone marks (combining marks)
        matches!(ch,
            '\u{0300}' // COMBINING GRAVE ACCENT (huyền)
            | '\u{0301}' // COMBINING ACUTE ACCENT (sắc)
            | '\u{0303}' // COMBINING TILDE (ngã)
            | '\u{0309}' // COMBINING HOOK ABOVE (hỏi)
            | '\u{0323}' // COMBINING DOT BELOW (nặng)
        )
    }
    
    fn identify_tone(ch: char) -> Tone {
        match ch {
            '\u{0300}' => Tone::Huyen,
            '\u{0301}' => Tone::Sac,
            '\u{0303}' => Tone::Nga,
            '\u{0309}' => Tone::Hoi,
            '\u{0323}' => Tone::Nang,
            _ => Tone::Ngang,
        }
    }
}
```

---

## Implementation Plan

### Step 1: Enhance RawBuffer Structure (1 hour)

**File**: `core/src/engine_v2/syllable_buffer.rs`

```rust
#[derive(Debug, Clone)]
pub struct RawBuffer {
    buffer: Vec<char>,
    tone_position: Option<usize>,  // NEW: Track where tone was placed
    tone_type: Option<Tone>,        // NEW: Track what tone was placed
    // ... existing fields ...
}

impl RawBuffer {
    pub fn new() -> Self {
        RawBuffer {
            buffer: Vec::with_capacity(20),
            tone_position: None,
            tone_type: None,
            // ... init other fields ...
        }
    }
    
    /// Record where tone mark was placed
    pub fn set_tone_position(&mut self, position: usize, tone: Tone) {
        self.tone_position = Some(position);
        self.tone_type = Some(tone);
    }
    
    /// Clear tone position tracking
    pub fn clear_tone_position(&mut self) {
        self.tone_position = None;
        self.tone_type = None;
    }
    
    /// Get the current tone (if any)
    pub fn get_tone(&self) -> Option<Tone> {
        self.tone_type
    }
    
    // ... existing methods ...
}
```

### Step 2: Create ToneRemovalProcessor (1.5 hours)

**File**: `core/src/engine_v2/tone_removal_processor.rs`

```rust
#[derive(Debug, Clone, Copy)]
pub struct ToneRemovalResult {
    pub removed: bool,
    pub tone_type: Option<Tone>,
}

pub struct ToneRemovalProcessor;

impl ToneRemovalProcessor {
    pub fn remove_tone(buffer: &mut RawBuffer) -> ToneRemovalResult {
        // Implementation as described above
    }
    
    fn is_tone_mark(ch: char) -> bool {
        // Detection logic
    }
    
    fn identify_tone(ch: char) -> Tone {
        // Identification logic
    }
    
    /// Check if buffer currently contains a tone mark
    pub fn has_tone_mark(buffer: &RawBuffer) -> bool {
        buffer.get_tone().is_some()
    }
    
    /// Get the current tone type
    pub fn get_tone_type(buffer: &RawBuffer) -> Option<Tone> {
        buffer.get_tone()
    }
}
```

### Step 3: Integrate with SyllableParser (1 hour)

**File**: `core/src/engine_v2/syllable_parser.rs`

Modify `process_keystroke()`:

```rust
pub fn process_keystroke(&mut self, ch: char) -> Result<ProcessResult> {
    // Check if this is a tone removal key
    let is_telex_z = ch == 'z' && self.input_method == InputMethod::Telex;
    let is_vni_0 = ch == '0' && self.input_method == InputMethod::VNI;
    
    if is_telex_z || is_vni_0 {
        // Special handling for tone removal
        let result = ToneRemovalProcessor::remove_tone(&mut self.buffer);
        
        if result.removed {
            // Success: tone was removed
            let output = self.buffer.to_string();
            return Ok(ProcessResult::with_output(output));
        } else {
            // No tone to remove: just return current state
            return Ok(ProcessResult::no_change());
        }
    }
    
    // ... rest of normal keystroke processing ...
}
```

### Step 4: Create Comprehensive Test Suite (2.5 hours)

**File**: `core/tests/tone_removal_tests.rs`

**Test Categories**:

#### Category A: Simple Tone Removal (Telex)
```rust
#[test]
fn test_remove_acute_telex_simple() {
    // ta + s = tá, then z = ta
    assert_eq!(process_telex("tasaz"), "ta");
}

#[test]
fn test_remove_grave_telex_simple() {
    // ta + f = tà, then z = ta
    assert_eq!(process_telex("tafaz"), "ta");
}

#[test]
fn test_remove_hook_telex_simple() {
    // ta + r = tả, then z = ta
    assert_eq!(process_telex("taraz"), "ta");
}

#[test]
fn test_remove_tilde_telex_simple() {
    // ta + x = tã, then z = ta
    assert_eq!(process_telex("taxaz"), "ta");
}

#[test]
fn test_remove_dot_telex_simple() {
    // ta + j = tạ, then z = ta
    assert_eq!(process_telex("tajaz"), "ta");
}
```

#### Category B: Tone Removal with Consonants
```rust
#[test]
fn test_remove_tone_with_final_consonant() {
    // tat + s = tát, then z = tat
    assert_eq!(process_telex("tatsaz"), "tat");
}

#[test]
fn test_remove_tone_hoa_with_consonant() {
    // hoat + s = hoát, then z = hoat
    assert_eq!(process_telex("hoatsaz"), "hoat");
}

#[test]
fn test_remove_tone_compound_vowel() {
    // toa + s = toá, then z = toa
    assert_eq!(process_telex("toasaz"), "toa");
}
```

#### Category C: VNI Equivalents
```rust
#[test]
fn test_remove_tone_vni_simple() {
    // ta + 1 + 0 = ta (sắc removed)
    assert_eq!(process_vni("ta10"), "ta");
}

#[test]
fn test_remove_tone_vni_grave() {
    // ta + 2 + 0 = ta (huyền removed)
    assert_eq!(process_vni("ta20"), "ta");
}

#[test]
fn test_remove_tone_vni_hook() {
    // ta + 3 + 0 = ta (hỏi removed)
    assert_eq!(process_vni("ta30"), "ta");
}

#[test]
fn test_remove_tone_vni_tilde() {
    // ta + 4 + 0 = ta (ngã removed)
    assert_eq!(process_vni("ta40"), "ta");
}

#[test]
fn test_remove_tone_vni_dot() {
    // ta + 5 + 0 = ta (nặng removed)
    assert_eq!(process_vni("ta50"), "ta");
}

#[test]
fn test_remove_tone_vni_with_consonant() {
    // tat + 1 + 0 = tat
    assert_eq!(process_vni("tat10"), "tat");
}
```

#### Category D: Edge Cases
```rust
#[test]
fn test_remove_tone_no_tone_present() {
    // ta (no tone) + z = ta (no-op)
    assert_eq!(process_telex("taz"), "ta");
}

#[test]
fn test_remove_tone_vni_no_tone() {
    // ta (no tone) + 0 = ta (no-op)
    assert_eq!(process_vni("ta0"), "ta");
}

#[test]
fn test_double_removal() {
    // ta + s + z + z = ta (first z removes, second z no-op)
    assert_eq!(process_telex("taszz"), "ta");
}

#[test]
fn test_remove_then_add() {
    // ta + s + z + s = ta + s = tá (remove then add back)
    assert_eq!(process_telex("taszs"), "tá");
}

#[test]
fn test_empty_buffer_removal() {
    // Empty + z = empty (no crash)
    assert_eq!(process_telex("z"), "");
}
```

#### Category E: Complex Scenarios
```rust
#[test]
fn test_remove_tone_nested_vowels() {
    // tiếng + z = tieng (tone removed)
    assert_eq!(process_telex("tieeengz"), "tieng");
}

#[test]
fn test_remove_tone_three_vowels() {
    // khuya + s + z = khuya (tone on compound)
    assert_eq!(process_telex("khuyasz"), "khuya");
}

#[test]
fn test_tone_removal_preserves_buffer() {
    // After removal, buffer should still be valid
    let output = process_telex("hoatsaz");
    assert_eq!(output, "hoat");
    
    // Should be able to add more input
    let extended = process_telex("hoatsaza"); // hoat + a
    assert_eq!(extended, "hoata");
}
```

#### Category F: Undo/Redo Integration
```rust
#[test]
fn test_undo_after_tone_removal() {
    // ta + s (tá) + z (ta) + undo = tá
    let mut state = BufferState::new();
    state.push('t');
    state.push('a');
    state.apply_tone(Tone::Sac);
    assert_eq!(state.to_string(), "tá");
    
    state.remove_tone();
    assert_eq!(state.to_string(), "ta");
    
    state.undo();
    assert_eq!(state.to_string(), "tá");
}

#[test]
fn test_redo_after_tone_removal_undo() {
    // ... undo, then redo
    // Should return to "ta"
}
```

**Total Test Cases**: 60+

### Step 5: Integration Testing (1 hour)

**File**: `core/tests/tone_removal_integration_tests.rs`

```rust
#[test]
fn test_tone_removal_with_backspace() {
    // ta + s (tá) + z (ta) + Backspace = t
    let mut buffer = RawBuffer::new();
    buffer.process("tas");
    buffer.remove_tone();
    assert_eq!(buffer.to_string(), "ta");
    
    buffer.backspace();
    assert_eq!(buffer.to_string(), "t");
}

#[test]
fn test_tone_removal_persistent_state() {
    // Tone removal should update SettingsManager tone tracking
    let mut settings = SettingsManager::new();
    settings.set_current_tone(Tone::Sac);
    
    remove_tone(&mut buffer);
    
    // Verify tone was cleared
    assert_eq!(settings.current_tone(), Tone::Ngang);
}

#[test]
fn test_tone_removal_respects_smart_mode() {
    // Smart Mode on/off shouldn't affect tone removal
    let mut settings = SettingsManager::new();
    settings.set_smart_mode(true);
    
    // Tone removal should work regardless
    assert!(can_remove_tone());
    
    settings.set_smart_mode(false);
    assert!(can_remove_tone());
}

#[test]
fn test_tone_removal_per_app_setting() {
    // Per-App mode shouldn't bypass tone removal
    let mut app_state = PerAppState::new("com.example.App");
    app_state.set_smart_mode_override(true);
    
    // Tone removal should still work
    assert!(can_remove_tone());
}
```

### Step 6: Platform Integration Testing (30 mins)

**File**: `platforms/macos/GoxVietTests/ToneRemovalTests.swift`

```swift
func testToneRemovalTelex() {
    // Verify InputManager correctly forwards z to engine
    let manager = InputManager()
    
    manager.handleKey(code: kVK_ANSI_T, ...)
    manager.handleKey(code: kVK_ANSI_A, ...)
    manager.handleKey(code: kVK_ANSI_S, ...)  // Apply sắc
    XCTAssertEqual(lastOutput, "tá")
    
    manager.handleKey(code: kVK_ANSI_Z, ...)  // Remove tone
    XCTAssertEqual(lastOutput, "ta")
}

func testToneRemovalVNI() {
    let manager = InputManager()
    
    manager.handleKey(code: kVK_ANSI_T, ...)
    manager.handleKey(code: kVK_ANSI_A, ...)
    manager.handleKey(code: kVK_1, ...)       // Apply sắc (VNI)
    XCTAssertEqual(lastOutput, "tá")
    
    manager.handleKey(code: kVK_0, ...)       // Remove tone (VNI)
    XCTAssertEqual(lastOutput, "ta")
}
```

---

## Test Coverage Matrix

| Category | Telex Cases | VNI Cases | Edge Cases | Total |
|----------|-------------|-----------|-----------|-------|
| Simple removal (all 6 tones) | 5 | 5 | - | 10 |
| With consonants | 3 | 3 | - | 6 |
| Compound vowels | 3 | 3 | - | 6 |
| Edge cases | 6 | 6 | 4 | 16 |
| Undo/Redo | - | - | 4 | 4 |
| Integration | - | - | 8 | 8 |
| Platform | 2 | 2 | - | 4 |
| **TOTAL** | **19** | **19** | **16** | **60+** |

---

## Dependencies & Prerequisites

- ✅ Core Rust engine has syllable parsing
- ✅ Telex/VNI input methods mapped
- ✅ Buffer management in place
- ✅ Undo/Redo framework exists
- ❌ ToneRemovalProcessor module doesn't exist (NEW)
- ❌ RawBuffer.tone_position tracking doesn't exist (NEW)

---

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|-----------|
| Tone removal doesn't work in all cases | Medium | HIGH | Comprehensive test coverage (60+ cases) |
| State corruption after removal | Low | HIGH | Integration tests verify buffer state |
| Performance regression | Low | HIGH | Profile removal function, keep <0.5ms |
| Inconsistency Telex vs VNI | Medium | MEDIUM | Test both methods equally |
| Platform-specific issues | Low | MEDIUM | Test on macOS and Windows |
| Undo/Redo breaking | Low | HIGH | Detailed undo/redo tests |

---

## Success Criteria

- ✅ All 60+ test cases passing
- ✅ All 6 tones removable (sắc, huyền, hỏi, ngã, nặng, ngang)
- ✅ Telex `z` and VNI `0` work consistently
- ✅ No regressions in existing functionality
- ✅ Keystroke latency remains <1ms
- ✅ Undo/Redo works correctly
- ✅ No panics or memory issues
- ✅ Smart Mode doesn't interfere
- ✅ Per-App settings don't interfere
- ✅ Code reviewed and approved

---

## Related Documents

- **Vietnamese Language System**: `.github/instructions/09_vietnamese-language-system.instructions.md` (Sections 7-9)
- **Backspace & Buffer**: `.github/instructions/07_backspace_buffer.instructions.md` (State management)
- **Tone Placement Rules**: `.github/instructions/09_vietnamese-language-system.instructions.md` (Section 7.6)
- **Rust Guidelines**: Custom instruction (error handling, no panics)
- **Phase 3 Planning**: `.planning/phases/PHASE3_QUALITY_TESTING.md` (Milestone 3.2)

---

**Status**: Ready for implementation planning  
**Next Step**: Schedule Task 3.2 implementation session after Phase 2.9 completes

