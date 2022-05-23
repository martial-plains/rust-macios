use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    foundation::{Array, String},
    id,
    objective_c_runtime::NSObject,
};

use self::key::NSLocaleKey;

pub mod key;

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
pub struct Locale {
    /// The raw pointer to the Objective-C object.
    pub obj: Id<Object>,
}

impl Locale {
    /* Initializing a Locale
     */

    /// Initializes a locale using a given locale identifier.
    pub fn init_with_locale_identifier<T>(locale_identifier: T) -> Self
    where
        T: Into<String>,
    {
        unsafe {
            let class: Locale = msg_send![class!(NSLocale), alloc];
            let obj = msg_send![class, initWithLocaleIdentifier: locale_identifier.into()];
            Locale { obj }
        }
    }

    /* Getting the User's Locale
     */

    /// A locale which tracks the user’s current preferences.
    pub fn auto_updating_current(&self) -> Locale {
        let class = class!(NSLocale);
        unsafe { msg_send![class, autoupdatingCurrent] }
    }

    /// A locale representing the user's region settings at the time the property is read.
    pub fn current() -> Locale {
        let class = class!(NSLocale);
        unsafe { msg_send![class, currentLocale] }
    }

    /// A locale representing the generic root values with little localization.
    pub fn system() -> Locale {
        let class = class!(NSLocale);
        unsafe { msg_send![class, systemLocale] }
    }

    /* Getting Known Identifiers and Codes
     */

