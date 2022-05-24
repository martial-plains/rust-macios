#![allow(non_snake_case)]

use std::{ops::Range, sync::Arc};

use libc::{
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint,
    c_ulong, c_ulonglong, c_ushort,
};
use objc::runtime::Object;

use crate::{
    foundation::String,
    id,
    objective_c_runtime::traits::{t_NSObject, t_NSValue},
};

use super::{
    key::NSLocaleKey, unichar, Array, CompareOptions, ComparisonResult, Encoding, Int,
    LanguageDirection, NSDecimalNumber, NSRoundingMode, StringTransform, UInt,
};

/// A static, plain-text Unicode string object.
pub trait t_NSString: t_NSObject {
    /// Creates a new `NSString`
    fn new() -> Self;

    /// In some cases, we want to wrap a system-provided NSString without retaining it.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it dereferrences a raw pointer.
    unsafe fn fromRetained(object: id) -> Self;

    /// Utility method for checking whether an `NSObject` is an `NSString`.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it dereferrences a raw pointer.
    unsafe fn is(obj: id) -> bool;

    /// Returns the UTF8 bytes for this `NSString`.
    fn bytes(&self) -> *const c_char;

    /// Gets the proper byte length for this `NSString` (the UTF8 variant).
    fn bytesLen(&self) -> UInt;

    /// Convert this `NSString` into a `&str`.
    fn asStr(&self) -> &str;

    /* Creating and Initializing Strings
     */

    /// Returns an initialized NSString object that contains no characters.
    fn init() -> Self;

    /// Returns an initialized NSString object containing a given number of bytes from a given
    /// buffer of bytes interpreted in a given encoding.
    ///
    /// # Arguments
    ///
    /// * `bytes` - A buffer of bytes interpreted in the encoding specified by encoding.
    ///
    /// * `len` - The number of bytes to use from bytes.
    ///
    /// * `encoding` - The character encoding applied to bytes. For possible values, see NSStringEncoding.
    ///
    /// # Returns
    ///
    /// An initialized NSString object containing length bytes from bytes interpreted using the encoding
    /// encoding. The returned object may be different from the original receiver.
    fn initWithBytesLenEncoding(bytes: *const c_char, len: UInt, encoding: Encoding) -> Self;

    /// Returns an initialized NSString object that contains a given number of bytes from
    /// a given buffer of bytes interpreted in a given encoding, and optionally frees the buffer.
    ///
    /// # Arguments
    ///
    /// * `s` - The string slice to create the `NSString` from.
    fn initWithNoCpyStr(s: &str) -> Self;

    /// Returns an initialized NSString object that contains a given number of characters from
    /// a given C array of UTF-16 code units.
    ///
    /// # Arguments
    ///
    /// * `characters` - A C array of UTF-16 code units; the value must not be NULL.
    /// * `length` - A C array of UTF-16 code units; the value must not be NULL.
    ///
    /// # Returns
    ///
    /// An initialized NSString object containing length characters from characters.
    fn initWithCharactersLen(characters: *const unichar, len: UInt) -> Self;

    /// Returns an NSString object initialized by copying the characters from another given string.
    ///
    /// # Arguments
    ///
    /// * `s` - The string from which to copy characters. This value must not be nil.
    ///
    /// # Returns
    ///
    /// An NSString object initialized by copying the characters from s.
    fn initWithStr(s: &str) -> Self;

    /* Getting a String’s Length
     */

    /// The number of Unicode characters in this string.
    fn length(&self) -> Int;

    /// Returns the number of bytes required to store the receiver in a given encoding.
    ///
    /// # Arguments
    ///
    /// * `enc` - The encoding for which to determine the receiver's length.
    ///
    /// # Returns
    ///
    /// The number of bytes required to store the receiver in the encoding enc in a
    /// non-external representation. The length does not include space for a terminating
    /// NULL character. Returns 0 if the specified encoding cannot be used to convert
    /// the receiver or if the amount of memory required for storing the results of the
    /// encoding conversion would exceed NSIntegerMax.
    fn lengthOfBytesUsingEncoding(&self, enc: Encoding) -> Int;

    /// Returns the maximum number of bytes needed to store the receiver in a given encoding.
    ///
    /// # Arguments
    ///
    /// * `enc` - The encoding for which to determine the receiver's length.
    ///
    /// # Returns
    ///
    /// The maximum number of bytes needed to store the receiver in encoding in a non-external
    /// representation. The length does not include space for a terminating NULL character.
    /// Returns 0 if the amount of memory required for storing the results of the encoding
    /// conversion would exceed NSIntegerMax.
    fn maximumLengthOfBytesUsingEncoding(&self, enc: Encoding) -> Int;

