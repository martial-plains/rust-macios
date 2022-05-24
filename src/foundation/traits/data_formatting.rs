use objc::runtime::Object;

use crate::{
    foundation::{Array, LanguageDirection, Locale, String, NSLocaleKey},
    objective_c_runtime::traits::t_NSObject,
};

/// The `Locale` class provides information about the user’s locale and formatting preferences.
pub trait t_NSLocale: t_NSObject {
    /* Initializing a Locale
     */

    /// Initializes a locale using a given locale identifier.
    fn initWithLocaleIdentifier<S>(locale_identifier: S) -> Self
    where
        S: Into<String>;

    /* Getting the User's Locale
     */

    /// A locale which tracks the user’s current preferences.
    fn autoUpdatingCurrent(&self) -> Locale;

    /// A locale representing the user's region settings at the time the property is read.
    fn current() -> Locale;

    /// A locale representing the generic root values with little localization.
    fn system() -> Locale;

    /* Getting Known Identifiers and Codes
     */

    /// The list of locale identifiers available on the system.
    fn availableLocaleIdentifiers() -> Array<String>;

    /// The list of known country or region codes.
    fn isoCountryCodes() -> Array<String>;

    /// The list of known language codes.
    fn isoLanguageCodes() -> Array<String>;

    /// The list of known currency codes.
    fn isoCurrencyCodes() -> Array<String>;

    /// A list of commonly encountered currency codes.
    fn commonIsocurrencyCodes() -> Array<String>;

    /* Converting Between Identifiers
     */

    /* Getting Information About a Locale
     */

    /// The identifier for the locale.
    fn localeIdentifier(&self) -> String;

    /// The country or region code for the locale.
    fn countryCode(&self) -> String;

    /// The language code for the locale.
    fn languageCode(&self) -> String;

    /// The script code for the locale.
    fn scriptCode(&self) -> String;

    /// The variant code for the locale.
    fn variantCode(&self) -> String;

    /// The collation identifier for the locale.
    fn collationIdentifier(&self) -> String;

    /// The collator identifier for the locale.
    fn collatorIdentifier(&self) -> String;

    /// A Boolean value that indicates whether the locale uses the metric system.
    fn usesMetricSystem(&self) -> bool;

    /// The decimal separator for the locale.
    fn decimalSeparator(&self) -> String;

    /// The grouping separator for the locale.
    fn groupingSeparator(&self) -> String;

    /// The currency code for the locale.
    fn currencyCode(&self) -> String;

    /// The currency symbol for the locale.
    fn currencySymbol(&self) -> String;

    /// The calendar identifier for the locale.
    fn calendarIdentifier(&self) -> String;

    /// The begin quotation symbol for the locale.
    fn quotationBeginDelimiter(&self) -> String;

    /// The end quotation symbol for the locale.
    fn quotationEndDelimiter(&self) -> String;

    /// The alternate begin quotation symbol for the locale.
    fn alternateQuotationBeginDelimiter(&self) -> String;

    /// The alternate end quotation symbol for the locale.
    fn alternateQuotationEndDelimiter(&self) -> String;

    /* Accessing Locale Information by Key
     */

    /// Returns the value of the component corresponding to the specified key.
    fn objectForKey(&self, key: NSLocaleKey) -> Option<Object>;

    /// Returns the display name for the given locale component value.
    fn displayNameForKeyValue<S>(&self, key: NSLocaleKey, value: S) -> Option<String>
    where
        S: Into<String>;

    /* Getting the User's Preferred Languages
     */

    /// An ordered list of the user's preferred languages.
    fn preferredLanguages() -> Array<String>;

    /* Getting Line and Character Direction for a Language
     */

    /// Returns the direction of the sequence of characters in a line for the specified ISO language code.
    fn characterDirectionForLanguage<S>(&self, iso_language_code: S) -> LanguageDirection
    where
        S: Into<String>;

    /// Returns the direction of the sequence of lines for the specified ISO language code.
    fn lineDirectionForLanguage<S>(&self, iso_language_code: S) -> LanguageDirection
    where
        S: Into<String>;
}
