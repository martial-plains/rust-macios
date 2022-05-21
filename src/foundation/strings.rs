use std::{
    ffi::{CStr, CString},
    fmt::{self, Debug},
    marker::PhantomData,
    ops::{Deref, DerefMut, Range},
    slice,
};

use libc::c_char;
use objc::{
    class, msg_send,
    runtime::{Object, BOOL, NO},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{id, objective_c_runtime::NSObjectProtocol, utils::to_bool};

use super::{ComparisonResult, Int, Locale, UInt};

/// Constants representing an ICU string transform.
pub type StringTransform = *const String;

#[allow(improper_ctypes)]
#[link(name = "Foundation", kind = "framework")]
extern "C" {
    /* Transliteration
     */

    /// A constant containing the transliteration of a string from any script to Latin script.
    #[link_name = "NSStringTransformToLatin"]
    pub static ToLatin: StringTransform;

    /// LatinToArabic
    #[link_name = "NSStringTransformLatinToArabic"]
    pub static LatinToArabic: StringTransform;

    /// A constant containing the transliteration of a string from Latin script to Cyrillic script.
    #[link_name = "NSStringTransformLatinToCyrillic"]
    pub static LatinToCyrillic: StringTransform;

    /// A constant containing the transliteration of a string from Latin script to Greek script.
    #[link_name = "NSStringTransformLatinToGreek"]
    pub static LatinToGreek: StringTransform;

    /// A constant containing the transliteration of a string from Latin script to Hangul script.
    #[link_name = "NSStringTransformLatinToHangul"]
    pub static LatinToHangul: StringTransform;

    /// A constant containing the transliteration of a string from Latin script to Hebrew script.
    #[link_name = "NSStringTransformLatinToHebrew"]
    pub static LatinToHebrew: StringTransform;

    /// A constant containing the transliteration of a string from Latin script to Hiragana script.
    #[link_name = "NSStringTransformLatinToHiragana"]
    pub static LatinToHiragana: StringTransform;

    /// A constant containing the transliteration of a string from Latin script to Katakana script.
    #[link_name = "NSStringTransformLatinToKatakana"]
    pub static LatinToKatakana: StringTransform;

    /// A constant containing the transliteration of a string from Latin script to Thai script.
    #[link_name = "NSStringTransformLatinToThai"]
    pub static LatinToThai: StringTransform;

    /// A constant containing the transliteration of a string from Hiragana script to Katakana script.
    #[link_name = "NSStringTransformHiraganaToKatakana"]
    pub static HiraganaToKatakana: StringTransform;

    /// A constant containing the transliteration of a string from Han script to Latin.
    #[link_name = "NSStringTransformHanziToLatin"]
    pub static MandarinToLatin: StringTransform;

    /* Diacritic and Combining Mark Removal
     */

    /// A constant containing the transformation of a string by removing diacritics.
    #[link_name = "NSStringTransformStripDiacritics"]
    pub static StripDiacritics: StringTransform;

    /// A constant containing the transformation of a string by removing combining marks.
    #[link_name = "NSStringTransformStripCombiningMarks"]
    pub static StripCombiningMarks: StringTransform;

    /* Halfwidth and Fullwidth Form Conversion
     */

    /// A constant containing the transformation of a string from full-width CJK characters to half-width forms.
    #[link_name = "NSStringTransformFullwidthToHalfwidth"]
    pub static FullwidthToHalfwidth: StringTransform;

    /* Character Representation
     */

    /// An identifier for a transform that converts characters to Unicode names.
    #[link_name = "NSStringTransformToUnicodeName"]
    pub static ToUnicodeName: StringTransform;

    /// A constant containing the transformation of a string from characters to XML hexadecimal escape codes.
    #[link_name = "NSStringTransformToXMLHex"]
    pub static ToXMLHex: StringTransform;

}

/// Size of UTF8 encoding
const UTF8_ENCODING: usize = 4;

/// The following constants are provided by NSString as possible string encodings.
#[allow(clippy::enum_clike_unportable_variant)]
#[derive(Debug)]
#[repr(u64)]
pub enum Encoding {
    /// Strict 7-bit ASCII encoding within 8-bit chars; ASCII values 0…127 only.
    ASCII = 1,
    /// 8-bit ASCII encoding with NEXTSTEP extensions.
    NEXTSTEP = 2,
    /// 8-bit EUC encoding for Japanese text.
    JapaneseEUC = 3,
    /// An 8-bit representation of Unicode characters, suitable for transmission or storage by ASCII-based systems.
    UTF8 = 4,
    /// 8-bit ISO Latin 1 encoding.
    ISOLatin1 = 5,
    /// 8-bit Adobe Symbol encoding vector.
    Symbol = 6,
    /// 7-bit verbose ASCII to represent all Unicode characters.
    NonLossyASCII = 7,
    /// 8-bit Shift-JIS encoding for Japanese text.
    ShiftJIS = 8,
    /// 8-bit ISO Latin 2 encoding.
    ISOLatin2 = 9,
    /// The canonical Unicode encoding for string objects.
    Unicode = 10,
    /// Microsoft Windows codepage 1251, encoding Cyrillic characters; equivalent to AdobeStandardCyrillic font encoding.
    WindowsCP1251 = 11,
    /// Microsoft Windows codepage 1252; equivalent to WinLatin1.
    WindowsCP1252 = 12,
    /// Microsoft Windows codepage 1253, encoding Greek characters.
    WindowsCP1253 = 13,
    /// Microsoft Windows codepage 1254, encoding Turkish characters.
    WindowsCP1254 = 14,
    /// Microsoft Windows codepage 1250; equivalent to WinLatin2.
    WindowsCP1250 = 15,
    /// ISO 2022 Japanese encoding for email.
    ISO2022JP = 21,
    /// Classic Macintosh Roman encoding.
    MacOSRoman = 30,
    /// UTF16 encoding with explicit endianness specified.
    UTF16BigEndian = 0x90000100,
    /// UTF16 encoding with explicit endianness specified.
    UTF16LittleEndian = 0x94000100,
    /// 32-bit UTF encoding.
    UTF32 = 0x8c000100,
    /// UTF32 encoding with explicit endianness specified.
    UTF32BigEndian = 0x98000100,
    /// UTF32 encoding with explicit endianness specified.
    UTF32LittleEndian = 0x9c000100,
    /// Installation-specific encoding.
    #[deprecated(note = "This encoding has been deprecated—there is no replacement.")]
    Proprietary = 65536,
}

impl Encoding {
    /// An alias for Unicode.
    pub const UTF16: Self = Encoding::Unicode;
}

/// These values represent the options available to many of the string classes’ search and comparison methods.
#[derive(Debug, PartialEq, Eq)]
pub enum CompareOptions {
    /// A case-insensitive search.
    CaseInsensitive = 1,
    /// Exact character-by-character equivalence.
    Literal = 2,
    /// Search from end of source string.
    Backwards = 4,
    /// Search is limited to start (or end, if NSBackwardsSearch) of source string.
    Anchored = 8,
    /// Numbers within strings are compared using numeric value, that is, Name2.txt < Name7.txt < Name25.txt.
    Numeric = 64,
    /// Search ignores diacritic marks.
    DiacriticInsensitive = 128,
    /// Search ignores width differences in characters that have full-width and half-width forms, as occurs in East Asian character sets.
    WidthInsensitive = 256,
    /// Comparisons are forced to return either NSOrderedAscending or NSOrderedDescending if the strings are equivalent but not strictly equal.
    ForcedOrdering = 512,
    /// The search string is treated as an ICU-compatible regular expression. If set, no other options can apply except caseInsensitive and anchored. You can use this option only with the rangeOfString:… methods and replacingOccurrences(of:with:options:range:).
    RegularExpression = 1024,
}

impl CompareOptions {
    /// Creates a new `CompareOptions` with the given flags.
    pub fn new(raw_value: usize) -> Self {
        match raw_value {
            1 => CompareOptions::CaseInsensitive,
            2 => CompareOptions::Literal,
            4 => CompareOptions::Backwards,
            8 => CompareOptions::Anchored,
            64 => CompareOptions::Numeric,
            128 => CompareOptions::DiacriticInsensitive,
            256 => CompareOptions::WidthInsensitive,
            512 => CompareOptions::ForcedOrdering,
            1024 => CompareOptions::RegularExpression,
            _ => panic!("Unknown CompareOptions value: {}", raw_value),
        }
    }
}

/// This is a mapping to the Objective-C NSString class.
#[repr(C)]
pub struct String {
    /// The raw pointer to the Objective-C object.
    pub objc: Id<Object>,
    marker: PhantomData<()>,
}

impl String {}

impl String {
    /// Creates a new `NSString`
    pub fn new() -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(msg_send![nsstring, init])
        };

        String {
            objc,
            marker: PhantomData,
        }
    }

    /// Creates a new `NSString` from a string slice. without copying the bytes.
    ///
    /// # Arguments
    ///
    /// * `s` - The string slice to create the `NSString` from.
    pub fn init_with_no_cpy_str(s: &str) -> Self {
        String {
            objc: unsafe {
                let nsstring: id = msg_send![class!(NSString), alloc];
                Id::from_ptr(msg_send![nsstring, initWithBytesNoCopy:s.as_ptr()
                    length:s.len()
                    encoding:UTF8_ENCODING
                    freeWhenDone:NO
                ])
            },

            marker: PhantomData,
        }
    }

    /// In some cases, we want to wrap a system-provided NSString without retaining it.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it dereferrences a raw pointer.
    pub unsafe fn from_retained(object: id) -> Self {
        String {
            objc: Id::from_retained_ptr(object),
            marker: PhantomData,
        }
    }

    /// Utility method for checking whether an `NSObject` is an `NSString`.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it dereferrences a raw pointer.
    pub unsafe fn is(obj: id) -> bool {
        let result: BOOL = msg_send![obj, isKindOfClass: class!(NSString)];
        to_bool(result)
    }

    /// Returns the UTF8 bytes for this `NSString`.
    fn bytes(&self) -> *const u8 {
        unsafe {
            let bytes: *const c_char = msg_send![&*self.objc, UTF8String];
            bytes as *const u8
        }
    }

    /// Gets the proper byte length for this `NSString` (the UTF8 variant).
    fn bytes_len(&self) -> UInt {
        unsafe { msg_send![&*self.objc, lengthOfBytesUsingEncoding: UTF8_ENCODING] }
    }

    /// Convert this `NSString` into a `&str`.
    pub fn as_str(&self) -> &str {
        let bytes = self.bytes();
        let len = self.bytes_len();

        unsafe {
            let bytes = slice::from_raw_parts(bytes, len as usize);
            std::str::from_utf8(bytes).unwrap()
        }
    }

    /* Getting a String’s Length
     */

    /// The number of Unicode characters in this string.
    pub fn length(&self) -> Int {
        unsafe { msg_send![&*self.objc, length] }
    }

    /// Returns the number of bytes required to store the receiver in a given encoding.
    ///
    /// # Arguments
    ///
    /// * `enc` - The encoding for which to determine the receiver's length.
    pub fn length_of_bytes(&self, enc: Encoding) -> Int {
        unsafe { msg_send![&*self.objc, lengthOfBytesUsingEncoding: enc] }
    }

    /// Returns the maximum number of bytes needed to store the receiver in a given encoding.
    ///
    /// # Arguments
    ///
    /// * `enc` - The encoding for which to determine the receiver's length.
    pub fn maximum_length_of_bytes(&self, enc: Encoding) -> Int {
        unsafe { msg_send![&*self.objc, maximumLengthOfBytesUsingEncoding: enc] }
    }

    /* Getting Characters and Bytes
     */

    /// Returns the character at a given UTF-16 code unit index.
    ///
    /// # Arguments
    ///
    /// * `index` - The character at the array position given by `index`.
    pub fn character_at(&self, index: Int) -> char {
        unsafe { msg_send![&*self.objc, characterAtIndex: index] }
    }

    /* Identifying and Comparing Strings
     */

    /// Returns the result of invoking compare:options: with NSCaseInsensitiveSearch as the only option.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare.
    pub fn case_insensitive_compare<T>(&self, string: T) -> ComparisonResult
    where
        T: Into<String>,
    {
        unsafe { msg_send![&*self.objc, caseInsensitiveCompare: string.into()] }
    }

    /// Compares the string with a given string using a case-insensitive, localized, comparison.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare.
    pub fn localized_case_insensitive_compare<T>(&self, string: T) -> ComparisonResult
    where
        T: Into<String>,
    {
        unsafe { msg_send![&*self.objc, localizedCaseInsensitiveCompare: string.into()] }
    }

    /// Returns the result of invoking compare:options:range: with no options and the receiver’s full extent as the range.
    ///
    /// # Arguments
    ///
    /// * `string` - The string with which to compare the receiver.
    ///
    /// # Safety
    ///
    /// This value must not be nil. If this value is nil, the behavior is undefined and may change in future versions of macOS.
    pub fn compare<T>(&self, string: T) -> ComparisonResult
    where
        T: Into<String>,
    {
        unsafe { msg_send![&*self.objc, compare: string.into()] }
    }
    /// Returns the result of invoking compare:options:range: with no options and the receiver’s full extent as the range.
    pub fn localized_compare<T>(&self, string: &T) -> ComparisonResult
    where
        T: Into<String>,
    {
        unsafe { msg_send![&*self.objc, localizedCompare: string] }
    }

    /// Compares strings as sorted by the Finder.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare with the receiver.
    ///
    /// # Returns
    ///
    /// The result of the comparison.
    pub fn localized_standard_compare<T>(&self, string: T) -> ComparisonResult
    where
        T: Into<String>,
    {
        unsafe { msg_send![&*self.objc, localizedStandardCompare: string.into()] }
    }

    /// Compares the string with the specified string using the given options.
    pub fn compare_with_options<T>(&self, string: T, options: CompareOptions) -> ComparisonResult
    where
        T: Into<String>,
    {
        unsafe { msg_send![&*self.objc, compare: string.into() options: options] }
    }

    /// Returns the result of invoking compare(_:options:range:locale:) with a nil locale.
    pub fn compare_with_options_range<T>(
        &self,
        string: T,
        options: CompareOptions,
        range: Range<UInt>,
    ) -> ComparisonResult
    where
        T: Into<String>,
    {
        unsafe { msg_send![&*self.objc, compare: string.into() options: options range: range] }
    }

    /// Compares the string using the specified options and returns the lexical ordering for the range.
    pub fn compare_with_options_range_locale<T>(
        &self,
        string: T,
        options: CompareOptions,
        range: Range<UInt>,
        locale: Locale,
    ) -> ComparisonResult
    where
        T: Into<String>,
    {
        unsafe {
            msg_send![&*self.objc, compare: string.into() options: options range: range locale: locale]
        }
    }

    /// Returns a Boolean value that indicates whether a given string matches the beginning characters of the receiver.
    ///
    /// # Arguments
    ///
    /// * `prefix` - The prefix to search for.
    pub fn has_prefix<T>(&self, prefix: T) -> bool
    where
        T: Into<String>,
    {
        unsafe {
            let prefix: String = prefix.into();
            let result: BOOL = msg_send![&*self.objc, hasPrefix: prefix];
            to_bool(result)
        }
    }

    /// Returns a Boolean value that indicates whether a given string matches the ending characters of the receiver.
    ///
    /// # Arguments
    ///
    /// * `suffix` - The suffix to search for.
    pub fn has_suffix<T>(&self, suffix: T) -> bool
    where
        T: Into<String>,
    {
        unsafe {
            let suffix: String = suffix.into();
            let result: BOOL = msg_send![&*self.objc, hasSuffix: suffix];
            to_bool(result)
        }
    }

    /// Returns a Boolean value that indicates whether a given string is equal to the receiver using a literal Unicode-based comparison.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to compare with the receiver.
    pub fn is_equal_to<T>(&self, string: T) -> bool
    where
        T: Into<String>,
    {
        unsafe {
            let string: String = string.into();
            let result: BOOL = msg_send![&*self.objc, isEqualToString: string];
            to_bool(result)
        }
    }

    /* Combining Strings
     */

    /// Returns a new string made by appending a given string to the receiver.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to append to the receiver. This value must not be nil.
    pub fn appending<T>(&self, string: T) -> String
    where
        T: Into<String>,
    {
        unsafe { msg_send![&*self.objc, stringByAppendingString: string.into()] }
    }

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
    pub fn padding<T>(&self, new_length: Int, pad_string: T, starting_at: Int) -> String
    where
        T: Into<String>,
    {
        unsafe {
            msg_send![&*self.objc, stringByPaddingToLength: new_length withString: pad_string.into() startingAtIndex: starting_at]
        }
    }

    /* Finding Characters and Substrings
     */

    /// Returns a boolean value indicating whether the string contains a given string by performing a case-sensitive, locale-unaware search.
    pub fn contains<T>(&self, other: T) -> bool
    where
        T: Into<String>,
    {
        unsafe {
            let other: String = other.into();
            let result: BOOL = msg_send![&*self.objc, containsString: other];
            to_bool(result)
        }
    }

    /* Changing Case
     */

    /// A lowercase representation of the string.
    pub fn lowercased(&self) -> String {
        unsafe { msg_send![&*self.objc, lowercaseString] }
    }

    /// Returns a version of the string with all letters converted to lowercase, taking into account the current locale.
    pub fn localized_lowercase(&self) -> String {
        unsafe { msg_send![&*self.objc, localizedLowercaseString] }
    }

    /// An uppercase representation of the string.
    pub fn uppercased(&self) -> String {
        unsafe { msg_send![&*self.objc, uppercaseString] }
    }

    /// Returns a version of the string with all letters converted to uppercase, taking into account the current locale.
    pub fn localized_uppercase(&self) -> String {
        unsafe { msg_send![&*self.objc, localizedUppercaseString] }
    }

    /// A capitalized representation of the string.
    pub fn capitalized(&self) -> String {
        unsafe { msg_send![&*self.objc, capitalizedString] }
    }

    /// Returns a capitalized representation of the receiver using the current locale.
    pub fn localized_capitalized(&self) -> String {
        unsafe { msg_send![&*self.objc, localizedCapitalizedString] }
    }

    /* Transforming Strings
     */

    /// Returns a new string made by appending to the receiver a given string.
    pub fn applying_transform(
        &mut self,
        transform: StringTransform,
        reverse: bool,
    ) -> Option<String> {
        let result: id = unsafe {
            msg_send![
            &*self.objc,
            stringByApplyingTransform: transform
            reverse: reverse
            ]
        };
        if result.is_null() {
            None
        } else {
            Some(result.into())
        }
    }
}