    /* Getting Characters and Bytes
     */

    /// Returns the character at a given UTF-16 code unit index.
    ///
    /// # Arguments
    ///
    /// * `index` - The character at the array position given by `index`.
    ///
    /// # Returns
    ///
    /// The character at the array position given by index.
    fn characterAtIndex(&self, index: Int) -> char;

    /* Identifying and Comparing Strings
     */

    /// Returns the result of invoking compare:options: with NSCaseInsensitiveSearch as the only option.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare.
    ///
    /// # Returns
    ///
    /// Returns an `ComparisonResult` value that indicates the lexical ordering.
    /// `OrderedAscending` the receiver precedes `string` in lexical ordering,
    /// `OrderedSame` the receiver and `string` are equivalent in lexical value,
    /// and `OrderedDescending` if the receiver follows `string`.
    fn caseInsensitiveCompare<T>(&self, string: T) -> ComparisonResult
    where
        T: Into<String>;

    /// Compares the string with a given string using a case-insensitive, localized, comparison.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare.
    ///
    /// # Returns
    ///
    /// Returns an `ComparisonResult` value that indicates the lexical ordering.
    /// `OrderedAscending` the receiver precedes `string` in lexical ordering,
    /// `OrderedSame` the receiver and `string` are equivalent in lexical value,
    /// and `OrderedDescending` if the receiver follows `string`
    fn localizedCaseInsensitiveCompare<T>(&self, string: T) -> ComparisonResult
    where
        T: Into<String>;

    /// Returns the result of invoking compare:options:range: with no options
    /// and the receiver’s full extent as the range.
    ///
    /// # Arguments
    ///
    /// * `string` - The string with which to compare the receiver.
    ///
    /// # Safety
    ///
    /// This value must not be nil. If this value is nil, the behavior is
    /// undefined and may change in future versions of macOS.
    ///
    /// # Returns
    ///
    /// Returns an `ComparisonResult` value that indicates the lexical ordering.
    /// `OrderedAscending` the receiver precedes `string` in lexical ordering,
    /// `OrderedSame` the receiver and `string` are equivalent in lexical value,
    /// and `OrderedDescending` if the receiver follows `string`
    fn compare<T>(&self, string: T) -> ComparisonResult
    where
        T: Into<String>;

    /// Returns the result of invoking compare:options:range: with no options
    /// and the receiver’s full extent as the range.
    ///
    /// # Arguments
    ///
    /// * `string` - The string with which to compare the receiver.
    ///
    /// Returns an `ComparisonResult` value that indicates the lexical ordering.
    /// `OrderedAscending` the receiver precedes `string` in lexical ordering,
    /// `OrderedSame` the receiver and `string` are equivalent in lexical value,
    /// and `OrderedDescending` if the receiver follows `string`
    fn localizedCompare<T>(&self, string: T) -> ComparisonResult
    where
        T: Into<String>;

    /// Compares strings as sorted by the Finder.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare with the receiver.
    ///
    /// # Returns
    ///
    /// The result of the comparison.
    fn localized_standard_compare<T>(&self, string: T) -> ComparisonResult
    where
        T: Into<String>;

    /// Compares the string with the specified string using the given options.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare with the receiver.
    ///
    /// * `mask` - The options for the comparison.
    ///
    /// # Returns
    ///
    /// Returns an `ComparisonResult` value that indicates the lexical ordering.
    /// `OrderedAscending` the receiver precedes `string` in lexical ordering,
    /// `OrderedSame` the receiver and `string` are equivalent in lexical value,
    /// and `OrderedDescending` if the receiver follows `string`
    fn compareWithOptions<T>(&self, string: T, mask: CompareOptions) -> ComparisonResult
    where
        T: Into<String>;

    /// Returns the result of invoking compare(_:options:range:locale:) with a nil locale.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare with the receiver.
    /// * `mask` - The options for the comparison.
    /// * `range` - The range of the receiver to compare.
    ///
    /// # Returns
    ///
    /// Returns an `ComparisonResult` value that indicates the lexical ordering.
    /// `OrderedAscending` the receiver precedes `string` in lexical ordering,
    /// `OrderedSame` the receiver and `string` are equivalent in lexical value,
    /// and `OrderedDescending` if the receiver follows `string`
    fn compareWithOptionsRange<T>(
        &self,
        string: T,
        options: CompareOptions,
        range: Range<UInt>,
    ) -> ComparisonResult
    where
        T: Into<String>;

