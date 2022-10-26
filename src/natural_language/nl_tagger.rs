use block::{ConcreteBlock, IntoConcreteBlock};
use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::{
        NSArray, NSDictionary, NSError, NSNumber, NSOrthography, NSRange, NSRangePointer, NSString,
        UInt,
    },
    objective_c_runtime::{
        id,
        macros::{interface_impl, object},
        nil,
        traits::{FromId, PNSObject},
        NSValue,
    },
    utils::to_optional,
};

use super::{NLGazetteer, NLLanguage, NLModel};

/// Constants representing linguistic units.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(i64)]
pub enum NLTokenUnit {
    /// An individual word.
    Word,
    /// An individual sentence.
    Sentence,
    /// An individual paragraph.
    Paragraph,
    /// The document in its entirety.
    Document,
}

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

        /// A tag indicating that the token is an adverb.
        #[link_name = "NLTagAdverb"]
        pub static Adverb: NLTag;

        /// A tag indicating that the token is a pronoun.
        #[link_name = "NLTagPronoun"]
        pub static Pronoun: NLTag;

        /// A tag indicating that the token is a determiner.
        #[link_name = "NLTagDeterminer"]
        pub static Determiner: NLTag;

        /// A tag indicating that the token is a particle.
        #[link_name = "NLTagParticle"]
        pub static Particle: NLTag;

        /// A tag indicating that the token is a preposition.
        #[link_name = "NLTagPreposition"]
        pub static Preposition: NLTag;

        /// A tag indicating that the token is a number.
        #[link_name = "NLTagNumber"]
        pub static Number: NLTag;

        /// A tag indicating that the token is a conjunction.
        #[link_name = "NLTagConjunction"]
        pub static Conjuction: NLTag;

        /// A tag indicating that the token is an interjection.
        #[link_name = "NLTagInterjection"]
        pub static Interjection: NLTag;

        /// A tag indicating that the token is a classifier.
        #[link_name = "NLTagClassifier"]
        pub static Classifier: NLTag;

        /// A tag indicating that the token is an idiom.
        #[link_name = "NLTagIdiom"]
        pub static Idion: NLTag;

        /// A tag indicating that the token is a word other than a kind described by other lexical classes (noun, verb, adjective, adverb, pronoun, determiner, particle, preposition, number, conjunction, interjection, classifier, and idiom).
        #[link_name = "NLTagOtherWord"]
        pub static OtherWord: NLTag;

        /// A tag indicating that the token is punctuation at the end of a sentence.
        #[link_name = "NLTagSentenceTerminator"]
        pub static SentenceTerminator: NLTag;

        /// A tag indicating that the token is an open quote.
        #[link_name = "NLTagOpenQuote"]
        pub static OpenQuote: NLTag;

        /// A tag indicating that the token is a close quote.
        #[link_name = "NLTagCloseQuote"]
        pub static CloseQuote: NLTag;

        /// A tag indicating that the token is an open parenthesis.
        #[link_name = "NLTagOpenParenthesis"]
        pub static OpenParenthesis: NLTag;

        /// A tag indicating that the token is a close parenthesis.
        #[link_name = "NLTagCloseParenthesis"]
        pub static CloseParenthesis: NLTag;

        /// A tag indicating that the token is a word joiner, signifying that two tokens on each side should not be broken up.
        #[link_name = "NLTagWordJoiner"]
        pub static WordJoiner: NLTag;

        /// A tag indicating that the token is a dash.
        #[link_name = "NLTagDash"]
        pub static Dash: NLTag;

        /// A tag indicating that the token is punctuation other than a kind described by other lexical classes (sentence terminator, open or close quote, open or close parenthesis, word joiner, and dash).
        #[link_name = "NLTagOtherPunctuation"]
        pub static OtherPunctuation: NLTag;

        /// A tag indicating that the token is a paragraph break.
        #[link_name = "NLTagParagraphBreak"]
        pub static ParagraphBreak: NLTag;

        /// A tag indicating that the token is whitespace other than a kind described by other lexical classes (paragraph break).
        #[link_name = "NLTagOtherWhitespace"]
        pub static OtherWhitespace: NLTag;

        /* Name types
         */

        /// A tag indicating that the token is a personal name.
        #[link_name = "NLTagPersonalName"]
        pub static PersonalName: NLTag;

        /// A tag indicating that the token is an organization name.
        #[link_name = "NLTagOrganizationName"]
        pub static OrganizationName: NLTag;

        /// A tag indicating that the token is a place name.
        #[link_name = "NLTagPlaceName"]
        pub static PlaceName: NLTag;
    }
}

