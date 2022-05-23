/// The following constants are provided by NSString as possible string encodings.
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
