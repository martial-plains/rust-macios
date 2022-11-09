use objc::{msg_send, sel, sel_impl};

use crate::{
    object,
    objective_c_runtime::{id, macros::interface_impl, traits::FromId},
    utils::to_bool,
};

use super::{
    INSFormatter, Int, NSAttributedString, NSDecimalNumberHandler, NSDictionary, NSError,
    NSFormattingContext, NSLocale, NSNumber, NSNumberFormatterBehavior,
    NSNumberFormatterPadPosition, NSNumberFormatterRoundingMode, NSNumberFormatterStyle, NSRange,
    NSString, UInt,
};

object! {
    /// A formatter that converts between numeric values and their textual
    /// representations.
    unsafe pub struct NSNumberFormatter;
}

impl INSFormatter for NSNumberFormatter {}

#[interface_impl(NSFormatter)]
impl NSNumberFormatter {
    /* Configuring Formatter Behavior and Style
     */

    /// The formatter behavior of the receiver.
    #[property]
    pub fn formatter_behavior(&self) -> NSNumberFormatterBehavior {
        unsafe { msg_send![self.m_self(), formatterBehavior] }
    }

    /// The formatter behavior of the receiver.
    #[property]
    pub fn set_formatter_behavior(&mut self, behavior: NSNumberFormatterBehavior) {
        unsafe { msg_send![self.m_self(), setFormatterBehavior: behavior] }
    }

    /// Sets the default formatter behavior for new instances of
    /// [`NSNumberFormatter`].
    #[method]
    pub fn set_default_formatter_behavior(behavior: NSNumberFormatterBehavior) {
        unsafe { msg_send![Self::m_class(), setDefaultFormatterBehavior: behavior] }
    }

    /// Returns an [`NSNumberFormatterBehavior`] constant that indicates default
    /// formatter behavior for new instances of NSNumberFormatter.
    #[method]
    pub fn default_formatter_behavior() -> NSNumberFormatterBehavior {
        unsafe { msg_send![Self::m_class(), defaultFormatterBehavior] }
    }

    /// The number style used by the receiver.
    #[property]
    pub fn number_style(&self) -> NSNumberFormatterStyle {
        unsafe { msg_send![self.m_self(), numberStyle] }
    }

    /// The number style used by the receiver.
    #[property]
    pub fn set_number_style(&mut self, style: NSNumberFormatterStyle) {
        unsafe { msg_send![self.m_self(), setNumberStyle: style] }
    }

    /// Determines whether the receiver creates instances of [`super::NSDecimalNumber`]
    /// when it converts strings to number objects.
    #[property]
    pub fn generates_decimal_numbers(&self) {
        unsafe { msg_send![self.m_self(), generatesDecimalNumbers] }
    }

    /* Converting Between Numbers and Strings
     */

    /// Returns by reference a cell-content object after creating it from
    /// a range of characters in a given string.
    #[method]
    pub fn get_object_value_for_string_range_error(
        &self,
        obj: id,
        string: NSString,
        rangep: NSRange,
        error: NSError,
    ) -> bool {
        unsafe {
            to_bool(
                msg_send![self.m_self(), getObjectValue:obj forString:string range:rangep error: error],
            )
        }
    }

    /// Returns an [`NSNumber`] object created by parsing a given string.
    #[method]
    pub fn number_from_string(&self, string: NSString) -> NSNumber {
        unsafe { NSNumber::from_id(msg_send![self.m_self(), numberFromString: string]) }
    }

