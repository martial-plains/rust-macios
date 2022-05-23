use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    foundation::{traits::t_NSLocale, Array, String},
    id,
    objective_c_runtime::traits::t_NSObject,
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

impl t_NSLocale for Locale {
    fn init_with_locale_identifier<T>(locale_identifier: T) -> Self
    where
        T: Into<String>,
    {
        unsafe {
            let class: Locale = msg_send![class!(NSLocale), alloc];
            let obj = msg_send![class, initWithLocaleIdentifier: locale_identifier.into()];
            Locale { obj }
        }
    }

    fn auto_updating_current(&self) -> Locale {
        let class = class!(NSLocale);
        unsafe { msg_send![class, autoupdatingCurrent] }
    }

    fn current() -> Locale {
        let class = class!(NSLocale);
        unsafe { msg_send![class, currentLocale] }
    }

    fn system() -> Locale {
        let class = class!(NSLocale);
        unsafe { msg_send![class, systemLocale] }
    }

    fn available_locale_identifiers() -> Array<String> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, availableLocaleIdentifiers] }
    }

    fn iso_country_codes() -> Array<String> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, ISOCountryCodes] }
    }

    fn iso_language_codes() -> Array<String> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, ISOLanguageCodes] }
    }

    fn iso_currency_codes() -> Array<String> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, ISOCurrencyCodes] }
    }

    fn common_isocurrency_codes() -> Array<String> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, commonISOCurrencyCodes] }
    }

    fn locale_identifier(&self) -> String {
        unsafe { msg_send![self.obj, localeIdentifier] }
    }

    fn country_code(&self) -> String {
        unsafe { msg_send![self.obj, countryCode] }
    }

    fn language_code(&self) -> String {
        unsafe { msg_send![self.obj, languageCode] }
    }

    fn script_code(&self) -> String {
        unsafe { msg_send![self.obj, scriptCode] }
    }

    fn variant_code(&self) -> String {
        unsafe { msg_send![self.obj, variantCode] }
    }

    fn collation_identifier(&self) -> String {
        unsafe { msg_send![self.obj, collationIdentifier] }
    }

    fn collator_identifier(&self) -> String {
        unsafe { msg_send![self.obj, collatorIdentifier] }
    }

    fn uses_metric_system(&self) -> bool {
        unsafe { msg_send![self.obj, usesMetricSystem] }
    }

    fn decimal_separator(&self) -> String {
        unsafe { msg_send![self.obj, decimalSeparator] }
    }

    fn grouping_separator(&self) -> String {
        unsafe { msg_send![self.obj, groupingSeparator] }
    }

    fn currency_code(&self) -> String {
        unsafe { msg_send![self.obj, currencyCode] }
    }

    fn currency_symbol(&self) -> String {
        unsafe { msg_send![self.obj, currencySymbol] }
    }

    fn calendar_identifier(&self) -> String {
        unsafe { msg_send![self.obj, calendarIdentifier] }
    }

    fn quotation_begin_delimiter(&self) -> String {
        unsafe { msg_send![self.obj, quotationBeginDelimiter] }
    }

    fn quotation_end_delimiter(&self) -> String {
        unsafe { msg_send![self.obj, quotationEndDelimiter] }
    }

    fn alternate_quotation_begin_delimiter(&self) -> String {
        unsafe { msg_send![self.obj, alternateQuotationBeginDelimiter] }
    }

    fn alternate_quotation_end_delimiter(&self) -> String {
        unsafe { msg_send![self.obj, alternateQuotationEndDelimiter] }
    }

    fn object_for_key(&self, key: NSLocaleKey) -> Option<Object> {
        unsafe { msg_send![self.obj, objectForKey: key] }
    }

    fn display_name_for_key_value<T>(&self, key: NSLocaleKey, value: T) -> Option<String>
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

    fn preferred_languages() -> Array<String> {
        let class = class!(NSLocale);
        unsafe { msg_send![class, preferredLanguages] }
    }

    fn character_direction_for_language<T>(&self, iso_language_code: T) -> LanguageDirection
    where
        T: Into<String>,
    {
        let class = class!(NSLocale);
        unsafe { msg_send![class, characterDirectionForLanguage: iso_language_code.into()] }
    }

    fn line_direction_for_language<T>(&self, iso_language_code: T) -> LanguageDirection
    where
        T: Into<String>,
    {
        let class = class!(NSLocale);
        unsafe { msg_send![class, lineDirectionForLanguage: iso_language_code.into()] }
    }
}

impl t_NSObject for Locale {
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
