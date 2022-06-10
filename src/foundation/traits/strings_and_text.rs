use std::ops::Range;

use libc::{c_char, c_void};

use crate::{
    foundation::{
        ns_coder::NSCoder, string::Encoding, unichar, Int, NSArray, NSCharacterSet,
        NSComparisonResult, NSData, NSLocale, NSMutableString, NSString, NSStringCompareOptions,
        NSStringEncodingConversionOptions, NSStringTransform, UInt, UInt8,
    },
    objective_c_runtime::traits::PNSObject,
};

/* ------------------------------------------------------------------------- */
/* Topic: - Strings  */
/* ------------------------------------------------------------------------- */

/// A static, plain-text Unicode string object.
pub trait INSString: PNSObject {
    /* Creating and Initializing Strings
     */

    /// Returns an empty string.
    fn tm_string() -> Self;

    /// Returns an initialized NSString object that contains no characters.
    fn im_init(self) -> Self;

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
    fn im_init_with_bytes_length_encoding(
        self,
        bytes: *const c_void,
        len: UInt,
        encoding: Encoding,
    ) -> Self;

    /// Returns an initialized NSString object that contains a given number of bytes from a given buffer of bytes interpreted in a given encoding, and optionally frees the buffer.
    ///
    /// # Arguments
    ///
    /// * `bytes` - A buffer of bytes interpreted in the encoding specified by encoding.
    /// * `len` - The number of bytes to use from bytes.
    /// * `encoding` - The character encoding applied to bytes. For possible values, see NSStringEncoding.
    /// * `free_buffer` - If YES, bytes is freed after use.
    ///
    /// # Returns
    ///
    /// An initialized NSString object containing length bytes from bytes interpreted using the encoding encoding. The returned object may be different from the original receiver.
    fn im_init_with_bytes_no_copy_length_encoding_free_when_done(
        self,
        bytes: *mut c_void,
        len: UInt,
        encoding: Encoding,
        free_buffer: bool,
    ) -> Self;

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
    fn im_init_with_characters_length(self, characters: *const unichar, len: UInt) -> Self;

    /// Returns an initialized NSString object that contains a given number of characters
    /// from a given C array of UTF-16 code units.
    ///
    /// # Arguments
    ///
    /// * `characters` - A C array of UTF-16 code units; the value must not be NULL.
    /// * `length` - A C array of UTF-16 code units; the value must not be NULL.
    /// * `free_buffer` - If YES, characters is freed after use.
    ///
    /// # Returns
    ///
    /// An initialized NSString object containing length characters from characters.
    fn im_init_with_characters_no_copy_length_free_when_done(
        self,
        characters: unichar,
        length: UInt,
        free_buffer: bool,
    ) -> Self;

    /// Returns an NSString object initialized by copying the characters from another given string.
    ///
    /// # Arguments
    ///
    /// * `s` - The string from which to copy characters. This value must not be nil.
    ///
    /// # Returns
    ///
    /// An NSString object initialized by copying the characters from s.
    fn im_init_with_string<S>(self, s: S) -> Self
    where
        S: Into<NSString>;

    /// Returns an NSString object initialized using the characters in a given C array,
    /// interpreted according to a given encoding.
    ///
    /// # Arguments
    ///
    /// * `c_str` - A C array of bytes.
    /// * `encoding` - The encoding to use to interpret c_str. For possible values, see `NSStringEncoding`.
    ///
    /// # Returns
    ///
    /// An NSString object initialized using the characters in c_str,
    /// interpreted according to encoding.
    fn im_init_with_cstring_encoding(self, c_str: *const c_char, encoding: Encoding) -> Self;

    /// Returns an NSString object initialized using the characters in a given C array,
    /// interpreted according to a UTF8 string.
    ///
    /// # Arguments
    ///
    /// * `c_str` - A C array of bytes.
    ///
    /// # Returns
    ///
    /// An NSString object initialized using the characters in c_str,
    /// interpreted as a UTF8 string.
    fn im_init_with_utf8_string(self, c_str: *const c_char) -> Self;

