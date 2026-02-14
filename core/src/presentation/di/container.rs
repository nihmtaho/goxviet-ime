//! IoC Container - Dependency Injection
//!
//! Wires up all dependencies following the Dependency Inversion Principle.
//! Inner layers define interfaces (ports), outer layers provide implementations.

use crate::application::dto::EngineConfig;
use crate::application::services::{ConfigService, ProcessorService};
use crate::application::use_cases::manage_shortcuts::ManageShortcutsUseCase;
use crate::domain::ports::input::{InputMethod, InputMethodId};
use crate::domain::ports::state::{BufferManager, HistoryTracker};
use crate::domain::ports::transformation::{MarkTransformer, ToneTransformer};
use crate::domain::ports::validation::{LanguageDetector, SyllableValidator};
use crate::infrastructure::adapters::input::{TelexAdapter, VniAdapter};
use crate::infrastructure::adapters::state::{MemoryBufferAdapter, SimpleHistoryAdapter};
use crate::infrastructure::adapters::transformation::{
    VietnameseMarkAdapter, VietnameseToneAdapter,
};
use crate::infrastructure::adapters::validation::{
    FsmValidatorAdapter, LanguageDetectorAdapter,
};
use std::sync::{Arc, Mutex};

/// IoC Container holding all dependencies
///
/// Follows the Dependency Inversion Principle:
/// - High-level modules depend on abstractions (traits)
/// - Low-level modules implement abstractions
/// - Container wires everything together
pub struct Container {
    config: Arc<Mutex<EngineConfig>>,
    config_service: Arc<ConfigService>,
    processor_service: Arc<Mutex<ProcessorService>>,
    shortcut_manager: Arc<Mutex<ManageShortcutsUseCase>>,
}

impl Container {
    /// Create new container with default configuration
    pub fn new() -> Self {
        Self::with_config(EngineConfig::default())
    }

    /// Create container with custom configuration
    pub fn with_config(config: EngineConfig) -> Self {
        let config = Arc::new(Mutex::new(config));
        let config_service = Arc::new(ConfigService::new());

        // Create processor service with all dependencies
        let processor_service = Self::create_processor_service(config.clone());
        
        // Create shortcut manager
        let shortcut_manager = Arc::new(Mutex::new(ManageShortcutsUseCase::new()));

        Self {
            config,
            config_service,
            processor_service: Arc::new(Mutex::new(processor_service)),
            shortcut_manager,
        }
    }

    /// Create ProcessorService with all wired dependencies
    fn create_processor_service(config: Arc<Mutex<EngineConfig>>) -> ProcessorService {
        // Get config snapshot for construction
        let config_snapshot = config.lock().unwrap().clone();
        let method_id = config_snapshot.input_method;

        // Create input method box based on config
        let input_method_box: Box<dyn InputMethod> = match method_id {
            InputMethodId::Telex => Box::new(TelexAdapter::new()),
            InputMethodId::Vni => Box::new(VniAdapter::new()),
            InputMethodId::Plain => Box::new(TelexAdapter::new()), // Fallback
        };

        // ProcessorService takes domain ports directly (not use cases)
        ProcessorService::new(
            input_method_box,
            Box::new(FsmValidatorAdapter::new()),
            Box::new(VietnameseToneAdapter::new(
                crate::domain::ports::transformation::ToneStrategy::default(),
            )),
            Box::new(VietnameseMarkAdapter::new()),
            Box::new(MemoryBufferAdapter::new()),
            Box::new(LanguageDetectorAdapter::new()),
            &config_snapshot,
        )
    }

    /// Create input method based on configuration
    fn create_input_method(config: &Arc<Mutex<EngineConfig>>) -> Arc<dyn InputMethod> {
        let method_id = config.lock().unwrap().input_method;
        match method_id {
            InputMethodId::Telex => Arc::new(TelexAdapter::new()),
            InputMethodId::Vni => Arc::new(VniAdapter::new()),
            InputMethodId::Plain => Arc::new(TelexAdapter::new()), // Fallback to Telex
        }
    }

    /// Create syllable validator (FSM-based)
    fn create_syllable_validator() -> Arc<dyn SyllableValidator> {
        Arc::new(FsmValidatorAdapter::new())
    }

