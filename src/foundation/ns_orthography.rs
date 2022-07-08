use crate::objective_c_runtime::macros::object;

use super::{traits::INSOrthography, NSArray, NSCoder, NSDictionary, NSString};

object! {
    /// A description of the linguistic content of natural language text, typically used for spelling and grammar checking.
    unsafe pub struct NSOrthography;
}

impl NSOrthography {
    /// Creates and returns an orthography object with the default language map for the specified language.
    pub fn default_orthography_for_language<S>(language: S) -> Self
    where
        S: Into<NSString>,
    {
        Self::tm_default_orthography_for_language(language.into())
    }

    /// Creates an orthography object with the specified dominant script and language map.
    pub fn init_with_dominant_script_language_map(
        &mut self,
        script: NSString,
        map: NSDictionary<NSString, NSArray<NSString>>,
    ) -> Self {
        self.im_init_with_dominant_script_language_map(script, map)
    }

    /// Creates and returns an orthography object with the specified dominant script and language map.
    pub fn orthography_with_dominant_script(
        script: NSString,
        map: NSDictionary<NSString, NSArray<NSString>>,
    ) -> Self {
        Self::tm_orthography_with_dominant_script(script, map)
    }

    /* Determining Correspondences Between Languages and Scripts
     */

    /// A dictionary that maps script tags to arrays of language tags.
    pub fn language_map(&self) -> NSDictionary<NSString, NSArray<NSString>> {
        self.ip_language_map()
    }

    /// The first language in the list of languages for the dominant script.
    pub fn dominant_language(&self) -> NSString {
        self.ip_dominant_language()
    }

    /// The dominant script for the text.
    pub fn dominant_script(&self) -> NSString {
        self.ip_dominant_script()
    }

    /// Returns the dominant language for the specified script.
    pub fn dominant_language_for_script(&self, script: NSString) -> NSString {
        self.im_dominant_language_for_script(script)
    }

    /// Returns the list of languages for the specified script.
    pub fn languages_for_script(&self, script: NSString) -> NSArray<NSString> {
        self.im_languages_for_script(script)
    }

    /// The scripts appearing as keys in the language map.
    pub fn all_scripts(&self) -> NSArray<NSString> {
        self.ip_all_scripts()
    }

    /// The languages appearing in values of the language map.
    pub fn all_languages(&self) -> NSArray<NSString> {
        self.ip_all_languages()
    }

    /// Initialize with [`NSCoder`]
    pub fn init_with_coder(&self, coder: NSCoder) -> Self {
        self.im_init_with_coder(coder)
    }
}

impl INSOrthography for NSOrthography {}
