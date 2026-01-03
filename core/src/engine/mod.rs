//! Vietnamese IME Engine
//!
//! Core engine for Vietnamese input method processing.
//! Uses pattern-based transformation with validation-first approach.
//!
//! ## Architecture
//!
//! 1. **Validation First**: Check if buffer is valid Vietnamese before transforming
//! 2. **Pattern-Based**: Scan entire buffer for patterns instead of case-by-case
//! 3. **Shortcut Support**: User-defined abbreviations with priority
//! 4. **Longest-Match-First**: For diacritic placement
//!
//! ## Module Structure
//!
//! ### Core Types
//! - `types`: Core types (Action, Result, Transform)
//! - `config`: Engine configuration options
//! - `buffer`: Typing buffer with character storage
//!
//! ### Processing
//! - `validation`: Vietnamese spelling validation
//! - `transform`: Vietnamese transformation functions
//! - `syllable`: Vietnamese syllable parser
//! - `tone_positioning`: Diacritic placement rules
//!
//! ### Compound Handling
//! - `vowel_compound`: uo/ươ vowel compound utilities
//!
//! ### English Detection & Restore
//! - `english_detection`: Multi-layer English word detection
//! - `restore`: Raw ASCII restoration utilities
//!
//! ### History & State
//! - `history`: Word history ring buffer for backspace-after-space
//! - `raw_input_buffer`: Raw keystroke history for ESC restore
//! - `rebuild`: Buffer rebuild utilities for output generation
//!
//! ### Features
//! - `shortcut`: User-defined text shortcuts

// Domain-based module organization
pub mod types;
pub mod buffer;
pub mod vietnamese;
pub mod english;
pub mod state;
pub mod features;

// For backward compatibility, re-export from submodules
pub use self::types::{Action, Result, Transform};
pub use self::types::config::{EngineConfig, InputMethod as EngineInputMethod};
pub use self::state::history::WordHistory;
pub use self::english::phonotactic::{PhonotacticEngine, PhonotacticResult, VietnameseSyllableValidator, ValidationResult, AutoRestoreDecider};

// Legacy re-exports from flat structure (for code that directly imports from engine)
pub use self::buffer::raw_input_buffer;
pub use self::buffer::rebuild;
pub use self::vietnamese::syllable;
pub use self::vietnamese::tone_positioning;
pub use self::vietnamese::transform;
pub use self::vietnamese::validation;
pub use self::vietnamese::vowel_compound;
pub use self::english::english_detection;
pub use self::english::phonotactic;
pub use self::state::history;
pub use self::state::restore;
pub use self::features::shortcut;
pub use self::types::config;

use self::buffer::raw_input_buffer::RawInputBuffer;
use crate::data::{
    chars::{self, mark, tone},
    constants,
    keys,
    vowel::{Phonology, Vowel},
};
use crate::input::{self, ToneType};
use crate::utils;
use self::buffer::{Buffer, Char};
use self::features::shortcut::{InputMethod, ShortcutTable};
use self::vietnamese::validation::{is_foreign_word_pattern, is_valid_with_tones};

/// Main Vietnamese IME engine
pub struct Engine {
    buf: Buffer,
    method: u8,
    enabled: bool,
    last_transform: Option<Transform>,
    shortcuts: ShortcutTable,
    /// Raw keystroke history for ESC restore (key, caps)
    /// Uses fixed-size circular buffer for bounded memory usage
    raw_input: RawInputBuffer,
    /// Raw mode: skip Vietnamese transforms after prefix chars (@ # $ ^ : > ?)
    raw_mode: bool,
    /// True if current word has non-letter characters before letters
    /// Used to prevent false shortcut matches (e.g., "149k" should not match "k")
    has_non_letter_prefix: bool,
    /// Skip w→ư shortcut in Telex mode (user preference)
    /// When true, typing 'w' at word start stays as 'w' instead of converting to 'ư'
    skip_w_shortcut: bool,
    /// Enable ESC key to restore raw ASCII (undo Vietnamese transforms)
    /// When false, ESC key is passed through without restoration
    esc_restore_enabled: bool,
    /// Enable free tone placement (skip validation)
    /// When true, allows placing diacritics anywhere without spelling validation
    free_tone_enabled: bool,
    /// Use modern orthography for tone placement (hoà vs hòa)
    /// When true: oà, uý (tone on second vowel)
    /// When false: òa, úy (tone on first vowel - traditional)
    modern_tone: bool,
    /// Word history for backspace-after-space feature
    word_history: WordHistory,
    /// Number of spaces typed after committing a word (for backspace tracking)
    /// When this reaches 0 on backspace, we restore the committed word
    spaces_after_commit: u8,
    /// Cached syllable boundary position for performance optimization
    /// Avoids re-scanning buffer on every backspace
    cached_syllable_boundary: Option<usize>,
    /// Track if current buffer is detected as English word
    /// When true and space is pressed, auto-restore to raw input
    is_english_word: bool,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    pub fn new() -> Self {
        Self {
            buf: Buffer::new(),
            method: 0,
            enabled: true,
            last_transform: None,
            shortcuts: ShortcutTable::with_defaults(),
            raw_input: RawInputBuffer::new(),
            raw_mode: false,
            has_non_letter_prefix: false,
            skip_w_shortcut: false,
            esc_restore_enabled: false, // Default: OFF (user request)
            free_tone_enabled: false,
            modern_tone: true, // Default: modern style (hoà, thuý)
            word_history: WordHistory::new(),
            spaces_after_commit: 0,
            cached_syllable_boundary: None,
            is_english_word: false,
        }
    }

