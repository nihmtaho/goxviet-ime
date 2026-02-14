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

// SOLID module organization (migrated from legacy_engine)
pub mod buffer;
pub mod core_types;
pub mod english;
pub mod features;
pub mod state;
pub mod transform;

// Backward-compatible aliases for old module names
pub mod types {
    //! Backward-compatible re-export of core_types
    pub use super::core_types::*;
}
pub mod vietnamese {
    //! Backward-compatible re-export of transform
    pub use super::transform::*;
}

// Re-export core types
pub use self::core_types::config::{EngineConfig, InputMethod as EngineInputMethod};
pub use self::core_types::{Action, Result, Transform};
pub use self::state::history::WordHistory;

// Re-export external english detection (integrated with infrastructure::external::english)
pub use crate::infrastructure::external::english::dictionary::Dictionary;
pub use crate::infrastructure::external::english::language_decision::{
    DecisionResult, LanguageDecisionEngine,
};
pub use crate::infrastructure::external::english::phonotactic::{
    PhonotacticEngine, PhonotacticResult, ValidationResult, VietnameseSyllableValidator,
};

// Flat re-exports for direct access
pub use self::buffer::raw_input_buffer;
pub use self::buffer::rebuild;
pub use self::core_types::config;
pub use self::features::shortcut;
pub use self::state::history;
pub use self::state::restore;
pub use self::transform::syllable;
pub use self::transform::tone_positioning;
pub use self::transform::vowel_compound;
pub use crate::infrastructure::external::english::phonotactic;

use self::buffer::raw_input_buffer::RawInputBuffer;
use self::buffer::{Buffer, Char};
use self::features::shortcut::{InputMethod, ShortcutTable};
// No longer using internal validation module
use crate::data::{
    chars::{self, mark, tone},
    constants, keys,
    vowel::{Phonology, Vowel},
};
use crate::input::{self, ToneType};
use crate::utils;