    /// Returns an NSString object initialized by converting given data into
    /// UTF-16 code units using a given encoding.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to convert into UTF-16 code units.
    /// * `encoding` - The encoding to use to interpret data. For possible values, see `NSStringEncoding`.
    ///
    /// # Returns
    ///
    /// An NSString object initialized by converting data into UTF-16 code units using encoding.
    fn im_init_with_data_encoding(self, data: NSData, encoding: Encoding) -> Self;

    /// Returns a localized string intended for display in a notification alert.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to use when looking up the string in the app’s Localizable.strings file.
    /// * `arguments` - An array of values to substitute for escaped characters in the string.
    fn tm_localized_user_notification_string_for_key_arguments<K, T>(
        key: K,
        arguments: NSArray<T>,
    ) -> NSString
    where
        K: Into<NSString>,
        T: PNSObject;

    /// Returns a string containing a given number of characters taken from a
    /// given C array of UTF-16 code units.
    ///
    /// # Arguments
    ///
    /// * `characters` - A C array of UTF-16 code units; the value must not be NULL.
    /// * `length` - A C array of UTF-16 code units; the value must not be NULL.
    ///
    /// # Returns
    ///
    /// An NSString object containing length characters from characters.
    fn tm_string_with_characters_length(characters: *const unichar, length: UInt) -> Self;

    /// Returns a string created by copying the characters from another given string.
    ///
    /// # Arguments
    ///
    /// * `s` - The string from which to copy characters. This value must not be nil.
    ///
    /// # Returns
    ///
    /// An NSString object initialized by copying the characters from s.
    fn tm_string_with_string<S>(s: S) -> Self
    where
        S: Into<NSString>;

    /// Returns a string containing the bytes in a given C array, interpreted
    /// according to a given encoding.
    ///
    /// # Arguments
    ///
    /// * `c_str` - A C array of bytes.
    /// * `encoding` - The encoding to use to interpret c_str. For possible values, see `NSStringEncoding`.
    ///
    /// # Returns
    ///
    /// An NSString object containing the bytes in c_str, interpreted according to encoding.
    fn tm_string_with_cstring_encoding(c_str: *const c_char, encoding: Encoding) -> Self;

    /// Returns a string created by copying the data from a given C array of
    /// UTF8-encoded bytes.
    ///
    /// # Arguments
    ///
    /// * `c_str` - A C array of bytes.
    ///
    /// # Returns
    ///
    /// An NSString object containing the bytes in c_str, interpreted as a UTF8 string.
    fn tm_string_with_utf8_string(c_str: *const c_char) -> Self;

    /* Getting a String’s Length
     */

    /// The number of Unicode characters in this string.
    fn ip_length(&self) -> UInt;

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
    fn im_length_of_bytes_using_encoding(&self, enc: Encoding) -> UInt;

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
    fn im_maximum_length_of_bytes_using_encoding(&self, enc: Encoding) -> Int;

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
    fn im_character_at_index(&self, index: UInt) -> char;

    /// Copies characters from a given range in the receiver into a given buffer.
    ///
    /// # Arguments
    ///
    /// * `buffer` - Upon return, contains the characters from the receiver. buffer must be large enough to contain the characters in the range aRange (aRange.length*sizeof(unichar)).
    /// * `range` - The range of characters to copy.
    fn im_get_characters_range(&self, buffer: *mut unichar, range: Range<UInt>);