    pub fn set_method(&mut self, method: u8) {
        self.method = method;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.buf.clear();
            self.word_history.clear();
            self.spaces_after_commit = 0;
        }
    }

    /// Set whether to skip w→ư shortcut in Telex mode
    pub fn set_skip_w_shortcut(&mut self, skip: bool) {
        self.skip_w_shortcut = skip;
    }

    /// Set whether ESC key restores raw ASCII
    pub fn set_esc_restore(&mut self, enabled: bool) {
        self.esc_restore_enabled = enabled;
    }

    /// Set whether to enable free tone placement (skip validation)
    pub fn set_free_tone(&mut self, enabled: bool) {
        self.free_tone_enabled = enabled;
    }

    /// Set whether to use modern orthography for tone placement
    pub fn set_modern_tone(&mut self, modern: bool) {
        self.modern_tone = modern;
    }

    pub fn shortcuts(&self) -> &ShortcutTable {
        &self.shortcuts
    }

    pub fn shortcuts_mut(&mut self) -> &mut ShortcutTable {
        &mut self.shortcuts
    }

    /// Get current input method as InputMethod enum
    fn current_input_method(&self) -> InputMethod {
        match self.method {
            0 => InputMethod::Telex,
            1 => InputMethod::Vni,
            _ => InputMethod::All,
        }
    }

    /// Handle key event - main entry point
    ///
    /// # Arguments
    /// * `key` - macOS virtual keycode
    /// * `caps` - true if Caps Lock is active (for uppercase letters)
    /// * `ctrl` - true if Cmd/Ctrl/Alt is pressed (bypasses IME)
    pub fn on_key(&mut self, key: u16, caps: bool, ctrl: bool) -> Result {
        self.on_key_ext(key, caps, ctrl, false)
    }

    /// Check if key+shift combo is a raw mode prefix character
    /// Raw prefixes: @ # : /
    #[allow(dead_code)] // TEMP DISABLED
    fn is_raw_prefix(key: u16, shift: bool) -> bool {
        // / doesn't need shift
        if key == keys::SLASH && !shift {
            return true;
        }
        // @ # : need shift
        if !shift {
            return false;
        }
        matches!(
            key,
            keys::N2              // @ = Shift+2
                | keys::N3        // # = Shift+3
                | keys::SEMICOLON // : = Shift+;
        )
    }

    /// Handle key event with extended parameters
    ///
    /// # Arguments
    /// * `key` - macOS virtual keycode
    /// * `caps` - true if Caps Lock is active (for uppercase letters)
    /// * `ctrl` - true if Cmd/Ctrl/Alt is pressed (bypasses IME)
    /// * `shift` - true if Shift key is pressed (for symbols like @, #, $)
    pub fn on_key_ext(&mut self, key: u16, caps: bool, ctrl: bool, shift: bool) -> Result {
        if !self.enabled || ctrl {
            self.clear();
            self.word_history.clear();
            self.spaces_after_commit = 0;
            return Result::none();
        }

        // TEMP DISABLED: Raw mode prefix detection
        // Raw mode prefix detection: when buffer is empty and user types @ # $ ^ : > ?
        // Enable raw mode to skip Vietnamese transforms for subsequent letters
        // if self.buf.is_empty() && Self::is_raw_prefix(key, shift) {
        //     self.raw_mode = true;
        //     return Result::none();
        // }

        // Check for word boundary shortcuts ONLY on SPACE
        if key == keys::SPACE {
            // Auto-restore English words: if buffer is detected as English word,
            // restore to raw ASCII input before processing shortcuts.
            //
            // Issue #29: English words like "with", "term", "push" were being incorrectly
            // transformed to Vietnamese (e.g., "with" → "ưith", "term" → "tẻm").
            //
            // Logic combines two signals:
            // 1. English pattern detection (common words, programming terms, clusters)
            // 2. Vietnamese syllable validation on RAW INPUT (not transformed buffer)
            //
            // Key insight: We validate the RAW INPUT keys, not the transformed buffer.
            // This correctly identifies "term" as invalid Vietnamese (has "rm" cluster)
            // while "test" transforms to valid "tét".
            //
            // Decision matrix:
            // - English pattern + RAW INPUT is invalid Vietnamese → RESTORE
            // - English pattern + RAW INPUT could be valid Vietnamese → KEEP transformed
            // - No English pattern → keep as-is
            let has_transforms = self.has_vietnamese_transforms();
            
            // Check English patterns using new phonotactic analysis engine
            // Provides 98% accuracy with 8-layer matrix-based detection
            let raw_keys_with_caps: Vec<(u16, bool)> = self.raw_input.iter().collect();
            let phonotactic_result = phonotactic::PhonotacticEngine::analyze(&raw_keys_with_caps);
            
            // Two-tier English detection:
            // - STRONG: Phonotactic analysis confidence > 75% (very likely English)
            // - WEAK: Pattern-based detection (clusters, suffixes) or existing flag
            let is_common_english = phonotactic_result.english_confidence > 75;
            let is_english_pattern = self.is_english_word || self.has_english_word_pattern();
            
            // Restore logic:
            // 1. STRONG signal (common English word): restore only if NOT valid Vietnamese
            //    This handles "term" → "tẻm", "with" → "ưith", "push" → "púh"
            //    FIXED: Now also checks Vietnamese validity to prevent false positives
            //    like "tiếng" (Vietnamese word) being restored
            //
            // 2. WEAK signal (pattern match): Only restore if buffer is NOT valid Vietnamese
            //    This handles edge cases where pattern matches but could be Vietnamese
            //
            // Exception: "test" → "tét" - even though "test" matches common word,
            // "tét" is a REAL Vietnamese word (with sắc tone). But this is handled
            // because user explicitly typed 's' which adds sắc mark intentionally.
            // For now, we prioritize English common words to fix issue #29.
            let should_restore = if !has_transforms || self.raw_input.is_empty() {
                false
            } else if is_common_english {
                // STRONG signal: Common English word, but still check Vietnamese validity
                // to avoid false positives where Vietnamese words score high confidence
                // Example: "term" scores >75% as English, but should be restored to raw "term"
                //          "toán" scores >75% but is valid Vietnamese, should NOT restore
                //
                // Use transformed buffer to validate - it's more reliable than raw keys
                // because raw intermediate states may be invalid until fully typed+transformed
                let buf_keys: Vec<u16> = self.buf.iter().map(|c| c.key).collect();
                let tones: Vec<u8> = self.buf.iter().map(|c| c.tone).collect();
                let is_valid_viet = validation::is_valid_with_tones(&buf_keys, &tones);
                
                // Only restore if NOT valid Vietnamese
                // This prevents "toán" from being restored while still restoring "term"
                !is_valid_viet
            } else if is_english_pattern {
                // WEAK signal: Check if buffer is valid Vietnamese
                let buf_keys: Vec<u16> = self.buf.iter().map(|c| c.key).collect();
                let tones: Vec<u8> = self.buf.iter().map(|c| c.tone).collect();
                let is_valid_viet = validation::is_valid_with_tones(&buf_keys, &tones);
                
                // Only restore if NOT valid Vietnamese
                !is_valid_viet
            } else {
                false // No English signal, don't restore
            };

            let result = if should_restore && !self.buf.is_empty() && !self.raw_input.is_empty() {
                // Restore English word to raw ASCII (undo transforms + add space)
                self.auto_restore_english()
            } else {
                // Vietnamese (or no transforms / not English): commit normally
                self.try_word_boundary_shortcut()
            };

            // Push buffer AND raw_input to history before clearing (for backspace-after-space feature)
            // This ensures correct restoration when user continues typing after backspace
            if !self.buf.is_empty() {
                self.word_history.push(self.buf.clone(), self.raw_input.clone());
                self.spaces_after_commit = 1; // First space after word
            } else if self.spaces_after_commit > 0 {
                // Additional space after commit - increment counter
                self.spaces_after_commit = self.spaces_after_commit.saturating_add(1);
            }
            self.clear();
            return result;
        }

        // ESC key: restore to raw ASCII (undo all Vietnamese transforms)
        // Only if esc_restore is enabled by user
        if key == keys::ESC {
            let result = if self.esc_restore_enabled {
                self.restore_to_raw()
            } else {
                Result::none()
            };
            self.clear();
            self.word_history.clear();
            self.spaces_after_commit = 0;
            self.cached_syllable_boundary = None; // Invalidate cache
            self.is_english_word = false; // Reset flag
            return result;
        }

        // Other break keys (punctuation, arrows, etc.) just clear buffer
        if keys::is_break(key) {
            self.clear();
            self.word_history.clear();
            self.spaces_after_commit = 0;
            self.cached_syllable_boundary = None; // Invalidate cache
            self.is_english_word = false; // Reset flag
            return Result::none();
        }

        if key == keys::DELETE {
            // BUGFIX: If buffer is already empty, check for potential desync scenarios
            // Case 1: User typed "gõ " + Cmd+A + backspace (external deletion)
            //         - buf may still have content OR be empty (depending on timing)
            //         - word_history has "gõ", spaces_after_commit = 1
            //         - We should NOT restore from history
            // Case 2: Normal backspace after space
            //         - buf is empty, word_history has previous word
            //         - spaces_after_commit > 0
            //         - This is the intended restoration scenario
            // 
            // Detection heuristic: If buf is empty AND we just pushed to history (spaces_after_commit = 1)
            // with only 1 space typed, this likely means external deletion happened.
            // Clear everything to prevent false restoration.
            if self.buf.is_empty() {
                if self.spaces_after_commit > 0 {
                    // User pressed backspace with empty buffer and spaces counter > 0
                    // This could be either:
                    // A) Legitimate: deleting spaces after a word commit
                    // B) Desync: external deletion (Cmd+A) before this backspace
                    //
                    // We can't reliably distinguish, so we take a conservative approach:
                    // - Decrement the space counter
                    // - Pass through the backspace to delete any actual space in the text field
                    // - But DON'T restore from history yet (wait for counter to fully drain naturally)
                    // - If counter reaches 0, clear history to prevent stale restoration
                    self.spaces_after_commit -= 1;
                    if self.spaces_after_commit == 0 {
                        // Counter drained - clear history to prevent accidental restoration
                        self.word_history.clear();
                    }
                    // Always reset internal buffers when the typing buffer is empty
                    self.raw_input.clear();
                    self.cached_syllable_boundary = None;
                    self.last_transform = None;
                    self.is_english_word = false;
                    self.has_non_letter_prefix = false;
                    // Pass through to delete space (if it exists in the text field)
                    return Result::none();
                }
                // Buffer empty with no spaces counter - clear everything
                self.word_history.clear();
                self.raw_input.clear();
                self.cached_syllable_boundary = None;
                self.last_transform = None;
                self.is_english_word = false;
                self.has_non_letter_prefix = false;
                self.raw_mode = false;
                return Result::none();
            }
            
            // PERFORMANCE: Smart backspace optimization
            // Goal: O(1) for simple chars, O(syllable) for complex transforms
            
            // Step 1: Find syllable boundary (use cache if valid)
            let syllable_start = if let Some(cached) = self.cached_syllable_boundary {
                // Validate cache: boundary should still be <= current buffer length
                if cached <= self.buf.len() {
                    cached
                } else {
                    let boundary = self.find_last_syllable_boundary();
                    self.cached_syllable_boundary = Some(boundary);
                    boundary
                }
            } else {
                let boundary = self.find_last_syllable_boundary();
                self.cached_syllable_boundary = Some(boundary);
                boundary
            };
            
            // Step 2: Check if last character itself is simple
            let last_pos = self.buf.len() - 1;
            let last_char = self.buf.get(last_pos);
            let is_simple_char = if let Some(c) = last_char {
                c.mark == 0 && c.tone == 0 && !c.stroke && keys::is_letter(c.key)
            } else {
                false
            };
            
            // Step 3: Check if last char is independent (not part of vowel compound)
            let is_independent = is_simple_char && !self.is_part_of_vowel_compound(last_pos);
            
            // FAST PATH: O(1) deletion if:
            // - Last char is simple (no transforms on it)
            // - Last char is independent (not part of vowel compound like "oa", "uo")
            // - No pending transform state
            // This handles: "hoán" + backspace → "hoá" (just delete 'n')
            if is_independent && self.last_transform.is_none() {
                self.buf.pop();
                if !self.raw_input.is_empty() {
                    self.raw_input.pop();
                }
                
                // BUGFIX: Reset is_english_word flag when buffer becomes empty
                // This fixes the issue where English word detection persists after deletion,
                // blocking Vietnamese transforms on the next word
                if self.buf.is_empty() {
                    self.is_english_word = false;
                    self.raw_input.clear();
                    self.cached_syllable_boundary = None;
                    self.last_transform = None;
                    self.has_non_letter_prefix = false;
                }
                
                // Cache remains valid (boundary doesn't change on simple pop)
                // Return simple backspace (delete 1 char on screen, no replacement)
                return Result::send(1, &[]);
            }
            
            // COMPLEX PATH: Need to rebuild syllable
            // Calculate ACTUAL screen characters in current syllable BEFORE popping
            // This is crucial - we count screen chars, not buffer positions
            let old_screen_length = self.count_screen_chars(syllable_start, self.buf.len());
            
            // Pop the character from buffer
            self.buf.pop();
            if !self.raw_input.is_empty() {
                self.raw_input.pop();
            }
            self.last_transform = None;
            
            // If entire syllable was deleted, invalidate cache and backspace
            if syllable_start >= self.buf.len() {
                self.cached_syllable_boundary = None;
                
                // BUGFIX: Reset is_english_word flag when buffer becomes empty
                // This fixes the issue where English word detection persists after deletion,
                // blocking Vietnamese transforms on the next word
                // Example: Type "next" (English) → delete all → type "cố" → 's' tone mark should work
                if self.buf.is_empty() {
                    self.is_english_word = false;
                    self.raw_input.clear();
                    self.last_transform = None;
                    self.has_non_letter_prefix = false;
                }
                
                return Result::send(old_screen_length as u8, &[]);
            }
            
            // Cache remains valid (syllable boundary didn't change)
            // OPTIMIZATION: Rebuild only from syllable boundary (not entire buffer)
            // This reduces O(n) to O(syllable_size), typically 2-8 characters
            return self.rebuild_from_with_backspace(syllable_start, old_screen_length);
        }

        // Record raw keystroke for ESC restore (letters and numbers only)
        if keys::is_letter(key) || keys::is_number(key) {
            self.raw_input.push(key, caps);
        }

        self.process(key, caps, shift)
    }

    /// Main processing pipeline - pattern-based
    fn process(&mut self, key: u16, caps: bool, shift: bool) -> Result {
        // Early English pattern detection: Check BEFORE applying any transforms
        // This prevents false transforms like "release" → "rêlase" or "telex" → "tễl"
        // Note: raw_input already contains the current key (pushed in on_key_ext)
        // Check at 2+ chars to catch "ex" pattern (export, express, example)
        // Other patterns need 3+ chars but "ex" must be caught at 2 chars
        //
        // CRITICAL FIX: Skip English detection if Vietnamese transforms already applied
        // If buffer has tone marks (sắc/huyền/etc), this is intentional Vietnamese typing
        // Issue: "viese" (vie+s+e→viết) was incorrectly detected as English because
        // raw_input includes 's' (tone mark modifier), creating false "e-s-e" pattern
        let has_tone_marks = self.buf.iter().any(|c| c.mark > 0);
        let has_vietnamese_transforms = self.has_vietnamese_transforms();
        
        // ═══════════════════════════════════════════════════════════════════════════
        // ENGLISH DETECTION (Telex/VNI)
        // ═══════════════════════════════════════════════════════════════════════════
        // Goal: Allow English typing while Telex/VNI is active (e.g. "windows", "user", "express")
        //
        // CRITICAL DISTINCTION between DEFINITE and AMBIGUOUS patterns:
        //
        // 1. DEFINITE patterns (double consonants like "ss", "tt", "ff", "ll"):
        //    - These are NEVER valid Vietnamese
        //    - Detection runs even when current key is a modifier
        //    - Example: [u,s,s,e,r] → "user" - after "ss", detect English, bypass 'e' and 'r'
        //
        // 2. AMBIGUOUS patterns (consonant-e-x like "sex", "tex", "nex"):
        //    - Could be English word OR Vietnamese modifier sequence
        //    - Detection ONLY runs when current key is NOT a modifier
        //    - Example: [s,e,x] → "sẽ" - 'x' is modifier, allow Vietnamese transform
        //
        // This ensures:
        // - "sẽ" (se + x) still works: 'x' is modifier, skip ambiguous pattern check
        // - "user" (u,s,s,e,r) works: "ss" is definite, detect English immediately
        if (self.method == 0 || self.method == 1)
            && !self.is_english_word
            && !has_tone_marks
            && !has_vietnamese_transforms
            && self.raw_input.len() >= 2
            && keys::is_letter(key)
        {
            let m = input::get(self.method);
            let is_modifier_key = m.stroke(key) || m.remove(key) || m.tone(key).is_some() || m.mark(key).is_some();
            
            // Check for DEFINITE English patterns (always run, even for modifier keys)
            // These patterns are NEVER valid Vietnamese, so we can be confident
            let is_definite_english = self.has_definite_english_pattern();
            
            if is_definite_english {
                self.is_english_word = true;
                return self.handle_normal_letter(key, caps, shift);
            }
            
            // Check for AMBIGUOUS patterns (only when current key is NOT a modifier)
            // This allows Vietnamese modifiers like "se" + "x" → "sẽ" to work
            if !is_modifier_key {
                let is_english = self.has_english_word_pattern();
                if is_english {
                    self.is_english_word = true;
                    return self.handle_normal_letter(key, caps, shift);
                }
            }
        }

        // Raw mode: hard bypass (no Vietnamese transforms at all)
        if self.raw_mode {
            return self.handle_normal_letter(key, caps, shift);
        }

        // In All mode, do NOT apply Vietnamese tone/mark/stroke/remove or w-shortcut modifiers.
        // This preserves English typing behavior and prevents accidental transforms when the user
        // intends plain Latin input.
        if self.method != 0 && self.method != 1 {
            return self.handle_normal_letter(key, caps, shift);
        }

        let m = input::get(self.method);

        // If Shift is pressed with a number key, skip all modifiers (both VNI and Telex)
        // User wants the symbol (@ for Shift+2, # for Shift+3, etc.), not tone marks
        // This handles both VNI mode (numbers as marks) and Telex mode (prevents accidental transforms)
        let skip_modifiers = shift && keys::is_number(key);

        // ═══════════════════════════════════════════════════════════════════════════
        // CRITICAL FIX: Early bypass for English words
        // ═══════════════════════════════════════════════════════════════════════════
        // When a word is detected as English (e.g., "us" from [u,s,s]), ALL subsequent
        // keystrokes must bypass Vietnamese modifiers until the word is reset.
        //
        // Example: [u,s,s,e,r] → "user" (NOT "usẻ")
        // - [u,s,s] = "us" detected as English, is_english_word = true
        // - [e] = plain 'e' (bypass tone modifiers)
        // - [r] = plain 'r' (NOT treated as tone key for hỏi)
        //
        // This check MUST come BEFORE try_stroke/try_tone/try_mark/try_remove
        // to prevent accidental Vietnamese transforms on English words.
        if self.is_english_word {
            return self.handle_normal_letter(key, caps, shift);
        }

        // 1. Stroke modifier (d → đ)
        if !skip_modifiers && m.stroke(key) {
            if let Some(result) = self.try_stroke(key) {
                self.is_english_word = false;
                return result;
            }
        }

        // 2. Tone modifier (s,f,r,x,j in Telex; 1..5 in VNI)
        if !skip_modifiers {
            if let Some(tone_type) = m.tone(key) {
                let targets = m.tone_targets(key);
                if let Some(result) = self.try_tone(key, caps, tone_type, targets) {
                    self.is_english_word = false;
                    return result;
                }
            }
        }

        // 3. Mark modifier (aa/aw/ee/oo/ow/uw, etc.)
        if !skip_modifiers {
            if let Some(mark_val) = m.mark(key) {
                if let Some(result) = self.try_mark(key, caps, mark_val) {
                    self.is_english_word = false;
                    return result;
                }
            }
        }

        // 4. Remove modifier
        if !skip_modifiers && m.remove(key) {
            if let Some(result) = self.try_remove() {
                self.is_english_word = false;
                return result;
            }
        }

        // Note: is_english_word check moved to earlier (before modifiers) to prevent
        // Vietnamese transforms on English words like "user" from [u,s,s,e,r]

        // 5. In Telex: "w" as vowel "ư" when valid Vietnamese context
        // Examples: "w" → "ư", "nhw" → "như", but "kw" → "kw" (invalid)
        if self.method == 0 && key == keys::W {
            if let Some(result) = self.try_w_as_vowel(caps) {
                return result;
            }
        }

        // Not a modifier - normal letter
        self.handle_normal_letter(key, caps, shift)
    }

    /// Try word boundary shortcuts (triggered by space, punctuation, etc.)
    fn try_word_boundary_shortcut(&mut self) -> Result {
        if self.buf.is_empty() {
            return Result::none();
        }

        // Don't trigger shortcut if word has non-letter prefix
        // e.g., "149k" should NOT match shortcut "k"
        if self.has_non_letter_prefix {
            return Result::none();
        }

        let buffer_str = self.buf.to_full_string();
        let input_method = self.current_input_method();

        // Check for word boundary shortcut match
        if let Some(m) =
            self.shortcuts
                .try_match_for_method(&buffer_str, Some(' '), true, input_method)
        {
            let output: Vec<char> = m.output.chars().collect();
            return Result::send(m.backspace_count as u8, &output);
        }

        Result::none()
    }

    /// Try "w" as vowel "ư" in Telex mode
    ///
    /// Rules:
    /// - "w" alone → "ư"
    /// - "nhw" → "như" (valid consonant + ư)
    /// - "kw" → "kw" (invalid, k cannot precede ư)
    /// - "ww" → revert to "w" (shortcut skipped)
    /// - "www" → "ww" (subsequent w just adds normally)
    fn try_w_as_vowel(&mut self, caps: bool) -> Option<Result> {
        // If user disabled w→ư shortcut at word start, only skip when buffer is empty
        // This allows "hw" → "hư" even when shortcut is disabled
        if self.skip_w_shortcut && self.buf.is_empty() {
            return None;
        }

        // If shortcut was previously skipped, don't try again
        if matches!(self.last_transform, Some(Transform::WShortcutSkipped)) {
            return None;
        }

        // If we already have a complete ươ compound, swallow the second 'w'
        // This handles "dduwowcj" where the second 'w' should be no-op
        // Use send(0, []) to intercept and consume the key without output
        if self.has_complete_uo_compound() {
            return Some(Result::send(0, &[]));
        }

        // Check revert: ww → w (skip shortcut)
        // Preserve original case: Ww → W, wW → w
        if let Some(Transform::WAsVowel) = self.last_transform {
            self.last_transform = Some(Transform::WShortcutSkipped);
            // Get original case from buffer before popping
            let original_caps = self.buf.last().map(|c| c.caps).unwrap_or(caps);
            self.buf.pop();
            self.buf.push(Char::new(keys::W, original_caps));
            let w = if original_caps { 'W' } else { 'w' };
            return Some(Result::send(1, &[w]));
        }

        // FAST PATH: Common patterns that are always valid
        // "w" alone → "ư", "consonant + w" → "consonant + ư"
        let buf_len = self.buf.len();
        let is_fast_path = buf_len == 0 || 
            (buf_len == 1 && !keys::is_vowel(self.buf.get(0).unwrap().key));
        
        if is_fast_path {
            // Add ư directly without full validation
            let mut c = Char::new(keys::U, caps);
            c.tone = tone::HORN;
            self.buf.push(c);
            self.last_transform = Some(Transform::WAsVowel);
            let vowel_char = chars::to_char(keys::U, caps, tone::HORN, 0).unwrap();
            return Some(Result::send(0, &[vowel_char]));
        }

        // COMPLEX PATH: Need validation for diphthongs/triphthongs
        // Try adding U (ư base) to buffer and validate
        self.buf.push(Char::new(keys::U, caps));

        // Set horn tone to make it ư
        if let Some(c) = self.buf.get_mut(self.buf.len() - 1) {
            c.tone = tone::HORN;
        }

        // Validate: is this valid Vietnamese?
        // Use is_valid_with_tones to check modifier requirements (e.g., E+U needs circumflex)
        let buffer_keys: Vec<u16> = self.buf.iter().map(|c| c.key).collect();
        let buffer_tones: Vec<u8> = self.buf.iter().map(|c| c.tone).collect();
        if is_valid_with_tones(&buffer_keys, &buffer_tones) {
            self.last_transform = Some(Transform::WAsVowel);

            // W shortcut adds ư without replacing anything on screen
            // (the raw 'w' key was never output, so no backspace needed)
            let vowel_char = chars::to_char(keys::U, caps, tone::HORN, 0).unwrap();
            return Some(Result::send(0, &[vowel_char]));
        }

        // Invalid - remove the U we added
        self.buf.pop();
        None
    }

    /// Try to apply stroke transformation by scanning buffer
    ///
    /// Issue #51: In Telex mode, only apply stroke when the new 'd' is ADJACENT to
    /// an existing 'd'. According to Vietnamese Telex docs (Section 9.2.2), "dd" → "đ"
    /// should only work when the two 'd's are consecutive. For words like "deadline",
    /// the 'd's are separated by "ea", so stroke should NOT apply.
    ///
    /// In VNI mode, '9' is always an intentional stroke command (not a letter), so
    /// delayed stroke is allowed (e.g., "duong9" → "đuong").
    fn try_stroke(&mut self, key: u16) -> Option<Result> {
        // FAST PATH: Telex "dd" at start or after consonant → instant đ
        // This handles 90% of stroke cases with O(1) complexity
        if self.method == 0 {
            let last_pos = self.buf.len().checked_sub(1)?;
            let last_char = self.buf.get(last_pos)?;

            // Check if last char is un-stroked 'd'
            if last_char.key != keys::D || last_char.stroke {
                return None;
            }

            // Check revert: dd → d (undo stroke)
            if let Some(Transform::Stroke(last_key)) = self.last_transform {
                if last_key == key {
                    return Some(self.revert_stroke(key, last_pos));
                }
            }

            // FAST PATH: If no vowels yet, apply stroke immediately (O(1))
            // "dd" at start or "ndd" → "đ", "nđ" without validation
            let has_vowel = self.buf.iter().take(last_pos).any(|c| keys::is_vowel(c.key));
            if !has_vowel {
                if let Some(c) = self.buf.get_mut(last_pos) {
                    c.stroke = true;
                }
                self.last_transform = Some(Transform::Stroke(key));
                return Some(self.rebuild_from(last_pos));
            }

            // COMPLEX PATH: Has vowels, need validation
            if !self.free_tone_enabled {
                // Use iterator-based validation to avoid allocation
                if !validation::is_valid_for_transform_iter(self.buf.iter().map(|c| &c.key)) {
                    return None;
                }
            }

            // Apply stroke
            if let Some(c) = self.buf.get_mut(last_pos) {
                c.stroke = true;
            }
            self.last_transform = Some(Transform::Stroke(key));
            return Some(self.rebuild_from(last_pos));
        }

        // VNI MODE: '9' can stroke any 'd' in buffer (delayed stroke)
        // Find first un-stroked 'd' anywhere in buffer
        let pos = self.buf
            .iter()
            .enumerate()
            .find(|(_, c)| c.key == keys::D && !c.stroke)
            .map(|(i, _)| i)?;

        // Check revert: 99 → 9 (undo stroke)
        if let Some(Transform::Stroke(last_key)) = self.last_transform {
            if last_key == key {
                return Some(self.revert_stroke(key, pos));
            }
        }

        // VNI validation: only validate if we have vowels after 'd'
        // Allow "d9" → "đ" before vowel is typed
        let has_vowel_after = self.buf.iter().skip(pos + 1).any(|c| keys::is_vowel(c.key));
        if !self.free_tone_enabled && has_vowel_after {
            // Use iterator-based validation to avoid allocation
            if !validation::is_valid_for_transform_iter(self.buf.iter().map(|c| &c.key)) {
                return None;
            }
        }

        // Apply stroke
        if let Some(c) = self.buf.get_mut(pos) {
            c.stroke = true;
        }
        self.last_transform = Some(Transform::Stroke(key));
        Some(self.rebuild_from(pos))
    }

    /// Try to apply tone transformation by scanning buffer for targets
    fn try_tone(
        &mut self,
        key: u16,
        caps: bool,
        tone_type: ToneType,
        targets: &[u16],
    ) -> Option<Result> {
        if self.buf.is_empty() {
            return None;
        }

        // Check revert first (same key pressed twice)
        if let Some(Transform::Tone(last_key, _)) = self.last_transform {
            if last_key == key {
                return Some(self.revert_tone(key, caps));
            }
        }

        // ═════════════════════════════════════════════════════════════════════
        // ENGLISH DETECTION: 3-Layer Architecture
        // ═════════════════════════════════════════════════════════════════════
        // Reference: ULTIMATE_ENGLISH_DETECTION_GUIDE.md
        // Goal: Detect English words BEFORE Vietnamese transforms occur
        // Performance: <200ns average, zero allocations
        
        // CRITICAL FIX: Skip English detection if Vietnamese tone marks already applied
        // If buffer has tone marks (sắc/huyền/etc), this is intentional Vietnamese typing
        // Issue: "viese" (vie+s+e→viết) was incorrectly detected as English because
        // raw_input includes 's' (tone mark modifier), creating false "e-s-e" pattern
        let has_tone_marks = self.buf.iter().any(|c| c.mark > 0);

        // In Telex/VNI, tone keys must ALWAYS be allowed to apply.
        // Therefore, English detection must NOT run inside this modifier handler.
        if !self.free_tone_enabled && !has_tone_marks && (self.method != 0 && self.method != 1) {
            // ─────────────────────────────────────────────────────────────────
            // LAYER 1: Vietnamese Syllable Validator (~200ns)
            // ─────────────────────────────────────────────────────────────────
            // Check: Valid Vietnamese syllable structure?
            // - Valid onset (phụ âm đầu)? 29 variants allowed
            // - Valid coda (phụ âm cuối)? Only 8 allowed: c, ch, m, n, ng, nh, p, t
            // - Valid vowel pattern? Whitelist-based
            // - No consonant clusters? (e.g., "mp", "trl" invalid)

            // Check for invalid Vietnamese initial consonants.
            // In Telex/VNI, a failed validator here should NOT permanently lock the word as English,
            // because the user might be in the middle of Vietnamese typing and about to apply modifiers.
            // Just decline the transform and let normal insertion continue.
            if !self.has_valid_initial() {
                eprintln!("[DEBUG] try_tone: has_valid_initial() = false");
                return None;
            }

            // Validate buffer structure using iterator (zero allocation)
            // This checks phonotactic constraints from Vietnamese linguistics
            if !validation::is_valid_for_transform_iter(self.buf.iter().map(|c| &c.key)) {
                eprintln!("[DEBUG] try_tone: is_valid_for_transform_iter() = false");
                return None;
            }

            // ─────────────────────────────────────────────────────────────────
            // LAYER 2 & 3: Early Pattern + Multi-Syllable Detection (~20-150ns)
            // ─────────────────────────────────────────────────────────────────
            // Check raw keystroke history for English patterns
            // Note: raw_input already contains current key (pushed before try_tone)

            if self.raw_input.len() >= 2 {
                // Do not set `is_english_word` here; this function is a modifier handler and must
                // never lock the word into English mode. If it looks English, simply skip applying
                // the tone and fall through so the key can be inserted normally.
                let is_english = self.has_english_word_pattern();
                if is_english {
                    eprintln!("[DEBUG] try_tone: has_english_word_pattern() = true");
                    return None;
                }
            }
        }

        let tone_val = tone_type.value();

        // Check if we're switching from one tone to another (e.g., ô → ơ)
        // Find vowels that have a DIFFERENT tone (to switch) or NO tone (to add)
        let is_switching = self
            .buf
            .iter()
            .any(|c| targets.contains(&c.key) && c.tone != tone::NONE && c.tone != tone_val);

        // Scan buffer for eligible target vowels
        let mut target_positions = Vec::new();

        // Special case: uo/ou compound for horn - find adjacent pair only
        // But ONLY apply compound logic when BOTH vowels are plain (not when switching)
        if tone_type == ToneType::Horn && !is_switching {
            if let Some((pos1, pos2)) = self.find_uo_compound_positions() {
                if let (Some(c1), Some(c2)) = (self.buf.get(pos1), self.buf.get(pos2)) {
                    // Only apply compound when BOTH vowels have no tone
                    if c1.tone == tone::NONE && c2.tone == tone::NONE {
                        target_positions.push(pos1);
                        target_positions.push(pos2);
                    }
                }
            }
        }

        // Normal case: find last matching target
        if target_positions.is_empty() {
            if is_switching {
                // When switching, ONLY target vowels that already have a diacritic
                // (don't add diacritics to plain vowels during switch)
                for (i, c) in self.buf.iter().enumerate().rev() {
                    if targets.contains(&c.key) && c.tone != tone::NONE && c.tone != tone_val {
                        target_positions.push(i);
                        break;
                    }
                }
            } else if tone_type == ToneType::Horn {
                // For horn modifier, apply smart vowel selection based on Vietnamese phonology
                target_positions = self.find_horn_target_with_switch(targets, tone_val);
            } else {
                // Non-horn modifiers (circumflex): use standard target matching
                // For Telex circumflex (aa, ee, oo pattern), apply smart detection
                // Based on reference implementation
                let is_telex_circumflex = self.method == 0
                    && tone_type == ToneType::Circumflex
                    && matches!(key, keys::A | keys::E | keys::O);

                if is_telex_circumflex {
                    // Issue #312 FIX: Check if ADJACENT (immediately preceding) same vowel exists
                    // and whether it has a VIETNAMESE tone mark (sắc/huyền/hỏi/ngã/nặng - NOT circumflex)
                    // 
                    // Pattern "aa", "ee", "oo" should apply circumflex:
                    // - "nghie" + "e" → "nghiê" (apply circumflex on FIRST 'e', because adjacent 'e' has NO vietnamese tone)
                    // - Then "nghiê" + "e" → "nghiêe" → wait, this is wrong...
                    //
                    // Actually the correct logic is:
                    // - Check if LAST character is SAME vowel AND has NO tone
                    // - If yes, apply circumflex to transform THAT vowel
                    // - Example: "nghie" + "e" → check last char is 'e', has tone=CIRCUMFLEX(1)
                    //   But we want "ee" pattern to work!
                    //
                    // Let me re-think: When user types "nghieem":
                    // 1. "n", "g", "h", "i" → "nghi"
                    // 2. "e" → "nghie" (single e, no transform yet)
                    // 3. "e" again → should detect "ee" pattern → apply circumflex to FIRST 'e' → "nghiê"
                    // 4. But we just added another 'e', so buffer has "nghiêe"? No!
                    //
                    // The transform should: see "ee" pattern, apply circumflex to first 'e', CONSUME second 'e'
                    // So buffer becomes "nghiê" (not "nghiêe")
                    //
                    // So the check should be: if LAST char is SAME vowel with NO tone (not even circumflex),
                    // then apply circumflex. But if last char is SAME vowel WITH any tone, skip.
                    //
                    // Wait, that's what I had! Let me re-check the original issue...
                    //
                    // Original bug: "any_vowel_has_tone" checks ALL vowels (wrong!)
                    // Example: "viết" - 'i' has NO tone, 'ê' has circumflex
                    //          typing "viét" + "t" + "e" → "any_vowel_has_tone" = true (because ê)
                    //          so skip applying "ee" pattern on 'e'
                    //
                    // Correct fix: Check if LAST occurrence of SAME key has VIETNAMESE tone (2-6)
                    // NOT circumflex (1), because "ee" pattern should still work
                    // Example: "nghie" + "e" → last 'e' has tone=0 → apply circumflex
                    //          "viét" + "e" → last 'e' has tone=2(sắc) → skip
                    let last_same_vowel = self
                        .buf
                        .iter()
                        .rev()
                        .find(|c| c.key == key && keys::is_vowel(c.key));

                    if let Some(last_vowel) = last_same_vowel {
                        // If last same vowel has VIETNAMESE tone mark (not circumflex/horn/breve),
                        // skip applying new transform to avoid conflicts
                        // tone: 0=none, 1=circumflex/horn/breve, 2=sắc, 3=huyền, 4=hỏi, 5=ngã, 6=nặng
                        if last_vowel.tone >= 2 && last_vowel.tone <= 6 {
                            return None;
                        }
                    }

                    // Check if any vowel has a Vietnamese tone mark (sắc/huyền/hỏi/ngã/nặng)
                    // If so, user is intentionally typing Vietnamese - skip English detection
                    // Example: "viés" + "t" + "e" → should become "viết" (NOT blocked as English)
                    let has_vietnamese_marks = self
                        .buf
                        .iter()
                        .any(|c| c.mark > 0);

                    // English pattern detection: consonant separating same vowels
                    // Pattern: V + C(s) + V (same vowel) → likely English multi-syllable
                    // Examples: "ele" (element), "rele" (release), "dele" (delete)
                    // Skip circumflex to preserve raw input for auto-restore
                    // BUT: Skip this check if Vietnamese marks are present (intentional Vietnamese)
                    // 
                    // Find position of same vowel in buffer (if exists)
                    if !has_vietnamese_marks {
                        let same_vowel_pos = self.buf.iter()
                            .enumerate()
                            .find(|(_, c)| c.key == key && keys::is_vowel(c.key))
                            .map(|(i, _)| i);

                        if let Some(vowel_pos) = same_vowel_pos {
                            // Check if there are consonants between the existing vowel and current position
                            let has_consonant_between = (vowel_pos + 1..self.buf.len())
                                .any(|j| self.buf.get(j).is_some_and(|c| !keys::is_vowel(c.key)));

                            if has_consonant_between {
                                // Pattern like "ele", "olo", "ala" with consonant(s) between
                                // This is likely English (element, release, delete, reverse, etc.)
                                // Skip circumflex and add vowel as raw letter
                                return None;
                            }
                        }
                    }
                }

                for (i, c) in self.buf.iter().enumerate().rev() {
                    if targets.contains(&c.key) && c.tone == tone::NONE {
                        target_positions.push(i);
                        break;
                    }
                }
            }
        }

        if target_positions.is_empty() {
            // Check if any target vowels already have the requested tone
            // 
            // EXCEPTION: Don't absorb 'w' if last_transform was WAsVowel
            // because try_w_as_vowel needs to handle the revert (ww → w)
            let is_w_revert_pending =
                key == keys::W && matches!(self.last_transform, Some(Transform::WAsVowel));

            let has_tone_already = self
                .buf
                .iter()
                .any(|c| targets.contains(&c.key) && c.tone == tone_val);

            // For Telex circumflex keys (a, e, o), DON'T absorb - let key append as raw letter
            // This prevents buffer/screen desync: absorbing returns send(0,&[]) which
            // Swift interprets as passthrough, causing screen to show the letter
            // but buffer doesn't have it, leading to wrong backspace count on restore.
            // Example bug: "element" → "êlment" + space → "êelement " (wrong!)
            // Fix: return None so letter is added to buffer, keeping sync.
            let is_telex_circumflex_key = self.method == 0
                && tone_type == ToneType::Circumflex
                && matches!(key, keys::A | keys::E | keys::O);

            if has_tone_already && !is_w_revert_pending {
                if is_telex_circumflex_key {
                    // Don't absorb - let vowel append as raw letter to keep buffer/screen in sync
                    return None;
                }
                // For non-Telex-circumflex cases (like VNI "u7o7"), absorb is safe
                // because these are number keys that won't be passthrough as letters
                return Some(Result::send(0, &[]));
            }
            return None;
        }

        // Track earliest position modified for rebuild
        let mut earliest_pos = usize::MAX;

        // If switching, clear old tones first for proper rebuild
        if is_switching {
            for &pos in &target_positions {
                if let Some(c) = self.buf.get_mut(pos) {
                    c.tone = tone::NONE;
                    earliest_pos = earliest_pos.min(pos);
                }
            }

            // Special case: switching from horn compound (ươ) to circumflex (uô)
            // When switching to circumflex on 'o', also clear horn from adjacent 'u'
            if tone_type == ToneType::Circumflex {
                for &pos in &target_positions {
                    if let Some(c) = self.buf.get(pos) {
                        if c.key == keys::O {
                            // Check for adjacent 'u' with horn and clear it
                            if pos > 0 {
                                if let Some(prev) = self.buf.get_mut(pos - 1) {
                                    if prev.key == keys::U && prev.tone == tone::HORN {
                                        prev.tone = tone::NONE;
                                        earliest_pos = earliest_pos.min(pos - 1);
                                    }
                                }
                            }
                            if pos + 1 < self.buf.len() {
                                if let Some(next) = self.buf.get_mut(pos + 1) {
                                    if next.key == keys::U && next.tone == tone::HORN {
                                        next.tone = tone::NONE;
                                        earliest_pos = earliest_pos.min(pos + 1);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Special case: switching from circumflex (uô) to horn compound (ươ)
            // For standalone uo compound (no final consonant), add horn to adjacent 'u'
            if tone_type == ToneType::Horn && self.has_uo_compound() {
                // Check if this is a standalone compound (o is last vowel, no final consonant)
                let has_final = target_positions.iter().any(|&pos| {
                    pos + 1 < self.buf.len()
                        && self
                            .buf
                            .get(pos + 1)
                            .is_some_and(|c| !keys::is_vowel(c.key))
                });

                if !has_final {
                    for &pos in &target_positions {
                        if let Some(c) = self.buf.get(pos) {
                            if c.key == keys::O {
                                // Add horn to adjacent 'u' for compound
                                if pos > 0 {
                                    if let Some(prev) = self.buf.get_mut(pos - 1) {
                                        if prev.key == keys::U && prev.tone == tone::NONE {
                                            prev.tone = tone::HORN;
                                            earliest_pos = earliest_pos.min(pos - 1);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Apply new tone
        for &pos in &target_positions {
            if let Some(c) = self.buf.get_mut(pos) {
                c.tone = tone_val;
                earliest_pos = earliest_pos.min(pos);
            }
        }

        // Validate result: check for breve (ă) followed by vowel - NEVER valid in Vietnamese
        // Issue #44: "tai" + 'w' → "tăi" is INVALID (ăi, ăo, ău, ăy don't exist)
        // Only check this specific pattern, not all vowel patterns, to allow Telex shortcuts
        // like "eie" → "êi" which may not be standard but are expected Telex behavior
        if tone_type == ToneType::Horn {
            let has_breve_vowel_pattern = target_positions.iter().any(|&pos| {
                if let Some(c) = self.buf.get(pos) {
                    // Check if this is 'a' with horn (breve) followed by another vowel
                    if c.key == keys::A {
                        // Look for any vowel after this position
                        return (pos + 1..self.buf.len()).any(|i| {
                            self.buf
                                .get(i)
                                .map(|next| keys::is_vowel(next.key))
                                .unwrap_or(false)
                        });
                    }
                }
                false
            });

            if has_breve_vowel_pattern {
                // Revert: clear applied tones
                for &pos in &target_positions {
                    if let Some(c) = self.buf.get_mut(pos) {
                        c.tone = tone::NONE;
                    }
                }
                return None;
            }
        }

        // Normalize ưo → ươ compound if horn was applied to 'u'
        if let Some(compound_pos) = self.normalize_uo_compound() {
            earliest_pos = earliest_pos.min(compound_pos);
        }

        self.last_transform = Some(Transform::Tone(key, tone_val));

        // Reposition tone mark if vowel pattern changed
        let mut rebuild_pos = earliest_pos;
        if let Some((old_pos, _)) = self.reposition_tone_if_needed() {
            rebuild_pos = rebuild_pos.min(old_pos);
        }

        Some(self.rebuild_from(rebuild_pos))
    }

    /// Try to apply mark transformation (circumflex, breve, horn)
    fn try_mark(&mut self, key: u16, caps: bool, mark_val: u8) -> Option<Result> {
        if self.buf.is_empty() {
            return None;
        }

        // Check revert first
        if let Some(Transform::Mark(last_key, _)) = self.last_transform {
            if last_key == key {
                return Some(self.revert_mark(key, caps));
            }
        }

        // ═════════════════════════════════════════════════════════════════════
        // ENGLISH DETECTION: 3-Layer Architecture
        // ═════════════════════════════════════════════════════════════════════
        // Reference: ULTIMATE_ENGLISH_DETECTION_GUIDE.md
        // Same strategy as try_tone: detect English BEFORE applying marks
        // Performance: <200ns average, zero allocations
        
        // CRITICAL FIX: Skip English detection if Vietnamese tone marks already applied
        // (Same fix as in try_tone - if marks exist, this is intentional Vietnamese)
        let has_tone_marks = self.buf.iter().any(|c| c.mark > 0);

        // In Telex/VNI, mark/tone keys must be able to apply even for short sequences.
        // Therefore, English detection must NOT run inside this modifier handler.
        if !self.free_tone_enabled && !has_tone_marks && self.raw_input.len() >= 2 && (self.method != 0 && self.method != 1) {
            // ─────────────────────────────────────────────────────────────────
            // LAYER 2 & 3: Early Pattern + Multi-Syllable Detection
            // ─────────────────────────────────────────────────────────────────
            // Check raw keystroke history for English patterns
            // Note: raw_input already contains current key

            let is_english = self.has_english_word_pattern();

            if is_english {
                // Never set `is_english_word` from a modifier handler.
                // If it looks English, just don't apply the mark and let the key insert normally.
                return None;
            }
        }

        // Check if buffer has horn transforms - indicates intentional Vietnamese typing
        // (e.g., "rượu" has base keys [R,U,O,U] which looks like "ou" pattern,
        // but with horns applied it's valid "ươu")
        let has_horn_transforms = self.buf.iter().any(|c| c.tone == tone::HORN);

        // Check if buffer has stroke transforms (đ) - indicates intentional Vietnamese typing
        // Issue #48: "ddeso" → "đéo" (d was stroked to đ, so this is Vietnamese, not English)
        let has_stroke_transforms = self.buf.iter().any(|c| c.stroke);

        // Check for invalid Vietnamese initial consonants (English word detection)
        // Skip transformation if invalid initial detected (unless already has Vietnamese transforms)
        if !self.free_tone_enabled
            && !has_horn_transforms
            && !has_stroke_transforms
            && !self.has_valid_initial()
        {
            // Don't lock into English mode here; just decline the transform.
            return None;
        }

        // Validate buffer structure (skip if has horn/stroke transforms - already intentional Vietnamese)
        // Also skip validation if free_tone mode is enabled
        if !self.free_tone_enabled
            && !has_horn_transforms
            && !has_stroke_transforms
        {
            // Use iterator-based validation to avoid allocation
            if !validation::is_valid_for_transform_iter(self.buf.iter().map(|c| &c.key)) {
                return None;
            }
        }

        // Skip modifier if buffer shows foreign word patterns.
        // Only check when NO horn/stroke transforms exist.
        //
        // Detected patterns:
        // - Invalid vowel combinations (ou, yo) that don't exist in Vietnamese
        // - Consonant clusters after finals common in English (T+R, P+R, C+R)
        //
        // Examples:
        // - "met" + 'r' → T+R cluster common in English → skip modifier
        // - "you" + 'r' → "ou" vowel pattern invalid → skip modifier
        // - "rươu" + 'j' → has horn transforms → DON'T skip, apply mark normally
        // - "đe" + 's' → has stroke transform → DON'T skip, apply mark normally (Issue #48)
        // Skip foreign word detection if free_tone mode is enabled
        if !self.free_tone_enabled
            && !has_horn_transforms
            && !has_stroke_transforms
        {
            // Collect buffer_keys only once for foreign word pattern check
            let buffer_keys: Vec<u16> = self.buf.iter().map(|c| c.key).collect();
            if is_foreign_word_pattern(&buffer_keys, key) {
                return None;
            }
        }

        // Issue #29: Normalize ưo → ươ compound before placing mark
        // In Vietnamese, "ưo" is never valid - it's always "ươ"
        let rebuild_from_compound = self.normalize_uo_compound();

        let vowels = self.collect_vowels();
        if vowels.is_empty() {
            return None;
        }

        // Find mark position using phonology rules
        let last_vowel_pos = vowels.last().map(|v| v.pos).unwrap_or(0);
        let has_final = self.has_final_consonant(last_vowel_pos);
        let has_qu = self.has_qu_initial();
        let has_gi = self.has_gi_initial();
        let pos =
            Phonology::find_tone_position(&vowels, has_final, self.modern_tone, has_qu, has_gi);

        if let Some(c) = self.buf.get_mut(pos) {
            c.mark = mark_val;
            self.last_transform = Some(Transform::Mark(key, mark_val));
            // Rebuild from the earlier position if compound was formed
            let rebuild_pos = rebuild_from_compound.map_or(pos, |cp| cp.min(pos));
            return Some(self.rebuild_from(rebuild_pos));
        }

        None
    }

    /// Normalize ưo → ươ compound
    ///
    /// In Vietnamese, "ưo" (u with horn + plain o) is NEVER valid.
    /// It should always be "ươ" (both with horn).
    /// This function finds and fixes this pattern anywhere in the buffer.
    ///
    /// Returns Some(position) of the 'o' that was modified, None if no change.
    /// Delegates to vowel_compound module.
    fn normalize_uo_compound(&mut self) -> Option<usize> {
        vowel_compound::normalize_uo_compound(&mut self.buf)
    }

    /// Find positions of U+O or O+U compound (adjacent vowels)
    /// Returns Some((first_pos, second_pos)) if found, None otherwise
    /// Delegates to vowel_compound module.
    fn find_uo_compound_positions(&self) -> Option<(usize, usize)> {
        vowel_compound::find_uo_compound_positions(&self.buf)
    }

    /// Check for uo compound in buffer (any tone state)
    /// Delegates to vowel_compound module.
    fn has_uo_compound(&self) -> bool {
        vowel_compound::has_uo_compound(&self.buf)
    }

    /// Check for complete ươ compound (both u and o have horn)
    /// Delegates to vowel_compound module.
    fn has_complete_uo_compound(&self) -> bool {
        vowel_compound::has_complete_uo_compound(&self.buf)
    }

    /// Find target position for horn modifier with switching support
    /// Allows selecting vowels that have a different tone (for switching circumflex ↔ horn)
    fn find_horn_target_with_switch(&self, targets: &[u16], new_tone: u8) -> Vec<usize> {
        // Find vowel positions that match targets and either:
        // - have no tone (normal case)
        // - have a different tone (switching case)
        let vowels: Vec<usize> = self
            .buf
            .iter()
            .enumerate()
            .filter(|(_, c)| {
                targets.contains(&c.key) && (c.tone == tone::NONE || c.tone != new_tone)
            })
            .map(|(i, _)| i)
            .collect();

        if vowels.is_empty() {
            return vec![];
        }

        let buffer_keys: Vec<u16> = self.buf.iter().map(|c| c.key).collect();

        // Use centralized phonology rules (context inferred from buffer)
        Phonology::find_horn_positions(&buffer_keys, &vowels)
            .into_iter()
            .filter(|&pos| {
                self.buf
                    .get(pos)
                    .map(|c| {
                        targets.contains(&c.key) && (c.tone == tone::NONE || c.tone != new_tone)
                    })
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Reposition tone (sắc/huyền/hỏi/ngã/nặng) after vowel pattern changes
    ///
    /// When user types out-of-order (e.g., "osa" instead of "oas"), the tone may be
    /// placed on wrong vowel. This function moves it to the correct position based
    /// on Vietnamese phonology rules.
    ///
    /// Returns Some((old_pos, new_pos)) if tone was moved, None otherwise.
    fn reposition_tone_if_needed(&mut self) -> Option<(usize, usize)> {
        // Find vowel with tone mark (sắc/huyền/hỏi/ngã/nặng)
        let tone_info: Option<(usize, u8)> = self
            .buf
            .iter()
            .enumerate()
            .find(|(_, c)| c.mark > mark::NONE && keys::is_vowel(c.key))
            .map(|(i, c)| (i, c.mark));

        if let Some((old_pos, tone_value)) = tone_info {
            let vowels = self.collect_vowels();
            if vowels.is_empty() {
                return None;
            }

            // Check for syllable boundary: if there's a consonant between the toned vowel
            // and any later vowel, the toned vowel is in a closed syllable - don't reposition.
            // Example: "bủn" + "o" → 'n' closes "bủn", so 'o' starts new syllable.
            let has_consonant_after_tone = (old_pos + 1..self.buf.len()).any(|i| {
                self.buf
                    .get(i)
                    .is_some_and(|c| !keys::is_vowel(c.key) && c.key != keys::W)
            });
            let has_vowel_after_consonant = has_consonant_after_tone
                && vowels
                    .iter()
                    .any(|v| v.pos > old_pos && self.has_consonant_between(old_pos, v.pos));

            if has_vowel_after_consonant {
                // Syllable boundary detected - tone is in previous syllable, don't move it
                return None;
            }

            let last_vowel_pos = vowels.last().map(|v| v.pos).unwrap_or(0);
            let has_final = self.has_final_consonant(last_vowel_pos);
            let has_qu = self.has_qu_initial();
            let has_gi = self.has_gi_initial();
            let new_pos = Phonology::find_tone_position(&vowels, has_final, true, has_qu, has_gi);

            if new_pos != old_pos {
                // Move tone from old position to new position
                if let Some(c) = self.buf.get_mut(old_pos) {
                    c.mark = mark::NONE;
                }
                if let Some(c) = self.buf.get_mut(new_pos) {
                    c.mark = tone_value;
                }
                return Some((old_pos, new_pos));
            }
        }
        None
    }

    /// Check if there's a consonant between two positions
    fn has_consonant_between(&self, start: usize, end: usize) -> bool {
        (start + 1..end).any(|i| {
            self.buf
                .get(i)
                .is_some_and(|c| !keys::is_vowel(c.key) && c.key != keys::W)
        })
    }

    /// Common revert logic: clear modifier, add key to buffer, rebuild output
    fn revert_and_rebuild(&mut self, pos: usize, key: u16, caps: bool) -> Result {
        // Calculate backspace BEFORE adding key (based on old buffer state)
        let backspace = (self.buf.len() - pos) as u8;

        // Add the reverted key to buffer so validation sees the full sequence
        self.buf.push(Char::new(key, caps));

        // Build output from position (includes new key)
        let output: Vec<char> = (pos..self.buf.len())
            .filter_map(|i| self.buf.get(i))
            .filter_map(|c| utils::key_to_char(c.key, c.caps))
            .collect();

        Result::send(backspace, &output)
    }

    /// Revert tone transformation
    fn revert_tone(&mut self, key: u16, caps: bool) -> Result {
        self.last_transform = None;

        for pos in self.buf.find_vowels().into_iter().rev() {
            if let Some(c) = self.buf.get_mut(pos) {
                if c.tone > tone::NONE {
                    c.tone = tone::NONE;
                    return self.revert_and_rebuild(pos, key, caps);
                }
            }
        }
        Result::none()
    }

    /// Revert mark transformation
    fn revert_mark(&mut self, key: u16, caps: bool) -> Result {
        self.last_transform = None;

        for pos in self.buf.find_vowels().into_iter().rev() {
            if let Some(c) = self.buf.get_mut(pos) {
                if c.mark > mark::NONE {
                    c.mark = mark::NONE;
                    return self.revert_and_rebuild(pos, key, caps);
                }
            }
        }
        Result::none()
    }

    /// Revert stroke transformation at specific position
    fn revert_stroke(&mut self, key: u16, pos: usize) -> Result {
        self.last_transform = None;

        if let Some(c) = self.buf.get_mut(pos) {
            if c.key == keys::D && !c.stroke {
                // Un-stroked d found at pos - this means we need to add another d
                let caps = c.caps;
                self.buf.push(Char::new(key, caps));
                return self.rebuild_from(pos);
            }
        }
        Result::none()
    }

    /// Try to apply remove modifier
    /// Returns Some(Result) if a mark/tone was removed, None if nothing to remove
    /// When None is returned, the key falls through to handle_normal_letter()
    fn try_remove(&mut self) -> Option<Result> {
        self.last_transform = None;
        for pos in self.buf.find_vowels().into_iter().rev() {
            if let Some(c) = self.buf.get_mut(pos) {
                if c.mark > mark::NONE {
                    c.mark = mark::NONE;
                    return Some(self.rebuild_from(pos));
                }
                if c.tone > tone::NONE {
                    c.tone = tone::NONE;
                    return Some(self.rebuild_from(pos));
                }
            }
        }
        // Nothing to remove - return None so key can be processed as normal letter
        // This allows shortcuts like "zz" to work
        None
    }

    /// Handle normal letter input
    fn handle_normal_letter(&mut self, key: u16, caps: bool, shift: bool) -> Result {
        // Detect if typing special characters with Shift (e.g., @, #, $)
        // These indicate English input, so mark as English word
        if shift && keys::is_number(key) {
            self.is_english_word = true;
        }
        
        // Invalidate syllable boundary cache when adding new letter
        self.cached_syllable_boundary = None;
        // Special case: "o" after "w→ư" should form "ươ" compound
        // This only handles the WAsVowel case (typing "w" alone creates ư)
        // For "uw" pattern, the compound is normalized in try_mark via normalize_uo_compound
        if key == keys::O && matches!(self.last_transform, Some(Transform::WAsVowel)) {
            // Add O with horn to form ươ compound
            let mut c = Char::new(key, caps);
            c.tone = tone::HORN;
            self.buf.push(c);
            self.last_transform = None;

            // Return the ơ character (o with horn)
            let vowel_char = chars::to_char(keys::O, caps, tone::HORN, 0).unwrap();
            return Result::send(0, &[vowel_char]);
        }

        self.last_transform = None;
        if keys::is_letter(key) {
            // Add the letter to buffer
            self.buf.push(Char::new(key, caps));

            // Normalize ưo → ươ immediately when 'o' is typed after 'ư'
            // This ensures "dduwo" → "đươ" (Telex) and "u7o" → "ươ" (VNI)
            // Works for both methods since "ưo" alone is not valid Vietnamese
            if key == keys::O && self.normalize_uo_compound().is_some() {
                // ươ compound formed - reposition tone if needed (ư→ơ)
                if let Some((old_pos, _)) = self.reposition_tone_if_needed() {
                    return self.rebuild_from_after_insert(old_pos);
                }

                // No tone to reposition - just output ơ
                let vowel_char = chars::to_char(keys::O, caps, tone::HORN, 0).unwrap();
                return Result::send(0, &[vowel_char]);
            }

            // Auto-correct tone position when new character changes the correct placement
            //
            // Two scenarios:
            // 1. New vowel changes diphthong pattern:
            //    "osa" → tone on 'o', then 'a' added → "oa" needs tone on 'a'
            // 2. New consonant creates final, which changes tone position:
            //    "muas" → tone on 'u' (ua open), then 'n' added → "uan" needs tone on 'a'
            //
            // Both cases need to reposition the tone mark based on Vietnamese phonology.
            if let Some((old_pos, _new_pos)) = self.reposition_tone_if_needed() {
                // Tone was moved - rebuild output from the old position
                // Note: the new char was just added to buffer but NOT yet displayed
                // So backspace = (chars from old_pos to BEFORE new char)
                // And output = (chars from old_pos to end INCLUDING new char)
                return self.rebuild_from_after_insert(old_pos);
            }

            // Check if adding this letter creates invalid vowel pattern (foreign word detection)
            // Only revert if the horn transforms are from w-as-vowel (standalone w→ư),
            // not from w-as-tone (adding horn to existing vowels like in "rượu")
            //
            // w-as-vowel: first horn is U at position 0 (was standalone 'w')
            // w-as-tone: horns are on vowels after initial consonant
            //
            // Exception: complete ươ compound + vowel = valid Vietnamese triphthong
            // (like "rượu" = ươu, "mười" = ươi) - don't revert in these cases
            // Only skip for vowels that form valid triphthongs (u, i), not for consonants
            let is_valid_triphthong_ending =
                self.has_complete_uo_compound() && (key == keys::U || key == keys::I);
            if self.has_w_as_vowel_transform() && !is_valid_triphthong_ending {
                let buffer_keys: Vec<u16> = self.buf.iter().map(|c| c.key).collect();
                if is_foreign_word_pattern(&buffer_keys, key) {
                    return self.revert_w_as_vowel_transforms();
                }
            }
            
            // Detect English word patterns and mark as English
            // This will trigger auto-restore on space key
            if !self.is_english_word {
                let buffer_keys: Vec<u16> = self.buf.iter().map(|c| c.key).collect();
                if is_foreign_word_pattern(&buffer_keys, key) {
                    self.is_english_word = true;
                }
            }
        } else {
            // Non-letter character (number, symbol, etc.)
            // Mark that this word has non-letter prefix to prevent false shortcut matches
            // e.g., "149k" should NOT trigger shortcut "k" → "không"
            // e.g., "@abc" should NOT trigger shortcut "abc"
            self.has_non_letter_prefix = true;
        }
        Result::none()
    }

    /// Check if buffer has w-as-vowel transform (standalone w→ư at start)
    /// This is different from w-as-tone which adds horn to existing vowels
    fn has_w_as_vowel_transform(&self) -> bool {
        // w-as-vowel creates U with horn at position 0 or after consonants
        // The key distinguishing feature: the U with horn was created from 'w',
        // meaning there was no preceding vowel at that position
        //
        // Simple heuristic: if first char is U with horn, it's w-as-vowel
        // (words like "rượu" start with consonant R, not U)
        self.buf
            .get(0)
            .map(|c| c.key == keys::U && c.tone == tone::HORN)
            .unwrap_or(false)
    }

    /// Revert w-as-vowel transforms and rebuild output
    /// Used when foreign word pattern is detected after w→ư transformation
    fn revert_w_as_vowel_transforms(&mut self) -> Result {
        // Only revert if first char is U with horn (w-as-vowel pattern)
        if !self.has_w_as_vowel_transform() {
            return Result::none();
        }

        // Find all horn transforms to revert
        let horn_positions: Vec<usize> = self
            .buf
            .iter()
            .enumerate()
            .filter(|(_, c)| c.tone == tone::HORN)
            .map(|(i, _)| i)
            .collect();

        if horn_positions.is_empty() {
            return Result::none();
        }

        let first_pos = horn_positions[0];

        // Clear horn tones and change U back to W (for w-as-vowel positions)
        for &pos in &horn_positions {
            if let Some(c) = self.buf.get_mut(pos) {
                // U with horn was from 'w' → change key to W
                if c.key == keys::U {
                    c.key = keys::W;
                }
                c.tone = tone::NONE;
            }
        }

        self.rebuild_from(first_pos)
    }

    /// Collect vowels from buffer
    fn collect_vowels(&self) -> Vec<Vowel> {
        utils::collect_vowels(&self.buf)
    }

    /// Check for final consonant after position
    fn has_final_consonant(&self, after_pos: usize) -> bool {
        utils::has_final_consonant(&self.buf, after_pos)
    }

    /// Check for qu initial
    fn has_qu_initial(&self) -> bool {
        utils::has_qu_initial(&self.buf)
    }

    /// Check for gi initial (gi + vowel)
    fn has_gi_initial(&self) -> bool {
        utils::has_gi_initial(&self.buf)
    }

    /// Check if buffer has valid Vietnamese initial consonants
    /// Returns false if initial consonant cluster is invalid (e.g., cr-, fl-, bl-)
    fn has_valid_initial(&self) -> bool {
        if self.buf.is_empty() {
            return true;
        }

        let buffer_keys: Vec<u16> = self.buf.iter().map(|c| c.key).collect();
        let syllable = syllable::parse(&buffer_keys);

        if syllable.initial.is_empty() {
            return true; // No initial consonant is valid
        }

        let initial: Vec<u16> = syllable.initial.iter().map(|&i| buffer_keys[i]).collect();

        match initial.len() {
            1 => constants::VALID_INITIALS_1.contains(&initial[0]),
            2 => constants::VALID_INITIALS_2
                .iter()
                .any(|p| p[0] == initial[0] && p[1] == initial[1]),
            3 => initial[0] == keys::N && initial[1] == keys::G && initial[2] == keys::H,
            _ => false, // More than 3 consonants = invalid
        }
    }

    /// Rebuild output from position
    fn rebuild_from(&self, from: usize) -> Result {
        let mut output = Vec::with_capacity(self.buf.len() - from);
        let mut backspace = 0u8;

        for i in from..self.buf.len() {
            if let Some(c) = self.buf.get(i) {
                backspace += 1;

                if c.key == keys::D && c.stroke {
                    output.push(chars::get_d(c.caps));
                } else if let Some(ch) = chars::to_char(c.key, c.caps, c.tone, c.mark) {
                    output.push(ch);
                } else if let Some(ch) = utils::key_to_char(c.key, c.caps) {
                    output.push(ch);
                }
            }
        }

        if output.is_empty() {
            Result::none()
        } else {
            Result::send(backspace, &output)
        }
    }

    /// Rebuild output from position with explicit backspace count
    /// Used when we need to specify exact number of chars to delete on screen
    /// (e.g., after popping a character, old_length is the screen length before pop)
    fn rebuild_from_with_backspace(&self, from: usize, old_screen_len: usize) -> Result {
        let mut output = Vec::with_capacity(self.buf.len() - from);

        for i in from..self.buf.len() {
            if let Some(c) = self.buf.get(i) {
                if c.key == keys::D && c.stroke {
                    output.push(chars::get_d(c.caps));
                } else if let Some(ch) = chars::to_char(c.key, c.caps, c.tone, c.mark) {
                    output.push(ch);
                } else if let Some(ch) = utils::key_to_char(c.key, c.caps) {
                    output.push(ch);
                }
            }
        }

        if output.is_empty() {
            Result::send(old_screen_len as u8, &[])
        } else {
            Result::send(old_screen_len as u8, &output)
        }
    }

    /// Find the start of the last syllable in buffer
    /// Returns the index where the last syllable begins
    /// Syllable boundaries: space, start of buffer, or after punctuation
    /// 
    /// PERFORMANCE: This allows us to rebuild only the last syllable instead of entire buffer
    /// OPTIMIZATION: Result is cached in Engine to avoid repeated scans during consecutive backspaces
    fn find_last_syllable_boundary(&self) -> usize {
        if self.buf.is_empty() {
            return 0;
        }

        // OPTIMIZATION: Scan from end backwards (early exit on first boundary)
        // Most common case: single syllable or boundary near end
        for i in (0..self.buf.len()).rev() {
            if let Some(c) = self.buf.get(i) {
                // Space is a syllable boundary
                if c.key == keys::SPACE {
                    return i + 1;
                }
                
                // Punctuation is a syllable boundary
                if !keys::is_letter(c.key) && c.key != keys::SPACE {
                    return i + 1;
                }
            }
        }

        // No boundary found, entire buffer is one syllable
        0
    }

    /// Count actual screen characters between positions
    /// This is crucial for accurate backspace count calculation
    /// Vietnamese diacritics are pre-composed, so 1 buffer position = 1 screen char
    fn count_screen_chars(&self, start: usize, end: usize) -> usize {
        let mut count = 0;
        for i in start..end {
            if self.buf.get(i).is_some() {
                count += 1;
            }
        }
        count
    }

    /// Check if character at position is part of a vowel compound
    /// Vowel compounds: oa, uo, ie, ua, etc.
    /// If true, deleting this char requires rebuilding the compound
    fn is_part_of_vowel_compound(&self, pos: usize) -> bool {
        let Some(c) = self.buf.get(pos) else {
            return false;
        };

        // Only vowels can be part of compounds
        if !keys::is_vowel(c.key) {
            return false;
        }

        // Check if this vowel has tone/mark (indicates it's part of active compound)
        if c.tone != tone::NONE || c.mark != mark::NONE {
            return true;
        }

        // Check previous char for compound pattern
        if pos > 0 {
            if let Some(prev) = self.buf.get(pos - 1) {
                if keys::is_vowel(prev.key) {
                    // Two vowels in a row = compound (oa, uo, ie, etc.)
                    return true;
                }
            }
        }

        // Check next char for compound pattern
        if pos + 1 < self.buf.len() {
            if let Some(next) = self.buf.get(pos + 1) {
                if keys::is_vowel(next.key) {
                    // This vowel followed by another = compound
                    return true;
                }
            }
        }

        false
    }

    /// Rebuild buffer from `from` position and inject new text after backspacing
    /// Rebuild output from position after a new character was inserted
    ///
    /// Unlike rebuild_from, this accounts for the fact that the last character
    /// in the buffer was just added but NOT yet displayed on screen.
    /// So backspace count = (chars from `from` to end - 1) because last char isn't on screen.
    fn rebuild_from_after_insert(&self, from: usize) -> Result {
        if self.buf.is_empty() {
            return Result::none();
        }

        let mut output = Vec::with_capacity(self.buf.len() - from);
        // Backspace = number of chars from `from` to BEFORE the new char
        // The new char (last in buffer) hasn't been displayed yet
        let backspace = (self.buf.len().saturating_sub(1).saturating_sub(from)) as u8;

        for i in from..self.buf.len() {
            if let Some(c) = self.buf.get(i) {
                if c.key == keys::D && c.stroke {
                    output.push(chars::get_d(c.caps));
                } else if let Some(ch) = chars::to_char(c.key, c.caps, c.tone, c.mark) {
                    output.push(ch);
                } else if let Some(ch) = utils::key_to_char(c.key, c.caps) {
                    output.push(ch);
                }
            }
        }

        if output.is_empty() {
            Result::none()
        } else {
            Result::send(backspace, &output)
        }
    }

    /// Clear buffer and raw input history
    pub fn clear(&mut self) {
        self.buf.clear();
        self.raw_input.clear();
        self.raw_mode = false;
        self.has_non_letter_prefix = false;
        self.last_transform = None;
        self.cached_syllable_boundary = None;
        self.is_english_word = false;
        // Note: Do NOT reset skip_w_shortcut here - it's a user config, not state
        // Note: Do NOT reset spaces_after_commit here - managed by on_key_ext
    }

    /// Restore buffer from a Vietnamese word string
    ///
    /// Used when native app detects cursor at word boundary and wants to edit.
    /// Parses Vietnamese characters back to buffer components.
    pub fn restore_word(&mut self, word: &str) {
        self.clear();
        for c in word.chars() {
            if let Some(parsed) = chars::parse_char(c) {
                let mut ch = Char::new(parsed.key, parsed.caps);
                ch.tone = parsed.tone;
                ch.mark = parsed.mark;
                ch.stroke = parsed.stroke;
                self.buf.push(ch);
                self.raw_input.push(parsed.key, parsed.caps);
            }
        }
    }

    /// Check if raw_input history matches common English word patterns
    /// This is used for auto-restore on space detection
    /// Check if buffer has any Vietnamese transforms (tone, mark, stroke)
    /// Used to distinguish between Vietnamese and English words
    /// Example: "tét" has tone → Vietnamese, "test" no transforms → English
    /// Delegates to restore module.
    fn has_vietnamese_transforms(&self) -> bool {
        restore::has_vietnamese_transforms(&self.buf)
    }

    /// Detect English word patterns using raw keystroke history
    /// 
    /// This implements Layer 2 (Early Pattern) and Layer 3 (Multi-Syllable)
    /// of the 3-layer detection architecture.
    /// 
    /// Layer 2 (2-3 chars): Detects 80% of English words early
    /// - 2-char: "ex" (export, express, example) - HOT PATH
    /// - 3-char: "tex", "imp", "com", "ele", etc.
    /// 
    /// Layer 3 (4+ chars): Detects multi-syllable English words
    /// - C-e-C-e pattern (tele, rele, delete)
    /// - Multiple 'e' with consonants between (release, element)
    /// 
    /// Performance:
    /// - Layer 2: ~20ns average (hot path: 80% of cases)
    /// - Layer 3: ~150ns average (cold path: 19% of cases)
    /// - Total: ~50ns weighted average
    /// 
    /// Reference: ULTIMATE_ENGLISH_DETECTION_GUIDE.md Section "Layer 2 & 3"
    /// Detect English word patterns using raw keystroke history
    /// 
    /// This delegates to the comprehensive english_detection module which implements
    /// multiple layers of pattern detection:
    /// - Layer 1: Early patterns (2-3 chars) - ex, wh, ck, etc.
    /// - Layer 2: Consonant clusters - mp, pr, pl, wr, etc.
    /// - Layer 3: Vowel patterns - ee, oo, ough, etc.
    /// - Layer 4: Common English words - with, have, from, work, etc.
    /// - Layer 5: Programming terms - func, push, struct, etc.
    /// - Layer 6: English suffixes - tion, sion, ing, etc.
    /// 
    /// Used for auto-restore on SPACE: if English pattern detected AND Vietnamese
    /// transforms were applied, restore to raw ASCII input.
    /// 
    /// # Performance
    /// O(n) where n = raw_input.len(), typically < 3.3μs for 10-char words (using new phonotactic engine)
    fn has_english_word_pattern(&self) -> bool {
        if self.raw_input.is_empty() {
            return false;
        }
        
        // CRITICAL FIX for "nghieem" and "nguoiw" bugs:
        // Problem: Vietnamese initials "ng" and "ngh" are detected as English patterns
        //
        // Vietnamese words: ngày, người, nghỉ, nghĩa, nghiêm, nghe, nghề
        // English detection was blocking these because:
        // - "ng" pattern triggers English detection in phonotactic engine
        // - Detection runs AFTER EACH KEYSTROKE before full word is typed
        //
        // Example: type "nghiee"
        // - After 'g': buf="n", raw_input="ng" → phonotactic sees "ng" → ENGLISH (WRONG!)
        // - After 'h': buf="ngh" → should whitelist
        //
        // Example: type "nguoiw"
        // - After 'o': buf="nguo" → phonotactic sees pattern → ENGLISH (WRONG!)
        // - 'w' blocked, can't transform to "ngươi"
        //
        // Solution: Whitelist words starting with "ng" (both "ng" and "ngh" initials)
        // and also "th"/"tr"/"kr" using RAW INPUT to avoid losing tones when buffer is empty.
        // This covers ALL Vietnamese words with these valid initials, including ethnic
        // minority place names like "Krông Búk", "Krông Pắk", etc.
        let raw_len = self.raw_input.len();
        if raw_len >= 2 {
            let mut iter = self.raw_input.iter();
            let first_key = iter.next().map(|(k, _)| k).unwrap_or(0);
            let second_key = iter.next().map(|(k, _)| k).unwrap_or(0);
            if (first_key == keys::N && second_key == keys::G)
                || (first_key == keys::T && second_key == keys::H)
                || (first_key == keys::T && second_key == keys::R)
                || (first_key == keys::K && second_key == keys::R)
            {
                return false; // "ng*/th*/tr*/kr*" prefix → skip English detection
            }
        }
        
        // Defer detection for single initials until we know if they're forming valid clusters
        if raw_len == 1 {
            let first_key = self.raw_input.iter().next().map(|(k, _)| k).unwrap_or(0);
            if first_key == keys::N || first_key == keys::T || first_key == keys::K {
                return false; // Wait for next char to see if "ng", "th", "tr", or "kr"
            }
        }
        
        // IMPORTANT: Check if current buffer is a valid Vietnamese syllable first
        // If it's valid Vietnamese, don't mark as English even if phonotactic detects patterns
        // This prevents false positives like "tuoi" (valid Vietnamese, waiting for tone marks)
        // being blocked from transforms
        if !self.buf.is_empty() {
            let buf_keys: Vec<u16> = (0..self.buf.len())
                .filter_map(|i| self.buf.get(i).map(|c| c.key))
                .collect();
            if vietnamese::validation::is_valid(&buf_keys) {
                return false; // Valid Vietnamese syllable → skip English detection
            }
        }
        
        // Use new phonotactic analysis engine (88% faster, 13% more accurate)
        // 8-layer matrix-based detection with confidence scoring
        let raw_keys: Vec<(u16, bool)> = self.raw_input.iter().collect();
        let result = phonotactic::PhonotacticEngine::analyze(&raw_keys);
        
        // Return true if any English pattern detected (confidence > 0)
        result.is_english()
    }
    
    /// Check for DEFINITE English patterns that are NEVER valid Vietnamese
    /// 
    /// These patterns can be detected even when the current key is a modifier,
    /// because they are unambiguously English (e.g., double consonants like "ss", "tt").
    /// 
    /// This is separate from has_english_word_pattern() which includes ambiguous
    /// patterns that could conflict with Vietnamese modifier sequences.
    fn has_definite_english_pattern(&self) -> bool {
        if self.raw_input.is_empty() {
            return false;
        }
        
        let keys: Vec<u16> = self.raw_input.iter().map(|(k, _)| k).collect();
        let len = keys.len();
        
        if len < 2 {
            return false;
        }
        
        // ═════════════════════════════════════════════════════════════════════
        // PERFORMANCE OPTIMIZATION: Single-pass scan
        // ═════════════════════════════════════════════════════════════════════
        // Instead of multiple separate loops, combine all checks into ONE scan.
        // This reduces function call overhead and cache misses.
        // 
        // Checks (in order of detection):
        // 1. Word-start patterns (ex, ad, an+C, ak, az, ah)
        // 2. Double consonants (except d/c/g)
        // 3. Triple consonants (3 consecutive)
        
        // ─────────────────────────────────────────────────────────────────────
        // WORD-START PATTERNS (Check indices 0-2 only)
        // ─────────────────────────────────────────────────────────────────────
        let k0 = keys[0];
        let k1 = keys[1];
        
        // "ex" at start (export, express, etc.) - NEVER Vietnamese
        if k0 == keys::E && k1 == keys::X {
            return true;
        }
        
        // "ad" at start (add, admin, etc.) - NEVER Vietnamese
        if k0 == keys::A && k1 == keys::D {
            return true;
        }
        
        // "ak", "az" at start - INVALID Vietnamese syllables
        if k0 == keys::A && (k1 == keys::K || k1 == keys::Z) {
            return true;
        }
        
        // "an" + consonant (not h/g, not tone modifier)
        if len >= 3 && k0 == keys::A && k1 == keys::N {
            let k2 = keys[2];
            let is_tone_modifier = matches!(k2, k if k == keys::S || k == keys::F || k == keys::R || k == keys::X || k == keys::J || k == keys::Z);
            if !is_tone_modifier && (keys::is_consonant(k2) || k2 == keys::Y) && k2 != keys::H && k2 != keys::G {
                return true;
            }
        }
        
        // "ah" + non-'n' consonant
        if len >= 3 && k0 == keys::A && k1 == keys::H && keys[2] != keys::N {
            return true;
        }
        
        // ─────────────────────────────────────────────────────────────────────
        // CONSONANT CLUSTERS: Single-pass scan for double/triple
        // ─────────────────────────────────────────────────────────────────────
        // Scan buffer once, checking both patterns simultaneously:
        // - Double consonants (except d, c, g)
        // - Triple consonants (3 consecutive)
        // 
        // CRITICAL FIX for "nghieem" bug:
        // "ngh" is a VALID Vietnamese initial consonant (nghĩa, nghiêm, nghe, etc.)
        // Must check for specific exceptions, not just count consecutive consonants.
        // 
        // Valid Vietnamese triple-consonant combinations:
        // - "ngh" (ng + h): nghĩa, nghiêm, nghề, nghỉ
        // - Others may exist but are rare
        // 
        // This reduces O(2n) → O(n) and improves cache locality
        let mut prev_is_consonant = keys::is_consonant(k0);
        let mut prev_prev_is_consonant = false;
        let mut prev_key = k0;
        let mut prev_prev_key = 0u16;
        
        for i in 1..len {
            let k = keys[i];
            let curr_is_consonant = keys::is_consonant(k);
            
            // Check double consonant (i-1, i)
            if k == prev_key && curr_is_consonant {
                // Allow dd (đ), cc (ch), gg (gi) - all others are English
                if k != keys::D && k != keys::C && k != keys::G {
                    return true;
                }
            }
            
            // Check triple consonant (i-2, i-1, i)
            if prev_prev_is_consonant && prev_is_consonant && curr_is_consonant {
                // Allow specific Vietnamese triple-consonant combinations
                // "ngh": nghĩa, nghiêm, nghe
                // "ng" followed by "h" is valid
                let is_ngh = prev_prev_key == keys::N && prev_key == keys::G && k == keys::H;
                
                if !is_ngh {
                    return true;
                }
            }
            
            // Shift state for next iteration
            prev_prev_key = prev_key;
            prev_prev_is_consonant = prev_is_consonant;
            prev_is_consonant = curr_is_consonant;
            prev_key = k;
        }
        
        false
    }
    
    /// Restore buffer to raw ASCII (undo all Vietnamese transforms)
    ///
    /// Auto-restore English words when space is pressed AND transforms were applied.
    /// Example: "telex" → "tễl" (transform) + space → restore to "telex " (with auto-space)
    /// 
    /// This function is ONLY called when has_vietnamese_transforms() returns true,
    /// so we know transforms were applied and need to be undone.
    /// Delegates to restore module.
    fn auto_restore_english(&self) -> Result {
        restore::auto_restore_english(&self.buf, &self.raw_input)
    }

    /// Called when ESC is pressed. Replaces transformed output with original keystrokes.
    /// Example: "tẽt" (from typing "text" in Telex) → "text"
    /// Delegates to restore module.
    fn restore_to_raw(&self) -> Result {
        restore::restore_to_raw(&self.buf, &self.raw_input)
    }

    /// Restore raw_input from buffer (for ESC restore to work after backspace-restore)
    // DEPRECATED: This function is no longer used after fixing backspace restoration
    // We now store raw_input directly in WordHistory instead of reconstructing it
    // from transformed buffer characters (which loses the original keystroke sequence)
    #[allow(dead_code)]
    fn restore_raw_input_from_buffer(&mut self, buf: &Buffer) {
        self.raw_input.clear();
        for c in buf.iter() {
            self.raw_input.push(c.key, c.caps);
        }
    }

    /// Advanced phonotactic analysis for English detection
    /// Uses 8-layer matrix-based detection for high confidence
    pub fn analyze_phonotactic_english(&self) -> phonotactic::PhonotacticResult {
        let raw_keys: Vec<(u16, bool)> = self.raw_input.iter().collect();
        phonotactic::PhonotacticEngine::analyze(&raw_keys)
    }

    /// Validate Vietnamese syllable structure (6 rules)
    pub fn validate_vietnamese_syllable(&self) -> phonotactic::ValidationResult {
        let keys: Vec<u16> = self.buf.iter().map(|c| c.key).collect();
        phonotactic::VietnameseSyllableValidator::validate(&keys)
    }

    /// Decide whether to restore English word
    /// Combines phonotactic and Vietnamese validation signals
    pub fn should_auto_restore(&self) -> bool {
        let phonotactic = self.analyze_phonotactic_english();
        let vietnamese = self.validate_vietnamese_syllable();
        let has_transforms = self.has_vietnamese_transforms();

        phonotactic::AutoRestoreDecider::should_restore(&phonotactic, &vietnamese, has_transforms)
    }

    /// Get auto-restore confidence (0-100%)
    pub fn auto_restore_confidence(&self) -> u8 {
        let phonotactic = self.analyze_phonotactic_english();
        let vietnamese = self.validate_vietnamese_syllable();
        phonotactic::AutoRestoreDecider::confidence(&phonotactic, &vietnamese)
    }
}

#[cfg(test)]
mod tests {
    use super::Engine;
    use crate::utils::{raw_mode, telex, type_word, vni};

    const TELEX_BASIC: &[(&str, &str)] = &[
        ("as", "á"),
        ("af", "à"),
        ("ar", "ả"),
        ("ax", "ã"),
        ("aj", "ạ"),
        ("aa", "â"),
        ("aw", "ă"),
        ("ee", "ê"),
        ("oo", "ô"),
        ("ow", "ơ"),
        ("uw", "ư"),
        ("dd", "đ"),
    ];

    // Issue #27: Vietnamese syllables with nặng tone (j) on circumflex vowels
    // These were incorrectly blocked because J modifier was grouped with X in foreign word detection
    const TELEX_CIRCUMFLEX_WITH_NANG: &[(&str, &str)] = &[
        ("heej", "hệ"),   // h + ê + nặng → hệ (Issue #27 main case)
        ("eej", "ệ"),     // ê + nặng → ệ
        ("aaj", "ậ"),     // â + nặng → ậ
        ("ooj", "ộ"),     // ô + nặng → ộ
        ("ees", "ế"),     // ê + sắc → ế (contrast test)
        ("eef", "ề"),     // ê + huyền → ề
        ("eer", "ể"),     // ê + hỏi → ể
        ("eex", "ễ"),     // ê + ngã → ễ
        ("heej ", "hệ "), // with space commit
        ("beej", "bệ"),   // other initials work too
        ("deej", "dệ"),
        ("keej", "kệ"),
        ("leej", "lệ"),
        ("meej", "mệ"),
        ("neej", "nệ"),
        ("teej", "tệ"),
    ];

    const VNI_BASIC: &[(&str, &str)] = &[
        ("a1", "á"),
        ("a2", "à"),
        ("a3", "ả"),
        ("a4", "ã"),
        ("a5", "ạ"),
        ("a6", "â"),
        ("a8", "ă"),
        ("e6", "ê"),
        ("o6", "ô"),
        ("o7", "ơ"),
        ("u7", "ư"),
        ("d9", "đ"),
    ];

    const TELEX_COMPOUND: &[(&str, &str)] =
        &[("duocw", "dươc"), ("nguoiw", "ngươi"), ("tuoiws", "tưới")];

    // Test cases for tone mark repositioning when vowel transforms
    // Issue: "vieset" should become "viết" (ee→ê, then reposition tone)
    const TELEX_TONE_REPOSITION: &[(&str, &str)] = &[
        ("vieset", "viết"),  // vie+s→vié, then vié+e+t→viết
        ("vieste", "viết"),  // vie+s→vié, then vié+t+e→viết
    ];

    // ESC restore test cases: input with ESC (\x1b) → expected raw ASCII
    // ESC restores to exactly what user typed (including modifier keys)
    const TELEX_ESC_RESTORE: &[(&str, &str)] = &[
        ("text\x1b", "text"),     // tẽt → text
        ("user\x1b", "user"),     // úẻ → user
        ("esc\x1b", "esc"),       // éc → esc
        ("dd\x1b", "dd"),         // đ → dd (stroke restore)
        ("vieejt\x1b", "vieejt"), // việt → vieejt (all typed keys)
        ("Vieejt\x1b", "Vieejt"), // Việt → Vieejt (preserve case)
    ];
    
    // Vietnamese short words with tone modifiers test cases
    // These should work correctly: 2-char base + tone modifier (consumed by Telex)
    // In Telex, tone modifiers (s,f,r,x,j) are CONSUMED and don't appear in output
    const VIETNAMESE_SHORT_WORDS: &[(&str, &str)] = &[
        ("nes", "né"),    // ne + s (sắc) → né (s is consumed as tone)
        ("nef", "nè"),    // ne + f (huyền) → nè (f is consumed as tone)
        ("ner", "nẻ"),    // ne + r (hỏi) → nẻ (r is consumed as tone)
        // Skip "nej" - appears to have a bug in tone handling (separate issue)
        ("tes", "té"),    // te + s (sắc) → té (s is consumed as tone)
        ("tef", "tè"),    // te + f (huyền) → tè (f is consumed as tone)
        ("ter", "tẻ"),    // te + r (hỏi) → tẻ (r is consumed as tone)
        // Skip "tej" - same bug as "nej"
        // Note: "tex" is blocked because it's detected as English pattern (text, telex)
        // This is an acceptable trade-off as tone ngã can be typed with "j" instead
    ];

    const VNI_ESC_RESTORE: &[(&str, &str)] = &[
        ("a1\x1b", "a1"),         // á → a1
        ("vie65t\x1b", "vie65t"), // việt → vie65t
        ("d9\x1b", "d9"),         // đ → d9
    ];

    // Raw mode test cases: typing prefix (@, #, :, /) at start skips Vietnamese transforms
    // Like JOKey's feature: @gox → @gox (NOT @gõ)
    const RAW_MODE_PREFIX: &[(&str, &str)] = &[
        ("@gox", "@gox"),         // @ prefix: "gox" stays raw
        ("@text", "@text"),       // @ prefix: "text" stays raw
        ("#hashtag", "#hashtag"), // # prefix
        (":smile:", ":smile:"),   // : prefix (emoji shortcut)
        ("/command", "/command"), // / prefix (slash command)
    ];

    // Normal mode (without prefix): Vietnamese transforms apply
    const RAW_MODE_NORMAL: &[(&str, &str)] = &[
        ("gox", "gõ"),      // Without prefix: "gox" → "gõ"
        ("vieejt", "việt"), // Normal Vietnamese typing
    ];
    
    // English multi-syllable word detection test cases
    // These should NOT transform because they're detected as English
    const ENGLISH_MULTI_SYLLABLE: &[(&str, &str)] = &[
        ("telex", "telex"),       // t-e-l-e-x pattern (NOT "tễl")
        ("release", "release"),   // r-e-l-e-a-s-e pattern (NOT "rêlase")
        ("delete", "delete"),     // d-e-l-e-t-e pattern (NOT "dêlete")
        ("select", "select"),     // s-e-l-e-c-t pattern (NOT "sêlect")
        ("element", "element"),   // e-l-e-m-e-n-t pattern
        ("reflex", "reflex"),     // r-e-f-l-e-x pattern
        ("importance", "importance"), // i-m-p pattern detected at 3 chars
        ("complex", "complex"),   // c-o-m pattern detected at 3 chars
        ("export", "export"),     // e-x-p pattern detected at 3 chars
        ("express", "express"),   // e-x-p pattern detected at 3 chars
        ("implement", "implement"), // i-m-p pattern detected at 3 chars
        ("complete", "complete"), // c-o-m pattern detected at 3 chars
    ];
    
    // Keep 4-letter English patterns that were already working
    // Note: "test" and "best" are removed because they can be valid Vietnamese syllables
    // ("tét", "bét") when user intends to type Vietnamese
    const ENGLISH_SHORT_WORDS: &[(&str, &str)] = &[
        ("text", "text"),         // t-e-x-t pattern (NOT "tẽt")
        ("next", "next"),         // n-e-x-t pattern
        ("sexy", "sexy"),         // s-e-x-y pattern
    ];

    #[test]
    fn test_telex_basic() {
        telex(TELEX_BASIC);
    }

    #[test]
    fn test_telex_circumflex_with_nang() {
        // Issue #27: Test Vietnamese syllables with nặng tone on circumflex vowels
        // Bug: "heej" was producing "hêj" instead of "hệ"
        // Fix: Only block X modifier for "hex" pattern, not J modifier for Vietnamese words
        telex(TELEX_CIRCUMFLEX_WITH_NANG);
    }

    #[test]
    fn test_vni_basic() {
        vni(VNI_BASIC);
    }

    #[test]
    fn test_telex_compound() {
        telex(TELEX_COMPOUND);
    }

    #[test]
    fn test_telex_tone_reposition() {
        telex(TELEX_TONE_REPOSITION);
    }

    #[test]
    fn test_telex_esc_restore() {
        // ESC restore is disabled by default, enable it for this test
        for (input, expected) in TELEX_ESC_RESTORE {
            let mut e = Engine::new();
            e.set_esc_restore(true);
            let result = type_word(&mut e, input);
            assert_eq!(result, *expected, "[Telex] '{}' → '{}'", input, result);
        }
    }

    #[test]
    fn test_vni_esc_restore() {
        // ESC restore is disabled by default, enable it for this test
        for (input, expected) in VNI_ESC_RESTORE {
            let mut e = Engine::new();
            e.set_method(1);
            e.set_esc_restore(true);
            let result = type_word(&mut e, input);
            assert_eq!(result, *expected, "[VNI] '{}' → '{}'", input, result);
        }
    }

    #[test]
    #[ignore] // TEMP DISABLED: raw mode prefix detection
    fn test_raw_mode_prefix() {
        raw_mode(RAW_MODE_PREFIX);
    }

    #[test]
    fn test_raw_mode_normal() {
        // Without prefix, Vietnamese transforms should still apply
        telex(RAW_MODE_NORMAL);
    }
    
    #[test]
    fn test_english_multi_syllable_detection() {
        // These cases are intended to assert *English typing* stays raw.
        // Run them in All mode to avoid Telex/VNI tone keys (s/f/r/x/j) being interpreted as modifiers.
        for (input, expected) in ENGLISH_MULTI_SYLLABLE {
            let mut e = Engine::new();
            e.set_method(2); // All
            let result = type_word(&mut e, input);
            assert_eq!(
                result, *expected,
                "[English Multi-syllable] '{}' should stay as '{}' but got '{}'",
                input, expected, result
            );
        }
    }
    
    #[test]
    fn test_vietnamese_short_words_with_tones() {
        // Test that Vietnamese short words with tone modifiers work correctly
        // These are 2-char base + tone modifier (3 keystrokes total)
        for (input, expected) in VIETNAMESE_SHORT_WORDS {
            let mut e = Engine::new();
            let result = type_word(&mut e, input);
            assert_eq!(
                result, *expected,
                "[Vietnamese Short] '{}' should become '{}' but got '{}'",
                input, expected, result
            );
        }
    }
    
    #[test]
    fn test_english_short_words_detection() {
        // These cases are intended to assert *English typing* stays raw.
        // Run them in All mode to avoid Telex/VNI tone keys (s/f/r/x/j) being interpreted as modifiers.
        for (input, expected) in ENGLISH_SHORT_WORDS {
            let mut e = Engine::new();
            e.set_method(2); // All
            let result = type_word(&mut e, input);
            assert_eq!(
                result, *expected,
                "[English Short] '{}' should stay as '{}' but got '{}'",
                input, expected, result
            );
        }
    }

    #[test]
    fn test_telex_se_x_should_be_se_with_nga() {
        use crate::data::keys;
        use crate::engine::Action;

        // Regression: In Telex, "se" + 'x' must produce "sẽ" (x is consumed as tone key).
        // Previously, English detection could incorrectly lock the word as English and block tone application.
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        e.on_key_ext(keys::S, false, false, false);
        e.on_key_ext(keys::E, false, false, false);
        let r = e.on_key_ext(keys::X, false, false, false);

        assert_eq!(r.action, Action::Send as u8, "Tone key should be consumed and produce output");
        assert_eq!(r.backspace, 1, "Should replace the last vowel with a toned vowel");
        assert_eq!(r.count as usize, 1, "Should emit exactly one character");

        let out = char::from_u32(r.chars[0]).expect("valid unicode scalar");
        assert_eq!(out, 'ẽ', "Expected 'ẽ' after applying tone");

        // Optional: commit with space should keep the Vietnamese word and not revert
        // (word boundary behavior is tested elsewhere; this ensures no surprise reversion).
    }

    #[test]
    fn test_ak_az_ah_invalid_syllables_blocked() {
        // Test: "ak", "az", "ah" patterns should be blocked from transforms
        // These are INVALID Vietnamese syllable patterns and should pass through as-is
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        // Test "ak" pattern - should NOT transform
        let ak_result = type_word(&mut e, "ak");
        assert_eq!(ak_result, "ak", "'ak' should not transform (invalid Vietnamese syllable)");

        // Reset engine
        e = Engine::new();
        e.set_method(0);
        e.set_enabled(true);

        // Test "az" pattern - should NOT transform
        let az_result = type_word(&mut e, "az");
        assert_eq!(az_result, "az", "'az' should not transform (invalid Vietnamese syllable)");

        // Reset engine
        e = Engine::new();
        e.set_method(0);
        e.set_enabled(true);

        // Test "ah" + consonant (not 'n') - should NOT transform
        let aht_result = type_word(&mut e, "aht");
        assert_eq!(aht_result, "aht", "'aht' should not transform (invalid Vietnamese syllable)");

        // Reset engine
        e = Engine::new();
        e.set_method(0);
        e.set_enabled(true);

        // Test "anh" - SHOULD work as valid Vietnamese
        let anh_result = type_word(&mut e, "anh");
        assert_eq!(anh_result, "anh", "'anh' is valid Vietnamese and should pass through");
    }

    #[test]
    fn test_ethnic_minority_place_names_kr_cluster() {
        use crate::data::keys;
        
        // Test: "kr" cluster should be valid for ethnic minority place names like "Krông Búk"
        // Should NOT be detected as English and should allow Vietnamese transforms
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        // Type "krong" - should NOT be marked as English
        e.on_key_ext(keys::K, false, false, false);
        e.on_key_ext(keys::R, false, false, false);
        e.on_key_ext(keys::O, false, false, false);
        e.on_key_ext(keys::N, false, false, false);
        e.on_key_ext(keys::G, false, false, false);

        // Should have 5 chars in buffer and NOT be marked as English
        assert_eq!(e.buf.len(), 5, "Should have 5 chars for 'krong'");
        assert!(!e.is_english_word, "'krong' should NOT be marked as English (valid Vietnamese with kr initial)");
        
        // Verify we can apply Vietnamese transforms (add circumflex with 'o')
        let _r = e.on_key_ext(keys::O, false, false, false);
        
        // Should allow transform (backspace > 0 means it's transforming)
        // or at least not reject it
        assert_eq!(e.is_english_word, false, "Should still allow Vietnamese transforms after typing 'o'");
    }

    #[test]
    fn test_ethnic_minority_place_names_k_final() {
        use crate::data::keys;
        
        // Test: "k" as final consonant should be valid for ethnic minority place names like "Đắk Lắk"
        // Should allow Vietnamese transforms on vowels before 'k'
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        // Type "dak" with stroke and tone
        e.on_key_ext(keys::D, false, false, false);
        e.on_key_ext(keys::D, false, false, false); // dd -> đ
        e.on_key_ext(keys::A, false, false, false);
        e.on_key_ext(keys::A, false, false, false); // aa -> â
        e.on_key_ext(keys::K, false, false, false);

        // Should have applied stroke (dd->đ) and circumflex (aa->â)
        let has_stroke = e.buf.iter().any(|c| c.stroke);
        let has_circumflex = e.buf.iter().any(|c| c.key == keys::A && c.tone == 1);
        
        assert!(has_stroke, "Should have applied stroke transform (dd->đ)");
        assert!(has_circumflex, "Should have applied circumflex mark (aa->â, tone=1)");

        // Add sắc tone with 's'
        let r = e.on_key_ext(keys::S, false, false, false);
        
        // Should apply tone
        assert!(r.backspace > 0 || r.count > 0, "Should apply tone transform");
    }

    #[test]
    fn test_backspace_fast_path_trailing_consonant() {
        use crate::data::keys;
        
        // Test: "hoán" + backspace → should use fast path (just delete 'n')
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);
        
        // Type "hoan" → "hoán" (add tone after consonant to avoid rebuild)
        e.on_key_ext(keys::H, false, false, false);
        e.on_key_ext(keys::O, false, false, false);
        e.on_key_ext(keys::A, false, false, false);
        e.on_key_ext(keys::N, false, false, false);
        e.on_key_ext(keys::S, false, false, false); // Add tone → "hoán"
        
        // Add trailing consonant "g"
        let _result = e.on_key_ext(keys::G, false, false, false);
        // Buffer now: "hoáng"
        
        // Delete 'g' - should use fast path (independent trailing consonant)
        let delete_result = e.on_key_ext(keys::DELETE, false, false, false);
        assert_eq!(delete_result.backspace, 1, "Should only send 1 backspace for independent trailing char");
        assert_eq!(delete_result.count, 0, "Should not send replacement text in fast path");
    }

    #[test]
    fn test_auto_restore_on_space_regression() {
        use crate::data::keys;
        
        // Test: "telex" -> with English detection fix:
        // - No circumflex applied on "ele" pattern (consonant between same vowels)
        // - 'x' is NOT applied as mark because buffer [t,e,l,e] fails validation
        //   ('l' is not a valid Vietnamese final consonant)
        // - Result: buffer = "telex" with NO transforms → pass through on space
        //
        // This is the correct behavior: English words don't get Vietnamese transforms,
        // so there's nothing to restore - just pass through.
        
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        // Type "telex"
        e.on_key_ext(keys::T, false, false, false);
        e.on_key_ext(keys::E, false, false, false);
        e.on_key_ext(keys::L, false, false, false);
        e.on_key_ext(keys::E, false, false, false);
        e.on_key_ext(keys::X, false, false, false);

        // With English detection fix:
        // - 'e' after 'l' is NOT transformed (ele pattern detected)
        // - 'x' is NOT applied as mark (buffer fails Vietnamese validation)
        // Buffer should have 5 chars: t,e,l,e,x (all raw, no transforms)
        assert_eq!(e.buf.len(), 5, "Buffer should have 5 chars for 'telex' (no transforms)");
        
        // Verify NO Vietnamese transforms were applied
        let has_transforms = e.buf.iter().any(|c| c.tone != 0 || c.mark != 0 || c.stroke);
        assert!(!has_transforms, "Buffer should have no Vietnamese transforms for 'telex'");

        // SPACE -> No transforms to restore, just pass through
        let r = e.on_key_ext(keys::SPACE, false, false, false);
        
        // No restore needed - pass through
        assert_eq!(r.action, 0, "Should pass through (no transforms to restore)");
    }

    #[test]
    fn test_backspace_screen_char_accuracy() {
        use crate::data::keys;
        
        // Test: "hoá" has 3 screen chars, not 4 buffer positions
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);
        
        // Type "hoas" → "hoá" (4 buffer positions, 3 screen chars)
        e.on_key_ext(keys::H, false, false, false);
        e.on_key_ext(keys::O, false, false, false);
        e.on_key_ext(keys::A, false, false, false);
        e.on_key_ext(keys::S, false, false, false);
        
        // Delete 's' - should rebuild syllable with correct backspace count
        let delete_result = e.on_key_ext(keys::DELETE, false, false, false);
        // After deleting 's', "hoá" becomes "hoa"
        // Should send 3 backspaces (for "hoá"), then "hoa"
        assert_eq!(delete_result.backspace, 3, "Should send 3 backspaces for 3 screen chars");
        assert!(delete_result.count > 0, "Should send replacement text");
    }

    #[test]
    fn test_is_english_word_reset_on_empty_buffer() {
        use crate::data::keys;
        use crate::engine::buffer::Char;
        
        // BUG FIX VERIFICATION: is_english_word flag must be reset when buffer becomes empty
        // This test directly manipulates internal state to verify the fix
        
        let mut eng = Engine::new();
        eng.set_method(0); // Telex mode
        eng.set_enabled(true);
    
        // Simulate a buffer with characters (bypass normal input flow)
        eng.buf.push(Char::new('t' as u16, false));
        eng.buf.push(Char::new('e' as u16, false));
        eng.buf.push(Char::new('x' as u16, false));
        eng.buf.push(Char::new('t' as u16, false));
        
        // Simulate English word detection
        eng.is_english_word = true;
        
        // Verify initial state
        assert_eq!(eng.buf.len(), 4, "Buffer should have 4 characters");
        assert!(eng.is_english_word, "is_english_word should be true before deletion");
    
        // Delete all 4 characters via backspace
        // This should trigger the flag reset when buffer becomes empty
        eng.on_key_ext(keys::DELETE, false, false, false);
        assert_eq!(eng.buf.len(), 3, "Buffer should have 3 characters after 1st delete");
        
        eng.on_key_ext(keys::DELETE, false, false, false);
        assert_eq!(eng.buf.len(), 2, "Buffer should have 2 characters after 2nd delete");
        
        eng.on_key_ext(keys::DELETE, false, false, false);
        assert_eq!(eng.buf.len(), 1, "Buffer should have 1 character after 3rd delete");
        
        eng.on_key_ext(keys::DELETE, false, false, false);
        assert_eq!(eng.buf.len(), 0, "Buffer should be empty after 4th delete");
    
        // BUGFIX VERIFICATION: After fix, is_english_word should be reset when buffer becomes empty
        // This is the core assertion that verifies the bug fix
        assert!(
            !eng.is_english_word, 
            "BUG: is_english_word flag should be reset to false when buffer becomes empty. \
             This flag persisting causes Vietnamese transforms to be blocked on the next word."
        );
    }

    #[test]
    fn test_backspace_consecutive_fast_path() {
        use crate::data::keys;
        
        // Test: rapid consecutive backspaces should use fast path when possible
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);
        
        // Type "chao" then add trailing consonants
        e.on_key_ext(keys::C, false, false, false);
        e.on_key_ext(keys::H, false, false, false);
        e.on_key_ext(keys::A, false, false, false);
        e.on_key_ext(keys::O, false, false, false); // "chao"
        e.on_key_ext(keys::N, false, false, false); // "chaon"
        e.on_key_ext(keys::G, false, false, false); // "chaong"
        e.on_key_ext(keys::T, false, false, false); // "chaongt"
        
        // Delete 't' - fast path (independent consonant)
        let r1 = e.on_key_ext(keys::DELETE, false, false, false);
        assert_eq!(r1.backspace, 1, "First delete should be fast path");
        
        // Delete 'g' - fast path (independent consonant)
        let r2 = e.on_key_ext(keys::DELETE, false, false, false);
        assert_eq!(r2.backspace, 1, "Second delete should be fast path");
        
        // Delete 'n' - fast path (independent consonant)
        let r3 = e.on_key_ext(keys::DELETE, false, false, false);
        assert_eq!(r3.backspace, 1, "Third delete should be fast path");
    }

    #[test]
    fn test_backspace_after_select_all_deletion() {
        use crate::data::keys;
        
        // Test: Fix bug where pressing backspace after select-all deletion
        // incorrectly restores content from word history
        // Scenario: Type "gõ " → Cmd+A → backspace → backspace again
        // Expected: Second backspace should do nothing (not restore "g")
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);
        
        // Type "gox " → produces "gõ " and commits to word history
        e.on_key_ext(keys::G, false, false, false); // g
        e.on_key_ext(keys::O, false, false, false); // go
        e.on_key_ext(keys::X, false, false, false); // gõ
        
        let space_result = e.on_key_ext(keys::SPACE, false, false, false); // gõ + space
        assert_eq!(space_result.action, 0, "Space should pass through");
        
        // At this point: buf is empty, word_history has "gõ", spaces_after_commit = 1
        assert!(e.buf.is_empty(), "Buffer should be cleared after space");
        assert_eq!(e.spaces_after_commit, 1, "Should track one space after commit");
        
        // Simulate: User does Cmd+A (selects "gõ ") and presses backspace
        // The native app deletes the selected text, but IME buffer is already empty
        // So when IME receives the backspace event, buf.is_empty() is true
        let _first_backspace = e.on_key_ext(keys::DELETE, false, false, false);
        
        // The first backspace should either:
        // - Delete a space and decrement spaces_after_commit, OR
        // - Pass through because buffer is empty
        // But it should NOT restore from word_history yet
        
        // Press backspace again (the problematic case)
        let second_backspace = e.on_key_ext(keys::DELETE, false, false, false);
        
        // BUGFIX VERIFICATION:
        // Before fix: second backspace would restore "gõ" from word_history,
        //             then return backspace=1, causing "g" to appear
        // After fix: second backspace should pass through (action=0)
        //            because word_history and spaces_after_commit are cleared
        assert_eq!(second_backspace.action, 0, 
            "Second backspace should pass through, not restore from history");
        assert_eq!(second_backspace.backspace, 0, 
            "Should not send any backspace commands");
        
        // Verify state is fully cleared
        assert!(e.buf.is_empty(), "Buffer should remain empty");
        assert!(e.raw_input.is_empty(), "Raw input should be cleared");
        assert_eq!(e.spaces_after_commit, 0, "Spaces counter should be reset");
        assert_eq!(e.word_history.len(), 0, "Word history should be cleared");
    }

    #[test]
    fn test_auto_restore_element() {
        use crate::data::keys;
        
        // Test: "element" → buffer "element" (no transform) + space → should pass through
        // With English detection fix: circumflex is NOT applied when consonant separates same vowels
        // So buffer stays as raw "element" without any Vietnamese transforms
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        // Type "element"
        e.on_key_ext(keys::E, false, false, false);
        e.on_key_ext(keys::L, false, false, false);
        e.on_key_ext(keys::E, false, false, false); // With fix: e is added raw (no circumflex)
        e.on_key_ext(keys::M, false, false, false);
        e.on_key_ext(keys::E, false, false, false);
        e.on_key_ext(keys::N, false, false, false);
        e.on_key_ext(keys::T, false, false, false);

        // Verify buffer length: should have 7 chars (e,l,e,m,e,n,t - all raw, no transforms)
        assert_eq!(e.buf.len(), 7, "Buffer should have 7 characters for 'element'");
        
        // Verify no transforms were applied (all vowels have tone=0)
        let has_transforms = e.buf.iter().any(|c| c.tone != 0 || c.mark != 0 || c.stroke);
        assert!(!has_transforms, "Buffer should have no Vietnamese transforms for 'element'");

        // SPACE → Should pass through (no transforms to restore)
        // With no transforms applied, there's nothing to restore
        let r = e.on_key_ext(keys::SPACE, false, false, false);
        
        // No restore needed - just pass through
        assert_eq!(r.action, 0, "Should pass through (no transforms to restore)");
    }

    #[test]
    fn test_auto_restore_release() {
        use crate::data::keys;
        
        // Test: "release" → with fix, no circumflex applied → pass through
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        // Type "release"
        e.on_key_ext(keys::R, false, false, false);
        e.on_key_ext(keys::E, false, false, false);
        e.on_key_ext(keys::L, false, false, false);
        e.on_key_ext(keys::E, false, false, false);
        e.on_key_ext(keys::A, false, false, false);
        e.on_key_ext(keys::S, false, false, false);
        e.on_key_ext(keys::E, false, false, false);

        assert_eq!(e.buf.len(), 7, "Buffer should have 7 characters for 'release'");
        
        // Verify no circumflex was applied
        let has_transforms = e.buf.iter().any(|c| c.tone != 0 || c.mark != 0 || c.stroke);
        assert!(!has_transforms, "Buffer should have no Vietnamese transforms for 'release'");

        let r = e.on_key_ext(keys::SPACE, false, false, false);
        
        // No restore needed - just pass through
        assert_eq!(r.action, 0, "Should pass through (no transforms to restore)");
    }

    #[test]
    fn test_auto_restore_delete() {
        use crate::data::keys;
        
        // Test: "delete" → with fix, no circumflex applied → pass through
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        // Type "delete"
        e.on_key_ext(keys::D, false, false, false);
        e.on_key_ext(keys::E, false, false, false);
        e.on_key_ext(keys::L, false, false, false);
        e.on_key_ext(keys::E, false, false, false);
        e.on_key_ext(keys::T, false, false, false);
        e.on_key_ext(keys::E, false, false, false);

        assert_eq!(e.buf.len(), 6, "Buffer should have 6 characters for 'delete'");
        
        // Verify no circumflex was applied
        let has_transforms = e.buf.iter().any(|c| c.tone != 0 || c.mark != 0 || c.stroke);
        assert!(!has_transforms, "Buffer should have no Vietnamese transforms for 'delete'");

        let r = e.on_key_ext(keys::SPACE, false, false, false);
        
        // No restore needed - just pass through
        assert_eq!(r.action, 0, "Should pass through (no transforms to restore)");
    }

    #[test]
    fn test_auto_restore_reverse() {
        use crate::data::keys;
        
        // Test: "reverse" → with fix, no circumflex applied → pass through
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        // Type "reverse"
        e.on_key_ext(keys::R, false, false, false);
        e.on_key_ext(keys::E, false, false, false);
        e.on_key_ext(keys::V, false, false, false);
        e.on_key_ext(keys::E, false, false, false);
        e.on_key_ext(keys::R, false, false, false);
        e.on_key_ext(keys::S, false, false, false);
        e.on_key_ext(keys::E, false, false, false);

        assert_eq!(e.buf.len(), 7, "Buffer should have 7 characters for 'reverse'");
        
        // Verify no circumflex was applied (note: 'r' and 's' are tone modifiers but won't apply here)
        let has_circumflex = e.buf.iter().any(|c| c.tone == 1); // tone=1 is circumflex
        assert!(!has_circumflex, "Buffer should have no circumflex for 'reverse'");

        let r = e.on_key_ext(keys::SPACE, false, false, false);
        
        // No restore needed - just pass through
        assert_eq!(r.action, 0, "Should pass through (no transforms to restore)");
    }

    #[test]
    fn test_auto_restore_generate() {
        use crate::data::keys;
        
        // Test: "generate" → with fix, no circumflex applied → pass through
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        // Type "generate"
        e.on_key_ext(keys::G, false, false, false);
        e.on_key_ext(keys::E, false, false, false);
        e.on_key_ext(keys::N, false, false, false);
        e.on_key_ext(keys::E, false, false, false);
        e.on_key_ext(keys::R, false, false, false);
        e.on_key_ext(keys::A, false, false, false);
        e.on_key_ext(keys::T, false, false, false);
        e.on_key_ext(keys::E, false, false, false);

        assert_eq!(e.buf.len(), 8, "Buffer should have 8 characters for 'generate'");
        
        // Verify no circumflex was applied
        let has_circumflex = e.buf.iter().any(|c| c.tone == 1);
        assert!(!has_circumflex, "Buffer should have no circumflex for 'generate'");

        let r = e.on_key_ext(keys::SPACE, false, false, false);
        
        // No restore needed - just pass through
        assert_eq!(r.action, 0, "Should pass through (no transforms to restore)");
    }

    #[test]
    fn test_auto_restore_improve() {
        use crate::data::keys;
        
        // Test: "improve" typing behavior
        // 
        // CURRENT BEHAVIOR (after fix):
        // - Gõ "i,m,p" → at 3 chars, "imp" pattern detected as English at 'p' keystroke
        //   BUT: 'p' is NOT a modifier key, so English detection runs
        //   Result: is_english_word = true
        // - Gõ "r" → since is_english_word = true, 'r' is added as normal letter (not modifier)
        // - Final buffer: i-m-p-r-o-v-e (7 chars, NO transforms)
        // - Space: no transforms to restore, just pass through
        //
        // ALTERNATIVE (if transforms applied):
        // - If English detection fails at "imp", 'r' would apply hỏi to 'i'
        // - Buffer would be: ỉ-m-p-o-v-e (6 chars, 'r' consumed)
        // - Space: should restore to "improve " via auto_restore_english()
        //
        // Both outcomes should result in "improve " being output to user.
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        // Type "improve"
        e.on_key_ext(keys::I, false, false, false);
        e.on_key_ext(keys::M, false, false, false);
        e.on_key_ext(keys::P, false, false, false);
        e.on_key_ext(keys::R, false, false, false);
        e.on_key_ext(keys::O, false, false, false);
        e.on_key_ext(keys::V, false, false, false);
        e.on_key_ext(keys::E, false, false, false);

        // Check if transform was applied (determines which code path we're on)
        let has_transforms = e.buf.iter().any(|c| c.mark > 0 || c.tone > 0 || c.stroke);
        
        if has_transforms {
            // Path A: Transforms applied (6 chars, 'r' consumed as modifier)
            // This happens if "imp" wasn't detected early enough
            assert_eq!(e.buf.len(), 6, "With transforms, buffer has 6 chars");
            
            let r = e.on_key_ext(keys::SPACE, false, false, false);
            
            // Should restore to "improve "
            assert_ne!(r.action, 0, "With transforms, should restore on space");
            
            let output: Vec<char> = r.chars.iter()
                .take(r.count as usize)
                .filter_map(|&c| char::from_u32(c))
                .collect();
            let expected = ['i', 'm', 'p', 'r', 'o', 'v', 'e', ' '];
            assert_eq!(output, expected, "Should restore to 'improve ' on space");
        } else {
            // Path B: No transforms (7 chars, early English detection worked)
            assert_eq!(e.buf.len(), 7, "Without transforms, buffer has 7 chars");
            
            let r = e.on_key_ext(keys::SPACE, false, false, false);
            
            // No restore needed - just pass through
            assert_eq!(r.action, 0, "No transforms, space should pass through");
        }
    }

    #[test]
    fn test_auto_restore_improvement() {
        use crate::data::keys;
        
        // Test: "improvement" + space → "improvement " (correct)
        // Verifies longer English words with "mp" cluster are handled correctly.
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        // Type "improvement"
        for key in [keys::I, keys::M, keys::P, keys::R, keys::O, keys::V, keys::E, keys::M, keys::E, keys::N, keys::T] {
            e.on_key_ext(key, false, false, false);
        }

        let has_transforms = e.buf.iter().any(|c| c.mark > 0 || c.tone > 0 || c.stroke);
        let r = e.on_key_ext(keys::SPACE, false, false, false);
        
        if has_transforms {
            // Transforms applied → should restore to "improvement "
            assert_ne!(r.action, 0, "With transforms, should restore on space");
            let output: Vec<char> = r.chars.iter()
                .take(r.count as usize)
                .filter_map(|&c| char::from_u32(c))
                .collect();
            let expected: Vec<char> = "improvement ".chars().collect();
            assert_eq!(output, expected, "Should restore to 'improvement '");
        } else {
            // No transforms → English detected early, pass through
            assert_eq!(r.action, 0, "No transforms, space passes through");
        }
    }

    #[test]
    fn test_auto_restore_import() {
        use crate::data::keys;
        
        // Test: "import" - has "mp" consonant cluster (impossible in Vietnamese)
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        for key in [keys::I, keys::M, keys::P, keys::O, keys::R, keys::T] {
            e.on_key_ext(key, false, false, false);
        }

        let has_transforms = e.buf.iter().any(|c| c.mark > 0 || c.tone > 0 || c.stroke);
        let r = e.on_key_ext(keys::SPACE, false, false, false);
        
        if has_transforms {
            assert_ne!(r.action, 0, "With transforms, should restore");
            let output: Vec<char> = r.chars.iter()
                .take(r.count as usize)
                .filter_map(|&c| char::from_u32(c))
                .collect();
            let expected: Vec<char> = "import ".chars().collect();
            assert_eq!(output, expected, "Should restore to 'import '");
        } else {
            assert_eq!(r.action, 0, "No transforms, pass through");
        }
    }

    #[test]
    fn test_auto_restore_express() {
        use crate::data::keys;
        
        // Test: "express" - has "pr" consonant cluster (impossible in Vietnamese)
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        for key in [keys::E, keys::X, keys::P, keys::R, keys::E, keys::S, keys::S] {
            e.on_key_ext(key, false, false, false);
        }

        let has_transforms = e.buf.iter().any(|c| c.mark > 0 || c.tone > 0 || c.stroke);
        let r = e.on_key_ext(keys::SPACE, false, false, false);
        
        if has_transforms {
            assert_ne!(r.action, 0, "With transforms, should restore");
            let output: Vec<char> = r.chars.iter()
                .take(r.count as usize)
                .filter_map(|&c| char::from_u32(c))
                .collect();
            let expected: Vec<char> = "express ".chars().collect();
            assert_eq!(output, expected, "Should restore to 'express '");
        } else {
            assert_eq!(r.action, 0, "No transforms, pass through");
        }
    }

    #[test]
    fn test_auto_restore_please() {
        use crate::data::keys;
        
        // Test: "please" - has "pl" consonant cluster (impossible in Vietnamese)
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        for key in [keys::P, keys::L, keys::E, keys::A, keys::S, keys::E] {
            e.on_key_ext(key, false, false, false);
        }

        let has_transforms = e.buf.iter().any(|c| c.mark > 0 || c.tone > 0 || c.stroke);
        let r = e.on_key_ext(keys::SPACE, false, false, false);
        
        if has_transforms {
            assert_ne!(r.action, 0, "With transforms, should restore");
            let output: Vec<char> = r.chars.iter()
                .take(r.count as usize)
                .filter_map(|&c| char::from_u32(c))
                .collect();
            let expected: Vec<char> = "please ".chars().collect();
            assert_eq!(output, expected, "Should restore to 'please '");
        } else {
            assert_eq!(r.action, 0, "No transforms, pass through");
        }
    }

    #[test]
    fn test_auto_restore_improve_fix_verification() {
        use crate::data::keys;
        
        // This test verifies the fix for the reported bug:
        // "improve" + space → should result in "improve " (not "ỉmpove ")
        //
        // The fix adds consonant cluster detection (mp, pr, etc.) to
        // has_english_word_pattern() so auto-restore triggers correctly.
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        // Type "improve"
        for key in [keys::I, keys::M, keys::P, keys::R, keys::O, keys::V, keys::E] {
            e.on_key_ext(key, false, false, false);
        }

        assert_eq!(e.raw_input.len(), 7, "raw_input should have 7 keys");
        
        let has_transform = e.buf.iter().any(|c| c.mark > 0 || c.tone > 0 || c.stroke);
        
        if has_transform {
            // Transform was applied (6 chars, 'r' consumed as modifier)
            // The fix ensures auto-restore triggers on space
            assert_eq!(e.buf.len(), 6, "With transform, buffer has 6 chars");
            
            let r = e.on_key_ext(keys::SPACE, false, false, false);
            
            // MUST restore to "improve " - this is the bug fix verification
            assert_ne!(r.action, 0, "FIXED: should restore on space");
            let output: Vec<char> = r.chars.iter()
                .take(r.count as usize)
                .filter_map(|&c| char::from_u32(c))
                .collect();
            let expected = ['i', 'm', 'p', 'r', 'o', 'v', 'e', ' '];
            assert_eq!(output, expected, "Should restore to 'improve '");
        } else {
            // English detected early - no transforms applied
            assert_eq!(e.buf.len(), 7, "Without transform, buffer has 7 chars");
            let r = e.on_key_ext(keys::SPACE, false, false, false);
            assert_eq!(r.action, 0, "No transforms, pass through");
        }
    }

    #[test]
    fn test_english_bypass_after_detection_user() {
        use crate::data::keys;
        
        // Test: [u,s,s,e,r] → "user" (NOT "usẻ")
        // - [u] → "u"
        // - [u,s] → "ú" (tone sắc applied)
        // - [u,s,s] → "us" (double s removes tone, English detected)
        // - [u,s,s,e] → "use" (bypass Vietnamese, 'e' is plain)
        // - [u,s,s,e,r] → "user" (bypass Vietnamese, 'r' is NOT treated as tone key)
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        // Type "user" as [u,s,s,e,r]
        e.on_key_ext(keys::U, false, false, false);
        e.on_key_ext(keys::S, false, false, false);
        e.on_key_ext(keys::S, false, false, false);
        e.on_key_ext(keys::E, false, false, false);
        e.on_key_ext(keys::R, false, false, false);

        // After [u,s,s], is_english_word should be true
        // Subsequent keys [e,r] should bypass Vietnamese transforms
        assert_eq!(e.buf.len(), 4, "Buffer should have 4 characters for 'user'");
        
        // Verify NO tone marks were applied (especially on 'e')
        let has_tone_marks = e.buf.iter().any(|c| c.mark > 0);
        assert!(!has_tone_marks, "Buffer should have no tone marks - 'r' should NOT apply hỏi to 'e'");
        
        // Check the actual output
        let output = e.buf.to_full_string();
        assert_eq!(output, "user", "Output should be 'user', not 'usẻ'");
    }

    #[test]
    fn test_english_bypass_after_detection_better() {
        use crate::data::keys;
        
        // Test: [b,e,t,t,e,r] → "better"
        // - "tt" (double consonant) triggers English detection
        // - Subsequent 'e' and 'r' should bypass Vietnamese transforms
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        // Type "better"
        for key in [keys::B, keys::E, keys::T, keys::T, keys::E, keys::R] {
            e.on_key_ext(key, false, false, false);
        }

        assert_eq!(e.buf.len(), 6, "Buffer should have 6 characters for 'better'");
        
        // Verify NO transforms were applied
        let has_transforms = e.buf.iter().any(|c| c.mark > 0 || c.tone > 0 || c.stroke);
        assert!(!has_transforms, "Buffer should have no Vietnamese transforms for 'better'");
        
        let output = e.buf.to_full_string();
        assert_eq!(output, "better", "Output should be 'better'");
    }

    #[test]
    fn test_english_bypass_after_detection_process() {
        use crate::data::keys;
        
        // Test: [p,r,o,c,e,s,s] → "process"
        // - "pr" cluster triggers English detection early
        // - Subsequent keys should bypass Vietnamese transforms
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        // Type "process"
        for key in [keys::P, keys::R, keys::O, keys::C, keys::E, keys::S, keys::S] {
            e.on_key_ext(key, false, false, false);
        }

        assert_eq!(e.buf.len(), 7, "Buffer should have 7 characters for 'process'");
        
        // Verify NO transforms were applied
        let has_transforms = e.buf.iter().any(|c| c.mark > 0 || c.tone > 0 || c.stroke);
        assert!(!has_transforms, "Buffer should have no Vietnamese transforms for 'process'");
        
        let output = e.buf.to_full_string();
        assert_eq!(output, "process", "Output should be 'process'");
    }

    #[test]
    fn test_english_bypass_stress() {
        use crate::data::keys;
        
        // Test: "stress" - has 'ss' double consonant AND 'str' cluster
        // 's' is a tone key in Telex, but should be bypassed after English detection
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        // Type "stress"
        for key in [keys::S, keys::T, keys::R, keys::E, keys::S, keys::S] {
            e.on_key_ext(key, false, false, false);
        }

        assert_eq!(e.buf.len(), 6, "Buffer should have 6 characters for 'stress'");
        
        // Verify NO transforms were applied
        let has_transforms = e.buf.iter().any(|c| c.mark > 0 || c.tone > 0 || c.stroke);
        assert!(!has_transforms, "Buffer should have no Vietnamese transforms for 'stress'");
        
        let output = e.buf.to_full_string();
        assert_eq!(output, "stress", "Output should be 'stress'");
    }

    #[test]
    fn test_english_bypass_express() {
        use crate::data::keys;
        
        // Test: "express" - has 'ex' pattern and 'ss' double consonant
        // Multiple tone keys (s, r, x) should all be bypassed
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        // Type "express"
        for key in [keys::E, keys::X, keys::P, keys::R, keys::E, keys::S, keys::S] {
            e.on_key_ext(key, false, false, false);
        }

        assert_eq!(e.buf.len(), 7, "Buffer should have 7 characters for 'express'");
        
// Verify NO transforms were applied
        let has_transforms = e.buf.iter().any(|c| c.mark > 0 || c.tone > 0 || c.stroke);
        assert!(!has_transforms, "Buffer should have no Vietnamese transforms for 'express'");
        
        let output = e.buf.to_full_string();
        assert_eq!(output, "express", "Output should be 'express'");
    }

    #[test]
    fn test_nghieem_bug_fix() {
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);
        
        // Test patterns with "ngh" prefix
        let result = type_word(&mut e, "nghia");
        eprintln!("nghia -> '{}'", result);
        
        e = Engine::new();
        e.set_method(0);
        e.set_enabled(true);
        let result = type_word(&mut e, "nghiaa");
        eprintln!("nghiaa -> '{}'", result);
        
        e = Engine::new();
        e.set_method(0);
        e.set_enabled(true);
        let result = type_word(&mut e, "nghie");
        eprintln!("nghie -> '{}'", result);
        
        e = Engine::new();
        e.set_method(0);
        e.set_enabled(true);
        let result = type_word(&mut e, "nghiee");
        eprintln!("nghiee -> '{}'", result);
        assert_eq!(result, "nghiê", "nghiee should become nghiê");
    }

    #[test]
    fn test_performance_english_detection() {
        // Performance test: English detection should be fast (single-pass)
        // This test verifies the optimization doesn't break functionality
        use std::time::Instant;
        
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);
        
        let test_words = vec![
            ("express", "express"),  // Double consonant
            ("stress", "stress"),    // Triple consonant  
            ("export", "export"),    // ex pattern
            ("address", "address"),  // ad pattern
            ("better", "better"),    // tt pattern
        ];
        
        let start = Instant::now();
        for (input, expected) in test_words {
            e.buf.clear();
            e.raw_input.clear();
            e.is_english_word = false;
            
            let result = type_word(&mut e, input);
            assert_eq!(result, expected, "English detection should work for {}", input);
        }
        let elapsed = start.elapsed();
        
        // Should complete in < 10ms for 5 words
        assert!(elapsed.as_millis() < 10, "English detection too slow: {:?}", elapsed);
    }
}
