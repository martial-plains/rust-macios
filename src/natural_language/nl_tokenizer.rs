use std::{fmt, ops::Range};

use block::{ConcreteBlock, IntoConcreteBlock, RcBlock};
use objc::{
    class, msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::{NSArray, NSRange, NSString, UInt},
    objective_c_runtime::{
        id,
        traits::{FromId, INSValue, PNSObject},
    },
};

use super::{traits::INLTokenizer, NLLanguage, NLTokenUnit, NLTokenizerAttributes};

/// A tokenizer that segments natural language text into semantic units.
#[repr(transparent)]
pub struct NLTokenizer {
    /// The underlying Objective-C object.
    pub obj: Id<Object>,
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

impl PNSObject for NLTokenizer {
    fn im_class<'a>() -> &'a Class {
        class!(NLTokenizer)
    }

    fn im_is_equal(&self, object: &Self) -> bool {
        unsafe { msg_send![self.obj, isEqual: object] }
    }

    fn ip_hash(&self) -> UInt {
        unsafe { msg_send![self.obj, hash] }
    }

    fn im_is_kind_of_class(&self, class: Class) -> bool {
        unsafe { msg_send![self.obj, isKindOfClass: class] }
    }

    fn im_is_member_of_class(&self, class: Class) -> bool {
        unsafe { msg_send![self.obj, isMemberOfClass: class] }
    }

    fn im_responds_to_selector(&self, selector: Sel) -> bool {
        unsafe { msg_send![self.obj, respondsToSelector: selector] }
    }

    fn im_conforms_to_protocol(&self, protocol: Protocol) -> bool {
        unsafe { msg_send![self.obj, conformsToProtocol: protocol] }
    }

    fn ip_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.obj, description]) }
    }

    fn ip_debug_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.obj, debugDescription]) }
    }

    fn im_perform_selector(&self, selector: Sel) -> id {
        unsafe { msg_send![self.obj, performSelector: selector] }
    }

    fn im_perform_selector_with_object(&self, selector: Sel, with_object: id) -> id {
        unsafe { msg_send![self.obj, performSelector: selector withObject: with_object] }
    }

    fn im_is_proxy(&self) -> bool {
        unsafe { msg_send![self.obj, isProxy] }
    }
}

impl INLTokenizer for NLTokenizer {
    /* Creating a Tokenizer
     */

    fn im_init_with_unit(unit: NLTokenUnit) -> Self {
        unsafe {
            let cls = class!(NLTokenizer);
            let obj: *mut Object = msg_send![cls, new];
            let obj = msg_send![obj, initWithUnit: unit];
            Self { obj }
        }
    }

    fn ip_string(&self) -> NSString {
        unsafe { msg_send![&*self.obj, string] }
    }

    fn ip_set_string(&self, string: NSString) {
        unsafe { msg_send![self.obj, setString: string] }
    }

    fn im_set_language(&self, language: NLLanguage) {
        unsafe { msg_send![self.obj, setLanguage: language] }
    }

    fn ip_unit(&self) -> NLTokenUnit {
        unsafe { msg_send![self.obj, unit] }
    }

    fn im_enumerate_tokens_in_range_using_block(
        &self,
        range: NSRange,
        block: RcBlock<(NSRange, NLTokenizerAttributes, *mut bool), ()>,
    ) {
        unsafe {
            msg_send![
                self.obj,
                enumerateTokensInRange: range
                usingBlock: block
            ]
        }
    }

    fn im_tokens_for_range<T>(&self, range: NSRange) -> NSArray<T>
    where
        T: INSValue,
    {
        unsafe { NSArray::from_id(msg_send![self.obj, tokensForRange: range]) }
    }

    fn im_token_range_at_index(&self, character_index: UInt) -> NSRange {
        unsafe { msg_send![self.obj, tokenRangeAtIndex: character_index] }
    }

    fn im_token_range_for_range(&self, range: NSRange) -> NSRange {
        unsafe { msg_send![self.obj, tokenRangeForRange: range] }
    }
}

impl fmt::Debug for NLTokenizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NLTokenizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

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
