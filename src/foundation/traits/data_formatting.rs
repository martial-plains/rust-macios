use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::{LanguageDirection, NSArray, NSLocale, NSLocaleKey, NSString},
    objective_c_runtime::{
        id, nil,
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};

/// The `Locale` class provides information about the user’s locale and formatting preferences.
pub trait INSLocale: PNSObject {
    /* Initializing a Locale
    */

    /// Initializes a locale using a given locale identifier.
    fn im_init_with_locale_identifier(locale_identifier: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            let cls: id = msg_send![Self::im_class(), new];
            let ptr = msg_send![cls, initWithLocaleIdentifier: locale_identifier];
            FromId::from_id(ptr)
        }
    }

    /* Getting the User's Locale
    */

    /// A locale which tracks the user’s current preferences.
    fn tp_autoupdating_current_locale() -> NSLocale {
        unsafe { msg_send![Self::im_class(), autoupdatingCurrentLocale] }
    }

    /// A locale representing the user's region settings at the time the property is read.
    fn tp_current_locale() -> NSLocale {
        unsafe { msg_send![Self::im_class(), currentLocale] }
    }

    /// A locale representing the generic root values with little localization.
    fn tp_system_locale() -> NSLocale {
        unsafe { msg_send![Self::im_class(), systemLocale] }
    }

    /* Getting Known Identifiers and Codes
    */

    /// The list of locale identifiers available on the system.
    fn tp_available_locale_identifiers() -> NSArray<NSString> {
        unsafe { msg_send![Self::im_class(), availableLocaleIdentifiers] }
    }

    /// The list of known country or region codes.
    fn tp_iso_country_codes() -> NSArray<NSString> {
        unsafe { msg_send![Self::im_class(), ISOCountryCodes] }
    }

    /// The list of known language codes.
    fn tp_iso_language_codes() -> NSArray<NSString> {
        unsafe { msg_send![Self::im_class(), ISOLanguageCodes] }
    }

    /// The list of known currency codes.
    fn tp_iso_currency_codes() -> NSArray<NSString> {
        unsafe { msg_send![Self::im_class(), ISOCurrencyCodes] }
    }

    /// A list of commonly encountered currency codes.
    fn tp_common_isocurrency_codes() -> NSArray<NSString> {
        unsafe { msg_send![Self::im_class(), commonISOCurrencyCodes] }
    }

    /* Converting Between Identifiers
    */

    /* Getting Information About a Locale
    */

    /// The identifier for the locale.
    fn ip_locale_identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), localeIdentifier]) }
    }

    /// The country or region code for the locale.
    fn ip_country_code(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), countryCode]) }
    }

    /// The language code for the locale.
    fn ip_language_code(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), languageCode]) }
    }

    /// The script code for the locale.
    fn ip_script_code(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), scriptCode]) }
    }

    /// The variant code for the locale.
    fn ip_variant_code(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), variantCode]) }
    }
    /// The collation identifier for the locale.
    fn ip_collation_identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), collationIdentifier]) }
    }

    /// The collator identifier for the locale.
    fn ip_collator_identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), collatorIdentifier]) }
    }

    /// A Boolean value that indicates whether the locale uses the metric system.
    fn ip_uses_metric_system(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), usesMetricSystem]) }
    }

    /// The decimal separator for the locale.
    fn ip_decimal_separator(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), decimalSeparator]) }
    }

    /// The grouping separator for the locale.
    fn ip_grouping_separator(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), groupingSeparator]) }
    }

    /// The currency code for the locale.
    fn ip_currency_code(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), currencyCode]) }
    }

    /// The currency symbol for the locale.
    fn ip_currency_symbol(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), currencySymbol]) }
    }

    /// The calendar identifier for the locale.
    fn ip_calendar_identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), calendarIdentifier]) }
    }

    /// The begin quotation symbol for the locale.
    fn ip_quotation_begin_delimiter(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), quotationBeginDelimiter]) }
    }

    /// The end quotation symbol for the locale.
    fn ip_quotation_end_delimiter(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), quotationEndDelimiter]) }
    }

    /// The alternate begin quotation symbol for the locale.
    fn ip_alternate_quotation_begin_delimiter(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), alternateQuotationBeginDelimiter]) }
    }

    /// The alternate end quotation symbol for the locale.
    fn ip_alternate_quotation_end_delimiter(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), alternateQuotationEndDelimiter]) }
    }

    /* Accessing Locale Information by Key
    */

    /// Returns the value of the component corresponding to the specified key.
    fn im_object_for_key(&self, key: NSLocaleKey) -> Option<id> {
        unsafe {
            let obj: id = msg_send![self.im_self(), objectForKey: key];
            if obj == nil {
                None
            } else {
                Some(obj)
            }
        }
    }

    /// Returns the display name for the given locale component value.
    fn im_display_name_for_key_value<S>(
        &self,
        key: NSLocaleKey,
        value: NSString,
    ) -> Option<NSString> {
        unsafe {
            let obj: id = msg_send![self.im_self(), displayNameForKey: key value: value];
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
    fn tp_preferred_languages() -> NSArray<NSString> {
        unsafe { NSArray::from_id(msg_send![Self::im_class(), preferredLanguages]) }
    }

    /* Getting Line and Character Direction for a Language
    */

    /// Returns the direction of the sequence of characters in a line for the specified ISO language code.
    fn tm_character_direction_for_language(iso_language_code: NSString) -> LanguageDirection {
        unsafe {
            msg_send![
                Self::im_class(),
                characterDirectionForLanguage: iso_language_code
            ]
        }
    }

    /// Returns the direction of the sequence of lines for the specified ISO language code.
    fn tm_line_direction_for_language(iso_language_code: NSString) -> LanguageDirection {
        unsafe {
            msg_send![
                Self::im_class(),
                lineDirectionForLanguage: iso_language_code
            ]
        }
    }
}