impl NSObjectProtocol for String {
    fn description(&self) -> String {
        unsafe {
            let description: id = msg_send![&*self.objc, description];
            description.into()
        }
    }

    fn debug_description(&self) -> String {
        unsafe {
            let description: id = msg_send![&*self.objc, debugDescription];
            description.into()
        }
    }
}

impl Default for String {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description().as_str())
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.debug_description().as_str())
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        unsafe { msg_send![&*self.objc, retain] }
    }
}

impl From<String> for id {
    /// Consumes and returns the pointer to the underlying NSString instance.
    fn from(mut string: String) -> Self {
        &mut *string.objc
    }
}

impl From<id> for String {
    /// Creates a new String from a pointer to an NSString instance.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from(objc: id) -> Self {
        String {
            objc: unsafe { Id::from_ptr(objc) },
            marker: PhantomData,
        }
    }
}

impl Deref for String {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.objc
    }
}

impl DerefMut for String {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.objc
    }
}

impl From<std::string::String> for String {
    /// Creates a new `NSString` from a `String`.
    fn from(s: std::string::String) -> Self {
        String {
            objc: unsafe {
                let nsstring: id = msg_send![class!(NSString), alloc];
                Id::from_ptr(msg_send![nsstring, initWithString:s.as_str()])
            },

            marker: PhantomData,
        }
    }
}

