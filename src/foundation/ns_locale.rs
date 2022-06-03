use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    foundation::{traits::INSLocale, NSArray, NSString},
    id,
    objective_c_runtime::traits::{FromId, PNSObject},
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

impl PNSObject for NSLocale {
    fn im_class<'a>() -> &'a objc::runtime::Class {
        class!(NSLocale)
    }

    fn im_isEqual(&self, object: &Self) -> bool {
        unsafe { msg_send![self.obj, isEqual: object] }
    }

    fn ip_hash(&self) -> super::UInt {
        unsafe { msg_send![self.obj, hash] }
    }

    fn im_isKindOfClass(&self, aClass: objc::runtime::Class) -> bool {
        unsafe { msg_send![self.obj, isKindOfClass: aClass] }
    }

    fn im_isMemberOfClass(&self, aClass: objc::runtime::Class) -> bool {
        unsafe { msg_send![self.obj, isMemberOfClass: aClass] }
    }

    fn im_respondsToSelector(&self, aSelector: objc::runtime::Sel) -> bool {
        unsafe { msg_send![self.obj, respondsToSelector: aSelector] }
    }

    fn im_conformsToProtocol(&self, aProtocol: objc::runtime::Protocol) -> bool {
        unsafe { msg_send![self.obj, conformsToProtocol: aProtocol] }
    }

    fn ip_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.obj, description]) }
    }

    fn ip_debugDescription(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.obj, debugDescription]) }
    }

    fn im_performSelector(&self, aSelector: objc::runtime::Sel) -> id {
        unsafe { msg_send![self.obj, performSelector: aSelector] }
    }

    fn im_performSelector_withObject(&self, aSelector: objc::runtime::Sel, withObject: id) -> id {
        unsafe { msg_send![self.obj, performSelector: aSelector withObject: withObject] }
    }

    fn im_isProxy(&self) -> bool {
        unsafe { msg_send![self.obj, isProxy] }
    }
}

impl INSLocale for NSLocale {
    fn im_initWithLocaleIdentifier<S>(locale_identifier: S) -> Self
    where
        S: Into<NSString>,
    {
        unsafe {
            let class: NSLocale = msg_send![class!(NSLocale), alloc];
            let obj = msg_send![class, initWithLocaleIdentifier: locale_identifier.into()];
            NSLocale { obj }
        }
    }

    fn tp_autoupdatingCurrentLocale(&self) -> NSLocale {
        let class = class!(NSLocale);
        unsafe { msg_send![class, autoupdatingCurrent] }
    }

    fn tp_currentLocale() -> NSLocale {
        let class = class!(NSLocale);
        unsafe { msg_send![class, currentLocale] }
    }

    fn tp_systemLocale() -> NSLocale {
        let class = class!(NSLocale);
        unsafe { msg_send![class, systemLocale] }
    }

    fn tp_availableLocaleIdentifiers() -> NSArray<NSString> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, availableLocaleIdentifiers] }
    }

    fn tp_isoCountryCodes() -> NSArray<NSString> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, ISOCountryCodes] }
    }

    fn tp_isoLanguageCodes() -> NSArray<NSString> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, ISOLanguageCodes] }
    }

    fn tp_isoCurrencyCodes() -> NSArray<NSString> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, ISOCurrencyCodes] }
    }

    fn tp_commonIsocurrencyCodes() -> NSArray<NSString> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, commonISOCurrencyCodes] }
    }

    fn ip_localeIdentifier(&self) -> NSString {
        unsafe { msg_send![self.obj, localeIdentifier] }
    }

    fn ip_countryCode(&self) -> NSString {
        unsafe { msg_send![self.obj, countryCode] }
    }

    fn ip_languageCode(&self) -> NSString {
        unsafe { msg_send![self.obj, languageCode] }
    }

    fn ip_scriptCode(&self) -> NSString {
        unsafe { msg_send![self.obj, scriptCode] }
    }

    fn ip_variantCode(&self) -> NSString {
        unsafe { msg_send![self.obj, variantCode] }
    }

    fn ip_collationIdentifier(&self) -> NSString {
        unsafe { msg_send![self.obj, collationIdentifier] }
    }

    fn ip_collatorIdentifier(&self) -> NSString {
        unsafe { msg_send![self.obj, collatorIdentifier] }
    }

    fn ip_usesMetricSystem(&self) -> bool {
        unsafe { msg_send![self.obj, usesMetricSystem] }
    }

    fn ip_decimalSeparator(&self) -> NSString {
        unsafe { msg_send![self.obj, decimalSeparator] }
    }

    fn ip_groupingSeparator(&self) -> NSString {
        unsafe { msg_send![self.obj, groupingSeparator] }
    }

    fn ip_currencyCode(&self) -> NSString {
        unsafe { msg_send![self.obj, currencyCode] }
    }

    fn ip_currencySymbol(&self) -> NSString {
        unsafe { msg_send![self.obj, currencySymbol] }
    }

    fn ip_calendarIdentifier(&self) -> NSString {
        unsafe { msg_send![self.obj, calendarIdentifier] }
    }

    fn ip_quotationBeginDelimiter(&self) -> NSString {
        unsafe { msg_send![self.obj, quotationBeginDelimiter] }
    }

    fn ip_quotationEndDelimiter(&self) -> NSString {
        unsafe { msg_send![self.obj, quotationEndDelimiter] }
    }

    fn ip_alternateQuotationBeginDelimiter(&self) -> NSString {
        unsafe { msg_send![self.obj, alternateQuotationBeginDelimiter] }
    }

    fn ip_alternateQuotationEndDelimiter(&self) -> NSString {
        unsafe { msg_send![self.obj, alternateQuotationEndDelimiter] }
    }

    fn im_objectForKey(&self, key: NSLocaleKey) -> Option<Object> {
        unsafe { msg_send![self.obj, objectForKey: key] }
    }

    fn im_displayNameForKey_value<T>(&self, key: NSLocaleKey, value: T) -> Option<NSString>
    where
        T: Into<NSString>,
    {
        let result: id = unsafe { msg_send![self.obj, displayNameForKey: key value: value.into()] };

        if result.is_null() {
            None
        } else {
            Some(unsafe { NSString::from_id(result) })
        }
    }

    fn tp_preferredLanguages() -> NSArray<NSString> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, preferredLanguages] }
    }

    fn tm_characterDirectionForLanguage<S>(&self, iso_language_code: S) -> LanguageDirection
    where
        S: Into<NSString>,
    {
        let class = class!(NSLocale);
        unsafe { msg_send![class, characterDirectionForLanguage: iso_language_code.into()] }
    }

    fn tm_lineDirectionForLanguage<S>(&self, iso_language_code: S) -> LanguageDirection
    where
        S: Into<NSString>,
    {
        let class = class!(NSLocale);
        unsafe { msg_send![class, lineDirectionForLanguage: iso_language_code.into()] }
    }
}

impl Display for NSLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl Debug for NSLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Locale: {}", self.ip_debugDescription())
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
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from(val: id) -> Self {
        NSLocale {
            obj: unsafe { msg_send![val, retain] },
        }
    }
}
