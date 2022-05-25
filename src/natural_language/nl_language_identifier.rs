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
    objective_c_runtime::traits::t_NSObject,
};

use super::{traits::t_NLLanguageRecognizer, NLLanguage};

/// The language of a body of text.
pub struct NLLanguageRecognizer {
    /// The underlying Objective-C object.
    pub obj: Id<Object>,
}

impl t_NSObject for NLLanguageRecognizer {
    fn init() -> Self {
        unsafe {
            let cls = class!(NLLanguageRecognizer);
            let obj = msg_send![cls, new];
            Self { obj }
        }
    }

    fn toId(mut self) -> id {
        &mut *self.obj
    }

    unsafe fn fromId(_obj: id) -> Self {
        todo!()
    }

    fn description(&self) -> NSString {
        let obj: id = unsafe { msg_send![&*self.obj, description] };
        unsafe { NSString::fromId(obj) }
    }

    fn debugDescription(&self) -> NSString {
        let obj: id = unsafe { msg_send![&*self.obj, debugDescription] };
        unsafe { NSString::fromId(obj) }
    }

    fn retain(&self) -> Self {
        let obj: id = unsafe { msg_send![&*self.obj, retain] };
        unsafe { Self::fromId(obj) }
    }
}

impl t_NLLanguageRecognizer for NLLanguageRecognizer {
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