    /// Compares the string using the specified options and returns the lexical ordering for the range.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare with the receiver.
    /// * `mask` - The options for the comparison.
    /// * `range` - The range of the receiver to compare.
    /// * `locale` - The locale to use for the comparison.
    ///
    /// # Returns
    ///
    /// Returns an `ComparisonResult` value that indicates the lexical ordering.
    /// `OrderedAscending` the receiver precedes `string` in lexical ordering,
    /// `OrderedSame` the receiver and `string` are equivalent in lexical value,
    /// and `OrderedDescending` if the receiver follows `string`
    fn compareWithOptionsRangeLocale<T>(
        &self,
        string: T,
        options: CompareOptions,
        range: Range<UInt>,
        locale: super::Locale,
    ) -> ComparisonResult
    where
        T: Into<String>;

    /// Returns a Boolean value that indicates whether a given string matches the beginning characters of the receiver.
    ///
    /// # Arguments
    ///
    /// * `prefix` - The prefix to search for.
    ///
    /// # Returns
    ///
    /// Returns `true` if the string begins with the prefix, otherwise `false`.
    fn hasPrefix<T>(&self, prefix: T) -> bool
    where
        T: Into<String>;

    /// Returns a Boolean value that indicates whether a given string matches the ending characters of the receiver.
    ///
    /// # Arguments
    ///
    /// * `suffix` - The suffix to search for.
    ///
    /// # Returns
    ///
    /// Returns `true` if the string ends with the suffix, otherwise `false`.
    fn hasSuffix<T>(&self, suffix: T) -> bool
    where
        T: Into<String>;

    /// Returns a Boolean value that indicates whether a given string is equal to the receiver using a literal Unicode-based comparison.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare with the receiver.
    ///
    /// # Returns
    ///
    /// Returns `true` if the string is equal to the receiver, otherwise `false`.
    fn isEqualTo<T>(&self, string: T) -> bool
    where
        T: Into<String>;

    /* Combining Strings
     */

    /// Returns a new string made by appending a given string to the receiver.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to append to the receiver. This value must not be nil.
    fn appending<T>(&self, string: T) -> String
    where
        T: Into<String>;

    /// Returns a new string formed from the receiver by either removing characters from the end, or by appending as many occurrences as necessary of a given pad string.
    ///
    /// # Arguments
    ///
    /// * `new_length` - The number of characters to be contained in the new string.
    /// * `pad` - The string to use for padding.
    /// * `starting_at` - The index in `pad_string` from which to start padding.
    ///
    /// # Returns
    ///
    /// A new string formed from the receiver by either removing characters from the end, or by appending as many occurrences of `pad_string` as necessary.
    fn padding<T>(&self, new_length: Int, pad_string: T, starting_at: Int) -> String
    where
        T: Into<String>;

    /* Finding Characters and Substrings
     */

    /// Returns a boolean value indicating whether the string contains a given string by performing a case-sensitive, locale-unaware search.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to search for.
    ///
    /// # Returns
    ///
    /// Returns `true` if `string` is contained in the receiver, otherwise `false`.
    fn contains<T>(&self, other: T) -> bool
    where
        T: Into<String>;

    /* Changing Case
     */

    /// A lowercase representation of the string.
    fn lowercased(&self) -> String;

    /// Returns a version of the string with all letters converted to lowercase,
    /// taking into account the current locale.
    fn localizedLowercase(&self) -> String;

    /// An uppercase representation of the string.
    fn uppercased(&self) -> String;

    /// Returns a version of the string with all letters converted to uppercase,
    /// taking into account the current locale.
    fn localizedUppercase(&self) -> String;

    /// A capitalized representation of the string.
    fn capitalized(&self) -> String;

    /// Returns a capitalized representation of the receiver using the current
    /// locale.
    fn localizedCapitalized(&self) -> String;

    /* Transforming Strings
     */

    /// Returns a new string made by appending to the receiver a given string.
    ///
    /// # Arguments
    ///
    /// * `transform` - The `StringTransform` to apply.
    /// * `reverse` - If `true`, the transformation is applied in reverse.
    fn stringByApplyingTransform(
        &mut self,
        transform: StringTransform,
        reverse: bool,
    ) -> Option<String>;
}

/// The group of methods that are used with `NSNumber` objects.
pub trait t_NSNumber: t_NSValue {
    /* Creating an NSNumber Object
     */

