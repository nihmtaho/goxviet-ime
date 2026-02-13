//! Engine Configuration DTO
//!
//! Data Transfer Object for engine configuration settings.
//!
//! # Design Principles
//!
//! - **Plain Data**: No business logic, just data holders
//! - **Cross-Layer Transfer**: Safely passes config between layers
//! - **Validation Separated**: Validation logic in domain/services, not in DTO
//!
//! # Usage
//!
//! DTOs are used by Application Layer to:
//! - Receive configuration from Presentation Layer (FFI)
//! - Pass settings to Infrastructure Layer (adapters)
//! - Store defaults and user preferences

use crate::domain::ports::input::InputMethodId;
use crate::domain::ports::transformation::ToneStrategy;

/// Engine configuration settings
///
/// Represents all configurable options for the Vietnamese IME engine.
///
/// # Examples
///
/// ```
/// # use goxviet_core::application::dto::EngineConfig;
/// # use goxviet_core::domain::ports::input::InputMethodId;
/// let config = EngineConfig::default();
/// assert_eq!(config.input_method, InputMethodId::Telex);
/// assert!(config.enabled);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct EngineConfig {
    /// Input method (Telex, VNI, Plain)
    pub input_method: InputMethodId,

    /// Tone positioning strategy (Modern or Traditional)
    pub tone_strategy: ToneStrategy,

    /// Enable/disable Vietnamese transformations
    pub enabled: bool,

    /// Enable smart mode (auto-detect Vietnamese vs English)
    pub smart_mode: bool,

    /// Enable spell checking
    pub spell_check: bool,

    /// Enable auto-correct
    pub auto_correct: bool,

    /// Maximum undo history size
    pub max_history_size: usize,

    /// Buffer timeout in milliseconds (0 = no timeout)
    pub buffer_timeout_ms: u32,

    /// Modern vs Traditional tone placement
    pub use_modern_tone_placement: bool,
    
    /// Enable text expansion shortcuts
    pub enable_shortcuts: bool,

    /// Enable instant auto-restore (restore English when Vietnamese is invalid)
    pub instant_restore_enabled: bool,

    /// Enable ESC key restore (restore original text on ESC)
    pub esc_restore_enabled: bool,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            input_method: InputMethodId::default(),
            tone_strategy: ToneStrategy::default(),
            enabled: true,
            smart_mode: true,
            spell_check: true,
            auto_correct: false,
            max_history_size: 100,
            buffer_timeout_ms: 1000,
            use_modern_tone_placement: true,
            enable_shortcuts: true,
            instant_restore_enabled: true,
            esc_restore_enabled: false,
        }
    }
}

impl EngineConfig {
    /// Creates a new configuration with defaults
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::application::dto::EngineConfig;
    /// let config = EngineConfig::new();
    /// assert!(config.enabled);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates configuration for Telex input method
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::application::dto::EngineConfig;
    /// # use goxviet_core::domain::ports::input::InputMethodId;
    /// let config = EngineConfig::telex();
    /// assert_eq!(config.input_method, InputMethodId::Telex);
    /// ```
    pub fn telex() -> Self {
        Self {
            input_method: InputMethodId::Telex,
            ..Default::default()
        }
    }

    /// Creates configuration for VNI input method
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::application::dto::EngineConfig;
    /// # use goxviet_core::domain::ports::input::InputMethodId;
    /// let config = EngineConfig::vni();
    /// assert_eq!(config.input_method, InputMethodId::Vni);
    /// ```
    pub fn vni() -> Self {
        Self {
            input_method: InputMethodId::Vni,
            ..Default::default()
        }
    }

