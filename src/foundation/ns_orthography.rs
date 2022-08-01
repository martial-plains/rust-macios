use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::objective_c_runtime::{
    macros::object,
    traits::{FromId, PNSObject},
};

use super::{NSArray, NSCoder, NSDictionary, NSString};

object! {
    /// A description of the linguistic content of natural language text, typically used for spelling and grammar checking.
    unsafe pub struct NSOrthography;
}

#[interface_impl(NSObject)]
impl NSOrthography {
    /* Creating Orthography Objects
     */

    /// Creates and returns an orthography object with the default language map for the specified language.
    #[method]
    pub fn default_orthography_for_language(language: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                Self::m_class(),
                defaultOrthographyForLanguage: language
            ])
        }
    }

    /// Creates an orthography object with the specified dominant script and language map.
    #[method]
    pub fn init_with_dominant_script_language_map(
        &mut self,
        script: NSString,
        map: NSDictionary<NSString, NSArray<NSString>>,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![self.m_self(), initWithDominantScript: script languageMap: map])
        }
    }

    /// Creates and returns an orthography object with the specified dominant script and language map.
    #[method]
    pub fn orthography_with_dominant_script(
        script: NSString,
        map: NSDictionary<NSString, NSArray<NSString>>,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(
                msg_send![Self::m_class(), orthographyWithDominantScript: script languageMap: map],
            )
        }
    }

    /* Determining Correspondences Between Languages and Scripts
     */

    /// A dictionary that maps script tags to arrays of language tags.
    #[property]
    pub fn language_map(&self) -> NSDictionary<NSString, NSArray<NSString>> {
        unsafe { NSDictionary::from_id(msg_send![self.m_self(), languageMap]) }
    }

    /// The first language in the list of languages for the dominant script.
    #[property]
    pub fn dominant_language(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), dominantLanguage]) }
    }

    /// The dominant script for the text.
    #[property]
    pub fn dominant_script(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), dominantScript]) }
    }

    /// Returns the dominant language for the specified script.
    #[method]
    pub fn dominant_language_for_script(&self, script: NSString) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), dominantLanguageForScript: script]) }
    }

    /// Returns the list of languages for the specified script.
    #[method]
    pub fn languages_for_script(&self, script: NSString) -> NSArray<NSString> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), languagesForScript: script]) }
    }

    /// The scripts appearing as keys in the language map.
    #[property]
    pub fn all_scripts(&self) -> NSArray<NSString> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), allScripts]) }
    }

    /// The languages appearing in values of the language map.
    #[property]
    pub fn all_languages(&self) -> NSArray<NSString> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), allLanguages]) }
    }

    /// Initialize with [`NSCoder`]
    #[method]
    pub fn init_with_coder(&mut self, coder: NSCoder) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithCoder: coder]) }
    }
}

impl Default for NSOrthography {
    fn default() -> Self {
        NSOrthography::m_new()
    }
}
