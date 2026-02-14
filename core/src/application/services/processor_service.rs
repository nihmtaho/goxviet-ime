//! Processor Service
//!
//! High-level orchestration service for keystroke processing.
//!
//! # Responsibilities
//!
//! - Orchestrate the processing pipeline
//! - Coordinate between domain ports
//! - Apply business rules from config
//! - Return structured results
//!
//! # Design
//!
//! This service depends on domain ports (traits), not concrete implementations.
//! Infrastructure layer will inject the actual adapters.

use crate::application::dto::{EngineConfig, ProcessingContext};
use crate::domain::{
    entities::key_event::Action,
    ports::{
        input::InputMethod,
        state::BufferManager,
        transformation::{MarkTransformer, ToneTransformer},
        validation::{LanguageDetector, SyllableValidator},
    },
    value_objects::{
        char_sequence::CharSequence, transformation::TransformResult,
        validation_result::ValidationResult,
    },
};

/// Processing result returned to presentation layer
#[derive(Debug, Clone, PartialEq)]
pub struct ProcessingOutput {
    /// The action to take
    pub action: Action,

    /// The text to output (for Insert/Replace actions)
    pub text: Option<CharSequence>,

    /// Whether the buffer was reset
    pub buffer_reset: bool,

    /// Whether transformation occurred
    pub transformed: bool,
}

impl ProcessingOutput {
    /// Create output for inserting text
    pub fn insert(text: impl Into<CharSequence>) -> Self {
        Self {
            action: Action::Insert,
            text: Some(text.into()),
            buffer_reset: false,
            transformed: false,
        }
    }

    /// Create output for replacing text
    pub fn replace(backspace_count: u8, text: impl Into<CharSequence>) -> Self {
        Self {
            action: Action::Replace { backspace_count },
            text: Some(text.into()),
            buffer_reset: false,
            transformed: true,
        }
    }

    /// Create output for clearing
    pub fn clear() -> Self {
        Self {
            action: Action::Clear,
            text: None,
            buffer_reset: true,
            transformed: false,
        }
    }

    /// Create output for no-op
    pub fn noop() -> Self {
        Self {
            action: Action::None,
            text: None,
            buffer_reset: false,
            transformed: false,
        }
    }

    /// Mark that buffer was reset
    pub fn with_buffer_reset(mut self) -> Self {
        self.buffer_reset = true;
        self
    }
}

/// Processor service errors
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessorError {
    /// Engine is disabled
    EngineDisabled,
    /// Invalid context
    InvalidContext(String),
    /// Processing failed
    ProcessingFailed(String),
}

impl std::fmt::Display for ProcessorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EngineDisabled => write!(f, "Engine is disabled"),
            Self::InvalidContext(msg) => write!(f, "Invalid context: {}", msg),
            Self::ProcessingFailed(msg) => write!(f, "Processing failed: {}", msg),
        }
    }
}

impl std::error::Error for ProcessorError {}

/// Processor service
///
/// Orchestrates keystroke processing using domain ports.
///
/// # Examples
///
/// ```ignore
/// // This example requires concrete implementations of traits
/// let service = ProcessorService::new(
///     Box::new(telex_adapter),
///     Box::new(validator),
///     Box::new(tone_transformer),
///     Box::new(mark_transformer),
///     Box::new(buffer_manager),
///     Box::new(detector),
/// );
///
/// let config = EngineConfig::default();
/// let context = ProcessingContext::new(KeyEvent::simple('a' as u16), InputMethodId::Telex);
/// let result = service.process(&config, &context)?;
/// ```
#[allow(dead_code)] // Fields wired for future use when legacy engine is fully replaced
pub struct ProcessorService {
    input_method: Box<dyn InputMethod>,
    validator: Box<dyn SyllableValidator>,
    tone_transformer: Box<dyn ToneTransformer>,
    mark_transformer: Box<dyn MarkTransformer>,
    buffer_manager: Box<dyn BufferManager>,
    language_detector: Box<dyn LanguageDetector>,
    // Temporary: Use legacy engine as adapter until full v2 implementation
    engine: crate::infrastructure::engine::Engine,
}

