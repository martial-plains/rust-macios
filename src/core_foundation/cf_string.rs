#![allow(missing_docs)]

use std::{
    borrow::Cow,
    ffi::{c_double, CStr, VaList},
    fmt,
};

use libc::{c_char, c_uchar, c_ulong, c_void};

use bitflags::bitflags;

use crate::{
    core_foundation::CFRange,
    kernel::{Boolean, SInt32, UInt32, UInt8, UniChar},
};

use self::iter::Iter;

use super::{
    cf_array::CFArrayRef, kCFAllocatorDefault, macros::declare_CFType, CFAllocatorRef, CFArray,
    CFCharacterSetRef, CFComparisonResult, CFData, CFDataRef, CFDictionaryRef, CFIndex,
    CFLocaleRef, CFOptionFlags, CFTypeID, CFTypeObject,
};

/// For function parameters only - means string is const.
pub type ConstStr255Param = *const c_uchar;

/// Pointer to a pascal string.
pub type StringPtr = *mut c_uchar;

/// A complete Unicode character in UTF-32 format, with values from 0 through 0x10FFFF (excluding the surrogate range 0xD800-0xDFFF and certain disallowed values).
pub type UTF32Char = u32;

pub mod iter;

#[derive(Debug)]
#[repr(C)]
pub struct __CFString(c_void);

/// A reference to a CFString object.
pub type CFStringRef = *const __CFString;

/// An integer type for constants used to specify supported string encodings in various CFString functions.
pub type CFStringEncoding = u32;

const KCF_STRING_INLINE_BUFFER_LENGTH: usize = 64;

