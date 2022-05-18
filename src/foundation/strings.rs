use objc::{msg_send, runtime::BOOL, sel, sel_impl};

use crate::objc::to_bool;
pub use crate::objc::NSString as String;

use super::{ComparisonResult, Int};

/// Constants representing an ICU string transform.
pub type StringTransform<'a> = *const String<'a>;

#[allow(improper_ctypes)]
#[link(name = "Foundation", kind = "framework")]
extern "C" {
    /* Transliteration
     */

    /// A constant containing the transliteration of a string from any script to Latin script.
    #[link_name = "NSStringTransformToLatin"]
    pub static ToLatin: StringTransform<'static>;

    /// LatinToArabic
    #[link_name = "NSStringTransformLatinToArabic"]
    pub static LatinToArabic: StringTransform<'static>;

    /// A constant containing the transliteration of a string from Latin script to Cyrillic script.
    #[link_name = "NSStringTransformLatinToCyrillic"]
    pub static LatinToCyrillic: StringTransform<'static>;

    /// A constant containing the transliteration of a string from Latin script to Greek script.
    #[link_name = "NSStringTransformLatinToGreek"]
    pub static LatinToGreek: StringTransform<'static>;

    /// A constant containing the transliteration of a string from Latin script to Hangul script.
    #[link_name = "NSStringTransformLatinToHangul"]
    pub static LatinToHangul: StringTransform<'static>;

    /// A constant containing the transliteration of a string from Latin script to Hebrew script.
    #[link_name = "NSStringTransformLatinToHebrew"]
    pub static LatinToHebrew: StringTransform<'static>;

    /// A constant containing the transliteration of a string from Latin script to Hiragana script.
    #[link_name = "NSStringTransformLatinToHiragana"]
    pub static LatinToHiragana: StringTransform<'static>;

    /// A constant containing the transliteration of a string from Latin script to Katakana script.
    #[link_name = "NSStringTransformLatinToKatakana"]
    pub static LatinToKatakana: StringTransform<'static>;

    /// A constant containing the transliteration of a string from Latin script to Thai script.
    #[link_name = "NSStringTransformLatinToThai"]
    pub static LatinToThai: StringTransform<'static>;

    /// A constant containing the transliteration of a string from Hiragana script to Katakana script.
    #[link_name = "NSStringTransformHiraganaToKatakana"]
    pub static HiraganaToKatakana: StringTransform<'static>;

    /// A constant containing the transliteration of a string from Han script to Latin.
    #[link_name = "NSStringTransformHanziToLatin"]
    pub static MandarinToLatin: StringTransform<'static>;

    /* Diacritic and Combining Mark Removal
     */

    /// A constant containing the transformation of a string by removing diacritics.
    #[link_name = "NSStringTransformStripDiacritics"]
    pub static StripDiacritics: StringTransform<'static>;

    /// A constant containing the transformation of a string by removing combining marks.
    #[link_name = "NSStringTransformStripCombiningMarks"]
    pub static StripCombiningMarks: StringTransform<'static>;

    /* Halfwidth and Fullwidth Form Conversion
     */

    /// A constant containing the transformation of a string from full-width CJK characters to half-width forms.
    #[link_name = "NSStringTransformFullwidthToHalfwidth"]
    pub static FullwidthToHalfwidth: StringTransform<'static>;

    /* Character Representation
     */

    /// An identifier for a transform that converts characters to Unicode names.
    #[link_name = "NSStringTransformToUnicodeName"]
    pub static ToUnicodeName: StringTransform<'static>;

    /// A constant containing the transformation of a string from characters to XML hexadecimal escape codes.
    #[link_name = "NSStringTransformToXMLHex"]
    pub static ToXMLHex: StringTransform<'static>;

}

/// The following constants are provided by NSString as possible string encodings.
#[allow(clippy::enum_clike_unportable_variant)]
#[derive(Debug)]
#[repr(C)]
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

impl<'a> String<'_> {
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
        T: Into<String<'a>>,
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
        T: Into<String<'a>>,
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
        T: Into<String<'a>>,
    {
        unsafe { msg_send![&*self.objc, compare: string.into()] }
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
        T: Into<String<'a>>,
    {
        unsafe { msg_send![&*self.objc, localizedStandardCompare: string.into()] }
    }

    /// Returns a Boolean value that indicates whether a given string matches the beginning characters of the receiver.
    ///
    /// # Arguments
    ///
    /// * `prefix` - The prefix to search for.
    pub fn has_prefix<T>(&self, prefix: T) -> bool
    where
        T: Into<String<'a>>,
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
        T: Into<String<'a>>,
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
        T: Into<String<'a>>,
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
        T: Into<String<'a>>,
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
        T: Into<String<'a>>,
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
        T: Into<String<'a>>,
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
    pub fn lowercased(&self) -> String<'a> {
        unsafe { msg_send![&*self.objc, lowercaseString] }
    }

    /// Returns a version of the string with all letters converted to lowercase, taking into account the current locale.
    pub fn localized_lowercase(&self) -> String<'a> {
        unsafe { msg_send![&*self.objc, localizedLowercaseString] }
    }

    /// An uppercase representation of the string.
    pub fn uppercased(&self) -> String<'a> {
        unsafe { msg_send![&*self.objc, uppercaseString] }
    }

    /// Returns a version of the string with all letters converted to uppercase, taking into account the current locale.
    pub fn localized_uppercase(&self) -> String<'a> {
        unsafe { msg_send![&*self.objc, localizedUppercaseString] }
    }

    /// A capitalized representation of the string.
    pub fn capitalized(&self) -> String<'a> {
        unsafe { msg_send![&*self.objc, capitalizedString] }
    }

    /// Returns a capitalized representation of the receiver using the current locale.
    pub fn localized_capitalized(&self) -> String<'a> {
        unsafe { msg_send![&*self.objc, localizedCapitalizedString] }
    }

    /* Transforming Strings
     */

    /// Returns a new string made by appending to the receiver a given string.
    pub fn applying_transform(&mut self, transform: StringTransform, reverse: bool) -> String<'a> {
        unsafe {
            msg_send![
            &*self.objc,
            stringByApplyingTransform: transform
            reverse: reverse
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::foundation::ComparisonResult;

    use super::*;

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
}
