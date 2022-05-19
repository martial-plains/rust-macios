//! The languages that the Natural Language framework supports.

use super::NLLanguage;

#[allow(improper_ctypes)]
extern "C" {
    /* Languages
     */

    /// The unique identifier string for the Arabic language.
    #[link_name = "NLLanguageArabic"]
    pub static Arabic: NLLanguage<'static>;

    /// The unique identifier string for the Armenian language.
    #[link_name = "NLLanguageArmenian"]
    pub static Armenian: NLLanguage<'static>;

    /// The unique identifier string for the Bengali language.
    #[link_name = "NLLanguageBengali"]
    pub static Bengali: NLLanguage<'static>;

    /// The unique identifier string for the Bulgarian language.
    #[link_name = "NLLanguageBulgarian"]
    pub static Bulgarian: NLLanguage<'static>;

    /// The unique identifier string for the Burmese language.
    #[link_name = "NLLanguageBurmese"]
    pub static Burmese: NLLanguage<'static>;

    /// The unique identifier string for the Catalan language.
    #[link_name = "NLLanguageCatalan"]
    pub static Catalan: NLLanguage<'static>;

    /// The unique identifier string for the Cherokee language.
    #[link_name = "NLLanguageCherokee"]
    pub static Cherokee: NLLanguage<'static>;

    /// The unique identifier string for the Croatian language.
    #[link_name = "NLLanguageCroatian"]
    pub static Croatian: NLLanguage<'static>;

    /// The unique identifier string for the Czech language.
    #[link_name = "NLLanguageCzech"]
    pub static Czech: NLLanguage<'static>;

    /// The unique identifier string for the Danish language.
    #[link_name = "NLLanguageDanish"]
    pub static Danish: NLLanguage<'static>;

    /// The unique identifier string for the Dutch language.
    #[link_name = "NLLanguageDutch"]
    pub static Dutch: NLLanguage<'static>;

    /// The unique identifier string for the English language.
    #[link_name = "NLLanguageEnglish"]
    pub static English: NLLanguage<'static>;

    /// The unique identifier string for the Finnish language.
    #[link_name = "NLLanguageFinnish"]
    pub static Finnish: NLLanguage<'static>;

    /// The unique identifier string for the French language.
    #[link_name = "NLLanguageFrench"]
    pub static French: NLLanguage<'static>;

    /// The unique identifier string for the Georgian language.
    #[link_name = "NLLanguageGeorgian"]
    pub static Georgian: NLLanguage<'static>;

    /// The unique identifier string for the German language.
    #[link_name = "NLLanguageGerman"]
    pub static German: NLLanguage<'static>;

    /// The unique identifier string for the Greek language.
    #[link_name = "NLLanguageGreek"]
    pub static Greek: NLLanguage<'static>;

    /// The unique identifier string for the Gujarati language.
    #[link_name = "NLLanguageGujarati"]
    pub static Gujarati: NLLanguage<'static>;

    /// The unique identifier string for the Hebrew language.
    #[link_name = "NLLanguageHebrew"]
    pub static Hebrew: NLLanguage<'static>;

    /// The unique identifier string for the Hindi language.
    #[link_name = "NLLanguageHindi"]
    pub static Hindi: NLLanguage<'static>;

    /// The unique identifier string for the Hungarian language.
    #[link_name = "NLLanguageHungarian"]
    pub static Hungarian: NLLanguage<'static>;

    /// The unique identifier string for the Icelandic language.
    #[link_name = "NLLanguageIcelandic"]
    pub static Icelandic: NLLanguage<'static>;

    /// The unique identifier string for the Indonesian language.
    #[link_name = "NLLanguageIndonesian"]
    pub static Indonesian: NLLanguage<'static>;

    /// The unique identifier string for the Italian language.
    #[link_name = "NLLanguageItalian"]
    pub static Italian: NLLanguage<'static>;

    /// The unique identifier string for the Japanese language.
    #[link_name = "NLLanguageJapanese"]
    pub static Japanese: NLLanguage<'static>;

    /// The unique identifier string for the Kannada language.
    #[link_name = "NLLanguageKannada"]
    pub static Kannada: NLLanguage<'static>;

    /// The unique identifier string for the Khmer language.
    #[link_name = "NLLanguageKhmer"]
    pub static Khmer: NLLanguage<'static>;

    /// The unique identifier string for the Korean language.
    #[link_name = "NLLanguageKorean"]
    pub static Korean: NLLanguage<'static>;

    /// The unique identifier string for the Lao language.
    #[link_name = "NLLanguageLao"]
    pub static Lao: NLLanguage<'static>;

    /// The unique identifier string for the Malay language.
    #[link_name = "NLLanguageMalay"]
    pub static Malay: NLLanguage<'static>;

    /// The unique identifier string for the Malayalam language.
    #[link_name = "NLLanguageMalayalam"]
    pub static Malayalam: NLLanguage<'static>;

    /// The unique identifier string for the Marathi language.
    #[link_name = "NLLanguageMarathi"]
    pub static Marathi: NLLanguage<'static>;

    /// The unique identifier string for the Mongolian language.
    #[link_name = "NLLanguageMongolian"]
    pub static Mongolian: NLLanguage<'static>;

    /// The unique identifier string for the Norwegian language.
    #[link_name = "NLLanguageNorwegian"]
    pub static Norwegian: NLLanguage<'static>;

    /// The unique identifier string for the Oriya language.
    #[link_name = "NLLanguageOriya"]
    pub static Oriya: NLLanguage<'static>;

    /// The unique identifier string for the Persian language.
    #[link_name = "NLLanguagePersian"]
    pub static Persian: NLLanguage<'static>;

    /// The unique identifier string for the Polish language.
    #[link_name = "NLLanguagePolish"]
    pub static Polish: NLLanguage<'static>;

    /// The unique identifier string for the Portuguese language.
    #[link_name = "NLLanguagePortuguese"]
    pub static Portuguese: NLLanguage<'static>;

    /// The unique identifier string for the Punjabi language.
    #[link_name = "NLLanguagePunjabi"]
    pub static Punjabi: NLLanguage<'static>;

    /// The unique identifier string for the Romanian language.
    #[link_name = "NLLanguageRomanian"]
    pub static Romanian: NLLanguage<'static>;

    /// The unique identifier string for the Russian language.
    #[link_name = "NLLanguageRussian"]
    pub static Russian: NLLanguage<'static>;

    /// The unique identifier string for the Simplified Chinese language.
    #[link_name = "NLLanguageSimplifiedChinese"]
    pub static SimplifiedChinese: NLLanguage<'static>;

    /// The unique identifier string for the Sinhalese language.
    #[link_name = "NLLanguageSinhalese"]
    pub static Sinhalese: NLLanguage<'static>;

    /// The unique identifier string for the Slovak language.
    #[link_name = "NLLanguageSlovak"]
    pub static Slovak: NLLanguage<'static>;

    /// The unique identifier string for the Spanish language.
    #[link_name = "NLLanguageSpanish"]
    pub static Spanish: NLLanguage<'static>;

    /// The unique identifier string for the Swedish language.
    #[link_name = "NLLanguageSwedish"]
    pub static Swedish: NLLanguage<'static>;

    /// The unique identifier string for the Tamil language.
    #[link_name = "NLLanguageTamil"]
    pub static Tamil: NLLanguage<'static>;

    /// The unique identifier string for the Telugu language.
    #[link_name = "NLLanguageTelugu"]
    pub static Telugu: NLLanguage<'static>;

    /// The unique identifier string for the Thai language.
    #[link_name = "NLLanguageThai"]
    pub static Thai: NLLanguage<'static>;

    /// The unique identifier string for the Tibetan language.
    #[link_name = "NLLanguageTibetan"]
    pub static Tibetan: NLLanguage<'static>;

    /// The unique identifier string for the Traditional Chinese language.
    #[link_name = "NLLanguageTraditionalChinese"]
    pub static TraditionalChinese: NLLanguage<'static>;

    /// The unique identifier string for the Turkish language.
    #[link_name = "NLLanguageTurkish"]
    pub static Turkish: NLLanguage<'static>;

    /// The unique identifier string for the Ukrainian language.
    #[link_name = "NLLanguageUkrainian"]
    pub static Ukrainian: NLLanguage<'static>;

    /// The unique identifier string for the Urdu language.
    #[link_name = "NLLanguageUrdu"]
    pub static Urdu: NLLanguage<'static>;

    /// The unique identifier string for the Vietnamese language.
    #[link_name = "NLLanguageVietnamese"]
    pub static Vietnamese: NLLanguage<'static>;

    /// The unique identifier string for a language the Natural Language framework doesn’t recognize.
    #[link_name = "NLLanguageUndetermined"]
    pub static Undetermined: NLLanguage<'static>;
}
