use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

use objc::{
    class, msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::{traits::INSLocale, NSArray, NSString},
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject},
    },
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
    fn im_class<'a>() -> &'a Class {
        class!(NSLocale)
    }

    fn im_is_equal(&self, object: &Self) -> bool {
        unsafe { msg_send![self.obj, isEqual: object] }
    }

    fn ip_hash(&self) -> super::UInt {
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

impl INSLocale for NSLocale {
    fn im_init_with_locale_identifier<S>(locale_identifier: S) -> Self
    where
        S: Into<NSString>,
    {
        unsafe {
            let class: NSLocale = msg_send![class!(NSLocale), alloc];
            let obj = msg_send![class, initWithLocaleIdentifier: locale_identifier.into()];
            NSLocale { obj }
        }
    }

    fn tp_autoupdating_current_locale(&self) -> NSLocale {
        let class = class!(NSLocale);
        unsafe { msg_send![class, autoupdatingCurrent] }
    }

    fn tp_current_locale() -> NSLocale {
        let class = class!(NSLocale);
        unsafe { msg_send![class, currentLocale] }
    }

    fn tp_system_locale() -> NSLocale {
        let class = class!(NSLocale);
        unsafe { msg_send![class, systemLocale] }
    }

    fn tp_available_locale_identifiers() -> NSArray<NSString> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, availableLocaleIdentifiers] }
    }

    fn tp_iso_country_codes() -> NSArray<NSString> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, ISOCountryCodes] }
    }

    fn tp_iso_language_codes() -> NSArray<NSString> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, ISOLanguageCodes] }
    }

    fn tp_iso_currency_codes() -> NSArray<NSString> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, ISOCurrencyCodes] }
    }

    fn tp_common_isocurrency_codes() -> NSArray<NSString> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, commonISOCurrencyCodes] }
    }

    fn ip_locale_identifier(&self) -> NSString {
        unsafe { msg_send![self.obj, localeIdentifier] }
    }

    fn ip_country_code(&self) -> NSString {
        unsafe { msg_send![self.obj, countryCode] }
    }

    fn ip_language_code(&self) -> NSString {
        unsafe { msg_send![self.obj, languageCode] }
    }

    fn ip_script_code(&self) -> NSString {
        unsafe { msg_send![self.obj, scriptCode] }
    }

    fn ip_variant_code(&self) -> NSString {
        unsafe { msg_send![self.obj, variantCode] }
    }

    fn ip_collation_identifier(&self) -> NSString {
        unsafe { msg_send![self.obj, collationIdentifier] }
    }

    fn ip_collator_identifier(&self) -> NSString {
        unsafe { msg_send![self.obj, collatorIdentifier] }
    }

    fn ip_uses_metric_system(&self) -> bool {
        unsafe { msg_send![self.obj, usesMetricSystem] }
    }

    fn ip_decimal_separator(&self) -> NSString {
        unsafe { msg_send![self.obj, decimalSeparator] }
    }

    fn ip_grouping_separator(&self) -> NSString {
        unsafe { msg_send![self.obj, groupingSeparator] }
    }

    fn ip_currency_code(&self) -> NSString {
        unsafe { msg_send![self.obj, currencyCode] }
    }

    fn ip_currency_symbol(&self) -> NSString {
        unsafe { msg_send![self.obj, currencySymbol] }
    }

    fn ip_calendar_identifier(&self) -> NSString {
        unsafe { msg_send![self.obj, calendarIdentifier] }
    }

    fn ip_quotation_begin_delimiter(&self) -> NSString {
        unsafe { msg_send![self.obj, quotationBeginDelimiter] }
    }

    fn ip_quotation_end_delimiter(&self) -> NSString {
        unsafe { msg_send![self.obj, quotationEndDelimiter] }
    }

    fn ip_alternate_quotation_begin_delimiter(&self) -> NSString {
        unsafe { msg_send![self.obj, alternateQuotationBeginDelimiter] }
    }

    fn ip_alternate_quotation_end_delimiter(&self) -> NSString {
        unsafe { msg_send![self.obj, alternateQuotationEndDelimiter] }
    }

    fn im_object_for_key(&self, key: NSLocaleKey) -> Option<Object> {
        unsafe { msg_send![self.obj, objectForKey: key] }
    }

    fn im_display_name_for_key_value<T>(&self, key: NSLocaleKey, value: T) -> Option<NSString>
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

    fn tp_preferred_languages() -> NSArray<NSString> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, preferredLanguages] }
    }

    fn tm_character_direction_for_language<S>(&self, iso_language_code: S) -> LanguageDirection
    where
        S: Into<NSString>,
    {
        let class = class!(NSLocale);
        unsafe { msg_send![class, characterDirectionForLanguage: iso_language_code.into()] }
    }

    fn tm_line_direction_for_language<S>(&self, iso_language_code: S) -> LanguageDirection
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
        write!(f, "Locale: {}", self.ip_debug_description())
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
