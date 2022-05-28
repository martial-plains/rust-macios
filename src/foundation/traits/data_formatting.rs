use objc::runtime::Object;

use crate::{
    foundation::{LanguageDirection, NSArray, NSLocale, NSLocaleKey, NSString},
    objective_c_runtime::traits::PNSObject,
};

/// The `Locale` class provides information about the user’s locale and formatting preferences.
pub trait t_NSLocale: PNSObject {
    /* Initializing a Locale
     */

    /// Initializes a locale using a given locale identifier.
    fn initWithLocaleIdentifier<S>(locale_identifier: S) -> Self
    where
        S: Into<NSString>;

    /* Getting the User's Locale
     */

    /// A locale which tracks the user’s current preferences.
    fn autoUpdatingCurrent(&self) -> NSLocale;

    /// A locale representing the user's region settings at the time the property is read.
    fn current() -> NSLocale;

    /// A locale representing the generic root values with little localization.
    fn system() -> NSLocale;

    /* Getting Known Identifiers and Codes
     */

    /// The list of locale identifiers available on the system.
    fn availableLocaleIdentifiers() -> NSArray<NSString>;

    /// The list of known country or region codes.
    fn isoCountryCodes() -> NSArray<NSString>;

    /// The list of known language codes.
    fn isoLanguageCodes() -> NSArray<NSString>;

    /// The list of known currency codes.
    fn isoCurrencyCodes() -> NSArray<NSString>;

    /// A list of commonly encountered currency codes.
    fn commonIsocurrencyCodes() -> NSArray<NSString>;

    /* Converting Between Identifiers
     */

    /* Getting Information About a Locale
     */

    /// The identifier for the locale.
    fn localeIdentifier(&self) -> NSString;

    /// The country or region code for the locale.
    fn countryCode(&self) -> NSString;

    /// The language code for the locale.
    fn languageCode(&self) -> NSString;

    /// The script code for the locale.
    fn scriptCode(&self) -> NSString;

    /// The variant code for the locale.
    fn variantCode(&self) -> NSString;

    /// The collation identifier for the locale.
    fn collationIdentifier(&self) -> NSString;

    /// The collator identifier for the locale.
    fn collatorIdentifier(&self) -> NSString;

    /// A Boolean value that indicates whether the locale uses the metric system.
    fn usesMetricSystem(&self) -> bool;

    /// The decimal separator for the locale.
    fn decimalSeparator(&self) -> NSString;

    /// The grouping separator for the locale.
    fn groupingSeparator(&self) -> NSString;

    /// The currency code for the locale.
    fn currencyCode(&self) -> NSString;

    /// The currency symbol for the locale.
    fn currencySymbol(&self) -> NSString;

    /// The calendar identifier for the locale.
    fn calendarIdentifier(&self) -> NSString;

    /// The begin quotation symbol for the locale.
    fn quotationBeginDelimiter(&self) -> NSString;

    /// The end quotation symbol for the locale.
    fn quotationEndDelimiter(&self) -> NSString;

    /// The alternate begin quotation symbol for the locale.
    fn alternateQuotationBeginDelimiter(&self) -> NSString;

    /// The alternate end quotation symbol for the locale.
    fn alternateQuotationEndDelimiter(&self) -> NSString;

    /* Accessing Locale Information by Key
     */

    /// Returns the value of the component corresponding to the specified key.
    fn objectForKey(&self, key: NSLocaleKey) -> Option<Object>;

    /// Returns the display name for the given locale component value.
    fn displayNameForKeyValue<S>(&self, key: NSLocaleKey, value: S) -> Option<NSString>
    where
        S: Into<NSString>;

    /* Getting the User's Preferred Languages
     */

    /// An ordered list of the user's preferred languages.
    fn preferredLanguages() -> NSArray<NSString>;

    /* Getting Line and Character Direction for a Language
     */

    /// Returns the direction of the sequence of characters in a line for the specified ISO language code.
    fn characterDirectionForLanguage<S>(&self, iso_language_code: S) -> LanguageDirection
    where
        S: Into<NSString>;

    /// Returns the direction of the sequence of lines for the specified ISO language code.
    fn lineDirectionForLanguage<S>(&self, iso_language_code: S) -> LanguageDirection
    where
        S: Into<NSString>;
}
