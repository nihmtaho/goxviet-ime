//! Vietnamese language processing modules — re-exported from adapters/transformation/
//!
//! Original implementation moved to `crate::infrastructure::adapters::transformation`.

pub use crate::infrastructure::adapters::transformation::syllable;
pub use crate::infrastructure::adapters::transformation::tone_positioning;
pub use crate::infrastructure::adapters::transformation::transform;
pub use crate::infrastructure::adapters::transformation::validation;
pub use crate::infrastructure::adapters::transformation::vowel_compound;

pub use crate::infrastructure::adapters::transformation::{
    ModifierType, Syllable, TransformResult,
};