    /// Gets a given range of characters as bytes in a specified encoding.
    ///
    /// # Arguments
    ///
    /// * `buffer` - Upon return, contains the bytes from the receiver. buffer must be large enough to contain the bytes in the range aRange (aRange.length*sizeof(char)).
    /// * `maxLength - The maximum number of bytes to get.
    /// * `usedLength` - Upon return, contains the actual number of bytes retrieved.
    /// * `encoding` - The encoding to use to interpret the bytes. For possible values, see `NSStringEncoding`.
    /// * `options` - The options to use for converting the receiver into the specified encoding. For possible values, see `NSStringEncodingConversionOptions`.
    /// * `range` - The range of characters to get.
    /// * `remainingRange` - Upon return, contains a range containing the remaining characters.
    #[allow(clippy::too_many_arguments)]
    fn im_get_bytes_max_length_used_length_encoding_options_range_remaining_range(
        &self,
        buffer: *mut c_void,
        max_length: Int,
        used_length: *mut Int,
        encoding: Encoding,
        options: NSStringEncodingConversionOptions,
        range: Range<UInt>,
        remaining_range: Range<UInt>,
    ) -> bool;

    /* Getting C Strings
     */

    /// Returns a representation of the string as a C string using a given encoding.
    ///
    /// # Arguments
    ///
    /// * `encoding` - The encoding to use to interpret the receiver. For possible values, see `NSStringEncoding`.
    ///
    /// # Returns
    ///
    /// A pointer to a C string containing the receiver. The pointer is owned by the string, and must not be freed by the caller.
    fn im_c_string_using_encoding(&self, encoding: Encoding) -> *const c_char;

    /// Returns a representation of the string as a C string.
    ///
    /// # Arguments
    ///
    /// * `buffer` - Upon return, contains the characters from the receiver. buffer must be large enough to contain the characters in the receiver.
    /// * `maxLength` - The maximum number of bytes to write into buffer.
    /// * `encoding` - The encoding to use to interpret the receiver. For possible values, see `NSStringEncoding`.
    ///
    /// # Returns
    ///
    /// `true` if the operation was successful, otherwise `false`. Returns `false` if conversion is not possible due to encoding errors or if buffer is too small.
    fn im_get_cstring_max_length_encoding(
        &self,
        buffer: *mut c_char,
        max_length: UInt,
        encoding: Encoding,
    ) -> bool;

    /// A null-terminated UTF8 representation of the string.
    fn ip_utf8_string(&self) -> *const c_char;

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
    fn im_case_insensitive_compare<S>(&self, string: S) -> NSComparisonResult
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
    fn im_localized_case_insensitive_compare<S>(&self, string: S) -> NSComparisonResult
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
    fn im_compare<S>(&self, string: S) -> NSComparisonResult
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
    fn im_localized_compare<S>(&self, string: S) -> NSComparisonResult
    where
        S: Into<NSString>;

    /// Compares the string with the specified string using the given options.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare.
    /// * `mask` - The mask to use when comparing the receiver and `string`.
    ///
    /// # Returns
    ///
    /// Returns an `ComparisonResult` value that indicates the lexical ordering.
    fn im_compare_options<S>(&self, string: S, mask: NSStringCompareOptions) -> NSComparisonResult
    where
        S: Into<NSString>;

    /// Compares the string with the specified string using the given options.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare.
    /// * `mask` - The mask to use when comparing the receiver and `string`.
    /// * `range` - The range of the receiver to compare.
    fn im_compare_options_range<S>(
        &self,
        string: S,
        mask: NSStringCompareOptions,
        range: Range<UInt>,
    ) -> NSComparisonResult
    where
        S: Into<NSString>;

    /// Compares the string using the specified options and returns the lexical ordering for the range.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare.
    /// * `mask` - The mask to use when comparing the receiver and `string`.
    /// * `range` - The range to compare.
    /// * `locale` - The locale to use when comparing the receiver and `string`.
    ///
    /// # Returns
    ///
    /// Returns an `ComparisonResult` value that indicates the lexical ordering.
    fn im_compare_options_range_locale<S>(
        &self,
        string: S,
        mask: NSStringCompareOptions,
        range: Range<UInt>,
        locale: NSLocale,
    ) -> NSComparisonResult
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
    fn im_localized_standard_compare<S>(&self, string: S) -> NSComparisonResult
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
    fn im_has_prefix<S>(&self, prefix: S) -> bool
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
    fn im_has_suffix<S>(&self, suffix: S) -> bool
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
    fn im_is_equal_to_string<S>(&self, string: S) -> bool
    where
        S: Into<NSString>;

