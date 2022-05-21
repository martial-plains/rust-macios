//! The language of a body of text.

use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::foundation::String;

use super::NLLanguage;

/// The language of a body of text.
pub struct NLLanguageRecognizer {
    /// The underlying Objective-C object.
    pub obj: Id<Object>,
}

impl NLLanguageRecognizer {
    /// Creates a recognizer that you can customize.
    pub fn init() -> Self {
        unsafe {
            let cls = class!(NLLanguageRecognizer);
            let obj: *mut Object = msg_send![cls, new];
            let obj = msg_send![obj, init];
            Self { obj }
        }
    }

    /// Finds the most likely language of a piece of text.
    pub fn dominant_language<T>(&self, string: T) -> NLLanguage
    where
        T: Into<String>,
    {
        unsafe { msg_send![self.obj, dominantLanguageForString: string] }
    }

    /// Analyzes the piece of text to determine its dominant language.
    pub fn process_string<T>(&self, string: T)
    where
        T: Into<String>,
    {
        unsafe { msg_send![self.obj, processString: string] }
    }

    /// Resets the recognizer to its initial state.
    pub fn reset(&self) {
        unsafe { msg_send![self.obj, reset] }
    }
}

impl fmt::Debug for NLLanguageRecognizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description: String = unsafe { msg_send![self.obj, debugDescription] };
        write!(f, "{:?}", description)
    }
}

impl fmt::Display for NLLanguageRecognizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description: String = unsafe { msg_send![self.obj, description] };
        write!(f, "{}", description)
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
