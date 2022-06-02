//! The language of a body of text.

use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use objc::{
    class, msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::{NSArray, NSDictionary, NSNumber, NSString, UInt},
    id,
    objective_c_runtime::traits::{FromId, PNSObject, ToId},
};

use super::{traits::INLLanguageRecognizer, NLLanguage};

/// The language of a body of text.
pub struct NLLanguageRecognizer {
    /// The underlying Objective-C object.
    pub obj: Id<Object>,
}

impl NLLanguageRecognizer {
    /// Creates a recognizer that you can customize.
    pub fn new() -> Self {
        Self::im_init()
    }

    /// Finds the most likely language of a piece of text.
    pub fn dominant_lang_for_str<T>(&self, string: T) -> NLLanguage
    where
        T: Into<NSString>,
    {
        Self::tm_dominantLanguageForString(self, string)
    }

    /// Analyzes the piece of text to determine its dominant language.
    pub fn process_str<T>(&mut self, string: T)
    where
        T: Into<NSString>,
    {
        Self::im_processString(self, string)
    }

    /// Finds the range of the token at the given index.
    pub fn dominant_lang(&self) -> NLLanguage {
        Self::ip_dominantLanguage(self)
    }

    /// Generates the probabilities of possible languages for the processed text.
    pub fn lang_hypotheses_with_max(&self, maximum: UInt) -> NSDictionary<NLLanguage, NSNumber> {
        Self::im_languageHypothesesWithMaximum(self, maximum)
    }

    /// Resets the recognizer to its initial state.
    pub fn reset(&self) {
        Self::im_reset(self)
    }

    /// A dictionary that maps languages to their probabilities in the language identification process.
    pub fn lang_hints(&self) -> NSDictionary<NLLanguage, NSNumber> {
        Self::ip_languageHints(self)
    }

    /// Sets a dictionary that maps languages to their probabilities in the language identification process.
    pub fn set_lang_hints(&self, language_hints: NSDictionary<NLLanguage, NSNumber>) {
        Self::ip_setLanguageHints(self, language_hints)
    }

    /// Limits the set of possible languages that the recognizer will return.
    pub fn lang_constraints(&self) -> NSArray<NLLanguage> {
        Self::ip_languageConstraints(self)
    }

    /// Sets the limits  of the set of possible languages that the recognizer will return.
    pub fn set_lang_constraints(&self, language_constraints: NSArray<NLLanguage>) {
        Self::ip_setLanguageConstraints(self, language_constraints)
    }
}

impl Default for NLLanguageRecognizer {
    fn default() -> Self {
        Self::new()
    }
}

impl PNSObject for NLLanguageRecognizer {
    fn class<'a>() -> &'a Class {
        class!(NLLanguageRecognizer)
    }

    fn superclass<'a>() -> &'a Class {
        unsafe { msg_send![Self::class(), superclass] }
    }

    fn isEqual(&self, object: &Self) -> bool {
        unsafe { msg_send![self.obj, isEqual: object] }
    }

    fn hash(&self) -> UInt {
        unsafe { msg_send![self.obj, hash] }
    }

    fn isKindOfClass(&self, aClass: Class) -> bool {
        unsafe { msg_send![self.obj, isKindOfClass: aClass] }
    }

    fn isMemberOfClass(&self, aClass: Class) -> bool {
        unsafe { msg_send![self.obj, isMemberOfClass: aClass] }
    }

    fn respondsToSelector(&self, aSelector: objc::runtime::Sel) -> bool {
        unsafe { msg_send![self.obj, respondsToSelector: aSelector] }
    }

    fn conformsToProtocol(&self, aProtocol: objc::runtime::Protocol) -> bool {
        unsafe { msg_send![self.obj, conformsToProtocol: aProtocol] }
    }

    fn description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.obj, description]) }
    }

    fn debugDescription(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.obj, debugDescription]) }
    }

    fn performSelector(&self, aSelector: objc::runtime::Sel) -> id {
        unsafe { msg_send![self.obj, performSelector: aSelector] }
    }

    fn performSelector_withObject(&self, aSelector: objc::runtime::Sel, withObject: id) -> id {
        unsafe { msg_send![self.obj, performSelector: aSelector withObject: withObject] }
    }

    fn isProxy(&self) -> bool {
        unsafe { msg_send![self.obj, isProxy] }
    }
}

impl INLLanguageRecognizer for NLLanguageRecognizer {
    fn im_init() -> Self {
        unsafe {
            let obj: id = msg_send![Self::class(), alloc];
            let obj: id = msg_send![obj, init];
            Self::from_id(obj)
        }
    }

    fn tm_dominantLanguageForString<T>(&self, string: T) -> NLLanguage
    where
        T: Into<NSString>,
    {
        unsafe {
            NLLanguage::from_id(msg_send![self.obj, dominantLanguageForString: string.into()])
        }
    }

    fn im_processString<T>(&mut self, string: T)
    where
        T: Into<NSString>,
    {
        unsafe { msg_send![self.obj, processString: string.into()] }
    }

    fn ip_dominantLanguage(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.obj, dominantLanguage]) }
    }

    fn im_languageHypothesesWithMaximum(
        &self,
        max_hypotheses: UInt,
    ) -> NSDictionary<NLLanguage, NSNumber> {
        unsafe { msg_send![&*self.obj, languageHypothesesWithMaximum: max_hypotheses] }
    }

    fn im_reset(&self) {
        unsafe { msg_send![self.obj, reset] }
    }

    fn ip_languageHints(&self) -> NSDictionary<NLLanguage, NSNumber> {
        unsafe { msg_send![&*self.obj, languageHints] }
    }

    fn ip_setLanguageHints(&self, language_hints: NSDictionary<NLLanguage, NSNumber>) {
        unsafe { msg_send![self.obj, setLanguageHints: language_hints] }
    }

    fn ip_languageConstraints(&self) -> NSArray<NLLanguage> {
        unsafe { msg_send![&*self.obj, languageConstraints] }
    }

    fn ip_setLanguageConstraints(&self, language_constraints: NSArray<NLLanguage>) {
        unsafe { msg_send![self.obj, setLanguageConstraints: language_constraints] }
    }
}

impl ToId for NLLanguageRecognizer {
    fn to_id(mut self) -> id {
        &mut *self.obj
    }
}

impl FromId for NLLanguageRecognizer {
    unsafe fn from_id(obj: id) -> Self {
        Self {
            obj: Id::from_ptr(obj),
        }
    }
}

impl fmt::Debug for NLLanguageRecognizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.debugDescription())
    }
}

impl fmt::Display for NLLanguageRecognizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Deref for NLLanguageRecognizer {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.obj
    }
}

impl DerefMut for NLLanguageRecognizer {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.obj
    }
}
