//! Processing Context DTO
//!
//! Data Transfer Object for passing context information during keystroke processing.
//!
//! # Design Principles
//!
//! - **Stateless**: Represents context at a specific moment
//! - **Immutable**: Once created, context doesn't change
//! - **Complete**: Contains all information needed for processing
//!
//! # Usage
//!
//! Context is passed through the processing pipeline:
//! 1. Presentation creates context from user input
//! 2. Application uses context to orchestrate processing
//! 3. Infrastructure adapters use context for decisions

use crate::domain::{
    entities::key_event::KeyEvent, ports::input::InputMethodId,
    value_objects::char_sequence::CharSequence,
};

/// Processing context for keystroke handling
///
/// Contains all contextual information needed to process a single keystroke.
///
/// # Examples
///
/// ```
/// # use goxviet_core::application::dto::ProcessingContext;
/// # use goxviet_core::domain::{entities::key_event::KeyEvent, ports::input::InputMethodId};
/// let context = ProcessingContext::new(
///     KeyEvent::char('a'),
///     InputMethodId::Telex
/// );
/// assert_eq!(context.input_method(), InputMethodId::Telex);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ProcessingContext {
    /// The key event being processed
    key_event: KeyEvent,

    /// Current input method
    input_method: InputMethodId,

    /// Current buffer content (before this keystroke)
    buffer_content: CharSequence,

    /// Application bundle ID or window title (for Smart Mode)
    app_context: Option<String>,

    /// Whether engine is currently enabled
    enabled: bool,

    /// Whether smart mode is active
    smart_mode: bool,

    /// Custom metadata (extensibility)
    metadata: Option<String>,
}

impl ProcessingContext {
    /// Creates a new processing context
    ///
    /// # Arguments
    ///
    /// - `key_event`: The key event to process
    /// - `input_method`: Current input method
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::application::dto::ProcessingContext;
    /// # use goxviet_core::domain::{entities::key_event::KeyEvent, ports::input::InputMethodId};
    /// let context = ProcessingContext::new(
    ///     KeyEvent::char('a'),
    ///     InputMethodId::Telex
    /// );
    /// ```
    pub fn new(key_event: KeyEvent, input_method: InputMethodId) -> Self {
        Self {
            key_event,
            input_method,
            buffer_content: CharSequence::empty(),
            app_context: None,
            enabled: true,
            smart_mode: false,
            metadata: None,
        }
    }

    /// Gets the key event
    pub fn key_event(&self) -> &KeyEvent {
        &self.key_event
    }

    /// Gets the input method
    pub fn input_method(&self) -> InputMethodId {
        self.input_method
    }

    /// Gets buffer content
    pub fn buffer_content(&self) -> &CharSequence {
        &self.buffer_content
    }

    /// Gets application context
    pub fn app_context(&self) -> Option<&str> {
        self.app_context.as_deref()
    }

    /// Checks if engine is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Checks if smart mode is active
    pub fn is_smart_mode(&self) -> bool {
        self.smart_mode
    }

    /// Gets metadata
    pub fn metadata(&self) -> Option<&str> {
        self.metadata.as_deref()
    }

    /// Builder: Set buffer content
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::application::dto::ProcessingContext;
    /// # use goxviet_core::domain::{
    /// #     entities::key_event::KeyEvent,
    /// #     ports::input::InputMethodId,
    /// #     value_objects::char_sequence::CharSequence,
    /// # };
    /// let context = ProcessingContext::new(KeyEvent::char('s'), InputMethodId::Telex)
    ///     .with_buffer(CharSequence::from("hoa"));
    /// assert_eq!(context.buffer_content().as_str(), "hoa");
    /// ```
    pub fn with_buffer(mut self, content: CharSequence) -> Self {
        self.buffer_content = content;
        self
    }

    /// Builder: Set application context
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::application::dto::ProcessingContext;
    /// # use goxviet_core::domain::{entities::key_event::KeyEvent, ports::input::InputMethodId};
    /// let context = ProcessingContext::new(KeyEvent::char('a'), InputMethodId::Telex)
    ///     .with_app_context("com.apple.Terminal");
    /// assert_eq!(context.app_context(), Some("com.apple.Terminal"));
    /// ```
    pub fn with_app_context(mut self, app: impl Into<String>) -> Self {
        self.app_context = Some(app.into());
        self
    }

    /// Builder: Set enabled state
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::application::dto::ProcessingContext;
    /// # use goxviet_core::domain::{entities::key_event::KeyEvent, ports::input::InputMethodId};
    /// let context = ProcessingContext::new(KeyEvent::char('a'), InputMethodId::Telex)
    ///     .with_enabled(false);
    /// assert!(!context.is_enabled());
    /// ```
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Builder: Set smart mode
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::application::dto::ProcessingContext;
    /// # use goxviet_core::domain::{entities::key_event::KeyEvent, ports::input::InputMethodId};
    /// let context = ProcessingContext::new(KeyEvent::char('a'), InputMethodId::Telex)
    ///     .with_smart_mode(true);
    /// assert!(context.is_smart_mode());
    /// ```
    pub fn with_smart_mode(mut self, enabled: bool) -> Self {
        self.smart_mode = enabled;
        self
    }

