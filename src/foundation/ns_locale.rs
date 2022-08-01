use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    objective_c_runtime::{
        id,
        macros::object,
        nil,
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};

use super::{NSArray, NSLocaleKey, NSString};

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

object! {
    /// The [`NSLocale`] class provides information about the user’s locale and formatting preferences.
    unsafe pub struct NSLocale;
}

#[interface_impl(NSObject)]
impl NSLocale {
    /* Initializing a Locale
     */

    /// Initializes a locale using a given locale identifier.
    #[method]
    pub fn init_with_locale_identifier(locale_identifier: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            let cls: id = msg_send![Self::m_class(), new];
            let ptr = msg_send![cls, initWithLocaleIdentifier: locale_identifier];
            FromId::from_id(ptr)
        }
    }

    /* Getting the User's Locale
     */

    /// A locale which tracks the user’s current preferences.
    #[property]
    fn autoupdating_current_locale() -> NSLocale {
        unsafe { msg_send![Self::m_class(), autoupdatingCurrentLocale] }
    }

    /// A locale representing the user's region settings at the time the property is read.
    #[property]
    fn current_locale() -> NSLocale {
        unsafe { msg_send![Self::m_class(), currentLocale] }
    }

    /// A locale representing the generic root values with little localization.
    #[property]
    fn system_locale() -> NSLocale {
        unsafe { msg_send![Self::m_class(), systemLocale] }
    }

    /* Getting Known Identifiers and Codes
     */

    /// The list of locale identifiers available on the system.
    #[property]
    fn available_locale_identifiers() -> NSArray<NSString> {
        unsafe { msg_send![Self::m_class(), availableLocaleIdentifiers] }
    }

    /// The list of known country or region codes.
    #[property]
    fn iso_country_codes() -> NSArray<NSString> {
        unsafe { msg_send![Self::m_class(), ISOCountryCodes] }
    }

    /// The list of known language codes.
    #[property]
    fn iso_language_codes() -> NSArray<NSString> {
        unsafe { msg_send![Self::m_class(), ISOLanguageCodes] }
    }

    /// The list of known currency codes.
    #[property]
    fn iso_currency_codes() -> NSArray<NSString> {
        unsafe { msg_send![Self::m_class(), ISOCurrencyCodes] }
    }

    /// A list of commonly encountered currency codes.
    #[property]
    fn common_isocurrency_codes() -> NSArray<NSString> {
        unsafe { msg_send![Self::m_class(), commonISOCurrencyCodes] }
    }

    /* Converting Between Identifiers
     */

    /* Getting Information About a Locale
     */

    /// The identifier for the locale.
    #[property]
    fn locale_identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), localeIdentifier]) }
    }

    /// The country or region code for the locale.
    #[property]
    fn country_code(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), countryCode]) }
    }

    /// The language code for the locale.
    #[property]
    fn language_code(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), languageCode]) }
    }

    /// The script code for the locale.
    #[property]
    fn script_code(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), scriptCode]) }
    }

    /// The variant code for the locale.
    #[property]
    fn variant_code(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), variantCode]) }
    }
    /// The collation identifier for the locale.
    #[property]
    fn collation_identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), collationIdentifier]) }
    }

    /// The collator identifier for the locale.
    #[property]
    fn collator_identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), collatorIdentifier]) }
    }

    /// A Boolean value that indicates whether the locale uses the metric system.
    #[property]
    fn uses_metric_system(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), usesMetricSystem]) }
    }

    /// The decimal separator for the locale.
    #[property]
    fn decimal_separator(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), decimalSeparator]) }
    }

    /// The grouping separator for the locale.
    #[property]
    fn grouping_separator(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), groupingSeparator]) }
    }

    /// The currency code for the locale.
    #[property]
    fn currency_code(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), currencyCode]) }
    }

    /// The currency symbol for the locale.
    #[property]
    fn currency_symbol(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), currencySymbol]) }
    }

    /// The calendar identifier for the locale.
    #[property]
    fn calendar_identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), calendarIdentifier]) }
    }

    /// The begin quotation symbol for the locale.
    #[property]
    fn quotation_begin_delimiter(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), quotationBeginDelimiter]) }
    }

    /// The end quotation symbol for the locale.
    #[property]
    fn quotation_end_delimiter(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), quotationEndDelimiter]) }
    }

    /// The alternate begin quotation symbol for the locale.
    #[property]
    fn alternate_quotation_begin_delimiter(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), alternateQuotationBeginDelimiter]) }
    }

    /// The alternate end quotation symbol for the locale.
    #[property]
    fn alternate_quotation_end_delimiter(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), alternateQuotationEndDelimiter]) }
    }

    /* Accessing Locale Information by Key
     */

    /// Returns the value of the component corresponding to the specified key.
    #[method]
    fn object_for_key(&self, key: NSLocaleKey) -> Option<id> {
        unsafe {
            let obj: id = msg_send![self.m_self(), objectForKey: key];
            if obj == nil {
                None
            } else {
                Some(obj)
            }
        }
    }

    /// Returns the display name for the given locale component value.
    #[method]
    fn display_name_for_key_value(&self, key: NSLocaleKey, value: NSString) -> Option<NSString> {
        unsafe {
            let obj: id = msg_send![self.m_self(), displayNameForKey: key value: value];
            if obj == nil {
                None
            } else {
                Some(NSString::from_id(obj))
            }
        }
    }

    /* Getting the User's Preferred Languages
     */

    /// An ordered list of the user's preferred languages.
    #[property]
    fn preferred_languages() -> NSArray<NSString> {
        unsafe { NSArray::from_id(msg_send![Self::m_class(), preferredLanguages]) }
    }

    /* Getting Line and Character Direction for a Language
     */

    /// Returns the direction of the sequence of characters in a line for the specified ISO language code.
    #[method]
    fn character_direction_for_language(iso_language_code: NSString) -> LanguageDirection {
        unsafe {
            msg_send![
                Self::m_class(),
                characterDirectionForLanguage: iso_language_code
            ]
        }
    }

    /// Returns the direction of the sequence of lines for the specified ISO language code.
    #[method]
    fn line_direction_for_language(iso_language_code: NSString) -> LanguageDirection {
        unsafe { msg_send![Self::m_class(), lineDirectionForLanguage: iso_language_code] }
    }
}

impl From<NSLocale> for id {
    /// Converts the `Locale` into an `Object`.
    fn from(val: NSLocale) -> Self {
        unsafe { &mut *val.m_self() }
    }
}

impl From<id> for NSLocale {
    /// Converts the `Object` into a `Locale`.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from(val: id) -> Self {
        unsafe { Self::from_id(val) }
    }
}
