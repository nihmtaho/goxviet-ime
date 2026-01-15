//! Engine Configuration
//!
//! This module provides configuration options for the Vietnamese IME engine.
//! Extracting configuration into a dedicated struct enables:
//! - Cleaner Engine struct with fewer fields
//! - Easy serialization/deserialization for persistence
//! - Type-safe configuration management
//! - Future extensibility without modifying Engine struct
//!
//! # Configuration Options
//!
//! - **Input Method**: Telex (0), VNI (1), or All (2)
//! - **Skip W Shortcut**: Whether `w` → `ư` conversion is disabled at word start
//! - **ESC Restore**: Whether ESC key restores raw ASCII input
//! - **Free Tone**: Whether to allow diacritics anywhere (skip validation)
//! - **Modern Tone**: Whether to use modern orthography (hoà vs hòa)
//!
//! # Example
//!
//! ```ignore
//! let mut config = EngineConfig::default();
//! config.set_method(InputMethod::Telex);
//! config.set_modern_tone(true);
//!
//! let engine = Engine::with_config(config);
//! ```

/// Input method type
///
/// Defines which Vietnamese input method to use for key processing.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum InputMethod {
    /// Telex input method (default)
    /// - Marks: s=sắc, f=huyền, r=hỏi, x=ngã, j=nặng
    /// - Tones: aa=â, aw=ă, ee=ê, oo=ô, ow=ơ, w=ư
    /// - Stroke: dd=đ
    /// - Remove: z
    #[default]
    Telex = 0,

    /// VNI input method
    /// - Marks: 1=sắc, 2=huyền, 3=hỏi, 4=ngã, 5=nặng
    /// - Tones: 6=circumflex, 7=horn, 8=breve
    /// - Stroke: 9=đ
    /// - Remove: 0
    Vni = 1,

    /// All/Plain mode - no Vietnamese transforms
    /// Pass through all keys without modification
    All = 2,
}

impl InputMethod {
    /// Convert from u8 method ID
    ///
    /// # Arguments
    /// * `id` - Method ID: 0=Telex, 1=VNI, anything else=All
    pub fn from_id(id: u8) -> Self {
        match id {
            0 => InputMethod::Telex,
            1 => InputMethod::Vni,
            _ => InputMethod::All,
        }
    }

    /// Convert to u8 method ID
    pub fn to_id(self) -> u8 {
        self as u8
    }

    /// Check if this method supports Vietnamese transforms
    pub fn supports_transforms(self) -> bool {
        matches!(self, InputMethod::Telex | InputMethod::Vni)
    }
}

/// Engine configuration options
///
/// Contains all configurable options for the Vietnamese IME engine.
/// This struct can be serialized/deserialized for persistence.
///
/// # Defaults
///
/// - `method`: Telex
/// - `enabled`: true
/// - `skip_w_shortcut`: false
/// - `esc_restore_enabled`: false
/// - `free_tone_enabled`: false
/// - `modern_tone`: true
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EngineConfig {
    /// Current input method (Telex, VNI, or All)
    pub method: InputMethod,

    /// Whether the engine is enabled
    /// When disabled, all keys are passed through without processing
    pub enabled: bool,

    /// Skip w→ư shortcut in Telex mode at word start
    ///
    /// When `true`, typing 'w' at word start stays as 'w' instead of 'ư'.
    /// This is useful for users who frequently type English words starting with 'w'.
    ///
    /// Note: This only affects the first 'w' in a word. After consonants like
    /// "nh", "w" will still produce "ư" (e.g., "nhw" → "như").
    pub skip_w_shortcut: bool,

    /// Enable ESC key to restore raw ASCII
    ///
    /// When `true` (default), pressing ESC restores the original keystrokes
    /// before Vietnamese transformation. This is useful for correcting mistakes.
    ///
    /// When `false`, ESC key is passed through to the application.
    pub esc_restore_enabled: bool,

    /// Enable free tone placement (skip validation)
    ///
    /// When `true`, allows placing diacritics anywhere without Vietnamese
    /// spelling validation. This enables typing non-standard combinations
    /// like "Zìa" which would normally be rejected.
    ///
    /// When `false` (default), validates Vietnamese spelling rules before
    /// applying transformations.
    pub free_tone_enabled: bool,

    /// Use modern orthography for tone placement
    ///
    /// Controls where tone marks are placed in vowel clusters:
    /// - `true` (modern): hoà, thuý (tone on second vowel)
    /// - `false` (traditional): hòa, thúy (tone on first vowel)
    ///
    /// Modern style is the default and is recommended by Vietnamese
    /// language authorities.
    pub modern_tone: bool,

    /// Enable instant auto-restore for English words
    ///
    /// When `true` (default), restores English words immediately upon detection
    /// without waiting for a space character.
    pub instant_restore_enabled: bool,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            method: InputMethod::Telex,
            enabled: true,
            skip_w_shortcut: true,
            esc_restore_enabled: false, // Default OFF per user request
            free_tone_enabled: true,
            modern_tone: true, // Modern style (hoà, thuý)
            instant_restore_enabled: true,
        }
    }
}