impl From<&str> for String {
    /// Creates a new `NSString` from a `&str`.
    fn from(s: &str) -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(
                msg_send![nsstring, initWithBytes: CString::new(s).unwrap().into_raw() as *mut Object
                    length:s.len()
                    encoding:UTF8_ENCODING
                ],
            )
        };

        String {
            objc,
            marker: PhantomData,
        }
    }
}

impl From<char> for String {
    /// Creates a new `NSString` from a `char`.
    fn from(c: char) -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(
                msg_send![nsstring, initWithBytes: c.encode_utf8(&mut [0; 4]).as_ptr()
                    length:1
                    encoding:UTF8_ENCODING
                ],
            )
        };

        String {
            objc,
            marker: PhantomData,
        }
    }
}

impl From<String> for &str {
    /// Converts a `NSString` to a `&str`.
    fn from(string: String) -> Self {
        unsafe {
            let ptr: *const c_char = msg_send![string.objc, UTF8String];
            CStr::from_ptr(ptr).to_str().unwrap()
        }
    }
}

impl From<(&str, Encoding)> for String {
    /// Creates a new `NSString` from a `&str` and an encoding.
    fn from((s, encoding): (&str, Encoding)) -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(msg_send![nsstring, initWithBytes:s.as_ptr()
                length:s.len()
                encoding:encoding
            ])
        };

        String {
            objc,
            marker: PhantomData,
        }
    }
}

