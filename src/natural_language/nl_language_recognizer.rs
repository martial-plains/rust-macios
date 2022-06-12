//! The language of a body of text.

use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use objc::{
    class, msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::{NSArray, NSDictionary, NSNumber, NSString, UInt},
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject, ToId},
    },
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
        Self::tm_dominant_language_for_string(self, string.into())
    }

    /// Analyzes the piece of text to determine its dominant language.
    pub fn process_str<T>(&mut self, string: T)
    where
        T: Into<NSString>,
    {
        Self::im_process_string(self, string.into())
    }

    /// Finds the range of the token at the given index.
    pub fn dominant_lang(&self) -> NLLanguage {
        Self::ip_dominant_language(self)
    }

    /// Generates the probabilities of possible languages for the processed text.
    pub fn lang_hypotheses_with_max(&self, maximum: UInt) -> NSDictionary<NLLanguage, NSNumber> {
        Self::im_language_hypotheses_with_maximum(self, maximum)
    }

    /// Resets the recognizer to its initial state.
    pub fn reset(&self) {
        Self::im_reset(self)
    }

    /// A dictionary that maps languages to their probabilities in the language identification process.
    pub fn lang_hints(&self) -> NSDictionary<NLLanguage, NSNumber> {
        Self::ip_language_hints(self)
    }

    /// Sets a dictionary that maps languages to their probabilities in the language identification process.
    pub fn set_lang_hints(&self, language_hints: NSDictionary<NLLanguage, NSNumber>) {
        Self::ip_set_language_hints(self, language_hints)
    }

    /// Limits the set of possible languages that the recognizer will return.
    pub fn lang_constraints(&self) -> NSArray<NLLanguage> {
        Self::ip_language_constraints(self)
    }

    /// Sets the limits  of the set of possible languages that the recognizer will return.
    pub fn set_lang_constraints(&self, language_constraints: NSArray<NLLanguage>) {
        Self::ip_set_language_constraints(self, language_constraints)
    }
}

impl Default for NLLanguageRecognizer {
    fn default() -> Self {
        Self::new()
    }
}

impl PNSObject for NLLanguageRecognizer {
    fn im_class<'a>() -> &'a Class {
        class!(NLLanguageRecognizer)
    }

    fn ip_superclass<'a>() -> Option<&'a Class> {
        unsafe { msg_send![Self::im_class(), superclass] }
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

impl INLLanguageRecognizer for NLLanguageRecognizer {
    fn im_init() -> Self {
        unsafe {
            let obj: id = msg_send![Self::im_class(), alloc];
            let obj: id = msg_send![obj, init];
            Self::from_id(obj)
        }
    }

    fn im_process_string(&mut self, string: NSString) {
        unsafe { msg_send![self.obj, processString: string] }
    }

    fn ip_dominant_language(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.obj, dominantLanguage]) }
    }

    fn im_language_hypotheses_with_maximum(
        &self,
        max_hypotheses: UInt,
    ) -> NSDictionary<NLLanguage, NSNumber> {
        unsafe {
            NSDictionary::from_id(msg_send![
                self.obj,
                languageHypothesesWithMaximum: max_hypotheses
            ])
        }
    }

    fn im_reset(&self) {
        unsafe { msg_send![self.obj, reset] }
    }

    fn ip_language_hints(&self) -> NSDictionary<NLLanguage, NSNumber> {
        unsafe { NSDictionary::from_id(msg_send![self.obj, languageHints]) }
    }

    fn ip_set_language_hints(&self, language_hints: NSDictionary<NLLanguage, NSNumber>) {
        unsafe { msg_send![self.obj, setLanguageHints: language_hints] }
    }

    fn ip_language_constraints(&self) -> NSArray<NLLanguage> {
        unsafe { NSArray::from_id(msg_send![self.obj, languageConstraints]) }
    }

    fn ip_set_language_constraints(&self, language_constraints: NSArray<NLLanguage>) {
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
        write!(f, "{:?}", self.ip_debug_description())
    }
}

impl fmt::Display for NLLanguageRecognizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip_description())
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

#[cfg(test)]
mod tests {
    use crate::{
        foundation::macros::{ns_array, ns_dictionary},
        natural_language::English,
    };

    use super::*;

    #[test]
    fn test_dominant_language() {
        let mut lr = NLLanguageRecognizer::new();
        lr.process_str("This is a test.");
        let lang = lr.ip_dominant_language();
        assert_eq!(lang, "en");
    }

    #[test]
    fn test_lang_hints() {
        let mut lr = NLLanguageRecognizer::new();
        lr.process_str("This is a test.");
        lr.set_lang_hints(ns_dictionary!(
            unsafe {English.clone()} => NSNumber::from(1.0),
        ));
        let lang_hints = lr.ip_language_hints();

        assert_eq!(lang_hints.count(), 1);
        assert_eq!(
            lang_hints.object_for_key(unsafe { English.clone() }),
            NSNumber::from(1.0)
        );

        lr.set_lang_hints(ns_dictionary!(
            unsafe {English.clone()} => NSNumber::from(1.0),
            unsafe {English.clone()} => NSNumber::from(2.0),
        ));
    }

    #[test]
    fn test_lang_constraints() {
        let mut lr = NLLanguageRecognizer::new();
        lr.process_str("This is a test.");
        lr.set_lang_constraints(ns_array!(unsafe { English.clone() }));
        let lang_constraints = lr.ip_language_constraints();

        assert_eq!(lang_constraints.count(), 1);
    }

    #[test]
    fn test_reset() {
        let mut lr = NLLanguageRecognizer::new();
        lr.process_str("This is a test.");
        let mut lang = lr.ip_dominant_language();
        assert_eq!(lang, "en");
        lr.reset();
        lr.process_str("Det här är ett test");
        lang = lr.ip_dominant_language();
        assert_eq!(lang, "sv");
    }

    #[test]
    fn test_lang_hypotheses() {
        let mut lr = NLLanguageRecognizer::new();
        lr.process_str("This is a test.");
        let lang_hypotheses = lr.im_language_hypotheses_with_maximum(1);
        assert_eq!(lang_hypotheses.count(), 1);
    }

    #[test]
    fn test_dominant_language_for_string() {
        let mut lr = NLLanguageRecognizer::new();
        lr.process_str("This is a test.");
        let lang = lr.dominant_lang_for_str("This is a test.");
        assert_eq!(lang, "en");
    }
}