    /// Creates and returns an NSNumber object containing a given value, treating it as a BOOL.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithBool(value: bool) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as a signed char.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithChar(value: c_schar) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as a double.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithDouble(value: c_double) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as a float.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithFloat(value: c_float) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as a signed int.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithInt(value: c_int) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an NSInteger.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithInteger(value: Int) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as a signed long.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithLong(value: c_long) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as a signed long long.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithLongLong(value: c_longlong) -> Self;

    /// Creates and returns an NSNumber object containing value, treating it as a signed short.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithShort(value: c_short) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned char.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithUnsignedChar(value: c_uchar) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned int.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithUnsignedInt(value: c_uint) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an NSUInteger.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithUnsignedInteger(value: UInt) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned long.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithUnsignedLong(value: c_ulong) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned long long.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithUnsignedLongLong(value: c_ulonglong) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned short.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithUnsignedShort(value: c_ushort) -> Self;

    /* Initializing an NSNumber Object
     */

    /// Returns an NSNumber object initialized to contain a given value, treated as a BOOL.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    fn initWithBool(&self, value: bool) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as a signed char.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithChar(&self, value: c_schar) -> Self;

    /// Returns an NSNumber object initialized to contain value, treated as a double.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithDouble(&self, value: c_double) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as a float.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithFloat(&self, value: c_float) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as a signed int.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithInt(&self, value: c_int) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an NSInteger.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithInteger(&self, value: Int) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as a signed long.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithLong(&self, value: c_long) -> Self;

    /// Returns an NSNumber object initialized to contain value, treated as a signed long long.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithLongLong(&self, value: c_longlong) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as a signed short.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithShort(&self, value: c_short) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned char.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithUnsignedChar(&self, value: c_uchar) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned int.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithUnsignedInt(&self, value: c_uint) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an NSUInteger.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithUnsignedInteger(&self, value: c_uint) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned long.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithUnsignedLong(&self, value: c_ulong) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned long long.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithUnsignedLongLong(&self, value: c_ulonglong) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned short.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithUnsignedShort(&self, value: c_ushort) -> Self;

    /* Accessing Numeric Values
     */

    /// The number object's value expressed as a Boolean value.
    fn boolValue(&self) -> bool;

    /// The number object's value expressed as a char.
    fn charValue(&self) -> c_schar;

    /// The number object's value expressed as an NSDecimal structure.
    //// fn decimalValue(&self) -> NSDecimal;

    /// The number object's value expressed as a double, converted as necessary.
    fn doubleValue(&self) -> c_double;

    /// The number object's value expressed as a float, converted as necessary.
    fn floatValue(&self) -> c_float;

    /// The number object's value expressed as an int, converted as necessary.
    fn intValue(&self) -> c_int;

    /// The number object's value expressed as an NSInteger object, converted as necessary.
    fn integerValue(&self) -> Int;

    /// The number object’s value expressed as a long long, converted as necessary.
    fn longLongValue(&self) -> c_longlong;

    /// The number object’s value expressed as a long, converted as necessary.
    fn longValue(&self) -> c_long;

    /// The number object's value expressed as a short, converted as necessary.
    fn shortValue(&self) -> c_short;

    /// The number object's value expressed as an unsigned char, converted as necessary.
    fn unsignedCharValue(&self) -> c_uchar;

    /// The number object's value expressed as an NSUInteger object, converted as necessary.
    fn unsignedIntegerValue(&self) -> UInt;

    /// The number object's value expressed as an unsigned int, converted as necessary.
    fn unsignedIntValue(&self) -> c_uint;

    /// The number object’s value expressed as an unsigned long long, converted as necessary.
    fn unsignedLongLongValue(&self) -> c_ulonglong;

    /// The number object's value expressed as an unsigned long, converted as necessary.
    fn unsignedLongValue(&self) -> c_ulong;

    /// The number object's value expressed as an unsigned short, converted as necessary.
    fn unsignedShortValue(&self) -> c_ushort;

    /* Retrieving String Representations
     */

    /// Returns a string that represents the contents of the number object for a given locale.
    ///
    /// # Arguments
    ///
    /// * `locale` - The locale to use to format the number.
    ///
    /// # Returns
    ///
    /// A string that represents the contents of the number object formatted using the locale information in `locale`.
    fn descriptionWithLocale(&self, locale: super::Locale) -> String;

    /// The number object's value expressed as a human-readable string.
    fn stringValue(&self) -> String;