impl PartialEq for String {
    /// Checks if two `NSString`s are equal.
    fn eq(&self, other: &Self) -> bool {
        self.localized_compare(other) == ComparisonResult::OrderedSame
    }
}

impl PartialEq<&str> for String {
    /// Checks if a `NSString` is equal to a `&str`.
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

#[cfg(test)]
mod tests {
    use crate::foundation::ComparisonResult;

    use super::*;

    #[test]
    fn test_from_str() {
        let s = String::from("Hello, World!");
        assert_eq!(s.as_str(), "Hello, World!");
    }

    #[test]
    fn test_from_no_cpy_str() {
        let s = String::init_with_no_cpy_str("Hello, World!");
        assert_eq!(s.as_str(), "Hello, World!");
    }

    #[test]
    fn test_bytes_len() {
        let s = String::from("Hello, World!");
        assert_eq!(s.bytes_len(), 13);
    }

    #[test]
    fn test_as_str() {
        let s = String::from("Hello, World!");
        assert_eq!(s.as_str(), "Hello, World!");
    }

    #[test]
    fn test_to_string() {
        let s = String::from("Hello, World!");
        assert_eq!(s.to_string(), "Hello, World!".to_string());
    }

    #[test]
    fn test_length() {
        let s = String::from("Hello, World!");
        assert_eq!(s.length(), 13);
    }

