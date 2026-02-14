//! Shortcut Table - Abbreviation expansion
//!
//! Allows users to define shortcuts like "vn" → "Việt Nam"
//! Shortcuts can be specific to input methods (Telex/VNI) or apply to all.

use crate::shared::buffer::MAX;
use std::collections::HashMap;

/// Maximum replacement length in UTF-32 codepoints (matches Result.chars array size)
/// This limit ensures replacement fits in the FFI result buffer.
/// Note: Vietnamese characters with diacritics (ồ, ế, ẫ) count as 1 codepoint each.
pub const MAX_REPLACEMENT_LEN: usize = MAX - 1; // -1 to leave room for trailing space

/// Input method that shortcut applies to
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum InputMethod {
    /// Apply to all input methods
    #[default]
    All,
    /// Apply only to Telex
    Telex,
    /// Apply only to VNI
    Vni,
}

/// Trigger condition for shortcut
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TriggerCondition {
    /// Trigger immediately when buffer matches
    Immediate,
    /// Trigger when word boundary (space, punctuation) is pressed
    OnWordBoundary,
}

/// Case handling mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CaseMode {
    /// Keep replacement exactly as defined
    Exact,
    /// Match case of trigger: "VN" → "VIỆT NAM", "vn" → "Việt Nam"
    MatchCase,
}

/// A single shortcut entry
#[derive(Debug, Clone)]
pub struct Shortcut {
    /// Trigger string (lowercase for matching)
    pub trigger: String,
    /// Replacement text
    pub replacement: String,
    /// When to trigger
    pub condition: TriggerCondition,
    /// How to handle case
    pub case_mode: CaseMode,
    /// Whether this shortcut is enabled
    pub enabled: bool,
    /// Which input method this shortcut applies to
    pub input_method: InputMethod,
}

impl Shortcut {
    /// Validate and truncate replacement if it exceeds MAX_REPLACEMENT_LEN.
    /// Counts UTF-32 codepoints (Vietnamese diacritics = 1 codepoint each).
    fn validate_replacement(replacement: &str) -> String {
        let char_count = replacement.chars().count();
        if char_count <= MAX_REPLACEMENT_LEN {
            replacement.to_string()
        } else {
            // Truncate to MAX_REPLACEMENT_LEN codepoints
            replacement.chars().take(MAX_REPLACEMENT_LEN).collect()
        }
    }

    /// Create a new shortcut with word boundary trigger (applies to all input methods)
    /// Trigger must match exactly (case-sensitive), output is exactly as defined.
    /// Replacement is truncated to MAX_REPLACEMENT_LEN (63) codepoints if too long.
    pub fn new(trigger: &str, replacement: &str) -> Self {
        Self {
            trigger: trigger.to_string(), // Keep original case
            replacement: Self::validate_replacement(replacement),
            condition: TriggerCondition::OnWordBoundary,
            case_mode: CaseMode::Exact, // Exact match, no case transformation
            enabled: true,
            input_method: InputMethod::All,
        }
    }

    /// Create an immediate trigger shortcut (applies to all input methods).
    /// Replacement is truncated to MAX_REPLACEMENT_LEN (63) codepoints if too long.
    pub fn immediate(trigger: &str, replacement: &str) -> Self {
        Self {
            trigger: trigger.to_string(), // Keep original case
            replacement: Self::validate_replacement(replacement),
            condition: TriggerCondition::Immediate,
            case_mode: CaseMode::Exact,
            enabled: true,
            input_method: InputMethod::All,
        }
    }

    /// Create a Telex-specific shortcut with immediate trigger.
    /// Replacement is truncated to MAX_REPLACEMENT_LEN (63) codepoints if too long.
    pub fn telex(trigger: &str, replacement: &str) -> Self {
        Self {
            trigger: trigger.to_string(), // Keep original case
            replacement: Self::validate_replacement(replacement),
            condition: TriggerCondition::Immediate,
            case_mode: CaseMode::Exact,
            enabled: true,
            input_method: InputMethod::Telex,
        }
    }

    /// Create a VNI-specific shortcut with immediate trigger.
    /// Replacement is truncated to MAX_REPLACEMENT_LEN (63) codepoints if too long.
    pub fn vni(trigger: &str, replacement: &str) -> Self {
        Self {
            trigger: trigger.to_string(), // Keep original case
            replacement: Self::validate_replacement(replacement),
            condition: TriggerCondition::Immediate,
            case_mode: CaseMode::Exact,
            enabled: true,
            input_method: InputMethod::Vni,
        }
    }

    /// Set the input method for this shortcut
    pub fn for_method(mut self, method: InputMethod) -> Self {
        self.input_method = method;
        self
    }

