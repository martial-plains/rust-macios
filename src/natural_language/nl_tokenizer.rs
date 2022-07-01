use std::ops::Range;

use block::{ConcreteBlock, IntoConcreteBlock};

use crate::{
    foundation::{NSArray, NSRange, NSString, UInt},
    objective_c_runtime::{macros::object, traits::INSValue},
};

use super::{traits::INLTokenizer, NLLanguage, NLTokenUnit, NLTokenizerAttributes};

object! {
    /// A tokenizer that segments natural language text into semantic units.
    unsafe pub struct NLTokenizer;
}

impl NLTokenizer {
    /// Creates a tokenizer with the specified unit.
    pub fn with_unit(unit: NLTokenUnit) -> Self {
        Self::im_init_with_unit(unit)
    }

    /// The text to be tokenized.
    pub fn string(&self) -> NSString {
        self.ip_string()
    }

    /// Sets the text to be tokenized.
    pub fn set_string<S>(&self, string: S)
    where
        S: Into<NSString>,
    {
        self.ip_set_string(string.into())
    }

    /// Sets the language of the text to be tokenized.
    pub fn set_language<L>(&self, language: L)
    where
        L: Into<NLLanguage>,
    {
        self.im_set_language(language.into())
    }

    /// The linguistic unit that this tokenizer uses.
    pub fn unit(&self) -> NLTokenUnit {
        self.ip_unit()
    }

    /// Enumerates over a given range of the string and calls the specified block for each token.
    ///
    /// # Argumnets
    ///
    /// * `range` - The range of the string to be tokenized.
    /// * `block` - The block to be called for each token.
    pub fn enumerate_tokens_in_range<F>(&self, range: NSRange, block: F)
    where
        F: IntoConcreteBlock<(NSRange, NLTokenizerAttributes, *mut bool), Ret = ()> + 'static,
    {
        let block = ConcreteBlock::new(block);
        let block = block.copy();
        self.im_enumerate_tokens_in_range_using_block(range, block)
    }

    /// Tokenizes the string within the provided range.
    pub fn tokens_for_range<T>(&self, range: NSRange) -> NSArray<T>
    where
        T: INSValue,
    {
        self.im_tokens_for_range(range)
    }

    /// Finds the range of the token at the given index.
    pub fn token_range_at(&self, index: UInt) -> NSRange {
        self.im_token_range_at_index(index)
    }

    /// Finds the entire range of all tokens contained completely or partially within the specified range.
    pub fn token_range_for(&self, range: Range<usize>) -> NSRange {
        self.im_token_range_for_range(range.into())
    }
}

impl INLTokenizer for NLTokenizer {}

#[cfg(test)]
mod tests {

    use crate::natural_language::English;

    use super::*;

    #[test]
    fn test_init() {
        let tokenizer = NLTokenizer::with_unit(NLTokenUnit::Word);
        assert_ne!(tokenizer.unit(), super::NLTokenUnit::Sentence);
        assert_eq!(tokenizer.unit(), super::NLTokenUnit::Word);
    }

    #[test]
    fn test_string() {
        let tokenizer = NLTokenizer::with_unit(NLTokenUnit::Word);
        tokenizer.set_string("Hello, world!");
        assert_ne!(tokenizer.string(), "Goodbye, world!");
        assert_eq!(tokenizer.string(), "Hello, world!");
    }

    #[test]
    fn test_set_string() {
        let tokenizer = NLTokenizer::with_unit(NLTokenUnit::Word);
        tokenizer.set_string("Hello, world!");
        assert_ne!(tokenizer.string(), "Goodbye, world!");
        assert_eq!(tokenizer.string(), "Hello, world!");
    }

    #[test]
    fn test_set_language() {
        let tokenizer = NLTokenizer::with_unit(NLTokenUnit::Word);
        tokenizer.set_language(unsafe { English.clone() });
    }

    #[test]
    fn test_unit() {
        let tokenizer = NLTokenizer::with_unit(NLTokenUnit::Word);
        assert_ne!(tokenizer.unit(), NLTokenUnit::Sentence);
        assert_eq!(tokenizer.unit(), NLTokenUnit::Word);
    }

    #[test]
    fn test_token_range_at_index() {
        let tokenizer = NLTokenizer::with_unit(NLTokenUnit::Word);
        tokenizer.set_string("Hello, world!");
        assert_eq!(tokenizer.token_range_at(0), (0..5).into());
    }

    #[test]
    fn test_token_range_for_range() {
        let tokenizer = NLTokenizer::with_unit(NLTokenUnit::Word);
        tokenizer.set_string("Hello, world!");
        assert_eq!(tokenizer.token_range_for(0..5), (0..5).into());
    }

    #[test]
    fn test_enumerate_tokens_in_range() {
        let tokenizer = NLTokenizer::with_unit(NLTokenUnit::Word);
        let text = "Hello";
        tokenizer.set_string(text);
        tokenizer.enumerate_tokens_in_range((0..text.len()).into(), |_, attr, _| {
            assert_eq!(attr, NLTokenizerAttributes::None);
        });
    }
}
