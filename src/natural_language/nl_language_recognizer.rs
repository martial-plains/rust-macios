//! The language of a body of text.

use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::{NSArray, NSDictionary, NSNumber, NSString, UInt},
    objective_c_runtime::{
        macros::object,
        traits::{FromId, PNSObject},
    },
};

use super::NLLanguage;

object! {
    /// The language of a body of text.
    unsafe pub struct NLLanguageRecognizer;
}

#[interface_impl(NSObject)]
impl NLLanguageRecognizer {
    /* Creating a Recognizer
     */

    /// Creates a recognizer that you can customize.
    #[method]
    pub fn init(&mut self) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), init]) }
    }

    /* Determining the Language
     */

    /// Finds the most likely language of a piece of text.
    #[method]
    pub fn dominant_language_for_string(&self, string: &NSString) -> NLLanguage {
        unsafe {
            NLLanguage::from_id(msg_send![
                Self::m_class(),
                dominantLanguageForString: string.m_self()
            ])
        }
    }

    /// Analyzes the piece of text to determine its dominant language.
    #[method]
    pub fn process_string(&mut self, string: &NSString) {
        unsafe { msg_send![self.m_self(), processString: string.m_self()] }
    }

    /// The most likely language for the processed text.
    #[method]
    pub fn dominant_language(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), dominantLanguage]) }
    }

    /// Generates the probabilities of possible languages for the processed text.
    #[method]
    pub fn language_hypotheses_with_maximum(
        &self,
        max_hypotheses: UInt,
    ) -> NSDictionary<NLLanguage, NSNumber> {
        unsafe {
            NSDictionary::from_id(msg_send![
                self.m_self(),
                languageHypothesesWithMaximum: max_hypotheses
            ])
        }
    }

    /// Resets the recognizer to its initial state.
    #[method]
    pub fn reset(&mut self) {
        unsafe { msg_send![self.m_self(), reset] }
    }

    /* Guiding the Recognizer
     */

    /// A dictionary that maps languages to their probabilities in the language identification process.
    #[property]
    pub fn language_hints(&self) -> NSDictionary<NLLanguage, NSNumber> {
        unsafe { NSDictionary::from_id(msg_send![self.m_self(), languageHints]) }
    }

    /// Sets a dictionary that maps languages to their probabilities in the language identification process.
    #[property]
    pub fn set_language_hints(&mut self, language_hints: &NSDictionary<NLLanguage, NSNumber>) {
        unsafe { msg_send![self.m_self(), setLanguageHints: language_hints.m_self()] }
    }

    /// Limits the set of possible languages that the recognizer will return.
    #[property]
    pub fn language_constraints(&self) -> NSArray<NLLanguage> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), languageConstraints]) }
    }

    /// Sets the limits  of the set of possible languages that the recognizer will return.
    #[property]
    pub fn set_language_constraints(&mut self, language_constraints: &NSArray<NLLanguage>) {
        unsafe { msg_send![self.m_self(), setLanguageConstraints: language_constraints.m_self()] }
    }
}

impl Default for NLLanguageRecognizer {
    fn default() -> Self {
        Self::m_new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        foundation::macros::{nsarray, nsdictionary},
        natural_language::English,
    };

    use super::*;

    #[test]
    fn test_dominant_language() {
        let mut lr = NLLanguageRecognizer::default();
        lr.process_string(&"This is a test.".into());
        let lang = lr.dominant_language();
        assert_eq!(lang, "en");
    }

    #[test]
    fn test_lang_hints() {
        let mut lr = NLLanguageRecognizer::default();
        lr.process_string(&"This is a test.".into());
        lr.set_language_hints(&nsdictionary!(
            unsafe {English.clone()} => NSNumber::from(1.0),
        ));
        let lang_hints = lr.language_hints();

        assert_eq!(lang_hints.count(), 1);
        assert_eq!(
            lang_hints.object_for_key(unsafe { English.clone() }),
            Some(NSNumber::from(1.0))
        );

        lr.set_language_hints(&nsdictionary!(
            unsafe {English.clone()} => NSNumber::from(1.0),
            unsafe {English.clone()} => NSNumber::from(2.0),
        ));
    }

    #[test]
    fn test_lang_constraints() {
        let mut lr = NLLanguageRecognizer::default();
        lr.process_string(&"This is a test.".into());
        lr.set_language_constraints(&nsarray!(unsafe { English.clone() }));
        let lang_constraints = lr.language_constraints();

        assert_eq!(lang_constraints.count(), 1);
    }

    #[test]
    fn test_reset() {
        let mut lr = NLLanguageRecognizer::default();
        lr.process_string(&"This is a test.".into());
        let mut lang = lr.dominant_language();
        assert_eq!(lang, "en");
        lr.reset();
        lr.process_string(&"Det här är ett test".into());
        lang = lr.dominant_language();
        assert_eq!(lang, "sv");
    }

    #[test]
    fn test_lang_hypotheses() {
        let mut lr = NLLanguageRecognizer::default();
        lr.process_string(&"This is a test.".into());
        let lang_hypotheses = lr.language_hypotheses_with_maximum(1);
        assert_eq!(lang_hypotheses.count(), 1);
    }

    #[test]
    fn test_dominant_language_for_string() {
        let mut lr = NLLanguageRecognizer::default();
        lr.process_string(&"This is a test.".into());
        let lang = lr.dominant_language_for_string(&"This is a test.".into());
        assert_eq!(lang, "en");
    }
}
