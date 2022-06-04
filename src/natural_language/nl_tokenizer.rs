use std::{fmt, ops::Range};

use block::{ConcreteBlock, IntoConcreteBlock, RcBlock};
use objc::{
    class, msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::{NSArray, NSRange, NSString, UInt},
    id,
    objective_c_runtime::traits::{FromId, INSValue, PNSObject},
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
        Self::im_initWithUnit(unit)
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
        self.ip_setString(string.into())
    }

    /// Sets the language of the text to be tokenized.
    pub fn set_language<L>(&self, language: L)
    where
        L: Into<NLLanguage>,
    {
        self.im_setLanguage(language.into())
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
        self.im_enumerateTokensInRange_usingBlock(range, block)
    }

    /// Tokenizes the string within the provided range.
    pub fn tokens_for_range<T>(&self, range: NSRange) -> NSArray<T>
    where
        T: INSValue,
    {
        self.im_tokensForRange(range)
    }

    /// Finds the range of the token at the given index.
    pub fn token_range_at(&self, index: UInt) -> NSRange {
        self.im_tokenRangeAtIndex(index)
    }

    /// Finds the entire range of all tokens contained completely or partially within the specified range.
    pub fn token_range_for(&self, range: Range<usize>) -> NSRange {
        self.im_tokenRangeForRange(range.into())
    }
}

impl PNSObject for NLTokenizer {
    fn im_class<'a>() -> &'a Class {
        class!(NLTokenizer)
    }

    fn im_isEqual(&self, object: &Self) -> bool {
        unsafe { msg_send![self.obj, isEqual: object] }
    }

    fn ip_hash(&self) -> UInt {
        unsafe { msg_send![self.obj, hash] }
    }

    fn im_isKindOfClass(&self, aClass: Class) -> bool {
        unsafe { msg_send![self.obj, isKindOfClass: aClass] }
    }

    fn im_isMemberOfClass(&self, aClass: Class) -> bool {
        unsafe { msg_send![self.obj, isMemberOfClass: aClass] }
    }

    fn im_respondsToSelector(&self, aSelector: objc::runtime::Sel) -> bool {
        unsafe { msg_send![self.obj, respondsToSelector: aSelector] }
    }

    fn im_conformsToProtocol(&self, aProtocol: objc::runtime::Protocol) -> bool {
        unsafe { msg_send![self.obj, conformsToProtocol: aProtocol] }
    }

    fn ip_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.obj, description]) }
    }

    fn ip_debugDescription(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.obj, debugDescription]) }
    }

    fn im_performSelector(&self, aSelector: objc::runtime::Sel) -> id {
        unsafe { msg_send![self.obj, performSelector: aSelector] }
    }

    fn im_performSelector_withObject(&self, aSelector: objc::runtime::Sel, withObject: id) -> id {
        unsafe { msg_send![self.obj, performSelector: aSelector withObject: withObject] }
    }

    fn im_isProxy(&self) -> bool {
        unsafe { msg_send![self.obj, isProxy] }
    }
}

impl INLTokenizer for NLTokenizer {
    /* Creating a Tokenizer
     */

    fn im_initWithUnit(unit: NLTokenUnit) -> Self {
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

    fn ip_setString(&self, string: NSString) {
        unsafe { msg_send![self.obj, setString: string] }
    }

    fn im_setLanguage(&self, language: NLLanguage) {
        unsafe { msg_send![self.obj, setLanguage: language] }
    }

    fn ip_unit(&self) -> NLTokenUnit {
        unsafe { msg_send![self.obj, unit] }
    }

    fn im_enumerateTokensInRange_usingBlock(
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

    fn im_tokensForRange<T>(&self, range: NSRange) -> NSArray<T>
    where
        T: INSValue,
    {
        unsafe { NSArray::from_id(msg_send![self.obj, tokensForRange: range]) }
    }

    fn im_tokenRangeAtIndex(&self, character_index: UInt) -> NSRange {
        unsafe { msg_send![self.obj, tokenRangeAtIndex: character_index] }
    }

    fn im_tokenRangeForRange(&self, range: NSRange) -> NSRange {
        unsafe { msg_send![self.obj, tokenRangeForRange: range] }
    }
}

impl fmt::Debug for NLTokenizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip_debugDescription())
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
