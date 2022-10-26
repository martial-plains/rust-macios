use objc::{msg_send, sel, sel_impl};

use crate::{
    objective_c_runtime::{
        id,
        macros::{interface_impl, object},
        nil,
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};

use super::{
    NSArray, NSCharacterSet, NSCoder, NSDictionary, NSLocaleKey, NSNotificationName, NSString,
};

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

    /// Returns a locale initialized using the given locale identifier.
    #[method]
    pub fn locale_with_locale_identifier(string: &NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            let ptr = msg_send![Self::m_class(), localeWithLocaleIdentifier: string.m_self()];
            FromId::from_id(ptr)
        }
    }

    /// Initializes a locale using a given locale identifier.
    #[method]
    pub fn init_with_locale_identifier(&mut self, locale_identifier: &NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            let ptr =
                msg_send![self.m_self(), initWithLocaleIdentifier: locale_identifier.m_self()];
            FromId::from_id(ptr)
        }
    }

    /// Returns a locale initialized from data in the given unarchiver.
    #[method]
    pub fn init_with_coder(&mut self, coder: &NSCoder) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithCoder: coder.m_self()]) }
    }

    /* Getting the User's Locale
     */

    /// A locale which tracks the user’s current preferences.
    #[property]
    pub fn autoupdating_current_locale() -> NSLocale {
        unsafe { NSLocale::from_id(msg_send![Self::m_class(), autoupdatingCurrentLocale]) }
    }

    /// A locale representing the user's region settings at the time the property is read.
    #[property]
    pub fn current_locale() -> NSLocale {
        unsafe { NSLocale::from_id(msg_send![Self::m_class(), currentLocale]) }
    }

    /// A locale representing the generic root values with little localization.
    #[property]
    pub fn system_locale() -> NSLocale {
        unsafe { NSLocale::from_id(msg_send![Self::m_class(), systemLocale]) }
    }

    /* Getting Known Identifiers and Codes
     */

    /// The list of locale identifiers available on the system.
    #[property]
    pub fn available_locale_identifiers() -> NSArray<NSString> {
        unsafe { NSArray::from_id(msg_send![Self::m_class(), availableLocaleIdentifiers]) }
    }

    /// The list of known country or region codes.
    #[property]
    pub fn iso_country_codes() -> NSArray<NSString> {
        unsafe { NSArray::from_id(msg_send![Self::m_class(), ISOCountryCodes]) }
    }

    /// The list of known language codes.
    #[property]
    pub fn iso_language_codes() -> NSArray<NSString> {
        unsafe { NSArray::from_id(msg_send![Self::m_class(), ISOLanguageCodes]) }
    }

    /// The list of known currency codes.
    #[property]
    pub fn iso_currency_codes() -> NSArray<NSString> {
        unsafe { NSArray::from_id(msg_send![Self::m_class(), ISOCurrencyCodes]) }
    }

    /// A list of commonly encountered currency codes.
    #[property]
    pub fn common_isocurrency_codes() -> NSArray<NSString> {
        unsafe { NSArray::from_id(msg_send![Self::m_class(), commonISOCurrencyCodes]) }
    }

    /* Converting Between Identifiers
     */

    /// Returns the canonical identifier for a given locale identification string.
    #[method]
    pub fn canonical_locale_identifier_from_string(string: &NSString) -> NSString {
        unsafe {
            NSString::from_id(msg_send![
                Self::m_class(),
                canonicalLocaleIdentifierFromString: string.m_self()
            ])
        }
    }

    /// Returns a dictionary that is the result of parsing a locale ID.
    #[method]
    pub fn components_from_locale_identifier(
        string: &NSString,
    ) -> NSDictionary<NSString, NSString> {
        unsafe {
            NSDictionary::from_id(msg_send![
                Self::m_class(),
                componentsFromLocaleIdentifier: string.m_self()
            ])
        }
    }

    /// Returns a locale identifier from the components specified in a given dictionary.
    #[method]
    pub fn locale_identifier_from_components(dict: &NSDictionary<NSString, NSString>) -> NSString {
        unsafe {
            NSString::from_id(msg_send![
                Self::m_class(),
                localeIdentifierFromComponents: dict.m_self()
            ])
        }
    }

    /// Returns a canonical language identifier by mapping an arbitrary locale identification string to the canonical identifier.
    #[method]
    pub fn canonical_language_identifier_from_string(string: &NSString) -> NSString {
        unsafe {
            NSString::from_id(msg_send![
                Self::m_class(),
                canonicalLanguageIdentifierFromString: string.m_self()
            ])
        }
    }

    /// Returns a locale identifier from a Windows locale code.
    #[method]
    pub fn locale_identifier_from_windows_locale_code(lcid: u32) -> Option<NSString> {
        unsafe {
            let ptr = msg_send![Self::m_class(), localeIdentifierFromWindowsLocaleCode: lcid];

            if ptr != nil {
                Some(NSString::from_id(ptr))
            } else {
                None
            }
        }
    }

    /// Returns a Window locale code from the locale identifier.
    #[method]
    pub fn windows_locale_code_from_locale_identifier(locale_identifier: &NSString) -> u32 {
        unsafe {
            msg_send![
                Self::m_class(),
                windowsLocaleCodeFromLocaleIdentifier: locale_identifier.m_self()
            ]
        }
    }

    /* Getting Information About a Locale
     */

    /// The identifier for the locale.
    #[property]
    pub fn locale_identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), localeIdentifier]) }
    }

    /// The country or region code for the locale.
    #[property]
    pub fn country_code(&self) -> Option<NSString> {
        unsafe {
            let ptr = msg_send![self.m_self(), countryCode];

            if ptr != nil {
                Some(NSString::from_id(ptr))
            } else {
                None
            }
        }
    }

    /// The language code for the locale.
    #[property]
    pub fn language_code(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), languageCode]) }
    }

    /// The script code for the locale.
    #[property]
    pub fn script_code(&self) -> Option<NSString> {
        unsafe {
            let ptr = msg_send![self.m_self(), scriptCode];

            if ptr != nil {
                Some(NSString::from_id(ptr))
            } else {
                None
            }
        }
    }

    /// The variant code for the locale.
    #[property]
    pub fn variant_code(&self) -> Option<NSString> {
        unsafe {
            let ptr = msg_send![self.m_self(), variantCode];

            if ptr != nil {
                Some(NSString::from_id(ptr))
            } else {
                None
            }
        }
    }

    /// The exemplar character set for the locale.
    #[property]
    pub fn exemplar_character_set(&self) -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![self.m_self(), exemplarCharacterSet]) }
    }

    /// The collation identifier for the locale.
    #[property]
    pub fn collation_identifier(&self) -> Option<NSString> {
        unsafe {
            let ptr = msg_send![self.m_self(), collationIdentifier];

            if ptr != nil {
                Some(NSString::from_id(ptr))
            } else {
                None
            }
        }
    }

    /// The collator identifier for the locale.
    #[property]
    pub fn collator_identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), collatorIdentifier]) }
    }

    /// A Boolean value that indicates whether the locale uses the metric system.
    #[property]
    pub fn uses_metric_system(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), usesMetricSystem]) }
    }

    /// The decimal separator for the locale.
    #[property]
    pub fn decimal_separator(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), decimalSeparator]) }
    }

    /// The grouping separator for the locale.
    #[property]
    pub fn grouping_separator(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), groupingSeparator]) }
    }

    /// The currency code for the locale.
    #[property]
    pub fn currency_code(&self) -> Option<NSString> {
        unsafe {
            let ptr = msg_send![self.m_self(), currencyCode];

            if ptr != nil {
                Some(NSString::from_id(ptr))
            } else {
                None
            }
        }
    }

    /// The currency symbol for the locale.
    #[property]
    pub fn currency_symbol(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), currencySymbol]) }
    }

    /// The calendar identifier for the locale.
    #[property]
    pub fn calendar_identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), calendarIdentifier]) }
    }

    /// The begin quotation symbol for the locale.
    #[property]
    pub fn quotation_begin_delimiter(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), quotationBeginDelimiter]) }
    }

    /// The end quotation symbol for the locale.
    #[property]
    pub fn quotation_end_delimiter(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), quotationEndDelimiter]) }
    }

    /// The alternate begin quotation symbol for the locale.
    #[property]
    pub fn alternate_quotation_begin_delimiter(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), alternateQuotationBeginDelimiter]) }
    }

    /// The alternate end quotation symbol for the locale.
    #[property]
    pub fn alternate_quotation_end_delimiter(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), alternateQuotationEndDelimiter]) }
    }

    /* Getting Display Information About a Locale
     */

    /// Returns the localized string for the specified locale identifier.
    #[method]
    pub fn localized_string_for_locale_identifier(&self, locale_identifier: &NSString) -> NSString {
        unsafe {
            NSString::from_id(msg_send![
                self.m_self(),
                localizedStringForLocaleIdentifier: locale_identifier.m_self()
            ])
        }
    }

    /// Returns the localized string for a country or region code.
    #[method]
    pub fn localized_string_for_country_code(&self, country_code: &NSString) -> Option<NSString> {
        unsafe {
            let ptr = msg_send![
                self.m_self(),
                localizedStringForCountryCode: country_code.m_self()
            ];

            if ptr != nil {
                Some(NSString::from_id(ptr))
            } else {
                None
            }
        }
    }

    /// Returns the localized string for the specified language code.
    #[method]
    pub fn localized_string_for_language_code(&self, language_code: &NSString) -> Option<NSString> {
        unsafe {
            let ptr =
                msg_send![self.m_self(), localizedStringForLanguageCode: language_code.m_self()];

            if ptr != nil {
                Some(NSString::from_id(ptr))
            } else {
                None
            }
        }
    }

    /// Returns the localized string for the specified script code.
    #[method]
    pub fn localized_string_for_script_code(&self, script_code: &NSString) -> Option<NSString> {
        unsafe {
            let ptr = msg_send![self.m_self(), localizedStringForScriptCode: script_code.m_self()];

            if ptr != nil {
                Some(NSString::from_id(ptr))
            } else {
                None
            }
        }
    }
    /// Returns the localized string for the specified variant code.
    #[method]
    pub fn localized_string_for_variant_code(&self, variant_code: &NSString) -> Option<NSString> {
        unsafe {
            let ptr =
                msg_send![self.m_self(), localizedStringForVariantCode: variant_code.m_self()];

            if ptr != nil {
                Some(NSString::from_id(ptr))
            } else {
                None
            }
        }
    }
    /// Returns the localized string for the specified collation identifier.
    #[method]
    pub fn localized_string_for_collation_identifier(
        &self,
        collation_identifier: &NSString,
    ) -> Option<NSString> {
        unsafe {
            let ptr = msg_send![
                self.m_self(),
                localizedStringForCollationIdentifier: collation_identifier.m_self()
            ];

            if ptr != nil {
                Some(NSString::from_id(ptr))
            } else {
                None
            }
        }
    }
    /// Returns the localized string for the specified collator identifier.
    #[method]
    pub fn localized_string_for_collator_identifier(
        &self,
        collator_identifier: &NSString,
    ) -> Option<NSString> {
        unsafe {
            let ptr = msg_send![
                self.m_self(),
                localizedStringForCollatorIdentifier: collator_identifier
            ];

            if ptr != nil {
                Some(NSString::from_id(ptr))
            } else {
                None
            }
        }
    }
    /// Returns the localized string for the specified currency code.
    #[method]
    pub fn localized_string_for_currency_code(&self, currency_code: &NSString) -> Option<NSString> {
        unsafe {
            let ptr =
                msg_send![self.m_self(), localizedStringForCurrencyCode: currency_code.m_self()];

            if ptr != nil {
                Some(NSString::from_id(ptr))
            } else {
                None
            }
        }
    }
    /// Returns the localized string for the specified language code.
    #[method]
    pub fn localized_string_for_calendar_identifier(
        &self,
        calendar_identifier: &NSString,
    ) -> Option<NSString> {
        unsafe {
            let ptr = msg_send![
                self.m_self(),
                localizedStringForCalendarIdentifier: calendar_identifier.m_self()
            ];

            if ptr != nil {
                Some(NSString::from_id(ptr))
            } else {
                None
            }
        }
    }
    /* Accessing Locale Information by Key
     */

    /// Returns the value of the component corresponding to the specified key.
    #[method]
    pub fn object_for_key(&self, key: NSLocaleKey) -> Option<id> {
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
    pub fn display_name_for_key_value(
        &self,
        key: NSLocaleKey,
        value: &NSString,
    ) -> Option<NSString> {
        unsafe {
            let obj: id = msg_send![self.m_self(), displayNameForKey: key value: value.m_self()];
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
    pub fn preferred_languages() -> NSArray<NSString> {
        unsafe { NSArray::from_id(msg_send![Self::m_class(), preferredLanguages]) }
    }

    /* Getting Line and Character Direction for a Language
     */

    /// Returns the direction of the sequence of characters in a line for the specified ISO language code.
    #[method]
    pub fn character_direction_for_language(iso_language_code: NSString) -> LanguageDirection {
        unsafe {
            msg_send![
                Self::m_class(),
                characterDirectionForLanguage: iso_language_code
            ]
        }
    }

    /// Returns the direction of the sequence of lines for the specified ISO language code.
    #[method]
    pub fn line_direction_for_language(iso_language_code: NSString) -> LanguageDirection {
        unsafe { msg_send![Self::m_class(), lineDirectionForLanguage: iso_language_code] }
    }
}

extern "C" {
    /// A notification that indicates that the user’s locale changed.
    pub static NSCurrentLocaleDidChangeNotification: NSNotificationName;
}

#[cfg(test)]
mod tests {
    use super::NSLocale;

    use crate::{
        foundation::NSString,
        objective_c_runtime::{nil, traits::PNSObject},
    };

    #[test]
    fn test_current_locale() {
        let locale = NSLocale::current_locale();
        assert!(locale.m_self() != nil)
    }

    #[test]
    fn test_init_with_locale_identifier() {
        let ident = NSLocale::current_locale().locale_identifier();

        assert!(
            NSLocale::m_alloc()
                .init_with_locale_identifier(&ident)
                .locale_identifier()
                == NSLocale::current_locale().locale_identifier()
        );

        assert!(
            NSLocale::m_alloc()
                .init_with_locale_identifier(&ident)
                .locale_identifier()
                == NSLocale::current_locale().locale_identifier()
        );

        assert!(
            NSLocale::m_alloc()
                .init_with_locale_identifier(&ident)
                .locale_identifier()
                == NSLocale::current_locale().locale_identifier()
        );
    }

    #[test]
    fn test_locale_with_locale_identifier() {
        let ident = NSLocale::current_locale().locale_identifier();

        assert!(
            NSLocale::locale_with_locale_identifier(&ident).locale_identifier()
                == NSLocale::current_locale().locale_identifier()
        );

        assert!(
            NSLocale::locale_with_locale_identifier(&ident).locale_identifier()
                == NSLocale::current_locale().locale_identifier()
        );

        assert!(
            NSLocale::locale_with_locale_identifier(&ident).locale_identifier()
                == NSLocale::current_locale().locale_identifier()
        );
    }

    #[test]
    fn test_country_less_locale() {
        let name: NSString = "zh-Hans".into();
        assert!(NSLocale::locale_with_locale_identifier(&name)
            .country_code()
            .is_none());
    }

    #[test]
    fn test_properties() {
        let en = NSLocale::locale_with_locale_identifier(&"en-US".into());
        assert_eq!(en.alternate_quotation_begin_delimiter(), "‘");
        assert_eq!(en.alternate_quotation_end_delimiter(), "’");
        assert!(en.collation_identifier().is_none());
        assert_eq!(en.collator_identifier(), "en-US");
        assert_eq!(en.country_code(), Some("US".into()));
        assert_eq!(en.currency_code(), Some("USD".into()));
        assert_eq!(en.currency_symbol(), "$");
        assert_eq!(en.decimal_separator(), ".");
        assert_eq!(en.grouping_separator(), ",");
        assert_eq!(en.language_code(), "en");
        assert_eq!(en.locale_identifier(), "en-US");
        assert_eq!(en.quotation_begin_delimiter(), "“");
        assert_eq!(en.quotation_end_delimiter(), "”");
    }
}
