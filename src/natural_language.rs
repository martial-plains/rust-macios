//! Analyze natural language text and deduce its language-specific metadata.

use crate::foundation;

/// The languages that the Natural Language framework supports.
pub type NLLanguage = foundation::NSString;

/* Tokenization
 */

mod nl_tokenizer;
pub use nl_tokenizer::*;

/* Language identification
 */

mod nl_language_recognizer;
pub use nl_language_recognizer::*;

mod nl_language;
pub use nl_language::*;

/* Linguistic tags
 */

mod nl_tagger;
pub use nl_tagger::*;

mod enums;
pub use enums::*;
