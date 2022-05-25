
use std::ops::Range;

use crate::{
    foundation::{Dictionary, NSNumber, String, UInt},
    objective_c_runtime::traits::t_NSObject,
};

use super::{NLLanguage, NLTokenUnit};

/// A tokenizer that segments natural language text into semantic units.
pub trait t_NLTokenizer: t_NSObject {
    /* Creating a Tokenizer
     */

    /// Creates a tokenizer with the specified unit.
    fn initWithUnit(unit: NLTokenUnit) -> Self;

    /* Configuring a Tokenizer
     */

    /// The text to be tokenized.
    fn string(&self) -> String;

    /// Sets the text to be tokenized.
    fn setString<S>(&self, string: S)
    where
        S: Into<String>;

    /// Sets the language of the text to be tokenized.
    fn setLanguage(&self, language: String);

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

pub trait t_NLLanguageRecognizer: t_NSObject {
    /// The most likely language for the processed text.
    fn dominantLanguage(&self) -> String;

    /// Finds the most likely language of a piece of text.
    fn dominantLanguageFor<T>(&self, string: T) -> NLLanguage
    where
        T: Into<String>;

    /// Analyzes the piece of text to determine its dominant language.
    fn processString<T>(&self, string: T)
    where
        T: Into<String>;

    /// Generates the probabilities of possible languages for the processed text.
    fn languageHypotheses(&self, max_hypotheses: UInt) -> Dictionary<NLLanguage, NSNumber>;

    /// Resets the recognizer to its initial state.
    fn reset(&self);

    /* Guiding the Recognizer
     */

    /// A dictionary that maps languages to their probabilities in the language identification process.
    fn languageHints(&self) -> Dictionary<NLLanguage, NSNumber>;

    /// Sets a dictionary that maps languages to their probabilities in the language identification process.
    fn setLanguageHints(&self, language_hints: Dictionary<NLLanguage, NSNumber>);

    /// Limits the set of possible languages that the recognizer will return.
    fn languageConstraints(&self) -> Dictionary<NLLanguage, NSNumber>;

    /// Sets the limits  of the set of possible languages that the recognizer will return.
    fn setLanguageConstraints(&self, language_constraints: Dictionary<NLLanguage, NSNumber>);
}