bitflags! {
    /// An integer type for constants used to specify supported string encodings in various CFString functions.
    pub struct KCFStringEncoding: CFStringEncoding {
        /// An encoding constant that identifies the Mac Roman encoding.
        const MAC_ROMAN = 0;
        /// An encoding constant that identifies the Windows Latin 1 encoding (ANSI codepage 1252).
        const WINDOWS_LATIN1 = 0x0500;
        /// An encoding constant that identifies the ISO Latin 1 encoding (ISO 8859-1)
        const ISOLATIN1 = 0x0201;
        /// An encoding constant that identifies the NextStep/OpenStep encoding.
        const NEXT_STEP_LATIN = 0x0B01;
        /// An encoding constant that identifies the ASCII encoding (decimal values 0 through 127).
        const ASCII = 0x0600;
        /// An encoding constant that identifies the Unicode encoding.
        const UNICODE = 0x0100;
        /// An encoding constant that identifies the UTF 8 encoding.
        const UTF8 = 0x08000100;
        /// An encoding constant that identifies non-lossy ASCII encoding.
        const NON_LOSSY_ASCII = 0x0BFF;
        /// An encoding constant that identifies kTextEncodingUnicodeDefault + kUnicodeUTF16Format encoding (alias of Unicode).
        const UTF16 = 0x0100;
        /// An encoding constant that identifies kTextEncodingUnicodeDefault + kUnicodeUTF16BEFormat encoding. This constant specifies big-endian byte order.
        const UTF16BE = 0x10000100;
        /// An encoding constant that identifies kTextEncodingUnicodeDefault + kUnicodeUTF16LEFormat encoding. This constant specifies little-endian byte order.
        const UTF16LE = 0x14000100;
        /// An encoding constant that identifies kTextEncodingUnicodeDefault + kUnicodeUTF32Format encoding.
        const UTF32 = 0x0c000100;
        /// An encoding constant that identifies kTextEncodingUnicodeDefault + kUnicodeUTF32BEFormat encoding. This constant specifies big-endian byte order.
        const UTF32BE = 0x18000100;
        /// An encoding constant that identifies kTextEncodingUnicodeDefault + kUnicodeUTF32LEFormat encoding. This constant specifies little-endian byte order.
        const UTF32LE = 0x1c000100;

        const MAC_JAPANESE = 1;
        const MAC_CHINESE_TRAD = 2;
        const MAC_KOREAN = 3;
        const MAC_ARABIC = 4;
        const MAC_HEBREW = 5;
        const MAC_GREEK = 6;
        const MAC_CYRILLIC = 7;
        const MAC_DEVANAGARI = 9;
        const MAC_GURMUKHI = 10;
        const MAC_GUJARATI = 11;
        const MAC_ORIYA = 12;
        const MAC_BENGALI = 13;
        const MAC_TAMIL = 14;
        const MAC_TELUGU = 15;
        const MAC_KANNADA = 16;
        const MAC_MALAYALAM = 17;
        const MAC_SINHALESE = 18;
        const MAC_BURMESE = 19;
        const MAC_KHMER = 20;
        const MAC_THAI = 21;
        const MAC_LAOTIAN = 22;
        const MAC_GEORGIAN = 23;
        const MAC_ARMENIAN = 24;
        const MAC_CHINESE_SIMP = 25;
        const MAC_TIBETAN = 26;
        const MAC_MONGOLIAN = 27;
        const MAC_ETHIOPIC = 28;
        const MAC_CENTRAL_EUR_ROMAN = 29;
        const MAC_VIETNAMESE = 30;
        const MAC_EXT_ARABIC = 31;
        /* The following use script code 0, smRoman */
        const MAC_SYMBOL = 33;
        const MAC_DINGBATS = 34;
        const MAC_TURKISH = 35;
        const MAC_CROATIAN = 36;
        const MAC_ICELANDIC = 37;
        const MAC_ROMANIAN = 38;
        const MAC_CELTIC = 39;
        const MAC_GAELIC = 40;
        /* The following use script code 4, smArabic */
        const MAC_FARSI = 0x8C;	/* Like MacArabic but uses Farsi digits */
        /* The following use script code 7, smCyrillic */
        const MAC_UKRAINIAN = 0x98;
        /* The following use script code 32, smUnimplemented */
        const MAC_INUIT = 0xEC;
        const MAC_VT100 = 0xFC;	/* VT100/102 font from Comm Toolbox: Latin-1 repertoire + box drawing etc */
        /* Special Mac OS encodings*/
        const MAC_HFS = 0xFF;	/* Meta-value, should never appear in a table */

        /* Unicode & ISO UCS encodings begin at 0x100 */

        /* ISO 8-bit and 7-bit encodings begin at 0x200 */
        const ISO_LATIN2 = 0x0202;	/* ISO 8859-2 */
        const ISO_LATIN3 = 0x0203;	/* ISO 8859-3 */
        const ISO_LATIN4 = 0x0204;	/* ISO 8859-4 */
        const ISO_LATIN_CYRILLIC = 0x0205;	/* ISO 8859-5 */
        const ISO_LATIN_ARABIC = 0x0206;	/* ISO 8859-6, =ASMO 708, =DOS CP 708 */
        const ISO_LATIN_GREEK = 0x0207;	/* ISO 8859-7 */
        const ISO_LATIN_HEBREW = 0x0208;	/* ISO 8859-8 */
        const ISO_LATIN5 = 0x0209;	/* ISO 8859-9 */
        const ISO_LATIN6 = 0x020A;	/* ISO 8859-10 */
        const ISO_LATIN_THAI = 0x020B;	/* ISO 8859-11 */
        const ISO_LATIN7 = 0x020D;	/* ISO 8859-13 */
        const ISO_LATIN8 = 0x020E;	/* ISO 8859-14 */
        const ISO_LATIN9 = 0x020F;	/* ISO 8859-15 */
        const ISO_LATIN10 = 0x0210;	/* ISO 8859-16 */

        /* MS-DOS & Windows encodings begin at 0x400 */
        const DOS_LATIN_US = 0x0400;	/* code page 437 */
        const DOS_GREEK = 0x0405;		/* code page 737 (formerly code page 437G) */
        const DOS_BALTIC_RIM = 0x0406;	/* code page 775 */
        const DOS_LATIN1 = 0x0410;	/* code page 850, "Multilingual" */
        const DOS_GREEK1 = 0x0411;	/* code page 851 */
        const DOS_LATIN2 = 0x0412;	/* code page 852, Slavic */
        const DOS_CYRILLIC = 0x0413;	/* code page 855, IBM Cyrillic */
        const DOS_TURKISH = 0x0414;	/* code page 857, IBM Turkish */
        const DOS_PORTUGUESE = 0x0415;	/* code page 860 */
        const DOS_ICELANDIC = 0x0416;	/* code page 861 */
        const DOS_HEBREW = 0x0417;	/* code page 862 */
        const DOS_CANADIAN_FRENCH = 0x0418; /* code page 863 */
        const DOS_ARABIC = 0x0419;	/* code page 864 */
        const DOS_NORDIC = 0x041A;	/* code page 865 */
        const DOS_RUSSIAN = 0x041B;	/* code page 866 */
        const DOS_GREEK2 = 0x041C;	/* code page 869, IBM Modern Greek */
        const DOS_THAI = 0x041D;		/* code page 874, also for Windows */
        const DOS_JAPANESE = 0x0420;	/* code page 932, also for Windows */
        const DOS_CHINESE_SIMPLIFF = 0x0421; /* code page 936, also for Windows */
        const DOS_KOREAN = 0x0422;	/* code page 949, also for Windows; Unified Hangul Code */
        const DOS_CHINESE_TRAD = 0x0423;	/* code page 950, also for Windows */
        const WINDOWS_LATIN2 = 0x0501;	/* code page 1250, Central Europe */
        const WINDOWS_CYRILLIC = 0x0502;	/* code page 1251, Slavic Cyrillic */
        const WINDOWS_GREEK = 0x0503;	/* code page 1253 */
        const WINDOWS_LATIN5 = 0x0504;	/* code page 1254, Turkish */
        const WINDOWS_HEBREW = 0x0505;	/* code page 1255 */
        const WINDOWS_ARABICC = 0x0506;	/* code page 1256 */
        const WINDOWS_BALTIC_RIM = 0x0507;	/* code page 1257 */
        const WINDOWS_VIETNAMESE = 0x0508; /* code page 1258 */
        const WINDOWS_KOREAN_JOHAB = 0x0510; /* code page 1361, for Windows NT */

        /* Various national standards begin at 0x600 */
        const ANSEL = 0x0601;	/* ANSEL (ANSI Z39.47) */
        const JIS_X0201_76 = 0x0620;
        const JIS_X0208_83 = 0x0621;
        const JIS_X0208_90 = 0x0622;
        const JIS_X0212_90 = 0x0623;
        const JIS_C6226_78 = 0x0624;
        const SHIFT_JIS_X0213 = 0x0628; /* Shift-JIS format encoding of JIS X0213 planes 1 and 2*/
        const SHIFT_JIS_X0213_MEN_KU_TEN = 0x0629;	/* JIS X0213 in plane-row-column notation */
        const GB_2312_80 = 0x0630;
        const GBK_95 = 0x0631;		/* annex to GB 13000-93; for Windows 95 */
        const GB_18030_2000 = 0x0632;
        const KSC_5601_87 = 0x0640;	/* same as KSC 5601-92 without Johab annex */
        const KSC_5601_92_JOHAB = 0x0641; /* KSC 5601-92 Johab annex */
        const CNS_11643_92_P1 = 0x0651;	/* CNS 11643-1992 plane 1 */
        const CNS_11643_92_P2 = 0x0652;	/* CNS 11643-1992 plane 2 */
        const CNS_11643_92_P3 = 0x0653;	/* CNS 11643-1992 plane 3 (was plane 14 in 1986 version) */

        /* ISO 2022 collections begin at 0x800 */
        const ISO_2022_JP = 0x0820;
        const ISO_2022_JP_2 = 0x0821;
        const ISO_2022_JP_1 = 0x0822; /* RFC 2237*/
        const ISO_2022_JP_3 = 0x0823; /* JIS X0213*/
        const ISO_2022_CN = 0x0830;
        const ISO_2022_CN_EXT = 0x0831;
        const ISO_2022_KR = 0x0840;

        /* EUC collections begin at 0x900 */
        const EUC_JP = 0x0920;		/* ISO 646, 1-byte katakana, JIS 208, JIS 212 */
        const EUC_CN = 0x0930;		/* ISO 646, GB 2312-80 */
        const EUC_TW = 0x0931;		/* ISO 646, CNS 11643-1992 Planes 1-16 */
        const EUC_KR = 0x0940;		/* ISO 646, KS C 5601-1987 */

        /* Misc standards begin at 0xA00 */
        const SHIFT_JIS = 0x0A01;		/* plain Shift-JIS */
        const KOI8_R = 0x0A02;		/* Russian internet standard */
        const BIG5 = 0x0A03;		/* Big-5 (has variants) */
        const MAC_ROMAN_LATIN1 = 0x0A04;	/* Mac OS Roman permuted to align with ISO Latin-1 */
        const HZ_GB_2312 = 0x0A05;	/* HZ (RFC 1842, for Chinese mail & news) */
        const BIG5_HKSCS_1999 = 0x0A06; /* Big-5 with Hong Kong special char set supplement*/
        const VISCII = 0x0A07;	/* RFC 1456, Vietnamese */
        const KOI8_U = 0x0A08;	/* RFC 2319, Ukrainian */
        const BIG5_E = 0x0A09;	/* Taiwan Big-5E standard */

        /* Other platform encodings*/
        const NEXT_STEP_JAPANESE = 0x0B02;	/* NextStep Japanese encoding */

        /* EBCDIC & IBM host encodings begin at 0xC00 */
        const EBCDIC_US = 0x0C01;	/* basic EBCDIC-US */
        const EBCDIC_CP037 = 0x0C02;	/* code page 037, extended EBCDIC (Latin-1 set) for US,Canada... */

        const UTF7 = 0x04000100; /* kTextEncodingUnicodeDefault + kUnicodeUTF7Format RFC2152 */
        const UTF7_IMAP = 0x0A10; /* UTF-7 (IMAP folder variant) RFC3501 */

        /* Deprecated constants */
        #[deprecated]
        const SHIFT_JIS_X0213_00 = 0x0628; /* Shift-JIS format encoding of JIS X0213 planes 1 and 2 (DEPRECATED) */
    }
}