impl ProcessorService {
    /// Creates a new processor service with injected dependencies
    ///
    /// # Arguments
    ///
    /// All arguments are trait objects (ports), allowing flexibility in implementation.
    pub fn new(
        input_method: Box<dyn InputMethod>,
        validator: Box<dyn SyllableValidator>,
        tone_transformer: Box<dyn ToneTransformer>,
        mark_transformer: Box<dyn MarkTransformer>,
        buffer_manager: Box<dyn BufferManager>,
        language_detector: Box<dyn LanguageDetector>,
        config: &crate::application::dto::EngineConfig,
    ) -> Self {
        // Create legacy engine based on input method
        let mut engine = crate::infrastructure::engine::Engine::new();
        
        // Configure legacy engine from EngineConfig
        engine.set_modern_tone(config.use_modern_tone_placement);
        engine.set_english_auto_restore(config.instant_restore_enabled);
        engine.set_esc_restore(config.esc_restore_enabled);
        
        // Set method based on input_method trait
        match input_method.method_id() {
            crate::domain::ports::input::InputMethodId::Telex => {
                engine.set_method(0);
            }
            crate::domain::ports::input::InputMethodId::Vni => {
                engine.set_method(1);
            }
            _ => {
                engine.set_method(0); // Default to Telex
            }
        }
        
        Self {
            input_method,
            validator,
            tone_transformer,
            mark_transformer,
            buffer_manager,
            language_detector,
            engine,
        }
    }

    /// Process a keystroke with given configuration and context
    ///
    /// This is the main entry point for keystroke processing.
    pub fn process(
        &self,
        config: &EngineConfig,
        context: &ProcessingContext,
    ) -> Result<ProcessingOutput, ProcessorError> {
        // Check if engine is enabled
        if !config.is_enabled() {
            return Ok(ProcessingOutput::noop());
        }

        // Check if this key should be processed
        if context.is_backspace() {
            return self.process_backspace(context);
        }

        if !context.is_char() {
            return Ok(ProcessingOutput::noop());
        }

        // Process regular character
        self.process_char(config, context)
    }

    /// Process a backspace key
    fn process_backspace(&self, context: &ProcessingContext) -> Result<ProcessingOutput, ProcessorError> {
        // If buffer is empty, just clear
        if context.buffer_content().is_empty() {
            return Ok(ProcessingOutput::noop());
        }

        // Check if we can undo transformation
        // For now, simple clear last char (replace with empty)
        Ok(ProcessingOutput::replace(1, CharSequence::empty()))
    }

    /// Process a regular character
    fn process_char(
        &self,
        _config: &EngineConfig,
        context: &ProcessingContext,
    ) -> Result<ProcessingOutput, ProcessorError> {
        let key_event = context.key_event();
        
        // Get character from key event
        let ch = match key_event.as_char() {
            Some(c) => c,
            None => return Ok(ProcessingOutput::noop()),
        };

        // Simple pass-through for now
        // In real implementation:
        // 1. Check if it's a Vietnamese transformation key (s, f, w, etc.)
        // 2. Apply transformation using input_method
        // 3. Validate result
        // 4. Update buffer
        // 5. Return appropriate action

        Ok(ProcessingOutput::insert(CharSequence::from(ch.to_string())))
    }

    /// Validate current buffer content
    pub fn validate_buffer(&self, content: &CharSequence) -> ValidationResult {
        // Use validator port
        if content.is_empty() {
            return ValidationResult::valid();
        }

        // Basic validation - real implementation would use self.validator
        ValidationResult::valid()
    }

    /// Detect language of buffer content
    pub fn detect_language(&self, content: &CharSequence) -> bool {
        // Use language_detector port
        // Returns true if Vietnamese, false if English
        if content.is_empty() {
            return false;
        }

        // Basic detection - real implementation would use self.language_detector
        true
    }

    /// Process keystroke (simple wrapper for FFI)
    ///
    /// # Arguments
    /// - `key_event`: The key event to process
    ///
    /// # Returns
    /// Transform result for the keystroke
     pub fn process_key(&mut self, key_event: crate::domain::entities::key_event::KeyEvent) -> Result<TransformResult, ProcessorError> {
        // Use legacy engine with correct parameter mapping:
        // Legacy engine: on_key_ext(key, caps, ctrl, shift)
        let result = self.engine.on_key_ext(
            key_event.keycode,
            key_event.caps,
            key_event.ctrl,
            key_event.shift,
        );
        
        // Build output text from legacy result
        let output_text = if result.count > 0 {
            let mut text = String::new();
            for i in 0..result.count as usize {
                unsafe {
                    if let Some(ch) = char::from_u32(*result.chars.offset(i as isize)) {
                        text.push(ch);
                    }
                }
            }
            CharSequence::from(text)
        } else {
            CharSequence::empty()
        };
        
        // Determine action based on legacy result
        let action = if result.backspace > 0 {
            // Has backspace: Replace action
            Action::Replace {
                backspace_count: result.backspace,
            }
        } else if result.action == 1 {
            // Insert action
            Action::Insert
        } else {
            // No action
            Action::None
        };
        
        Ok(TransformResult::new(action, output_text))
    }

