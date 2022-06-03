use objc::runtime::Object;

use crate::{
    foundation::{LanguageDirection, NSArray, NSLocale, NSLocaleKey, NSString},
    objective_c_runtime::traits::PNSObject,
};

/// The `Locale` class provides information about the user’s locale and formatting preferences.
pub trait INSLocale: PNSObject {
    /* Initializing a Locale
     */

    /// Initializes a locale using a given locale identifier.
    fn im_initWithLocaleIdentifier<S>(locale_identifier: S) -> Self
    where
        S: Into<NSString>;

    /* Getting the User's Locale
     */

    /// A locale which tracks the user’s current preferences.
    fn tp_autoupdatingCurrentLocale(&self) -> NSLocale;

    /// A locale representing the user's region settings at the time the property is read.
    fn tp_currentLocale() -> NSLocale;

    /// A locale representing the generic root values with little localization.
    fn tp_systemLocale() -> NSLocale;

    /* Getting Known Identifiers and Codes
     */

    /// The list of locale identifiers available on the system.
    fn tp_availableLocaleIdentifiers() -> NSArray<NSString>;

    /// The list of known country or region codes.
    fn tp_isoCountryCodes() -> NSArray<NSString>;

    /// The list of known language codes.
    fn tp_isoLanguageCodes() -> NSArray<NSString>;

    /// The list of known currency codes.
    fn tp_isoCurrencyCodes() -> NSArray<NSString>;

    /// A list of commonly encountered currency codes.
    fn tp_commonIsocurrencyCodes() -> NSArray<NSString>;

    /* Converting Between Identifiers
     */

    /* Getting Information About a Locale
     */

    /// The identifier for the locale.
    fn ip_localeIdentifier(&self) -> NSString;

    /// The country or region code for the locale.
    fn ip_countryCode(&self) -> NSString;

    /// The language code for the locale.
    fn ip_languageCode(&self) -> NSString;

    /// The script code for the locale.
    fn ip_scriptCode(&self) -> NSString;

    /// The variant code for the locale.
    fn ip_variantCode(&self) -> NSString;

    /// The collation identifier for the locale.
    fn ip_collationIdentifier(&self) -> NSString;

    /// The collator identifier for the locale.
    fn ip_collatorIdentifier(&self) -> NSString;

    /// A Boolean value that indicates whether the locale uses the metric system.
    fn ip_usesMetricSystem(&self) -> bool;

    /// The decimal separator for the locale.
    fn ip_decimalSeparator(&self) -> NSString;

    /// The grouping separator for the locale.
    fn ip_groupingSeparator(&self) -> NSString;

    /// The currency code for the locale.
    fn ip_currencyCode(&self) -> NSString;

    /// The currency symbol for the locale.
    fn ip_currencySymbol(&self) -> NSString;

    /// The calendar identifier for the locale.
    fn ip_calendarIdentifier(&self) -> NSString;

    /// The begin quotation symbol for the locale.
    fn ip_quotationBeginDelimiter(&self) -> NSString;

    /// The end quotation symbol for the locale.
    fn ip_quotationEndDelimiter(&self) -> NSString;

    /// The alternate begin quotation symbol for the locale.
    fn ip_alternateQuotationBeginDelimiter(&self) -> NSString;

    /// The alternate end quotation symbol for the locale.
    fn ip_alternateQuotationEndDelimiter(&self) -> NSString;

    /* Accessing Locale Information by Key
     */

    /// Returns the value of the component corresponding to the specified key.
    fn im_objectForKey(&self, key: NSLocaleKey) -> Option<Object>;

    /// Returns the display name for the given locale component value.
    fn im_displayNameForKey_value<S>(&self, key: NSLocaleKey, value: S) -> Option<NSString>
    where
        S: Into<NSString>;

    /* Getting the User's Preferred Languages
     */

    /// An ordered list of the user's preferred languages.
    fn tp_preferredLanguages() -> NSArray<NSString>;

    /* Getting Line and Character Direction for a Language
     */

    /// Returns the direction of the sequence of characters in a line for the specified ISO language code.
    fn tm_characterDirectionForLanguage<S>(&self, iso_language_code: S) -> LanguageDirection
    where
        S: Into<NSString>;

    /// Returns the direction of the sequence of lines for the specified ISO language code.
    fn tm_lineDirectionForLanguage<S>(&self, iso_language_code: S) -> LanguageDirection
    where
        S: Into<NSString>;
}