    #[test]
    fn test_contains() {
        let s = String::from("Hello, World!");
        assert!(s.contains("Hello"));
        assert!(!s.contains("Goodbye"));
    }

    #[test]
    fn test_character_at() {
        let s = String::from("Hello, World!");
        assert_eq!(s.character_at(0), 'H');
        assert_eq!(s.character_at(1), 'e');
        assert_eq!(s.character_at(2), 'l');
        assert_eq!(s.character_at(3), 'l');
        assert_eq!(s.character_at(4), 'o');
        assert_eq!(s.character_at(5), ',');
        assert_eq!(s.character_at(6), ' ');
        assert_eq!(s.character_at(7), 'W');
        assert_eq!(s.character_at(8), 'o');
        assert_eq!(s.character_at(9), 'r');
        assert_eq!(s.character_at(10), 'l');
        assert_eq!(s.character_at(11), 'd');
        assert_eq!(s.character_at(12), '!');
    }

    #[test]
    fn test_has_prefix() {
        let s = String::from("Hello, World!");
        assert!(s.has_prefix("Hello"));
        assert!(!s.has_prefix("Goodbye"));
    }

    #[test]
    fn test_has_suffix() {
        let s = String::from("Hello, World!");
        assert!(s.has_suffix("World!"));
        assert!(!s.has_suffix("Goodbye"));
    }