/// Constants for linguistic tagger enumeration specifying which tokens to omit and whether to join names.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
    pub fn string(&self) -> Option<NSString> {
        unsafe { to_optional(msg_send![self.m_self(), string]) }
    }

    /// Sets the string being analyzed by the linguistic tagger.
    #[property]
    pub fn set_string(&mut self, value: Option<NSString>) {
        unsafe {
            msg_send![self.m_self(), setString: if let Some(value) = value {value.m_self()} else {nil} ]
        }
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
            let completion_hanlder = ConcreteBlock::new(completion_hanlder);
            let completion_hanlder = completion_hanlder.copy();

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
    pub fn dominant_language(&self) -> Option<NLLanguage> {
        unsafe { to_optional(msg_send![self.m_self(), dominantLanguage]) }
    }

    /// Sets the dominant language of the string set for the linguistic tagger.
    #[property]
    pub fn set_dominant_language(&self, language: Option<&NLLanguage>) {
        unsafe {
            msg_send![self.m_self(), setDominantLanguage: if let Some(language) = language { language.m_self() } else { nil }]
        }
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
        &self,
        range: NSRange,
        unit: NLTokenUnit,
        scheme: &NLTagScheme,
        options: &[NLTaggerOptions],
        block: F,
    ) where
        F: IntoConcreteBlock<(NLTag, NSRange, *mut bool), Ret = ()> + 'static,
    {
        unsafe {
            let options = options.iter().fold(0, |init, unit| init | *unit as UInt);

            let block = ConcreteBlock::new(block);
            let block = block.copy();

            msg_send![
                self.m_self(), enumerateTagsInRange: range unit: unit scheme: scheme.m_self() options: options usingBlock: block
            ]
        }
    }

    /* Getting linguistic tags
     */

    /// Finds an array of linguistic tags and token ranges for a given string range and linguistic unit.
    #[method]
    pub fn tags_in_range_unit_scheme_options_token_ranges(
        &self,
        range: NSRange,
        unit: NLTokenUnit,
        scheme: &NLTagScheme,
        options: &[NLTaggerOptions],
        token_ranges: &mut NSArray<NSValue>,
    ) -> NSArray<NLTag> {
        unsafe {
            let options = options
                .iter()
                .fold(0, |init, option| init | *option as UInt);

            NSArray::from_id(
                msg_send![self.m_self(), tagsInRange: range unit:unit scheme: scheme.m_self() options: options tokenRanges: token_ranges
                ],
            )
        }
    }

    /// Finds a tag for a given linguistic unit, for a single scheme, at the specified character position.
    #[method]
    pub fn tag_at_index_unit_scheme_token_range(
        &self,
        character_index: UInt,
        unit: NLTokenUnit,
        scheme: &NLTagScheme,
        token_range: NSRangePointer,
    ) -> Option<NLTag> {
        unsafe {
            let ptr: id = msg_send![self.m_self(), tagAtIndex:character_index unit: unit scheme: scheme.m_self() tokenRange: token_range];

            if ptr != nil {
                Some(NLTag::from_id(ptr))
            } else {
                None
            }
        }
    }

    /// Finds multiple possible tags for a given linguistic unit, for a single scheme, at the specified character position.
    #[method]
    pub fn tag_hypotheses_at_index_unit_scheme_maximum_count_token_range(
        &self,
        character_index: UInt,
        unit: NLTokenUnit,
        scheme: &NLTagScheme,
        maximum_count: UInt,
        token_range: Option<NSRangePointer>,
    ) -> NSDictionary<NLTag, NSNumber> {
        unsafe {
            let token_range = match token_range {
                Some(value) => value,
                None => std::ptr::null_mut(),
            };

            NSDictionary::from_id(
                msg_send![self.m_self(), tagHypothesesAtIndex:character_index unit: unit scheme: scheme.m_self() maximumCount: maximum_count tokenRange: token_range],
            )
        }
    }

    /* Determining the range of a unit token */

    /// Returns the range of the linguistic unit containing the specified character index.
    #[method]
    pub fn token_range_at_index_unit(&self, character_index: UInt, unit: NLTokenUnit) -> NSRange {
        unsafe { msg_send![self.m_self(), tokenRangeAtIndex: character_index unit: unit] }
    }

    /// Finds the entire range of all tokens of the specified linguistic unit contained completely or partially within the specified range.
    #[method]
    pub fn token_range_for_range_unit(&self, range: NSRange, unit: NLTokenUnit) -> NSRange {
        unsafe { msg_send![self.m_self(), tokenRangeForRange: range unit: unit] }
    }

    /* Using models with a tagger
     */

    /// Assigns models for a tag scheme.
    #[method]
    pub fn set_models_for_tag_scheme(
        &mut self,
        models: &NSArray<NLModel>,
        tag_scheme: &NLTagScheme,
    ) {
        unsafe {
            msg_send![self.m_self(),  setModels:models.m_self() forTagScheme: tag_scheme.m_self()]
        }
    }

    /// Returns the models that apply to the given tag scheme.
    #[method]
    pub fn models_for_tag_scheme(&self, tag_scheme: &NLTagScheme) -> NSArray<NLModel> {
        unsafe {
            NSArray::from_id(msg_send![self.m_self(), modelsForTagScheme: tag_scheme.m_self()])
        }
    }

    /* Using gazetteers with a tagger
     */

    /// Attaches gazetteers to a tag scheme, typically one gazetteer per language or one language-independent gazetteer.
    #[method]
    pub fn set_gazetteers_for_tag_scheme(
        &mut self,
        gazetteers: &NSArray<NLGazetteer>,
        tag_scheme: &NLTagScheme,
    ) {
        unsafe {
            msg_send![self.m_self(), setGazetteers: gazetteers.m_self() forTagScheme: tag_scheme.m_self()]
        }
    }

    /// Retrieves the gazetteers attached to a tag scheme.
    #[method]
    pub fn gazetteers_for_tag_scheme(&self, tag_scheme: &NLTagScheme) -> NSArray<NLGazetteer> {
        unsafe {
            NSArray::from_id(msg_send![self.m_self(), gazetteersForTagScheme: tag_scheme.m_self()])
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ptr::addr_of_mut;

    use crate::{
        foundation::{macros::nsarray, ns_array::INSArray, INSDictionary, NSNumber, NSRange, UInt},
        natural_language::{nl_language, NLTokenUnit},
        objective_c_runtime::traits::PNSObject,
    };

    use super::{nl_tag_scheme, NLTagger};

    const TEXT: &str =
        "The ripe taste of cheese improves with age, but it can not get younger nor shouldn't it.";

    #[test]
    fn test_get_tag() {
        unsafe {
            let mut tagger =
                NLTagger::m_new().init_with_tag_schemes(nsarray![nl_tag_scheme::Lemma.clone()]);

            tagger.set_string(Some(TEXT.into()));

            let mut range = NSRange::default();

            let tag = tagger.tag_at_index_unit_scheme_token_range(
                0,
                NLTokenUnit::Word,
                &nl_tag_scheme::Lemma,
                addr_of_mut!(range),
            );

            assert!(tagger.dominant_language() == Some(nl_language::English.clone()));
            assert!(range.location == 0);
            assert!(range.length == 3);

            if let Some(tag) = tag {
                assert!(tag == "the");
            }
        }
    }

    #[test]
    fn test_get_tag_hypotheses() {
        unsafe {
            let mut tagger = NLTagger::m_new()
                .init_with_tag_schemes(nsarray![nl_tag_scheme::LexicalClass.clone()]);

            tagger.set_string(Some(TEXT.into()));

            let dict = tagger.tag_hypotheses_at_index_unit_scheme_maximum_count_token_range(
                0,
                NLTokenUnit::Sentence,
                &nl_tag_scheme::LexicalClass,
                UInt::MAX,
                None,
            );

            assert!(dict.count() == 1);
            let item = dict.p_all_values().m_object_at_index(0);
            assert!(item == NSNumber::from(1.0))
        }
    }

    #[test]
    fn test_get_tag_hypotheses_range() {
        unsafe {
            let mut tagger = NLTagger::m_new()
                .init_with_tag_schemes(nsarray![nl_tag_scheme::LexicalClass.clone()]);

            tagger.set_string(Some(TEXT.into()));

            let mut range = NSRange::default();

            let dict = tagger.tag_hypotheses_at_index_unit_scheme_maximum_count_token_range(
                0,
                NLTokenUnit::Sentence,
                &nl_tag_scheme::LexicalClass,
                UInt::MAX,
                Some(addr_of_mut!(range)),
            );

            assert!(dict.count() == 1);
            let item = dict.p_all_values().m_object_at_index(0);
            assert!(item == NSNumber::from(1.0));
            assert!(range.location == 0);
            assert!(range.length == 88);
        }
    }
}
