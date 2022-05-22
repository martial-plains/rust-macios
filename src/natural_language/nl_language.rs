//! The languages that the Natural Language framework supports.

use super::NLLanguage;

extern "C" {
    /* Languages
     */

    /// The unique identifier string for the Arabic language.
    #[link_name = "NLLanguageArabic"]
    pub static Arabic: NLLanguage;

    /// The unique identifier string for the Armenian language.
    #[link_name = "NLLanguageArmenian"]
    pub static Armenian: NLLanguage;

    /// The unique identifier string for the Bengali language.
    #[link_name = "NLLanguageBengali"]
    pub static Bengali: NLLanguage;

    /// The unique identifier string for the Bulgarian language.
    #[link_name = "NLLanguageBulgarian"]
    pub static Bulgarian: NLLanguage;

    /// The unique identifier string for the Burmese language.
    #[link_name = "NLLanguageBurmese"]
    pub static Burmese: NLLanguage;

    /// The unique identifier string for the Catalan language.
    #[link_name = "NLLanguageCatalan"]
    pub static Catalan: NLLanguage;

    /// The unique identifier string for the Cherokee language.
    #[link_name = "NLLanguageCherokee"]
    pub static Cherokee: NLLanguage;

    /// The unique identifier string for the Croatian language.
    #[link_name = "NLLanguageCroatian"]
    pub static Croatian: NLLanguage;

    /// The unique identifier string for the Czech language.
    #[link_name = "NLLanguageCzech"]
    pub static Czech: NLLanguage;

    /// The unique identifier string for the Danish language.
    #[link_name = "NLLanguageDanish"]
    pub static Danish: NLLanguage;

    /// The unique identifier string for the Dutch language.
    #[link_name = "NLLanguageDutch"]
    pub static Dutch: NLLanguage;

    /// The unique identifier string for the English language.
    #[link_name = "NLLanguageEnglish"]
    pub static English: NLLanguage;

    /// The unique identifier string for the Finnish language.
    #[link_name = "NLLanguageFinnish"]
    pub static Finnish: NLLanguage;

    /// The unique identifier string for the French language.
    #[link_name = "NLLanguageFrench"]
    pub static French: NLLanguage;

    /// The unique identifier string for the Georgian language.
    #[link_name = "NLLanguageGeorgian"]
    pub static Georgian: NLLanguage;

    /// The unique identifier string for the German language.
    #[link_name = "NLLanguageGerman"]
    pub static German: NLLanguage;

    /// The unique identifier string for the Greek language.
    #[link_name = "NLLanguageGreek"]
    pub static Greek: NLLanguage;

    /// The unique identifier string for the Gujarati language.
    #[link_name = "NLLanguageGujarati"]
    pub static Gujarati: NLLanguage;

    /// The unique identifier string for the Hebrew language.
    #[link_name = "NLLanguageHebrew"]
    pub static Hebrew: NLLanguage;

    /// The unique identifier string for the Hindi language.
    #[link_name = "NLLanguageHindi"]
    pub static Hindi: NLLanguage;

    /// The unique identifier string for the Hungarian language.
    #[link_name = "NLLanguageHungarian"]
    pub static Hungarian: NLLanguage;

    /// The unique identifier string for the Icelandic language.
    #[link_name = "NLLanguageIcelandic"]
    pub static Icelandic: NLLanguage;

    /// The unique identifier string for the Indonesian language.
    #[link_name = "NLLanguageIndonesian"]
    pub static Indonesian: NLLanguage;

    /// The unique identifier string for the Italian language.
    #[link_name = "NLLanguageItalian"]
    pub static Italian: NLLanguage;

    /// The unique identifier string for the Japanese language.
    #[link_name = "NLLanguageJapanese"]
    pub static Japanese: NLLanguage;

    /// The unique identifier string for the Kannada language.
    #[link_name = "NLLanguageKannada"]
    pub static Kannada: NLLanguage;

    /// The unique identifier string for the Khmer language.
    #[link_name = "NLLanguageKhmer"]
    pub static Khmer: NLLanguage;

    /// The unique identifier string for the Korean language.
    #[link_name = "NLLanguageKorean"]
    pub static Korean: NLLanguage;

    /// The unique identifier string for the Lao language.
    #[link_name = "NLLanguageLao"]
    pub static Lao: NLLanguage;

    /// The unique identifier string for the Malay language.
    #[link_name = "NLLanguageMalay"]
    pub static Malay: NLLanguage;

    /// The unique identifier string for the Malayalam language.
    #[link_name = "NLLanguageMalayalam"]
    pub static Malayalam: NLLanguage;

    /// The unique identifier string for the Marathi language.
    #[link_name = "NLLanguageMarathi"]
    pub static Marathi: NLLanguage;

    /// The unique identifier string for the Mongolian language.
    #[link_name = "NLLanguageMongolian"]
    pub static Mongolian: NLLanguage;

    /// The unique identifier string for the Norwegian language.
    #[link_name = "NLLanguageNorwegian"]
    pub static Norwegian: NLLanguage;

    /// The unique identifier string for the Oriya language.
    #[link_name = "NLLanguageOriya"]
    pub static Oriya: NLLanguage;

    /// The unique identifier string for the Persian language.
    #[link_name = "NLLanguagePersian"]
    pub static Persian: NLLanguage;

    /// The unique identifier string for the Polish language.
    #[link_name = "NLLanguagePolish"]
    pub static Polish: NLLanguage;

    /// The unique identifier string for the Portuguese language.
    #[link_name = "NLLanguagePortuguese"]
    pub static Portuguese: NLLanguage;

    /// The unique identifier string for the Punjabi language.
    #[link_name = "NLLanguagePunjabi"]
    pub static Punjabi: NLLanguage;

    /// The unique identifier string for the Romanian language.
    #[link_name = "NLLanguageRomanian"]
    pub static Romanian: NLLanguage;

    /// The unique identifier string for the Russian language.
    #[link_name = "NLLanguageRussian"]
    pub static Russian: NLLanguage;

    /// The unique identifier string for the Simplified Chinese language.
    #[link_name = "NLLanguageSimplifiedChinese"]
    pub static SimplifiedChinese: NLLanguage;

    /// The unique identifier string for the Sinhalese language.
    #[link_name = "NLLanguageSinhalese"]
    pub static Sinhalese: NLLanguage;

    /// The unique identifier string for the Slovak language.
    #[link_name = "NLLanguageSlovak"]
    pub static Slovak: NLLanguage;

    /// The unique identifier string for the Spanish language.
    #[link_name = "NLLanguageSpanish"]
    pub static Spanish: NLLanguage;

    /// The unique identifier string for the Swedish language.
    #[link_name = "NLLanguageSwedish"]
    pub static Swedish: NLLanguage;

    /// The unique identifier string for the Tamil language.
    #[link_name = "NLLanguageTamil"]
    pub static Tamil: NLLanguage;

    /// The unique identifier string for the Telugu language.
    #[link_name = "NLLanguageTelugu"]
    pub static Telugu: NLLanguage;

    /// The unique identifier string for the Thai language.
    #[link_name = "NLLanguageThai"]
    pub static Thai: NLLanguage;

    /// The unique identifier string for the Tibetan language.
    #[link_name = "NLLanguageTibetan"]
    pub static Tibetan: NLLanguage;

    /// The unique identifier string for the Traditional Chinese language.
    #[link_name = "NLLanguageTraditionalChinese"]
    pub static TraditionalChinese: NLLanguage;

    /// The unique identifier string for the Turkish language.
    #[link_name = "NLLanguageTurkish"]
    pub static Turkish: NLLanguage;

    /// The unique identifier string for the Ukrainian language.
    #[link_name = "NLLanguageUkrainian"]
    pub static Ukrainian: NLLanguage;

    /// The unique identifier string for the Urdu language.
    #[link_name = "NLLanguageUrdu"]
    pub static Urdu: NLLanguage;

    /// The unique identifier string for the Vietnamese language.
    #[link_name = "NLLanguageVietnamese"]
    pub static Vietnamese: NLLanguage;

    /// The unique identifier string for a language the Natural Language framework doesn’t recognize.
    #[link_name = "NLLanguageUndetermined"]
    pub static Undetermined: NLLanguage;
}