    /// Check if shortcut applies to given input method
    ///
    /// - If shortcut is for `All`: matches any method
    /// - If shortcut is for `Telex`: matches `Telex` or `All` query
    /// - If shortcut is for `Vni`: matches `Vni` or `All` query
    pub fn applies_to(&self, query_method: InputMethod) -> bool {
        match self.input_method {
            // Shortcut for All → matches any query
            InputMethod::All => true,
            // Shortcut for specific method → matches if query is same method OR query is All
            InputMethod::Telex => {
                query_method == InputMethod::Telex || query_method == InputMethod::All
            }
            InputMethod::Vni => {
                query_method == InputMethod::Vni || query_method == InputMethod::All
            }
        }
    }
}

/// Shortcut match result
#[derive(Debug)]
pub struct ShortcutMatch {
    /// Number of characters to backspace
    pub backspace_count: usize,
    /// Replacement text to output
    pub output: String,
    /// Whether to include the trigger key in output
    pub include_trigger_key: bool,
}

/// Maximum number of shortcuts allowed (prevents unbounded memory growth)
/// 200 shortcuts is reasonable for most users while preventing memory bloat
const MAX_SHORTCUTS: usize = 200;

/// Shortcut table manager
#[derive(Debug, Default)]
pub struct ShortcutTable {
    /// Shortcuts indexed by trigger (lowercase)
    shortcuts: HashMap<String, Shortcut>,
    /// Sorted triggers by length (longest first) for matching
    sorted_triggers: Vec<String>,
}

impl ShortcutTable {
    pub fn new() -> Self {
        Self {
            shortcuts: HashMap::new(),
            sorted_triggers: vec![],
        }
    }

    /// Create with default Vietnamese shortcuts (common abbreviations)
    ///
    /// Note: "w" → "ư" is NOT a shortcut, it's handled by the engine
    /// as a vowel key with Vietnamese validation.
    ///
    /// Currently disabled - returns empty table
    pub fn with_defaults() -> Self {
        // Temporarily disabled default shortcuts
        Self::new()

        // Original defaults (uncomment to re-enable):
        // let mut table = Self::new();
        // table.add(Shortcut::new("vn", "Việt Nam"));
        // table.add(Shortcut::new("hcm", "Hồ Chí Minh"));
        // table.add(Shortcut::new("hn", "Hà Nội"));
        // table.add(Shortcut::new("dc", "được"));
        // table.add(Shortcut::new("ko", "không"));
        // table
    }

    /// Create with Telex defaults only
    pub fn with_telex_defaults() -> Self {
        // No Telex-specific shortcuts
        // "w" → "ư" is handled by the engine, not shortcuts
        Self::new()
    }

    /// Create with VNI defaults only
    pub fn with_vni_defaults() -> Self {
        Self::new()
    }

    /// Create with all defaults (common abbreviations)
    pub fn with_all_defaults() -> Self {
        let mut table = Self::new();

        // Common abbreviations (apply to all input methods)
        table.add(Shortcut::new("vn", "Việt Nam"));
        table.add(Shortcut::new("hcm", "Hồ Chí Minh"));
        table.add(Shortcut::new("hn", "Hà Nội"));
        table.add(Shortcut::new("dc", "được"));
        table.add(Shortcut::new("ko", "không"));

        table
    }

    /// Add a shortcut
    ///
    /// Returns true if added successfully, false if limit reached
    pub fn add(&mut self, shortcut: Shortcut) -> bool {
        // Check capacity limit (only if adding new shortcut, not replacing)
        if !self.shortcuts.contains_key(&shortcut.trigger) && self.shortcuts.len() >= MAX_SHORTCUTS
        {
            return false;
        }

        let trigger = shortcut.trigger.clone();
        self.shortcuts.insert(trigger.clone(), shortcut);
        self.rebuild_sorted_triggers();
        true
    }

    /// Check if shortcut table is at capacity
    pub fn is_at_capacity(&self) -> bool {
        self.shortcuts.len() >= MAX_SHORTCUTS
    }

    /// Get maximum capacity
    pub fn capacity(&self) -> usize {
        MAX_SHORTCUTS
    }

    /// Remove a shortcut (exact match, case-sensitive)
    pub fn remove(&mut self, trigger: &str) -> Option<Shortcut> {
        let result = self.shortcuts.remove(trigger);
        if result.is_some() {
            self.rebuild_sorted_triggers();
        }
        result
    }

    /// Check if buffer matches any shortcut (for any input method)
    ///
    /// Returns (trigger, shortcut) if match found
    pub fn lookup(&self, buffer: &str) -> Option<(&str, &Shortcut)> {
        self.lookup_for_method(buffer, InputMethod::All)
    }

    /// Check if buffer matches any shortcut for specific input method
    ///
    /// Returns (trigger, shortcut) if match found
    pub fn lookup_for_method(
        &self,
        buffer: &str,
        method: InputMethod,
    ) -> Option<(&str, &Shortcut)> {
        // Longest-match-first, exact case-sensitive match
        for trigger in &self.sorted_triggers {
            if buffer == *trigger {
                if let Some(shortcut) = self.shortcuts.get(trigger) {
                    if shortcut.enabled && shortcut.applies_to(method) {
                        return Some((trigger, shortcut));
                    }
                }
            }
        }
        None
    }