    /// Creates configuration with all features disabled
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::application::dto::EngineConfig;
    /// let config = EngineConfig::disabled();
    /// assert!(!config.enabled);
    /// assert!(!config.smart_mode);
    /// assert!(!config.spell_check);
    /// ```
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            smart_mode: false,
            spell_check: false,
            auto_correct: false,
            ..Default::default()
        }
    }

    /// Builder pattern: Set input method
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::application::dto::EngineConfig;
    /// # use goxviet_core::domain::ports::input::InputMethodId;
    /// let config = EngineConfig::new()
    ///     .with_input_method(InputMethodId::Vni);
    /// assert_eq!(config.input_method, InputMethodId::Vni);
    /// ```
    pub fn with_input_method(mut self, method: InputMethodId) -> Self {
        self.input_method = method;
        self
    }

    /// Builder pattern: Set tone strategy
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::application::dto::EngineConfig;
    /// # use goxviet_core::domain::ports::transformation::ToneStrategy;
    /// let config = EngineConfig::new()
    ///     .with_tone_strategy(ToneStrategy::Traditional);
    /// assert_eq!(config.tone_strategy, ToneStrategy::Traditional);
    /// ```
    pub fn with_tone_strategy(mut self, strategy: ToneStrategy) -> Self {
        self.tone_strategy = strategy;
        self
    }

    /// Builder pattern: Set enabled state
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::application::dto::EngineConfig;
    /// let config = EngineConfig::new().with_enabled(false);
    /// assert!(!config.enabled);
    /// ```
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Builder pattern: Set smart mode
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::application::dto::EngineConfig;
    /// let config = EngineConfig::new().with_smart_mode(false);
    /// assert!(!config.smart_mode);
    /// ```
    pub fn with_smart_mode(mut self, enabled: bool) -> Self {
        self.smart_mode = enabled;
        self
    }

    /// Validates configuration consistency
    ///
    /// # Returns
    ///
    /// - `Ok(())` if configuration is valid
    /// - `Err(String)` with error message if invalid
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::application::dto::EngineConfig;
    /// let config = EngineConfig::new();
    /// assert!(config.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        if self.max_history_size == 0 {
            return Err("max_history_size must be greater than 0".to_string());
        }

        if self.buffer_timeout_ms > 10000 {
            return Err("buffer_timeout_ms must be <= 10000 (10 seconds)".to_string());
        }

        Ok(())
    }

    // Getters for reading configuration

    /// Gets the input method
    pub fn input_method(&self) -> InputMethodId {
        self.input_method
    }

    /// Gets the tone strategy
    pub fn tone_strategy(&self) -> ToneStrategy {
        self.tone_strategy
    }

    /// Checks if engine is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Checks if smart mode is enabled
    pub fn is_smart_mode_enabled(&self) -> bool {
        self.smart_mode
    }

    /// Checks if spell checking is enabled
    pub fn is_spell_check_enabled(&self) -> bool {
        self.spell_check
    }

    /// Checks if auto-correct is enabled
    pub fn is_auto_correct_enabled(&self) -> bool {
        self.auto_correct
    }

    /// Gets max history size
    pub fn max_history_size(&self) -> usize {
        self.max_history_size
    }

    /// Gets buffer timeout in milliseconds
    pub fn buffer_timeout_ms(&self) -> u32 {
        self.buffer_timeout_ms
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = EngineConfig::default();
        assert_eq!(config.input_method, InputMethodId::Telex);
        assert_eq!(config.tone_strategy, ToneStrategy::Modern);
        assert!(config.enabled);
        assert!(config.smart_mode);
        assert_eq!(config.max_history_size, 100);
    }

    #[test]
    fn test_config_new() {
        let config = EngineConfig::new();
        assert!(config.enabled);
        assert!(config.smart_mode);
    }

    #[test]
    fn test_config_telex() {
        let config = EngineConfig::telex();
        assert_eq!(config.input_method, InputMethodId::Telex);
        assert!(config.enabled);
    }

    #[test]
    fn test_config_vni() {
        let config = EngineConfig::vni();
        assert_eq!(config.input_method, InputMethodId::Vni);
        assert!(config.enabled);
    }

    #[test]
    fn test_config_disabled() {
        let config = EngineConfig::disabled();
        assert!(!config.enabled);
        assert!(!config.smart_mode);
        assert!(!config.spell_check);
        assert!(!config.auto_correct);
    }

    #[test]
    fn test_config_with_input_method() {
        let config = EngineConfig::new().with_input_method(InputMethodId::Vni);
        assert_eq!(config.input_method, InputMethodId::Vni);
    }

    #[test]
    fn test_config_with_tone_strategy() {
        let config = EngineConfig::new().with_tone_strategy(ToneStrategy::Traditional);
        assert_eq!(config.tone_strategy, ToneStrategy::Traditional);
    }

    #[test]
    fn test_config_with_enabled() {
        let config = EngineConfig::new().with_enabled(false);
        assert!(!config.enabled);
    }

    #[test]
    fn test_config_with_smart_mode() {
        let config = EngineConfig::new().with_smart_mode(false);
        assert!(!config.smart_mode);
    }

    #[test]
    fn test_config_builder_chain() {
        let config = EngineConfig::new()
            .with_input_method(InputMethodId::Vni)
            .with_tone_strategy(ToneStrategy::Traditional)
            .with_enabled(true)
            .with_smart_mode(false);

        assert_eq!(config.input_method, InputMethodId::Vni);
        assert_eq!(config.tone_strategy, ToneStrategy::Traditional);
        assert!(config.enabled);
        assert!(!config.smart_mode);
    }

    #[test]
    fn test_config_validate_success() {
        let config = EngineConfig::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validate_zero_history() {
        let mut config = EngineConfig::new();
        config.max_history_size = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validate_excessive_timeout() {
        let mut config = EngineConfig::new();
        config.buffer_timeout_ms = 20000; // 20 seconds - too long
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_clone() {
        let config1 = EngineConfig::new();
        let config2 = config1.clone();
        assert_eq!(config1, config2);
    }
}