    #[test]
    fn test_is_equal_to() {
        let s = String::from("Hello, World!");
        assert!(s.is_equal_to("Hello, World!"));
        assert!(!s.is_equal_to("Goodbye, World!"));
    }

    #[test]
    fn test_length_of_bytes() {
        let s = String::from("Hello, World!");
        assert_eq!(s.length_of_bytes(Encoding::UTF8), 13);
    }

    #[test]
    fn test_maximum_length_of_bytes() {
        let s = String::from("Hello, World!");
        assert_eq!(s.maximum_length_of_bytes(Encoding::UTF8), 39);
    }

    #[test]
    fn test_case_insensitive_compare() {
        let s = String::from("Hello, World!");
        assert_eq!(
            s.case_insensitive_compare("hello, world!"),
            ComparisonResult::OrderedSame
        );
        assert_eq!(
            s.case_insensitive_compare("goodbye, world!"),
            ComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_localized_case_insensitive_compare() {
        let s = String::from("Hello, World!");
        assert_eq!(
            s.localized_case_insensitive_compare("hello, world!"),
            ComparisonResult::OrderedSame
        );
        assert_eq!(
            s.localized_case_insensitive_compare("goodbye, world!"),
            ComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_compare() {
        let s = String::from("Hello, World!");
        assert_eq!(s.compare("Hello, World!"), ComparisonResult::OrderedSame);
        assert_eq!(
            s.compare("Goodbye, World!"),
            ComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_localized_standard_compare() {
        let s = String::from("Hello, World!");
        assert_eq!(
            s.localized_standard_compare("Hello, World!"),
            ComparisonResult::OrderedSame
        );
        assert_eq!(
            s.localized_standard_compare("Goodbye, World!"),
            ComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_applying_transform() {
        let mut s = String::from("Katakana!");
        let transform = unsafe { LatinToKatakana };
        assert_eq!(s.applying_transform(transform, false).unwrap(), "カタカナ!");
    }
}
