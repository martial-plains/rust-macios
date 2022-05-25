use std::ops::Range;

use libc::c_char;

use crate::{
    foundation::{
        string::Encoding, unichar, CompareOptions, ComparisonResult, Int, NSLocale, NSString,
        NSStringTransform, UInt,
    },
    objective_c_runtime::traits::t_NSObject,
};

/// A static, plain-text Unicode string object.
pub trait t_NSString: t_NSObject {
    /* Creating and Initializing Strings
     */

    /// Creates a new `NSString`
    fn string() -> Self;

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
    fn caseInsensitiveCompare<S>(&self, string: S) -> ComparisonResult
    where
        S: Into<NSString>;

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
    fn localizedCaseInsensitiveCompare<S>(&self, string: S) -> ComparisonResult
    where
        S: Into<NSString>;

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
    fn compare<S>(&self, string: S) -> ComparisonResult
    where
        S: Into<NSString>;

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
    fn localizedCompare<S>(&self, string: S) -> ComparisonResult
    where
        S: Into<NSString>;

    /// Compares strings as sorted by the Finder.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare with the receiver.
    ///
    /// # Returns
    ///
    /// The result of the comparison.
    fn localized_standard_compare<S>(&self, string: S) -> ComparisonResult
    where
        S: Into<NSString>;

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
    fn compareWithOptions<S>(&self, string: S, mask: CompareOptions) -> ComparisonResult
    where
        S: Into<NSString>;

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
    fn compareWithOptionsRange<S>(
        &self,
        string: S,
        options: CompareOptions,
        range: Range<UInt>,
    ) -> ComparisonResult
    where
        S: Into<NSString>;

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
    fn compareWithOptionsRangeLocale<S>(
        &self,
        string: S,
        options: CompareOptions,
        range: Range<UInt>,
        locale: NSLocale,
    ) -> ComparisonResult
    where
        S: Into<NSString>;

    /// Returns a Boolean value that indicates whether a given string matches the beginning characters of the receiver.
    ///
    /// # Arguments
    ///
    /// * `prefix` - The prefix to search for.
    ///
    /// # Returns
    ///
    /// Returns `true` if the string begins with the prefix, otherwise `false`.
    fn hasPrefix<S>(&self, prefix: S) -> bool
    where
        S: Into<NSString>;

    /// Returns a Boolean value that indicates whether a given string matches the ending characters of the receiver.
    ///
    /// # Arguments
    ///
    /// * `suffix` - The suffix to search for.
    ///
    /// # Returns
    ///
    /// Returns `true` if the string ends with the suffix, otherwise `false`.
    fn hasSuffix<S>(&self, suffix: S) -> bool
    where
        S: Into<NSString>;

    /// Returns a Boolean value that indicates whether a given string is equal to the receiver using a literal Unicode-based comparison.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare with the receiver.
    ///
    /// # Returns
    ///
    /// Returns `true` if the string is equal to the receiver, otherwise `false`.
    fn isEqualTo<S>(&self, string: S) -> bool
    where
        S: Into<NSString>;

    /* Combining Strings
     */

    /// Returns a new string made by appending a given string to the receiver.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to append to the receiver. This value must not be nil.
    fn appending<S>(&self, string: S) -> NSString
    where
        S: Into<NSString>;

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
    fn padding<S>(&self, new_length: Int, pad_string: S, starting_at: Int) -> NSString
    where
        S: Into<NSString>;

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
    fn contains<S>(&self, other: S) -> bool
    where
        S: Into<NSString>;

    /* Changing Case
     */

    /// A lowercase representation of the string.
    fn lowercased(&self) -> NSString;

    /// Returns a version of the string with all letters converted to lowercase,
    /// taking into account the current locale.
    fn localizedLowercase(&self) -> NSString;

    /// An uppercase representation of the string.
    fn uppercased(&self) -> NSString;

    /// Returns a version of the string with all letters converted to uppercase,
    /// taking into account the current locale.
    fn localizedUppercase(&self) -> NSString;

    /// A capitalized representation of the string.
    fn capitalized(&self) -> NSString;

    /// Returns a capitalized representation of the receiver using the current
    /// locale.
    fn localizedCapitalized(&self) -> NSString;

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
        transform: NSStringTransform,
        reverse: bool,
    ) -> Option<NSString>;
}
