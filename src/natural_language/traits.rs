use block::RcBlock;

use crate::{
    foundation::{NSArray, NSDictionary, NSNumber, NSRange, NSString, UInt},
    objective_c_runtime::traits::{INSValue, PNSObject},
};

use super::{NLLanguage, NLTokenUnit, NLTokenizerAttributes};

/// A tokenizer that segments natural language text into semantic units.
pub trait INLTokenizer: PNSObject {
    /* Creating a Tokenizer
     */

    /// Creates a tokenizer with the specified unit.
    fn im_initWithUnit(unit: NLTokenUnit) -> Self;

    /* Configuring a Tokenizer
     */

    /// The text to be tokenized.
    fn ip_string(&self) -> NSString;

    /// Sets the text to be tokenized.
    fn ip_setString(&self, string: NSString);

    /// Sets the language of the text to be tokenized.
    fn im_setLanguage(&self, language: NLLanguage);

    /// The linguistic unit that this tokenizer uses.
    fn ip_unit(&self) -> NLTokenUnit;

    /* Enumerating the Tokens
     */

    /// Enumerates over a given range of the string and calls the specified block for each token.
    fn im_enumerateTokensInRange_usingBlock(
        &self,
        range: NSRange,
        block: RcBlock<(NSRange, NLTokenizerAttributes, *mut bool), ()>,
    );

    /// Tokenizes the string within the provided range.
    fn im_tokensForRange<T>(&self, range: NSRange) -> NSArray<T>
    where
        T: INSValue;

    /// Finds the range of the token at the given index.
    fn im_tokenRangeAtIndex(&self, character_index: UInt) -> NSRange;

    /// Finds the entire range of all tokens contained completely or partially within the specified range.
    fn im_tokenRangeForRange(&self, range: NSRange) -> NSRange;
}

/// The language of a body of text.

pub trait INLLanguageRecognizer: PNSObject {
    /* Creating a Recognizer
     */

    /// Creates a recognizer that you can customize.
    fn im_init() -> Self;

    /* Determining the Language
     */

    /// Finds the most likely language of a piece of text.
    fn tm_dominantLanguageForString(&self, string: NSString) -> NLLanguage;

    /// Analyzes the piece of text to determine its dominant language.
    fn im_processString(&mut self, string: NSString);

    /// The most likely language for the processed text.
    fn ip_dominantLanguage(&self) -> NSString;

    /// Generates the probabilities of possible languages for the processed text.
    fn im_languageHypothesesWithMaximum(
        &self,
        max_hypotheses: UInt,
    ) -> NSDictionary<NLLanguage, NSNumber>;

    /// Resets the recognizer to its initial state.
    fn im_reset(&self);

    /* Guiding the Recognizer
     */

    /// A dictionary that maps languages to their probabilities in the language identification process.
    fn ip_languageHints(&self) -> NSDictionary<NLLanguage, NSNumber>;

    /// Sets a dictionary that maps languages to their probabilities in the language identification process.
    fn ip_setLanguageHints(&self, language_hints: NSDictionary<NLLanguage, NSNumber>);

    /// Limits the set of possible languages that the recognizer will return.
    fn ip_languageConstraints(&self) -> NSArray<NLLanguage>;

    /// Sets the limits  of the set of possible languages that the recognizer will return.
    fn ip_setLanguageConstraints(&self, language_constraints: NSArray<NLLanguage>);
}