bitflags! {
    /// A [`CFOptionFlags`] type for specifying options for string comparison .
    pub struct CFStringCompareFlags: CFOptionFlags {
        const DEFAULT = 0;
        /// Specifies that the comparison should ignore differences in case between alphabetical characters.
        const CASE_INSENSITIVE = 1;
        /// Specifies that the comparison should start at the last elements of the entities being compared (for example, strings or arrays).
        const BACKWARDS = 4;
        /// Performs searching only on characters at the beginning or end of the range.
        const ANCHORED = 8;
        /// Specifies that loose equivalence is acceptable, especially as pertains to diacritical marks.
        const NONLITERAL = 16;
        /// Specifies that the comparison should take into account differences related to locale, such as the thousands separator character.
        const LOCALIZED = 32;
        /// Specifies that represented numeric values should be used as the basis for comparison and not the actual character values.
        const NUMERICALLY = 64;
        /// Specifies that the comparison should ignore diacritic markers.
        const DIACRITIC_INSENSITIVE  = 128;
        /// Specifies that the comparison should ignore width differences.
        const WIDTH_INSENSITIVE  = 256;
        /// Specifies that the comparison is forced to return either [`super::CFComparisonResult::KCFCompareLessThan`] or [`super::CFComparisonResult::KCFCompareGreaterThan`] if the strings are equivalent but not strictly equal.
        const FORCED_ORDERING  = 512;
    }
}

declare_CFType! {
    /// A reference to a CFString object.
    #[repr(C)]
    CFString, CFStringRef
}

/// Defines the buffer and related fields used for in-line buffer access of characters in CFString objects.
#[derive(Debug)]
#[repr(C)]
pub struct CFStringInlineBuffer {
    pub buffer: [UniChar; KCF_STRING_INLINE_BUFFER_LENGTH],
    pub string: CFStringRef,
    pub direct_unichar_buffer: *const UniChar,
    pub direct_c_string_buffer: *const c_char,
    /// Range in string to buffer
    pub range_to_buffer: CFRange,
    /// Start of range currently buffered (relative to rangeToBuffer.location)
    pub buffered_range_start: CFIndex,
    /// bufferedRangeStart + number of chars actually buffered
    pub buffered_range_end: CFIndex,
}

impl CFString {
    /// Creates a new [`CFStringRef`]
    pub fn with_str(s: &str) -> Self {
        unsafe {
            let cstr = CStr::from_ptr(s.as_ptr() as *const i8);

            Self(CFStringCreateWithBytes(
                kCFAllocatorDefault,
                cstr.as_ptr() as *const u8,
                s.len() as CFIndex,
                0,
                false,
            ))
        }
    }

