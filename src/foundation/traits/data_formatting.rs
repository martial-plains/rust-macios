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
    fn im_init_with_locale_identifier<S>(locale_identifier: S) -> Self
    where
        S: Into<NSString>;

    /* Getting the User's Locale
     */

    /// A locale which tracks the user’s current preferences.
    fn tp_autoupdating_current_locale(&self) -> NSLocale;

    /// A locale representing the user's region settings at the time the property is read.
    fn tp_current_locale() -> NSLocale;

    /// A locale representing the generic root values with little localization.
    fn tp_system_locale() -> NSLocale;

    /* Getting Known Identifiers and Codes
     */

    /// The list of locale identifiers available on the system.
    fn tp_available_locale_identifiers() -> NSArray<NSString>;

    /// The list of known country or region codes.
    fn tp_iso_country_codes() -> NSArray<NSString>;

    /// The list of known language codes.
    fn tp_iso_language_codes() -> NSArray<NSString>;

    /// The list of known currency codes.
    fn tp_iso_currency_codes() -> NSArray<NSString>;

    /// A list of commonly encountered currency codes.
    fn tp_common_isocurrency_codes() -> NSArray<NSString>;

    /* Converting Between Identifiers
     */

    /* Getting Information About a Locale
     */

    /// The identifier for the locale.
    fn ip_locale_identifier(&self) -> NSString;

    /// The country or region code for the locale.
    fn ip_country_code(&self) -> NSString;

    /// The language code for the locale.
    fn ip_language_code(&self) -> NSString;

    /// The script code for the locale.
    fn ip_script_code(&self) -> NSString;

    /// The variant code for the locale.
    fn ip_variant_code(&self) -> NSString;

    /// The collation identifier for the locale.
    fn ip_collation_identifier(&self) -> NSString;

    /// The collator identifier for the locale.
    fn ip_collator_identifier(&self) -> NSString;

    /// A Boolean value that indicates whether the locale uses the metric system.
    fn ip_uses_metric_system(&self) -> bool;

    /// The decimal separator for the locale.
    fn ip_decimal_separator(&self) -> NSString;

    /// The grouping separator for the locale.
    fn ip_grouping_separator(&self) -> NSString;

    /// The currency code for the locale.
    fn ip_currency_code(&self) -> NSString;

    /// The currency symbol for the locale.
    fn ip_currency_symbol(&self) -> NSString;

    /// The calendar identifier for the locale.
    fn ip_calendar_identifier(&self) -> NSString;

    /// The begin quotation symbol for the locale.
    fn ip_quotation_begin_delimiter(&self) -> NSString;

    /// The end quotation symbol for the locale.
    fn ip_quotation_end_delimiter(&self) -> NSString;

    /// The alternate begin quotation symbol for the locale.
    fn ip_alternate_quotation_begin_delimiter(&self) -> NSString;

    /// The alternate end quotation symbol for the locale.
    fn ip_alternate_quotation_end_delimiter(&self) -> NSString;

    /* Accessing Locale Information by Key
     */

    /// Returns the value of the component corresponding to the specified key.
    fn im_object_for_key(&self, key: NSLocaleKey) -> Option<Object>;

    /// Returns the display name for the given locale component value.
    fn im_display_name_for_key_value<S>(&self, key: NSLocaleKey, value: S) -> Option<NSString>
    where
        S: Into<NSString>;

    /* Getting the User's Preferred Languages
     */

    /// An ordered list of the user's preferred languages.
    fn tp_preferred_languages() -> NSArray<NSString>;

    /* Getting Line and Character Direction for a Language
     */

    /// Returns the direction of the sequence of characters in a line for the specified ISO language code.
    fn tm_character_direction_for_language<S>(&self, iso_language_code: S) -> LanguageDirection
    where
        S: Into<NSString>;

    /// Returns the direction of the sequence of lines for the specified ISO language code.
    fn tm_line_direction_for_language<S>(&self, iso_language_code: S) -> LanguageDirection
    where
        S: Into<NSString>;
}