    /// Try to match buffer with trigger key (for any input method)
    ///
    /// # Arguments
    /// * `buffer` - Current buffer content (as string)
    /// * `key_char` - The key that was just pressed
    /// * `is_word_boundary` - Whether key_char is a word boundary
    ///
    /// # Returns
    /// ShortcutMatch if a shortcut should be triggered
    pub fn try_match(
        &self,
        buffer: &str,
        key_char: Option<char>,
        is_word_boundary: bool,
    ) -> Option<ShortcutMatch> {
        self.try_match_for_method(buffer, key_char, is_word_boundary, InputMethod::All)
    }

    /// Try to match buffer with trigger key for specific input method
    ///
    /// # Arguments
    /// * `buffer` - Current buffer content (as string)
    /// * `key_char` - The key that was just pressed
    /// * `is_word_boundary` - Whether key_char is a word boundary
    /// * `method` - The current input method (Telex/VNI)
    ///
    /// # Returns
    /// ShortcutMatch if a shortcut should be triggered
    pub fn try_match_for_method(
        &self,
        buffer: &str,
        key_char: Option<char>,
        is_word_boundary: bool,
        method: InputMethod,
    ) -> Option<ShortcutMatch> {
        let (trigger, shortcut) = self.lookup_for_method(buffer, method)?;

        match shortcut.condition {
            TriggerCondition::Immediate => {
                let output = self.apply_case(buffer, &shortcut.replacement, shortcut.case_mode);
                Some(ShortcutMatch {
                    backspace_count: trigger.len(),
                    output,
                    include_trigger_key: false,
                })
            }
            TriggerCondition::OnWordBoundary => {
                if is_word_boundary {
                    let mut output =
                        self.apply_case(buffer, &shortcut.replacement, shortcut.case_mode);
                    // Append the trigger key (space, etc.)
                    if let Some(ch) = key_char {
                        output.push(ch);
                    }
                    Some(ShortcutMatch {
                        backspace_count: trigger.len(),
                        output,
                        include_trigger_key: true,
                    })
                } else {
                    None
                }
            }
        }
    }

    /// Apply case transformation based on mode
    fn apply_case(&self, trigger: &str, replacement: &str, mode: CaseMode) -> String {
        match mode {
            CaseMode::Exact => replacement.to_string(),
            CaseMode::MatchCase => {
                if trigger.chars().all(|c| c.is_uppercase()) {
                    // All uppercase → replacement all uppercase
                    replacement.to_uppercase()
                } else if trigger
                    .chars()
                    .next()
                    .map(|c| c.is_uppercase())
                    .unwrap_or(false)
                {
                    // First char uppercase → capitalize replacement
                    let mut chars = replacement.chars();
                    match chars.next() {
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                        None => String::new(),
                    }
                } else {
                    // Lowercase → keep replacement as-is
                    replacement.to_string()
                }
            }
        }
    }

    /// Rebuild sorted triggers list (longest first)
    fn rebuild_sorted_triggers(&mut self) {
        self.sorted_triggers = self.shortcuts.keys().cloned().collect();
        self.sorted_triggers
            .sort_by_key(|s| std::cmp::Reverse(s.len()));
    }

    /// Check if shortcut table is empty
    pub fn is_empty(&self) -> bool {
        self.shortcuts.is_empty()
    }

    /// Get number of shortcuts
    pub fn len(&self) -> usize {
        self.shortcuts.len()
    }

    /// Clear all shortcuts
    pub fn clear(&mut self) {
        self.shortcuts.clear();
        self.sorted_triggers.clear();
    }

    /// Get memory usage estimate in bytes
    pub fn memory_usage(&self) -> usize {
        // Estimate: HashMap overhead + Vec overhead + string data
        let hashmap_overhead = self.shortcuts.capacity()
            * (std::mem::size_of::<String>() + std::mem::size_of::<Shortcut>());
        let vec_overhead = self.sorted_triggers.capacity() * std::mem::size_of::<String>();

        let string_data: usize = self
            .shortcuts
            .iter()
            .map(|(trigger, shortcut)| {
                trigger.len() + shortcut.trigger.len() + shortcut.replacement.len()
            })
            .sum();

        hashmap_overhead + vec_overhead + string_data
    }

    // ============================================================
    // JSON Import/Export
    // ============================================================

    /// Escape a string for JSON (handles special characters)
    fn escape_json_string(s: &str) -> String {
        let mut result = String::with_capacity(s.len());
        for c in s.chars() {
            match c {
                '"' => result.push_str("\\\""),
                '\\' => result.push_str("\\\\"),
                '\n' => result.push_str("\\n"),
                '\r' => result.push_str("\\r"),
                '\t' => result.push_str("\\t"),
                c if c.is_control() => {
                    result.push_str(&format!("\\u{:04x}", c as u32));
                }
                c => result.push(c),
            }
        }
        result
    }

