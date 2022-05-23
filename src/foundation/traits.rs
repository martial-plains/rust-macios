use std::ops::Range;

use libc::{
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint,
    c_ulong, c_ulonglong, c_ushort,
};
use objc::runtime::Object;

use crate::{
    foundation::String,
    id,
    objective_c_runtime::traits::{NSObject, NSValue},
};

use super::{
    key::NSLocaleKey, unichar, Array, CompareOptions, ComparisonResult, Encoding, Int,
    LanguageDirection, StringTransform, UInt,
};

/// A static, plain-text Unicode string object.
pub trait NSString: NSObject {
    /// Creates a new `NSString`
    fn new() -> Self;

    /// In some cases, we want to wrap a system-provided NSString without retaining it.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it dereferrences a raw pointer.
    unsafe fn from_retained(object: id) -> Self;

    /// Utility method for checking whether an `NSObject` is an `NSString`.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it dereferrences a raw pointer.
    unsafe fn is(obj: id) -> bool;

    /// Returns the UTF8 bytes for this `NSString`.
    fn bytes(&self) -> *const c_char;

    /// Gets the proper byte length for this `NSString` (the UTF8 variant).
    fn bytes_len(&self) -> UInt;

    /// Convert this `NSString` into a `&str`.
    fn as_str(&self) -> &str;

    /* Creating and Initializing Strings
     */

    /// Returns an initialized NSString object that contains no characters.
    fn init() -> Self;

    /// Returns an initialized NSString object containing a given number of bytes from a given
    /// buffer of bytes interpreted in a given encoding.
    fn init_with_bytes_len_encoding(bytes: *const c_char, len: UInt, encoding: Encoding) -> Self;

    /// Returns an initialized NSString object that contains a given number of bytes from
    /// a given buffer of bytes interpreted in a given encoding, and optionally frees the buffer.
    ///
    /// # Arguments
    ///
    /// * `s` - The string slice to create the `NSString` from.
    fn init_with_no_cpy_str(s: &str) -> Self;

    /// Returns an initialized NSString object that contains a given number of characters from
    /// a given C array of UTF-16 code units.
    fn init_with_characters_len(characters: *const unichar, len: UInt) -> Self;

    /// Returns an NSString object initialized by copying the characters from another given string.
    fn init_with_str(s: &str) -> Self;

    /* Getting a String’s Length
     */

    /// The number of Unicode characters in this string.
    fn length(&self) -> Int;

    /// Returns the number of bytes required to store the receiver in a given encoding.
    ///
    /// # Arguments
    ///
    /// * `enc` - The encoding for which to determine the receiver's length.
    fn length_of_bytes(&self, enc: Encoding) -> Int;

    /// Returns the maximum number of bytes needed to store the receiver in a given encoding.
    ///
    /// # Arguments
    ///
    /// * `enc` - The encoding for which to determine the receiver's length.
    fn maximum_length_of_bytes(&self, enc: Encoding) -> Int;

    /* Getting Characters and Bytes
     */

    /// Returns the character at a given UTF-16 code unit index.
    ///
    /// # Arguments
    ///
    /// * `index` - The character at the array position given by `index`.
    fn character_at(&self, index: Int) -> char;

    /* Identifying and Comparing Strings
     */

    /// Returns the result of invoking compare:options: with NSCaseInsensitiveSearch as the only option.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare.
    fn case_insensitive_compare<T>(&self, string: T) -> ComparisonResult
    where
        T: Into<String>;

    /// Compares the string with a given string using a case-insensitive, localized, comparison.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare.
    fn localized_case_insensitive_compare<T>(&self, string: T) -> ComparisonResult
    where
        T: Into<String>;

    /// Returns the result of invoking compare:options:range: with no options and the receiver’s full extent as the range.
    ///
    /// # Arguments
    ///
    /// * `string` - The string with which to compare the receiver.
    ///
    /// # Safety
    ///
    /// This value must not be nil. If this value is nil, the behavior is undefined and may change in future versions of macOS.
    fn compare<T>(&self, string: T) -> ComparisonResult
    where
        T: Into<String>;

