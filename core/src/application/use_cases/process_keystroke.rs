//! Process Keystroke Use Case
//!
//! The main use case for handling keystroke events in the Vietnamese IME.
//!
//! # Responsibilities
//!
//! - Accept keystroke from presentation layer
//! - Orchestrate processing through services
//! - Return action to presentation layer
//!
//! # Design
//!
//! This is a Command pattern implementation - a single public method
//! that encapsulates the entire keystroke processing workflow.

use crate::application::{
    dto::{EngineConfig, ProcessingContext},
    services::{ProcessingOutput, ProcessorService, ProcessorError},
};

/// Process keystroke use case
///
/// This is the primary entry point for keystroke processing in the application layer.
///
/// # Examples
///
/// ```ignore
/// // Requires concrete service implementation
/// let use_case = ProcessKeystrokeUseCase::new(processor_service);
/// let config = EngineConfig::default();
/// let context = ProcessingContext::new(key_event, InputMethodId::Telex);
/// let output = use_case.execute(&config, &context)?;
/// ```
pub struct ProcessKeystrokeUseCase {
    processor: ProcessorService,
}

impl ProcessKeystrokeUseCase {
    /// Creates a new use case with injected processor service
    pub fn new(processor: ProcessorService) -> Self {
        Self { processor }
    }

    /// Executes the keystroke processing
    ///
    /// # Arguments
    ///
    /// * `config` - Current engine configuration
    /// * `context` - Processing context with keystroke and buffer state
    ///
    /// # Returns
    ///
    /// `ProcessingOutput` containing the action to take
    ///
    /// # Errors
    ///
    /// Returns `ProcessorError` if processing fails
    pub fn execute(
        &self,
        config: &EngineConfig,
        context: &ProcessingContext,
    ) -> Result<ProcessingOutput, ProcessorError> {
        // Validate inputs
        if !config.is_enabled() {
            return Ok(ProcessingOutput::noop());
        }

        // Delegate to processor service
        self.processor.process(config, context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        entities::{key_event::KeyEvent, buffer::InputBuffer, syllable::Syllable, key_event::Action},
        ports::{
            input::{InputMethod, InputMethodId},
            state::BufferManager,
            transformation::{MarkTransformer, ToneTransformer, MarkType},
            validation::{LanguageDetector, SyllableValidator, DetectionResult, DetectedLanguage, ConfidenceLevel},
        },
        value_objects::{
            char_sequence::CharSequence,
            transformation::TransformResult,
            validation_result::ValidationResult,
        },
    };

    // Mock implementations
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
            _mark: MarkType,
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
        fn detect(&self, _text: &CharSequence) -> DetectionResult {
            DetectionResult::vietnamese(ConfidenceLevel::High)
        }
        fn is_vietnamese(&self, _text: &CharSequence) -> bool {
            true
        }
    }

    fn create_test_use_case() -> ProcessKeystrokeUseCase {
        let config = crate::application::dto::EngineConfig::default();
        let processor = ProcessorService::new(
            Box::new(MockInputMethod),
            Box::new(MockValidator),
            Box::new(MockToneTransformer),
            Box::new(MockMarkTransformer),
            Box::new(MockBufferManager::new()),
            Box::new(MockLanguageDetector),
            &config,
        );
        ProcessKeystrokeUseCase::new(processor)
    }

    #[test]
    fn test_use_case_creation() {
        let use_case = create_test_use_case();
        let config = EngineConfig::default();
        let context = ProcessingContext::new(KeyEvent::simple('a' as u16), InputMethodId::Telex);
        let result = use_case.execute(&config, &context);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_with_enabled_engine() {
        let use_case = create_test_use_case();
        let config = EngineConfig::default();
        let context = ProcessingContext::new(KeyEvent::simple('a' as u16), InputMethodId::Telex);
        let result = use_case.execute(&config, &context).unwrap();
        assert_eq!(result.action, Action::Insert);
    }

    #[test]
    fn test_execute_with_disabled_engine() {
        let use_case = create_test_use_case();
        let config = EngineConfig::disabled();
        let context = ProcessingContext::new(KeyEvent::simple('a' as u16), InputMethodId::Telex);
        let result = use_case.execute(&config, &context).unwrap();
        assert_eq!(result.action, Action::None);
    }

    #[test]
    fn test_execute_backspace() {
        let use_case = create_test_use_case();
        let config = EngineConfig::default();
        let context = ProcessingContext::new(KeyEvent::simple(8), InputMethodId::Telex);
        let result = use_case.execute(&config, &context).unwrap();
        // Backspace on empty buffer returns noop
        assert_eq!(result.action, Action::None);
    }

    #[test]
    fn test_execute_with_buffer_content() {
        let use_case = create_test_use_case();
        let config = EngineConfig::default();
        let context = ProcessingContext::new(KeyEvent::simple('s' as u16), InputMethodId::Telex)
            .with_buffer(CharSequence::from("hoa"));
        let result = use_case.execute(&config, &context).unwrap();
        assert_eq!(result.action, Action::Insert);
    }

    #[test]
    fn test_execute_vietnamese_text() {
        let use_case = create_test_use_case();
        let config = EngineConfig::default();
        let context = ProcessingContext::new(KeyEvent::simple('n' as u16), InputMethodId::Telex)
            .with_buffer(CharSequence::from("truon"));
        let result = use_case.execute(&config, &context).unwrap();
        assert_eq!(result.action, Action::Insert);
    }
}