    /* Combining Strings
     */

    /// Returns a new string made by appending a given string to the receiver.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to append to the receiver. This value must not be nil.
    fn im_string_by_appending_string<S>(&self, string: S) -> NSString
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
    fn im_string_by_padding_to_length_with_string_starting_at_index<S>(
        &self,
        new_length: UInt,
        pad_string: S,
        starting_at: UInt,
    ) -> NSString
    where
        S: Into<NSString>;

    /* Changing Case
     */

    /// A lowercase representation of the string.
    fn ip_lowercase_string(&self) -> NSString;

    /// Returns a version of the string with all letters converted to lowercase,
    /// taking into account the current locale.
    fn ip_localized_lowercase_string(&self) -> NSString;

    /// Returns a version of the string with all letters converted to
    /// lowercase, taking into account the specified locale.
    ///
    /// # Arguments
    ///
    /// * `locale` - The locale to use when converting the letters to lowercase.
    ///
    /// # Returns
    ///
    /// A new string with all letters converted to lowercase.
    fn im_lowercase_string_with_locale(&self, locale: NSLocale) -> NSString;

    /// An uppercase representation of the string.
    fn ip_uppercase_string(&self) -> NSString;

    /// Returns a version of the string with all letters converted to uppercase,
    /// taking into account the current locale.
    fn ip_localized_uppercase_string(&self) -> NSString;

    /// Returns a version of the string with all letters converted to uppercase,
    /// taking into account the specified locale.
    ///
    /// # Arguments
    ///
    /// * `locale` - The locale to use when converting the letters to uppercase.
    ///
    /// # Returns
    ///
    /// A new string with all letters converted to uppercase.
    fn im_uppercase_string_with_locale(&self, locale: NSLocale) -> NSString;

    /// A capitalized representation of the string.
    fn ip_capitalized_string(&self) -> NSString;

    /// Returns a capitalized representation of the receiver using the current
    /// locale.
    fn ip_localized_capitalized_string(&self) -> NSString;

    /// Returns a capitalized representation of the receiver using the
    /// specified locale.
    fn im_capitalized_string_with_locale(&self, locale: NSLocale) -> NSString;

    /* Dividing Strings
     */

    /// Returns an array containing substrings from the receiver that have been divided by a given separator.
    fn im_components_separated_by_string<S>(&self, separator: S) -> NSArray<NSString>
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
    fn im_contains_string<S>(&self, other: S) -> bool
    where
        S: Into<NSString>;

    /* Transforming Strings
     */

    /// Returns a new string made by appending to the receiver a given string.
    ///
    /// # Arguments
    ///
    /// * `transform` - The `StringTransform` to apply.
    /// * `reverse` - If `true`, the transformation is applied in reverse.
    fn im_string_by_applying_transform_reverse(
        &mut self,
        transform: NSStringTransform,
        reverse: bool,
    ) -> Option<NSString>;

    /* Working with Encodings
     */

    /// Returns a zero-terminated list of the encodings string objects support in the application’s environment.
    fn tp_available_string_encodings() -> *const Encoding;

    /// Returns the C-string encoding assumed for any method accepting a C string as an argument.
    fn tp_default_cstring_encoding() -> Encoding;

    /// Returns a Boolean value that indicates whether the receiver can be converted to a given encoding without loss of information.
    fn im_can_be_converted_to_encoding(&self, encoding: Encoding) -> bool;

    /// Returns an NSData object containing a representation of the receiver encoded using a given encoding.
    fn im_data_using_encoding(&self, encoding: Encoding) -> NSData;
}

/// A dynamic plain-text Unicode string object.
pub trait INSMutableString: INSString {
    /* Creating and Initializing a Mutable String
     */