    /// Returns the result of invoking compare:options:range: with no options and the receiver’s full extent as the range.
    fn localized_compare<T>(&self, string: T) -> ComparisonResult
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
    fn compare_with_options<T>(&self, string: T, options: CompareOptions) -> ComparisonResult
    where
        T: Into<String>;

    /// Returns the result of invoking compare(_:options:range:locale:) with a nil locale.
    fn compare_with_options_range<T>(
        &self,
        string: T,
        options: CompareOptions,
        range: Range<UInt>,
    ) -> ComparisonResult
    where
        T: Into<String>;

    /// Compares the string using the specified options and returns the lexical ordering for the range.
    fn compare_with_options_range_locale<T>(
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
    fn has_prefix<T>(&self, prefix: T) -> bool
    where
        T: Into<String>;

    /// Returns a Boolean value that indicates whether a given string matches the ending characters of the receiver.
    ///
    /// # Arguments
    ///
    /// * `suffix` - The suffix to search for.
    fn has_suffix<T>(&self, suffix: T) -> bool
    where
        T: Into<String>;

    /// Returns a Boolean value that indicates whether a given string is equal to the receiver using a literal Unicode-based comparison.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare with the receiver.
    fn is_equal_to<T>(&self, string: T) -> bool
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
    fn contains<T>(&self, other: T) -> bool
    where
        T: Into<String>;

    /* Changing Case
     */

    /// A lowercase representation of the string.
    fn lowercased(&self) -> String;

    /// Returns a version of the string with all letters converted to lowercase, taking into account the current locale.
    fn localized_lowercase(&self) -> String;

    /// An uppercase representation of the string.
    fn uppercased(&self) -> String;

    /// Returns a version of the string with all letters converted to uppercase, taking into account the current locale.
    fn localized_uppercase(&self) -> String;

    /// A capitalized representation of the string.
    fn capitalized(&self) -> String;

    /// Returns a capitalized representation of the receiver using the current locale.
    fn localized_capitalized(&self) -> String;

    /* Transforming Strings
     */

    /// Returns a new string made by appending to the receiver a given string.
    fn applying_transform(&mut self, transform: StringTransform, reverse: bool) -> Option<String>;
}

/// The group of methods that are used with `NSNumber` objects.
pub trait NSNumber: NSValue {
    /* Creating an NSNumber Object
     */

    /// Creates and returns an NSNumber object containing a given value, treating it as a BOOL.
    fn number_with_bool(value: bool) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as a signed char.
    fn number_with_char(value: c_schar) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as a double.
    fn number_with_double(value: c_double) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as a float.
    fn number_with_float(value: c_float) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as a signed int.
    fn number_with_int(value: c_int) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an NSInteger.
    fn number_with_integer(value: Int) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as a signed long.
    fn number_with_long(value: c_long) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as a signed long long.
    fn number_with_long_long(value: c_longlong) -> Self;

    /// Creates and returns an NSNumber object containing value, treating it as a signed short.
    fn number_with_short(value: c_short) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned char.
    fn number_with_unsigned_char(value: c_uchar) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned int.
    fn number_with_unsigned_int(value: c_uint) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an NSUInteger.
    fn number_with_unsigned_integer(value: UInt) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned long.
    fn number_with_unsigned_long(value: c_ulong) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned long long.
    fn number_with_unsigned_long_long(value: c_ulonglong) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned short.
    fn number_with_unsigned_short(value: c_ushort) -> Self;

    /* Initializing an NSNumber Object
     */

    /// Returns an NSNumber object initialized to contain a given value, treated as a BOOL.
    fn init_with_bool(&self, value: bool) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as a signed char.
    fn init_with_char(&self, value: c_schar) -> Self;