    /// Builder: Set metadata
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::application::dto::ProcessingContext;
    /// # use goxviet_core::domain::{entities::key_event::KeyEvent, ports::input::InputMethodId};
    /// let context = ProcessingContext::new(KeyEvent::char('a'), InputMethodId::Telex)
    ///     .with_metadata("debug_session_123");
    /// assert_eq!(context.metadata(), Some("debug_session_123"));
    /// ```
    pub fn with_metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }

    /// Checks if this is a modifier key
    pub fn is_modifier_key(&self) -> bool {
        self.key_event.has_modifiers()
    }

    /// Checks if this is a backspace
    pub fn is_backspace(&self) -> bool {
        self.key_event.is_backspace()
    }

    /// Checks if this is a regular character
    pub fn is_char(&self) -> bool {
        self.key_event.is_letter() || self.key_event.is_digit()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_new() {
        let context = ProcessingContext::new(KeyEvent::simple('a' as u16), InputMethodId::Telex);
        assert_eq!(context.input_method(), InputMethodId::Telex);
        assert!(context.buffer_content().is_empty());
        assert!(context.is_enabled());
        assert!(!context.is_smart_mode());
    }

    #[test]
    fn test_context_with_buffer() {
        let context = ProcessingContext::new(KeyEvent::simple('s' as u16), InputMethodId::Telex)
            .with_buffer(CharSequence::from("hoa"));
        assert_eq!(context.buffer_content().as_str(), "hoa");
    }

    #[test]
    fn test_context_with_app_context() {
        let context = ProcessingContext::new(KeyEvent::simple('a' as u16), InputMethodId::Telex)
            .with_app_context("com.apple.Terminal");
        assert_eq!(context.app_context(), Some("com.apple.Terminal"));
    }

    #[test]
    fn test_context_with_enabled() {
        let context = ProcessingContext::new(KeyEvent::simple('a' as u16), InputMethodId::Telex)
            .with_enabled(false);
        assert!(!context.is_enabled());
    }

    #[test]
    fn test_context_with_smart_mode() {
        let context = ProcessingContext::new(KeyEvent::simple('a' as u16), InputMethodId::Telex)
            .with_smart_mode(true);
        assert!(context.is_smart_mode());
    }

    #[test]
    fn test_context_with_metadata() {
        let context = ProcessingContext::new(KeyEvent::simple('a' as u16), InputMethodId::Telex)
            .with_metadata("test_metadata");
        assert_eq!(context.metadata(), Some("test_metadata"));
    }

    #[test]
    fn test_context_builder_chain() {
        let context = ProcessingContext::new(KeyEvent::simple('a' as u16), InputMethodId::Vni)
            .with_buffer(CharSequence::from("test"))
            .with_app_context("VSCode")
            .with_enabled(true)
            .with_smart_mode(true)
            .with_metadata("session_1");

        assert_eq!(context.input_method(), InputMethodId::Vni);
        assert_eq!(context.buffer_content().as_str(), "test");
        assert_eq!(context.app_context(), Some("VSCode"));
        assert!(context.is_enabled());
        assert!(context.is_smart_mode());
        assert_eq!(context.metadata(), Some("session_1"));
    }

    #[test]
    fn test_context_is_modifier_key() {
        let context = ProcessingContext::new(KeyEvent::simple('a' as u16), InputMethodId::Telex);
        assert!(!context.is_modifier_key()); // Simple key has no modifiers
    }

    #[test]
    fn test_context_is_backspace() {
        let context = ProcessingContext::new(KeyEvent::simple(8), InputMethodId::Telex); // ASCII 8 = backspace
        assert!(context.is_backspace());
    }

    #[test]
    fn test_context_is_char() {
        let context = ProcessingContext::new(KeyEvent::simple('a' as u16), InputMethodId::Telex);
        assert!(context.is_char());
    }

    #[test]
    fn test_context_clone() {
        let context1 = ProcessingContext::new(KeyEvent::simple('a' as u16), InputMethodId::Telex);
        let context2 = context1.clone();
        assert_eq!(context1, context2);
    }

    #[test]
    fn test_context_vietnamese_buffer() {
        let context = ProcessingContext::new(KeyEvent::simple('s' as u16), InputMethodId::Telex)
            .with_buffer(CharSequence::from("trường"));
        assert_eq!(context.buffer_content().as_str(), "trường");
    }
}