    /// Returns an empty NSMutableString object with initial storage for a
    /// given number of characters.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The number of characters to allocate space for.
    ///
    /// # Returns
    ///
    /// An empty NSMutableString object with initial storage for a given number of characters.
    fn tm_string_with_capacity(capacity: UInt) -> NSMutableString;

    /// Returns an NSMutableString object initialized with initial storage for
    /// a given number of characters,
    ///
    /// # Arguments
    ///
    /// * `capacity` - The number of characters to allocate space for.
    fn im_init_with_capacity(self, capacity: UInt) -> NSMutableString;

    /* Modifying a String
     */

    /// Adds a constructed string to the receiver.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to append to the receiver.
    fn im_append_string<S>(&mut self, string: S)
    where
        S: Into<NSString>;

    /// Transliterates the receiver by applying a specified ICU string transform.
    ///
    /// # Arguments
    ///
    /// * `transform` - The `StringTransform` to apply.
    /// * `reverse` - If `true`, the transformation is applied in reverse.
    /// * `range` - The range of characters to apply the transformation to.
    /// * `out_range` - The range of characters that were transformed.
    ///
    /// # Returns
    ///
    /// `true` if the transformation was successful, otherwise `false`.
    fn im_apply_transform_reverse_range_updated_range(
        &mut self,
        transform: NSStringTransform,
        reverse: bool,
        range: Range<UInt>,
        out_range: Range<UInt>,
    ) -> bool;

    /// Removes from the receiver the characters in a given range.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of characters to remove.
    fn im_delete_characters_in_range(&mut self, range: Range<UInt>);

    /// Inserts into the receiver the characters of a given string at a given
    /// location.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to insert into the receiver.
    /// * `loc` - The location at which to insert `string`.
    fn im_insert_string_at_index<S>(&mut self, string: S, loc: UInt)
    where
        S: Into<NSString>;

    /// Replaces the characters from aRange with those in `string`.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of characters to replace.
    /// * `string` - The string to replace with.
    fn im_replace_characters_in_range_with_string<S>(&mut self, range: Range<UInt>, string: S)
    where
        S: Into<NSString>;

    /// Replaces all occurrences of a given string in a given range with
    /// another given string, returning the number of replacements.
    fn im_replace_occurrences_of_string_with_string_options_range<S>(
        &mut self,
        target: NSString,
        replacement: S,
        options: NSStringCompareOptions,
        search_range: Range<UInt>,
    ) -> UInt
    where
        S: Into<NSString>;

    /// Replaces the characters of the receiver with those in a given string.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to replace the characters of the receiver with. string must not be `null`.
    fn im_set_string<S>(&mut self, string: S)
    where
        S: Into<NSString>;
}

/// An object representing a fixed set of Unicode character values for use in
/// search operations.
pub trait INSCharacterSet: PNSObject {
    /* Getting Standard Character Sets
     */

    /// A character set containing the characters in Unicode General Categories L*, M*, and N*.
    fn tp_alphanumeric_character_set() -> NSCharacterSet;

    /// A character set containing the characters in Unicode General Category Lt.
    fn tp_capitalized_letter_character_set() -> NSCharacterSet;

    /// A character set containing the characters in Unicode General Category Cc and Cf.
    fn tp_control_character_set() -> NSCharacterSet;

    /// A character set containing the characters in the category of Decimal Numbers.
    fn tp_decimal_digit_character_set() -> NSCharacterSet;

    /// A character set containing individual Unicode characters that can also be represented as composed character sequences (such as for letters with accents), by the definition of “standard decomposition” in version 3.2 of the Unicode character encoding standard.
    fn tp_decomposable_character_set() -> NSCharacterSet;

    /// A character set containing values in the category of Non-Characters or that have not yet been defined in version 3.2 of the Unicode standard.
    fn tp_illegal_character_set() -> NSCharacterSet;