impl EngineConfig {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a Telex configuration
    pub fn telex() -> Self {
        Self {
            method: InputMethod::Telex,
            ..Default::default()
        }
    }

    /// Create a VNI configuration
    pub fn vni() -> Self {
        Self {
            method: InputMethod::Vni,
            ..Default::default()
        }
    }

    /// Set the input method
    pub fn set_method(&mut self, method: InputMethod) -> &mut Self {
        self.method = method;
        self
    }

    /// Set the input method by ID (0=Telex, 1=VNI, other=All)
    pub fn set_method_id(&mut self, id: u8) -> &mut Self {
        self.method = InputMethod::from_id(id);
        self
    }

    /// Set whether the engine is enabled
    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = enabled;
        self
    }

    /// Set whether to skip w→ư shortcut at word start
    pub fn set_skip_w_shortcut(&mut self, skip: bool) -> &mut Self {
        self.skip_w_shortcut = skip;
        self
    }

    /// Set whether ESC key restores raw ASCII
    pub fn set_esc_restore(&mut self, enabled: bool) -> &mut Self {
        self.esc_restore_enabled = enabled;
        self
    }

    /// Set whether to enable free tone placement
    pub fn set_free_tone(&mut self, enabled: bool) -> &mut Self {
        self.free_tone_enabled = enabled;
        self
    }

    /// Set whether to use modern orthography
    pub fn set_modern_tone(&mut self, modern: bool) -> &mut Self {
        self.modern_tone = modern;
        self
    }

    /// Check if Vietnamese transforms should be applied
    ///
    /// Returns `true` if:
    /// - Engine is enabled
    /// - Method supports transforms (Telex or VNI)
    pub fn should_transform(&self) -> bool {
        self.enabled && self.method.supports_transforms()
    }
}

// ============================================================
// Builder Pattern (Alternative)
// ============================================================

/// Builder for EngineConfig
///
/// Provides a fluent API for constructing configuration.
///
/// # Example
///
/// ```ignore
/// let config = EngineConfigBuilder::new()
///     .method(InputMethod::Telex)
///     .modern_tone(true)
///     .esc_restore(true)
///     .build();
/// ```
#[derive(Clone, Debug, Default)]
pub struct EngineConfigBuilder {
    config: EngineConfig,
}

impl EngineConfigBuilder {
    /// Create a new builder with default configuration
    pub fn new() -> Self {
        Self {
            config: EngineConfig::default(),
        }
    }

    /// Set the input method
    pub fn method(mut self, method: InputMethod) -> Self {
        self.config.method = method;
        self
    }