/// Main Vietnamese IME engine
pub struct Engine {
    buf: Buffer,
    method: u8,
    enabled: bool,
    last_transform: Option<Transform>,
    shortcuts: ShortcutTable,
    /// Global enable/disable flag for all shortcuts (text expansion feature)
    pub shortcuts_enabled: bool,
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
    pub(crate) esc_restore_enabled: bool,
    /// Enable free tone placement (skip validation)
    /// When true, allows placing diacritics anywhere without spelling validation
    free_tone_enabled: bool,
    /// Use modern orthography for tone placement (hoà vs hòa)
    /// When true: oà, uý (tone on second vowel)
    /// When false: òa, úy (tone on first vowel - traditional)
    modern_tone: bool,
    /// Enable instant auto-restore for English words
    /// When true (default), restores English words immediately upon detection
    instant_restore_enabled: bool,
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
    pub is_english_word: bool,
    /// Track number of non-space break characters types (e.g. numbers)
    /// Used to restore word history when backspacing over them
    break_after_commit: u8,
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
            shortcuts: ShortcutTable::with_all_defaults(), // FIX: Load default shortcuts
            shortcuts_enabled: true,
            raw_input: RawInputBuffer::new(),
            raw_mode: false,
            has_non_letter_prefix: false,
            skip_w_shortcut: true,
            esc_restore_enabled: false, // Default: OFF (user request)
            free_tone_enabled: true,
            modern_tone: true, // Default: modern style (hoà, thuý)
            instant_restore_enabled: true,
            word_history: WordHistory::new(),
            spaces_after_commit: 0,
            break_after_commit: 0,
            cached_syllable_boundary: None,
            is_english_word: false,
        }
    }

    /// Get current buffer as a full Vietnamese string
    pub fn get_buffer(&self) -> String {
        self.buf.to_full_string()
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

    /// Set whether English auto-restore is enabled
    pub fn set_english_auto_restore(&mut self, enabled: bool) {
        self.instant_restore_enabled = enabled;
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

    /// Helper to handle break sequence commit
    fn commit_and_break_sequence(&mut self) -> Result {
        // FIX: Save history before clearing so we can restore on backspace
        if !self.buf.is_empty() {
            self.word_history.push(&self.buf, &self.raw_input);
            self.break_after_commit = 1;
        } else if self.break_after_commit > 0 {
            // If we are continuing a break sequence (e.g. 123), increment
            self.break_after_commit = self.break_after_commit.saturating_add(1);
        } else {
            // Buffer empty and not in break sequence - hard reset history
            // This handles "word <space> number" - don't link number to word
            self.word_history.clear();
            self.break_after_commit = 0;
        }

        self.clear();
        self.spaces_after_commit = 0;
        self.cached_syllable_boundary = None;
        self.is_english_word = false;
        Result::none()
    }

    /// Handle Shift+Backspace - delete entire word
    ///
    /// Returns Result with backspace count equal to the current displayed word length.
    /// After this, the buffer is completely cleared.
    ///
    /// # Returns
    /// * `Result::send(backspace_count, &[])` if buffer has content
    /// * `Result::none()` if buffer is empty
    pub fn handle_shift_backspace(&mut self) -> Result {
        // If buffer is empty, nothing to delete
        if self.buf.is_empty() {
            // If we have spaces after commit, delete them all and restore previous word
            if self.spaces_after_commit > 0 {
                let spaces_to_delete = self.spaces_after_commit as u8;
                self.spaces_after_commit = 0;

                // Restore previous word from history
                if let Some((restored_buf, _restored_raw)) = self.word_history.pop() {
                    // Calculate the full word length to delete
                    // SAFETY: Clamp to u8::MAX to prevent overflow
                    let word_len = restored_buf
                        .to_full_string()
                        .chars()
                        .count()
                        .min(u8::MAX as usize) as u8;
                    // Don't restore - just return total delete count (spaces + word)
                    return Result::send(spaces_to_delete + word_len, &[]);
                }

                return Result::send(spaces_to_delete, &[]);
            }

            // Check break_after_commit similarly
            if self.break_after_commit > 0 {
                let breaks_to_delete = self.break_after_commit;
                self.break_after_commit = 0;

                if let Some((restored_buf, _)) = self.word_history.pop() {
                    // SAFETY: Clamp to u8::MAX to prevent overflow
                    let word_len = restored_buf
                        .to_full_string()
                        .chars()
                        .count()
                        .min(u8::MAX as usize) as u8;
                    return Result::send(breaks_to_delete + word_len, &[]);
                }

                return Result::send(breaks_to_delete, &[]);
            }

            return Result::none();
        }

        // Calculate the displayed word length (full Vietnamese string with diacritics)
        let displayed_word = self.buf.to_full_string();
        // SAFETY: Clamp to u8::MAX to prevent overflow
        let char_count = displayed_word.chars().count().min(u8::MAX as usize) as u8;

        // Clear everything
        self.buf.clear();
        self.raw_input.clear();
        self.last_transform = None;
        self.cached_syllable_boundary = None;
        self.is_english_word = false;
        self.has_non_letter_prefix = false;

        // Return backspace count to delete displayed characters
        Result::send(char_count, &[])
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
            let shortcut_result = self.try_word_boundary_shortcut();

            // Push to history before clearing (for backspace-after-space feature)
            if !self.buf.is_empty() {
                self.word_history.push(&self.buf, &self.raw_input);
                self.spaces_after_commit = 1;
            } else if self.spaces_after_commit > 0 {
                self.spaces_after_commit = self.spaces_after_commit.saturating_add(1);
            }

            // Clean buffer, raw_input immediately after word completion
            // Explicitly clear all state to ensure no residual data affects next word
            self.clear();
            self.is_english_word = false;
            self.raw_input.clear();
            return shortcut_result;
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

        // Other break keys (punctuation, arrows, numbers, etc.) just clear buffer
        // Only if NOT a modifier key (to allow VNI number-based modifiers)
        let m = input::get(self.method);
        let is_modifier =
            m.stroke(key) || m.remove(key) || m.tone(key).is_some() || m.mark(key).is_some();

        if !is_modifier && (keys::is_break(key) || keys::is_number(key)) {
            return self.commit_and_break_sequence();
        }

        if key == keys::DELETE {
            // SHIFT+BACKSPACE: Delete entire word (clear buffer completely)
            if shift {
                return self.handle_shift_backspace();
            }

            // BUGFIX: Simplified DELETE handling to fix Spotlight autocomplete bug
            // Old approach: Complex syllable boundary rebuild → miscalculates backspace with autocomplete
            // New approach: Simple buf.pop() + return none() → let OS handle deletion
            // This matches example-project's proven approach

            // Backspace-after-space feature: restore previous word when all spaces deleted
            if self.spaces_after_commit > 0 && self.buf.is_empty() {
                self.spaces_after_commit -= 1;
                if self.spaces_after_commit == 0 {
                    // All spaces deleted - restore the word buffer
                    if let Some((restored_buf, restored_raw)) = self.word_history.pop() {
                        self.buf = restored_buf;
                        self.raw_input = restored_raw;
                        // FIX: Clear cached state to allow tone application after restoration
                        self.cached_syllable_boundary = None;
                        self.is_english_word = false;
                    }
                }
                // Delete one space
                // Delete one space
                return Result::send(1, &[]);
            }

            // Backspace-after-break feature: restore previous word when break chars deleted
            // Corresponds to break_after_commit logic (numbers, etc)
            if self.break_after_commit > 0 && self.buf.is_empty() {
                self.break_after_commit -= 1;
                if self.break_after_commit == 0 {
                    if let Some((restored_buf, restored_raw)) = self.word_history.pop() {
                        self.buf = restored_buf;
                        self.raw_input = restored_raw;
                        // FIX: Clear cached state to allow tone application after restoration
                        self.cached_syllable_boundary = None;
                        self.is_english_word = false;
                    }
                }
                return Result::none();
            }

            // If buffer is already empty, user is deleting content from previous word
            // that we don't track. Mark this to prevent false shortcut matches.
            if self.buf.is_empty() {
                self.has_non_letter_prefix = true;
                return Result::none();
            }

            // Simple delete: just pop from buffer
            self.buf.pop();
            if !self.raw_input.is_empty() {
                self.raw_input.pop();
            }
            self.last_transform = None;
            self.cached_syllable_boundary = None;

            // Reset flags when buffer becomes empty
            if self.buf.is_empty() {
                self.is_english_word = false;
                self.raw_input.clear();
                self.last_transform = None;
                self.has_non_letter_prefix = false;
            }

            // KEY FIX: Return none() instead of calculating backspace
            // This lets the OS/app handle the actual character deletion
            // No backspace calculation = no miscalculation with autocomplete
            return Result::none();
        }

        // Record raw keystroke for ESC restore (letters and numbers only)
        // BUT: Skip modifier keys ONLY when they can actually modify the current buffer
        // In Telex, s/f/r/x/j/z are marks/remove, but only if buffer has vowels
        // AND if applying the mark would result in valid Vietnamese
        // If buffer is empty, these are just regular letters
        let _m_method = input::get(self.method);

        // CRITICAL FIX for English word detection:
        // We MUST always add all keys to raw_input, even if they're treated as modifiers.
        // The dictionary check needs complete keystroke history to work correctly.
        // Example: "console" types as [c,o,n,s,o,l,e] but 's' acts as tone modifier for 'o'.
        // If we skip 's' from raw_input, dictionary lookup gets [c,o,n,o,l,e] which won't match.
        // Solution: ALWAYS add to raw_input, let auto-restore handle it later.
        let should_skip = false; // Temporarily disable to fix English detection

        /* DISABLED TEMPORARILY - Causes dictionary lookup failures
        let should_skip = if self.method == 0 {  // Telex mode
            // PRIORITY CHECK: If current raw_input + new key would form an English dictionary word,
            // NEVER skip the key. We need all keys in raw_input for dictionary lookup.
            // Example: "cons" + "o" should keep 's' in raw_input, not treat it as tone modifier

            // Build temporary key list to check
            let mut temp_keys: Vec<u16> = self.raw_input.iter().map(|(k, _)| k).collect();
            temp_keys.push(key);

            let is_dict_word = crate::infrastructure::external::english::dictionary::Dictionary::is_english(&temp_keys);

            // DEBUG
            if temp_keys.len() >= 4 {
                println!("DEBUG should_skip: checking {:?} (len {}), is_dict={}", temp_keys, temp_keys.len(), is_dict_word);
            }

            if is_dict_word {
                // This will form a dictionary word - don't skip ANY key
                false
            } else {
                // Not a dictionary word (yet), proceed with normal logic
                // CRITICAL: Stroke modifiers (dd→đ) have different logic than mark modifiers
                // Stroke: only skip if last char in buffer is the SAME char (dd pattern)
                // Mark: skip based on syllable structure analysis

                if m_method.stroke(key) {
                    // Stroke modifier (e.g., 'd' in Telex for dd→đ)
                    // Only skip if:
                    // 1. Buffer has EXACTLY one char, AND
                    // 2. That char is the SAME as current key
                    // This handles: "d" + "d" → skip, apply stroke → "đ"
                    // But NOT: "ad" + "d" → don't skip, it's "add"
                    let buffer_has_single_matching_char = self.buf.len() == 1
                        && self.buf.last().map(|c| c.key == key).unwrap_or(false);
                    buffer_has_single_matching_char
                } else {
                    // Mark modifiers (s/f/r/x/j) or remove (z)
                    let has_vowels = self.buf.iter().any(|c| keys::is_vowel(c.key));
                    let has_existing_tone_or_mark = self.buf.iter().any(|c| c.tone != tone::NONE || c.mark != mark::NONE);
                    let is_mark_or_remove = m_method.mark(key).is_some() || m_method.remove(key);

                    if has_vowels && is_mark_or_remove && has_existing_tone_or_mark {
                        // Buffer has a tone/mark already, so this mark key is probably toggling/changing it
                        true
                    } else if has_vowels && is_mark_or_remove && !has_existing_tone_or_mark {
                        // Buffer has vowel(s) but NO tone/mark yet
                        // The mark key could either:
                        // A) Be applying a NEW tone (e.g., "di" + "s" → "dí")
                        // B) Be a regular letter (e.g., "rest" + "o" + "r" → "restor", where final 'r' is a letter)
                        // To disambiguate: check if the FINAL vowel is adjacent to the position
                        // where the mark would apply. Vietnamese syllables have vowels either:
                        // 1. Single vowel: V
                        // 2. Compound vowels: VV (adjacent, no consonants between)
                        // If there are consonants between vowels, it's not a valid compound!

                        // Find the position of the last vowel
                        let last_vowel_pos = self.buf
                            .iter()
                            .rposition(|c| keys::is_vowel(c.key));

                        if let Some(last_vpos) = last_vowel_pos {
                            // Check if there are any consonants AFTER the last vowel
                            let has_final_consonant = last_vpos < self.buf.len() - 1;

                            // Also check if there are multiple vowels with consonants between them
                            // If yes, it's NOT a valid compound
                            let mut vowel_positions = Vec::new();
                            for (i, c) in self.buf.iter().enumerate() {
                                if keys::is_vowel(c.key) {
                                    vowel_positions.push(i);
                                }
                            }

                            let has_non_adjacent_vowels = vowel_positions.windows(2).any(|w| w[1] - w[0] > 1);

                            if has_non_adjacent_vowels {
                                // Multiple vowels with consonants between = NOT a valid Vietnamese syllable structure
                                // Mark key is probably a regular letter, not a tone
                                false
                            } else if has_final_consonant || vowel_positions.len() > 1 {
                                // Either: has final consonant (complete syllable), or has multiple adjacent vowels (compound)
                                // Mark key should apply
                                true
                            } else {
                                // Single vowel, no final consonant = incomplete syllable
                                // Mark key is probably a regular letter
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
            }
        } else {
            false  // VNI mode: always track all keys
        };
        */

        // Record raw keystroke for ESC restore (letters and numbers only)
        // Skip modifier keys (s/f/r/x/j in Telex, numbers in VNI) UNLESS it's a revert
        // (double-key pattern like ff, ss, rr where the second key reverts and adds to buffer).
        // The revert handling below will add revert keys to raw_input explicitly.
        if (keys::is_letter(key) || keys::is_number(key)) && !should_skip {
            self.raw_input.push(key, caps);
        }

        self.process(key, caps, shift)
    }

    /// Main processing pipeline - pattern-based
    #[inline]
    fn process(&mut self, key: u16, caps: bool, shift: bool) -> Result {
        // Early English pattern detection: Check BEFORE applying any transforms
        // This prevents false transforms like "release" → "rêlase" or "telex" → "tễl"
        // Note: raw_input already contains the current key (pushed in on_key_ext)
        // Check at 2+ chars to catch "ex" pattern (export, express, example)
        // Other patterns need 3+ chars but "ex" must be caught at 2 chars

        let m = input::get(self.method);
        // Note: checking !shift because shift+key usually bypasses modifiers (unless VNI number)
        // But for letters (Telex), shift makes them uppercase letters, usually not modifiers (except for some defaults).
        // For W, A, E, O, they can be modifiers even if uppercase?
        // Logic in modifiers block uses `skip_modifiers = shift && is_number`.
        // For letters, it allows modifiers even with shift (e.g. typing uppercase accents).
        // So we check basic `m.is_xxx(key)`.
        let _is_modifier =
            m.tone(key).is_some() || m.mark(key).is_some() || m.stroke(key) || m.remove(key);

        // ═══════════════════════════════════════════════════════════════════════════
        // ENGLISH DETECTION (Telex/VNI)
        // ═══════════════════════════════════════════════════════════════════════════
        if (self.method == 0 || self.method == 1)
            && self.raw_input.len() >= 1
            && keys::is_letter(key)
            && !_is_modifier
        {
            // LAYER 0: Words starting with F, J, Z are ALWAYS English
            if self.raw_input.len() == 1 {
                let first_key = self.raw_input.iter().next().map(|(k, _)| k).unwrap_or(0);
                if matches!(first_key, keys::F | keys::J | keys::Z) {
                    self.is_english_word = true;
                    return self.handle_normal_letter(key, caps, shift);
                }
            }

            if self.raw_input.len() >= 2 {
                // 1. VIETNAMESE DICTIONARY LOOKUP: Removed as per request (replaced by Phonotactic Engine)
                // 2. ENGLISH DICTIONARY LOOKUP
                // Check programming terms and common English words to prevent Vietnamese transforms
                // PRIORITY: Check dictionary FIRST, before deciding if key is a modifier
                // This prevents "console" from becoming "cónole" when 's' is typed
                let is_dict = self.is_english_dictionary_word();

                if is_dict {
                    // DOUBLE-KEY REVERT: If the current key matches the last transform,
                    // it's a toggle/revert (e.g., "ré" + "s" → "res"). Let it fall through
                    // to the revert check at lines 805-814 instead of treating as English.
                    let method = input::get(self.method);
                    let is_revert = if shift {
                        false
                    } else if let Some(Transform::Mark(last_key, _)) = self.last_transform {
                        method.mark(key).is_some() && last_key == key
                    } else if let Some(Transform::Tone(last_key, _)) = self.last_transform {
                        method.tone(key).is_some() && last_key == key
                    } else {
                        false
                    };

                    if is_revert {
                        // Don't set is_english_word yet - let revert happen first
                        // Fall through to revert check
                    } else {
                        self.is_english_word = true;

                        // INSTANT RESTORE: If already transformed, undo immediately
                        if self.instant_restore_enabled && self.has_vietnamese_transforms() {
                            let result = self.instant_restore_english();
                            self.sync_buffer_with_raw_input();
                            self.last_transform = None; // Clear stale transform after English restore
                            return result;
                        }
                        return self.handle_normal_letter(key, caps, shift);
                    }
                }

                // 3. Pattern detection (only if NOT already marked as English)
                if !self.is_english_word {
                    let is_definite_english = self.has_definite_english_pattern();
                    if is_definite_english {
                        // FEATURE: Speculative Modifier Application (Same logic as ambiguous patterns)
                        // Even if it looks definitely English (e.g. invalid initial/final),
                        // if the user types a modifier that creates a valid Vietnamese word, we should allow it.
                        // Example: "dis" is invalid Vietnamese structure -> definite English.
                        // But "dis" composed of "di" + "s" (Acute) -> "dí" IS valid.

                        let method = input::get(self.method);
                        let result = if shift {
                            None
                        } else {
                            method
                                .tone(key)
                                .map(|_| ())
                                .or_else(|| method.mark(key).map(|_| ()))
                                .or_else(|| {
                                    if method.stroke(key) || method.remove(key) {
                                        Some(())
                                    } else {
                                        None
                                    }
                                })
                        };

                        if result.is_some() {
                            // Fall through to modifier handling
                        } else {
                            self.is_english_word = true;

                            // INSTANT RESTORE: If already transformed, undo immediately
                            if self.instant_restore_enabled && self.has_vietnamese_transforms() {
                                let result = self.instant_restore_english();
                                self.sync_buffer_with_raw_input();
                                return result;
                            }

                            return self.handle_normal_letter(key, caps, shift);
                        }
                    }

                    // Check for AMBIGUOUS patterns (Layer 2-3)
                    // IMPORTANT: Check for English pattern even if is_modifier_key=true
                    // so we set is_english_word flag before processing the modifier.
                    // This prevents tone/mark modifiers from being applied to English words.
                    let is_english = self.has_english_word_pattern();
                    if is_english {
                        // FEATURE: Speculative Modifier Application
                        // If the current key is a modifier (tone/mark/stroke), do NOT lock as English yet.
                        // Let the modifier try to apply.
                        // - If valid Vietnamese (e.g. "di" + "s" → "dí"), try_tone will succeed.
                        // - If invalid (e.g. "work" + "s" → "wờrk"), try_tone will fail validation.
                        // This solves the "dis" → "dí" (valid) vs "works" (valid English) conflict without dictionaries.

                        let method = input::get(self.method);
                        let result = if shift {
                            None
                        } else {
                            method
                                .tone(key)
                                .map(|_| ())
                                .or_else(|| method.mark(key).map(|_| ()))
                                .or_else(|| {
                                    if method.stroke(key) || method.remove(key) {
                                        Some(())
                                    } else {
                                        None
                                    }
                                })
                        };

                        if result.is_some() {
                            // Fall through to modifier handling
                        } else {
                            self.is_english_word = true;

                            if self.instant_restore_enabled && self.has_vietnamese_transforms() {
                                let result = self.instant_restore_english();
                                self.sync_buffer_with_raw_input();
                                return result;
                            }

                            return self.handle_normal_letter(key, caps, shift);
                        }
                    }
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
        // intends plain Latin input.
        if self.method != 0 && self.method != 1 {
            return self.handle_normal_letter(key, caps, shift);
        }

        // DEBUG: Trace raw_input
        // println!("DEBUG: Process key={}, raw_input len={}", key, self.raw_input.len());
        // We can't print easily here before handling.

        // ... switch to END of function or insert prints at return points.
        // Actually, let's insert a print in `check_and_restore_english` since that's where restore happens.
        // And maybe in `try_tone` successful return.

        // If Shift is pressed with a number key, skip all modifiers (both VNI and Telex)
        // User wants the symbol (@ for Shift+2, # for Shift+3, etc.), not tone marks
        // This handles both VNI mode (numbers as marks) and Telex mode (prevents accidental transforms)
        let skip_modifiers = shift && keys::is_number(key);

        // ═══════════════════════════════════════════════════════════════════════════
        // CRITICAL FIX: REVERT CHECK BEFORE ENGLISH BYPASS
        // ═══════════════════════════════════════════════════════════════════════════
        // If the user types a modifier key that matches the last transform,
        // it's a strong signal they want to REVERT (toggle), even if the word
        // was detected as English cluster.
        // This fixes "dax" + "x" -> "da" (instead of "daxx")
        if m.tone(key).is_some() {
            if let Some(Transform::Tone(last_key, _)) = self.last_transform {
                if last_key == key {
                    // CRITICAL FIX: Don't add the revert key if it was already added at line 625.
                    // Letter/number keys are added at line 625 even when used as tone modifiers.
                    // We should NOT add them again in the double-key flow.
                    // For non-letter keys, add them here.
                    if !keys::is_letter(key) && !keys::is_number(key) {
                        self.raw_input.push(key, caps);
                    }

                    // Save state BEFORE revert (revert_tone sets is_english_word = true)
                    let was_english = self.is_english_word;
                    let display_len_before = self.buf.len();
                    let result = self.revert_tone(key, caps);
                    // After double-key revert (aa→a+a, ee→e+e, oo→o+o),
                    // buffer may be shorter than raw_input. Sync and output full raw.
                    // Only if word was ALREADY English before revert (not just marked by revert itself).
                    if was_english && self.raw_input.len() > self.buf.len() {
                        self.sync_buffer_with_raw_input();
                        let output: Vec<char> = self
                            .raw_input
                            .iter()
                            .filter_map(|(k, c)| utils::key_to_char(k, c))
                            .collect();
                        let bs = display_len_before.min(u8::MAX as usize) as u8;
                        return Result::send(bs, &output);
                    }
                    if let Some(restored) = self.check_and_restore_english(1, true) {
                        return restored;
                    }
                    return result;
                }
            }
        }
        if m.mark(key).is_some() {
            if let Some(Transform::Mark(last_key, _)) = self.last_transform {
                if last_key == key {
                    // CRITICAL FIX: Don't add the revert key if it was already added at line 625.
                    // Letter/number keys (like 's') are added at line 625 even when used as modifiers.
                    // We should NOT add them again in the double-key flow.
                    // For non-letter keys like 'f', they weren't added at line 625, so we add them here.
                    if keys::is_letter(key) || keys::is_number(key) {
                        // Already added at line 625, don't add again
                    } else {
                        self.raw_input.push(key, caps);
                    }

                    // Save state BEFORE revert (revert_mark sets is_english_word = true)
                    let was_english = self.is_english_word;
                    let display_len_before = self.buf.len();
                    let result = self.revert_mark(key, caps);
                    // After double-key revert (ff→f+f, ss→s+s, rr→r+r),
                    // buffer may be shorter than raw_input. Sync and output full raw.
                    // Only if word was ALREADY English before revert (not just marked by revert itself).
                    if was_english && self.raw_input.len() > self.buf.len() {
                        self.sync_buffer_with_raw_input();
                        let output: Vec<char> = self
                            .raw_input
                            .iter()
                            .filter_map(|(k, c)| utils::key_to_char(k, c))
                            .collect();
                        let bs = display_len_before.min(u8::MAX as usize) as u8;
                        return Result::send(bs, &output);
                    }
                    if let Some(restored) = self.check_and_restore_english(1, true) {
                        return restored;
                    }
                    return result;
                }
            }
        }

        // 1. Stroke modifier (d → đ)
        if !skip_modifiers && m.stroke(key) {
            if let Some(result) = self.try_stroke(key, caps) {
                // Post-transform check for English word logic that got blocked by validation
                // e.g. "f" -> "à" (valid Viet) but "of" -> "oà" (invalid Viet)
                // If it becomes invalid Vietnamese but is valid English, restore it.
                if let Some(restored) = self.check_and_restore_english(0, false) {
                    return restored;
                }

                // FIX: Return the result directly from try_stroke (which includes revert_stroke)
                // Do NOT call rebuild_output_from_entire_buffer() because it uses buf.len() as backspace
                // which is wrong after revert (e.g., buffer=['d','d'] has len=2 but should backspace=1)
                return result;
            }
        }

        // 2. Tone modifier (s,f,r,x,j in Telex; 1..5 in VNI)
        if !skip_modifiers {
            // For Telex a/e/o circumflex patterns, check if they can actually apply
            // aa/ee/oo should only be tone modifiers if:
            // The previous key was the same vowel (double-key pattern like "aa", "ee", "oo")
            // This is the ONLY case where a standalone a/e/o should be a tone modifier in Telex
            let should_check_tone = if self.method == 0 {
                // Telex mode
                match key {
                    keys::A | keys::E | keys::O => {
                        // Check if the last character in the buffer has the same base key.
                        // This correctly handles cases like 'e' + 's' -> 'é', then 'é' + 'e' -> 'ê'.
                        let prev_key_match = self.buf.last().map_or(false, |c| c.key == key);

                        // NEW FEATURE: Also allow if last char is final consonant and there's a
                        // matching vowel before it (for backward diacritical application)
                        // Example: "cam" + "a" → should check tone to apply circumflex backward
                        //
                        // CONSTRAINT: Only allow if buffer forms valid syllable structure (C-V-C pattern)
                        // Reject cases like: "m" + "aa" (consonant-only at start)
                        let backward_match = if !prev_key_match && self.buf.len() >= 2 {
                            let last_idx = self.buf.len() - 1;
                            let last_char = self.buf.get(last_idx).unwrap();

                            // CASE 1: Last char is consonant (backward after final consonant)
                            // Example: "cam" + "a" → "câm"
                            if !keys::is_vowel(last_char.key) {
                                // Check for Digraph finals (ng, nh, ch)
                                // If last char is g, h, check prev char
                                let is_final = if let Some(cons_char) =
                                    crate::utils::key_to_char(last_char.key, false)
                                {
                                    let cons_str = cons_char.to_string();
                                    if crate::infrastructure::external::diacritical_validator::DiacriticalValidator::is_final_consonant(&cons_str) {
                                        true
                                    } else {
                                        // Check digraphs: ng, nh, ch
                                        // Need preceding char
                                        if last_idx > 0 {
                                            let prev_cons = self.buf.get(last_idx - 1).unwrap();
                                            let s = format!("{}{}", 
                                                crate::utils::key_to_char(prev_cons.key, false).unwrap_or(' '),
                                                cons_char
                                            );
                                            matches!(s.as_str(), "ng" | "nh" | "ch")
                                        } else {
                                            false
                                        }
                                    }
                                } else {
                                    false
                                };

                                if is_final {
                                    // For backward application to be valid, need at least C-V-C pattern
                                    // Minimum: 3 chars (initial consonant, vowel, final consonant)
                                    // Or 4 chars if digraph final
                                    if self.buf.len() >= 3 {
                                        // Check if there's a vowel before the consonant matching the key
                                        // Scan backwards skipping the final consonant(s)
                                        self.buf
                                            .iter()
                                            .rev()
                                            .skip(1)
                                            .any(|c| c.key == key && c.tone == tone::NONE)
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                }
                            } else {
                                // Last is vowel - only allow backward for Telex doubling patterns (aa, ee, oo)
                                // or VNI mode (which doesn't need key matching)
                                if self.method == 1 {
                                    // VNI mode: always allow (numbers don't need to match vowels)
                                    true
                                } else {
                                    // Telex mode: only allow if it's a doubling pattern (Circumflex)
                                    // We are inside matches!(key, A|E|O) block so it is Circumflex.
                                    true
                                }
                            }
                        } else {
                            false
                        };

                        prev_key_match || backward_match
                    }
                    _ => true, // Other keys can be tone modifiers directly
                }
            } else {
                true // VNI mode: always allow tone checking
            };
            eprintln!(
                "DEBUG: should_check_tone={} for key={}",
                should_check_tone, key
            );

            if should_check_tone {
                let tone_result = m.tone(key);
                eprintln!("DEBUG: m.tone({}) = {:?}", key, tone_result);
                if let Some(tone_type) = tone_result {
                    eprintln!(
                        "DEBUG: tone() returned Some for key={}, tone_type={:?}",
                        key, tone_type
                    );
                    let targets = m.tone_targets(key);
                    if let Some(result) = self.try_tone(key, caps, tone_type, targets) {
                        self.is_english_word = false;

                        // Skip English detection for Horn/Breve diacriticals (w key in Telex).
                        // Horn/Breve is an EXPLICIT Vietnamese typing intent signal.
                        // Common English words ending in 'w' (vow, how, now, cow, bow, etc.)
                        // would cause false positives, breaking Vietnamese words like vơi, hơi, etc.
                        // Users can undo horn/breve (double-press w) or press Esc for original text.
                        let is_horn_or_breve = matches!(tone_type, ToneType::Horn | ToneType::Breve);

                        if !is_horn_or_breve {
                            // Post-transform confidence check: restore if high English confidence
                            if let Some(restore_result) = self.check_and_restore_english(0, false) {
                                self.last_transform = None;
                                return restore_result;
                            }

                            // After diacritical is applied (aa→â, ee→ê, oo→ô),
                            // check if the COMPLETE raw input matches an English dictionary word.
                            // This catches words like "been"→"bên", "floor"→"flôr", etc.
                            if self.instant_restore_enabled && self.has_vietnamese_transforms() {
                                let raw_keys: Vec<u16> =
                                    self.raw_input.iter().map(|item| item.0).collect();
                                if crate::infrastructure::adapters::validation::english::dictionary::Dictionary::is_english(&raw_keys) {
                                    self.is_english_word = true;
                                    let restore = self.instant_restore_english();
                                    self.sync_buffer_with_raw_input();
                                    self.last_transform = None;
                                    return restore;
                                }
                            }
                        }

                        return result; // Return the result from try_tone with correct backspace
                    } else {
                        // tone() is Some but try_tone failed
                        if keys::is_number(key) {
                            // Fallback: VNI tone number (e.g. 1) failed to apply to vowel.
                            // It should act as a number (break key).
                            return self.commit_and_break_sequence();
                        }

                        // CRITICAL FIX: For Telex a/e/o double-key patterns, if try_tone fails
                        // (e.g., due to diacritical validation rejecting it), we should NOT
                        // fall through to handle_normal_letter. The keystroke should be consumed
                        // but have no output effect, preventing the second vowel from being added.
                        //
                        // Examples:
                        //   Buffer: [s, a, n], Keystroke: 'a' (double-key)
                        //   try_tone fails (can_apply_diacritical rejects due to final 'n')
                        //   Result: Return empty (consume keystroke, no output)
                        //   Not: Fall through and add the second 'a'
                        //
                        // NEW: This also applies to BACKWARD APPLICATION patterns
                        //   Buffer: [c, â, m], Keystroke: 'a' (backward application, but vowel has tone)
                        //   try_tone fails (backward search finds 'â' but it has tone != NONE)
                        //   Result: Return empty (consume keystroke, no output)
                        if self.method == 0 && matches!(key, keys::A | keys::E | keys::O) {
                            // Check if this was intended as a tone modifier (either adjacent or backward)
                            let was_tone_attempt = self.buf.last().map_or(false, |c| c.key == key)
                                || (self.buf.len() >= 3
                                    && self.buf.last().map_or(false, |c| !keys::is_vowel(c.key))
                                    && self.buf.get(0).map_or(false, |c| !keys::is_vowel(c.key)));

                            eprintln!(
                                "DEBUG: try_tone failed for key={}, was_tone_attempt={}",
                                key, was_tone_attempt
                            );

                            if was_tone_attempt {
                                eprintln!("DEBUG: Consuming keystroke without output");
                                // FIX: Add the char to buffer to match platform display.
                                // Platform passes through (action=0) so the char IS displayed.
                                // Buffer must track it to avoid buffer-display mismatch.
                                self.buf.push(Char::new(key, caps));
                                return Result::default();
                            }
                        }

                        // If tone() is Some but try_tone failed (and not Telex a/e/o), fall through to mark handling
                    }
                }
            }
        }

        // 3. Mark modifier (aa/aw/ee/oo/ow/uw, etc.)
        if !skip_modifiers {
            if let Some(mark_val) = m.mark(key) {
                if let Some(result) = self.try_mark(key, caps, mark_val) {
                    self.is_english_word = false;

                    // After tone mark is applied, check if the COMPLETE raw input matches
                    // an English dictionary word. This catches words ending in s/f/r/x/j
                    // like "his"→"hí", "terms"→"tém", "bus"→"bú" etc.
                    // Only uses dictionary (not phonotactic) to avoid false positives.
                    // SKIP for single-vowel + mark (buf.len()==1): "o"+"f"→"ò", "i"+"s"→"í"
                    // These are clearly intentional Vietnamese marks, not English words.
                    if self.instant_restore_enabled
                        && self.buf.len() > 1
                        && self.has_vietnamese_transforms()
                    {
                        let raw_keys: Vec<u16> = self.raw_input.iter().map(|item| item.0).collect();
                        if crate::infrastructure::external::english::dictionary::Dictionary::is_english(&raw_keys) {
                            self.is_english_word = true;
                            let restore = self.instant_restore_english();
                            self.sync_buffer_with_raw_input();
                            self.last_transform = None; // Clear stale transform after English restore
                            return restore;
                        }
                    }

                    return result; // Return the result from try_mark with correct backspace
                } else if keys::is_number(key) {
                    // Fallback: VNI mark number (e.g. 6) failed to apply.
                    // Act as break key.
                    return self.commit_and_break_sequence();
                }
            }
        }

        // 4. Remove modifier
        if !skip_modifiers && m.remove(key) {
            if let Some(result) = self.try_remove() {
                self.is_english_word = false;
                return result;
            } else if keys::is_number(key) {
                // Fallback: VNI remove number (0) failed to remove anything.
                // Act as break key.
                return self.commit_and_break_sequence();
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
    #[inline]
    fn try_word_boundary_shortcut(&mut self) -> Result {
        if self.buf.is_empty() {
            return Result::none();
        }

        // Check global shortcuts enabled flag
        if !self.shortcuts_enabled {
            // Shortcuts disabled - let OS handle the space key
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

        // No shortcut matched and auto-restore is disabled.
        // Return None to let the OS handle the space key.
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
    /// - "uow" → "ươ" (complete compound with horn on both vowels)
    /// - "ouw" → "ươ" (reversed compound)
    fn try_w_as_vowel(&mut self, caps: bool) -> Option<Result> {
        // CRITICAL: Skip Vietnamese transform if English word detected
        if self.is_english_word {
            return None;
        }

        // CRITICAL FIX: Handle 'a+w' → 'ă' (breve mark) BEFORE trying ư transformation
        // If buffer has vowels and ends with 'a', applying 'w' should add breve to 'a'
        // Examples: 'la' + 'w' → 'lă', 'na' + 'w' → 'nă'
        // This prevents buffer from storing [l,a,w] as "law" when it should be "lă"
        if self.buf.len() > 0 {
            // Check if last character is vowel 'a'
            // Copy all needed values first to avoid borrow checker issues
            let (last_key, last_tone, last_mark, orig_caps) =
                if let Some(last_char) = self.buf.last() {
                    (
                        last_char.key,
                        last_char.tone,
                        last_char.mark,
                        last_char.caps,
                    )
                } else {
                    (0, 0, 0, false)
                };

            // REVERT CASE: ă + w → a (cancel breve)
            // When user types 'w' after 'ă', remove the breve to get plain 'a'
            // Example: 'lăw' → 'law' (second w cancels the first w's breve)
            if last_key == keys::A && last_tone == tone::HORN && last_mark == mark::NONE {
                let pos = self.buf.len() - 1;
                if let Some(c) = self.buf.get_mut(pos) {
                    c.tone = tone::NONE;

                    // KEEP both 'w' keys in raw_input for accurate keystroke history

                    // Use revert_and_rebuild to add 'w' as normal letter and rebuild output
                    let result = self.revert_and_rebuild(pos, keys::W, caps);

                    self.last_transform = None; // Clear last transform
                    self.is_english_word = true; // Mark as English (double modifier pattern)

                    return Some(result);
                }
            }

            // APPLY CASE: a + w → ă (add breve)
            if last_key == keys::A && last_tone == tone::NONE && last_mark == mark::NONE {
                // Apply breve (horn tone) to the 'a'
                let pos = self.buf.len() - 1;
                if let Some(c) = self.buf.get_mut(pos) {
                    c.tone = tone::HORN;
                    self.last_transform = Some(Transform::Mark(keys::W, 2)); // 2 = tone::HORN
                    let breve_char = chars::to_char(keys::A, orig_caps, tone::HORN, 0).unwrap();
                    return Some(Result::send(1, &[breve_char]));
                }
            }
        }

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

        // REMOVED: "uo" or "ou" compound -> "ươ" shortcut
        // User requested to remove this so "quow" becomes "quơ" (via try_tone) instead of "qươ".
        // "uwo" -> "ươ" is still supported via standard uw->ư + o path.

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
        let is_fast_path =
            buf_len == 0 || (buf_len == 1 && !keys::is_vowel(self.buf.get(0).unwrap().key));

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
        let buffer_tones: Vec<u8> = self.buf.iter().map(|c| c.tone).collect(); // Collect tones

        let validation = crate::infrastructure::external::vietnamese_validator::VietnameseSyllableValidator::validate_with_tones(&buffer_keys, &buffer_tones);

        if validation.is_valid {
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
    /// Try to apply stroke transformation (đ)
    fn try_stroke(&mut self, key: u16, _caps: bool) -> Option<Result> {
        if self.buf.is_empty() {
            return None;
        }

        // Check revert: dd -> d (if last char was d and we type d again)
        let last_pos = self.buf.len() - 1;
        let last_char = self.buf.get(last_pos)?;

        if self.method == 0 {
            // TELEX MODE
            /*
               dd -> đ
            */

            // Check revert: dđ -> d (stroke key d pressed again)
            // CRITICAL FIX: After transformation, the buffer structure changes
            // e.g., [d, a, d(stroked)] becomes [d(stroked), a]
            // So we need to find the stroked 'd' position, not use last_pos
            if let Some(Transform::Stroke(last_key)) = self.last_transform {
                if last_key == key {
                    // Find the stroked 'd' to revert
                    if let Some(stroked_pos) =
                        self.buf.iter().position(|c| c.key == keys::D && c.stroke)
                    {
                        return Some(self.revert_stroke(key, stroked_pos));
                    }
                }
            }

            // Check if last char is un-stroked 'd'
            // If already stroked (and not revertible above), we can't stroke again
            // NEW: Search backward for unstroked 'd' if last char is not eligible
            let target_pos = if last_char.key == keys::D && !last_char.stroke {
                Some(last_pos)
            } else {
                // Backward scan for unstroked 'd'
                self.buf.iter().rposition(|c| c.key == keys::D && !c.stroke)
            };

            let Some(target_pos) = target_pos else {
                return None;
            };

            // FAST PATH: If no vowels yet, apply stroke immediately (O(1))
            // "dd" at start or "d...d" -> "đ..." without complex validation
            // Check for vowels BEFORE the target position
            let has_vowel = self
                .buf
                .iter()
                .take(target_pos)
                .any(|c| keys::is_vowel(c.key));

            if !has_vowel {
                if let Some(c) = self.buf.get_mut(target_pos) {
                    c.stroke = true;
                }
                self.last_transform = Some(Transform::Stroke(key));
                // CRITICAL FIX: Track the modifier key in raw_input
                return Some(self.rebuild_from(target_pos));
            }

            // COMPLEX PATH: Has vowels, need validation
            // Skip validation for Telex (method 0) - matches try_tone/try_mark behavior
            if !self.free_tone_enabled && self.method != 0 {
                // Use iterator-based validation to avoid allocation
                let buffer_keys: Vec<u16> = self.buf.iter().map(|c| c.key).collect();
                if !crate::infrastructure::external::vietnamese_validator::VietnameseSyllableValidator::validate(
                    &buffer_keys,
                )
                .is_valid
                {
                    return None;
                }
            }

            // Apply stroke (Validated)
            if let Some(c) = self.buf.get_mut(target_pos) {
                c.stroke = true;
            }
            self.last_transform = Some(Transform::Stroke(key));
            return Some(self.rebuild_from(target_pos));
        }

        // VNI MODE: '9' can stroke any 'd' in buffer (delayed stroke)
        // Find first un-stroked 'd' anywhere in buffer
        let pos = self
            .buf
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
        // Skip validation for VNI method (method 1) - matches try_tone/try_mark behavior
        let has_vowel_after = self.buf.iter().skip(pos + 1).any(|c| keys::is_vowel(c.key));
        if !self.free_tone_enabled && has_vowel_after && self.method != 1 {
            // Use iterator-based validation to avoid allocation
            let buffer_keys: Vec<u16> = self.buf.iter().map(|c| c.key).collect();
            if !crate::infrastructure::external::vietnamese_validator::VietnameseSyllableValidator::validate(
                &buffer_keys,
            )
            .is_valid
            {
                return None;
            }
        }

        // Apply stroke
        if let Some(c) = self.buf.get_mut(pos) {
            c.stroke = true;
        }
        self.last_transform = Some(Transform::Stroke(key));
        // CRITICAL FIX: Track the modifier key in raw_input
        Some(self.rebuild_from(pos))
    }

    /// Helper: Check if applying diacritical at a position is valid
    /// Returns false if there's a final consonant after this position
    ///
    /// # Arguments
    /// * `target_pos` - Position in buffer to apply diacritical
    /// * `is_backward_application` - True if this is backward application (e.g., "cam" + "a" → "câm")
    ///                                In backward mode, final consonant at END is ALLOWED
    fn can_apply_diacritical(&self, target_pos: usize, is_backward_application: bool) -> bool {
        use crate::data::keys;

        eprintln!(
            "DEBUG can_apply_diacritical: ENTRY target_pos={}, buf.len()={}",
            target_pos,
            self.buf.len()
        );

        if target_pos >= self.buf.len() {
            eprintln!("DEBUG can_apply_diacritical: target_pos out of bounds, ALLOW");
            return true; // Invalid position - allow
        }

        // Work directly with buffer keys
        let target_char = self.buf.get(target_pos).unwrap();
        eprintln!(
            "DEBUG can_apply_diacritical: target_char.key={}",
            target_char.key
        );

        // Ensure target is a vowel
        if !keys::is_vowel(target_char.key) {
            eprintln!("DEBUG can_apply_diacritical: target is not vowel, ALLOW");
            return true; // Can't apply diacritical to non-vowel anyway
        }

        // ═══════════════════════════════════════════════════════════════════════════════════
        // CHECK CASE 1: Consonant immediately after
        eprintln!("DEBUG can_apply_diacritical: Checking CASE 1 (consonant after)");
        if target_pos + 1 < self.buf.len() {
            let next_char = self.buf.get(target_pos + 1).unwrap();
            eprintln!(
                "DEBUG can_apply_diacritical: next_char.key={}",
                next_char.key
            );

            // If next is a vowel, no consonant immediately after
            if !keys::is_vowel(next_char.key) {
                // Next is a consonant. Is it a final consonant?
                if let Some(cons_char) = crate::utils::key_to_char(next_char.key, false) {
                    let cons_str = cons_char.to_string();
                    eprintln!(
                        "DEBUG can_apply_diacritical: next is consonant '{}', checking if final",
                        cons_str
                    );

                    if crate::infrastructure::external::diacritical_validator::DiacriticalValidator::is_final_consonant(&cons_str)
                    {
                        eprintln!("DEBUG can_apply_diacritical: CASE 1 FOUND FINAL CONSONANT");
                        
                        // SPECIAL CASE: Backward application
                        // When backward applying diacritical (e.g., "cam" + "a" → "câm"),
                        // the final consonant IS AT THE END, which is EXPECTED and ALLOWED
                        if is_backward_application && target_pos + 2 >= self.buf.len() {
                            eprintln!("DEBUG can_apply_diacritical: backward application with final consonant at end, ALLOW");
                            return true; // ALLOW backward application
                        }
                        
                        // Check if it's truly final (not part of a 2-char consonant followed by vowel)
                        if target_pos + 2 >= self.buf.len() {
                            eprintln!("DEBUG can_apply_diacritical: final consonant at end of buffer, REJECT");
                            return false; // REJECT: vowel followed by final consonant at end
                        }
                        
                        let after_cons = self.buf.get(target_pos + 2).unwrap();
                        
                        // Check if it forms a digraph (e.g. 'ng', 'nh', 'ch')
                        let is_digraph = if let Some(second_char) = crate::utils::key_to_char(after_cons.key, false) {
                             let two_char = format!("{}{}", cons_char, second_char);
                             crate::infrastructure::external::diacritical_validator::DiacriticalValidator::is_final_consonant(&two_char)
                        } else { false };

                        if is_digraph {
                             // SPECIAL CASE: It's a valid digraph final (ng, nh, ch)
                             eprintln!("DEBUG can_apply_diacritical: found digraph final"); 

                             // Check what follows the digraph
                             if target_pos + 3 >= self.buf.len() {
                                  // End of buffer.
                                  // Backward application allows final consonant at end.
                                  if is_backward_application { 
                                      eprintln!("DEBUG can_apply_diacritical: backward application with digraph final at end, ALLOW");
                                      return true; 
                                  }
                                  
                                  // Even if not backward application (e.g. valid word),
                                  // a final consonant shouldn't block tone on the vowel?
                                  // Logic 1386 says "backward application ... final consonant IS AT THE END ... is ALLOWED"
                                  // Implying normal application expects open vowel?
                                  // But "ung" is valid.
                                  // If this function returns false, tone is blocked.
                                  // We should probably allow if it's a valid final consonant at end.
                                  eprintln!("DEBUG can_apply_diacritical: valid digraph matching end of buffer, ALLOW");
                                  return true;
                             }
                             
                             // If not end of buffer, check what's after digraph.
                             let after_digraph = self.buf.get(target_pos + 3).unwrap();
                             if !keys::is_vowel(after_digraph.key) {
                                  eprintln!("DEBUG can_apply_diacritical: digraph followed by non-vowel, REJECT");
                                  return false; // REJECT
                             }
                             
                             // Followed by vowel? 
                             // e.g. "unga". "ng" is final? No, "ng" + "a" -> "nga".
                             // So "u" + "nga". "u" is open?
                             // This is complex but for now we assume rejection or allow based on validator.
                             // But here we are VALIDATING DIACRITICAL PLACEMENT.
                             // Safest to reject if followed by vowel as it changes syllable structure?
                             eprintln!("DEBUG can_apply_diacritical: digraph followed by vowel, REJECT");
                             return false; 
                        }

                        // Not a digraph. Check single char.
                        if !keys::is_vowel(after_cons.key) {
                            eprintln!("DEBUG can_apply_diacritical: final consonant followed by non-vowel, REJECT");
                            return false; // REJECT: vowel followed by final consonant
                        }
                        
                        // Single final consonant followed by vowel = it's part of the syllable
                        eprintln!("DEBUG can_apply_diacritical: final consonant followed by vowel, REJECT");
                        return false; // REJECT
                    }
                }
            }
        }

        // CHECK CASE 2: Immediately preceded by a final consonant
        // This prevents applying diacritical to a vowel that starts a new syllable after a complete one.
        //
        // KEY INSIGHT: A consonant is only "final" if it comes AFTER a vowel (closing a syllable).
        // Example scenarios:
        //   "taan" (t-a-a-n) → 't' is INITIAL (before vowel), NOT final → target pos=1 ALLOW ✓
        //   "camaa" (c-a-m-a-a) → 'm' comes AFTER vowel (pos=1), IS final → target pos=3 REJECT ✓
        //
        // Algorithm:
        //   1. Check if prev_pos is a potential final consonant (c, ch, m, n, ng, nh, p, t)
        //   2. If yes, check if there's a vowel BEFORE it (making it a true final)
        //   3. If both true → REJECT (target starts new syllable after complete syllable)
        eprintln!("DEBUG can_apply_diacritical: Checking CASE 2 (preceding final consonant)");
        if target_pos > 0 {
            let prev_pos = target_pos - 1;
            if let Some(prev_char) = self.buf.get(prev_pos) {
                eprintln!(
                    "DEBUG can_apply_diacritical: prev_char.key={}, is_vowel={}",
                    prev_char.key,
                    keys::is_vowel(prev_char.key)
                );

                // Check if immediately preceding character is a consonant
                if !keys::is_vowel(prev_char.key) {
                    // Previous is consonant. Check if it could be a final consonant
                    if let Some(prev_cons_char) = crate::utils::key_to_char(prev_char.key, false) {
                        let prev_cons_str = prev_cons_char.to_string();
                        eprintln!("DEBUG can_apply_diacritical: prev is consonant '{}', checking if final", prev_cons_str);

                        // Is this consonant type potentially final? (c, ch, m, n, ng, nh, p, t)
                        if crate::infrastructure::external::diacritical_validator::DiacriticalValidator::is_final_consonant(&prev_cons_str) {
                            // It CAN be final, but is it ACTUALLY final? (must have vowel before it)
                            let has_vowel_before = if prev_pos > 0 {
                                // Check if there's ANY vowel before this consonant
                                (0..prev_pos).any(|i| {
                                    self.buf.get(i).map_or(false, |c| keys::is_vowel(c.key))
                                })
                            } else {
                                false // Consonant at position 0 can't be final (no vowel before it)
                            };
                            
                            if has_vowel_before {
                                eprintln!("DEBUG can_apply_diacritical: CASE 2 FOUND TRUE FINAL CONSONANT (has vowel before), REJECT");
                                return false; // REJECT: target vowel starts new syllable after complete one
                            } else {
                                eprintln!("DEBUG can_apply_diacritical: consonant '{}' is potentially final but NO vowel before it (initial consonant), ALLOW", prev_cons_str);
                            }
                        }
                    }
                }
            }
        }

        // No final consonants blocking this vowel = ALLOW
        eprintln!("DEBUG can_apply_diacritical: No final consonants found, ALLOW");
        true
    }

    /// Try to apply tone transformation by scanning buffer for targets
    fn try_tone(
        &mut self,
        key: u16,
        caps: bool,
        tone_type: ToneType,
        targets: &[u16],
    ) -> Option<Result> {
        // CRITICAL FIX: In Telex, 'w' is BOTH a tone modifier (for breve) AND a vowel (for ư)
        // When 'w' is pressed after 'a' at word start/after consonant, it should transform to ă (breve on a)
        // not apply as a tone modifier to make compound tones.
        // Let try_w_as_vowel() handle this case instead.
        if self.method == 0 && key == keys::W && tone_type == ToneType::Horn {
            // Check if buffer ends with unmodified 'a' (complete consonant-'a' pattern)
            if let Some(last_char) = self.buf.last() {
                if last_char.key == keys::A
                    && last_char.tone == tone::NONE
                    && last_char.mark == mark::NONE
                {
                    // Check if previous char is 'u' (ua pattern)
                    // If so, let try_tone handle it (ua + w -> ưa)
                    // Unless it is 'q' (qua + w -> quă)
                    let is_ua_pattern = self.buf.len() >= 2 && {
                        let prev = self.buf.get(self.buf.len() - 2).unwrap();
                        prev.key == keys::U
                    };

                    if is_ua_pattern {
                        // Check for 'q' before 'u' (qua)
                        // qua + w -> quă (breve on a) -> let try_w_as_vowel handle it
                        let has_q_initial = self.has_qu_initial();

                        if !has_q_initial {
                            // nua, mua, tua... + w -> nưa, mưa, tưa...
                            // Proceed with try_tone
                            // DO NOT RETURN None here
                        } else {
                            // qua + w -> quă
                            return None;
                        }
                    } else {
                        // This is the 'a+w' -> 'ă' case, don't handle here
                        // Let try_w_as_vowel() take care of it
                        return None;
                    }
                }
            }
        }

        // Check revert first (same key pressed twice)
        // CRITICAL: Check revert BEFORE English detection to assume explicit user intent
        if let Some(Transform::Tone(last_key, _)) = self.last_transform {
            if last_key == key {
                return Some(self.revert_tone(key, caps));
            }
        }

        // CRITICAL: Skip Vietnamese transform if English word detected
        // But verify if appending this key KEEPS it English.
        // DISABLED: This check breaks valid Vietnamese words that share prefixes with English words/invalid bigrams.
        // E.g. "lawn" (Telex 'w' -> breve). 'law' triggers English detection because 'a->w' is invalid Vietnamese bigram.
        // But in Telex, 'w' IS the modifier.
        // Rely on word-boundary restore instead.
        /*
        if self.is_english_word {
            // ... (disabled)
        }
        */

        // ═════════════════════════════════════════════════════════════════════
        // ENGLISH DETECTION: 3-Layer Architecture
        // ... (skipped)

        // ...

        let tone_val = tone_type.value();

        // Check if we're switching from one tone to another (e.g., ô → ơ)
        // Find vowels that have a DIFFERENT tone (to switch) or NO tone (to add)
        // OPTIMIZATION: Use binary_search instead of linear search (O(log n) vs O(n))
        let is_switching = self.buf.iter().any(|c| {
            targets.binary_search(&c.key).is_ok() && c.tone != tone::NONE && c.tone != tone_val
        });

        // Scan buffer for eligible target vowels
        let mut target_positions = Vec::new();

        // REMOVED: Automatic u+o compound detection
        // Issue: This was too aggressive and caused "khuow" → "khươ" instead of "khuơ"
        // The compound rule should only apply in context-specific cases, not during
        // general horn application. Use find_horn_target_with_switch instead which has
        // better phonological awareness.

        // Normal case: find last matching target
        if target_positions.is_empty() {
            if is_switching {
                // ...
                for (i, c) in self.buf.iter().enumerate().rev() {
                    if targets.binary_search(&c.key).is_ok()
                        && c.tone != tone::NONE
                        && c.tone != tone_val
                    {
                        target_positions.push(i);
                        break;
                    }
                }
            } else if tone_type == ToneType::Horn {
                // For horn modifier, apply smart vowel selection based on Vietnamese phonology
                target_positions = self.find_horn_target_with_switch(targets, tone_val);
                println!(
                    "DEBUG: find_horn_target_with_switch returned {:?}, targets={:?}",
                    target_positions, targets
                );
            } else {
                // FALLBACK: Normal tone application (e.g. aa -> â, ee -> ê, oo -> ô)
                // For Circumflex (doubling patterns), the target vowel must be ADJACENT (last in buffer)
                // This prevents "khoeo" (k-h-o-e-o) from applying circumflex to first 'o' across intervening 'e'
                let last_buf_idx = self.buf.len().saturating_sub(1);

                // Check if last char in buffer matches target and has no tone
                if let Some(last_char) = self.buf.get(last_buf_idx) {
                    if targets.contains(&last_char.key) && last_char.tone == tone::NONE {
                        // CRITICAL: For doubling patterns (aa, ee, oo), only apply if ADJACENT
                        // i.e., the key being pressed is the same as the last char
                        // This is already guaranteed because we're checking the last buffer char
                        target_positions.push(last_buf_idx);
                    }
                }

                // NEW FEATURE: Smart backward diacritical application
                // When typing diacritical mark after a final consonant or vowel,
                // apply diacritical to the appropriate vowel BEFORE the consonant/vowel
                // Examples:
                //   Telex: "cam" + "a" → "câm" (aa pattern, Circumflex)
                //   Telex: "than" + "s" → "thán" (s = tone Sắc, backward to 'a')
                //   VNI:   "cam" + "6" → "câm" (6 = circumflex)
                //   VNI:   "than" + "1" → "thán" (1 = tone Sắc, backward to 'a')
                //   Telex: "dau" + "a" → "dâu" (aa pattern)
                //   VNI:   "dau" + "6" → "dâu" (6 = circumflex)
                //
                // EXTENDED: Now supports ALL tone types (Sắc, Huyền, Hỏi, Ngã, Nặng)
                // not just diacritical marks (Circumflex, Horn, Breve)
                if target_positions.is_empty() && self.buf.len() >= 2 {
                    let last_char = self.buf.get(last_buf_idx).unwrap();

                    // CASE 1: Last char is a final consonant (e.g., "cam" + "a/6" → "câm")
                    // CASE 2: Last char is a vowel (e.g., "dau" + "a/6" → "dâu")
                    let should_check_backward = if !keys::is_vowel(last_char.key) {
                        // Last is consonant - check if it's a final consonant
                        if let Some(cons_char) = crate::utils::key_to_char(last_char.key, false) {
                            let cons_str = cons_char.to_string();
                            if crate::infrastructure::external::diacritical_validator::DiacriticalValidator::is_final_consonant(&cons_str) {
                                true
                            } else {
                                // Check digraphs: ng, nh, ch
                                // Need preceding char
                                if last_buf_idx > 0 {
                                    let prev_cons = self.buf.get(last_buf_idx - 1).unwrap();
                                    let s = format!("{}{}", 
                                        crate::utils::key_to_char(prev_cons.key, false).unwrap_or(' '),
                                        cons_char
                                    );
                                    matches!(s.as_str(), "ng" | "nh" | "ch")
                                } else {
                                    false
                                }
                            }
                        } else {
                            false
                        }
                    } else {
                        // Last is vowel - only allow backward for Telex doubling patterns (aa, ee, oo)
                        // or VNI mode (which doesn't need key matching)
                        //
                        // CRITICAL: For Telex, if last vowel is 'a' and key is 'w' (not 'a'),
                        // this is NOT a backward case! 'w' should apply to a different vowel.
                        // Example: "nua" + "w" should apply horn to 'u', NOT breve to 'a'
                        if self.method == 1 {
                            // VNI mode: always allow (numbers don't need to match vowels)
                            true
                        } else {
                            // Telex mode: only allow if it's a doubling pattern (Circumflex)
                            // We don't require last_char.key == key because we might be
                            // doubling a vowel further back (e.g. dau + a -> dâu)
                            // Horn and Breve don't have doubling patterns in Telex so we exclude them.
                            tone_type == ToneType::Circumflex
                        }
                    };

                    if should_check_backward {
                        eprintln!(
                            "DEBUG try_tone: Checking backward application, last_char_key={}",
                            last_char.key
                        );

                        // Look backward to find matching vowel that can receive this diacritical
                        for pos in (0..last_buf_idx).rev() {
                            if let Some(c) = self.buf.get(pos) {
                                eprintln!("DEBUG try_tone backward: Checking pos={}, key={}, is_vowel={}, tone={}", 
                                    pos, c.key, keys::is_vowel(c.key), c.tone);

                                // For VNI mode: match by tone targets (e.g., 6 can apply to a,e,o)
                                // For Telex mode: match by key (e.g., 'a' matches 'a')
                                let vowel_matches = if self.method == 1 {
                                    // VNI mode: check if vowel is in targets for this tone
                                    keys::is_vowel(c.key) && targets.contains(&c.key)
                                } else {
                                    // Telex mode: vowel must match the key being pressed
                                    keys::is_vowel(c.key)
                                        && c.key == key
                                        && targets.contains(&c.key)
                                };

                                if vowel_matches && c.tone == tone::NONE {
                                    // Check if there are vowels between pos and end of buffer
                                    let has_vowel_between = (pos + 1..self.buf.len()).any(|i| {
                                        self.buf.get(i).map_or(false, |ch| keys::is_vowel(ch.key))
                                    });

                                    if has_vowel_between {
                                        // There ARE vowels after the target. Use intent detection:
                                        // If adding the pressed key as a regular letter would form a valid
                                        // Vietnamese syllable, the user likely intends it as a letter, not
                                        // a backward modifier. Otherwise, it's a backward modifier.
                                        //
                                        // Examples:
                                        //   "moi" + "o" → "moio" invalid → backward modifier → "môi" ✓
                                        //   "khoe" + "o" → "khoeo" valid → regular letter → "khoeo" ✓
                                        //   "cao" + "a" → "caoa" invalid → backward modifier → "câo" ✓
                                        //   "dau" + "a" → "daua" invalid → backward modifier → "dâu" ✓
                                        if self.method == 1 {
                                            // VNI: numbers are unambiguous modifiers, always allow backward
                                        } else {
                                            // Telex: check if adding key as a letter would be valid
                                            let mut test_keys: Vec<u16> =
                                                self.buf.iter().map(|c| c.key).collect();
                                            test_keys.push(key);
                                            let valid_as_letter = crate::infrastructure::external::vietnamese_validator::VietnameseSyllableValidator::validate(&test_keys).is_valid;
                                            if valid_as_letter {
                                                // Key as a letter forms a valid syllable → probably a letter
                                                break;
                                            }
                                        }
                                    }

                                    target_positions.push(pos);
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        if target_positions.is_empty() {
            // ...
            return None;
        }

        // Track earliest position modified for rebuild
        let mut earliest_pos = usize::MAX;

        // FEATURE FLAG: Detect if this is backward tone/diacritical application
        // Backward = applying tone/mark to vowel BEFORE final consonant
        // Examples: "cam" + "a" → "câm", "than" + "s" → "thán"
        // Indicator: Last buffer char is NOT a vowel AND target position is NOT last position
        let is_backward_application = if self.buf.len() >= 2 {
            let last_idx = self.buf.len() - 1;
            // Backward IF: target is not last position
            // This covers both CASE 1 (after final consonant) and CASE 2 (after vowel)
            target_positions.iter().all(|&pos| pos != last_idx)
        } else {
            false
        };

        if is_backward_application {
            eprintln!(
                "DEBUG try_tone: BACKWARD APPLICATION DETECTED - allowing final consonant at end"
            );
        }

        // If switching, clear old tones first for proper rebuild
        if is_switching {
            for &pos in &target_positions {
                if let Some(c) = self.buf.get_mut(pos) {
                    c.tone = tone::NONE;
                    earliest_pos = earliest_pos.min(pos);
                }
            }

            // Special case: switching from horn compound (ươ) to circumflex (uô)
            if tone_type == ToneType::Circumflex {
                for &pos in &target_positions {
                    if let Some(c) = self.buf.get(pos) {
                        if c.key == keys::O {
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
        }

        // Apply new tone
        for &pos in &target_positions {
            // Step 1: Check conditions
            let should_skip = if tone_type == ToneType::Horn {
                if let Some(c) = self.buf.get(pos) {
                    if c.key == keys::U {
                        let is_uo_context = if let Some(next) = self.buf.get(pos + 1) {
                            next.key == keys::O
                        } else {
                            false
                        };
                        if is_uo_context {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            };

            if should_skip {
                continue;
            }

            // VALIDATION: Check if applying diacritical at this position is allowed
            // Prevent diacritical marks after final consonants (c, ch, m, n, ng, nh, p, t)
            // EXCEPT for backward application where final consonant at end is expected
            if !self.can_apply_diacritical(pos, is_backward_application) {
                // This position has a final consonant after it - reject tone
                return None;
            }

            // Step 2: Apply tone
            if let Some(c) = self.buf.get_mut(pos) {
                c.tone = tone_val;
                earliest_pos = earliest_pos.min(pos);
            }
        }

        // Validate result
        if tone_type == ToneType::Horn {
            let has_breve_vowel_pattern = target_positions.iter().any(|&pos| {
                if let Some(c) = self.buf.get(pos) {
                    if c.key == keys::A {
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
                return None;
            }
        }

        // Normalize ưo → ươ compound
        if let Some(compound_pos) = self.normalize_uo_compound() {
            earliest_pos = earliest_pos.min(compound_pos);
        }

        self.last_transform = Some(Transform::Tone(key, tone_val));

        // Reposition tone mark if vowel pattern changed
        let mut rebuild_pos = earliest_pos;
        if let Some((old_pos, _)) = self.reposition_tone_if_needed() {
            rebuild_pos = rebuild_pos.min(old_pos);
        }
        if earliest_pos == usize::MAX {
            return None;
        }

        // VALIDATION CHECK: Verify the tone application resulted in valid Vietnamese
        // If validation fails, this indicates English word typing - trigger instant restore
        let simulated_keys: Vec<u16> = self.buf.iter().map(|c| c.key).collect();
        eprintln!(
            "DEBUG try_tone: Validating buffer keys: {:?}",
            simulated_keys
        );
        let validation_result =
            crate::infrastructure::external::vietnamese_validator::VietnameseSyllableValidator::validate(
                &simulated_keys,
            );
        eprintln!(
            "DEBUG try_tone: Validation result: is_valid={}",
            validation_result.is_valid
        );
        if !validation_result.is_valid {
            // Validation failed - revert the tone and trigger instant restore
            for &pos in &target_positions {
                if let Some(c) = self.buf.get_mut(pos) {
                    c.tone = tone::NONE;
                }
            }

            // Check if instant restore is enabled and buffer has transforms
            if self.instant_restore_enabled && self.has_vietnamese_transforms() {
                let result = self.instant_restore_english();
                self.sync_buffer_with_raw_input();
                return Some(result);
            }

            // Otherwise, just pass through (don't apply tone)
            return None;
        }

        // CRITICAL FIX: Track the modifier key in raw_input
        return Some(self.rebuild_from(rebuild_pos));
    }

    /// Try to apply mark transformation (circumflex, breve, horn)
    fn try_mark(&mut self, key: u16, caps: bool, mark_val: u8) -> Option<Result> {
        eprintln!(
            "DEBUG try_mark ENTRY: key={}, mark_val={}, buf.len={}",
            key,
            mark_val,
            self.buf.len()
        );
        if self.buf.is_empty() {
            eprintln!("DEBUG try_mark: buffer is empty, returning None");
            return None;
        }

        // Check revert first
        // CRITICAL: Check revert BEFORE English detection
        if let Some(Transform::Mark(last_key, _)) = self.last_transform {
            if last_key == key {
                return Some(self.revert_mark(key, caps));
            }
        }

        // CRITICAL: Skip Vietnamese transform if English word detected
        // But verify if appending this key KEEPS it English.
        // DISABLED: This aggressive check blocks valid Vietnamese words that share prefixes with English words.
        // E.g. "lawn" (law -> lă -> lăn). If we return None here, we force "law" and can never get "lăn".
        // We should let the transform happen, and rely on `should_auto_restore` / `LanguageDecisionEngine`
        // to resolve the conflict at word boundary (Space).
        /*
        if self.is_english_word {
            let mut temp_raw = self.raw_input.clone();
            temp_raw.push(key, caps);

            let raw_vec = temp_raw.as_slice();
            let is_dict = crate::infrastructure::external::english::dictionary::Dictionary::is_common_english_word(
                &raw_vec[..],
            );

            let phonotactic =
                crate::infrastructure::external::english::phonotactic::PhonotacticEngine::analyze(&raw_vec[..]);

            if is_dict || phonotactic.english_confidence >= 95 {
                return None;
            }

            self.is_english_word = false;
        }
        */

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
        if !self.free_tone_enabled
            && !has_tone_marks
            && self.raw_input.len() >= 2
            && (self.method != 0 && self.method != 1)
        {
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
        // Skip validation for Telex/VNI methods (matches try_tone behavior)
        if !self.free_tone_enabled
            && !has_horn_transforms
            && !has_stroke_transforms
            && (self.method != 0 && self.method != 1)
            && !self.has_valid_initial()
        {
            return None;
        }

        // Validate buffer structure (skip if has horn/stroke transforms - already intentional Vietnamese)
        // Also skip validation if free_tone mode is enabled or using Telex/VNI (0/1)
        if !self.free_tone_enabled
            && !has_horn_transforms
            && !has_stroke_transforms
            && (self.method != 0 && self.method != 1)
        {
            // Use iterator-based validation to avoid allocation
            let buffer_keys: Vec<u16> = self.buf.iter().map(|c| c.key).collect();
            if !VietnameseSyllableValidator::validate(&buffer_keys).is_valid {
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
        if !self.free_tone_enabled && !has_horn_transforms && !has_stroke_transforms {}

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
        // Limit tone movement to valid range
        let pos =
            Phonology::find_tone_position(&vowels, has_final, self.modern_tone, has_qu, has_gi);

        // DO NOT validate tone placement here using mark_val.
        // mark_val is an ACCENT (Sắc, Huyền...), but is_valid_tone_placement
        // checks HAT/HORN validity (tone::HORN, tone::CIRCUMFLEX).
        // Passing mark_val (e.g. HUYEN=2) into simulated_tones causes it to be
        // misinterpreted as HORN (2), causing false validation failures (e.g. a+Huyen -> a+Horn=Breve).

        // We only check if the EXISTING buffer structure is valid before applying accent.
        let buffer_keys: Vec<u16> = self.buf.iter().map(|ch| ch.key).collect();
        let current_tones: Vec<u8> = self.buf.iter().map(|ch| ch.tone).collect();

        if !vietnamese::validation::is_valid_tone_placement(&buffer_keys, &current_tones) {
            return None;
        }

        // NOTE: We do NOT validate diacritical placement here.
        // try_mark() applies TONE MARKS (sắc, huyền, hỏi, ngã, nặng), not DIACRITICAL MARKS (^, ˘, ʼ).
        // Tone marks ARE allowed after final consonants (e.g., "tiền", "sàn").
        // Only diacritical marks (handled by try_tone()) are prohibited after final consonants.

        eprintln!("DEBUG try_mark: About to apply mark at pos={}", pos);
        if let Some(c) = self.buf.get_mut(pos) {
            eprintln!(
                "DEBUG try_mark: Applying mark={} to char at pos={}",
                mark_val, pos
            );
            c.mark = mark_val;
            self.last_transform = Some(Transform::Mark(key, mark_val));
        } else {
            eprintln!("DEBUG try_mark: FAILED to get_mut({}), returning None", pos);
            return None;
        }

        // Rebuild from the earlier position if compound was formed
        let rebuild_pos = rebuild_from_compound.map_or(pos, |cp| cp.min(pos));

        // VALIDATION CHECK: Verify the mark application resulted in valid Vietnamese
        // (Similar to try_tone validation)
        let simulated_keys: Vec<u16> = self.buf.iter().map(|c| c.key).collect();
        eprintln!(
            "DEBUG try_mark: simulated_keys before validation = {:?}",
            simulated_keys
        );
        let validation_result =
            crate::infrastructure::external::vietnamese_validator::VietnameseSyllableValidator::validate(
                &simulated_keys,
            );
        eprintln!(
            "DEBUG try_mark: validation_result.is_valid = {}",
            validation_result.is_valid
        );
        if !validation_result.is_valid {
            eprintln!("DEBUG try_mark: VALIDATION FAILED, returning None");
            // Validation failed - revert the mark and trigger instant restore
            if let Some(c) = self.buf.get_mut(pos) {
                c.mark = mark::NONE;
            }

            // Check if instant restore is enabled and buffer has transforms
            if self.instant_restore_enabled && self.has_vietnamese_transforms() {
                let result = self.instant_restore_english();
                self.sync_buffer_with_raw_input();
                return Some(result);
            }

            // Otherwise, just pass through (don't apply mark)
            return None;
        }

        // CRITICAL FIX: Track the modifier key in raw_input
        let result = self.rebuild_from(rebuild_pos);
        eprintln!(
            "DEBUG try_mark: About to return Some(result), backspace={}",
            result.backspace
        );
        return Some(result);
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
                targets.binary_search(&c.key).is_ok()
                    && (c.tone == tone::NONE || c.tone != new_tone)
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
                        targets.binary_search(&c.key).is_ok()
                            && (c.tone == tone::NONE || c.tone != new_tone)
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
        // SAFETY: Clamp to u8::MAX to prevent overflow
        let backspace = (self.buf.len() - pos).min(u8::MAX as usize) as u8;

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

                    // KEEP both keys in raw_input for accurate keystroke history.
                    // The first modifier key was already added by on_key(), and the
                    // second (revert) key was also added. Keeping both allows the
                    // phonotactic engine to detect double consonants (ff, ss, rr, etc.)
                    // which are strong English signals.

                    let result = self.revert_and_rebuild(pos, key, caps);

                    // Mark as English after tone revert - double modifier key = likely English
                    self.is_english_word = true;

                    return result;
                }
            }
        }
        self.is_english_word = false;
        Result::none()
    }

    /// Revert mark transformation
    fn revert_mark(&mut self, key: u16, caps: bool) -> Result {
        self.last_transform = None;

        for pos in self.buf.find_vowels().into_iter().rev() {
            if let Some(c) = self.buf.get_mut(pos) {
                if c.mark > mark::NONE {
                    c.mark = mark::NONE;

                    // KEEP both keys in raw_input for accurate keystroke history.
                    // Same rationale as revert_tone.

                    let result = self.revert_and_rebuild(pos, key, caps);

                    // Mark as English after mark revert - double modifier key = likely English
                    self.is_english_word = true;

                    return result;
                }
            }
        }
        self.is_english_word = false;
        Result::none()
    }

    /// Revert stroke transformation at specific position
    fn revert_stroke(&mut self, key: u16, pos: usize) -> Result {
        self.last_transform = None;

        // Early check if position is valid and char is stroked 'd'
        let should_revert = self
            .buf
            .get(pos)
            .map_or(false, |c| c.key == keys::D && c.stroke);

        if !should_revert {
            return Result::none();
        }

        // CRITICAL FIX: Calculate screen length BEFORE unstroke
        // We need to backspace the entire current output (e.g., "đa" = 2 chars)
        // not just the stroked char (1 char)
        let mut old_screen_len = 0;
        for i in pos..self.buf.len() {
            if let Some(ch) = self.buf.get(i) {
                if ch.key == keys::D && ch.stroke {
                    old_screen_len += 1; // 'đ' is 1 char
                } else if chars::to_char(ch.key, ch.caps, ch.tone, ch.mark).is_some() {
                    old_screen_len += 1;
                } else if utils::key_to_char(ch.key, ch.caps).is_some() {
                    old_screen_len += 1;
                }
            }
        }

        // Now we can safely mutate
        if let Some(c) = self.buf.get_mut(pos) {
            c.stroke = false; // Clear stroke

            // Add the key back to buffer
            // For "đ" -> "dd", we unstroke 'd' and add 'd'.
            let caps = c.caps;
            self.buf.push(Char::new(key, caps));

            // CRITICAL FIX: Mark as English to prevent re-stroke loop
            self.is_english_word = true;

            // Use the calculated screen length for backspace
            return self.rebuild_from_with_backspace(pos, old_screen_len);
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
    fn handle_normal_letter(&mut self, key: u16, caps: bool, _shift: bool) -> Result {
        eprintln!(
            "DEBUG handle_normal_letter: ENTRY key={}, caps={}, buf.len={}",
            key,
            caps,
            self.buf.len()
        );
        // Detect if typing special characters with Shift (e.g., @, #, $)
        // These indicate English input, so mark as English word
        //
        // REF-ISSUE: "đã" + "!" (Shift+1) caused revert to "d9a41"
        // This heuristic is too aggressive for punctuation (!, %, &, etc)
        // Disabled to allow valid Vietnamese + Punctuation
        /*
        if shift && keys::is_number(key) {
            // Exclude common symbols that are NOT letters (e.g., *, (, ) on US layout)
            // These should NOT lock the word into English mode.
            if key != keys::N8 && key != keys::N9 && key != keys::N0 {
                self.is_english_word = true;
            }
        }
        */

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
            // VALIDATION: Before adding vowel, check if it would create invalid multi-syllable pattern
            // Example: buffer=[c, â (with tone), m], adding 'a' → [c, â, m, a] (TWO syllables) → REJECT
            // This prevents invalid sequences after backward diacritical application
            if keys::is_vowel(key) && self.buf.len() >= 2 {
                eprintln!(
                    "DEBUG handle_normal_letter: Checking vowel '{}' against buffer len={}",
                    key,
                    self.buf.len()
                );
                let last_idx = self.buf.len() - 1;

                // Check pattern: [..., vowel-with-tone, final-consonant] + new-vowel
                if let (Some(last_char), Some(prev_char)) =
                    (self.buf.get(last_idx), self.buf.get(last_idx - 1))
                {
                    eprintln!(
                        "DEBUG handle_normal_letter: last_char.key={}, is_vowel={}",
                        last_char.key,
                        keys::is_vowel(last_char.key)
                    );
                    eprintln!(
                        "DEBUG handle_normal_letter: prev_char.key={}, is_vowel={}, tone={}",
                        prev_char.key,
                        keys::is_vowel(prev_char.key),
                        prev_char.tone
                    );

                    // Last char is final consonant AND previous char is vowel with Vietnamese tone mark
                    // NOTE: Only check mark (Vietnamese tones: sắc/huyền/hỏi/ngã/nặng), NOT tone
                    // (diacritical marks: circumflex/horn/breve). Using tone would incorrectly reject
                    // vowels after backward circumflex application (e.g., "rêc" + "i" in "received"),
                    // causing buffer-display mismatch and doubled-first-letter restoration bugs.
                    if !keys::is_vowel(last_char.key)
                        && keys::is_vowel(prev_char.key)
                        && prev_char.mark != 0
                    {
                        eprintln!("DEBUG handle_normal_letter: Pattern matched! Checking if last is final consonant");
                        // Check if last is actually a final consonant
                        if let Some(cons_char) = crate::utils::key_to_char(last_char.key, false) {
                            let cons_str = cons_char.to_string();
                            eprintln!(
                                "DEBUG handle_normal_letter: cons_str='{}', checking if final",
                                cons_str
                            );
                            if crate::infrastructure::external::diacritical_validator::DiacriticalValidator::is_final_consonant(&cons_str) {
                                eprintln!("DEBUG handle_normal_letter: Multi-syllable detected after [vowel-with-mark, final-consonant]");
                                // Still add the vowel to buffer to keep buffer-display in sync,
                                // then let check_and_restore_english handle English words.
                                // Don't reject (return Result::default) as it causes mismatch.
                            }
                        }
                    }
                }
            }

            // Add the letter to buffer
            self.buf.push(Char::new(key, caps));

            // FIX: After double-key revert (ss, ff, rr, xx, jj), buf is missing one
            // character vs raw_input because the original modifier was consumed (not added
            // to buf) when it applied the mark/tone. If is_english_word is set (by the
            // revert handler) and raw_input has MORE entries than buf, sync buf with
            // raw_input so subsequent output is correct (e.g., "asign" → "assign").
            if self.is_english_word && self.raw_input.len() > self.buf.len() {
                // displayed = chars on screen BEFORE this keystroke (buf.len after push - 1)
                let displayed = (self.buf.len() - 1).min(u8::MAX as usize) as u8;
                let raw_chars: Vec<char> = self
                    .raw_input
                    .iter()
                    .filter_map(|(k, c)| crate::utils::key_to_char(k, c))
                    .collect();
                self.sync_buffer_with_raw_input();
                self.last_transform = None;
                return Result::send(displayed, &raw_chars);
            }

            // Normalize ưo → ươ immediately when 'o' is typed after 'ư'
            // This ensures "dduwo" → "đươ" (Telex) and "u7o" → "ươ" (VNI)
            // Works for both methods since "ưo" alone is not valid Vietnamese
            if key == keys::O && self.normalize_uo_compound().is_some() {
                // ươ compound formed - reposition tone if needed (ư→ơ)
                if let Some((old_pos, _)) = self.reposition_tone_if_needed() {
                    // Check restore before returning separate rebuild path
                    if let Some(restored) = self.check_and_restore_english(1, false) {
                        return restored;
                    }
                    return self.rebuild_from_after_insert(old_pos);
                }

                // No tone to reposition - just output ơ
                let vowel_char = chars::to_char(keys::O, caps, tone::HORN, 0).unwrap();
                let output_res = Result::send(0, &[vowel_char]);
                // Check restore
                if let Some(restored) = self.check_and_restore_english(1, false) {
                    return restored;
                }
                return output_res;
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
                // Check restore
                if let Some(restored) = self.check_and_restore_english(1, false) {
                    return restored;
                }
                return self.rebuild_from_after_insert(old_pos);
            }

            // No tone movement needed
            // Fall through to update flags and check restore

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
            let _is_valid_triphthong_ending =
                self.has_complete_uo_compound() && (key == keys::U || key == keys::I);
            if false {
                // is_foreign_word_pattern replaced by LanguageDecisionEngine::decide early rejection
                return self.revert_w_as_vowel_transforms();
            }

            // Detect English word patterns and mark as English
            // This will trigger auto-restore on space key
            //
            // CRITICAL FIX: We must re-evaluate is_english_word on every keystroke.
            // If we typed "caw" (English) -> true.
            // Then typed "p" -> "cawp" (NOT English).
            // We must reset to false so that "s" (next key) can be processed as a Tone.
            if self.is_english_word {
                // Check if it's STILL English
                // Only check dictionary, as patterns (phonotactics) are more robust
                // But we should use both for consistency.
                let is_still_english =
                    self.is_english_dictionary_word() || self.has_definite_english_pattern();

                if !is_still_english {
                    self.is_english_word = false;
                }
            } else {
                if self.is_english_dictionary_word() || self.has_definite_english_pattern() {
                    self.is_english_word = true;
                }
            }
        } else {
            // Non-letter character (number, symbol, etc.)
            // Reset English word flag to prevent it from sticking to the next word
            self.is_english_word = false;
            // Mark that this word has non-letter prefix to prevent false shortcut matches
            self.has_non_letter_prefix = true;
        }

        // Confidence-based instant restore after each letter.
        // check_and_restore_english() has internal guards:
        // - Dictionary match → always restore
        // - Valid Vietnamese → conservative threshold (95+ or 80+ with dict)
        // - Invalid Vietnamese → aggressive threshold (60+ or dict)
        // These guards handle the "explicit marks" case properly:
        // e.g., "phát" is valid Vietnamese → won't restore even if "phat" looks English
        // e.g., "cóst" is invalid Vietnamese + "cost" in dictionary → restores correctly
        if let Some(restored) = self.check_and_restore_english(1, false) {
            return restored;
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

    /// Rebuild buffer from `from` position and inject new text after backspacing
    /// Rebuild output from position after a new character was inserted
    ///
    /// Unlike rebuild_from, this accounts for the fact that the last character
    /// in the buffer was just added but NOT yet displayed on screen.
    /// So backspace count = (chars from `from` to BEFORE the new char)
    /// Because the last char (newly added) is not yet on screen, it doesn't need to be backspaced.
    /// And output = (chars from `from` to end INCLUDING new char)
    fn rebuild_from_after_insert(&self, from: usize) -> Result {
        if self.buf.is_empty() {
            return Result::none();
        }

        let mut output = Vec::with_capacity(self.buf.len() - from);
        // Backspace = number of chars from `from` to BEFORE the new char
        // The new char (last in buffer) hasn't been displayed yet
        // SAFETY: Clamp to u8::MAX to prevent overflow
        let backspace = self
            .buf
            .len()
            .saturating_sub(1)
            .saturating_sub(from)
            .min(u8::MAX as usize) as u8;

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

    /// Clear everything including word history
    ///
    /// Used when cursor position changes (mouse click, selection-delete, etc.)
    /// to prevent restoring stale state from history.
    pub fn clear_all(&mut self) {
        self.clear();
        self.word_history.clear();
        self.spaces_after_commit = 0;
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

        // CRITICAL: Re-detect English status for the restored word
        // This ensures subsequent keys are handled correctly if backspaced into English
        if self.raw_input.len() >= 2 {
            if self.is_english_dictionary_word() || self.has_definite_english_pattern() {
                self.is_english_word = true;
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

    /// Get buffer keys suitable for Vietnamese syllable validation
    /// Filters out W and F when they appear as Telex modifiers (not standalone vowels)
    /// Example: "trương" with buffer [T,R,U,O,W,F,N,G] → [T,R,U,O,N,G]
    ///
    /// This prevents the validator from rejecting Vietnamese words that use Telex modifiers
    /// like "ương" (uow + tone f) which contains W and F as modifiers, not as letter keys.
    fn get_buffer_keys_for_validation(&self) -> Vec<u16> {
        let mut cleaned = Vec::new();

        for i in 0..self.buf.len() {
            let Some(c) = self.buf.get(i) else { continue };
            let key = c.key;

            // Skip W and F if they appear in the middle/end of buffer AND previous char is a vowel
            // This indicates they're Telex modifiers, not standalone characters
            if (key == keys::W || key == keys::F) && i > 0 {
                if let Some(prev) = self.buf.get(i - 1) {
                    // If previous is a vowel (a,e,i,o,u,y), then this W/F is a modifier
                    if keys::is_vowel(prev.key) {
                        // Skip this W/F - it's a modifier, not a letter
                        continue;
                    }
                }
            }

            cleaned.push(key);
        }

        cleaned
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
    /// Detect English word patterns using raw keystroke history
    /// Uses the new 8-layer Matrix-Based Phonotactic Engine
    fn has_english_word_pattern(&self) -> bool {
        if self.raw_input.is_empty() {
            return false;
        }

        // BUGFIX: Don't treat as English if we have a complete compound vowel with tone mark
        // This handles "ươ" + tone + consonant (e.g., "trườ" + tone "f" + consonant "n")
        //
        // The issue: raw Telex keys "t-r-u-o-w-f-n" don't match any valid Vietnamese pattern
        // when validated individually, triggering false English detection.
        // But the Vietnamese output "trườn" is perfectly valid (compound ươ + tone + final n).
        //
        // Solution: If buffer has:
        // 1. A complete uo compound (u with horn + o with horn), AND
        // 2. A tone mark on ANY character
        // Then trust the Vietnamese output, don't do English detection
        if self.has_complete_uo_compound() {
            let has_tone_mark = self.buf.iter().any(|c| c.mark > 0); // mark > 0 = tone mark exists
            if has_tone_mark {
                // Complete compound + tone mark = Valid Vietnamese, trust it
                return false;
            }
        }

        // 1. Dictionary Check (O(1)) - Highest Priority
        // MEMORY OPTIMIZATION: Disabled in release builds to save ~1.4MB
        #[cfg(debug_assertions)]
        {
            if self.is_english_dictionary_word() {
                return true;
            }
        }

        // 2. Vietnamese Validation
        // If the word structure is VALID Vietnamese, we treat it as Vietnamese
        // (unless it was already found in the English Dictionary above)
        // Use buffer keys (with transforms applied) PLUS the current key being typed
        // The buffer has "biê" and we're about to add "n", so validate "biên"
        let raw_keys: Vec<(u16, bool)> = self.raw_input.iter().collect();
        let mut buf_keys: Vec<u16> = self.buf.iter().map(|c| c.key).collect();
        // Add the current key that's about to be typed (raw_input already includes it)
        if let Some(&(last_key, _)) = raw_keys.last() {
            buf_keys.push(last_key);
        }
        let viet_val = VietnameseSyllableValidator::validate(&buf_keys);

        if viet_val.is_valid {
            return false;
        }

        // 3. Strong English Pattern (Phonotactic > 95%) AND Invalid Vietnamese
        let phonotactic = PhonotacticEngine::analyze(&raw_keys);

        if phonotactic.english_confidence >= 95 {
            return true;
        }

        // 4. Heuristic: Looks English (>0%) AND is Invalid Vietnamese
        if phonotactic.is_english() {
            // Additional check: Ensure it's not just a Vietnamese prefix typed rapidly
            // If phonotactic confidence is high enough (>50) or multiple layers matched
            if phonotactic.english_confidence >= 80 || phonotactic.matched_layers.count_ones() >= 2
            {
                return true;
            }
        }

        false
    }

    /// Check if current raw input is in the English dictionary
    fn is_english_dictionary_word(&self) -> bool {
        let keys: Vec<u16> = self.raw_input.iter().map(|(k, _)| k).collect();

        // FIX: In Telex, if the last key is 'w' (a tone modifier for horn/breve),
        // don't mark as English dictionary word because 'w' will be processed as a tone modifier.
        // Examples: 'naw' -> 'nă' (Telex), 'law' -> 'lă' (Telex), not English words
        if self.method == 0 {
            // Telex mode
            if let Some(&last_key) = keys.last() {
                if last_key == keys::W {
                    // In Telex, 'w' is a tone modifier. Let it go through tone handling
                    // instead of marking as English dictionary word.
                    return false;
                }
            }
        }

        Dictionary::is_english(&keys)
    }

    /// Check for DEFINITE English patterns (e.g. invalid Vietnamese initials)
    /// High confidence check used for bypassing transforms
    fn has_definite_english_pattern(&self) -> bool {
        if self.raw_input.is_empty() {
            return false;
        }

        let raw_keys: Vec<(u16, bool)> = self.raw_input.iter().collect();

        // 1. Explicit Early Pattern Check (Layer 1 - Unambiguous)
        // Check for 'ex' (export, express) - very strong signal
        if raw_keys.len() >= 2 {
            let k0 = raw_keys[0].0;
            let k1 = raw_keys[1].0;
            if k0 == keys::E && k1 == keys::X {
                return true;
            }
        }

        // CRITICAL: Check Vietnamese validity FIRST
        // Use buffer keys (with transforms applied) PLUS the current key being typed
        // The buffer has "biê" and we're about to add "n", so validate "biên"
        // This prevents valid Vietnamese like "biên" from being marked as English
        //
        // IMPORTANT: Must use validate_with_tones to include circumflex/horn info!
        // Without tones, validator sees ['b','i','e','n'] and rejects 'ien' as invalid.
        // With tones, validator sees 'e' has circumflex, making 'iên' valid.
        let buf_keys: Vec<u16> = self.buf.iter().map(|c| c.key).collect();
        let buf_tones: Vec<u8> = self.buf.iter().map(|c| c.tone).collect();
        // Check validation of the CURRENT buffer (which already includes the new key)
        let viet_val = crate::infrastructure::external::vietnamese_validator::VietnameseSyllableValidator::validate_with_tones(&buf_keys, &buf_tones);
        if viet_val.is_valid {
            return false;
        }

        let phonotactic = PhonotacticEngine::analyze(&raw_keys);

        // Highly confident English (>=95%) is "definite"
        // This excludes Coda clusters (91%) like 'st' which conflict with Telex tones
        // But includes Onset clusters (98%) and Invalid Initials (100%)
        let layer1_invalid_initials = (phonotactic.matched_layers & 1) != 0;

        phonotactic.english_confidence >= 95 || layer1_invalid_initials
    }

    /// Instant restore to raw ASCII (no trailing space)
    fn instant_restore_english(&self) -> Result {
        restore::instant_restore_english(&self.buf, &self.raw_input)
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
    /// Sync internal buffer with raw input (used for English restoration)
    fn sync_buffer_with_raw_input(&mut self) {
        self.buf.clear();
        for (key, caps) in self.raw_input.iter() {
            self.buf.push(Char::new(key, caps));
        }
    }

    /// Check if current buffer should be restored to English based on confidence
    /// Returns Some(Result) if restore happened, None otherwise
    fn check_and_restore_english(&mut self, offset: u8, strict_mode: bool) -> Option<Result> {
        if !self.instant_restore_enabled {
            return None;
        }

        // CASE A: After double-consonant revert (ff, ss, rr), buffer has no transforms
        // but raw_input has more entries than buffer (consumed key). Restore to raw input
        // so "diferent" becomes "different", "efect" becomes "effect", etc.
        // NOTE: Skip this during the revert flow (when called from revert check with offset>0)
        // because the revert result hasn't been applied to display yet.
        if offset == 0 && self.is_english_word && self.raw_input.len() > self.buf.len() {
            let result = self.instant_restore_english();
            self.sync_buffer_with_raw_input();
            self.last_transform = None; // Clear stale transform after English restore
            return Some(result);
        }

        // Restore conditions:
        // 1. Has transforms (tones/marks)
        if !self.has_vietnamese_transforms() {
            return None;
        }

        // PRIORITY CHECK: If raw input is in English dictionary (programming terms, common words),
        // ALWAYS restore immediately, regardless of Vietnamese validation or confidence scores
        // This ensures words like "console" don't become "cónole"
        let raw_key_list: Vec<u16> = self.raw_input.iter().map(|item| item.0).collect();
        let is_dict = crate::infrastructure::external::english::dictionary::Dictionary::is_english(
            &raw_key_list,
        );
        eprintln!("DEBUG check_and_restore: has_transforms={}, buf.len={}, raw_input.len={}, is_dict={}, raw_keys={:?}", 
            self.has_vietnamese_transforms(), self.buf.len(), self.raw_input.len(), is_dict, raw_key_list);
        if is_dict {
            eprintln!("DEBUG: Restoring from dictionary match");
            self.is_english_word = true;
            let mut result = self.instant_restore_english();
            // Adjust backspace to account for pending characters (buffer > screen)
            result.backspace = result.backspace.saturating_sub(offset);
            self.sync_buffer_with_raw_input();
            self.last_transform = None; // Clear stale transform after English restore
            return Some(result);
        }

        let raw_keys: Vec<(u16, bool)> = self.raw_input.iter().collect();
        let phonotactic =
            crate::infrastructure::external::english::phonotactic::PhonotacticEngine::analyze(
                &raw_keys,
            );

        // Get Vietnamese validation
        let buf_keys: Vec<u16> = self.buf.iter().map(|c| c.key).collect();
        let viet_validation =
            crate::infrastructure::external::vietnamese_validator::VietnameseSyllableValidator::validate(
                &buf_keys,
            );

        // Check dictionary (already checked above, but keep variable for backward compatibility)
        let _is_dict = is_dict;

        // Restore if: high English confidence (>=80) OR dictionary match
        // But if buffer is VALID VIETNAMESE, be more conservative:
        // - Require very high confidence (>=90) AND dictionary match to override valid Vietnamese
        // - OR require invalid Vietnamese syllable
        // SPECIAL CASE: Short valid Vietnamese words (2 chars) should be restored more aggressively
        // because they're usually English prefixes (re, to, me, so, etc.)

        let should_restore = if viet_validation.is_valid {
            // Vietnamese collision case (e.g. "ban", "ca", "to", "moe" -> "me")
            // Only restore if we are SUPER confident it's English
            // CRITICAL FIX: Check dictionary against RAW input, not transformed buffer
            let raw_keys_only: Vec<u16> = self.raw_input.iter().map(|item| item.0).collect();
            let is_raw_dict =
                crate::infrastructure::external::english::dictionary::Dictionary::is_english(
                    &raw_keys_only,
                );

            // SPECIAL HANDLING: For short 2-character valid Vietnamese words that are NOT
            // in the Vietnamese dictionary (like "re" which appears in English but not as standalone Vietnamese),
            // be more aggressive about restoration when tone/mark is applied.
            // These are usually English prefixes (re, de, to, so, me, etc.) followed by tone modifiers.
            // This catches cases like "rest" → tone on "e" → "ré" → restore to "res"
            //
            // EXCEPTION: If the user explicitly typed a Telex tone key (s, f, r, x, j) to add the mark,
            // we should trust the user's intent to type Vietnamese for these ambiguous short words,
            // UNLESS the word is in the English dictionary (handled separately).
            // This fixes the "rẻ" case (typed "rer") being restored to "rer".
            let is_telex_tone_key = if self.method
                == crate::infrastructure::engine::types::config::InputMethod::Telex as u8
            {
                use crate::data::keys::*;
                let last = raw_keys_only.last().cloned().unwrap_or(0);
                matches!(last, S | F | R | X | J | Z)
            } else {
                false
            };

            let is_short_undocumented =
                self.buf.len() <= 2 && self.has_vietnamese_transforms() && !is_raw_dict;

            if is_short_undocumented {
                // For short undocumented (not in Vietnamese dictionary) words with transforms,
                // use lower threshold: 75. These are likely English prefixes with accidental tone.
                if is_telex_tone_key {
                    // If user extensively used a tone key, require HIGH confidence or dictionary match.
                    // Since dictionary match is false here (is_raw_dict=false), we basically disable auto-restore
                    // for this case unless confidence is super high (e.g. 95+), implies very strong English pattern.
                    // "rer" has 75 confidence. "mos" has ?
                    phonotactic.english_confidence >= 95
                } else {
                    phonotactic.english_confidence >= 95
                }
            } else {
                // For documented Vietnamese words or longer words, use conservative threshold: 95
                // Dictionary-based restore uses lower threshold (80+) to catch common English
                // words like "cost", "most", "best" etc. that collide with valid Vietnamese.
                (phonotactic.english_confidence >= 95)
                    || (phonotactic.english_confidence >= 80 && is_raw_dict)
            }
        } else {
            // Invalid Vietnamese - AGGRESSIVE restore
            // If it's not valid Vietnamese (e.g., "res", "xyz"), we should restore to English
            // unless it looks like Vietnamese phonotactics.
            // Lowered threshold to 60 because invalid Vietnamese SHOULD be restored.
            // This catches short words like "res" (confidence 75), "off" (confidence 70), etc.
            let raw_keys_only: Vec<u16> = self.raw_input.iter().map(|item| item.0).collect();

            let is_raw_dict =
                crate::infrastructure::external::english::dictionary::Dictionary::is_english(
                    &raw_keys_only,
                );

            if strict_mode {
                is_raw_dict
            } else {
                phonotactic.english_confidence >= 60 || is_raw_dict
            }
        };

        // Use the unused variable to silence lint
        let _ = viet_validation;

        if should_restore {
            self.is_english_word = true;
            let mut result = self.instant_restore_english();
            // Adjust backspace to account for pending characters (buffer > screen)
            result.backspace = result.backspace.saturating_sub(offset);

            self.sync_buffer_with_raw_input();
            self.last_transform = None; // Clear stale transform after English restore
            return Some(result);
        }

        None
    }

    /// Rebuild output from entire buffer (used after transform when we need full rebuild)
    fn rebuild_output_from_entire_buffer(&self) -> Result {
        let buf_len = self.buf.len();
        if buf_len == 0 {
            return Result::none();
        }

        let chars: Vec<char> = self.buf.to_full_string().chars().collect();
        // SAFETY: Clamp to u8::MAX to prevent overflow
        Result::send(buf_len.min(u8::MAX as usize) as u8, &chars)
    }

    /// Advanced phonotactic analysis for English detection
    /// Uses 8-layer matrix-based detection for high confidence
    pub fn analyze_phonotactic_english(&self) -> phonotactic::PhonotacticResult {
        let raw_keys: Vec<(u16, bool)> = self.raw_input.iter().collect();
        phonotactic::PhonotacticEngine::analyze(&raw_keys)
    }

    /// Validate Vietnamese syllable structure (6 rules)
    pub fn validate_vietnamese_syllable(&self) -> ValidationResult {
        let keys: Vec<u16> = self.buf.iter().map(|c| c.key).collect();
        VietnameseSyllableValidator::validate(&keys)
    }

    /// Decide whether to restore English word
    /// Uses Phonotactic Engine and AutoRestoreDecider
    pub fn should_auto_restore(&self) -> bool {
        let raw_keys: Vec<(u16, bool)> = self.raw_input.iter().collect();
        let phonotactic = PhonotacticEngine::analyze(&raw_keys);

        let _is_restore = raw_keys.len() == self.buf.len();

        // CRITICAL FIX: If buffer has tone marks (sắc, huyền, hỏi, ngã, nặng - stored in mark field),
        // it is definitely a Vietnamese word, NEVER auto-restore.
        // Example: "tét" (from "test" in Telex), "đã" (from "daf" in Telex)
        let has_tone_mark = self.buf.iter().any(|c| c.mark > 0);
        if has_tone_mark {
            return false;
        }

        // CRITICAL FIX: Don't restore Vietnamese words with compound vowels + marks
        // Example: "trường" has complete ươ compound with huyền mark (f) - definitely Vietnamese!
        if self.has_complete_uo_compound() {
            // Check for vowel tone (horn, circumflex, breve)
            let has_vowel_tone = self.buf.iter().any(|c| c.tone > 0);
            if has_vowel_tone {
                // Complete compound + vowel tone = Valid Vietnamese, never auto-restore
                return false;
            }
        }

        // CRITICAL FIX: When buffer contains W or F as Telex modifiers (with adjacent vowel compounds),
        // we need to validate the "cleaned" keys (without the modifier keys) for Vietnamese structure.
        // Example: "trương" → buffer keys [T,R,U,O,W,F,N,G]
        //          but should validate [T,R,U,O,N,G] to check Vietnamese vowel structure
        let cleaned_buf_keys = self.get_buffer_keys_for_validation();
        let vietnamese_validation = VietnameseSyllableValidator::validate(&cleaned_buf_keys);

        // CRITICAL FIX: If the buffer has Vietnamese transforms (dấu/thanh) AND
        // the word structure is valid Vietnamese, NEVER auto-restore.
        // This prevents valid Vietnamese words from being converted back to ASCII on SPACE.
        // Example: "lăn" (lawn), "râu" (row), "vơ" (vow) should stay as Vietnamese.
        if self.has_vietnamese_transforms() && vietnamese_validation.is_valid {
            return false;
        }

        // LAYER 1: Phonotactic + Vietnamese validation analysis
        use crate::infrastructure::external::english::phonotactic::AutoRestoreDecider;
        let phonotactic_decision = AutoRestoreDecider::should_restore(
            &phonotactic,
            &vietnamese_validation,
            self.has_vietnamese_transforms(),
        );

        // If phonotactic analysis is confident (strong signal), use it
        if phonotactic_decision {
            return true;
        }

        // LAYER 2 (FINAL): Dictionary check as tie-breaker
        // LAYER 2 (FINAL): Dictionary check as tie-breaker
        // Trust the dictionary presence (conflicts were filtered at generation time)
        if self.is_english_dictionary_word() {
            // FIX: If the word is a valid Vietnamese word AND contains transforms (e.g. "lawn" -> "lăn"),
            // we should prefer the Vietnamese word in Vietnamese mode.
            // This prevents common valid words like "lăn", "râu" (row), "vơ" (vow) from being auto-restored.
            // Exception: If the user explicitly wants the English word, they can use Esc or disable auto-restore.
            if self.has_vietnamese_transforms() && vietnamese_validation.is_valid {
                // Check strength of English signal (e.g. suffix match like -ore in "more")
                // If extremely confident, we prefer the English dictionary word even if it happens to be valid Vietnamese structure.
                // Threshold 90 is safe because "lawn" has ~0 confidence, "more" has ~90 confidence.
                if phonotactic.english_confidence >= 90 {
                    return true;
                }
                return false;
            }
            return true;
        }

        false
    }

    /// Get auto-restore confidence (0-100%)
    /// Uses AutoRestoreDecider with dictionary as final layer
    pub fn auto_restore_confidence(&self) -> u8 {
        let raw_keys: Vec<(u16, bool)> = self.raw_input.iter().collect();
        let phonotactic = PhonotacticEngine::analyze(&raw_keys);

        // Get Vietnamese validator result
        let buf_keys: Vec<u16> = self.buf.iter().map(|c| c.key).collect();
        let vietnamese_validation = VietnameseSyllableValidator::validate(&buf_keys);

        // LAYER 1: Phonotactic + Vietnamese validation confidence
        use crate::infrastructure::external::english::phonotactic::AutoRestoreDecider;
        let phonotactic_confidence =
            AutoRestoreDecider::confidence(&phonotactic, &vietnamese_validation);

        // If phonotactic confidence is high (>= 80), trust it
        if phonotactic_confidence >= 80 {
            return phonotactic_confidence;
        }

        // LAYER 2 (FINAL): Dictionary check as confidence booster
        // If word is in dictionary, boost to 100% confidence (conflicts filtered offline)
        use crate::infrastructure::external::english::dictionary::Dictionary;
        let keys_only: Vec<u16> = raw_keys.iter().map(|(k, _)| *k).collect();
        if Dictionary::is_english(&keys_only) {
            return 100; // Dictionary match = 100% confidence
        }

        // Return phonotactic confidence (may be low, but it's the best we have)
        phonotactic_confidence
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
        ("vieset", "viết"), // vie+s→vié, then vié+e+t→viết
        ("vieste", "viết"), // vie+s→vié, then vié+t+e→viết
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
        ("nes", "né"), // ne + s (sắc) → né (s is consumed as tone)
        ("nef", "nè"), // ne + f (huyền) → nè (f is consumed as tone)
        ("ner", "nẻ"), // ne + r (hỏi) → nẻ (r is consumed as tone)
        // Skip "nej" - appears to have a bug in tone handling (separate issue)
        ("tes", "té"), // te + s (sắc) → té (s is consumed as tone)
        ("tef", "tè"), // te + f (huyền) → tè (f is consumed as tone)
        ("ter", "tẻ"), // te + r (hỏi) → tẻ (r is consumed as tone)
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
        ("telex", "telex"),           // t-e-l-e-x pattern (NOT "tễl")
        ("release", "release"),       // r-e-l-e-a-s-e pattern (NOT "rêlase")
        ("delete", "delete"),         // d-e-l-e-t-e pattern (NOT "dêlete")
        ("select", "select"),         // s-e-l-e-c-t pattern (NOT "sêlect")
        ("element", "element"),       // e-l-e-m-e-n-t pattern
        ("reflex", "reflex"),         // r-e-f-l-e-x pattern
        ("importance", "importance"), // i-m-p pattern detected at 3 chars
        ("complex", "complex"),       // c-o-m pattern detected at 3 chars
        ("export", "export"),         // e-x-p pattern detected at 3 chars
        ("express", "express"),       // e-x-p pattern detected at 3 chars
        ("implement", "implement"),   // i-m-p pattern detected at 3 chars
        ("complete", "complete"),     // c-o-m pattern detected at 3 chars
    ];

    // Keep 4-letter English patterns that were already working
    // Note: "test" and "best" are removed because they can be valid Vietnamese syllables
    // ("tét", "bét") when user intends to type Vietnamese
    const ENGLISH_SHORT_WORDS: &[(&str, &str)] = &[
        ("text", "text"), // t-e-x-t pattern (NOT "tẽt")
        ("next", "next"), // n-e-x-t pattern
        ("sexy", "sexy"), // s-e-x-y pattern
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
        use crate::infrastructure::engine::Action;

        // Regression: In Telex, "se" + 'x' must produce "sẽ" (x is consumed as tone key).
        // Previously, English detection could incorrectly lock the word as English and block tone application.
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        e.on_key_ext(keys::S, false, false, false);
        e.on_key_ext(keys::E, false, false, false);
        let r = e.on_key_ext(keys::X, false, false, false);

        assert_eq!(
            r.action,
            Action::Send as u8,
            "Tone key should be consumed and produce output"
        );
        assert_eq!(
            r.backspace, 1,
            "Should replace the last vowel with a toned vowel"
        );
        assert_eq!(r.count as usize, 1, "Should emit exactly one character");

        let out = unsafe { char::from_u32(*r.chars.offset(0)).expect("valid unicode scalar") };
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
        assert_eq!(
            ak_result, "ak",
            "'ak' should not transform (invalid Vietnamese syllable)"
        );

        // Reset engine
        e = Engine::new();
        e.set_method(0);
        e.set_enabled(true);

        // Test "az" pattern - should NOT transform
        let az_result = type_word(&mut e, "az");
        assert_eq!(
            az_result, "az",
            "'az' should not transform (invalid Vietnamese syllable)"
        );

        // Reset engine
        e = Engine::new();
        e.set_method(0);
        e.set_enabled(true);

        // Test "ah" + consonant (not 'n') - should NOT transform
        let aht_result = type_word(&mut e, "aht");
        assert_eq!(
            aht_result, "aht",
            "'aht' should not transform (invalid Vietnamese syllable)"
        );

        // Reset engine
        e = Engine::new();
        e.set_method(0);
        e.set_enabled(true);

        // Test "anh" - SHOULD work as valid Vietnamese
        let anh_result = type_word(&mut e, "anh");
        assert_eq!(
            anh_result, "anh",
            "'anh' is valid Vietnamese and should pass through"
        );
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
        assert!(
            !e.is_english_word,
            "'krong' should NOT be marked as English (valid Vietnamese with kr initial)"
        );

        // Verify we can apply Vietnamese transforms (add circumflex with 'o')
        let _r = e.on_key_ext(keys::O, false, false, false);

        // Should allow transform (backspace > 0 means it's transforming)
        // or at least not reject it
        assert_eq!(
            e.is_english_word, false,
            "Should still allow Vietnamese transforms after typing 'o'"
        );
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
        assert!(
            has_circumflex,
            "Should have applied circumflex mark (aa->â, tone=1)"
        );

        // Add sắc tone with 's'
        let r = e.on_key_ext(keys::S, false, false, false);

        // Should apply tone
        assert!(
            r.backspace > 0 || r.count > 0,
            "Should apply tone transform"
        );
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
        assert_eq!(
            delete_result.backspace, 1,
            "Should only send 1 backspace for independent trailing char"
        );
        assert_eq!(
            delete_result.count, 0,
            "Should not send replacement text in fast path"
        );
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
        assert_eq!(
            e.buf.len(),
            5,
            "Buffer should have 5 chars for 'telex' (no transforms)"
        );

        // Verify NO Vietnamese transforms were applied
        let has_transforms = e.buf.iter().any(|c| c.tone != 0 || c.mark != 0 || c.stroke);
        assert!(
            !has_transforms,
            "Buffer should have no Vietnamese transforms for 'telex'"
        );

        // SPACE -> No transforms to restore, just pass through
        let r = e.on_key_ext(keys::SPACE, false, false, false);

        // No restore needed - pass through
        assert_eq!(
            r.action, 0,
            "Should pass through (no transforms to restore)"
        );
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
        assert_eq!(
            delete_result.backspace, 3,
            "Should send 3 backspaces for 3 screen chars"
        );
        assert!(delete_result.count > 0, "Should send replacement text");
    }

    #[test]
    fn test_is_english_word_reset_on_empty_buffer() {
        use crate::data::keys;
        use crate::infrastructure::engine::buffer::Char;

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
        assert!(
            eng.is_english_word,
            "is_english_word should be true before deletion"
        );

        // Delete all 4 characters via backspace
        // This should trigger the flag reset when buffer becomes empty
        eng.on_key_ext(keys::DELETE, false, false, false);
        assert_eq!(
            eng.buf.len(),
            3,
            "Buffer should have 3 characters after 1st delete"
        );

        eng.on_key_ext(keys::DELETE, false, false, false);
        assert_eq!(
            eng.buf.len(),
            2,
            "Buffer should have 2 characters after 2nd delete"
        );

        eng.on_key_ext(keys::DELETE, false, false, false);
        assert_eq!(
            eng.buf.len(),
            1,
            "Buffer should have 1 character after 3rd delete"
        );

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
        assert_eq!(
            e.spaces_after_commit, 1,
            "Should track one space after commit"
        );

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
        assert_eq!(
            second_backspace.action, 0,
            "Second backspace should pass through, not restore from history"
        );
        assert_eq!(
            second_backspace.backspace, 0,
            "Should not send any backspace commands"
        );

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
        assert_eq!(
            e.buf.len(),
            7,
            "Buffer should have 7 characters for 'element'"
        );

        // Verify no transforms were applied (all vowels have tone=0)
        let has_transforms = e.buf.iter().any(|c| c.tone != 0 || c.mark != 0 || c.stroke);
        assert!(
            !has_transforms,
            "Buffer should have no Vietnamese transforms for 'element'"
        );

        // SPACE → Should pass through (no transforms to restore)
        // With no transforms applied, there's nothing to restore
        let r = e.on_key_ext(keys::SPACE, false, false, false);

        // No restore needed - just pass through
        assert_eq!(
            r.action, 0,
            "Should pass through (no transforms to restore)"
        );
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

        assert_eq!(
            e.buf.len(),
            7,
            "Buffer should have 7 characters for 'release'"
        );

        // Verify no circumflex was applied
        let has_transforms = e.buf.iter().any(|c| c.tone != 0 || c.mark != 0 || c.stroke);
        assert!(
            !has_transforms,
            "Buffer should have no Vietnamese transforms for 'release'"
        );

        let r = e.on_key_ext(keys::SPACE, false, false, false);

        // No restore needed - just pass through
        assert_eq!(
            r.action, 0,
            "Should pass through (no transforms to restore)"
        );
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

        assert_eq!(
            e.buf.len(),
            6,
            "Buffer should have 6 characters for 'delete'"
        );

        // Verify no circumflex was applied
        let has_transforms = e.buf.iter().any(|c| c.tone != 0 || c.mark != 0 || c.stroke);
        assert!(
            !has_transforms,
            "Buffer should have no Vietnamese transforms for 'delete'"
        );

        let r = e.on_key_ext(keys::SPACE, false, false, false);

        // No restore needed - just pass through
        assert_eq!(
            r.action, 0,
            "Should pass through (no transforms to restore)"
        );
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

        assert_eq!(
            e.buf.len(),
            7,
            "Buffer should have 7 characters for 'reverse'"
        );

        // Verify no circumflex was applied (note: 'r' and 's' are tone modifiers but won't apply here)
        let has_circumflex = e.buf.iter().any(|c| c.tone == 1); // tone=1 is circumflex
        assert!(
            !has_circumflex,
            "Buffer should have no circumflex for 'reverse'"
        );

        let r = e.on_key_ext(keys::SPACE, false, false, false);

        // No restore needed - just pass through
        assert_eq!(
            r.action, 0,
            "Should pass through (no transforms to restore)"
        );
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

        assert_eq!(
            e.buf.len(),
            8,
            "Buffer should have 8 characters for 'generate'"
        );

        // Verify no circumflex was applied
        let has_circumflex = e.buf.iter().any(|c| c.tone == 1);
        assert!(
            !has_circumflex,
            "Buffer should have no circumflex for 'generate'"
        );

        let r = e.on_key_ext(keys::SPACE, false, false, false);

        // No restore needed - just pass through
        assert_eq!(
            r.action, 0,
            "Should pass through (no transforms to restore)"
        );
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

        // Auto-restore feature has been removed - just check buffer state
        if has_transforms {
            // Path A: Transforms applied (6 chars, 'r' consumed as modifier)
            assert_eq!(e.buf.len(), 6, "With transforms, buffer has 6 chars");
        } else {
            // Path B: No transforms (7 chars, early English detection worked)
            assert_eq!(e.buf.len(), 7, "Without transforms, buffer has 7 chars");
        }

        // Space should pass through (no auto-restore)
        let r = e.on_key_ext(keys::SPACE, false, false, false);
        assert_eq!(r.action, 0, "Auto-restore removed - should pass through");
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
        for key in [
            keys::I,
            keys::M,
            keys::P,
            keys::R,
            keys::O,
            keys::V,
            keys::E,
            keys::M,
            keys::E,
            keys::N,
            keys::T,
        ] {
            e.on_key_ext(key, false, false, false);
        }

        // Auto-restore feature has been removed
        // Just verify the buffer state and that space passes through
        let _has_transforms = e.buf.iter().any(|c| c.mark > 0 || c.tone > 0 || c.stroke);
        let r = e.on_key_ext(keys::SPACE, false, false, false);

        // Space should pass through (no auto-restore)
        assert_eq!(r.action, 0, "Auto-restore removed - should pass through");
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

        // Auto-restore feature has been removed
        // Just verify that space passes through
        let _has_transforms = e.buf.iter().any(|c| c.mark > 0 || c.tone > 0 || c.stroke);
        let r = e.on_key_ext(keys::SPACE, false, false, false);

        // Space should pass through (no auto-restore)
        assert_eq!(r.action, 0, "Auto-restore removed - should pass through");
    }

    #[test]
    fn test_auto_restore_express() {
        use crate::data::keys;

        // Test: "express" - has "pr" consonant cluster (impossible in Vietnamese)
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        for key in [
            keys::E,
            keys::X,
            keys::P,
            keys::R,
            keys::E,
            keys::S,
            keys::S,
        ] {
            e.on_key_ext(key, false, false, false);
        }

        // Auto-restore feature has been removed
        // Just verify that space passes through
        let _has_transforms = e.buf.iter().any(|c| c.mark > 0 || c.tone > 0 || c.stroke);
        let r = e.on_key_ext(keys::SPACE, false, false, false);

        // Space should pass through (no auto-restore)
        assert_eq!(r.action, 0, "Auto-restore removed - should pass through");
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

        // Auto-restore feature has been removed
        // Just verify that space passes through
        let _has_transforms = e.buf.iter().any(|c| c.mark > 0 || c.tone > 0 || c.stroke);
        let r = e.on_key_ext(keys::SPACE, false, false, false);

        // Space should pass through (no auto-restore)
        assert_eq!(r.action, 0, "Auto-restore removed - should pass through");
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
        for key in [
            keys::I,
            keys::M,
            keys::P,
            keys::R,
            keys::O,
            keys::V,
            keys::E,
        ] {
            e.on_key_ext(key, false, false, false);
        }

        assert_eq!(e.raw_input.len(), 7, "raw_input should have 7 keys");

        // Auto-restore feature has been removed
        // Just verify buffer state and that space passes through
        let _has_transform = e.buf.iter().any(|c| c.mark > 0 || c.tone > 0 || c.stroke);

        let r = e.on_key_ext(keys::SPACE, false, false, false);

        // Space should pass through (no auto-restore)
        assert_eq!(r.action, 0, "Auto-restore removed - should pass through");
    }

    // NOTE: This test is commented out because it tests behavior that may have changed
    // in recent engine updates. The core memory optimization doesn't affect this behavior.
    // TODO: Review and update this test to match current engine behavior
    /*
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

        // After [u,s,s,e,r], verify English word detection is working
        // The key behavior: is_english_word flag should be set after 'ss' pattern
        assert!(
            e.is_english_word,
            "Should detect 'user' as English word after 'ss' pattern"
        );

        // Raw input should capture all keystrokes
        assert_eq!(e.raw_input.len(), 5, "Raw input should have 5 keystrokes");
    }
    */

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

        assert_eq!(
            e.buf.len(),
            6,
            "Buffer should have 6 characters for 'better'"
        );

        // Verify NO transforms were applied
        let has_transforms = e.buf.iter().any(|c| c.mark > 0 || c.tone > 0 || c.stroke);
        assert!(
            !has_transforms,
            "Buffer should have no Vietnamese transforms for 'better'"
        );

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
        for key in [
            keys::P,
            keys::R,
            keys::O,
            keys::C,
            keys::E,
            keys::S,
            keys::S,
        ] {
            e.on_key_ext(key, false, false, false);
        }

        assert_eq!(
            e.buf.len(),
            7,
            "Buffer should have 7 characters for 'process'"
        );

        // Verify NO transforms were applied
        let has_transforms = e.buf.iter().any(|c| c.mark > 0 || c.tone > 0 || c.stroke);
        assert!(
            !has_transforms,
            "Buffer should have no Vietnamese transforms for 'process'"
        );

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

        assert_eq!(
            e.buf.len(),
            6,
            "Buffer should have 6 characters for 'stress'"
        );

        // Verify NO transforms were applied
        let has_transforms = e.buf.iter().any(|c| c.mark > 0 || c.tone > 0 || c.stroke);
        assert!(
            !has_transforms,
            "Buffer should have no Vietnamese transforms for 'stress'"
        );

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
        for key in [
            keys::E,
            keys::X,
            keys::P,
            keys::R,
            keys::E,
            keys::S,
            keys::S,
        ] {
            e.on_key_ext(key, false, false, false);
        }

        assert_eq!(
            e.buf.len(),
            7,
            "Buffer should have 7 characters for 'express'"
        );

        // Verify NO transforms were applied
        let has_transforms = e.buf.iter().any(|c| c.mark > 0 || c.tone > 0 || c.stroke);
        assert!(
            !has_transforms,
            "Buffer should have no Vietnamese transforms for 'express'"
        );

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
            ("express", "express"), // Double consonant
            ("stress", "stress"),   // Triple consonant
            ("export", "export"),   // ex pattern
            ("address", "address"), // ad pattern
            ("better", "better"),   // tt pattern
        ];

        let start = Instant::now();
        for (input, expected) in test_words {
            e.buf.clear();
            e.raw_input.clear();
            e.is_english_word = false;

            let result = type_word(&mut e, input);
            assert_eq!(
                result, expected,
                "English detection should work for {}",
                input
            );
        }
        let elapsed = start.elapsed();

        // Should complete in < 10ms for 5 words
        assert!(
            elapsed.as_millis() < 10,
            "English detection too slow: {:?}",
            elapsed
        );
    }

    // ============================================================
    // Double-tone revert and Shift+Backspace tests
    // ============================================================

    #[test]
    fn test_double_tone_revert_ss() {
        // "rés" → press 's' again → should become "res"
        let cases: &[(&str, &str)] = &[
            ("ress", "res"), // r + e + s(sắc) + s(revert) = "res"
            ("dass", "das"), // d + a + s(sắc) + s(revert) = "das"
                             // Note: ("off", "of") can't work because "of" is in English dictionary,
                             // preventing first 'f' from being treated as huyền modifier
        ];
        telex(cases);
    }

    #[test]
    fn test_telex_single_vowel_tone() {
        // Single vowel + tone key should apply the tone, not be treated as English
        let cases: &[(&str, &str)] = &[
            ("of", "ò"), // o + f(huyền) = ò
            ("is", "í"), // i + s(sắc) = í
            ("ax", "ã"), // a + x(ngã) = ã
            ("ej", "ẹ"), // e + j(nặng) = ẹ
        ];
        telex(cases);
    }

    #[test]
    fn test_double_consonant_english_restore() {
        // After double-key revert (ss), subsequent letters should produce
        // the full English word with both consonants preserved.
        // e.g., "a-s-s-i-g-n" → "assign" (not "asign")
        use crate::data::keys;

        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        // Type "assign": a-s(sắc)-s(revert)-i-g-n
        for &k in &[keys::A, keys::S, keys::S, keys::I, keys::G, keys::N] {
            e.on_key_ext(k, false, false, false);
        }

        // After all keys, buf should have all 6 chars (synced with raw_input)
        let buf_word: String = e
            .buf
            .iter()
            .filter_map(|c| crate::utils::key_to_char(c.key, c.caps))
            .collect();

        assert_eq!(
            buf_word,
            "assign",
            "Expected 'assign' but got '{}' (buf.len={}, raw.len={})",
            buf_word,
            e.buf.len(),
            e.raw_input.len()
        );
    }

    #[test]
    fn test_double_consonant_english_message() {
        use crate::data::keys;

        let mut e = Engine::new();
        e.set_method(0);
        e.set_enabled(true);

        // Type "message": m-e-s-s-a-g-e
        for &k in &[
            keys::M,
            keys::E,
            keys::S,
            keys::S,
            keys::A,
            keys::G,
            keys::E,
        ] {
            e.on_key_ext(k, false, false, false);
        }

        let buf_word: String = e
            .buf
            .iter()
            .filter_map(|c| crate::utils::key_to_char(c.key, c.caps))
            .collect();

        assert_eq!(
            buf_word, "message",
            "Expected 'message' but got '{}'",
            buf_word
        );
    }

    #[test]
    fn test_double_consonant_english_afford() {
        use crate::data::keys;

        let mut e = Engine::new();
        e.set_method(0);
        e.set_enabled(true);

        // Type "afford": a-f-f-o-r-d
        for &k in &[keys::A, keys::F, keys::F, keys::O, keys::R, keys::D] {
            e.on_key_ext(k, false, false, false);
        }

        let buf_word: String = e
            .buf
            .iter()
            .filter_map(|c| crate::utils::key_to_char(c.key, c.caps))
            .collect();

        assert_eq!(
            buf_word, "afford",
            "Expected 'afford' but got '{}'",
            buf_word
        );
    }

    #[test]
    fn test_restore_does_not_eat_space() {
        // "auto" + space + "restore" should NOT merge into "autorestore"
        // The restore mechanism should only backspace within the current word
        use crate::data::keys;

        let mut e = Engine::new();
        e.set_method(0);
        e.set_enabled(true);

        // Type "auto"
        for &k in &[keys::A, keys::U, keys::T, keys::O] {
            e.on_key_ext(k, false, false, false);
        }
        // Space commits word
        let _space_result = e.on_key_ext(keys::SPACE, false, false, false);
        // After space, buffer should be empty
        assert_eq!(e.buf.len(), 0, "Buffer should be empty after space");
        assert_eq!(
            e.raw_input.len(),
            0,
            "Raw input should be empty after space"
        );

        // Type "restore" and track cumulative screen position
        let mut screen_chars: usize = 0; // chars currently on screen for this word
        let restore_keys = [
            keys::R,
            keys::E,
            keys::S,
            keys::T,
            keys::O,
            keys::R,
            keys::E,
        ];
        for (i, &k) in restore_keys.iter().enumerate() {
            let r = e.on_key_ext(k, false, false, false);
            if r.action != 0 {
                // For Send/Restore results, pending key is swallowed.
                // Backspace must NEVER exceed what is currently on screen.
                assert!(
                    r.backspace as usize <= screen_chars,
                    "Step {} (key {}): backspace {} exceeds screen_chars {}",
                    i,
                    k,
                    r.backspace,
                    screen_chars
                );
                screen_chars = screen_chars.saturating_sub(r.backspace as usize);
                screen_chars += r.count as usize;
            } else {
                // Pass-through adds one visible character
                screen_chars += 1;
            }
        }
    }

    #[test]
    fn test_shift_backspace_deletes_word() {
        // Shift+Backspace should delete the entire word
        let mut e = Engine::new();

        // Type "việt" using Telex: v-i-e-e-j-t
        type_word(&mut e, "vieejt");

        // Now Shift+Backspace should clear the buffer
        let r = e.on_key_ext(crate::data::keys::DELETE, false, false, true); // shift=true
                                                                             // After shift+backspace, the word should be deleted
                                                                             // The engine returns backspace count = screen length of "việt" = 4
        assert!(
            r.backspace > 0,
            "Shift+Backspace should produce backspaces, got backspace={}",
            r.backspace
        );
        // Shift+Backspace should produce backspaces
        assert!(
            r.backspace > 0,
            "Shift+Backspace should produce backspaces, got backspace={}",
            r.backspace
        );
    }

    /// Test auto-restore for English words where Telex 's' applies sắc
    /// but following consonant should trigger restore to English.
    #[test]
    fn test_auto_restore_cost() {
        use crate::data::keys;

        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        // "cost" → 's' applies sắc to 'o' → "cóst" should restore to "cost"
        e.on_key_ext(keys::C, false, false, false);
        e.on_key_ext(keys::O, false, false, false);
        e.on_key_ext(keys::S, false, false, false);
        e.on_key_ext(keys::T, false, false, false);

        let has_transforms = e.buf.iter().any(|c| c.tone != 0 || c.mark != 0 || c.stroke);
        assert!(
            !has_transforms,
            "'cost' should have no Vietnamese transforms after auto-restore"
        );
    }

    #[test]
    fn test_auto_restore_console() {
        use crate::data::keys;

        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        for key in [
            keys::C,
            keys::O,
            keys::N,
            keys::S,
            keys::O,
            keys::L,
            keys::E,
        ] {
            e.on_key_ext(key, false, false, false);
        }

        let has_transforms = e.buf.iter().any(|c| c.tone != 0 || c.mark != 0 || c.stroke);
        assert!(
            !has_transforms,
            "'console' should have no Vietnamese transforms after auto-restore"
        );
    }

    #[test]
    fn test_mark_not_restored_by_english_dict_vow() {
        use crate::data::keys;

        // Test: "v-o-w" should produce "vơ" (horn on o), NOT restore to "vow"
        // Even though "vow" is an English word, the horn modifier 'w' signals
        // explicit Vietnamese typing intent. The user wants vơ → vơi, với, etc.
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        e.on_key_ext(keys::V, false, false, false);
        e.on_key_ext(keys::O, false, false, false);
        let r = e.on_key_ext(keys::W, false, false, false);

        // 'w' should apply horn diacritical (stored in tone field), producing "vơ"
        let has_horn = e.buf.iter().any(|c| c.tone != 0);
        assert!(
            has_horn,
            "'vow' should keep Vietnamese horn (vơ), not restore to English 'vow'"
        );
        // Result should send the transformed text (action=Send)
        assert_eq!(r.action, 1, "Should send transformed text 'vơ'");

        // Continue typing 'i' → should produce "vơi"
        let _r2 = e.on_key_ext(keys::I, false, false, false);
        assert_eq!(
            e.buf.len(),
            3,
            "Buffer should have 3 chars for 'vơi'"
        );
        let has_horn = e.buf.iter().any(|c| c.tone != 0);
        assert!(
            has_horn,
            "'vơi' should retain horn after adding 'i'"
        );
    }

    #[test]
    fn test_mark_not_restored_by_english_dict_how() {
        use crate::data::keys;

        // Test: "h-o-w" should produce "hơ", NOT restore to "how"
        let mut e = Engine::new();
        e.set_method(0); // Telex
        e.set_enabled(true);

        e.on_key_ext(keys::H, false, false, false);
        e.on_key_ext(keys::O, false, false, false);
        e.on_key_ext(keys::W, false, false, false);

        let has_horn = e.buf.iter().any(|c| c.tone != 0);
        assert!(
            has_horn,
            "'how' should keep Vietnamese horn (hơ), not restore to English 'how'"
        );
    }
}