    /// Creates an iterator.
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            string: self,
            index: 0,
        }
    }

    /* Creating a CFString
     */

    /// Creates an array of CFString objects from a single CFString object.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn create_array_by_separating_strings(
        alloc: CFAllocatorRef,
        string: CFStringRef,
        separator: CFStringRef,
    ) -> CFArray {
        CFArray::create_with_ref(CFStringCreateArrayBySeparatingStrings(
            alloc, string, separator,
        ))
    }

    /// Creates a single string from the individual CFString objects that comprise the elements of an array.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn create_by_combining_strings(
        alloc: CFAllocatorRef,
        array: CFArrayRef,
        separator: CFStringRef,
    ) -> CFString {
        CFString::create_with_ref(CFStringCreateByCombiningStrings(alloc, array, separator))
    }

    /// Creates an immutable copy of a string.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn create_copy(alloc: CFAllocatorRef, string: CFStringRef) -> CFString {
        CFString::create_with_ref(CFStringCreateCopy(alloc, string))
    }

    /// Creates a string from its “external representation.”
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn create_from_external_representation(
        alloc: CFAllocatorRef,
        data: CFDataRef,
        encoding: CFStringEncoding,
    ) -> CFString {
        CFString::create_with_ref(CFStringCreateFromExternalRepresentation(
            alloc, data, encoding,
        ))
    }

    /// Creates a string from a buffer containing characters in a specified encoding.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn create_with_bytes(
        alloc: CFAllocatorRef,
        bytes: *const UInt8,
        num_bytes: CFIndex,
        encoding: CFStringEncoding,
        is_external_representation: bool,
    ) -> Self {
        Self::create_with_ref(CFStringCreateWithBytes(
            alloc,
            bytes,
            num_bytes,
            encoding,
            is_external_representation,
        ))
    }

    /// Creates a string from a buffer, containing characters in a specified encoding, that might serve as the backing store for the new string.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn create_with_bytes_no_copy(
        alloc: CFAllocatorRef,
        bytes: *const UInt8,
        num_bytes: CFIndex,
        encoding: CFStringEncoding,
        is_external_representation: Boolean,
        contents_deallocator: CFAllocatorRef,
    ) -> CFString {
        CFString::create_with_ref(CFStringCreateWithBytesNoCopy(
            alloc,
            bytes,
            num_bytes,
            encoding,
            is_external_representation,
            contents_deallocator,
        ))
    }

    /// Creates a string from a buffer of Unicode characters.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn create_with_characters(
        alloc: CFAllocatorRef,
        chars: *const UniChar,
        num_chars: CFIndex,
    ) -> CFString {
        CFString::create_with_ref(CFStringCreateWithCharacters(alloc, chars, num_chars))
    }

    /// Creates a string from a buffer of Unicode characters that might serve as the backing store for the object.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn create_with_characters_no_copy(
        alloc: CFAllocatorRef,
        chars: *const UniChar,
        num_chars: CFIndex,
        contents_deallocator: CFAllocatorRef,
    ) -> CFString {
        CFString::create_with_ref(CFStringCreateWithCharactersNoCopy(
            alloc,
            chars,
            num_chars,
            contents_deallocator,
        ))
    }

    /// Creates an immutable string from a C string.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn create_with_c_string(
        alloc: CFAllocatorRef,
        c_str: *const c_char,
        encoding: CFStringEncoding,
    ) -> CFString {
        CFString::create_with_ref(CFStringCreateWithCString(alloc, c_str, encoding))
    }

    /// Creates a CFString object from an external C string buffer that might serve as the backing store for the object.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn create_with_c_string_no_copy(
        alloc: CFAllocatorRef,
        c_str: *const c_char,
        encoding: CFStringEncoding,
        contents_deallocator: CFAllocatorRef,
    ) -> CFString {
        CFString::create_with_ref(CFStringCreateWithCStringNoCopy(
            alloc,
            c_str,
            encoding,
            contents_deallocator,
        ))
    }

    /// Creates an immutable string from a formatted string and a variable number of arguments (specified in a parameter of type va_list).
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn create_with_format_and_arguments(
        alloc: CFAllocatorRef,
        format_options: CFDictionaryRef,
        format: CFStringRef,
        va_list: VaList,
    ) -> CFString {
        CFString::create_with_ref(CFStringCreateWithFormatAndArguments(
            alloc,
            format_options,
            format,
            va_list,
        ))
    }

    /// Creates an immutable CFString object from a Pascal string.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn create_with_pascal_string(
        alloc: CFAllocatorRef,
        p_str: ConstStr255Param,
        encoding: CFStringEncoding,
    ) -> CFString {
        CFString::create_with_ref(CFStringCreateWithPascalString(alloc, p_str, encoding))
    }

    /// Creates a CFString object from an external Pascal string buffer that might serve as the backing store for the object.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn create_with_pascal_string_no_copy(
        alloc: CFAllocatorRef,
        p_str: ConstStr255Param,
        encoding: CFStringEncoding,
        contents_deallocator: CFAllocatorRef,
    ) -> CFString {
        CFString::create_with_ref(CFStringCreateWithPascalStringNoCopy(
            alloc,
            p_str,
            encoding,
            contents_deallocator,
        ))
    }

    /// Creates an immutable string from a segment (substring) of an existing string.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn create_with_substring(
        alloc: CFAllocatorRef,
        s: CFStringRef,
        range: CFRange,
    ) -> CFString {
        CFString::create_with_ref(CFStringCreateWithSubstring(alloc, s, range))
    }

    /* Searching Strings
     */

    /// Searches a string for multiple occurrences of a substring and creates an array of ranges identifying the locations of these substrings within the target string.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn create_array_with_find_results(
        alloc: CFAllocatorRef,
        string: CFStringRef,
        string_to_find: CFStringRef,
        range_to_search: CFRange,
        compare_options: CFStringCompareFlags,
    ) -> CFArray {
        CFArray::create_with_ref(CFStringCreateArrayWithFindResults(
            alloc,
            string,
            string_to_find,
            range_to_search,
            compare_options,
        ))
    }

    /// Searches for a substring within a string and, if it is found, yields the range of the substring within the object's characters.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn find(
        string: CFStringRef,
        string_to_find: CFStringRef,
        compare_options: CFStringCompareFlags,
    ) -> CFRange {
        CFStringFind(string, string_to_find, compare_options)
    }

    /// Query the range of the first character contained in the specified character set.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn find_character_from_set(
        string: CFStringRef,
        set: CFCharacterSetRef,
        range_to_search: CFRange,
        search_options: CFStringCompareFlags,
        result: *mut CFRange,
    ) -> bool {
        CFStringFindCharacterFromSet(string, set, range_to_search, search_options, result)
    }

    /// Searches for a substring within a range of the characters represented by a string and, if the substring is found, returns its range within the object's characters.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn find_with_options(
        string: CFStringRef,
        string_to_find: CFStringRef,
        range_to_search: CFRange,
        search_options: CFStringCompareFlags,
        result: *mut CFRange,
    ) -> bool {
        CFStringFindWithOptions(
            string,
            string_to_find,
            range_to_search,
            search_options,
            result,
        )
    }

    /// Returns a Boolean value that indicates whether a given string was found in a given source string.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn find_with_options_and_locale(
        string: CFStringRef,
        string_to_find: CFStringRef,
        range_to_search: CFRange,
        search_options: CFStringCompareFlags,
        locale: CFLocaleRef,
        result: *mut CFRange,
    ) -> bool {
        CFStringFindWithOptionsAndLocale(
            string,
            string_to_find,
            range_to_search,
            search_options,
            locale,
            result,
        )
    }

    /// Given a range of characters in a string, obtains the line bounds—that is, the indexes of the first character and the final characters of the lines containing the range.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_line_bounds(
        string: CFStringRef,
        range: CFRange,
        line_begin_index: *mut CFIndex,
        line_end_index: *mut CFIndex,
        contents_end_index: *mut CFIndex,
    ) {
        CFStringGetLineBounds(
            string,
            range,
            line_begin_index,
            line_end_index,
            contents_end_index,
        )
    }

    /* Comparing Strings
     */

    /// Compares one string with another string.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn compare(
        string1: CFStringRef,
        string2: CFStringRef,
        compare_options: CFStringCompareFlags,
    ) -> CFComparisonResult {
        CFStringCompare(string1, string2, compare_options)
    }

    /// Compares a range of the characters in one string with that of another string.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn compare_with_options(
        string1: CFStringRef,
        string2: CFStringRef,
        range_to_compare: CFRange,
        compare_options: CFStringCompareFlags,
    ) -> CFComparisonResult {
        CFStringCompareWithOptions(string1, string2, range_to_compare, compare_options)
    }

    /// Compares a range of the characters in one string with another string using a given locale.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn compare_with_options_and_locale(
        string1: CFStringRef,
        string2: CFStringRef,
        range_to_compare: CFRange,
        compare_options: CFStringCompareFlags,
        locale: CFLocaleRef,
    ) -> CFComparisonResult {
        CFStringCompareWithOptionsAndLocale(
            string1,
            string2,
            range_to_compare,
            compare_options,
            locale,
        )
    }

    /// Determines if the character data of a string begin with a specified sequence of characters.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn has_prefix(string: CFStringRef, prefix: CFStringRef) -> bool {
        CFStringHasPrefix(string, prefix)
    }

    ///Determines if a string ends with a specified sequence of characters.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn has_suffix(string: CFStringRef, suffix: CFStringRef) -> bool {
        CFStringHasSuffix(string, suffix)
    }

    /* Accessing Characters
     */

    /// Creates an “external representation” of a CFString object, that is, a CFData object.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn create_external_representation(
        alloc: CFStringRef,
        string: CFStringEncoding,
        encoding: CFStringEncoding,
        loss_byte: UInt8,
    ) -> CFData {
        CFData::create_with_ref(CFStringCreateExternalRepresentation(
            alloc, string, encoding, loss_byte,
        ))
    }

    /// Fetches a range of the characters from a string into a byte buffer after converting the characters to a specified encoding.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn get_bytes(
        string: CFStringRef,
        range: CFRange,
        encoding: CFStringEncoding,
        loss_byte: UInt8,
        is_external_representation: Boolean,
        buffer: *mut UInt8,
        max_buf_len: CFIndex,
        used_buf_len: *mut CFIndex,
    ) -> CFIndex {
        CFStringGetBytes(
            string,
            range,
            encoding,
            loss_byte,
            is_external_representation,
            buffer,
            max_buf_len,
            used_buf_len,
        )
    }

    /// Returns the Unicode character at a specified location in a string.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_character_at_index(string: CFStringRef, idx: CFIndex) -> UniChar {
        CFStringGetCharacterAtIndex(string, idx)
    }

    /// Copies a range of the Unicode characters from a string to a user-provided buffer.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_characters(string: CFStringRef, range: CFRange, buffer: &mut [UniChar]) {
        CFStringGetCharacters(string, range, buffer)
    }

    /// Quickly obtains a pointer to the contents of a string as a buffer of Unicode characters.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_characters_ptr(string: CFStringRef) -> *const UniChar {
        CFStringGetCharactersPtr(string)
    }

    /// Returns the Unicode character at a specific location in an in-line buffer.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_character_from_inline_buffer(
        buf: *mut CFStringInlineBuffer,
        idx: CFIndex,
    ) -> UniChar {
        CFStringGetCharacterFromInlineBuffer(buf, idx)
    }

    /// Copies the character contents of a string to a local C string buffer after converting the characters to a given encoding.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_c_string(
        string: CFStringRef,
        buffer: &mut [c_char],
        buffer_size: CFIndex,
        encoding: CFStringEncoding,
    ) -> bool {
        CFStringGetCString(string, buffer, buffer_size, encoding)
    }

    /// Quickly obtains a pointer to a C-string buffer containing the characters of a string in a given encoding.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_c_string_ptr(
        string: CFStringRef,
        encoding: CFStringEncoding,
    ) -> *const c_char {
        CFStringGetCStringPtr(string, encoding)
    }

    /// Returns the number (in terms of UTF-16 code pairs) of Unicode characters in a string.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_length(string: CFStringRef) -> CFIndex {
        CFStringGetLength(string)
    }

    /// Copies the character contents of a [`CFString`] object to a local Pascal string buffer after converting the characters to a requested encoding.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_pascal_string(
        string: CFStringRef,
        buffer: StringPtr,
        buffer_size: CFIndex,
        encoding: CFStringEncoding,
    ) -> bool {
        CFStringGetPascalString(string, buffer, buffer_size, encoding)
    }

    /// Quickly obtains a pointer to a Pascal buffer containing the characters of a string in a given encoding.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_pascal_string_ptr(
        string: CFStringRef,
        encoding: CFStringEncoding,
    ) -> ConstStr255Param {
        CFStringGetPascalStringPtr(string, encoding)
    }

    /// Returns the range of the composed character sequence at a specified index.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_range_of_composed_characters_at_index(
        string: CFStringRef,
        index: CFIndex,
    ) -> CFRange {
        CFStringGetRangeOfComposedCharactersAtIndex(string, index)
    }

    /// Initializes an in-line buffer to use for efficient access of a [`CFString`] object's characters.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn init_inline_buffer(
        s: CFStringRef,
        buf: *mut CFStringInlineBuffer,
        range: CFRange,
    ) {
        CFStringInitInlineBuffer(s, buf, range)
    }

    /* Working With Hyphenation
     */

    /// Retrieve the first potential hyphenation location found before the specified location.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_hyphenation_location_before_index(
        string: CFStringRef,
        location: CFIndex,
        limit_range: CFRange,
        options: CFOptionFlags,
        locale: CFLocaleRef,
        character: *mut UTF32Char,
    ) -> CFIndex {
        CFStringGetHyphenationLocationBeforeIndex(
            string,
            location,
            limit_range,
            options,
            locale,
            character,
        )
    }

    /// Returns a [`Boolean`] value that indicates whether hyphenation data is available.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn is_hyphenation_available_for_locale(locale: CFLocaleRef) -> Boolean {
        CFStringIsHyphenationAvailableForLocale(locale)
    }

    /* Working With Encodings
     */

    /// Returns the name of the IANA registry “charset” that is the closest mapping to a specified string encoding.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn convert_encoding_to_ianachar_set_name(encoding: CFStringEncoding) -> CFString {
        CFString::create_with_ref(CFStringConvertEncodingToIANACharSetName(encoding))
    }

    /// Returns the Cocoa encoding constant that maps most closely to a given Core Foundation encoding constant.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn convert_encoding_to_nsstring_encoding(encoding: CFStringEncoding) -> c_ulong {
        CFStringConvertEncodingToNSStringEncoding(encoding)
    }

    /// Returns the Windows codepage identifier that maps most closely to a given Core Foundation encoding constant.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn convert_encoding_to_windows_codepage(encoding: CFStringEncoding) -> UInt32 {
        CFStringConvertEncodingToWindowsCodepage(encoding)
    }

    /// Returns the Core Foundation encoding constant that is the closest mapping to a given IANA registry “charset” name.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn convert_iana_char_set_name_to_encodingng(
        string: CFStringRef,
    ) -> CFStringEncoding {
        CFStringConvertIANACharSetNameToEncoding(string)
    }

    /// Returns the Core Foundation encoding constant that is the closest mapping to a given Cocoa encoding.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn convert_nsstring_encoding_to_encodingng(encoding: c_ulong) -> CFStringEncoding {
        CFStringConvertNSStringEncodingToEncoding(encoding)
    }

    /// Returns the Core Foundation encoding constant that is the closest mapping to a given Windows codepage identifier.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn convert_windows_codepage_to_encoding(codepage: UInt32) -> CFStringEncoding {
        CFStringConvertWindowsCodepageToEncoding(codepage)
    }

    /// Returns for a [`CFString`] object the character encoding that requires the least conversion time.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_fastest_encoding(string: CFStringRef) -> CFStringEncoding {
        CFStringGetFastestEncoding(string)
    }

    /// Returns a pointer to a list of string encodings supported by the current system.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_list_of_available_encodings() -> *const CFStringEncoding {
        CFStringGetListOfAvailableEncodings()
    }

    /// Returns the maximum number of bytes a string of a specified length (in Unicode characters) will take up if encoded in a specified encoding.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_maximum_size_for_encoding(
        length: CFIndex,
        encoding: CFStringEncoding,
    ) -> CFIndex {
        CFStringGetMaximumSizeForEncoding(length, encoding)
    }

    /// Returns the most compatible Mac OS script value for the given input encoding.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_most_compatible_mac_string_encoding(
        encoding: CFStringEncoding,
    ) -> CFStringEncoding {
        CFStringGetMostCompatibleMacStringEncoding(encoding)
    }

    /// Returns the canonical name of a specified string encoding.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_name_of_encoding(encoding: CFStringEncoding) -> CFString {
        CFString::create_with_ref(CFStringGetNameOfEncoding(encoding))
    }

    /// Returns the smallest encoding on the current system for the character contents of a string.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_smallest_encoding(string: CFStringRef) -> CFStringEncoding {
        CFStringGetSmallestEncoding(string)
    }

    /// Returns the default encoding used by the operating system when it creates strings.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_system_encoding() -> CFStringEncoding {
        CFStringGetSystemEncoding()
    }

    /// Determines whether a given Core Foundation string encoding is available on the current system.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn is_encoding_available(encoding: CFStringEncoding) -> bool {
        CFStringIsEncodingAvailable(encoding)
    }

    /* Getting Numeric Values
     */

    /// Returns the primary [`c_double`] value represented by a string.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_double_value(s: CFStringRef) -> c_double {
        CFStringGetDoubleValue(s)
    }

    /// Returns the [`SInt32`] value represented by a string.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_int_value(s: CFStringRef) -> SInt32 {
        CFStringGetIntValue(s)
    }

    /* Getting String Properties
     */

    /// Prints the attributes of a string during debugging.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn cf_show_str(s: CFStringRef) {
        CFShowStr(s)
    }

    /// Returns the type identifier for the [`CFString`] opaque type.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_type_id() -> CFTypeID {
        CFStringGetTypeID()
    }

    /* String File System Representations
     */

    /// Creates a [`CFString`] from a zero-terminated POSIX file system representation.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn create_with_file_system_representation(
        alloc: CFAllocatorRef,
        buffer: *const c_char,
    ) -> CFString {
        CFString::create_with_ref(CFStringCreateWithFileSystemRepresentation(alloc, buffer))
    }

    /// Extracts the contents of a string as a NULL-terminated 8-bit string appropriate for passing to POSIX APIs.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_file_system_representation(
        string: CFStringRef,
        buffer: &mut [c_char],
        max_buf_len: CFIndex,
    ) -> Boolean {
        CFStringGetFileSystemRepresentation(string, buffer, max_buf_len)
    }

    /// Determines the upper bound on the number of bytes required to hold the file system representation of the string.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_maximum_size_of_file_system_representation(string: CFStringRef) -> CFIndex {
        CFStringGetMaximumSizeOfFileSystemRepresentation(string)
    }

    /* Getting Paragraph Bounds
     */

    /// Determines the upper bound on the number of bytes required to hold the file system representation of the string.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_paragraph_bounds(
        string: CFStringRef,
        range: CFRange,
        par_begin_index: *mut CFIndex,
        par_end_index: *mut CFIndex,
        contents_end_index: *mut CFIndex,
    ) {
        CFStringGetParagraphBounds(
            string,
            range,
            par_begin_index,
            par_end_index,
            contents_end_index,
        )
    }

    /* Managing Surrogates
     */

    /// Returns a UTF-32 character that corresponds to a given pair of UTF-16 surrogate characters.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_long_character_for_surrogate_pair(
        surrogate_high: UniChar,
        surrogate_low: UniChar,
    ) -> UTF32Char {
        CFStringGetLongCharacterForSurrogatePair(surrogate_high, surrogate_low)
    }

    /// Maps a given UTF-32 character to a pair of UTF-16 surrogate characters.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_surrogate_pair_for_long_character(
        character: UTF32Char,
        surrogates: &mut [UniChar],
    ) -> Boolean {
        CFStringGetSurrogatePairForLongCharacter(character, surrogates)
    }

    /// Returns a [`Boolean`] value that indicates whether a given character is a high character in a surrogate pair.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn is_surrogate_high_character(character: UniChar) -> Boolean {
        CFStringIsSurrogateHighCharacter(character)
    }

    /// Returns a [`Boolean`] value that indicates whether a given character is a low character in a surrogate pair.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn is_surrogate_low_character(character: UniChar) -> Boolean {
        CFStringIsSurrogateLowCharacter(character)
    }
}