    /// Returns an NSNumber object initialized to contain value, treated as a double.
    fn init_with_double(&self, value: c_double) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as a float.
    fn init_with_float(&self, value: c_float) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as a signed int.
    fn init_with_int(&self, value: c_int) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an NSInteger.
    fn init_with_integer(&self, value: Int) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as a signed long.
    fn init_with_long(&self, value: c_long) -> Self;

    /// Returns an NSNumber object initialized to contain value, treated as a signed long long.
    fn init_with_long_long(&self, value: c_longlong) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as a signed short.
    fn init_with_short(&self, value: c_short) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned char.
    fn init_with_unsigned_char(&self, value: c_uchar) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned int.
    fn init_with_unsigned_int(&self, value: c_uint) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an NSUInteger.
    fn init_with_unsigned_integer(&self, value: c_uint) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned long.
    fn init_with_unsigned_long(&self, value: c_ulong) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned long long.
    fn init_with_unsigned_long_long(&self, value: c_ulonglong) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned short.
    fn init_with_unsigned_short(&self, value: c_ushort) -> Self;

    /* Accessing Numeric Values
     */

    /// The number object's value expressed as a Boolean value.
    fn bool_value(&self) -> bool;

    /// The number object's value expressed as a char.
    fn char_value(&self) -> c_schar;

    /// The number object's value expressed as an NSDecimal structure.
    //// fn decimal_value(&self) -> NSDecimal;

    /// The number object's value expressed as a double, converted as necessary.
    fn double_value(&self) -> c_double;

    /// The number object's value expressed as a float, converted as necessary.
    fn float_value(&self) -> c_float;

    /// The number object's value expressed as an int, converted as necessary.
    fn int_value(&self) -> c_int;

    /// The number object's value expressed as an NSInteger object, converted as necessary.
    fn integer_value(&self) -> Int;

    /// The number object’s value expressed as a long long, converted as necessary.
    fn long_long_value(&self) -> c_longlong;

    /// The number object’s value expressed as a long, converted as necessary.
    fn long_value(&self) -> c_long;

    /// The number object's value expressed as a short, converted as necessary.
    fn short_value(&self) -> c_short;

    /// The number object's value expressed as an unsigned char, converted as necessary.
    fn unsigned_char_value(&self) -> c_uchar;

    /// The number object's value expressed as an NSUInteger object, converted as necessary.
    fn unsigned_integer_value(&self) -> UInt;

    /// The number object's value expressed as an unsigned int, converted as necessary.
    fn unsigned_int_value(&self) -> c_uint;

    /// The number object’s value expressed as an unsigned long long, converted as necessary.
    fn unsigned_long_long_value(&self) -> c_ulonglong;

    /// The number object's value expressed as an unsigned long, converted as necessary.
    fn unsigned_long_value(&self) -> c_ulong;

    /// The number object's value expressed as an unsigned short, converted as necessary.
    fn unsigned_short_value(&self) -> c_ushort;

    /* Retrieving String Representations
     */

    /// Returns a string that represents the contents of the number object for a given locale.
    fn description_with_locale(&self, locale: super::Locale) -> String;

    /// The number object's value expressed as a human-readable string.
    fn string_value(&self) -> String;

    /* Comparing NSNumber Objects
     */

    /// Returns an NSComparisonResult value that indicates whether the number object’s value is greater than, equal to, or less than a given number.
    fn compare(&self, other: &Self) -> ComparisonResult;

    /// Returns a Boolean value that indicates whether the number object’s value and a given number are equal.    
    fn is_equal_to_number(&self, other: &Self) -> bool;
}

/// The `Locale` class provides information about the user’s locale and formatting preferences.
pub trait NSLocale: NSObject {
    /* Initializing a Locale
     */

    /// Initializes a locale using a given locale identifier.
    fn init_with_locale_identifier<T>(locale_identifier: T) -> Self
    where
        T: Into<String>;

