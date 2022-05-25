use super::NSStringTransform;

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    /* Transliteration
     */

    /// A constant containing the transliteration of a string from any script to Latin script.
    #[link_name = "NSStringTransformToLatin"]
    pub static ToLatin: NSStringTransform;

    /// LatinToArabic
    #[link_name = "NSStringTransformLatinToArabic"]
    pub static LatinToArabic: NSStringTransform;

    /// A constant containing the transliteration of a string from Latin script to Cyrillic script.
    #[link_name = "NSStringTransformLatinToCyrillic"]
    pub static LatinToCyrillic: NSStringTransform;

    /// A constant containing the transliteration of a string from Latin script to Greek script.
    #[link_name = "NSStringTransformLatinToGreek"]
    pub static LatinToGreek: NSStringTransform;

    /// A constant containing the transliteration of a string from Latin script to Hangul script.
    #[link_name = "NSStringTransformLatinToHangul"]
    pub static LatinToHangul: NSStringTransform;

    /// A constant containing the transliteration of a string from Latin script to Hebrew script.
    #[link_name = "NSStringTransformLatinToHebrew"]
    pub static LatinToHebrew: NSStringTransform;

    /// A constant containing the transliteration of a string from Latin script to Hiragana script.
    #[link_name = "NSStringTransformLatinToHiragana"]
    pub static LatinToHiragana: NSStringTransform;

    /// A constant containing the transliteration of a string from Latin script to Katakana script.
    #[link_name = "NSStringTransformLatinToKatakana"]
    pub static LatinToKatakana: NSStringTransform;

    /// A constant containing the transliteration of a string from Latin script to Thai script.
    #[link_name = "NSStringTransformLatinToThai"]
    pub static LatinToThai: NSStringTransform;

    /// A constant containing the transliteration of a string from Hiragana script to Katakana script.
    #[link_name = "NSStringTransformHiraganaToKatakana"]
    pub static HiraganaToKatakana: NSStringTransform;

    /// A constant containing the transliteration of a string from Han script to Latin.
    #[link_name = "NSStringTransformHanziToLatin"]
    pub static MandarinToLatin: NSStringTransform;

    /* Diacritic and Combining Mark Removal
     */

    /// A constant containing the transformation of a string by removing diacritics.
    #[link_name = "NSStringTransformStripDiacritics"]
    pub static StripDiacritics: NSStringTransform;

    /// A constant containing the transformation of a string by removing combining marks.
    #[link_name = "NSStringTransformStripCombiningMarks"]
    pub static StripCombiningMarks: NSStringTransform;

    /* Halfwidth and Fullwidth Form Conversion
     */

    /// A constant containing the transformation of a string from full-width CJK characters to half-width forms.
    #[link_name = "NSStringTransformFullwidthToHalfwidth"]
    pub static FullwidthToHalfwidth: NSStringTransform;

    /* Character Representation
     */

    /// An identifier for a transform that converts characters to Unicode names.
    #[link_name = "NSStringTransformToUnicodeName"]
    pub static ToUnicodeName: NSStringTransform;

    /// A constant containing the transformation of a string from characters to XML hexadecimal escape codes.
    #[link_name = "NSStringTransformToXMLHex"]
    pub static ToXMLHex: NSStringTransform;

}