impl<'a> From<&'a CFString> for Cow<'a, str> {
    fn from(cf_str: &'a CFString) -> Cow<'a, str> {
        unsafe {
            // Do this without allocating if we can get away with it
            let c_string =
                CFStringGetCStringPtr(cf_str.get_internal_object(), KCFStringEncoding::UTF8.bits);
            match c_string.is_null() {
                true => {
                    let char_len = CFString::get_length(cf_str.get_internal_object());

                    // First, ask how big the buffer ought to be.
                    let mut bytes_required: CFIndex = 0;
                    let _ = CFStringGetBytes(
                        cf_str.get_internal_object(),
                        CFRange {
                            location: 0,
                            length: char_len,
                        },
                        KCFStringEncoding::UTF8.bits,
                        0,
                        false,
                        std::ptr::null_mut(),
                        0,
                        &mut bytes_required,
                    );

                    // Then, allocate the buffer and actually copy.
                    let mut buffer = vec![b'\x00'; bytes_required as usize];

                    let mut bytes_used: CFIndex = 0;
                    let chars_written = CFStringGetBytes(
                        cf_str.get_internal_object(),
                        CFRange {
                            location: 0,
                            length: char_len,
                        },
                        KCFStringEncoding::UTF8.bits,
                        0,
                        false,
                        buffer.as_mut_ptr(),
                        buffer.len() as CFIndex,
                        &mut bytes_used,
                    );
                    assert_eq!(chars_written, char_len);

                    // This is dangerous; we over-allocate and null-terminate the string (during
                    // initialization).
                    assert_eq!(bytes_used, buffer.len() as CFIndex);
                    Cow::Owned(String::from_utf8_unchecked(buffer))
                }
                _ => {
                    let c_str = CStr::from_ptr(c_string);
                    String::from_utf8_lossy(c_str.to_bytes())
                }
            }
        }
    }
}

