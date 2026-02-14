//! Configuration Service
//!
//! Application service for managing engine configuration.
//!
//! # Responsibilities
//!
//! - Validate configuration changes
//! - Provide configuration queries
//! - Coordinate between DTOs and domain
//!
//! # Design
//!
//! This service is stateless - it operates on EngineConfig DTOs.
//! It doesn't store state, just provides operations.

use crate::application::dto::EngineConfig;
use crate::domain::ports::input::InputMethodId;

/// Configuration validation errors
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigError {
    /// Invalid tone style configuration
    InvalidToneStyle,
    /// Invalid shortcut configuration
    InvalidShortcut(String),
    /// Input method not supported
    UnsupportedInputMethod,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidToneStyle => write!(f, "Invalid tone style configuration"),
            Self::InvalidShortcut(msg) => write!(f, "Invalid shortcut: {}", msg),
            Self::UnsupportedInputMethod => write!(f, "Unsupported input method"),
        }
    }
}

impl std::error::Error for ConfigError {}

/// Configuration service
///
/// Provides operations for managing and validating engine configuration.
///
/// # Examples
///
/// ```
/// # use goxviet_core::application::services::ConfigService;
/// # use goxviet_core::application::dto::EngineConfig;
/// let service = ConfigService::new();
/// let config = EngineConfig::default();
/// assert!(service.validate(&config).is_ok());
/// ```
pub struct ConfigService;

impl ConfigService {
    /// Creates a new configuration service
    pub fn new() -> Self {
        Self
    }

    /// Validates a configuration
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::application::services::ConfigService;
    /// # use goxviet_core::application::dto::EngineConfig;
    /// let service = ConfigService::new();
    /// let config = EngineConfig::default();
    /// assert!(service.validate(&config).is_ok());
    /// ```
    pub fn validate(&self, config: &EngineConfig) -> Result<(), ConfigError> {
        // For now, all configs are valid
        // In future: validate shortcuts syntax, app contexts, etc.
        let _ = config; // Use the parameter
        Ok(())
    }

    /// Checks if auto-restore is enabled for current configuration
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::application::services::ConfigService;
    /// # use goxviet_core::application::dto::EngineConfig;
    /// let service = ConfigService::new();
    /// let config = EngineConfig::default();
    /// assert!(service.is_auto_restore_enabled(&config));
    /// ```
    pub fn is_auto_restore_enabled(&self, config: &EngineConfig) -> bool {
        config.is_auto_correct_enabled()
    }

    /// Checks if smart mode is enabled
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::application::services::ConfigService;
    /// # use goxviet_core::application::dto::EngineConfig;
    /// let service = ConfigService::new();
    /// let config = EngineConfig::default();
    /// assert!(service.is_smart_mode_enabled(&config));
    /// ```
    pub fn is_smart_mode_enabled(&self, config: &EngineConfig) -> bool {
        config.is_smart_mode_enabled()
    }

    /// Checks if engine is enabled
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::application::services::ConfigService;
    /// # use goxviet_core::application::dto::EngineConfig;
    /// let service = ConfigService::new();
    /// let config = EngineConfig::default();
    /// assert!(service.is_engine_enabled(&config));
    /// ```
    pub fn is_engine_enabled(&self, config: &EngineConfig) -> bool {
        config.is_enabled()
    }

    /// Gets the configured input method
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::application::services::ConfigService;
    /// # use goxviet_core::application::dto::EngineConfig;
    /// # use goxviet_core::domain::ports::input::InputMethodId;
    /// let service = ConfigService::new();
    /// let config = EngineConfig::default();
    /// assert_eq!(service.get_input_method(&config), InputMethodId::Telex);
    /// ```
    pub fn get_input_method(&self, config: &EngineConfig) -> InputMethodId {
        config.input_method()
    }

    /// Checks if spell checking is enabled
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::application::services::ConfigService;
    /// # use goxviet_core::application::dto::EngineConfig;
    /// let service = ConfigService::new();
    /// let config = EngineConfig::default();
    /// assert!(service.is_spell_checking_enabled(&config));
    /// ```
    pub fn is_spell_checking_enabled(&self, config: &EngineConfig) -> bool {
        config.is_spell_check_enabled()
    }