    /// A character set containing the characters in Unicode General Category L* & M*.
    fn tp_letter_character_set() -> NSCharacterSet;

    /// A character set containing the characters in Unicode General Category Ll.
    fn tp_lowercase_letter_character_set() -> NSCharacterSet;

    /// A character set containing the newline characters (U+000A ~ U+000D, U+0085, U+2028, and U+2029).
    fn tp_newline_character_set() -> NSCharacterSet;

    /// A character set containing the characters in Unicode General Category M*.
    fn tp_non_base_character_set() -> NSCharacterSet;

    /// A character set containing the characters in Unicode General Category P*.
    fn tp_punctuation_character_set() -> NSCharacterSet;

    /// A character set containing the characters in Unicode General Category S*.
    fn tp_symbol_character_set() -> NSCharacterSet;

    /// A character set containing the characters in Unicode General Category Lu and Lt.
    fn tp_uppercase_letter_character_set() -> NSCharacterSet;

    /// A character set containing characters in Unicode General Category Z*, U+000A ~ U+000D, and U+0085.
    fn tp_whitespace_and_newline_character_set() -> NSCharacterSet;

    /// A character set containing the characters in Unicode General Category Zs and CHARACTER TABULATION (U+0009).
    fn tp_whitespace_character_set() -> NSCharacterSet;

    /* Getting Character Sets for URL Encoding
     */

    /// Returns the character set for characters allowed in a fragment URL component.
    fn tp_urlfragment_allowed_character_set() -> NSCharacterSet;

    /// Returns the character set for characters allowed in a host URL subcomponent.
    fn tp_urlhost_allowed_character_set() -> NSCharacterSet;

    /// Returns the character set for characters allowed in a password URL subcomponent.
    fn tp_urlpassword_allowed_character_set() -> NSCharacterSet;

    /// Returns the character set for characters allowed in a path URL component.
    fn tp_urlpath_allowed_character_set() -> NSCharacterSet;

    /// Returns the character set for characters allowed in a query URL component.
    fn tp_urlquery_allowed_character_set() -> NSCharacterSet;

    /// Returns the character set for characters allowed in a user URL subcomponent.
    fn tp_urluser_allowed_character_set() -> NSCharacterSet;

    /* Creating a Custom Character Set
     */

    /// Initializing with coder
    fn im_init_with_coder(self, coder: NSCoder) -> Self;

    /// Returns a character set containing the characters in a given string.
    fn tm_character_set_with_characters_in_string(string: NSString) -> NSCharacterSet;

    /// Returns a character set containing characters with Unicode values in a given range.
    fn tm_character_set_with_range(range: Range<UInt>) -> NSCharacterSet;

    /* Creating and Managing Character Sets as Bitmap Representations
     */

    /// Returns a character set containing characters determined by a given bitmap representation.
    fn tm_character_set_with_bitmap_representation(data: NSData) -> NSCharacterSet;

    /// Returns a character set read from the bitmap representation stored in the file a given path.
    fn tm_character_set_with_contents_of_file(path: NSString) -> NSCharacterSet;

    /// An NSData object encoding the receiver in binary format.
    fn ip_bitmap_representation(&self) -> NSData;

    /// A character set containing only characters that don’t exist in the receiver.
    fn ip_inverted_set(&self) -> NSCharacterSet;

    /* Testing Set Membership
     */

    /// Returns a Boolean value that indicates whether a given character is in the receiver.
    fn im_character_is_member(&self, character: unichar) -> bool;

    /// Returns a Boolean value that indicates whether the receiver has at least one member in a given character plane.
    fn im_has_member_in_plane(&self, plane: UInt8) -> bool;

    /// Returns a Boolean value that indicates whether the receiver is a superset of another given character set.
    fn im_is_superset_of_set(&self, other: NSCharacterSet) -> bool;

    /// Returns a Boolean value that indicates whether a given long character is a member of the receiver.
    fn im_long_character_is_member(&self, long_char: u32) -> bool;
}
