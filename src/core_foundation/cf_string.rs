use std::ffi::CString;

use libc::c_char;

use bitflags::bitflags;

use crate::{core_foundation::CFRange, kernel::UniChar};

use super::{kCFAllocatorDefault, macros::declare_CFType, CFAllocatorRef, CFDataRef, CFIndex};

/// A reference to a CFString object.
pub type CFStringRef = *const CFString;

/// An integer type for constants used to specify supported string encodings in various CFString functions.
pub type CFStringEncoding = u32;

const KCF_STRING_INLINE_BUFFER_LENGTH: usize = 64;

bitflags! {
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

declare_CFType! {
    /// A reference to a CFString object.
    #[repr(C)]
    CFString
}

/// The next two functions allow fast access to the contents of a string,
/// assuming you are doing sequential or localized accesses. To use, call
/// CFStringInitInlineBuffer() with a CFStringInlineBuffer (on the stack, say),
/// and a range in the string to look at. Then call CFStringGetCharacterFromInlineBuffer()
/// as many times as you want, with a index into that range (relative to the start
/// of that range). These are INLINE functions and will end up calling CFString only
/// once in a while, to fill a buffer.  CFStringGetCharacterFromInlineBuffer() returns 0 if
/// a location outside the original range is specified.
#[derive(Debug)]
#[repr(C)]
pub struct CFStringInlineBuffer {
    ///
    pub buffer: [UniChar; KCF_STRING_INLINE_BUFFER_LENGTH],
    ///
    pub string: CFStringRef,
    ///
    pub direct_unichar_buffer: *const UniChar,
    ///
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
    pub fn with_str(s: &str) -> CFStringRef {
        unsafe {
            let cstr = CString::new(s).unwrap();
            CFStringCreateWithBytes(
                kCFAllocatorDefault,
                cstr.as_ptr() as *const u8,
                s.len() as CFIndex,
                0,
                false,
            )
        }
    }

    /// Creates a string from a buffer containing characters in a specified encoding.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn create_with_bytes(
        alloc: CFAllocatorRef,
        bytes: *const u8,
        num_bytes: CFIndex,
        encoding: CFStringEncoding,
        is_external_representation: bool,
    ) -> CFStringRef {
        CFStringCreateWithBytes(
            alloc,
            bytes,
            num_bytes,
            encoding,
            is_external_representation,
        )
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
        loss_byte: u8,
    ) -> CFDataRef {
        CFStringCreateExternalRepresentation(alloc, string, encoding, loss_byte)
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
        loss_byte: u8,
        is_external_representation: bool,
        buffer: *mut u8,
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
    pub unsafe fn get_characters(string: CFStringRef, range: CFRange, buffer: *mut UniChar) {
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
        buffer: *mut c_char,
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
}

extern "C" {
    fn CFStringCreateWithBytes(
        alloc: CFAllocatorRef,
        bytes: *const u8,
        num_bytes: CFIndex,
        encoding: CFStringEncoding,
        is_external_representation: bool,
    ) -> CFStringRef;

    fn CFStringCreateExternalRepresentation(
        alloc: CFStringRef,
        string: CFStringEncoding,
        encoding: CFStringEncoding,
        loss_byte: u8,
    ) -> CFDataRef;

    fn CFStringGetBytes(
        string: CFStringRef,
        range: CFRange,
        encoding: CFStringEncoding,
        loss_byte: u8,
        is_external_representation: bool,
        buffer: *mut u8,
        max_buf_len: CFIndex,
        used_buf_len: *mut CFIndex,
    ) -> CFIndex;

    fn CFStringGetCharacterAtIndex(string: CFStringRef, idx: CFIndex) -> UniChar;

    fn CFStringGetCharacters(string: CFStringRef, range: CFRange, buffer: *mut UniChar);

    fn CFStringGetCharactersPtr(string: CFStringRef) -> *const UniChar;

    fn CFStringGetCharacterFromInlineBuffer(
        buf: *mut CFStringInlineBuffer,
        idx: CFIndex,
    ) -> UniChar;

    fn CFStringGetCString(
        string: CFStringRef,
        buffer: *mut c_char,
        buffer_size: CFIndex,
        encoding: CFStringEncoding,
    ) -> bool;

    fn CFStringGetCStringPtr(string: CFStringRef, encoding: CFStringEncoding) -> *const c_char;

    fn CFStringGetLength(string: CFStringRef) -> CFIndex;
}