impl<'a> IntoIterator for &'a CFString {
    type Item = UniChar;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl fmt::Display for CFString {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&Cow::from(self))
    }
}

impl fmt::Debug for CFString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl PartialEq for CFString {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            matches!(
                CFStringCompare(
                    self.get_internal_object(),
                    other.get_internal_object(),
                    CFStringCompareFlags::DEFAULT,
                ),
                CFComparisonResult::KCFCompareEqualTo
            )
        }
    }
}

impl Eq for CFString {}

impl PartialEq<&str> for CFString {
    fn eq(&self, other: &&str) -> bool {
        &self.to_string() == other
    }
}

extern "C" {
    pub fn CFStringCreateArrayBySeparatingStrings(
        alloc: CFAllocatorRef,
        string: CFStringRef,
        separator: CFStringRef,
    ) -> CFArrayRef;

    pub fn CFStringCreateByCombiningStrings(
        alloc: CFAllocatorRef,
        array: CFArrayRef,
        separator: CFStringRef,
    ) -> CFStringRef;

    pub fn CFStringCreateCopy(alloc: CFAllocatorRef, string: CFStringRef) -> CFStringRef;

    pub fn CFStringCreateFromExternalRepresentation(
        alloc: CFAllocatorRef,
        data: CFDataRef,
        encoding: CFStringEncoding,
    ) -> CFStringRef;

    pub fn CFStringCreateWithBytes(
        alloc: CFAllocatorRef,
        bytes: *const UInt8,
        num_bytes: CFIndex,
        encoding: CFStringEncoding,
        is_external_representation: Boolean,
    ) -> CFStringRef;

    pub fn CFStringCreateWithBytesNoCopy(
        alloc: CFAllocatorRef,
        bytes: *const UInt8,
        num_bytes: CFIndex,
        encoding: CFStringEncoding,
        is_external_representation: Boolean,
        contents_deallocator: CFAllocatorRef,
    ) -> CFStringRef;

    pub fn CFStringCreateWithCharacters(
        alloc: CFAllocatorRef,
        chars: *const UniChar,
        num_chars: CFIndex,
    ) -> CFStringRef;

    pub fn CFStringCreateWithCharactersNoCopy(
        alloc: CFAllocatorRef,
        chars: *const UniChar,
        num_chars: CFIndex,
        contents_deallocator: CFAllocatorRef,
    ) -> CFStringRef;

    pub fn CFStringCreateWithCString(
        alloc: CFAllocatorRef,
        c_str: *const c_char,
        encoding: CFStringEncoding,
    ) -> CFStringRef;

    pub fn CFStringCreateWithCStringNoCopy(
        alloc: CFAllocatorRef,
        c_str: *const c_char,
        encoding: CFStringEncoding,
        contents_deallocator: CFAllocatorRef,
    ) -> CFStringRef;

    pub fn CFStringCreateWithFormat(
        alloc: CFAllocatorRef,
        format_options: CFDictionaryRef,
        format: CFStringRef,
        ...
    ) -> CFStringRef;

    pub fn CFStringCreateWithFormatAndArguments(
        alloc: CFAllocatorRef,
        format_options: CFDictionaryRef,
        format: CFStringRef,
        va_list: VaList,
    ) -> CFStringRef;

    pub fn CFStringCreateWithPascalString(
        alloc: CFAllocatorRef,
        p_str: ConstStr255Param,
        encoding: CFStringEncoding,
    ) -> CFStringRef;

    pub fn CFStringCreateWithPascalStringNoCopy(
        alloc: CFAllocatorRef,
        p_str: ConstStr255Param,
        encoding: CFStringEncoding,
        contents_deallocator: CFAllocatorRef,
    ) -> CFStringRef;

    pub fn CFStringCreateWithSubstring(
        alloc: CFAllocatorRef,
        s: CFStringRef,
        range: CFRange,
    ) -> CFStringRef;

    pub fn CFStringCreateArrayWithFindResults(
        alloc: CFAllocatorRef,
        string: CFStringRef,
        string_to_find: CFStringRef,
        range_to_search: CFRange,
        compare_options: CFStringCompareFlags,
    ) -> CFArrayRef;

    pub fn CFStringFind(
        string: CFStringRef,
        string_to_find: CFStringRef,
        compare_options: CFStringCompareFlags,
    ) -> CFRange;

    pub fn CFStringFindCharacterFromSet(
        string: CFStringRef,
        set: CFCharacterSetRef,
        range_to_search: CFRange,
        search_options: CFStringCompareFlags,
        result: *mut CFRange,
    ) -> bool;

    pub fn CFStringFindWithOptions(
        string: CFStringRef,
        string_to_find: CFStringRef,
        range_to_search: CFRange,
        search_options: CFStringCompareFlags,
        result: *mut CFRange,
    ) -> bool;

    pub fn CFStringFindWithOptionsAndLocale(
        string: CFStringRef,
        string_to_find: CFStringRef,
        range_to_search: CFRange,
        search_options: CFStringCompareFlags,
        locale: CFLocaleRef,
        result: *mut CFRange,
    ) -> bool;

    pub fn CFStringGetLineBounds(
        string: CFStringRef,
        range: CFRange,
        line_begin_index: *mut CFIndex,
        line_end_index: *mut CFIndex,
        contents_end_index: *mut CFIndex,
    );

    pub fn CFStringCompare(
        string1: CFStringRef,
        string2: CFStringRef,
        compare_options: CFStringCompareFlags,
    ) -> CFComparisonResult;

    pub fn CFStringCompareWithOptions(
        string1: CFStringRef,
        string2: CFStringRef,
        range_to_compare: CFRange,
        compare_options: CFStringCompareFlags,
    ) -> CFComparisonResult;

    pub fn CFStringCompareWithOptionsAndLocale(
        string1: CFStringRef,
        string2: CFStringRef,
        range_to_compare: CFRange,
        compare_options: CFStringCompareFlags,
        locale: CFLocaleRef,
    ) -> CFComparisonResult;

    pub fn CFStringHasPrefix(string: CFStringRef, prefix: CFStringRef) -> Boolean;

    pub fn CFStringHasSuffix(string: CFStringRef, suffix: CFStringRef) -> Boolean;

    pub fn CFStringCreateExternalRepresentation(
        alloc: CFStringRef,
        string: CFStringEncoding,
        encoding: CFStringEncoding,
        loss_byte: UInt8,
    ) -> CFDataRef;

    pub fn CFStringGetBytes(
        string: CFStringRef,
        range: CFRange,
        encoding: CFStringEncoding,
        loss_byte: UInt8,
        is_external_representation: Boolean,
        buffer: *mut UInt8,
        max_buf_len: CFIndex,
        used_buf_len: *mut CFIndex,
    ) -> CFIndex;

    pub fn CFStringGetCharacterAtIndex(string: CFStringRef, idx: CFIndex) -> UniChar;

    pub fn CFStringGetCharacters(string: CFStringRef, range: CFRange, buffer: &mut [UniChar]);

    pub fn CFStringGetCharactersPtr(string: CFStringRef) -> *const UniChar;

    pub fn CFStringGetCharacterFromInlineBuffer(
        buf: *mut CFStringInlineBuffer,
        idx: CFIndex,
    ) -> UniChar;

    pub fn CFStringGetCString(
        string: CFStringRef,
        buffer: &mut [c_char],
        buffer_size: CFIndex,
        encoding: CFStringEncoding,
    ) -> Boolean;

    pub fn CFStringGetCStringPtr(string: CFStringRef, encoding: CFStringEncoding) -> *const c_char;

    pub fn CFStringGetLength(string: CFStringRef) -> CFIndex;

    pub fn CFStringGetPascalString(
        string: CFStringRef,
        buffer: StringPtr,
        buffer_size: CFIndex,
        encoding: CFStringEncoding,
    ) -> Boolean;

    pub fn CFStringGetPascalStringPtr(
        string: CFStringRef,
        encoding: CFStringEncoding,
    ) -> ConstStr255Param;

    pub fn CFStringGetRangeOfComposedCharactersAtIndex(
        string: CFStringRef,
        index: CFIndex,
    ) -> CFRange;

    pub fn CFStringInitInlineBuffer(s: CFStringRef, buf: *mut CFStringInlineBuffer, range: CFRange);

    pub fn CFStringGetHyphenationLocationBeforeIndex(
        string: CFStringRef,
        location: CFIndex,
        limit_range: CFRange,
        options: CFOptionFlags,
        locale: CFLocaleRef,
        character: *mut UTF32Char,
    ) -> CFIndex;

    pub fn CFStringIsHyphenationAvailableForLocale(locale: CFLocaleRef) -> Boolean;

    pub fn CFStringConvertEncodingToIANACharSetName(encoding: CFStringEncoding) -> CFStringRef;

    pub fn CFStringConvertEncodingToNSStringEncoding(encoding: CFStringEncoding) -> c_ulong;

    pub fn CFStringConvertEncodingToWindowsCodepage(encoding: CFStringEncoding) -> UInt32;

    pub fn CFStringConvertIANACharSetNameToEncoding(string: CFStringRef) -> CFStringEncoding;

    pub fn CFStringConvertNSStringEncodingToEncoding(encoding: c_ulong) -> CFStringEncoding;

    pub fn CFStringConvertWindowsCodepageToEncoding(codepage: UInt32) -> CFStringEncoding;

    pub fn CFStringGetFastestEncoding(string: CFStringRef) -> CFStringEncoding;

    pub fn CFStringGetListOfAvailableEncodings() -> *const CFStringEncoding;

    pub fn CFStringGetMaximumSizeForEncoding(
        length: CFIndex,
        encoding: CFStringEncoding,
    ) -> CFIndex;

    pub fn CFStringGetMostCompatibleMacStringEncoding(
        encoding: CFStringEncoding,
    ) -> CFStringEncoding;

    pub fn CFStringGetNameOfEncoding(encoding: CFStringEncoding) -> CFStringRef;

    pub fn CFStringGetSmallestEncoding(string: CFStringRef) -> CFStringEncoding;

    pub fn CFStringGetSystemEncoding() -> CFStringEncoding;

    pub fn CFStringIsEncodingAvailable(encoding: CFStringEncoding) -> Boolean;

    pub fn CFStringGetDoubleValue(s: CFStringRef) -> c_double;

    pub fn CFStringGetIntValue(s: CFStringRef) -> SInt32;

    pub fn CFShowStr(s: CFStringRef);

    pub fn CFStringGetTypeID() -> CFTypeID;

    pub fn CFStringCreateWithFileSystemRepresentation(
        alloc: CFAllocatorRef,
        buffer: *const c_char,
    ) -> CFStringRef;

    pub fn CFStringGetFileSystemRepresentation(
        string: CFStringRef,
        buffer: &mut [c_char],
        max_buf_len: CFIndex,
    ) -> Boolean;

    pub fn CFStringGetMaximumSizeOfFileSystemRepresentation(string: CFStringRef) -> CFIndex;

    pub fn CFStringGetParagraphBounds(
        string: CFStringRef,
        range: CFRange,
        par_begin_index: *mut CFIndex,
        par_end_index: *mut CFIndex,
        contents_end_index: *mut CFIndex,
    );

    pub fn CFStringGetLongCharacterForSurrogatePair(
        surrogate_high: UniChar,
        surrogate_low: UniChar,
    ) -> UTF32Char;

    pub fn CFStringGetSurrogatePairForLongCharacter(
        character: UTF32Char,
        surrogates: &mut [UniChar],
    ) -> Boolean;

    pub fn CFStringIsSurrogateHighCharacter(character: UniChar) -> Boolean;

    pub fn CFStringIsSurrogateLowCharacter(character: UniChar) -> Boolean;
}

