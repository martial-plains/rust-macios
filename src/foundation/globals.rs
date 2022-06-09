use super::NSEdgeInsets;

/// Size of UTF8 encoding
pub const UTF8_ENCODING: usize = 4;

/// The keys used to access components of a locale.
pub mod ns_localekey {
    use crate::foundation::NSLocaleKey;

    extern "C" {
        /// The locale identifier.
        #[link_name = "NSLocaleIdentifier"]
        pub static Identifier: NSLocaleKey;

        /// The locale country or region code.
        #[link_name = "NSLocaleCountryCode"]
        pub static CountryCode: NSLocaleKey;

        /// The locale language code.
        #[link_name = "NSLocaleLanguageCode"]
        pub static LanguageCode: NSLocaleKey;

        /// The locale script code.
        #[link_name = "NSLocaleScriptCode"]
        pub static ScriptCode: NSLocaleKey;

        /// The locale variant code.
        #[link_name = "NSLocaleVariantCode"]
        pub static VariantCode: NSLocaleKey;

        /// The exemplar character set for the locale.
        #[link_name = "NSLocaleExemplarCharacterSet"]
        pub static ExemplarCharacterSet: NSLocaleKey;

        /// The calendar associated with the locale.
        #[link_name = "NSLocaleCalendar"]
        pub static Calendar: NSLocaleKey;

        /// The collation associated with the locale.
        #[link_name = "NSLocaleCollationIdentifier"]
        pub static CollationIdentifier: NSLocaleKey;

        /// The collation identifier for the locale.
        #[link_name = "NSLocaleCollatorIdentifier"]
        pub static CollatorIdentifier: NSLocaleKey;

        /// A flag that indicates whether the locale uses the metric system.
        #[link_name = "NSLocaleUsesMetricSystem"]
        pub static UsesMetricSystem: NSLocaleKey;

        /// The measurement system associated with the locale.
        #[link_name = "NSLocaleMeasurementSystem"]
        pub static MeasurementSystem: NSLocaleKey;

        /// The decimal separator associated with the locale.
        #[link_name = "NSLocaleDecimalSeparator"]
        pub static DecimalSeparator: NSLocaleKey;

        /// The numeric grouping separator associated with the locale.
        #[link_name = "NSLocaleGroupingSeparator"]
        pub static GroupingSeparator: NSLocaleKey;

        /// The currency symbol associated with the locale.
        #[link_name = "NSLocaleCurrencySymbol"]
        pub static CurrencySymbol: NSLocaleKey;

        /// The currency code associated with the locale.
        #[link_name = "NSLocaleCurrencyCode"]
        pub static CurrencyCode: NSLocaleKey;

        /// The end quotation symbol associated with the locale.
        #[link_name = "NSLocaleQuotationBeginSymbol"]
        pub static QuotationBeginSymbol: NSLocaleKey;

        /// The begin quotation symbol associated with the locale.
        #[link_name = "NSLocaleQuotationEndSymbol"]
        pub static QuotationEndSymbol: NSLocaleKey;

        /// The alternate end quotation symbol associated with the locale.
        #[link_name = "NSLocaleAlternateQuotationBeginSymbol"]
        pub static AlternateQuotationBeginSymbol: NSLocaleKey;

        /// The alternating begin quotation symbol associated with the locale.
        #[link_name = "NSLocaleAlternateQuotationEndSymbol"]
        pub static AlternateQuotationEndSymbol: NSLocaleKey;
    }
}

extern "C" {
    /// A zero initialized `NSEdgeInsets`.
    pub static NSEdgeInsetsZero: NSEdgeInsets;
}
