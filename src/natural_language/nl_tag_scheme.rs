//! Constants for the tag schemes specified when initializing a linguistic tagger.

use super::NLTagScheme;

extern "C" {
    /* Schemes
     */

    /// A scheme that classifies tokens according to their broad type: word, punctuation, or whitespace.
    #[link_name = "NLTagSchemeTokenType"]
    pub static TokenType: NLTagScheme;

    /// A scheme that classifies tokens according to class: part of speech, type of punctuation, or whitespace.
    #[link_name = "NLTagSchemeLexicalClass"]
    pub static LexicalClass: NLTagScheme;

    /// A scheme that classifies tokens according to whether they are part of a named entity.
    #[link_name = "NLTagSchemeNameType"]
    pub static NameType: NLTagScheme;

    /// A scheme that classifies tokens corresponding to names according to NLTagSchemeNameType, and classifies all other tokens according to NLTagSchemeLexicalClass.
    #[link_name = "NLTagSchemeNameTypeOrLexicalClass"]
    pub static NameTypeOrLexicalClass: NLTagScheme;

    /// A scheme that supplies a stem form of a word token, if known.
    #[link_name = "NLTagSchemeLemma"]
    pub static Lemma: NLTagScheme;

    /// A scheme that supplies the language for a token, if it can determine one.
    #[link_name = "NLTagSchemeLanguage"]
    pub static Language: NLTagScheme;

    /// A scheme that supplies the script for a token, if it can determine one.
    #[link_name = "NLTagSchemeScript"]
    pub static Script: NLTagScheme;

    /// A scheme that scores text as positive, negative, or neutral based on its sentiment polarity.
    #[link_name = "NLTagSchemeSentimentScore"]
    pub static SentimentScore: NLTagScheme;
}