    /// Set whether the engine is enabled
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.config.enabled = enabled;
        self
    }

    /// Set whether to skip w→ư shortcut
    pub fn skip_w_shortcut(mut self, skip: bool) -> Self {
        self.config.skip_w_shortcut = skip;
        self
    }

    /// Set whether ESC restores raw ASCII
    pub fn esc_restore(mut self, enabled: bool) -> Self {
        self.config.esc_restore_enabled = enabled;
        self
    }

    /// Set whether to enable free tone placement
    pub fn free_tone(mut self, enabled: bool) -> Self {
        self.config.free_tone_enabled = enabled;
        self
    }

    /// Set whether to use modern orthography
    pub fn modern_tone(mut self, modern: bool) -> Self {
        self.config.modern_tone = modern;
        self
    }

    /// Build the configuration
    pub fn build(self) -> EngineConfig {
        self.config
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_method_from_id() {
        assert_eq!(InputMethod::from_id(0), InputMethod::Telex);
        assert_eq!(InputMethod::from_id(1), InputMethod::Vni);
        assert_eq!(InputMethod::from_id(2), InputMethod::All);
        assert_eq!(InputMethod::from_id(255), InputMethod::All);
    }

    #[test]
    fn test_input_method_to_id() {
        assert_eq!(InputMethod::Telex.to_id(), 0);
        assert_eq!(InputMethod::Vni.to_id(), 1);
        assert_eq!(InputMethod::All.to_id(), 2);
    }

    #[test]
    fn test_input_method_supports_transforms() {
        assert!(InputMethod::Telex.supports_transforms());
        assert!(InputMethod::Vni.supports_transforms());
        assert!(!InputMethod::All.supports_transforms());
    }

    #[test]
    fn test_default_config() {
        let config = EngineConfig::default();
        assert_eq!(config.method, InputMethod::Telex);
        assert!(config.enabled);
        assert!(config.skip_w_shortcut); // Default is now true
        assert!(!config.esc_restore_enabled);
        assert!(config.free_tone_enabled); // Default is now true
        assert!(config.modern_tone);
    }

    #[test]
    fn test_telex_config() {
        let config = EngineConfig::telex();
        assert_eq!(config.method, InputMethod::Telex);
    }

    #[test]
    fn test_vni_config() {
        let config = EngineConfig::vni();
        assert_eq!(config.method, InputMethod::Vni);
    }

    #[test]
    fn test_config_setters() {
        let mut config = EngineConfig::new();
        config
            .set_method(InputMethod::Vni)
            .set_enabled(false)
            .set_skip_w_shortcut(true)
            .set_esc_restore(true)
            .set_free_tone(true)
            .set_modern_tone(false);

        assert_eq!(config.method, InputMethod::Vni);
        assert!(!config.enabled);
        assert!(config.skip_w_shortcut);
        assert!(config.esc_restore_enabled);
        assert!(config.free_tone_enabled);
        assert!(!config.modern_tone);
    }

    #[test]
    fn test_set_method_id() {
        let mut config = EngineConfig::new();
        config.set_method_id(1);
        assert_eq!(config.method, InputMethod::Vni);
    }

    #[test]
    fn test_should_transform() {
        let mut config = EngineConfig::new();

        // Default: enabled + Telex = should transform
        assert!(config.should_transform());

        // Disabled = should not transform
        config.set_enabled(false);
        assert!(!config.should_transform());

        // Enabled + All = should not transform
        config.set_enabled(true).set_method(InputMethod::All);
        assert!(!config.should_transform());

        // Enabled + VNI = should transform
        config.set_method(InputMethod::Vni);
        assert!(config.should_transform());
    }

    #[test]
    fn test_builder() {
        let config = EngineConfigBuilder::new()
            .method(InputMethod::Vni)
            .enabled(true)
            .skip_w_shortcut(true)
            .esc_restore(true)
            .free_tone(false)
            .modern_tone(false)
            .build();

        assert_eq!(config.method, InputMethod::Vni);
        assert!(config.enabled);
        assert!(config.skip_w_shortcut);
        assert!(config.esc_restore_enabled);
        assert!(!config.free_tone_enabled);
        assert!(!config.modern_tone);
    }

    #[test]
    fn test_builder_default() {
        let config = EngineConfigBuilder::new().build();
        assert_eq!(config, EngineConfig::default());
    }

    #[test]
    fn test_config_clone() {
        let config1 = EngineConfig::telex();
        let config2 = config1.clone();
        assert_eq!(config1, config2);
    }

    #[test]
    fn test_input_method_default() {
        assert_eq!(InputMethod::default(), InputMethod::Telex);
    }
}
