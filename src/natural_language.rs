//! Analyze natural language text and deduce its language-specific metadata.

mod enums;

pub use enums::*;

use crate::foundation;

/// The languages that the Natural Language framework supports.
pub type NLLanguage = *mut foundation::String;

/// Constants for the tag schemes specified when initializing a linguistic tagger.
pub type NLTagScheme = *mut foundation::String;

mod nl_language;
mod nl_language_identifier;
mod nl_tag_scheme;

pub use nl_language::*;
pub use nl_language_identifier::*;
pub use nl_tag_scheme::*;
