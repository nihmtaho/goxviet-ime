//! Transform Text Use Case
//!
//! Use case for transforming Vietnamese text (tone marks, diacritics).
//!
//! # Responsibilities
//!
//! - Apply tone marks to text
//! - Remove tone marks
//! - Apply diacritical marks (horn, breve, circumflex)
//! - Remove diacritical marks
//!
//! # Design
//!
//! Command pattern implementation - performs transformations and returns results.

use crate::domain::{
    entities::{syllable::Syllable, tone::ToneType},
    ports::transformation::{mark_transformer::MarkType, MarkTransformer, ToneTransformer},
    value_objects::{char_sequence::CharSequence, transformation::TransformResult},
};

/// Transformation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformationType {
    /// Apply tone mark
    ApplyTone,
    /// Remove tone mark
    RemoveTone,
    /// Apply diacritical mark
    ApplyMark,
    /// Remove diacritical mark
    RemoveMark,
}

/// Transformation request
#[derive(Debug, Clone)]
pub struct TransformRequest {
    /// Text to transform
    pub text: CharSequence,
    /// Type of transformation
    pub transformation: TransformationType,
    /// Tone to apply (for ApplyTone)
    pub tone: Option<ToneType>,
    /// Mark type (for ApplyMark)
    pub mark: Option<MarkType>,
    /// Position in text (for positional transformations)
    pub position: Option<usize>,
}

impl TransformRequest {
    /// Create request to apply tone
    pub fn apply_tone(text: impl Into<CharSequence>, tone: ToneType) -> Self {
        Self {
            text: text.into(),
            transformation: TransformationType::ApplyTone,
            tone: Some(tone),
            mark: None,
            position: None,
        }
    }

    /// Create request to remove tone
    pub fn remove_tone(text: impl Into<CharSequence>) -> Self {
        Self {
            text: text.into(),
            transformation: TransformationType::RemoveTone,
            tone: None,
            mark: None,
            position: None,
        }
    }

    /// Create request to apply mark
    pub fn apply_mark(
        text: impl Into<CharSequence>,
        mark: MarkType,
        position: usize,
    ) -> Self {
        Self {
            text: text.into(),
            transformation: TransformationType::ApplyMark,
            tone: None,
            mark: Some(mark),
            position: Some(position),
        }
    }

    /// Create request to remove mark
    pub fn remove_mark(text: impl Into<CharSequence>, position: usize) -> Self {
        Self {
            text: text.into(),
            transformation: TransformationType::RemoveMark,
            tone: None,
            mark: None,
            position: Some(position),
        }
    }
}

/// Transform text use case
///
/// Applies various transformations to Vietnamese text.
///
/// # Examples
///
/// ```ignore
/// let use_case = TransformTextUseCase::new(tone_transformer, mark_transformer);
/// let request = TransformRequest::apply_tone("hoa", ToneType::Sắc);
/// let result = use_case.execute(&request);
/// assert_eq!(result.text().as_str(), "hoá");
/// ```
pub struct TransformTextUseCase {
    tone_transformer: Box<dyn ToneTransformer>,
    mark_transformer: Box<dyn MarkTransformer>,
}

impl TransformTextUseCase {
    /// Creates a new use case with injected transformers
    pub fn new(
        tone_transformer: Box<dyn ToneTransformer>,
        mark_transformer: Box<dyn MarkTransformer>,
    ) -> Self {
        Self {
            tone_transformer,
            mark_transformer,
        }
    }

    /// Executes the transformation
    ///
    /// # Arguments
    ///
    /// * `request` - Transformation request
    ///
    /// # Returns
    ///
    /// `TransformResult` with transformed text
    pub fn execute(&self, request: &TransformRequest) -> TransformResult {
        match request.transformation {
            TransformationType::ApplyTone => self.apply_tone(request),
            TransformationType::RemoveTone => self.remove_tone(request),
            TransformationType::ApplyMark => self.apply_mark(request),
            TransformationType::RemoveMark => self.remove_mark(request),
        }
    }

    fn apply_tone(&self, request: &TransformRequest) -> TransformResult {
        let tone = request.tone.unwrap_or(ToneType::Ngang);
        let syllable = Syllable::new().with_vowel(request.text.clone());
        self.tone_transformer.apply_tone(&syllable, tone)
    }

    fn remove_tone(&self, request: &TransformRequest) -> TransformResult {
        let syllable = Syllable::new().with_vowel(request.text.clone());
        self.tone_transformer.remove_tone(&syllable)
    }