    /// Export all shortcuts to JSON string
    ///
    /// Format:
    /// ```json
    /// {
    ///   "version": 1,
    ///   "shortcuts": [
    ///     {"trigger": "vn", "replacement": "Việt Nam", "enabled": true, "method": "all"},
    ///     ...
    ///   ]
    /// }
    /// ```
    pub fn to_json(&self) -> String {
        let mut json = String::from("{\n  \"version\": 1,\n  \"shortcuts\": [\n");
        let shortcuts: Vec<_> = self.shortcuts.values().collect();

        for (i, shortcut) in shortcuts.iter().enumerate() {
            let method_str = match shortcut.input_method {
                InputMethod::All => "all",
                InputMethod::Telex => "telex",
                InputMethod::Vni => "vni",
            };
            let condition_str = match shortcut.condition {
                TriggerCondition::Immediate => "immediate",
                TriggerCondition::OnWordBoundary => "word_boundary",
            };

            json.push_str(&format!(
                "    {{\"trigger\": \"{}\", \"replacement\": \"{}\", \"enabled\": {}, \"method\": \"{}\", \"condition\": \"{}\"}}",
                Self::escape_json_string(&shortcut.trigger),
                Self::escape_json_string(&shortcut.replacement),
                shortcut.enabled,
                method_str,
                condition_str
            ));

            if i < shortcuts.len() - 1 {
                json.push(',');
            }
            json.push('\n');
        }

        json.push_str("  ]\n}");
        json
    }

    /// Import shortcuts from JSON string
    ///
    /// Returns Ok(count) with number of shortcuts imported, or Err with message
    pub fn from_json(&mut self, json: &str) -> Result<usize, &'static str> {
        // Simple JSON parser for our specific format
        // Find shortcuts array
        let shortcuts_start = json
            .find("\"shortcuts\"")
            .and_then(|pos| json[pos..].find('[').map(|p| pos + p))
            .ok_or("Invalid JSON: missing shortcuts array")?;

        let shortcuts_end = json[shortcuts_start..]
            .find(']')
            .map(|p| shortcuts_start + p)
            .ok_or("Invalid JSON: unclosed shortcuts array")?;

        let shortcuts_content = &json[shortcuts_start + 1..shortcuts_end];

        let mut count = 0;
        let mut pos = 0;

        while pos < shortcuts_content.len() {
            // Find next object
            let obj_start = match shortcuts_content[pos..].find('{') {
                Some(p) => pos + p,
                None => break,
            };
            let obj_end = match shortcuts_content[obj_start..].find('}') {
                Some(p) => obj_start + p,
                None => break,
            };

            let obj = &shortcuts_content[obj_start..=obj_end];

            // Parse fields
            if let (Some(trigger), Some(replacement)) = (
                Self::extract_json_string(obj, "trigger"),
                Self::extract_json_string(obj, "replacement"),
            ) {
                let enabled = Self::extract_json_bool(obj, "enabled").unwrap_or(true);
                let method = match Self::extract_json_string(obj, "method").as_deref() {
                    Some("telex") => InputMethod::Telex,
                    Some("vni") => InputMethod::Vni,
                    _ => InputMethod::All,
                };
                let condition = match Self::extract_json_string(obj, "condition").as_deref() {
                    Some("immediate") => TriggerCondition::Immediate,
                    _ => TriggerCondition::OnWordBoundary,
                };

                let mut shortcut = Shortcut::new(&trigger, &replacement);
                shortcut.enabled = enabled;
                shortcut.input_method = method;
                shortcut.condition = condition;

                if self.add(shortcut) {
                    count += 1;
                }
            }

            pos = obj_end + 1;
        }

