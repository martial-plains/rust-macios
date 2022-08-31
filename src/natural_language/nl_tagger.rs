use block::IntoConcreteBlock;
use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::{NSArray, NSError, NSOrthography, NSRange, NSString},
    objective_c_runtime::{
        macros::object,
        traits::{FromId, PNSObject},
    },
};

use super::{NLLanguage, NLTokenUnit};

object! {
    /// A tagger that analyzes natural language text.
    unsafe pub struct NLTagger;
}

/// Constants for the tag schemes specified when initializing a linguistic tagger.
pub type NLTagScheme = NSString;

/// A token type, lexical class, name, lemma, language, or script returned by a linguistic tagger for natural language text.
pub type NLTag = NSString;

/// Constants for the tag schemes specified when initializing a linguistic tagger.
pub mod nl_tag_scheme {
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
}

/// A token type, lexical class, name, lemma, language, or script returned by a linguistic tagger for natural language text.
pub mod nl_tag {
    use super::NLTag;

    extern "C" {
        /* Tags
         */

        /* Token types */

        /// A tag indicating that the token is a word.
        #[link_name = "NLTagWord"]
        pub static Word: NLTag;

        /// A tag indicating that the token is punctuation.
        #[link_name = "NLTagPunctuation"]
        pub static Punctuation: NLTag;

        /// A tag indicating that the token is white space of any sort.
        #[link_name = "NLTagWhitespace"]
        pub static Whitespace: NLTag;

        /// A tag indicating that the token is a non-linguistic item, such as a symbol.
        #[link_name = "NLTagOther"]
        pub static Other: NLTag;

        /* Lexical classes */

        /// A tag indicating that the token is a noun.
        #[link_name = "NLTagNoun"]
        pub static Noun: NLTag;

        /// A tag indicating that the token is a verb.
        #[link_name = "NLTagVerb"]
        pub static Verb: NLTag;

        /// A tag indicating that the token is an adjective
        #[link_name = "NLTagAdjective"]
        pub static Adjective: NLTag;

    }
}

/// Constants for linguistic tagger enumeration specifying which tokens to omit and whether to join names.
#[derive(Debug)]
#[repr(u64)]
pub enum NLTaggerOptions {
    /// Omit tokens of type word (items considered to be words).
    OmitWords = 1 << 0,
    /// Omit tokens of type punctuation (all punctuation).
    OmitPunctuation = 1 << 1,
    /// Omit tokens of type whitespace (whitespace of all sorts).
    OmitWhitespace = 1 << 2,
    /// Omit tokens of type other (non-linguistic items, such as symbols).
    OmitOther = 1 << 3,
    /// Typically, multiple-word names will be returned as multiple tokens, following the standard tokenization practice of the tagger.
    JoinNames = 1 << 4,
    /// Contractions will be returned as one token.
    JoinContractions = 1 << 5,
}

/// The response to an asset request.
#[derive(Debug, PartialEq, Eq)]
#[repr(i64)]
pub enum NLTaggerAssetsResult {
    /// The asset is now available and loaded onto the device.
    Available,
    /// The asset is unavailable on the device.
    NotAvailable,
    /// The framework couldn’t load the asset due to an error.
    Error,
}

#[interface_impl(NSObject)]
impl NLTagger {
    /* Creating a tagger
     */

    /// Creates a linguistic tagger instance using the specified tag schemes and options.
    #[method]
    pub fn init_with_tag_schemes(&mut self, tag_schemes: NSArray<NLTagScheme>) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithTagSchemes: tag_schemes]) }
    }

    /// The string being analyzed by the linguistic tagger.
    #[property]
    pub fn string(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), string]) }
    }

    /* Getting the tag schemes
     */

    /// Retrieves the tag schemes available for a particular unit (like word or sentence) and language on the current device.
    #[method]
    pub fn available_tag_schemes_for_unit_language(
        unit: NLTokenUnit,
        language: &NLLanguage,
    ) -> NSArray<NLTagScheme> {
        unsafe {
            NSArray::from_id(
                msg_send![Self::m_class(), availableTagSchemesForUnit: unit language: language.m_self()],
            )
        }
    }

    /// Asks the Natural Language framework to load any missing assets for a tag scheme onto the device for the given language.
    #[method]
    pub fn request_assets_for_language_tag_scheme_completion_handler<F>(
        language: &NLLanguage,
        tag_scheme: &NLTagScheme,
        completion_hanlder: F,
    ) where
        F: IntoConcreteBlock<(NLTaggerAssetsResult, NSError), Ret = ()> + 'static,
    {
        unsafe {
            msg_send![Self::m_class(), requestAssetsForLanguage: language.m_self() tagScheme: tag_scheme.m_self() completionHandler: completion_hanlder]
        }
    }

    /// The tag schemes configured for this linguistic tagger.
    #[property]
    pub fn tag_schemes(&self) -> NSArray<NLTagScheme> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), tagSchemes]) }
    }

    /* Determining the dominant language and orthography
     */

    /// The dominant language of the string set for the linguistic tagger.
    #[property]
    pub fn dominant_language(&self) -> NLLanguage {
        unsafe { NLLanguage::from_id(msg_send![self.m_self(), dominantLanguage]) }
    }

    /// Sets the language for a range of text within the tagger's string.
    #[method]
    pub fn set_language_range(&mut self, language: &NLLanguage, range: NSRange) {
        unsafe {
            msg_send![
                self.m_self(), setLanguage: language.m_self() range:range
            ]
        }
    }

    /// Sets the orthography for the specified range.
    #[method]
    pub fn set_orthography_range(&mut self, orthography: &NSOrthography, range: NSRange) {
        unsafe {
            msg_send![
                self.m_self(), setOrthography: orthography.m_self() range: range
            ]
        }
    }

    /* Enumerating linguistic tags
     */

    /// Enumerates a block over the tagger’s string, given a range, token unit, and tag scheme.
    #[method]
    pub fn enumerate_tags_in_range_unit_scheme_options_using_block<F>(
        &mut self,
        range: NSRange,
        unit: NLTokenUnit,
        scheme: &NLTagScheme,
        options: NLTaggerOptions,
        block: F,
    ) where
        F: IntoConcreteBlock<(NLTag, NSRange, *mut bool), Ret = ()> + 'static,
    {
        unsafe {
            msg_send![
                self.m_self(), enumerateTagsInRange: range unit: unit scheme: scheme.m_self() options: options usingBlock: block
            ]
        }
    }
}