    fn apply_mark(&self, request: &TransformRequest) -> TransformResult {
        let mark = request.mark.unwrap_or(MarkType::Horn);
        let position = request.position.unwrap_or(0);
        self.mark_transformer
            .apply_mark(&request.text, mark, position)
    }

    fn remove_mark(&self, request: &TransformRequest) -> TransformResult {
        let position = request.position.unwrap_or(0);
        self.mark_transformer
            .remove_mark(&request.text, position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::key_event::Action;
    use crate::domain::ports::transformation::ToneStrategy;

    struct MockToneTransformer;

    impl ToneTransformer for MockToneTransformer {
        fn apply_tone(&self, _syllable: &Syllable, _tone: ToneType) -> TransformResult {
            TransformResult::new(Action::Insert, CharSequence::from("hoá"))
        }

        fn remove_tone(&self, _syllable: &Syllable) -> TransformResult {
            TransformResult::new(Action::Insert, CharSequence::from("hoa"))
        }

        fn strategy(&self) -> ToneStrategy {
            ToneStrategy::Modern
        }
    }

    struct MockMarkTransformer;

    impl MarkTransformer for MockMarkTransformer {
        fn apply_mark(&self, _text: &CharSequence, _mark: MarkType, _position: usize) -> TransformResult {
            TransformResult::new(Action::Insert, CharSequence::from("ơ"))
        }

        fn remove_mark(&self, _text: &CharSequence, _position: usize) -> TransformResult {
            TransformResult::new(Action::Insert, CharSequence::from("o"))
        }
    }

    fn create_test_use_case() -> TransformTextUseCase {
        TransformTextUseCase::new(
            Box::new(MockToneTransformer),
            Box::new(MockMarkTransformer),
        )
    }

    #[test]
    fn test_use_case_creation() {
        let use_case = create_test_use_case();
        let request = TransformRequest::apply_tone("hoa", ToneType::Sac);
        let result = use_case.execute(&request);
        assert_eq!(result.new_text().as_str(), "hoá");
    }

    #[test]
    fn test_apply_tone() {
        let use_case = create_test_use_case();
        let request = TransformRequest::apply_tone("hoa", ToneType::Sac);
        let result = use_case.execute(&request);
        assert_eq!(result.new_text().as_str(), "hoá");
        assert_eq!(result.action(), Action::Insert);
    }

    #[test]
    fn test_remove_tone() {
        let use_case = create_test_use_case();
        let request = TransformRequest::remove_tone("hoá");
        let result = use_case.execute(&request);
        assert_eq!(result.new_text().as_str(), "hoa");
    }

    #[test]
    fn test_apply_mark() {
        let use_case = create_test_use_case();
        let request = TransformRequest::apply_mark("o", MarkType::Horn, 0);
        let result = use_case.execute(&request);
        assert_eq!(result.new_text().as_str(), "ơ");
    }

    #[test]
    fn test_remove_mark() {
        let use_case = create_test_use_case();
        let request = TransformRequest::remove_mark("ơ", 0);
        let result = use_case.execute(&request);
        assert_eq!(result.new_text().as_str(), "o");
    }

    #[test]
    fn test_request_apply_tone_builder() {
        let request = TransformRequest::apply_tone("hoa", ToneType::Sac);
        assert_eq!(request.transformation, TransformationType::ApplyTone);
        assert_eq!(request.tone, Some(ToneType::Sac));
    }

    #[test]
    fn test_request_remove_tone_builder() {
        let request = TransformRequest::remove_tone("hoá");
        assert_eq!(request.transformation, TransformationType::RemoveTone);
        assert_eq!(request.tone, None);
    }

    #[test]
    fn test_request_apply_mark_builder() {
        let request = TransformRequest::apply_mark("o", MarkType::Horn, 0);
        assert_eq!(request.transformation, TransformationType::ApplyMark);
        assert_eq!(request.mark, Some(MarkType::Horn));
        assert_eq!(request.position, Some(0));
    }

    #[test]
    fn test_request_remove_mark_builder() {
        let request = TransformRequest::remove_mark("ơ", 0);
        assert_eq!(request.transformation, TransformationType::RemoveMark);
        assert_eq!(request.position, Some(0));
    }

    #[test]
    fn test_multiple_tones() {
        let use_case = create_test_use_case();
        for tone in [ToneType::Sac, ToneType::Huyen, ToneType::Hoi] {
            let request = TransformRequest::apply_tone("hoa", tone);
            let result = use_case.execute(&request);
            assert_eq!(result.new_text().as_str(), "hoá"); // Mock always returns same
        }
    }
}