    /* Comparing NSNumber Objects
     */

    /// Returns an NSComparisonResult value that indicates whether the number object’s value is greater than, equal to, or less than a given number.
    ///
    /// # Arguments
    ///
    /// * `other` - The number to compare to the number object’s value.
    fn compare(&self, other: &Self) -> ComparisonResult;

    /// Returns a Boolean value that indicates whether the number object’s value and a given number are equal.
    ///
    /// # Arguments
    ///
    /// * `other` - The number to compare to the number object’s value.
    fn isEqualToNumber(&self, other: &Self) -> bool;
}

/// The `Locale` class provides information about the user’s locale and formatting preferences.
pub trait t_NSLocale: t_NSObject {
    /* Initializing a Locale
     */

    /// Initializes a locale using a given locale identifier.
    fn initWithLocaleIdentifier<T>(locale_identifier: T) -> Self
    where
        T: Into<String>;

    /* Getting the User's Locale
     */

    /// A locale which tracks the user’s current preferences.
    fn autoUpdatingCurrent(&self) -> super::Locale;

    /// A locale representing the user's region settings at the time the property is read.
    fn current() -> super::Locale;

    /// A locale representing the generic root values with little localization.
    fn system() -> super::Locale;

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
    fn displayNameForKeyValue<T>(&self, key: NSLocaleKey, value: T) -> Option<String>
    where
        T: Into<String>;

    /* Getting the User's Preferred Languages
     */

    /// An ordered list of the user's preferred languages.
    fn preferredLanguages() -> Array<String>;

    /* Getting Line and Character Direction for a Language
     */

    /// Returns the direction of the sequence of characters in a line for the specified ISO language code.
    fn characterDirectionForLanguage<T>(&self, iso_language_code: T) -> LanguageDirection
    where
        T: Into<String>;

    /// Returns the direction of the sequence of lines for the specified ISO language code.
    fn lineDirectionForLanguage<T>(&self, iso_language_code: T) -> LanguageDirection
    where
        T: Into<String>;
}

/// A static ordered collection of objects.
pub trait t_NSArray<T> {
    /// Creates a new `Array` from a raw pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    fn new(ptr: *mut Object) -> Self;

    /// Creates a new array with the specified capacity.
    ///
    /// # Arguments
    ///
    /// * `objects` - Collection of objects to make into an `Array`.
    ///
    /// # Returns
    ///
    /// A new array with the specified capacity.
    fn fromObjects(objects: &[T]) -> Self
    where
        T: t_NSObject;

    /// In some cases, we're vended an `NSArray` by the system, and it's ideal to not retain that.
    /// This handles that edge case.
    ///
    /// # Arguments
    ///
    /// * `array` - The `NSArray` to dereference.
    ///
    /// # Returns
    ///
    /// The dereferenced `NSArray`.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it might dereference a raw pointer.
    unsafe fn fromRetained(array: id) -> Self;

    /* Querying an Array
     */

    /// Returns a Boolean value that indicates whether a given object is present in the array.
    ///
    /// # Arguments
    ///
    /// * `object` - An object to look for in the array..
    ///
    /// # Returns
    ///
    /// A Boolean value that indicates whether `object` is present in the array.
    ///
    fn contains(&self, object: T) -> bool
    where
        T: t_NSObject;

    /// The number of objects in the array.
    fn count(&self) -> UInt;

    /// The first object in the array.
    fn firstObject(&self) -> Option<T>
    where
        T: t_NSObject;

    /// The last object in the array.
    fn lastObject(&self) -> Option<T>
    where
        T: t_NSObject;

    /// The object at the specified index.
    fn objectAt(&self, index: UInt) -> T
    where
        T: t_NSObject;

    /// The index of the specified object.
    fn objectAtIndexedSubscript(&self, index: UInt) -> Option<id>;

    /* Finding Objects in an Array
     */

    /// Returns the lowest index whose corresponding array value is equal to a given object.
    fn indexOf(&self, object: T) -> UInt
    where
        T: t_NSObject;

    /// Returns the lowest index within a specified range whose corresponding array value is equal to a given object .
    fn indexOfObjectInRange(&self, object: T, range: Range<UInt>) -> UInt
    where
        T: t_NSObject;

    /// Returns the lowest index whose corresponding array value is identical to a given object.
    fn indexOfObjectIdenticalTo(&self, object: T) -> UInt
    where
        T: t_NSObject;