    /* Getting the User's Locale
     */

    /// A locale which tracks the user’s current preferences.
    fn auto_updating_current(&self) -> super::Locale;

    /// A locale representing the user's region settings at the time the property is read.
    fn current() -> super::Locale;

    /// A locale representing the generic root values with little localization.
    fn system() -> super::Locale;

    /* Getting Known Identifiers and Codes
     */

    /// The list of locale identifiers available on the system.
    fn available_locale_identifiers() -> Array<String>;

    /// The list of known country or region codes.
    fn iso_country_codes() -> Array<String>;

    /// The list of known language codes.
    fn iso_language_codes() -> Array<String>;

    /// The list of known currency codes.
    fn iso_currency_codes() -> Array<String>;

    /// A list of commonly encountered currency codes.
    fn common_isocurrency_codes() -> Array<String>;

    /* Converting Between Identifiers
     */

    /* Getting Information About a Locale
     */

    /// The identifier for the locale.
    fn locale_identifier(&self) -> String;

    /// The country or region code for the locale.
    fn country_code(&self) -> String;

    /// The language code for the locale.
    fn language_code(&self) -> String;

    /// The script code for the locale.
    fn script_code(&self) -> String;

    /// The variant code for the locale.
    fn variant_code(&self) -> String;

    /// The collation identifier for the locale.
    fn collation_identifier(&self) -> String;

    /// The collator identifier for the locale.
    fn collator_identifier(&self) -> String;

    /// A Boolean value that indicates whether the locale uses the metric system.
    fn uses_metric_system(&self) -> bool;

    /// The decimal separator for the locale.
    fn decimal_separator(&self) -> String;

    /// The grouping separator for the locale.
    fn grouping_separator(&self) -> String;

    /// The currency code for the locale.
    fn currency_code(&self) -> String;

    /// The currency symbol for the locale.
    fn currency_symbol(&self) -> String;

    /// The calendar identifier for the locale.
    fn calendar_identifier(&self) -> String;

    /// The begin quotation symbol for the locale.
    fn quotation_begin_delimiter(&self) -> String;

    /// The end quotation symbol for the locale.
    fn quotation_end_delimiter(&self) -> String;

    /// The alternate begin quotation symbol for the locale.
    fn alternate_quotation_begin_delimiter(&self) -> String;

    /// The alternate end quotation symbol for the locale.
    fn alternate_quotation_end_delimiter(&self) -> String;

    /* Accessing Locale Information by Key
     */

    /// Returns the value of the component corresponding to the specified key.
    fn object_for_key(&self, key: NSLocaleKey) -> Option<Object>;

    /// Returns the display name for the given locale component value.
    fn display_name_for_key_value<T>(&self, key: NSLocaleKey, value: T) -> Option<String>
    where
        T: Into<String>;

    /* Getting the User's Preferred Languages
     */

    /// An ordered list of the user's preferred languages.
    fn preferred_languages() -> Array<String>;

    /* Getting Line and Character Direction for a Language
     */

    /// Returns the direction of the sequence of characters in a line for the specified ISO language code.
    fn character_direction_for_language<T>(&self, iso_language_code: T) -> LanguageDirection
    where
        T: Into<String>;

    /// Returns the direction of the sequence of lines for the specified ISO language code.
    fn line_direction_for_language<T>(&self, iso_language_code: T) -> LanguageDirection
    where
        T: Into<String>;
}

/// A static ordered collection of objects.
pub trait NSArray<T> {
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
    fn from_objects(objects: &[T]) -> Self
    where
        T: NSObject;

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
    unsafe fn from_retained(array: id) -> Self;

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
        T: NSObject;

    /// The number of objects in the array.
    fn count(&self) -> UInt;

    /// The first object in the array.
    fn first_object(&self) -> Option<T>
    where
        T: NSObject;

