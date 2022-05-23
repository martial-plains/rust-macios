//! The language of a body of text.

use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    foundation::{Dictionary, NSNumber, String, UInt},
    id,
    objective_c_runtime::traits::NSObject,
};

use super::{traits::NLLanguageRecognizer as t_NLLanguageRecognizer, NLLanguage};

/// The language of a body of text.
pub struct NLLanguageRecognizer {
    /// The underlying Objective-C object.
    pub obj: Id<Object>,
}

impl NSObject for NLLanguageRecognizer {
    fn init() -> Self {
        unsafe {
            let cls = class!(NLLanguageRecognizer);
            let obj = msg_send![cls, new];
            Self { obj }
        }
    }

    fn to_id(mut self) -> id {
        &mut *self.obj
    }

    fn from_id(_obj: id) -> Self {
        todo!()
    }

    fn description(&self) -> String {
        let obj: id = unsafe { msg_send![&*self.obj, description] };
        String::from_id(obj)
    }

    fn debug_description(&self) -> String {
        let obj: id = unsafe { msg_send![&*self.obj, debugDescription] };
        String::from_id(obj)
    }

    fn retain(&self) -> Self {
        let obj: id = unsafe { msg_send![&*self.obj, retain] };
        Self::from_id(obj)
    }
}

impl t_NLLanguageRecognizer for NLLanguageRecognizer {
    /// The most likely language for the processed text.
    fn dominant_language(&self) -> String {
        unsafe { msg_send![self.obj, dominantLanguage] }
    }

    /// Finds the most likely language of a piece of text.
    fn dominant_language_for<T>(&self, string: T) -> NLLanguage
    where
        T: Into<String>,
    {
        unsafe { msg_send![self.obj, dominantLanguageForString: string.into()] }
    }

    /// Analyzes the piece of text to determine its dominant language.
    fn process_string<T>(&self, string: T)
    where
        T: Into<String>,
    {
        unsafe { msg_send![self.obj, processString: string.into()] }
    }

    /// Generates the probabilities of possible languages for the processed text.
    fn language_hypotheses(&self, max_hypotheses: UInt) -> Dictionary<NLLanguage, NSNumber> {
        unsafe { msg_send![&*self.obj, languageHypothesesWithMaximum: max_hypotheses] }
    }

    /// Resets the recognizer to its initial state.
    fn reset(&self) {
        unsafe { msg_send![self.obj, reset] }
    }

    /* Guiding the Recognizer
     */

    /// A dictionary that maps languages to their probabilities in the language identification process.
    fn language_hints(&self) -> Dictionary<NLLanguage, NSNumber> {
        unsafe { msg_send![&*self.obj, languageHints] }
    }

    /// Sets a dictionary that maps languages to their probabilities in the language identification process.
    fn set_language_hints(&self, language_hints: Dictionary<NLLanguage, NSNumber>) {
        unsafe { msg_send![self.obj, setLanguageHints: language_hints] }
    }

    /// Limits the set of possible languages that the recognizer will return.
    fn language_constraints(&self) -> Dictionary<NLLanguage, NSNumber> {
        unsafe { msg_send![&*self.obj, languageConstraints] }
    }

    /// Sets the limits  of the set of possible languages that the recognizer will return.
    fn set_language_constraints(&self, language_constraints: Dictionary<NLLanguage, NSNumber>) {
        unsafe { msg_send![self.obj, setLanguageConstraints: language_constraints] }
    }
}

impl fmt::Debug for NLLanguageRecognizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.debug_description())
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
