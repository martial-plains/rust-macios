use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    foundation::{traits::t_NSLocale, NSArray, NSString},
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
pub struct NSLocale {
    /// The raw pointer to the Objective-C object.
    pub obj: Id<Object>,
}

impl t_NSLocale for NSLocale {
    fn initWithLocaleIdentifier<S>(locale_identifier: S) -> Self
    where
        S: Into<NSString>,
    {
        unsafe {
            let class: NSLocale = msg_send![class!(NSLocale), alloc];
            let obj = msg_send![class, initWithLocaleIdentifier: locale_identifier.into()];
            NSLocale { obj }
        }
    }

    fn autoUpdatingCurrent(&self) -> NSLocale {
        let class = class!(NSLocale);
        unsafe { msg_send![class, autoupdatingCurrent] }
    }

    fn current() -> NSLocale {
        let class = class!(NSLocale);
        unsafe { msg_send![class, currentLocale] }
    }

    fn system() -> NSLocale {
        let class = class!(NSLocale);
        unsafe { msg_send![class, systemLocale] }
    }

    fn availableLocaleIdentifiers() -> NSArray<NSString> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, availableLocaleIdentifiers] }
    }

    fn isoCountryCodes() -> NSArray<NSString> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, ISOCountryCodes] }
    }

    fn isoLanguageCodes() -> NSArray<NSString> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, ISOLanguageCodes] }
    }

    fn isoCurrencyCodes() -> NSArray<NSString> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, ISOCurrencyCodes] }
    }

    fn commonIsocurrencyCodes() -> NSArray<NSString> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, commonISOCurrencyCodes] }
    }

    fn localeIdentifier(&self) -> NSString {
        unsafe { msg_send![self.obj, localeIdentifier] }
    }

    fn countryCode(&self) -> NSString {
        unsafe { msg_send![self.obj, countryCode] }
    }

    fn languageCode(&self) -> NSString {
        unsafe { msg_send![self.obj, languageCode] }
    }

    fn scriptCode(&self) -> NSString {
        unsafe { msg_send![self.obj, scriptCode] }
    }

    fn variantCode(&self) -> NSString {
        unsafe { msg_send![self.obj, variantCode] }
    }

    fn collationIdentifier(&self) -> NSString {
        unsafe { msg_send![self.obj, collationIdentifier] }
    }

    fn collatorIdentifier(&self) -> NSString {
        unsafe { msg_send![self.obj, collatorIdentifier] }
    }

    fn usesMetricSystem(&self) -> bool {
        unsafe { msg_send![self.obj, usesMetricSystem] }
    }

    fn decimalSeparator(&self) -> NSString {
        unsafe { msg_send![self.obj, decimalSeparator] }
    }

    fn groupingSeparator(&self) -> NSString {
        unsafe { msg_send![self.obj, groupingSeparator] }
    }

    fn currencyCode(&self) -> NSString {
        unsafe { msg_send![self.obj, currencyCode] }
    }

    fn currencySymbol(&self) -> NSString {
        unsafe { msg_send![self.obj, currencySymbol] }
    }

    fn calendarIdentifier(&self) -> NSString {
        unsafe { msg_send![self.obj, calendarIdentifier] }
    }

    fn quotationBeginDelimiter(&self) -> NSString {
        unsafe { msg_send![self.obj, quotationBeginDelimiter] }
    }

    fn quotationEndDelimiter(&self) -> NSString {
        unsafe { msg_send![self.obj, quotationEndDelimiter] }
    }

    fn alternateQuotationBeginDelimiter(&self) -> NSString {
        unsafe { msg_send![self.obj, alternateQuotationBeginDelimiter] }
    }

    fn alternateQuotationEndDelimiter(&self) -> NSString {
        unsafe { msg_send![self.obj, alternateQuotationEndDelimiter] }
    }

    fn objectForKey(&self, key: NSLocaleKey) -> Option<Object> {
        unsafe { msg_send![self.obj, objectForKey: key] }
    }

    fn displayNameForKeyValue<T>(&self, key: NSLocaleKey, value: T) -> Option<NSString>
    where
        T: Into<NSString>,
    {
        let result: id = unsafe { msg_send![self.obj, displayNameForKey: key value: value.into()] };

        if result.is_null() {
            None
        } else {
            Some(unsafe { NSString::fromId(result) })
        }
    }

    fn preferredLanguages() -> NSArray<NSString> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, preferredLanguages] }
    }

    fn characterDirectionForLanguage<S>(&self, iso_language_code: S) -> LanguageDirection
    where
        S: Into<NSString>,
    {
        let class = class!(NSLocale);
        unsafe { msg_send![class, characterDirectionForLanguage: iso_language_code.into()] }
    }

    fn lineDirectionForLanguage<S>(&self, iso_language_code: S) -> LanguageDirection
    where
        S: Into<NSString>,
    {
        let class = class!(NSLocale);
        unsafe { msg_send![class, lineDirectionForLanguage: iso_language_code.into()] }
    }
}

impl t_NSObject for NSLocale {
    fn new() -> Self {
        todo!()
    }

    fn toId(mut self) -> id {
        &mut *self.obj
    }

    unsafe fn fromId(_obj: id) -> Self {
        todo!()
    }

    fn description(&self) -> NSString {
        unsafe {
            let description: id = msg_send![self.obj, description];
            NSString::fromId(description)
        }
    }

    fn debugDescription(&self) -> NSString {
        unsafe {
            let description: id = msg_send![self.obj, debugDescription];
            NSString::fromId(description)
        }
    }

    fn retain(&self) -> Self {
        unsafe { msg_send![self.obj, retain] }
    }
}

impl Display for NSLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Debug for NSLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Locale: {}", self.debugDescription())
    }
}

impl Clone for NSLocale {
    fn clone(&self) -> Self {
        NSLocale {
            obj: { unsafe { msg_send![self.obj, retain] } },
        }
    }
}

impl Deref for NSLocale {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.obj
    }
}

impl DerefMut for NSLocale {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.obj
    }
}

impl From<NSLocale> for id {
    /// Converts the `Locale` into an `Object`.
    fn from(mut val: NSLocale) -> Self {
        &mut *val.obj
    }
}

impl From<id> for NSLocale {
    /// Converts the `Object` into a `Locale`.
    fn from(val: id) -> Self {
        NSLocale {
            obj: unsafe { msg_send![val, retain] },
        }
    }
}