    /// The list of locale identifiers available on the system.
    pub fn available_locale_identifiers() -> Array<String> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, availableLocaleIdentifiers] }
    }

    /// The list of known country or region codes.
    pub fn iso_country_codes() -> Array<String> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, ISOCountryCodes] }
    }

    /// The list of known language codes.
    pub fn iso_language_codes() -> Array<String> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, ISOLanguageCodes] }
    }

    /// The list of known currency codes.
    pub fn iso_currency_codes() -> Array<String> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, ISOCurrencyCodes] }
    }

    /// A list of commonly encountered currency codes.
    pub fn common_isocurrency_codes() -> Array<String> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, commonISOCurrencyCodes] }
    }

    /* Converting Between Identifiers
     */

    /* Getting Information About a Locale
     */

    /// The identifier for the locale.
    pub fn locale_identifier(&self) -> String {
        unsafe { msg_send![self.obj, localeIdentifier] }
    }

    /// The country or region code for the locale.
    pub fn country_code(&self) -> String {
        unsafe { msg_send![self.obj, countryCode] }
    }

    /// The language code for the locale.
    pub fn language_code(&self) -> String {
        unsafe { msg_send![self.obj, languageCode] }
    }

    /// The script code for the locale.
    pub fn script_code(&self) -> String {
        unsafe { msg_send![self.obj, scriptCode] }
    }

    /// The variant code for the locale.
    pub fn variant_code(&self) -> String {
        unsafe { msg_send![self.obj, variantCode] }
    }

    /// The collation identifier for the locale.
    pub fn collation_identifier(&self) -> String {
        unsafe { msg_send![self.obj, collationIdentifier] }
    }

    /// The collator identifier for the locale.
    pub fn collator_identifier(&self) -> String {
        unsafe { msg_send![self.obj, collatorIdentifier] }
    }

    /// A Boolean value that indicates whether the locale uses the metric system.
    pub fn uses_metric_system(&self) -> bool {
        unsafe { msg_send![self.obj, usesMetricSystem] }
    }

    /// The decimal separator for the locale.
    pub fn decimal_separator(&self) -> String {
        unsafe { msg_send![self.obj, decimalSeparator] }
    }

    /// The grouping separator for the locale.
    pub fn grouping_separator(&self) -> String {
        unsafe { msg_send![self.obj, groupingSeparator] }
    }

    /// The currency code for the locale.
    pub fn currency_code(&self) -> String {
        unsafe { msg_send![self.obj, currencyCode] }
    }

    /// The currency symbol for the locale.
    pub fn currency_symbol(&self) -> String {
        unsafe { msg_send![self.obj, currencySymbol] }
    }

    /// The calendar identifier for the locale.
    pub fn calendar_identifier(&self) -> String {
        unsafe { msg_send![self.obj, calendarIdentifier] }
    }

    /// The begin quotation symbol for the locale.
    pub fn quotation_begin_delimiter(&self) -> String {
        unsafe { msg_send![self.obj, quotationBeginDelimiter] }
    }

    /// The end quotation symbol for the locale.
    pub fn quotation_end_delimiter(&self) -> String {
        unsafe { msg_send![self.obj, quotationEndDelimiter] }
    }

    /// The alternate begin quotation symbol for the locale.
    pub fn alternate_quotation_begin_delimiter(&self) -> String {
        unsafe { msg_send![self.obj, alternateQuotationBeginDelimiter] }
    }

    /// The alternate end quotation symbol for the locale.
    pub fn alternate_quotation_end_delimiter(&self) -> String {
        unsafe { msg_send![self.obj, alternateQuotationEndDelimiter] }
    }

    /* Accessing Locale Information by Key
     */

    /// Returns the value of the component corresponding to the specified key.
    pub fn object_for_key(&self, key: NSLocaleKey) -> Option<Object> {
        unsafe { msg_send![self.obj, objectForKey: key] }
    }

    /// Returns the display name for the given locale component value.
    pub fn display_name_for_key_value<T>(&self, key: NSLocaleKey, value: T) -> Option<String>
    where
        T: Into<String>,
    {
        let result: id = unsafe { msg_send![self.obj, displayNameForKey: key value: value.into()] };

        if result.is_null() {
            None
        } else {
            Some(String::from_id(result))
        }
    }

    /* Getting the User's Preferred Languages
     */

    /// An ordered list of the user's preferred languages.
    pub fn preferred_languages() -> Array<String> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, preferredLanguages] }
    }

    /* Getting Line and Character Direction for a Language
     */

    /// Returns the direction of the sequence of characters in a line for the specified ISO language code.
    pub fn character_direction_for_language<T>(&self, iso_language_code: T) -> LanguageDirection
    where
        T: Into<String>,
    {
        let class = class!(NSLocale);
        unsafe { msg_send![class, characterDirectionForLanguage: iso_language_code.into()] }
    }

    /// Returns the direction of the sequence of lines for the specified ISO language code.
    pub fn line_direction_for_language<T>(&self, iso_language_code: T) -> LanguageDirection
    where
        T: Into<String>,
    {
        let class = class!(NSLocale);
        unsafe { msg_send![class, lineDirectionForLanguage: iso_language_code.into()] }
    }
}

impl NSObject for Locale {
    fn init() -> Self {
        todo!()
    }

    fn to_id(mut self) -> id {
        &mut *self.obj
    }

    fn from_id(_obj: id) -> Self {
        todo!()
    }

    fn description(&self) -> String {
        unsafe {
            let description: id = msg_send![self.obj, description];
            String::from_id(description)
        }
    }

    fn debug_description(&self) -> String {
        unsafe {
            let description: id = msg_send![self.obj, debugDescription];
            String::from_id(description)
        }
    }

    fn retain(&self) -> Self {
        unsafe { msg_send![self.obj, retain] }
    }
}

impl Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Debug for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Locale: {}", self.debug_description())
    }
}

impl Clone for Locale {
    fn clone(&self) -> Self {
        Locale {
            obj: { unsafe { msg_send![self.obj, retain] } },
        }
    }
}

impl Deref for Locale {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.obj
    }
}

impl DerefMut for Locale {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.obj
    }
}

impl From<Locale> for id {
    /// Converts the `Locale` into an `Object`.
    #[allow(trivial_casts)]
    fn from(val: Locale) -> Self {
        &*val as *const _ as *mut Object
    }
}

impl From<id> for Locale {
    /// Converts the `Object` into a `Locale`.
    fn from(val: id) -> Self {
        Locale {
            obj: unsafe { msg_send![val, retain] },
        }
    }
}
