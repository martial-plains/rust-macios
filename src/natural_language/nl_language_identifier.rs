//! The language of a body of text.

use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    foundation::{NSDictionary, NSNumber, NSString, UInt},
    id,
    objective_c_runtime::traits::{FromId, PNSObject, ToId},
};

use super::{traits::INLLanguageRecognizer, NLLanguage};

/// The language of a body of text.
pub struct NLLanguageRecognizer {
    /// The underlying Objective-C object.
    pub obj: Id<Object>,
}

impl PNSObject for NLLanguageRecognizer {
    fn class<'a>() -> &'a objc::runtime::Class {
        class!(NLLanguageRecognizer)
    }

    fn superclass<'a>() -> &'a objc::runtime::Class {
        unsafe { msg_send![Self::class(), superclass] }
    }

    fn isEqual(&self, object: &Self) -> bool {
        unsafe { msg_send![self.obj, isEqual: object] }
    }

    fn hash(&self) -> UInt {
        unsafe { msg_send![self.obj, hash] }
    }

    fn isKindOfClass(&self, aClass: objc::runtime::Class) -> bool {
        unsafe { msg_send![self.obj, isKindOfClass: aClass] }
    }

    fn isMemberOfClass(&self, aClass: objc::runtime::Class) -> bool {
        unsafe { msg_send![self.obj, isMemberOfClass: aClass] }
    }

    fn respondsToSelector(&self, aSelector: objc::runtime::Sel) -> bool {
        unsafe { msg_send![self.obj, respondsToSelector: aSelector] }
    }

    fn conformsToProtocol(&self, aProtocol: objc::runtime::Protocol) -> bool {
        unsafe { msg_send![self.obj, conformsToProtocol: aProtocol] }
    }

    fn description(&self) -> NSString {
        unsafe { msg_send![self.obj, description] }
    }

    fn debugDescription(&self) -> NSString {
        unsafe { msg_send![self.obj, debugDescription] }
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
    /// The most likely language for the processed text.
    fn dominantLanguage(&self) -> NSString {
        unsafe { msg_send![self.obj, dominantLanguage] }
    }

    /// Finds the most likely language of a piece of text.
    fn dominantLanguageFor<T>(&self, string: T) -> NLLanguage
    where
        T: Into<NSString>,
    {
        unsafe { msg_send![self.obj, dominantLanguageForString: string.into()] }
    }

    /// Analyzes the piece of text to determine its dominant language.
    fn processString<T>(&self, string: T)
    where
        T: Into<NSString>,
    {
        unsafe { msg_send![self.obj, processString: string.into()] }
    }

    /// Generates the probabilities of possible languages for the processed text.
    fn languageHypotheses(&self, max_hypotheses: UInt) -> NSDictionary<NLLanguage, NSNumber> {
        unsafe { msg_send![&*self.obj, languageHypothesesWithMaximum: max_hypotheses] }
    }

    /// Resets the recognizer to its initial state.
    fn reset(&self) {
        unsafe { msg_send![self.obj, reset] }
    }

    /* Guiding the Recognizer
     */

    /// A dictionary that maps languages to their probabilities in the language identification process.
    fn languageHints(&self) -> NSDictionary<NLLanguage, NSNumber> {
        unsafe { msg_send![&*self.obj, languageHints] }
    }

    /// Sets a dictionary that maps languages to their probabilities in the language identification process.
    fn setLanguageHints(&self, language_hints: NSDictionary<NLLanguage, NSNumber>) {
        unsafe { msg_send![self.obj, setLanguageHints: language_hints] }
    }

    /// Limits the set of possible languages that the recognizer will return.
    fn languageConstraints(&self) -> NSDictionary<NLLanguage, NSNumber> {
        unsafe { msg_send![&*self.obj, languageConstraints] }
    }

    /// Sets the limits  of the set of possible languages that the recognizer will return.
    fn setLanguageConstraints(&self, language_constraints: NSDictionary<NLLanguage, NSNumber>) {
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