    /// Restore current buffer to raw ASCII input (undo all Vietnamese transforms)
    /// Used for Double OPTION key restore on macOS
    pub fn restore_to_raw(&mut self) -> TransformResult {
        // Temporarily enable ESC restore, send ESC key, then restore the setting
        let was_enabled = self.engine.esc_restore_enabled;
        self.engine.set_esc_restore(true);
        let result = self.engine.on_key_ext(
            crate::data::keys::ESC,
            false, false, false,
        );
        self.engine.set_esc_restore(was_enabled);

        let output_text = if result.count > 0 {
            let mut text = String::new();
            for i in 0..result.count as usize {
                unsafe {
                    if let Some(ch) = char::from_u32(*result.chars.offset(i as isize)) {
                        text.push(ch);
                    }
                }
            }
            CharSequence::from(text)
        } else {
            CharSequence::empty()
        };

        let action = if result.backspace > 0 {
            Action::Replace { backspace_count: result.backspace }
        } else {
            Action::None
        };

        TransformResult::new(action, output_text)
    }

    /// Add a text expansion shortcut to the engine
    pub fn add_shortcut(&mut self, trigger: &str, expansion: &str) -> bool {
        use crate::features::shortcut::Shortcut;
        self.engine.shortcuts_mut().add(Shortcut::new(trigger, expansion))
    }

    /// Remove a text expansion shortcut from the engine
    pub fn remove_shortcut(&mut self, trigger: &str) -> bool {
        self.engine.shortcuts_mut().remove(trigger).is_some()
    }

    /// Clear all shortcuts from the engine
    pub fn clear_shortcuts(&mut self) {
        self.engine.shortcuts_mut().clear();
    }

    /// Get the number of shortcuts in the engine
    pub fn shortcuts_count(&self) -> usize {
        self.engine.shortcuts().len()
    }

    /// Set whether shortcuts (text expansion) are enabled
    pub fn set_shortcuts_enabled(&mut self, enabled: bool) {
        self.engine.shortcuts_enabled = enabled;
    }

    /// Reset buffer state (clear current word without destroying shortcuts/config)
    pub fn reset_buffer(&mut self) {
        self.engine.clear();
    }

    /// Reset all state including word history (without destroying shortcuts/config)
    pub fn reset_all(&mut self) {
        self.engine.clear_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        entities::{buffer::InputBuffer, key_event::KeyEvent, syllable::Syllable},
        ports::input::InputMethodId,
        value_objects::validation_result::ValidationError,
    };

    // Mock implementations for testing
    struct MockInputMethod;
    impl InputMethod for MockInputMethod {
        fn method_id(&self) -> InputMethodId {
            InputMethodId::Telex
        }
        fn detect_tone(&self, _event: &KeyEvent) -> Option<crate::domain::entities::tone::ToneType> {
            None
        }
        fn detect_diacritic(&self, _event: &KeyEvent) -> Option<crate::domain::ports::input::DiacriticType> {
            None
        }
        fn is_remove_mark(&self, _event: &KeyEvent) -> bool {
            false
        }
    }

    struct MockValidator;
    impl SyllableValidator for MockValidator {
        fn validate(&self, _syllable: &Syllable) -> ValidationResult {
            ValidationResult::valid()
        }
    }

    struct MockToneTransformer;
    impl ToneTransformer for MockToneTransformer {
        fn apply_tone(
            &self,
            _syllable: &Syllable,
            _tone: crate::domain::entities::tone::ToneType,
        ) -> TransformResult {
            TransformResult::new(Action::None, CharSequence::empty())
        }
        fn remove_tone(&self, _syllable: &Syllable) -> TransformResult {
            TransformResult::new(Action::None, CharSequence::empty())
        }
        fn strategy(&self) -> crate::domain::ports::transformation::ToneStrategy {
            crate::domain::ports::transformation::ToneStrategy::Modern
        }
    }

    struct MockMarkTransformer;
    impl MarkTransformer for MockMarkTransformer {
        fn apply_mark(
            &self,
            _text: &CharSequence,
            _mark: crate::domain::ports::transformation::MarkType,
            _position: usize,
        ) -> TransformResult {
            TransformResult::new(Action::None, CharSequence::empty())
        }
        fn remove_mark(&self, _text: &CharSequence, _position: usize) -> TransformResult {
            TransformResult::new(Action::None, CharSequence::empty())
        }
    }

    struct MockBufferManager {
        buffer: InputBuffer,
    }
    
    impl MockBufferManager {
        fn new() -> Self {
            Self {
                buffer: InputBuffer::new(),
            }
        }
    }
    