    /// Merges two configurations, with new config taking precedence
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::application::services::ConfigService;
    /// # use goxviet_core::application::dto::{EngineConfig, ToneStrategy};
    /// # use goxviet_core::domain::ports::input::InputMethodId;
    /// let service = ConfigService::new();
    /// let base = EngineConfig::default();
    /// let new = EngineConfig::vni().with_tone_strategy(ToneStrategy::Traditional);
    /// let merged = service.merge(&base, &new);
    /// assert_eq!(merged.input_method(), InputMethodId::Vni);
    /// assert_eq!(merged.tone_strategy(), ToneStrategy::Traditional);
    /// ```
    pub fn merge(&self, _base: &EngineConfig, new: &EngineConfig) -> EngineConfig {
        // New config values override base - just return new for now
        // In real implementation, might want selective merge
        new.clone()
    }

    /// Creates a configuration for a specific application context
    ///
    /// This allows per-app customization (Smart Mode feature).
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::application::services::ConfigService;
    /// # use goxviet_core::application::dto::EngineConfig;
    /// let service = ConfigService::new();
    /// let base = EngineConfig::default();
    /// let terminal_config = service.for_app_context(&base, "com.apple.Terminal");
    /// // In real implementation, this would check per-app settings
    /// assert_eq!(terminal_config.input_method(), base.input_method());
    /// ```
    pub fn for_app_context(&self, base: &EngineConfig, app_context: &str) -> EngineConfig {
        // For now, return base config
        // In future: lookup per-app settings and merge
        let _ = app_context;
        base.clone()
    }
}

impl Default for ConfigService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ports::transformation::ToneStrategy;

    #[test]
    fn test_service_new() {
        let service = ConfigService::new();
        let config = EngineConfig::default();
        assert!(service.validate(&config).is_ok());
    }

    #[test]
    fn test_validate_default_config() {
        let service = ConfigService::new();
        let config = EngineConfig::default();
        assert!(service.validate(&config).is_ok());
    }

    #[test]
    fn test_is_auto_restore_enabled() {
        let service = ConfigService::new();
        let config = EngineConfig::default();
        // Default config has auto_correct = false
        let result = service.is_auto_restore_enabled(&config);
        assert!(!result); // Default is false
    }

    #[test]
    fn test_is_smart_mode_enabled() {
        let service = ConfigService::new();
        let config = EngineConfig::default();
        // Default has smart_mode = true
        assert!(service.is_smart_mode_enabled(&config));
    }

    #[test]
    fn test_is_engine_enabled() {
        let service = ConfigService::new();
        let config = EngineConfig::default();
        assert!(service.is_engine_enabled(&config));
    }

    #[test]
    fn test_get_input_method() {
        let service = ConfigService::new();
        let config = EngineConfig::default();
        assert_eq!(service.get_input_method(&config), InputMethodId::Telex);
    }

    #[test]
    fn test_is_spell_checking_enabled() {
        let service = ConfigService::new();
        let config = EngineConfig::default();
        assert!(service.is_spell_checking_enabled(&config));
    }

    #[test]
    fn test_merge_configs() {
        let service = ConfigService::new();
        let base = EngineConfig::default();
        let new = EngineConfig::vni().with_enabled(false);

        let merged = service.merge(&base, &new);

        assert_eq!(merged.input_method(), InputMethodId::Vni);
        assert!(!merged.is_enabled());
    }

    #[test]
    fn test_merge_preserves_new_values() {
        let service = ConfigService::new();
        let base = EngineConfig::telex();
        let new = EngineConfig::vni()
            .with_smart_mode(true)
            .with_tone_strategy(ToneStrategy::Traditional);

        let merged = service.merge(&base, &new);

        assert_eq!(merged.input_method(), InputMethodId::Vni);
        assert_eq!(merged.tone_strategy(), ToneStrategy::Traditional);
        assert!(merged.is_smart_mode_enabled());
    }

    #[test]
    fn test_for_app_context() {
        let service = ConfigService::new();
        let base = EngineConfig::default();
        let app_config = service.for_app_context(&base, "com.apple.Terminal");

        // For now, should return same config
        assert_eq!(app_config.input_method(), base.input_method());
        assert_eq!(app_config.is_enabled(), base.is_enabled());
    }

    #[test]
    fn test_for_app_context_different_apps() {
        let service = ConfigService::new();
        let base = EngineConfig::default();

        let terminal_config = service.for_app_context(&base, "com.apple.Terminal");
        let vscode_config = service.for_app_context(&base, "com.microsoft.VSCode");

        // Currently returns same config for all apps
        assert_eq!(terminal_config.input_method(), vscode_config.input_method());
    }

    #[test]
    fn test_service_default() {
        let service = ConfigService::default();
        let config = EngineConfig::default();
        assert!(service.validate(&config).is_ok());
    }
}