    /// Returns a string containing the formatted value of the provided number
    /// object.
    #[method]
    pub fn string_from_number(&self, number: NSNumber) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), stringFromNumber: number]) }
    }

    /// Returns a localized number string with the specified style.
    #[method]
    pub fn localized_string_from_number_number_style(
        num: NSNumber,
        nstyle: NSNumberFormatterStyle,
    ) -> NSString {
        unsafe {
            NSString::from_id(
                msg_send![Self::m_class(), localizedStringFromNumber: num numberStyle: nstyle],
            )
        }
    }

    /* Managing Localization of Numbers
     */

    /// Determines whether the dollar sign character ($), decimal separator
    /// character (.), and thousand separator character (,) are converted to
    /// appropriately localized characters as specified by the user’s
    /// localization preference.
    #[property]
    pub fn localizes_format(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), localizesFormat]) }
    }

    /// Sets whether the dollar sign character ($), decimal separator
    /// character (.), and thousand separator character (,) are converted to
    /// appropriately localized characters as specified by the user’s
    /// localization preference.
    #[property]
    pub fn set_localizes_format(&mut self, localizes_format: bool) {
        unsafe { msg_send![self.m_self(), setLocalizesFormat: localizes_format] }
    }

    /// The locale of the receiver.
    #[property]
    pub fn locale(&self) -> NSLocale {
        unsafe { NSLocale::from_id(msg_send![self.m_self(), locale]) }
    }

    /// Sets the locale of the receiver.
    #[property]
    pub fn set_locale(&mut self, locale: NSLocale) {
        unsafe { msg_send![self.m_self(), setLocale: locale] }
    }

    /* Configuring Rounding Behavior
     */

    /// The rounding behavior used by the receiver.
    #[property]
    pub fn rounding_behavior(&self) -> NSDecimalNumberHandler {
        unsafe { NSDecimalNumberHandler::from_id(msg_send![self.m_self(), roundingBehavior]) }
    }

    /// Set the rounding behavior used by the receiver.
    #[property]
    pub fn set_rounding_behavior(&mut self, behavior: NSNumberFormatterBehavior) {
        unsafe { msg_send![self.m_self(), setRoundingBehavior: behavior] }
    }

    /// The rounding increment used by the receiver.
    #[property]
    pub fn rounding_increment(&self) -> NSNumber {
        unsafe { NSNumber::from_id(msg_send![self.m_self(), roundingIncrement]) }
    }

    /// The rounding increment used by the receiver.
    #[property]
    pub fn set_rounding_increment(&mut self, number: NSNumber) {
        unsafe { msg_send![self.m_self(), setRoundingIncrement: number] }
    }

    /// The rounding mode used by the receiver.
    #[property]
    pub fn rounding_mode(&self) -> NSNumberFormatterRoundingMode {
        unsafe { msg_send![self.m_self(), roundingMode] }
    }

    /// The rounding mode used by the receiver.
    #[property]
    pub fn set_rounding_mode(&mut self, mode: NSNumberFormatterRoundingMode) {
        unsafe { msg_send![self.m_self(), setRoundingMode: mode] }
    }

    /* Configuring Integer and Fraction Digits
     */

    /// The minimum number of digits before the decimal separator.
    #[property]
    pub fn minimum_integer_digits(&self) -> UInt {
        unsafe { msg_send![self.m_self(), minimumIntegerDigits] }
    }

    /// Sets the minimum number of digits before the decimal separator.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_macios::objective_c_runtime::traits::PNSObject;
    /// use rust_macios::foundation::NSNumberFormatter;
    ///
    /// let mut number_formatter = NSNumberFormatter::m_new();
    ///
    /// number_formatter.set_minimum_integer_digits(0); // default
    /// assert_eq!(number_formatter.string_from_number(123.into()), "123");
    ///
    /// number_formatter.set_minimum_integer_digits(5);
    /// assert_eq!(number_formatter.string_from_number(123.into()), "00123");
    /// ```
    #[property]
    pub fn set_minimum_integer_digits(&mut self, min: UInt) {
        unsafe { msg_send![self.m_self(), setMinimumIntegerDigits: min] }
    }

    /// The maximum number of digits before the decimal separator.
    #[property]
    pub fn maximum_integer_digits(&self) -> UInt {
        unsafe { msg_send![self.m_self(), maximumIntegerDigits] }
    }

    /// The maximum number of digits before the decimal separator.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_macios::objective_c_runtime::traits::PNSObject;
    /// use rust_macios::foundation::NSNumberFormatter;
    ///
    /// let mut number_formatter = NSNumberFormatter::m_new();
    ///
    /// number_formatter.set_maximum_integer_digits(42); // default
    /// assert_eq!(number_formatter.string_from_number(12345.into()), "12345");
    ///
    /// number_formatter.set_maximum_integer_digits(3);
    /// assert_eq!(number_formatter.string_from_number(12345.into()), "345");
    /// ```
    #[property]
    pub fn set_maximum_integer_digits(&mut self, max: UInt) {
        unsafe { msg_send![self.m_self(), setMaximumIntegerDigits: max] }
    }

    /// The minimum number of digits after the decimal separator.
    #[property]
    pub fn minimum_fraction_digits(&self) -> UInt {
        unsafe { msg_send![self.m_self(), minimumFractionDigits] }
    }

    /// Sets the minimum number of digits after the decimal separator.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_macios::objective_c_runtime::traits::PNSObject;
    /// use rust_macios::foundation::NSNumberFormatter;
    ///
    /// let mut number_formatter = NSNumberFormatter::m_new();
    ///
    /// number_formatter.set_minimum_fraction_digits(0); // default
    /// assert_eq!(number_formatter.string_from_number(123.456.into()), "123");
    ///
    /// number_formatter.set_minimum_fraction_digits(5);
    /// assert_eq!(number_formatter.string_from_number(123.456.into()), "123.45600");
    /// ```
    #[property]
    pub fn set_minimum_fraction_digits(&mut self, min: UInt) {
        unsafe { msg_send![self.m_self(), setMinimumFractionDigits: min] }
    }

    /// The maximum number of digits after the decimal separator.
    #[property]
    pub fn maximum_fraction_digits(&self) -> UInt {
        unsafe { msg_send![self.m_self(), maximumFractionDigits] }
    }

    /// Sets the maximum number of digits after the decimal separator.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_macios::objective_c_runtime::traits::PNSObject;
    /// use rust_macios::foundation::NSNumberFormatter;
    ///
    /// let mut number_formatter = NSNumberFormatter::m_new();
    ///
    /// number_formatter.set_maximum_fraction_digits(0); // default
    /// assert_eq!(number_formatter.string_from_number(123.456.into()), "123");
    ///
    /// number_formatter.set_maximum_fraction_digits(3);
    /// assert_eq!(number_formatter.string_from_number(123.456789.into()), "123.457");
    /// ```
    #[property]
    pub fn set_maximum_fraction_digits(&mut self, max: UInt) {
        unsafe { msg_send![self.m_self(), setMaximumFractionDigits: max] }
    }

    /// A Boolean value indicating whether the formatter uses minimum and
    /// maximum significant digits when formatting numbers.
    #[property]
    pub fn uses_significant_digits(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), setUsesSignificantDigits]) }
    }

    /// A Boolean value indicating whether the formatter uses minimum and
    /// maximum significant digits when formatting numbers.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_macios::objective_c_runtime::traits::PNSObject;
    /// use rust_macios::foundation::NSNumberFormatter;
    ///
    /// let mut number_formatter = NSNumberFormatter::m_new();
    ///
    /// // Using significant digits
    /// number_formatter.set_uses_significant_digits(true);
    /// assert_eq!(number_formatter.string_from_number(12345678.into()), "12345700");
    /// assert_eq!(number_formatter.string_from_number(1234.5678.into()), "1234.57");
    /// assert_eq!(number_formatter.string_from_number(100.2345678.into()), "100.235");
    /// assert_eq!(number_formatter.string_from_number(1.230000.into()), "1.23");
    /// assert_eq!(number_formatter.string_from_number(0.00000123.into()), "0.00000123");
    ///
    /// // Using integer and fraction digits
    /// number_formatter.set_uses_significant_digits(false);
    /// assert_eq!(number_formatter.string_from_number(12345678.into()), "12345678");
    /// assert_eq!(number_formatter.string_from_number(1234.5678.into()), "1235");
    /// assert_eq!(number_formatter.string_from_number(100.2345678.into()), "100");
    /// assert_eq!(number_formatter.string_from_number(1.230000.into()), "1");
    /// assert_eq!(number_formatter.string_from_number(0.00000123.into()), "0");
    /// ```
    #[property]
    pub fn set_uses_significant_digits(&mut self, uses: bool) {
        unsafe { msg_send![self.m_self(), setUsesSignificantDigits: uses] }
    }

    /// The minimum number of significant digits for the number formatter.
    #[property]
    pub fn minimum_significant_digits(&self) -> UInt {
        unsafe { msg_send![self.m_self(), minimumSignificantDigits] }
    }

    /// Sets the minimum number of significant digits for the number formatter.
    #[property]
    pub fn set_minimum_significant_digits(&self, min: UInt) {
        unsafe { msg_send![self.m_self(), setMinimumSignificantDigits: min] }
    }

    /// The maximum number of significant digits for the number formatter.
    #[property]
    pub fn maximum_significant_digits(&self) -> UInt {
        unsafe { msg_send![self.m_self(), maximumSignificantDigits] }
    }

    /// Sets the maximum number of significant digits for the number formatter.
    #[property]
    pub fn set_maximum_significant_digits(&self, max: UInt) {
        unsafe { msg_send![self.m_self(), setMaximumSignificantDigits: max] }
    }

    /* Configuring Numeric Formats
     */

    /// The receiver’s format.
    #[property]
    pub fn format(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), format]) }
    }

    /// Sets the receiver’s format.
    #[property]
    pub fn set_format(&mut self, format: NSString) {
        unsafe { msg_send![self.m_self(), setFormat: format] }
    }

    /// The capitalization formatting context used when formatting a number.
    #[property]
    pub fn formatting_context(&self) -> NSFormattingContext {
        unsafe { msg_send![self.m_self(), formattingContext] }
    }

    /// Sets the capitalization formatting context used when formatting a number.
    #[property]
    pub fn set_formatting_context(&mut self, context: NSFormattingContext) {
        unsafe { msg_send![self.m_self(), setFormattingContext: context] }
    }

    /// The format width used by the receiver.
    #[property]
    pub fn format_width(&self) -> UInt {
        unsafe { msg_send![self.m_self(), formatWidth] }
    }

    /// Sets the format width used by the receiver.
    #[property]
    pub fn set_format_width(&mut self, width: UInt) {
        unsafe { msg_send![self.m_self(), setFormatWidth: width] }
    }

    /// The format the receiver uses to display negative values.
    #[property]
    pub fn negative_format(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), negativeFormat]) }
    }

    /// Sets the format the receiver uses to display negative values.
    #[property]
    pub fn set_negative_format(&mut self) {
        unsafe { msg_send![self.m_self(), setNegativeFormat] }
    }

    /// The format the receiver uses to display positive values.
    #[property]
    pub fn positive_format(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), positiveFormat]) }
    }

    /// Sets the format the receiver uses to display positive values.
    #[property]
    pub fn set_positive_format(&mut self, format: NSString) {
        unsafe { msg_send![self.m_self(), setPositiveFormat: format] }
    }

    /// The multiplier of the receiver.
    #[property]
    pub fn multiplier(&self) -> NSNumber {
        unsafe { NSNumber::from_id(msg_send![self.m_self(), multiplier]) }
    }

    /// Sets the multiplier of the receiver.
    #[property]
    pub fn set_multiplier(&mut self, multiplier: NSNumber) {
        unsafe { msg_send![self.m_self(), setMultiplier: multiplier] }
    }

    /* Configuring Numeric Symbols
     */

    /// The string used to represent a percent symbol.
    #[property]
    pub fn percent_symbol(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), percentSymbol]) }
    }

    /// Sets the string used to represent a percent symbol.
    #[property]
    pub fn set_percent_symbol(&mut self, symbol: NSString) {
        unsafe { msg_send![self.m_self(), setPercentSymbol: symbol] }
    }

    /// The string used to represent a per-mill (per-thousand) symbol.
    #[property]
    pub fn per_mill_symbol(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), perMillSymbol]) }
    }

    /// The string used to represent a per-mill (per-thousand) symbol.
    #[property]
    pub fn set_per_mill_symbol(&mut self, symbol: NSString) {
        unsafe { msg_send![self.m_self(), setPerMillSymbol: symbol] }
    }

    /// The string used to represent a minus sign.
    #[property]
    pub fn minus_sign(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), minusSign]) }
    }

    /// Sets the string used to represent a minus sign.
    #[property]
    pub fn set_minus_sign(&mut self, sign: NSString) {
        unsafe { msg_send![self.m_self(), setMinusSign: sign] }
    }

    /// The string used to represent a plus sign.
    #[property]
    pub fn plus_sign(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), plusSign]) }
    }

    /// Sets the string used to represent a plus sign.
    #[property]
    pub fn set_plus_sign(&mut self, sign: NSString) {
        unsafe { msg_send![self.m_self(), setPlusSign: sign] }
    }

    /// The string used to represent an exponent symbol.
    #[property]
    pub fn exponent_symbol(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), exponentSymbol]) }
    }

    /// Sets the string used to represent an exponent symbol.
    #[property]
    pub fn set_exponent_symbol(&mut self, sign: NSString) {
        unsafe { msg_send![self.m_self(), setExponentSymbol: sign] }
    }

    /// The string used to represent a zero symbol.
    #[property]
    pub fn zero_symbol(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), zeroSymbol]) }
    }

    /// Sets the string used to represent a zero symbol.
    #[property]
    pub fn set_zero_symbol(&mut self, sign: NSString) {
        unsafe { msg_send![self.m_self(), setZeroSymbol: sign] }
    }

    /// The string used to represent a nil symbol.
    #[property]
    pub fn nil_symbol(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), nilSymbol]) }
    }

    /// Sets the string used to represent a nil symbol.
    #[property]
    pub fn set_nil_symbol(&mut self, sign: NSString) {
        unsafe { msg_send![self.m_self(), setNilSymbol: sign] }
    }

    /// The string used to represent a NaN (“not a number”) value.
    #[property]
    pub fn not_a_number_symbol(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), notANumberSymbol]) }
    }

    /// Sets the string used to represent a NaN (“not a number”) value.
    #[property]
    pub fn set_not_a_number_symbol(&mut self, symbol: NSString) {
        unsafe { msg_send![self.m_self(), setNotANumberSymbol: symbol] }
    }

    /// The string used to represent a negative infinity symbol.
    #[property]
    pub fn negative_infinity_symbol(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), negativeInfinitySymbol]) }
    }

    /// Sets the string used to represent a negative infinity symbol.
    #[property]
    pub fn set_negative_infinity_symbol(&self, symbol: NSString) {
        unsafe { msg_send![self.m_self(), setNegativeInfinitySymbol: symbol] }
    }

    /// The string used to represent a positive infinity symbol.
    #[property]
    pub fn positive_infinity_symbol(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), positiveInfinitySymbol]) }
    }

    /// Sets the string used to represent a positive infinity symbol.
    #[property]
    pub fn set_positive_infinity_symbol(&self, symbol: NSString) {
        unsafe { msg_send![self.m_self(), setPositiveInfinitySymbol: symbol] }
    }

    /* Configuring the Format of Currency
     */

    /// The string used by the receiver as a local currency symbol.
    #[property]
    pub fn currency_symbol(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), currencySymbol]) }
    }

    /// Sets the string used by the receiver as a local currency symbol.
    #[property]
    pub fn set_currency_symbol(&mut self, symbol: NSString) {
        unsafe { msg_send![self.m_self(), setCurrencySymbol: symbol] }
    }

    /// The receiver’s currency code.
    #[property]
    pub fn currency_code(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), currencyCode]) }
    }

    /// Sets the receiver’s currency code.
    #[property]
    pub fn set_currency_code(&mut self, code: NSString) {
        unsafe { msg_send![self.m_self(), setCurrencyCode: code] }
    }

    /// The international currency symbol used by the receiver.
    #[property]
    pub fn international_currency_symbol(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), internationalCurrencySymbol]) }
    }

    /// Sets the international currency symbol used by the receiver.
    #[property]
    pub fn set_international_currency_symbol(&mut self, symbol: NSString) {
        unsafe { msg_send![self.m_self(), setInternationalCurrencySymbol: symbol] }
    }

    /// The currency grouping separator for the receiver.
    #[property]
    pub fn currency_grouping_separator(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), currencyGroupingSepator]) }
    }

    /* Configuring Numeric Prefixes and Suffixes
     */

    /// The string the receiver uses as the prefix for positive values.
    #[property]
    pub fn positive_prefix(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), positivePrefix]) }
    }

    /// Sets the string the receiver uses as the prefix for positive values.
    #[property]
    pub fn set_positive_prefix(&mut self, prefix: NSString) {
        unsafe { msg_send![self.m_self(), setPositivePrefix: prefix] }
    }

    /// The string the receiver uses as the suffix for positive values.
    #[property]
    pub fn positive_suffix(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), positiveSuffix]) }
    }

    /// Sets the string the receiver uses as the suffix for positive values.
    #[property]
    pub fn set_positive_suffix(&mut self, suffix: NSString) {
        unsafe { msg_send![self.m_self(), setPositiveSuffix: suffix] }
    }

    /// The string the receiver uses as a prefix for negative values.
    #[property]
    pub fn negative_prefix(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), negativePrefix]) }
    }

    /// Sets the string the receiver uses as a prefix for negative values.
    #[property]
    pub fn set_negative_prefix(&mut self, prefix: NSString) {
        unsafe { msg_send![self.m_self(), setNegativePrefix: prefix] }
    }

    /// The string the receiver uses as a suffix for negative values.
    #[property]
    pub fn negative_suffix(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), negativeSuffix]) }
    }

    /* Configuring the Display of Numeric Values
     */

    /// The text attributes to be used in displaying negative values.
    #[property]
    pub fn text_attributes_for_negative_values(&self) -> NSDictionary<NSString, id> {
        unsafe { NSDictionary::from_id(msg_send![self.m_self(), negativeAttributes]) }
    }

    /// Sets the text attributes to be used in displaying negative values.
    #[property]
    pub fn set_text_attributes_for_negative_values(
        &mut self,
        attributes: NSDictionary<NSString, id>,
    ) {
        unsafe { msg_send![self.m_self(), setNegativeAttributes: attributes] }
    }

    /// The text attributes to be used in displaying positive values.
    #[property]
    pub fn text_attributes_for_positive_values(&self) -> NSDictionary<NSString, id> {
        unsafe { NSDictionary::from_id(msg_send![self.m_self(), positiveAttributes]) }
    }

    /// Sets the text attributes to be used in displaying positive values.
    #[property]
    pub fn set_text_attributes_for_positive_values(
        &mut self,
        attributes: NSDictionary<NSString, id>,
    ) {
        unsafe { msg_send![self.m_self(), setPositiveAttributes: attributes] }
    }

    /// The attributed string that the receiver uses to display zero values.
    #[property]
    pub fn attributed_string_for_zero(&self) -> NSAttributedString {
        unsafe { NSAttributedString::from_id(msg_send![self.m_self(), zeroAttributedString]) }
    }

    /// Sets the attributed string that the receiver uses to display zero values.
    #[property]
    pub fn set_attributed_string_for_zero(&mut self, string: NSAttributedString) {
        unsafe { msg_send![self.m_self(), setZeroAttributedString: string] }
    }

    /// The text attributes used to display a zero value.
    #[property]
    pub fn text_attributes_for_zero(&self) -> NSDictionary<NSString, id> {
        unsafe { NSDictionary::from_id(msg_send![self.m_self(), zeroAttributes]) }
    }

    /// Sets the text attributes used to display a zero value.
    #[property]
    pub fn set_text_attributes_for_zero(&mut self, attributes: NSDictionary<NSString, id>) {
        unsafe { msg_send![self.m_self(), setZeroAttributes: attributes] }
    }

    /// The attributed string the receiver uses to display nil values.
    #[property]
    pub fn attributed_string_for_nil(&self) -> NSAttributedString {
        unsafe { NSAttributedString::from_id(msg_send![self.m_self(), nilAttributedString]) }
    }

    /// Sets the attributed string the receiver uses to display nil values.
    #[property]
    pub fn set_attributed_string_for_nil(&mut self, string: NSAttributedString) {
        unsafe { msg_send![self.m_self(), setNilAttributedString: string] }
    }

    /// The text attributes used to display the nil symbol.
    #[property]
    pub fn text_attributes_for_nil(&self) -> NSDictionary<NSString, id> {
        unsafe { NSDictionary::from_id(msg_send![self.m_self(), nilAttributes]) }
    }

    /// Sets the text attributes used to display the nil symbol.
    #[property]
    pub fn set_text_attributes_for_nil(&mut self, attributes: NSDictionary<NSString, id>) {
        unsafe { msg_send![self.m_self(), setNilAttributes: attributes] }
    }

    /// The attributed string the receiver uses to display “not a number” values.
    #[property]
    pub fn attributed_string_for_not_a_number(&self) -> NSAttributedString {
        unsafe { NSAttributedString::from_id(msg_send![self.m_self(), notANumberAttributedString]) }
    }

    /// Sets the attributed string the receiver uses to display “not a number” values.
    #[property]
    pub fn set_attributed_string_for_not_a_number(&mut self, string: NSAttributedString) {
        unsafe { msg_send![self.m_self(), setNotANumberAttributedString: string] }
    }

    /// The text attributes used to display the NaN (“not a number”) string.
    #[property]
    pub fn text_attributes_for_not_a_number(&self) -> NSDictionary<NSString, id> {
        unsafe { NSDictionary::from_id(msg_send![self.m_self(), notANumberAttributes]) }
    }

    /// Sets the text attributes used to display the NaN (“not a number”) string.
    #[property]
    pub fn set_text_attributes_for_not_a_number(&mut self, attributes: NSDictionary<NSString, id>) {
        unsafe { msg_send![self.m_self(), setNotANumberAttributes: attributes] }
    }

    /// The text attributes used to display the positive infinity symbol.
    #[property]
    pub fn text_attributes_for_positive_infinity(&self) -> NSDictionary<NSString, id> {
        unsafe { NSDictionary::from_id(msg_send![self.m_self(), positiveInfinityAttributes]) }
    }

    /// Sets the text attributes used to display the positive infinity symbol.
    #[property]
    pub fn set_text_attributes_for_positive_infinity(
        &mut self,
        attributes: NSDictionary<NSString, id>,
    ) {
        unsafe { msg_send![self.m_self(), setPositiveInfinityAttributes: attributes] }
    }

    /// The text attributes used to display the negative infinity symbol.
    #[property]
    pub fn text_attributes_for_negative_infinity(&self) -> NSDictionary<NSString, id> {
        unsafe { NSDictionary::from_id(msg_send![self.m_self(), negativeInfinityAttributes]) }
    }

    /// Sets the text attributes used to display the negative infinity symbol.
    #[property]
    pub fn set_text_attributes_for_negative_infinity(
        &mut self,
        attributes: NSDictionary<NSString, id>,
    ) {
        unsafe { msg_send![self.m_self(), setNegativeInfinityAttributes: attributes] }
    }

    /* Configuring Separators and Grouping Size
     */

    /// The string used by the receiver for a grouping separator.
    #[property]
    pub fn grouping_separator(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), groupingSeparator]) }
    }

    /// Sets the string used by the receiver for a grouping separator.
    #[property]
    pub fn set_grouping_separator(&mut self, separator: NSString) {
        unsafe { msg_send![self.m_self(), setGroupingSeparator: separator] }
    }

    /// Determines whether the receiver displays the group separator.
    #[property]
    pub fn uses_grouping_separator(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), usesGroupingSeparator]) }
    }

    /// Sets whether the receiver displays the group separator.
    #[property]
    pub fn set_uses_grouping_separator(&mut self, flag: bool) {
        unsafe { msg_send![self.m_self(), setUsesGroupingSeparator: flag] }
    }

    /// The character the receiver uses as a thousand separator.
    #[property]
    pub fn thousand_separator(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), thousandSeparator]) }
    }

    /// Sets the character the receiver uses as a thousand separator.
    #[property]
    pub fn set_thousand_separator(&mut self, separator: NSString) {
        unsafe { msg_send![self.m_self(), setThousandSeparator: separator] }
    }

    /// Determines whether the receiver uses thousand separators.
    #[property]
    pub fn uses_thousand_separators(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), usesThousandSeparators]) }
    }

    /// Sets whether the receiver uses thousand separators.
    #[property]
    pub fn set_uses_thousand_separators(&mut self, flag: bool) {
        unsafe { msg_send![self.m_self(), setUsesThousandSeparators: flag] }
    }

    /// The character the receiver uses as a decimal separator.
    #[property]
    pub fn decimal_separator(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), decimalSeparator]) }
    }

    /// Determines whether the receiver always shows the decimal separator, even for integer numbers.
    #[property]
    pub fn always_show_decimal_separator(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), alwaysShowDecimalSeparator]) }
    }

    /// Sets whether the receiver always shows the decimal separator, even for integer numbers.
    #[property]
    pub fn set_always_show_decimal_separator(&mut self, flag: bool) {
        unsafe { msg_send![self.m_self(), setAlwaysShowDecimalSeparator: flag] }
    }

    /// The string used by the receiver as a currency decimal separator.
    #[property]
    pub fn currency_decimal_separator(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), currencyDecimalSeparator]) }
    }

    /// Sets the string used by the receiver as a currency decimal separator.
    #[property]
    pub fn set_currency_decimal_separator(&mut self, separator: NSString) {
        unsafe { msg_send![self.m_self(), setCurrencyDecimalSeparator: separator] }
    }

    /// The grouping size of the receiver.
    #[property]
    pub fn grouping_size(&self) -> Int {
        unsafe { msg_send![self.m_self(), groupingSize] }
    }

    /// Sets the grouping size of the receiver.
    #[property]
    pub fn set_grouping_size(&mut self, size: Int) {
        unsafe { msg_send![self.m_self(), setGroupingSize: size] }
    }

    /// The secondary grouping size of the receiver.
    #[property]
    pub fn secondary_grouping_size(&self) -> Int {
        unsafe { msg_send![self.m_self(), secondaryGroupingSize] }
    }

    /// Sets the secondary grouping size of the receiver.
    #[property]
    pub fn set_secondary_grouping_size(&mut self, size: Int) {
        unsafe { msg_send![self.m_self(), setSecondaryGroupingSize: size] }
    }

    /* Managing the Padding of Numbers
     */

    /// The string that the receiver uses to pad numbers in the formatted string representation.
    #[property]
    pub fn padding_character(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), paddingCharacter]) }
    }

    /// Sets the string that the receiver uses to pad numbers in the formatted string representation.
    #[property]
    pub fn set_padding_character(&mut self, padding: NSString) {
        unsafe { msg_send![self.m_self(), setPaddingCharacter: padding] }
    }

    /// The padding position used by the receiver.
    #[property]
    pub fn padding_position(&self) -> NSNumberFormatterPadPosition {
        unsafe { msg_send![self.m_self(), paddingPosition] }
    }

    /// Sets the padding position used by the receiver.
    #[property]
    pub fn set_padding_position(&mut self, position: NSNumberFormatterPadPosition) {
        unsafe { msg_send![self.m_self(), setPaddingPosition: position] }
    }

    /* Managing Input and Output Attributes
     */

    /// Determines whether the receiver allows as input floating-point values (that is, values that include the period character [.]).
    #[property]
    pub fn allows_floats(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), allowsFloats]) }
    }

    /// Sets whether the receiver allows as input floating-point values (that is, values that include the period character [.]).
    #[property]
    pub fn set_allows_floats(&mut self, flag: bool) {
        unsafe { msg_send![self.m_self(), setAllowsFloats: flag] }
    }

    /// The lowest number allowed as input by the receiver.
    #[property]
    pub fn minimum(&self) -> NSNumber {
        unsafe { NSNumber::from_id(msg_send![self.m_self(), minimum]) }
    }

    /// Sets the lowest number allowed as input by the receiver.
    #[property]
    pub fn set_minimum(&mut self, number: NSNumber) {
        unsafe { msg_send![self.m_self(), setMinimum: number] }
    }

    /// The highest number allowed as input by the receiver.
    #[property]
    pub fn maximum(&self) -> NSNumber {
        unsafe { NSNumber::from_id(msg_send![self.m_self(), maximum]) }
    }

    /// Sets the highest number allowed as input by the receiver.
    #[property]
    pub fn set_maximum(&mut self, number: NSNumber) {
        unsafe { msg_send![self.m_self(), setMaximum: number] }
    }

    /* Managing Leniency Behavior
     */

    /// Determines whether the receiver will use heuristics to guess at the number which is intended by a string.
    #[property]
    pub fn lenient(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isLenient]) }
    }

    /// Sets whether the receiver will use heuristics to guess at the number which is intended by a string.
    #[property]
    pub fn set_lenient(&mut self, flag: bool) {
        unsafe { msg_send![self.m_self(), setLenient: flag] }
    }

    /* Managing the Validation of Partial Numeric Strings
     */

    /// Determines whether partial string validation is enabled for the receiver.
    #[property]
    pub fn partial_string_validation_enabled(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isPartialStringValidationEnabled]) }
    }

    /// Sets whether partial string validation is enabled for the receiver.
    #[property]
    pub fn set_partial_string_validation_enabled(&mut self, flag: bool) {
        unsafe { msg_send![self.m_self(), setPartialStringValidationEnabled: flag] }
    }
}