    /// The last object in the array.
    fn last_object(&self) -> Option<T>
    where
        T: NSObject;

    /// The object at the specified index.
    fn object_at(&self, index: UInt) -> T
    where
        T: NSObject;

    /// The index of the specified object.
    fn object_at_indexed_subscript(&self, index: UInt) -> Option<id>;

    /* Finding Objects in an Array
     */

    /// Returns the lowest index whose corresponding array value is equal to a given object.
    fn index_of(&self, object: T) -> UInt
    where
        T: NSObject;

    /// Returns the lowest index within a specified range whose corresponding array value is equal to a given object .
    fn index_of_object_in_range(&self, object: T, range: Range<UInt>) -> UInt
    where
        T: NSObject;

    /// Returns the lowest index whose corresponding array value is identical to a given object.
    fn index_of_object_identical_to(&self, object: T) -> UInt
    where
        T: NSObject;

    /// Returns the lowest index within a specified range whose corresponding array value is equal to a given object .
    fn index_of_object_identical_to_in_range(&self, object: T, range: Range<UInt>) -> UInt
    where
        T: NSObject;

    /* Comparing Arrays
     */

    /// Returns the first object contained in the receiving array that’s equal to an object in another given array.
    fn first_object_common_with(&self, other: &Array<T>) -> Option<T>
    where
        T: NSObject;

    /// Compares the receiving array to another array.
    fn is_equal_to(&self, other: &Array<T>) -> bool
    where
        T: NSObject;

    /* Deriving New Arrays
     */

    /// Returns a new array that is a copy of the receiving array with a given object added to the end.
    fn adding(&self, object: T) -> Array<T>
    where
        T: NSObject;

    /// Returns a new array that is a copy of the receiving array with the objects contained in another array added to the end.
    fn adding_objects(&self, objects: &Array<T>) -> Array<T>
    where
        T: NSObject;

    /// Returns a new array containing the receiving array’s elements that fall within the limits specified by a given range.
    fn subarray_with_range(&self, range: Range<UInt>) -> Array<T>
    where
        T: NSObject;
    /* Creating a Description
     */

    /// A string that represents the contents of the array, formatted as a property list.

    /// Returns a string that represents the contents of the array, formatted as a property list.
    fn description_with_locale(&self, locale: &super::Locale) -> String;

    /// Returns a string that represents the contents of the array, formatted as a property list.
    fn description_with_locale_indent(&self, locale: &super::Locale, indent: UInt) -> String;

    /* Rust Conversions
     */

    /// Returns an iterator over the objects in the array.
    fn iter(&self) -> super::array::iter::Iter<'_, T>
    where
        T: NSObject;
}

/// A mutable, static ordered collection of objects.
pub trait NSMutableArray<T>: NSArray<T> {
    /// Creates a new `MutableArray`.
    fn new() -> Self;

    /// The number of objects in the array.
    fn count(&self) -> UInt;

    /* Creating and Initializing a Mutable Array
     */

    /// Creates and returns an NSMutableArray object with enough allocated memory to initially hold a given number of objects.
    fn array_with_capacity(capacity: usize) -> Self;

    /// Creates and returns a mutable array containing the contents of the file specified by the given path.
    fn array_with_contents_of_file<S>(path: S) -> Self
    where
        S: Into<String>;

    /// Creates and returns a mutable array containing the contents specified by a given URL.
    fn array_with_contents_of_url<S>(url: S) -> Self
    where
        S: Into<String>;

    /// Returns an array, initialized with enough memory to initially hold a given number of objects.
    fn init_with_capacity(capacity: UInt) -> Self;

    /// Initializes a newly allocated mutable array with the contents of the file specified by a given path
    fn init_with_contents_of_file<S>(&mut self, path: S) -> bool
    where
        S: Into<String>;

    /* Adding Objects
     */

    /// Inserts a given object at the end of the array.
    fn add(&mut self, object: &T);

