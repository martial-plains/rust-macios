use block::{ConcreteBlock, IntoConcreteBlock};
use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::{NSArray, NSRange, NSString, UInt},
    objective_c_runtime::{
        macros::object,
        traits::{FromId, PNSObject},
        INSValue,
    },
};

use super::{NLLanguage, NLTokenUnit};

/// Hints about the contents of the string for the tokenizer.
#[derive(Debug, PartialEq, Eq)]
#[repr(u64)]
pub enum NLTokenizerAttributes {
    /// Doesn't contain any special attributes.
    None = 0,
    /// The string contains numbers.
    Numeric = 1 << 0,
    /// The string contains symbols.
    Symbolic = 1 << 1,
    /// The string contains emoji.
    Emoji = 1 << 2,
}

object! {
    /// A tokenizer that segments natural language text into semantic units.
    unsafe pub struct NLTokenizer;
}

#[interface_impl(NSObject)]
impl NLTokenizer {
    /* Creating a Tokenizer
     */

    /// Creates a tokenizer with the specified unit.
    #[method]
    pub fn init_with_unit(&mut self, unit: NLTokenUnit) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithUnit: unit]) }
    }

    /* Configuring a Tokenizer
     */

    /// The text to be tokenized.
    #[property]
    pub fn string(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), string]) }
    }

    /// Sets the text to be tokenized.
    #[property]
    pub fn set_string(&mut self, string: NSString) {
        unsafe { msg_send![self.m_self(), setString: string] }
    }

    /// Sets the language of the text to be tokenized.
    #[method]
    pub fn set_language(&mut self, language: NLLanguage) {
        unsafe { msg_send![self.m_self(), setLanguage: language] }
    }

    /// The linguistic unit that this tokenizer uses.
    #[property]
    pub fn unit(&self) -> NLTokenUnit {
        unsafe { msg_send![self.m_self(), unit] }
    }

    /* Enumerating the Tokens
     */

    /// Enumerates over a given range of the string and calls the specified block for each token.
    #[method]
    pub fn enumerate_tokens_in_range_using_block<F>(&self, range: NSRange, block: F)
    where
        F: IntoConcreteBlock<(NSRange, NLTokenizerAttributes, *mut bool), Ret = ()> + 'static,
    {
        let block = ConcreteBlock::new(block);
        let block = block.copy();
        unsafe {
            msg_send![
                self.m_self(),
                enumerateTokensInRange: range
                usingBlock: block
            ]
        }
    }

    /// Tokenizes the string within the provided range.
    #[method]
    pub fn tokens_for_range<T>(&self, range: NSRange) -> NSArray<T>
    where
        T: INSValue,
    {
        unsafe { NSArray::from_id(msg_send![self.m_self(), tokensForRange: range]) }
    }

    /// Finds the range of the token at the given index.
    #[method]
    pub fn token_range_at_index(&self, character_index: UInt) -> NSRange {
        unsafe { msg_send![self.m_self(), tokenRangeAtIndex: character_index] }
    }

    /// Finds the entire range of all tokens contained completely or partially within the specified range.
    #[method]
    pub fn token_range_for_range(&self, range: NSRange) -> NSRange {
        unsafe { msg_send![self.m_self(), tokenRangeForRange: range] }
    }
}

impl Default for NLTokenizer {
    fn default() -> Self {
        Self::m_new()
    }
}

#[cfg(test)]
mod tests {

    use crate::natural_language::English;

    use super::*;

    #[test]
    fn test_init() {
        let mut tokenizer = NLTokenizer::default();
        tokenizer = tokenizer.init_with_unit(NLTokenUnit::Word);
        assert_ne!(tokenizer.unit(), super::NLTokenUnit::Sentence);
        assert_eq!(tokenizer.unit(), super::NLTokenUnit::Word);
    }

    #[test]
    fn test_string() {
        let mut tokenizer = NLTokenizer::default();
        tokenizer = tokenizer.init_with_unit(NLTokenUnit::Word);
        tokenizer.set_string("Hello, world!".into());
        assert_ne!(tokenizer.string(), "Goodbye, world!");
        assert_eq!(tokenizer.string(), "Hello, world!");
    }

    #[test]
    fn test_set_string() {
        let mut tokenizer = NLTokenizer::default();
        tokenizer = tokenizer.init_with_unit(NLTokenUnit::Word);
        tokenizer.set_string("Hello, world!".into());
        assert_ne!(tokenizer.string(), "Goodbye, world!");
        assert_eq!(tokenizer.string(), "Hello, world!");
    }

    #[test]
    fn test_set_language() {
        let mut tokenizer = NLTokenizer::default();
        tokenizer = tokenizer.init_with_unit(NLTokenUnit::Word);
        tokenizer.set_language(unsafe { English.clone() });
    }

    #[test]
    fn test_unit() {
        let mut tokenizer = NLTokenizer::default();
        tokenizer = tokenizer.init_with_unit(NLTokenUnit::Word);
        assert_ne!(tokenizer.unit(), NLTokenUnit::Sentence);
        assert_eq!(tokenizer.unit(), NLTokenUnit::Word);
    }

    #[test]
    fn test_token_range_at_index() {
        let mut tokenizer = NLTokenizer::default();
        tokenizer = tokenizer.init_with_unit(NLTokenUnit::Word);
        tokenizer.set_string("Hello, world!".into());
        assert_eq!(tokenizer.token_range_at_index(0), (0..5).into());
    }

    #[test]
    fn test_token_range_for_range() {
        let mut tokenizer = NLTokenizer::default();
        tokenizer = tokenizer.init_with_unit(NLTokenUnit::Word);
        tokenizer.set_string("Hello, world!".into());
        assert_eq!(
            tokenizer.token_range_for_range((0..5).into()),
            (0..5).into()
        );
    }

    #[test]
    fn test_enumerate_tokens_in_range() {
        let mut tokenizer = NLTokenizer::default();
        tokenizer = tokenizer.init_with_unit(NLTokenUnit::Word);
        let text = "Hello";
        tokenizer.set_string(text.into());
        tokenizer.enumerate_tokens_in_range_using_block((0..text.len()).into(), |_, attr, _| {
            assert_eq!(attr, NLTokenizerAttributes::None);
        });
    }
}
