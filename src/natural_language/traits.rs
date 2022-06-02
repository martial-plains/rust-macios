use std::ops::Range;

use crate::{
    foundation::{NSDictionary, NSNumber, NSString, UInt},
    objective_c_runtime::traits::PNSObject,
};

use super::{NLLanguage, NLTokenUnit};

/// A tokenizer that segments natural language text into semantic units.
pub trait INLTokenizer: PNSObject {
    /* Creating a Tokenizer
     */

    /// Creates a tokenizer with the specified unit.
    fn initWithUnit(unit: NLTokenUnit) -> Self;

    /* Configuring a Tokenizer
     */

    /// The text to be tokenized.
    fn string(&self) -> NSString;

    /// Sets the text to be tokenized.
    fn setString<S>(&self, string: S)
    where
        S: Into<NSString>;

    /// Sets the language of the text to be tokenized.
    fn setLanguage(&self, language: NSString);

    /// The linguistic unit that this tokenizer uses.
    fn unit(&self) -> NLTokenUnit;

    /* Enumerating the Tokens
     */

    /// Finds the range of the token at the given index.
    fn tokenRangeAtIndex(&self, character_index: UInt) -> Range<UInt>;

    /// Finds the entire range of all tokens contained completely or partially within the specified range.
    fn tokenRangeForRange(&self, range: Range<UInt>) -> Range<UInt>;
}

/// The language of a body of text.

pub trait INLLanguageRecognizer: PNSObject {
    /* Creating a Recognizer
     */

    /// Creates a recognizer that you can customize.
    fn im_init() -> Self;

    /* Determining the Language
     */

    /// The most likely language for the processed text.
    fn dominantLanguage(&self) -> NSString;

    /// Finds the most likely language of a piece of text.
    fn dominantLanguageFor<T>(&self, string: T) -> NLLanguage
    where
        T: Into<NSString>;

    /// Analyzes the piece of text to determine its dominant language.
    fn processString<T>(&self, string: T)
    where
        T: Into<NSString>;

    /// Generates the probabilities of possible languages for the processed text.
    fn languageHypotheses(&self, max_hypotheses: UInt) -> NSDictionary<NLLanguage, NSNumber>;

    /// Resets the recognizer to its initial state.
    fn reset(&self);

    /* Guiding the Recognizer
     */

    /// A dictionary that maps languages to their probabilities in the language identification process.
    fn languageHints(&self) -> NSDictionary<NLLanguage, NSNumber>;

    /// Sets a dictionary that maps languages to their probabilities in the language identification process.
    fn setLanguageHints(&self, language_hints: NSDictionary<NLLanguage, NSNumber>);

    /// Limits the set of possible languages that the recognizer will return.
    fn languageConstraints(&self) -> NSDictionary<NLLanguage, NSNumber>;

    /// Sets the limits  of the set of possible languages that the recognizer will return.
    fn setLanguageConstraints(&self, language_constraints: NSDictionary<NLLanguage, NSNumber>);
}