    /// Returns the lowest index within a specified range whose corresponding array value is equal to a given object .
    fn indexOfObjectIdenticalToInRange(&self, object: T, range: Range<UInt>) -> UInt
    where
        T: t_NSObject;

    /* Comparing Arrays
     */

    /// Returns the first object contained in the receiving array that’s equal to an object in another given array.
    fn firstObjectCommonWith(&self, other: &Array<T>) -> Option<T>
    where
        T: t_NSObject;

    /// Compares the receiving array to another array.
    fn isEqualTo(&self, other: &Array<T>) -> bool
    where
        T: t_NSObject;

    /* Deriving New Arrays
     */

    /// Returns a new array that is a copy of the receiving array with a given object added to the end.
    fn adding(&self, object: T) -> Array<T>
    where
        T: t_NSObject;

    /// Returns a new array that is a copy of the receiving array with the objects contained in another array added to the end.
    fn addingObjects(&self, objects: &Array<T>) -> Array<T>
    where
        T: t_NSObject;

    /// Returns a new array containing the receiving array’s elements that fall within the limits specified by a given range.
    fn subarrayWithRange(&self, range: Range<UInt>) -> Array<T>
    where
        T: t_NSObject;
    /* Creating a Description
     */

    /// A string that represents the contents of the array, formatted as a property list.

    /// Returns a string that represents the contents of the array, formatted as a property list.
    fn descriptionWithLocale(&self, locale: &super::Locale) -> String;

    /// Returns a string that represents the contents of the array, formatted as a property list.
    fn descriptionWithLocaleIndent(&self, locale: &super::Locale, indent: UInt) -> String;

    /* Rust Conversions
     */

    /// Returns an iterator over the objects in the array.
    fn iter(&self) -> super::array::iter::Iter<'_, T>
    where
        T: t_NSObject;
}

/// A mutable, static ordered collection of objects.
pub trait t_NSMutableArray<T>: t_NSArray<T> {
    /// Creates a new `MutableArray`.
    fn new() -> Self;

    /// The number of objects in the array.
    fn count(&self) -> UInt;

    /* Creating and Initializing a Mutable Array
     */

    /// Creates and returns an NSMutableArray object with enough allocated memory to initially hold a given number of objects.
    fn arrayWithCapacity(capacity: usize) -> Self;

    /// Creates and returns a mutable array containing the contents of the file specified by the given path.
    fn array_with_contents_of_file<S>(path: S) -> Self
    where
        S: Into<String>;

    /// Creates and returns a mutable array containing the contents specified by a given URL.
    fn arrayWithContentsOfUrl<S>(url: S) -> Self
    where
        S: Into<String>;

    /// Returns an array, initialized with enough memory to initially hold a given number of objects.
    fn initWithCapacity(capacity: UInt) -> Self;

    /// Initializes a newly allocated mutable array with the contents of the file specified by a given path
    fn initWithContentsOfFile<S>(&mut self, path: S) -> bool
    where
        S: Into<String>;

    /* Adding Objects
     */

    /// Inserts a given object at the end of the array.
    fn add(&mut self, object: &T);

    /// Adds the objects contained in another given array to the end of the receiving array’s content.
    fn addObjectsFromArray(&mut self, other_array: &Array<T>);

    /// Inserts a given object into the array’s contents at a given index.
    fn insert(&mut self, index: UInt, object: &T);

    /* Removing Objects
     */

    /// Empties the array of all its elements.
    fn removeAllObjects(&mut self);

    /// Removes the object with the highest-valued index in the array
    fn removeLastObject(&mut self);

    /// Removes all occurrences in the array of a given object.
    fn removeObject(&mut self, object: &T);

    /// Removes all occurrences within a specified range in the array of a given object.
    fn removeObjectInRange(&mut self, object: &T, range: Range<UInt>);

    /// Removes the object at index .
    fn removeObjectAtIndex(&mut self, index: UInt);

    /// Removes all occurrences of a given object in the array.
    fn removeObjectIdenticalTo(&mut self, object: &T);

    /// Removes all occurrences of anObject within the specified range in the array.
    fn removeObjectIdenticalToInRange(&mut self, object: &T, range: Range<UInt>);

    /// Removes from the receiving array the objects in another given array.
    fn removeObjectsInArray(&mut self, other_array: &Array<T>);

    /// Removes from the array each of the objects within a given range.
    fn removeObjectsInRange(&mut self, range: Range<UInt>);

    /* Replacing Objects
     */

    /// Replaces the object at index with anObject.
    fn replaceObjectAtIndex(&mut self, index: UInt, object: &T);