    impl BufferManager for MockBufferManager {
        fn current(&self) -> &InputBuffer {
            &self.buffer
        }
        fn current_mut(&mut self) -> &mut InputBuffer {
            &mut self.buffer
        }
        fn append(&mut self, _text: &str) -> bool {
            true
        }
        fn delete(&mut self, _count: usize) -> usize {
            0
        }
        fn replace(&mut self, _new_content: &str) {}
        fn clear(&mut self) {}
    }

    struct MockLanguageDetector;
    impl LanguageDetector for MockLanguageDetector {
        fn detect(&self, _text: &CharSequence) -> crate::domain::ports::validation::DetectionResult {
            use crate::domain::ports::validation::{DetectedLanguage, ConfidenceLevel};
            crate::domain::ports::validation::DetectionResult::vietnamese(ConfidenceLevel::High)
        }
        fn is_vietnamese(&self, _text: &CharSequence) -> bool {
            true
        }
    }

    fn create_test_service() -> ProcessorService {
        let config = EngineConfig::default();
        ProcessorService::new(
            Box::new(MockInputMethod),
            Box::new(MockValidator),
            Box::new(MockToneTransformer),
            Box::new(MockMarkTransformer),
            Box::new(MockBufferManager::new()),
            Box::new(MockLanguageDetector),
            &config,
        )
    }

    #[test]
    fn test_service_creation() {
        let service = create_test_service();
        let config = EngineConfig::default();
        let context = ProcessingContext::new(KeyEvent::simple('a' as u16), InputMethodId::Telex);
        let result = service.process(&config, &context);
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_with_disabled_engine() {
        let service = create_test_service();
        let config = EngineConfig::disabled();
        let context = ProcessingContext::new(KeyEvent::simple('a' as u16), InputMethodId::Telex);
        let result = service.process(&config, &context).unwrap();
        assert_eq!(result.action, Action::None);
    }

    #[test]
    fn test_process_regular_char() {
        let service = create_test_service();
        let config = EngineConfig::default();
        let context = ProcessingContext::new(KeyEvent::simple('a' as u16), InputMethodId::Telex);
        let result = service.process(&config, &context).unwrap();
        assert_eq!(result.action, Action::Insert);
        assert!(result.text.is_some());
    }

    #[test]
    fn test_process_backspace_empty_buffer() {
        let service = create_test_service();
        let config = EngineConfig::default();
        let context = ProcessingContext::new(KeyEvent::simple(8), InputMethodId::Telex); // Backspace
        let result = service.process(&config, &context).unwrap();
        assert_eq!(result.action, Action::None); // Empty buffer, noop
    }

    #[test]
    fn test_process_backspace_with_buffer() {
        let service = create_test_service();
        let config = EngineConfig::default();
        let context = ProcessingContext::new(KeyEvent::simple(8), InputMethodId::Telex)
            .with_buffer(CharSequence::from("hoa"));
        let result = service.process(&config, &context).unwrap();
        assert!(matches!(result.action, Action::Replace { .. }));
    }

    #[test]
    fn test_validate_empty_buffer() {
        let service = create_test_service();
        let result = service.validate_buffer(&CharSequence::empty());
        assert!(result.is_valid());
    }

    #[test]
    fn test_validate_non_empty_buffer() {
        let service = create_test_service();
        let result = service.validate_buffer(&CharSequence::from("hoa"));
        assert!(result.is_valid());
    }

    #[test]
    fn test_detect_language_empty() {
        let service = create_test_service();
        let result = service.detect_language(&CharSequence::empty());
        assert!(!result); // Empty is not Vietnamese
    }

    #[test]
    fn test_detect_language_vietnamese() {
        let service = create_test_service();
        let result = service.detect_language(&CharSequence::from("trường"));
        assert!(result);
    }

    #[test]
    fn test_processing_output_insert() {
        let output = ProcessingOutput::insert("a");
        assert_eq!(output.action, Action::Insert);
        assert_eq!(output.text.unwrap().as_str(), "a");
    }

    #[test]
    fn test_processing_output_replace() {
        let output = ProcessingOutput::replace(1, "á");
        assert!(matches!(output.action, Action::Replace { backspace_count: 1 }));
        assert_eq!(output.text.unwrap().as_str(), "á");
        assert!(output.transformed);
    }

    #[test]
    fn test_processing_output_clear() {
        let output = ProcessingOutput::clear();
        assert_eq!(output.action, Action::Clear);
        assert!(output.text.is_none());
        assert!(output.buffer_reset);
    }

    #[test]
    fn test_processing_output_noop() {
        let output = ProcessingOutput::noop();
        assert_eq!(output.action, Action::None);
        assert!(output.text.is_none());
    }

    #[test]
    fn test_processing_output_with_buffer_reset() {
        let output = ProcessingOutput::insert("a").with_buffer_reset();
        assert!(output.buffer_reset);
    }
}
