use block::RcBlock;
use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::{NSArray, NSDictionary, NSNumber, NSRange, NSString, UInt},
    objective_c_runtime::{
        id,
        traits::{FromId, INSValue, PNSObject},
    },
};

use super::{NLLanguage, NLTokenUnit, NLTokenizerAttributes};

/// A tokenizer that segments natural language text into semantic units.
pub trait INLTokenizer: PNSObject {
    /* Creating a Tokenizer
     */

    /// Creates a tokenizer with the specified unit.
    fn im_init_with_unit(unit: NLTokenUnit) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            let cls = Self::im_class();
            let obj: id = msg_send![cls, new];
            let tokenizer = msg_send![obj, initWithUnit: unit];
            Self::from_id(tokenizer)
        }
    }

    /* Configuring a Tokenizer
     */

    /// The text to be tokenized.
    fn ip_string(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), string]) }
    }

    /// Sets the text to be tokenized.
    fn ip_set_string(&self, string: NSString) {
        unsafe { msg_send![self.im_self(), setString: string] }
    }

    /// Sets the language of the text to be tokenized.
    fn im_set_language(&self, language: NLLanguage) {
        unsafe { msg_send![self.im_self(), setLanguage: language] }
    }

    /// The linguistic unit that this tokenizer uses.
    fn ip_unit(&self) -> NLTokenUnit {
        unsafe { msg_send![self.im_self(), unit] }
    }

    /* Enumerating the Tokens
     */

    /// Enumerates over a given range of the string and calls the specified block for each token.
    fn im_enumerate_tokens_in_range_using_block(
        &self,
        range: NSRange,
        block: RcBlock<(NSRange, NLTokenizerAttributes, *mut bool), ()>,
    ) {
        unsafe {
            msg_send![
                self.im_self(),
                enumerateTokensInRange: range
                usingBlock: block
            ]
        }
    }

    /// Tokenizes the string within the provided range.
    fn im_tokens_for_range<T>(&self, range: NSRange) -> NSArray<T>
    where
        T: INSValue,
    {
        unsafe { NSArray::from_id(msg_send![self.im_self(), tokensForRange: range]) }
    }

    /// Finds the range of the token at the given index.
    fn im_token_range_at_index(&self, character_index: UInt) -> NSRange {
        unsafe { msg_send![self.im_self(), tokenRangeAtIndex: character_index] }
    }

    /// Finds the entire range of all tokens contained completely or partially within the specified range.
    fn im_token_range_for_range(&self, range: NSRange) -> NSRange {
        unsafe { msg_send![self.im_self(), tokenRangeForRange: range] }
    }
}

/// The language of a body of text.

pub trait INLLanguageRecognizer: PNSObject {
    /* Creating a Recognizer
     */

    /// Creates a recognizer that you can customize.
    fn im_init() -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            let cls = Self::im_class();
            let obj: id = msg_send![cls, new];
            Self::from_id(obj)
        }
    }

    /* Determining the Language
     */

    /// Finds the most likely language of a piece of text.
    fn tm_dominant_language_for_string(&self, string: NSString) -> NLLanguage {
        unsafe {
            NLLanguage::from_id(msg_send![
                Self::im_class(),
                dominantLanguageForString: string
            ])
        }
    }

    /// Analyzes the piece of text to determine its dominant language.
    fn im_process_string(&mut self, string: NSString) {
        unsafe { msg_send![self.im_self(), processString: string] }
    }

    /// The most likely language for the processed text.
    fn ip_dominant_language(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), dominantLanguage]) }
    }

    /// Generates the probabilities of possible languages for the processed text.
    fn im_language_hypotheses_with_maximum(
        &self,
        max_hypotheses: UInt,
    ) -> NSDictionary<NLLanguage, NSNumber> {
        unsafe {
            NSDictionary::from_id(msg_send![
                self.im_self(),
                languageHypothesesWithMaximum: max_hypotheses
            ])
        }
    }

    /// Resets the recognizer to its initial state.
    fn im_reset(&self) {
        unsafe { msg_send![self.im_self(), reset] }
    }

    /* Guiding the Recognizer
     */

    /// A dictionary that maps languages to their probabilities in the language identification process.
    fn ip_language_hints(&self) -> NSDictionary<NLLanguage, NSNumber> {
        unsafe { NSDictionary::from_id(msg_send![self.im_self(), languageHints]) }
    }

    /// Sets a dictionary that maps languages to their probabilities in the language identification process.
    fn ip_set_language_hints(&self, language_hints: NSDictionary<NLLanguage, NSNumber>) {
        unsafe { msg_send![self.im_self(), setLanguageHints: language_hints] }
    }

    /// Limits the set of possible languages that the recognizer will return.
    fn ip_language_constraints(&self) -> NSArray<NLLanguage> {
        unsafe { NSArray::from_id(msg_send![self.im_self(), languageConstraints]) }
    }

    /// Sets the limits  of the set of possible languages that the recognizer will return.
    fn ip_set_language_constraints(&self, language_constraints: NSArray<NLLanguage>) {
        unsafe { msg_send![self.im_self(), setLanguageConstraints: language_constraints] }
    }
}
