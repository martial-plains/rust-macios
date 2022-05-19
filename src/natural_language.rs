//! Analyze natural language text and deduce its language-specific metadata.

mod enums;

pub use enums::*;

use crate::foundation;

/// The languages that the Natural Language framework supports.
pub type NLLanguage<'a> = *mut foundation::String<'a>;

/// Constants for the tag schemes specified when initializing a linguistic tagger.
pub type NLTagScheme<'a> = *mut foundation::String<'a>;

pub mod nl_language;
pub mod nl_tag_scheme;
