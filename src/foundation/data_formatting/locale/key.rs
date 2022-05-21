//! The keys used to access components of a locale.

use crate::foundation::String;

/// The keys used to access components of a locale.
pub type NSLocaleKey<'a> = *mut String<'a>;

#[allow(improper_ctypes)]
extern "C" {
    /// The locale identifier.
    #[link_name = "NSLocaleIdentifier"]
    pub static Identifier: NSLocaleKey<'static>;

    /// The locale country or region code.
    #[link_name = "NSLocaleCountryCode"]
    pub static CountryCode: NSLocaleKey<'static>;

    /// The locale language code.
    #[link_name = "NSLocaleLanguageCode"]
    pub static LanguageCode: NSLocaleKey<'static>;

    /// The locale script code.
    #[link_name = "NSLocaleScriptCode"]
    pub static ScriptCode: NSLocaleKey<'static>;

    /// The locale variant code.
    #[link_name = "NSLocaleVariantCode"]
    pub static VariantCode: NSLocaleKey<'static>;

    /// The exemplar character set for the locale.
    #[link_name = "NSLocaleExemplarCharacterSet"]
    pub static ExemplarCharacterSet: NSLocaleKey<'static>;

    /// The calendar associated with the locale.
    #[link_name = "NSLocaleCalendar"]
    pub static Calendar: NSLocaleKey<'static>;

    /// The collation associated with the locale.
    #[link_name = "NSLocaleCollationIdentifier"]
    pub static CollationIdentifier: NSLocaleKey<'static>;

    /// The collation identifier for the locale.
    #[link_name = "NSLocaleCollatorIdentifier"]
    pub static CollatorIdentifier: NSLocaleKey<'static>;

    /// A flag that indicates whether the locale uses the metric system.
    #[link_name = "NSLocaleUsesMetricSystem"]
    pub static UsesMetricSystem: NSLocaleKey<'static>;

    /// The measurement system associated with the locale.
    #[link_name = "NSLocaleMeasurementSystem"]
    pub static MeasurementSystem: NSLocaleKey<'static>;

    /// The decimal separator associated with the locale.
    #[link_name = "NSLocaleDecimalSeparator"]
    pub static DecimalSeparator: NSLocaleKey<'static>;

    /// The numeric grouping separator associated with the locale.
    #[link_name = "NSLocaleGroupingSeparator"]
    pub static GroupingSeparator: NSLocaleKey<'static>;

    /// The currency symbol associated with the locale.
    #[link_name = "NSLocaleCurrencySymbol"]
    pub static CurrencySymbol: NSLocaleKey<'static>;

    /// The currency code associated with the locale.
    #[link_name = "NSLocaleCurrencyCode"]
    pub static CurrencyCode: NSLocaleKey<'static>;

    /// The end quotation symbol associated with the locale.
    #[link_name = "NSLocaleQuotationBeginSymbol"]
    pub static QuotationBeginSymbol: NSLocaleKey<'static>;

    /// The begin quotation symbol associated with the locale.
    #[link_name = "NSLocaleQuotationEndSymbol"]
    pub static QuotationEndSymbol: NSLocaleKey<'static>;

    /// The alternate end quotation symbol associated with the locale.
    #[link_name = "NSLocaleAlternateQuotationBeginSymbol"]
    pub static AlternateQuotationBeginSymbol: NSLocaleKey<'static>;

    /// The alternating begin quotation symbol associated with the locale.
    #[link_name = "NSLocaleAlternateQuotationEndSymbol"]
    pub static AlternateQuotationEndSymbol: NSLocaleKey<'static>;
}