    /// Adds the objects contained in another given array to the end of the receiving array’s content.
    fn add_objects_from_array(&mut self, other_array: &Array<T>);

    /// Inserts a given object into the array’s contents at a given index.
    fn insert(&mut self, index: UInt, object: &T);

    /* Removing Objects
     */

    /// Empties the array of all its elements.
    fn remove_all_objects(&mut self);

    /// Removes the object with the highest-valued index in the array
    fn remove_last_object(&mut self);

    /// Removes all occurrences in the array of a given object.
    fn remove_object(&mut self, object: &T);

    /// Removes all occurrences within a specified range in the array of a given object.
    fn remove_object_in_range(&mut self, object: &T, range: Range<UInt>);

    /// Removes the object at index .
    fn remove_object_at_index(&mut self, index: UInt);

    /// Removes all occurrences of a given object in the array.
    fn remove_object_identical_to(&mut self, object: &T);

    /// Removes all occurrences of anObject within the specified range in the array.
    fn remove_object_identical_to_in_range(&mut self, object: &T, range: Range<UInt>);

    /// Removes from the receiving array the objects in another given array.
    fn remove_objects_in_array(&mut self, other_array: &Array<T>);

    /// Removes from the array each of the objects within a given range.
    fn remove_objects_in_range(&mut self, range: Range<UInt>);

    /* Replacing Objects
     */

    /// Replaces the object at index with anObject.
    fn replace_object_at_index(&mut self, index: UInt, object: &T);

    /// Sets the receiving array’s elements to those in another given array.
    fn set_array(&mut self, other_array: &Array<T>);
}

/// A dynamic collection of objects associated with unique keys.
pub trait NSMutableDictionary<K, V>: NSDictionary<K, V> {
    /// Creates and initialize a dictionary
    fn init_with_dictionary(&mut self, dictionary: super::Dictionary<K, V>);

    /* Adding Entries to a Mutable Dictionary
     */

    /// Adds a given key-value pair to the dictionary.
    fn set_object(&mut self, key: K, value: V)
    where
        K: NSObject,
        V: NSObject;

    /// Adds a given key-value pair to the dictionary.
    fn set_object_for_keyed_superscript(&mut self, key: K, value: V)
    where
        K: Into<id>,
        V: Into<id>;

    /// Adds a given key-value pair to the dictionary.
    fn set_value(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<id>;

    /// Adds to the receiving dictionary the entries from another dictionary.
    fn add_entries_from_dictionary(&mut self, dictionary: super::Dictionary<K, V>);

    /// Sets the contents of the receiving dictionary to entries in a given dictionary.
    fn set_dictionary(&mut self, dictionary: super::Dictionary<K, V>);

    /* Removing Entries From a Mutable Dictionary
     */

    /// Removes a given key and its associated value from the dictionary.
    fn remove_object_for_key(&mut self, key: K)
    where
        K: Into<id>;

    /// Empties the dictionary of its entries.
    fn remove_all_objects(&mut self);

    /// Removes from the dictionary entries specified by elements in a given array.
    fn remove_objects_for_keys(&mut self, keys: Array<K>)
    where
        K: NSObject;
}

/// A static collection of objects associated with unique keys.
pub trait NSDictionary<K, V>: NSObject {
    /* Creating an Empty Dictionary
     */

    /// Creates an empty dictionary.
    fn new() -> Self;

    /* Creating a Dictionary from Objects and Keys
     */

    /// Creates a dictionary containing entries constructed from the contents of an array of keys and an array of values.
    fn dictionary_with_objects(objects: Array<V>, keys: Array<K>) -> Self;

    /// Creates a mutable dictionary containing entries constructed from the contents of an array of keys and an array of values.
    fn as_mut_dictionary(&mut self) -> super::MutableDictionary<K, V>;

    /* Counting Entries
     */

    /// The number of entries in the dictionary.
    fn count(&self) -> UInt;
}
