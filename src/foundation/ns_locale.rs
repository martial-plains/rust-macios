use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    foundation::{traits::t_NSLocale, Array, String},
    id,
    objective_c_runtime::traits::t_NSObject,
};

use super::NSLocaleKey;

/// The directions that a language may take across a page of text.
#[repr(usize)]
#[derive(Debug)]
pub enum LanguageDirection {
    /// The direction of the language is unknown.
    Unknown = 0,
    /// The language direction is from left to right.
    LeftToRight = 1,
    /// The language direction is from right to left.
    RightToLeft = 2,
    /// The language direction is from top to bottom.
    TopToBottom = 3,
    /// The language direction is from bottom to top.
    BottomToTop = 4,
}

/// The `Locale` class provides information about the user’s locale and formatting preferences.
pub struct Locale {
    /// The raw pointer to the Objective-C object.
    pub obj: Id<Object>,
}

impl t_NSLocale for Locale {
    fn initWithLocaleIdentifier<S>(locale_identifier: S) -> Self
    where
        S: Into<String>,
    {
        unsafe {
            let class: Locale = msg_send![class!(NSLocale), alloc];
            let obj = msg_send![class, initWithLocaleIdentifier: locale_identifier.into()];
            Locale { obj }
        }
    }

    fn autoUpdatingCurrent(&self) -> Locale {
        let class = class!(NSLocale);
        unsafe { msg_send![class, autoupdatingCurrent] }
    }

    fn current() -> Locale {
        let class = class!(NSLocale);
        unsafe { msg_send![class, currentLocale] }
    }

    fn system() -> Locale {
        let class = class!(NSLocale);
        unsafe { msg_send![class, systemLocale] }
    }

    fn availableLocaleIdentifiers() -> Array<String> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, availableLocaleIdentifiers] }
    }

    fn isoCountryCodes() -> Array<String> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, ISOCountryCodes] }
    }

    fn isoLanguageCodes() -> Array<String> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, ISOLanguageCodes] }
    }

    fn isoCurrencyCodes() -> Array<String> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, ISOCurrencyCodes] }
    }

    fn commonIsocurrencyCodes() -> Array<String> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, commonISOCurrencyCodes] }
    }

    fn localeIdentifier(&self) -> String {
        unsafe { msg_send![self.obj, localeIdentifier] }
    }

    fn countryCode(&self) -> String {
        unsafe { msg_send![self.obj, countryCode] }
    }

    fn languageCode(&self) -> String {
        unsafe { msg_send![self.obj, languageCode] }
    }

    fn scriptCode(&self) -> String {
        unsafe { msg_send![self.obj, scriptCode] }
    }

    fn variantCode(&self) -> String {
        unsafe { msg_send![self.obj, variantCode] }
    }

    fn collationIdentifier(&self) -> String {
        unsafe { msg_send![self.obj, collationIdentifier] }
    }

    fn collatorIdentifier(&self) -> String {
        unsafe { msg_send![self.obj, collatorIdentifier] }
    }

    fn usesMetricSystem(&self) -> bool {
        unsafe { msg_send![self.obj, usesMetricSystem] }
    }

    fn decimalSeparator(&self) -> String {
        unsafe { msg_send![self.obj, decimalSeparator] }
    }

    fn groupingSeparator(&self) -> String {
        unsafe { msg_send![self.obj, groupingSeparator] }
    }

    fn currencyCode(&self) -> String {
        unsafe { msg_send![self.obj, currencyCode] }
    }

    fn currencySymbol(&self) -> String {
        unsafe { msg_send![self.obj, currencySymbol] }
    }

    fn calendarIdentifier(&self) -> String {
        unsafe { msg_send![self.obj, calendarIdentifier] }
    }

    fn quotationBeginDelimiter(&self) -> String {
        unsafe { msg_send![self.obj, quotationBeginDelimiter] }
    }

    fn quotationEndDelimiter(&self) -> String {
        unsafe { msg_send![self.obj, quotationEndDelimiter] }
    }

    fn alternateQuotationBeginDelimiter(&self) -> String {
        unsafe { msg_send![self.obj, alternateQuotationBeginDelimiter] }
    }

    fn alternateQuotationEndDelimiter(&self) -> String {
        unsafe { msg_send![self.obj, alternateQuotationEndDelimiter] }
    }

    fn objectForKey(&self, key: NSLocaleKey) -> Option<Object> {
        unsafe { msg_send![self.obj, objectForKey: key] }
    }

    fn displayNameForKeyValue<T>(&self, key: NSLocaleKey, value: T) -> Option<String>
    where
        T: Into<String>,
    {
        let result: id = unsafe { msg_send![self.obj, displayNameForKey: key value: value.into()] };

        if result.is_null() {
            None
        } else {
            Some(unsafe { String::fromId(result) })
        }
    }

    fn preferredLanguages() -> Array<String> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, preferredLanguages] }
    }

    fn characterDirectionForLanguage<S>(&self, iso_language_code: S) -> LanguageDirection
    where
        S: Into<String>,
    {
        let class = class!(NSLocale);
        unsafe { msg_send![class, characterDirectionForLanguage: iso_language_code.into()] }
    }

    fn lineDirectionForLanguage<S>(&self, iso_language_code: S) -> LanguageDirection
    where
        S: Into<String>,
    {
        let class = class!(NSLocale);
        unsafe { msg_send![class, lineDirectionForLanguage: iso_language_code.into()] }
    }
}

impl t_NSObject for Locale {
    fn init() -> Self {
        todo!()
    }

    fn toId(mut self) -> id {
        &mut *self.obj
    }

    unsafe fn fromId(_obj: id) -> Self {
        todo!()
    }

    fn description(&self) -> String {
        unsafe {
            let description: id = msg_send![self.obj, description];
            String::fromId(description)
        }
    }

    fn debugDescription(&self) -> String {
        unsafe {
            let description: id = msg_send![self.obj, debugDescription];
            String::fromId(description)
        }
    }

    fn retain(&self) -> Self {
        unsafe { msg_send![self.obj, retain] }
    }
}

impl Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Debug for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Locale: {}", self.debugDescription())
    }
}

impl Clone for Locale {
    fn clone(&self) -> Self {
        Locale {
            obj: { unsafe { msg_send![self.obj, retain] } },
        }
    }
}

impl Deref for Locale {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.obj
    }
}

impl DerefMut for Locale {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.obj
    }
}

impl From<Locale> for id {
    /// Converts the `Locale` into an `Object`.
    fn from(mut val: Locale) -> Self {
        &mut *val.obj
    }
}

impl From<id> for Locale {
    /// Converts the `Object` into a `Locale`.
    fn from(val: id) -> Self {
        Locale {
            obj: unsafe { msg_send![val, retain] },
        }
    }
}