    /// Create language detector
    fn create_language_detector() -> Arc<dyn LanguageDetector> {
        Arc::new(LanguageDetectorAdapter::new())
    }

    /// Create tone transformer
    fn create_tone_transformer() -> Arc<dyn ToneTransformer> {
        Arc::new(VietnameseToneAdapter::new(
            crate::domain::ports::transformation::ToneStrategy::default(),
        ))
    }

    /// Create mark transformer
    fn create_mark_transformer() -> Arc<dyn MarkTransformer> {
        Arc::new(VietnameseMarkAdapter::new())
    }

    /// Create buffer manager (in-memory)
    fn create_buffer_manager() -> Arc<dyn BufferManager> {
        Arc::new(MemoryBufferAdapter::new())
    }

    /// Create history tracker
    fn create_history_tracker() -> Arc<dyn HistoryTracker> {
        Arc::new(SimpleHistoryAdapter::new(100)) // 100 capacity default
    }

    /// Get config service
    pub fn config_service(&self) -> Arc<ConfigService> {
        self.config_service.clone()
    }

    /// Get processor service
    pub fn processor_service(&self) -> Arc<Mutex<ProcessorService>> {
        self.processor_service.clone()
    }
    
    /// Get shortcut manager
    pub fn shortcut_manager(&self) -> Arc<Mutex<ManageShortcutsUseCase>> {
        self.shortcut_manager.clone()
    }

    /// Update configuration (recreates processor service)
    pub fn update_config(&mut self, new_config: EngineConfig) {
        *self.config.lock().unwrap() = new_config;
        self.processor_service = Arc::new(Mutex::new(Self::create_processor_service(
            self.config.clone(),
        )));
    }

    /// Get current configuration
    pub fn get_config(&self) -> EngineConfig {
        self.config.lock().unwrap().clone()
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ports::input::InputMethodId;

    #[test]
    fn test_container_new() {
        let container = Container::new();
        let config = container.get_config();
        assert_eq!(config.input_method, InputMethodId::Telex);
    }

    #[test]
    fn test_container_with_custom_config() {
        let custom_config = EngineConfig {
            input_method: InputMethodId::Vni,
            tone_strategy: crate::domain::ports::transformation::ToneStrategy::default(),
            enabled: true,
            smart_mode: false,
            spell_check: true,
            auto_correct: false,
            max_history_size: 100,
            buffer_timeout_ms: 1000,
            use_modern_tone_placement: false,
            enable_shortcuts: true,
            instant_restore_enabled: true,
            esc_restore_enabled: false,
        };

        let container = Container::with_config(custom_config.clone());
        let config = container.get_config();

        assert_eq!(config.input_method, InputMethodId::Vni);
        assert!(!config.smart_mode);
    }

    #[test]
    fn test_container_update_config() {
        let mut container = Container::new();

        let new_config = EngineConfig {
            input_method: InputMethodId::Vni,
            tone_strategy: crate::domain::ports::transformation::ToneStrategy::default(),
            enabled: true,
            smart_mode: false,
            spell_check: true,
            auto_correct: false,
            max_history_size: 100,
            buffer_timeout_ms: 1000,
            use_modern_tone_placement: true,
            enable_shortcuts: true,
            instant_restore_enabled: true,
            esc_restore_enabled: false,
        };

        container.update_config(new_config.clone());
        let config = container.get_config();

        assert_eq!(config.input_method, InputMethodId::Vni);
    }

    #[test]
    fn test_container_provides_config_service() {
        let container = Container::new();
        let _config_service = container.config_service();
        let config = container.get_config();
        assert_eq!(config.input_method, InputMethodId::Telex);
    }

    #[test]
    fn test_container_provides_processor_service() {
        let container = Container::new();
        let processor_service = container.processor_service();
        assert!(processor_service.lock().is_ok());
    }

    #[test]
    fn test_container_default() {
        let container = Container::default();
        let config = container.get_config();
        assert_eq!(config.input_method, InputMethodId::Telex);
    }

    #[test]
    fn test_container_integration() {
        // Test full integration: config -> services -> use cases
        let container = Container::new();
        let processor = container.processor_service();
        let locked = processor.lock().unwrap();

        // Verify processor service is properly wired
        // (ProcessorService has internal use cases that should be accessible)
        drop(locked); // Release lock
        assert!(processor.lock().is_ok());
    }
}