#[cfg(test)]
mod tests {

    use crate::core_foundation::{kCFAllocatorDefault, CFRange, CFTypeObject};

    use super::{CFString, KCFStringEncoding};

    #[test]
    fn test_to_string() {
        let original = "The quick brown fox jumped over the slow lazy dog.";
        let cfstr = CFString::with_str(original);
        let converted = cfstr.to_string();
        assert_eq!(converted, original);
    }

    #[test]
    fn test_index() {
        let s = "Hello World!";
        let native_str = CFString::with_str(s);

        for i in 0..s.len() {
            assert_eq!(
                s.chars().nth(i).unwrap(),
                native_str.iter().nth(i).unwrap() as u8 as char
            );
        }
    }

    #[test]
    fn test_cmp_cf_string() {
        let native_str1 = CFString::with_str("Hello World!");
        let native_str2 = CFString::with_str("Hello World!");
        let native_str3 = CFString::with_str("Goodbye World!");

        assert_eq!(native_str1, native_str2);
        assert_ne!(native_str1, native_str3);
    }

    #[test]
    fn test_cmp_string() {
        let s = "Hello World!";
        let native_str = CFString::with_str(s);

        assert_eq!(native_str, s);
    }

    #[test]
    fn test_create_with_bytes() {
        unsafe {
            let original = "The quick brown fox jumped over the slow lazy dog.";
            let cfstr = CFString::create_with_bytes(
                kCFAllocatorDefault,
                original.as_ptr(),
                original.bytes().len() as u64,
                KCFStringEncoding::UTF8.bits,
                false,
            );

            assert_eq!(cfstr, original);
        }
    }

    #[test]
    fn test_get_characters() {
        let original = "The quick brown fox jumped over the slow lazy dog.";
        let cfstr = CFString::with_str(original);

        unsafe {
            let mut buffer = vec![0; original.len()];

            CFString::get_characters(
                cfstr.get_internal_object(),
                CFRange {
                    location: 0,
                    length: CFString::get_length(cfstr.get_internal_object()),
                },
                &mut buffer,
            );

            assert_eq!(original, String::from_utf16(&buffer).unwrap())
        }
    }

    #[test]
    fn test_get_length() {
        let original = "The quick brown fox jumped over the slow lazy dog.";
        let cfstr = CFString::with_str(original);

        unsafe {
            assert_eq!(
                original.len(),
                CFString::get_length(cfstr.get_internal_object()) as usize
            )
        }
    }
}