    /// Sets the receiving array’s elements to those in another given array.
    fn setArray(&mut self, other_array: &Array<T>);
}

/// A dynamic collection of objects associated with unique keys.
pub trait t_NSMutableDictionary<K, V>: t_NSDictionary<K, V> {
    /// Creates and initialize a dictionary
    fn initWithDictionary(&mut self, dictionary: super::Dictionary<K, V>);

    /* Adding Entries to a Mutable Dictionary
     */

    /// Adds a given key-value pair to the dictionary.
    fn setObject(&mut self, key: K, value: V)
    where
        K: t_NSObject,
        V: t_NSObject;

    /// Adds a given key-value pair to the dictionary.
    fn setObjectForKeyedSuperscript(&mut self, key: K, value: V)
    where
        K: Into<id>,
        V: Into<id>;

    /// Adds a given key-value pair to the dictionary.
    fn setValue(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<id>;

    /// Adds to the receiving dictionary the entries from another dictionary.
    fn addEntriesFromDictionary(&mut self, dictionary: super::Dictionary<K, V>);

    /// Sets the contents of the receiving dictionary to entries in a given dictionary.
    fn setDictionary(&mut self, dictionary: super::Dictionary<K, V>);

    /* Removing Entries From a Mutable Dictionary
     */

    /// Removes a given key and its associated value from the dictionary.
    fn removeObjectForKey(&mut self, key: K)
    where
        K: Into<id>;

    /// Empties the dictionary of its entries.
    fn removeAllObjects(&mut self);

    /// Removes from the dictionary entries specified by elements in a given array.
    fn removeObjectsForKeys(&mut self, keys: Array<K>)
    where
        K: t_NSObject;
}

/// A static collection of objects associated with unique keys.
pub trait t_NSDictionary<K, V>: t_NSObject {
    /* Creating an Empty Dictionary
     */

    /// Creates an empty dictionary.
    fn new() -> Self;

    /* Creating a Dictionary from Objects and Keys
     */

    /// Creates a dictionary containing entries constructed from the contents of an array of keys and an array of values.
    fn dictionaryWithObjects(objects: Array<V>, keys: Array<K>) -> Self;

    /// Creates a mutable dictionary containing entries constructed from the contents of an array of keys and an array of values.
    fn asMutDictionary(&mut self) -> super::MutableDictionary<K, V>;

    /* Counting Entries
     */

    /// The number of entries in the dictionary.
    fn count(&self) -> UInt;
}

/// An object for representing and performing arithmetic on base-10 numbers.
pub trait t_NSDecimalNumber: t_NSNumber {
    /* Creating a Decimal Number
     */

    /// Creates and returns a decimal number equivalent to a given decimal structure.
    fn decimalNumberWithDecimal(decimal: NSDecimalNumber) -> Self;

    /// Creates and returns a decimal number equivalent to the number specified by the arguments.
    fn decimalNumberWithMantissa(
        mantissa: c_ulonglong,
        exponent: c_short,
        is_negative: bool,
    ) -> Self;

    /// Creates a decimal number whose value is equivalent to that in a given numeric string.
    fn decimalNumberWithString<S>(string: S) -> Self
    where
        S: Into<String>;

    /// Creates a decimal number whose value is equivalent to that in a given numeric string, interpreted using a given locale.
    fn decimalNumberWithStringLocale<S, L>(string: S, locale: L) -> Self
    where
        S: Into<String>,
        L: t_NSLocale;

    /// A decimal number equivalent to the number 1.0.
    fn one() -> Self;

    /// A decimal number equivalent to the number 0.0.
    fn zero() -> Self;

    /// A decimal number that specifies no number.
    fn notANumber() -> Self;

    /* Initializing a Decimal Number
     */

    /// Initializes a decimal number to represent a given decimal.
    fn initWithDecimal(&mut self, decimal: NSDecimalNumber);

    /// Initializes a decimal number using the given mantissa, exponent, and sign.
    fn initWithMantissaExponentIsNegative(
        &mut self,
        mantissa: c_ulonglong,
        exponent: c_short,
        is_negative: bool,
    );

    /// Initializes a decimal number so that its value is equivalent to that in a given numeric string.
    fn initWithString<S>(&mut self, string: S)
    where
        S: Into<String>;

    /// Initializes a decimal number so that its value is equivalent to that in a given numeric string, interpreted using a given locale.
    fn initWithStringLocale<S, L>(&mut self, string: S, locale: L)
    where
        S: Into<String>,
        L: t_NSLocale;

