//! Analyze natural language text and deduce its language-specific metadata.

use crate::foundation;

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

/// The languages that the Natural Language framework supports.
pub type NLLanguage = foundation::NSString;

/* Linguistic tags
 */

mod nl_tagger;
pub use nl_tagger::*;

mod nl_gazetteer;
pub use nl_gazetteer::*;

/* Text embedding */
mod nl_embedding;
pub use nl_embedding::*;

/* Natural language models
 */

mod nl_model;
pub use nl_model::*;

mod nl_model_configuration;
pub use nl_model_configuration::*;