        Ok(count)
    }

    /// Extract a string value from JSON object
    fn extract_json_string(obj: &str, key: &str) -> Option<String> {
        let key_pattern = format!("\"{}\"", key);
        let key_pos = obj.find(&key_pattern)?;
        let after_key = &obj[key_pos + key_pattern.len()..];

        // Skip whitespace and colon
        let colon_pos = after_key.find(':')?;
        let after_colon = &after_key[colon_pos + 1..];

        // Find opening quote
        let quote_start = after_colon.find('"')?;
        let value_start = quote_start + 1;

        // Find closing quote (handle escapes)
        let value_content = &after_colon[value_start..];
        let mut end_pos = 0;
        let mut escaped = false;

        for (i, c) in value_content.char_indices() {
            if escaped {
                escaped = false;
                continue;
            }
            if c == '\\' {
                escaped = true;
                continue;
            }
            if c == '"' {
                end_pos = i;
                break;
            }
        }

        let raw_value = &value_content[..end_pos];
        Some(Self::unescape_json_string(raw_value))
    }

    /// Unescape a JSON string
    fn unescape_json_string(s: &str) -> String {
        let mut result = String::with_capacity(s.len());
        let mut chars = s.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('n') => result.push('\n'),
                    Some('r') => result.push('\r'),
                    Some('t') => result.push('\t'),
                    Some('"') => result.push('"'),
                    Some('\\') => result.push('\\'),
                    Some('u') => {
                        // Unicode escape \uXXXX
                        let hex: String = chars.by_ref().take(4).collect();
                        if let Ok(code) = u32::from_str_radix(&hex, 16) {
                            if let Some(c) = char::from_u32(code) {
                                result.push(c);
                            }
                        }
                    }
                    Some(c) => result.push(c),
                    None => {}
                }
            } else {
                result.push(c);
            }
        }
        result
    }

    /// Extract a boolean value from JSON object
    fn extract_json_bool(obj: &str, key: &str) -> Option<bool> {
        let key_pattern = format!("\"{}\"", key);
        let key_pos = obj.find(&key_pattern)?;
        let after_key = &obj[key_pos + key_pattern.len()..];

        if after_key.contains("true") {
            let true_pos = after_key.find("true")?;
            let comma_pos = after_key.find(',').unwrap_or(after_key.len());
            let brace_pos = after_key.find('}').unwrap_or(after_key.len());
            if true_pos < comma_pos && true_pos < brace_pos {
                return Some(true);
            }
        }
        if after_key.contains("false") {
            let false_pos = after_key.find("false")?;
            let comma_pos = after_key.find(',').unwrap_or(after_key.len());
            let brace_pos = after_key.find('}').unwrap_or(after_key.len());
            if false_pos < comma_pos && false_pos < brace_pos {
                return Some(false);
            }
        }
        None
    }

    /// Export shortcuts to a Vec of (trigger, replacement) tuples
    /// Useful for simple iteration without full Shortcut details
    pub fn export_all(&self) -> Vec<(String, String)> {
        self.shortcuts
            .values()
            .filter(|s| s.enabled)
            .map(|s| (s.trigger.clone(), s.replacement.clone()))
            .collect()
    }

    /// Import shortcuts from Vec of (trigger, replacement) tuples
    /// Returns number of shortcuts imported
    pub fn import_all(&mut self, shortcuts: Vec<(String, String)>) -> usize {
        let mut count = 0;
        for (trigger, replacement) in shortcuts {
            if self.add(Shortcut::new(&trigger, &replacement)) {
                count += 1;
            }
        }
        count
    }

    /// Get iterator over all shortcuts
    pub fn iter(&self) -> impl Iterator<Item = &Shortcut> {
        self.shortcuts.values()
    }

    /// Get mutable iterator over all shortcuts
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Shortcut> {
        self.shortcuts.values_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: Create table with one word-boundary shortcut
    fn table_with_shortcut(trigger: &str, replacement: &str) -> ShortcutTable {
        let mut table = ShortcutTable::new();
        assert!(table.add(Shortcut::new(trigger, replacement)));
        table
    }

    // Helper: Create table with one immediate shortcut
    fn table_with_immediate(trigger: &str, replacement: &str) -> ShortcutTable {
        let mut table = ShortcutTable::new();
        assert!(table.add(Shortcut::immediate(trigger, replacement)));
        table
    }

    // Helper: Create table with Telex-specific shortcut
    fn table_with_telex_shortcut(trigger: &str, replacement: &str) -> ShortcutTable {
        let mut table = ShortcutTable::new();
        assert!(table.add(Shortcut::telex(trigger, replacement)));
        table
    }

    // Helper: Create table with VNI-specific shortcut
    fn table_with_vni_shortcut(trigger: &str, replacement: &str) -> ShortcutTable {
        let mut table = ShortcutTable::new();
        assert!(table.add(Shortcut::vni(trigger, replacement)));
        table
    }

    // Helper: Assert shortcut matches and check output/backspace
    fn assert_shortcut_match(
        table: &ShortcutTable,
        buffer: &str,
        key_char: Option<char>,
        is_boundary: bool,
        expected_output: &str,
        expected_backspace: usize,
        method: InputMethod,
    ) {
        let result = table.try_match_for_method(buffer, key_char, is_boundary, method);
        assert!(
            result.is_some(),
            "Shortcut should match for buffer: {}",
            buffer
        );
        let m = result.unwrap();
        assert_eq!(m.output, expected_output);
        assert_eq!(m.backspace_count, expected_backspace);
    }

    // Helper: Assert no shortcut match
    fn assert_no_match(
        table: &ShortcutTable,
        buffer: &str,
        key_char: Option<char>,
        is_boundary: bool,
        method: InputMethod,
    ) {
        let result = table.try_match_for_method(buffer, key_char, is_boundary, method);
        assert!(
            result.is_none(),
            "Shortcut should NOT match for buffer: {}",
            buffer
        );
    }

    #[test]
    fn test_basic_shortcut() {
        let table = table_with_shortcut("vn", "Việt Nam");
        assert_shortcut_match(
            &table,
            "vn",
            Some(' '),
            true,
            "Việt Nam ",
            2,
            InputMethod::All,
        );
    }

    #[test]
    fn test_case_matching() {
        let table = table_with_shortcut("vn", "Việt Nam");

        // Exact match (lowercase "vn" matches "vn")
        assert_shortcut_match(
            &table,
            "vn",
            Some(' '),
            true,
            "Việt Nam ",
            2,
            InputMethod::All,
        );

        // Uppercase "VN" does NOT match lowercase "vn" (case-sensitive)
        assert_no_match(&table, "VN", Some(' '), true, InputMethod::All);

        // Title case "Vn" does NOT match lowercase "vn" (case-sensitive)
        assert_no_match(&table, "Vn", Some(' '), true, InputMethod::All);
    }

    #[test]
    fn test_immediate_shortcut() {
        let table = table_with_immediate("w", "ư");

        // Immediate triggers without word boundary
        let result = table.try_match("w", None, false);
        assert!(result.is_some());
        let m = result.unwrap();
        assert_eq!(m.output, "ư");
        assert!(!m.include_trigger_key);
    }

    #[test]
    fn test_word_boundary_required() {
        let table = table_with_shortcut("vn", "Việt Nam");

        // Without word boundary - should not match
        assert_no_match(&table, "vn", Some('a'), false, InputMethod::All);

        // With word boundary - should match
        assert_shortcut_match(
            &table,
            "vn",
            Some(' '),
            true,
            "Việt Nam ",
            2,
            InputMethod::All,
        );
    }

    #[test]
    fn test_longest_match() {
        let mut table = ShortcutTable::new();
        assert!(table.add(Shortcut::new("h", "họ")));
        assert!(table.add(Shortcut::new("hcm", "Hồ Chí Minh")));

        // "hcm" should match the longer shortcut
        let (trigger, _) = table.lookup("hcm").unwrap();
        assert_eq!(trigger, "hcm");
    }

    #[test]
    fn test_disabled_shortcut() {
        let mut table = ShortcutTable::new();
        let mut shortcut = Shortcut::new("vn", "Việt Nam");
        shortcut.enabled = false;
        assert!(table.add(shortcut));

        let result = table.lookup("vn");
        assert!(result.is_none());
    }

    #[test]
    fn test_telex_specific_shortcut() {
        let table = table_with_telex_shortcut("w", "ư");

        // Should match for Telex
        assert_shortcut_match(&table, "w", None, false, "ư", 1, InputMethod::Telex);

        // Should NOT match for VNI
        assert_no_match(&table, "w", None, false, InputMethod::Vni);

        // Should match for All (fallback)
        assert_shortcut_match(&table, "w", None, false, "ư", 1, InputMethod::All);
    }

    #[test]
    fn test_vni_specific_shortcut() {
        let table = table_with_vni_shortcut("7", "ơ");

        // Should match for VNI
        assert_shortcut_match(&table, "7", None, false, "ơ", 1, InputMethod::Vni);

        // Should NOT match for Telex
        assert_no_match(&table, "7", None, false, InputMethod::Telex);
    }

    #[test]
    fn test_all_input_method_shortcut() {
        let table = table_with_shortcut("vn", "Việt Nam");

        // Should match for Telex
        assert_shortcut_match(
            &table,
            "vn",
            Some(' '),
            true,
            "Việt Nam ",
            2,
            InputMethod::Telex,
        );

        // Should match for VNI
        assert_shortcut_match(
            &table,
            "vn",
            Some(' '),
            true,
            "Việt Nam ",
            2,
            InputMethod::Vni,
        );

        // Should match for All
        assert_shortcut_match(
            &table,
            "vn",
            Some(' '),
            true,
            "Việt Nam ",
            2,
            InputMethod::All,
        );
    }

    #[test]
    fn test_with_defaults_has_common_shortcuts() {
        let table = ShortcutTable::with_defaults();

        // Default shortcuts are currently disabled - table should be empty
        assert!(table.is_empty());

        // "vn" → "Việt Nam" should NOT exist (disabled)
        let result = table.lookup_for_method("vn", InputMethod::All);
        assert!(result.is_none());

        // "w" is NOT a shortcut (handled by engine)
        let result = table.lookup_for_method("w", InputMethod::Telex);
        assert!(result.is_none());
    }

    #[test]
    fn test_shortcut_for_method_builder() {
        let shortcut = Shortcut::new("test", "Test").for_method(InputMethod::Telex);
        assert_eq!(shortcut.input_method, InputMethod::Telex);

        let shortcut = Shortcut::immediate("x", "y").for_method(InputMethod::Vni);
        assert_eq!(shortcut.input_method, InputMethod::Vni);
    }

    #[test]
    fn test_applies_to() {
        let all_shortcut = Shortcut::new("vn", "Việt Nam");
        assert!(all_shortcut.applies_to(InputMethod::All));
        assert!(all_shortcut.applies_to(InputMethod::Telex));
        assert!(all_shortcut.applies_to(InputMethod::Vni));

        let telex_shortcut = Shortcut::telex("test", "Test");
        assert!(telex_shortcut.applies_to(InputMethod::All));
        assert!(telex_shortcut.applies_to(InputMethod::Telex));
        assert!(!telex_shortcut.applies_to(InputMethod::Vni));

        let vni_shortcut = Shortcut::vni("7", "ơ");
        assert!(vni_shortcut.applies_to(InputMethod::All));
        assert!(!vni_shortcut.applies_to(InputMethod::Telex));
        assert!(vni_shortcut.applies_to(InputMethod::Vni));
    }

    #[test]
    fn test_replacement_validation_within_limit() {
        // Vietnamese text within limit (21 codepoints)
        let shortcut = Shortcut::new("tphcm", "Thành phố Hồ Chí Minh");
        assert_eq!(shortcut.replacement, "Thành phố Hồ Chí Minh");
        assert_eq!(shortcut.replacement.chars().count(), 21);
    }

    #[test]
    fn test_replacement_validation_truncation() {
        // Create a very long replacement that exceeds MAX_REPLACEMENT_LEN (255 chars)
        // Text is repeated to ensure it's long enough
        let base_text = "Đây là một đoạn văn bản tiếng Việt. ";
        let long_text: String = base_text.repeat(10); // ~360 chars
        let char_count = long_text.chars().count();
        assert!(
            char_count > MAX_REPLACEMENT_LEN,
            "Test text ({} chars) should exceed limit ({})",
            char_count,
            MAX_REPLACEMENT_LEN
        );

        let shortcut = Shortcut::new("long", &long_text);
        let result_count = shortcut.replacement.chars().count();
        assert_eq!(
            result_count, MAX_REPLACEMENT_LEN,
            "Should truncate to MAX_REPLACEMENT_LEN"
        );
    }

    #[test]
    fn test_replacement_validation_vietnamese_diacritics() {
        // Each Vietnamese character with diacritic is 1 codepoint
        // "ồ" = 1 codepoint, "ế" = 1 codepoint, "ẫ" = 1 codepoint
        let vietnamese = "ồếẫơưáàảãạăắằẳẵặâấầẩẫậ"; // 22 Vietnamese chars
        let shortcut = Shortcut::new("viet", vietnamese);
        assert_eq!(shortcut.replacement.chars().count(), 22);
        assert_eq!(shortcut.replacement, vietnamese);
    }

    #[test]
    fn test_bounded_capacity() {
        let mut table = ShortcutTable::new();

        // Add shortcuts up to capacity
        for i in 0..MAX_SHORTCUTS {
            let trigger = format!("trigger{}", i);
            let replacement = format!("replacement{}", i);
            let result = table.add(Shortcut::new(&trigger, &replacement));
            assert!(
                result,
                "Should be able to add shortcut {} of {}",
                i + 1,
                MAX_SHORTCUTS
            );
        }

        // Verify we're at capacity
        assert_eq!(table.len(), MAX_SHORTCUTS);
        assert!(table.is_at_capacity());

        // Try to add one more - should fail
        let result = table.add(Shortcut::new("overflow", "should fail"));
        assert!(!result, "Should not be able to add beyond capacity");
        assert_eq!(
            table.len(),
            MAX_SHORTCUTS,
            "Length should remain at capacity"
        );
    }

    #[test]
    fn test_capacity_replace_existing() {
        let mut table = ShortcutTable::new();

        // Fill to capacity
        for i in 0..MAX_SHORTCUTS {
            let trigger = format!("t{}", i);
            assert!(table.add(Shortcut::new(&trigger, "original")));
        }

        assert!(table.is_at_capacity());

        // Replacing existing shortcut should still work
        let result = table.add(Shortcut::new("t0", "updated"));
        assert!(result, "Should be able to replace existing shortcut");

        // Verify replacement happened
        let (_, shortcut) = table.lookup("t0").unwrap();
        assert_eq!(shortcut.replacement, "updated");
        assert_eq!(table.len(), MAX_SHORTCUTS);
    }

    #[test]
    fn test_capacity_methods() {
        let table = ShortcutTable::new();

        assert_eq!(table.capacity(), MAX_SHORTCUTS);
        assert!(!table.is_at_capacity());
        assert_eq!(table.len(), 0);
    }

    #[test]
    fn test_memory_usage_estimate() {
        let mut table = ShortcutTable::new();

        // Empty table should have minimal memory
        let empty_usage = table.memory_usage();
        // Note: usize is always >= 0, so we just verify it exists

        // Add some shortcuts
        table.add(Shortcut::new("vn", "Việt Nam"));
        table.add(Shortcut::new("hcm", "Hồ Chí Minh"));

        let usage_with_shortcuts = table.memory_usage();
        assert!(
            usage_with_shortcuts > empty_usage,
            "Memory usage should increase with shortcuts"
        );
    }

    #[test]
    fn test_clear_resets_capacity_check() {
        let mut table = ShortcutTable::new();

        // Fill to capacity
        for i in 0..MAX_SHORTCUTS {
            assert!(table.add(Shortcut::new(&format!("t{}", i), "test")));
        }

        assert!(table.is_at_capacity());

        // Clear and verify we can add again
        table.clear();
        assert!(!table.is_at_capacity());
        assert_eq!(table.len(), 0);

        // Should be able to add again
        assert!(table.add(Shortcut::new("new", "test")));
    }

    // ============================================================
    // JSON Import/Export Tests
    // ============================================================

    #[test]
    fn test_to_json_basic() {
        let mut table = ShortcutTable::new();
        table.add(Shortcut::new("vn", "Việt Nam"));

        let json = table.to_json();
        assert!(json.contains("\"version\": 1"));
        assert!(json.contains("\"shortcuts\""));
        assert!(json.contains("\"trigger\": \"vn\""));
        assert!(json.contains("\"replacement\": \"Việt Nam\""));
        assert!(json.contains("\"enabled\": true"));
        assert!(json.contains("\"method\": \"all\""));
    }

    #[test]
    fn test_to_json_empty() {
        let table = ShortcutTable::new();
        let json = table.to_json();
        assert!(json.contains("\"version\": 1"));
        assert!(json.contains("\"shortcuts\": ["));
        assert!(json.contains("]"));
    }

    #[test]
    fn test_from_json_basic() {
        let json = r#"{
            "version": 1,
            "shortcuts": [
                {"trigger": "vn", "replacement": "Việt Nam", "enabled": true, "method": "all", "condition": "word_boundary"}
            ]
        }"#;

        let mut table = ShortcutTable::new();
        let count = table.from_json(json).unwrap();
        assert_eq!(count, 1);
        assert_eq!(table.len(), 1);

        let (_, shortcut) = table.lookup("vn").unwrap();
        assert_eq!(shortcut.replacement, "Việt Nam");
    }

    #[test]
    fn test_json_roundtrip() {
        let mut table = ShortcutTable::new();
        table.add(Shortcut::new("vn", "Việt Nam"));
        table.add(Shortcut::telex("hcm", "Hồ Chí Minh"));
        table.add(Shortcut::immediate("dc", "được"));

        // Export to JSON
        let json = table.to_json();

        // Import to new table
        let mut table2 = ShortcutTable::new();
        let count = table2.from_json(&json).unwrap();
        assert_eq!(count, 3);

        // Verify all shortcuts exist
        assert!(table2.lookup("vn").is_some());
        assert!(table2.lookup("hcm").is_some());
        assert!(table2.lookup("dc").is_some());
    }

    #[test]
    fn test_json_vietnamese_special_chars() {
        let vietnamese_text = "Thành phố Hồ Chí Minh – TP.HCM (đẹp nhất)";
        let mut table = ShortcutTable::new();
        table.add(Shortcut::new("tphcm", vietnamese_text));

        let json = table.to_json();
        let mut table2 = ShortcutTable::new();
        table2.from_json(&json).unwrap();

        let (_, shortcut) = table2.lookup("tphcm").unwrap();
        assert_eq!(shortcut.replacement, vietnamese_text);
    }

    #[test]
    fn test_json_escape_special_chars() {
        let text_with_quotes = "He said \"hello\"";
        let text_with_newline = "Line1\nLine2";

        let mut table = ShortcutTable::new();
        table.add(Shortcut::new("q", text_with_quotes));
        table.add(Shortcut::new("nl", text_with_newline));

        let json = table.to_json();
        assert!(json.contains("\\\"hello\\\"")); // Escaped quotes
        assert!(json.contains("\\n")); // Escaped newline

        let mut table2 = ShortcutTable::new();
        table2.from_json(&json).unwrap();

        let (_, sq) = table2.lookup("q").unwrap();
        assert_eq!(sq.replacement, text_with_quotes);

        let (_, snl) = table2.lookup("nl").unwrap();
        assert_eq!(snl.replacement, text_with_newline);
    }

    #[test]
    fn test_from_json_invalid() {
        let mut table = ShortcutTable::new();

        // Missing shortcuts array
        let result = table.from_json("{}");
        assert!(result.is_err());

        // Invalid JSON
        let result = table.from_json("not json");
        assert!(result.is_err());
    }

    #[test]
    fn test_export_all() {
        let mut table = ShortcutTable::new();
        table.add(Shortcut::new("vn", "Việt Nam"));
        table.add(Shortcut::new("hcm", "Hồ Chí Minh"));

        // Disable one
        if let Some(s) = table.shortcuts.get_mut("hcm") {
            s.enabled = false;
        }

        let exported = table.export_all();
        assert_eq!(exported.len(), 1); // Only enabled shortcuts
        assert_eq!(exported[0], ("vn".to_string(), "Việt Nam".to_string()));
    }

    #[test]
    fn test_import_all() {
        let mut table = ShortcutTable::new();
        let shortcuts = vec![
            ("vn".to_string(), "Việt Nam".to_string()),
            ("hcm".to_string(), "Hồ Chí Minh".to_string()),
        ];

        let count = table.import_all(shortcuts);
        assert_eq!(count, 2);
        assert_eq!(table.len(), 2);
        assert!(table.lookup("vn").is_some());
        assert!(table.lookup("hcm").is_some());
    }

    #[test]
    fn test_iter() {
        let mut table = ShortcutTable::new();
        table.add(Shortcut::new("a", "A"));
        table.add(Shortcut::new("b", "B"));

        let shortcuts: Vec<_> = table.iter().collect();
        assert_eq!(shortcuts.len(), 2);
    }
}