    /* Performing Arithmetic
     */

    /// Adds this number to another given number.
    fn decimalNumberByAdding(&self, decimal_number: Self) -> Self;

    /// Subtracts another given number from this one.
    fn decimalNumberBySubtracting(&self, decimal_number: Self) -> Self;

    /// Multiplies the number by another given number.
    fn decimalNumberByMultiplyingBy(&self, decimal_number: Self) -> Self;

    /// Divides the number by another given number.
    fn decimalNumberByDividingBy(&self, decimal_number: Self) -> Self;

    /// Raises the number to a given power.
    fn decimalNumberByRaisingToPower(&self, power: c_uint) -> Self;

    /// Multiplies the number by 10 raised to the given power.
    fn decimalNumberByMultiplyingByPowerOf10(&self, power: c_short) -> Self;

    /// Adds this number to another given number using the specified behavior.
    fn decimalNumberByAddingWithBehavior(
        &self,
        decimal_number: &Self,
        with_behavior: Arc<dyn t_NSDecimalNumberBehaviors>,
    ) -> Self;

    /// Subtracts this a given number from this one using the specified behavior.
    fn decimalNumberBySubtractingWithBehavior(
        &self,
        decimal_number: &Self,
        with_behavior: Arc<dyn t_NSDecimalNumberBehaviors>,
    ) -> Self;

    /// Multiplies this number by another given number using the specified behavior.
    fn decimalNumberByMultiplyingByWithBehavior(
        &self,
        decimal_number: &Self,
        with_behavior: Arc<dyn t_NSDecimalNumberBehaviors>,
    ) -> Self;

    /// Divides this number by another given number using the specified behavior.
    fn decimalNumberByDividingByWithBehavior(
        &self,
        decimal_number: &Self,
        with_behavior: Arc<dyn t_NSDecimalNumberBehaviors>,
    ) -> Self;

    /// Raises the number to a given power using the specified behavior.
    fn decimalNumberByRaisingToPowerWithBehavior(
        &self,
        power: c_uint,
        with_behavior: Arc<dyn t_NSDecimalNumberBehaviors>,
    ) -> Self;

    /// Multiplies the number by 10 raised to the given power using the specified behavior.
    fn decimalNumberByMultiplyingByPowerOf10WithBehavior(
        &self,
        power: c_short,
        with_behavior: Arc<dyn t_NSDecimalNumberBehaviors>,
    ) -> Self;

    /* Rounding Off
     */
    /// Returns a rounded version of the decimal number using the specified rounding behavior.
    fn decimalNumberByRoundingAccordingToBehavior(
        &self,
        behavior: Arc<dyn t_NSDecimalNumberBehaviors>,
    ) -> Self;

    /* Managing Behavior
     */

    /// The way arithmetic methods round off and handle error conditions.
    fn defaultBehavior() -> Arc<dyn t_NSDecimalNumberBehaviors>;

    /// Sets the way arithmetic methods round off and handle error conditions.
    fn setDefaultBehavior(behavior: Arc<dyn t_NSDecimalNumberBehaviors>);

    /// The decimal number’s closest approximate double value.
    fn doubleValue(&self) -> f64;

    /// Returns a string representation of the decimal number appropriate for the specified locale.
    fn descriptionWithLocale<L>(&self, locale: L) -> String
    where
        L: t_NSLocale;

    /// A C string containing the Objective-C type for the data contained in the decimal number object.
    fn objcType(&self) -> *const c_char;

    /* Comparing Decimal Numbers
     */

    /// Compares this decimal number and another.
    fn compare(&self, decimal_number: &Self) -> ComparisonResult;

    /* Getting Maximum and Minimum Possible Values
     */

    /// Returns the largest possible value of a decimal number.
    fn maximumDecimalNumber() -> Self;

    /// Returns the smallest possible value of a decimal number.
    fn minimumDecimalNumber() -> Self;
}

/// A protocol that declares three methods that control the discretionary aspects of working with decimal numbers.
pub trait t_NSDecimalNumberBehaviors {
    /* Rounding Behavior
     */

    /// Returns the way that NSDecimalNumber's decimalNumberBy... methods round their return values.
    fn roundingMode(&self) -> NSRoundingMode;

    /// Returns the number of digits allowed after the decimal separator.
    fn scale(&self) -> c_short;
}

/// A container for information broadcast through a notification center to all registered observers.
pub trait t_NSNotification: t_NSObject {
    /// Initializes an empty notification.
    fn init() -> Self;
}
