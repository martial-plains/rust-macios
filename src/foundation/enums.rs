//! Enums for the Foundation library.
use crate::core_graphics::CGRectEdge;

/// Enums for String Encoding
pub mod string {
    /// The following constants are provided by NSString as possible string encodings.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
}

/// These values represent the options available to many of the string classes’ search and comparison methods.
#[derive(Debug, PartialEq, Eq)]
#[repr(u64)]
pub enum NSStringCompareOptions {
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

impl NSStringCompareOptions {
    /// Creates a new `CompareOptions` with the given flags.
    pub fn new(raw_value: usize) -> Self {
        match raw_value {
            1 => NSStringCompareOptions::CaseInsensitive,
            2 => NSStringCompareOptions::Literal,
            4 => NSStringCompareOptions::Backwards,
            8 => NSStringCompareOptions::Anchored,
            64 => NSStringCompareOptions::Numeric,
            128 => NSStringCompareOptions::DiacriticInsensitive,
            256 => NSStringCompareOptions::WidthInsensitive,
            512 => NSStringCompareOptions::ForcedOrdering,
            1024 => NSStringCompareOptions::RegularExpression,
            _ => panic!("Unknown CompareOptions value: {}", raw_value),
        }
    }
}

/// Constants that indicate sort order.
#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub enum NSComparisonResult {
    /// The left operand is smaller than the right operand.
    OrderedAscending = -1,
    /// The two operands are equal.
    OrderedSame = 0,
    /// The left operand is greater than the right operand.
    OrderedDescending = 1,
}

/// The constants used to specify interaction with the cached responses.
#[derive(Debug)]
#[repr(u64)]
pub enum NSUrlRequestCachePolicy {
    /// Use the caching logic defined in the protocol implementation, if any, for a particular URL load request.
    UseProtocolCachePolicy = 0,
    /// The URL load should be loaded only from the originating source.
    ReloadIgnoringLocalCacheData = 1,
    /// Ignore local cache data, and instruct proxies and other intermediates to disregard their caches so far as the protocol allows.
    ReloadIgnoringLocalAndRemoteCacheData = 4, // Unimplemented
    /// Use existing cache data, regardless or age or expiration date, loading from originating source only if there is no cached data.
    ReturnCacheDataElseLoad = 2,
    /// Use existing cache data, regardless or age or expiration date, and fail if no cached data is available.
    ReturnCacheDataDontLoad = 3,
    /// Use cache data if the origin source can validate it; otherwise, load from the origin.
    ReloadRevalidatingCacheData = 5, // Unimplemented
}

impl NSUrlRequestCachePolicy {
    /// Replaced by NSURLRequestReloadIgnoringLocalCacheData.
    #[allow(non_upper_case_globals)]
    pub const ReloadIgnoringCacheData: Self = Self::ReloadIgnoringLocalCacheData;
}

/// The type declared for the constants listed in [Stream Status Constants](https://developer.apple.com/documentation/foundation/nsstream/stream_status_constants).
#[derive(Debug)]
#[repr(u64)]
pub enum NSStreamStatus {
    /// The stream is not open for reading or writing. This status is returned before the
    /// underlying call to open a stream but after it’s been created.
    NotOpen = 0,
    /// The stream is in the process of being opened for reading or for writing. For network streams, this status might include the time after the stream was opened, but while network DNS resolution is happening.
    Opening = 1,
    /// The stream is open, but no reading or writing is occurring.
    Open = 2,
    /// Data is being read from the stream. This status would be returned if code on another thread were to call streamStatus on the stream while a read:maxLength: call (NSInputStream) was in progress.
    Reading = 3,
    /// Data is being written to the stream. This status would be returned if code on another thread were to call streamStatus on the stream while a write:maxLength: call (NSOutputStream) was in progress.
    Writing = 4,
    /// There is no more data to read, or no more data can be written to the stream. When this status is returned, the stream is in a “non-blocking” mode and no data are available.
    AtEnd = 5,
    /// The stream is closed (close has been called on it).
    Closed = 6,
    /// The remote end of the connection can’t be contacted, or the connection has been severed for some other reason.
    Error = 7,
}

/// These constants are used to specify a property list serialization format.
#[derive(Debug)]
#[repr(u64)]
pub enum NSPropertyListFormat {
    /// Specifies the ASCII property list format inherited from the OpenStep APIs.
    OpenStep = 1,
    /// Specifies the XML property list format.
    Xml = 100,
    /// Specifies the binary property list format.``
    Binary = 200,
}

/// These constants specify mutability options in property lists.
#[derive(Debug)]
#[repr(u64)]
pub enum NSPropertyListMutabilityOptions {
    /// Causes the returned property list to contain immutable objects.
    Immutable = 0,
    /// Causes the returned property list to have mutable containers but immutable leaves.
    MutableContainers = 1,
    /// Causes the returned property list to have mutable containers and leaves.
    MutableContainersAndLeaves = 2,
}

/// These constants specify options for a network service.
#[derive(Debug)]
#[repr(u64)]
pub enum NSNetServiceOptions {
    /// Specifies that the network service should not rename itself in the event of a name collision.
    NoAutoRename = 1 << 0,
    /// Specifies that a TCP listener should be started for both IPv4 and IPv6
    /// on the port specified by this service. If the listening port can't be
    /// opened, the service calls its delegate’s netService:didNotPublish:
    /// method to report the error.
    ListenForConnections = 1 << 1,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSDateFormatterStyle {
    /// Specifies no style.
    None,
    /// Specifies a short style, typically numeric only, such as “11/23/37” or “3:30 PM”.
    Short,
    /// Specifies a medium style, typically with abbreviated text, such as “Nov 23, 1937” or “3:30:32 PM”.
    Medium,
    /// Specifies a long style, typically with full text, such as “November 23, 1937” or “3:30:32 PM PST”.
    Long,
    /// Specifies a full style with complete details, such as “Tuesday, April 12, 1952 AD” or “3:30:42 PM Pacific Standard Time”.
    Full,
}

/// Constants that specify the behavior NSDateFormatter should exhibit.
#[derive(Debug)]
#[repr(u64)]
pub enum NSDateFormatterBehavior {
    /// Specifies default formatting behavior.
    Default = 0,
    /// Specifies formatting behavior equivalent to that in OS X 10.0.
    #[cfg(target_os = "macos")]
    #[allow(non_camel_case_types)]
    Mode_10_0 = 1000,
    /// Specifies formatting behavior equivalent for OS X 10.4.
    #[allow(non_camel_case_types)]
    Mode_10_4 = 1040,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSHttpCookieAcceptPolicy {
    Always,
    Never,
    OnlyFromMainDocumentDomain,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSCalendarUnit {
    Era = 2,
    Year = 4,
    Month = 8,
    Day = 16,
    Hour = 32,
    Minute = 64,
    Second = 128,
    #[deprecated]
    Week = 256,
    Weekday = 512,
    WeekdayOrdinal = 1024,
    Quarter = 2048,

    WeekOfMonth = (1 << 12),
    WeekOfYear = (1 << 13),
    YearForWeakOfYear = (1 << 14),

    Nanosecond = (1 << 15),

    Calendar = (1 << 20),
    TimeZone = (1 << 21),
}

#[derive(Debug)]
#[repr(i64)]
pub enum NSOperationQueuePriority {
    VeryLow = -8,
    Low = -4,
    Normal = 0,
    High = 4,
    VeryHigh = 8,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSNotificationCoalescing {
    NoCoalescing = 0,
    CoalescingOnName = 1,
    CoalescingOnSender = 2,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSPostingStyle {
    PostWhenIdle = 1,
    PostASAP = 2,
    Now = 3,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSExpressionType {
    ConstantValue = 0,
    EvaluatedObject,
    Variable,
    KeyPath,
    Function,
    UnionSet,
    IntersectSet,
    MinusSet,
    Subquery = 13,
    NSAggregate,
    AnyKey = 15,
    Block = 19,
    Conditional = 20,
}

#[derive(Debug)]
#[repr(C)]
pub enum NSUrlError {
    Unknown = -1,

    BackgroundSessionRequiresSharedContainer = -995,
    BackgroundSessionInUseByAnotherProcess = -996,
    BackgroundSessionWasDisconnected = -997,

    Cancelled = -999,
    BadURL = -1000,
    TimedOut = -1001,
    UnsupportedURL = -1002,
    CannotFindHost = -1003,
    CannotConnectToHost = -1004,
    NetworkConnectionLost = -1005,
    DNSLookupFailed = -1006,
    HTTPTooManyRedirects = -1007,
    ResourceUnavailable = -1008,
    NotConnectedToInternet = -1009,
    RedirectToNonExistentLocation = -1010,
    BadServerResponse = -1011,
    UserCancelledAuthentication = -1012,
    UserAuthenticationRequired = -1013,
    ZeroByteResource = -1014,
    CannotDecodeRawData = -1015,
    CannotDecodeContentData = -1016,
    CannotParseResponse = -1017,
    InternationalRoamingOff = -1018,
    CallIsActive = -1019,
    DataNotAllowed = -1020,
    RequestBodyStreamExhausted = -1021,
    AppTransportSecurityRequiresSecureConnection = -1022,

    FileDoesNotExist = -1100,
    FileIsDirectory = -1101,
    NoPermissionsToReadFile = -1102,
    DataLengthExceedsMaximum = -1103,
    FileOutsideSafeArea = -1104,

    SecureConnectionFailed = -1200,
    ServerCertificateHasBadDate = -1201,
    ServerCertificateUntrusted = -1202,
    ServerCertificateHasUnknownRoot = -1203,
    ServerCertificateNotYetValid = -1204,
    ClientCertificateRejected = -1205,
    ClientCertificateRequired = -1206,

    CannotLoadFromNetwork = -2000,

    // Download and file I/O errors
    CannotCreateFile = -3000,
    CannotOpenFile = -3001,
    CannotCloseFile = -3002,
    CannotWriteToFile = -3003,
    CannotRemoveFile = -3004,
    CannotMoveFile = -3005,
    DownloadDecodingFailedMidStream = -3006,
    DownloadDecodingFailedToComplete = -3007,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSKeyValueObservingOptions {
    New = 1,
    Old = 2,
    OldNew = 3,
    Initial = 4,
    Prior = 8,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSKeyValueChange {
    Setting = 1,
    Insertion,
    Removal,
    Replacement,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSKeyValueSetMutationKind {
    UnionSet = 1,
    MinusSet,
    IntersectSet,
    SetSet,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSEnumerationOptions {
    SortConcurrent = 1,
    Reverse = 2,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSStreamEvent {
    None = 0,
    OpenCompleted = 1 << 0,
    HasBytesAvailable = 1 << 1,
    HasSpaceAvailable = 1 << 2,
    ErrorOccurred = 1 << 3,
    EndEncountered = 1 << 4,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSComparisonPredicateModifier {
    Direct,
    All,
    Any,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSPredicateOperatorType {
    LessThan,
    LessThanOrEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    EqualTo,
    NotEqualTo,
    Matches,
    Like,
    BeginsWith,
    EndsWith,
    In,
    CustomSelector,
    Contains = 99,
    Between,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSComparisonPredicateOptions {
    None = 0x00,
    CaseInsensitive = 1 << 0,
    DiacriticInsensitive = 1 << 1,
    Normalized = 1 << 2,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSCompoundPredicateType {
    Not,
    And,
    Or,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSVolumeEnumerationOptions {
    None = 0,
    // skip                  = 1 << 0,
    SkipHiddenVolumes = 1 << 1,
    ProduceFileReferenceUrls = 1 << 2,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSDirectoryEnumerationOptions {
    None = 0,
    SkipsSubdirectoryDescendants = 1 << 0,
    SkipsPackageDescendants = 1 << 1,
    SkipsHiddenFiles = 1 << 2,
    IncludesDirectoriesPostOrder = 1 << 3,
    ProducesRelativePathUrls = 1 << 4,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSFileManagerItemReplacementOptions {
    None = 0,
    UsingNewMetadataOnly = 1 << 0,
    WithoutDeletingBackupItem = 1 << 1,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSSearchPathDirectory {
    ApplicationDirectory = 1,
    DemoApplicationDirectory,
    DeveloperApplicationDirectory,
    AdminApplicationDirectory,
    LibraryDirectory,
    DeveloperDirectory,
    UserDirectory,
    DocumentationDirectory,
    DocumentDirectory,
    CoreServiceDirectory,
    AutosavedInformationDirectory = 11,
    DesktopDirectory = 12,
    CachesDirectory = 13,
    ApplicationSupportDirectory = 14,
    DownloadsDirectory = 15,
    InputMethodsDirectory = 16,
    MoviesDirectory = 17,
    MusicDirectory = 18,
    PicturesDirectory = 19,
    PrinterDescriptionDirectory = 20,
    SharedPublicDirectory = 21,
    PreferencePanesDirectory = 22,
    #[cfg(target_os = "macos")]
    ApplicationScriptsDirectory = 23,
    ItemReplacementDirectory = 99,
    AllApplicationsDirectory = 100,
    AllLibrariesDirectory = 101,
    TrashDirectory = 102,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSSearchPathDomain {
    None = 0,
    User = 1 << 0,
    Local = 1 << 1,
    Network = 1 << 2,
    System = 1 << 3,
    All = 0x0ffff,
}

/// These constants specify rounding behaviors.
#[derive(Debug)]
#[repr(u64)]
pub enum NSRoundingMode {
    /// Round to the closest possible return value; when caught halfway between two positive numbers, round up; when caught between two negative numbers, round down.
    Plain,
    /// Round return values down.
    Down,
    /// Round return values up.
    Up,
    /// Round to the closest possible return value; when halfway between two possibilities, return the possibility whose last digit is even.
    Bankers,
}

/// Calculation error constants used to describe an error in exceptionDuringOperation:error:leftOperand:rightOperand:.
#[derive(Debug)]
#[repr(u64)]
pub enum NSCalculationError {
    /// No error occurred.
    None,
    /// The number can’t be represented in 38 significant digits.
    PrecisionLoss,
    /// The number is too small to represent.
    Underflow,
    /// The number is too large to represent.
    Overflow,
    /// The caller tried to divide by 0.
    DivideByZero,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSStringDrawingOptions {
    UsesLineFragmentOrigin = (1 << 0),
    UsesFontLeading = (1 << 1),
    DisableScreenFontSubstitution = (1 << 2),
    UsesDeviceMetrics = (1 << 3),
    OneShot = (1 << 4),
    TruncatesLastVisibleLine = (1 << 5),
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSNumberFormatterStyle {
    /// An integer representation.
    None = 0,
    /// A decimal style format.
    Decimal = 1,
    /// A currency style format that uses the currency symbol defined by the
    /// number formatter locale.
    Currency = 2,
    /// A percent style format.
    Percent = 3,
    /// A scientific style format.
    Scientific = 4,
    /// A style format in which numbers are spelled out in the language
    /// defined by the number formatter locale.
    SpellOut = 5,
    /// An ordinal style format.
    OrdinalStyle = 6,
    /// A currency style format that uses the ISO 4217 currency code defined
    /// by the number formatter locale.
    CurrencyIsoCodeStyle = 8,
    /// A currency style format that uses the pluralized denomination defined
    /// by the number formatter locale.
    CurrencyPluralStyle = 9,
    /// An accounting currency style format that uses the currency symbol
    /// defined by the number formatter locale.
    CurrencyAccountingStyle = 10,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSNumberFormatterBehavior {
    /// The number-formatter behavior set as the default for new instances.
    /// You can set the default formatter behavior with the class method setDefaultFormatterBehavior:.
    Default = 0,
    /// The number-formatter behavior as it existed prior to macOS 10.4.
    #[cfg(target_os = "macos")]
    #[allow(non_camel_case_types)]
    Version_10_0 = 1000,
    /// The number-formatter behavior since macOS 10.4.
    #[allow(non_camel_case_types)]
    Version_10_4 = 1040,
}

/// These constants are used to specify how numbers should be padded. These constants are used by the paddingPosition property.
#[derive(Debug)]
#[repr(u64)]
pub enum NSNumberFormatterPadPosition {
    /// Specifies that the padding should occur before the prefix.
    BeforePrefix,
    /// Specifies that the padding should occur after the prefix.
    AfterPrefix,
    /// Specifies that the padding should occur before the suffix.
    BeforeSuffix,
    /// Specifies that the padding should occur after the suffix.
    AfterSuffix,
}

/// These constants are used to specify how numbers should be rounded. These
/// constants are used by the roundingMode property.

#[derive(Debug)]
#[repr(u64)]
pub enum NSNumberFormatterRoundingMode {
    /// Round towards positive infinity.
    Ceiling,
    /// Round towards negative infinity.
    Floor,
    /// Round towards zero.
    Down,
    /// Round away from zero.
    Up,
    /// Round towards the nearest integer, or towards an even number if equidistant.
    HalfEven,
    /// Round towards the nearest integer, or towards zero if equidistant.
    HalfDown,
    /// Round towards the nearest integer, or away from zero if equidistant.
    HalfUp,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSFileVersionReplacingOptions {
    ByMoving = 1 << 0,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSFileVersionAddingOptions {
    ByMoving = 1 << 0,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSFileCoordinatorReadingOptions {
    WithoutChanges = 1,
    ResolvesSymbolicLink = 1 << 1,
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    ImmediatelyAvailableMetadataOnly = 1 << 2,
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    ForUploading = 1 << 3,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSFileCoordinatorWritingOptions {
    ForDeleting = 1,
    ForMoving = 2,
    ForMerging = 4,
    ForReplacing = 8,
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    ContentIndependentMetadataOnly = 16,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSLinguisticTaggerOptions {
    OmitWords = 1,
    OmitPunctuation = 2,
    OmitWhitespace = 4,
    OmitOther = 8,
    JoinNames = 16,
}

#[derive(Debug)]
#[repr(i64)]
pub enum NSUbiquitousKeyValueStoreChangeReason {
    ServerChange,
    InitialSyncChange,
    QuotaViolationChange,
    AccountChange,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSJsonReadingOptions {
    MutableContainers = 1,
    MutableLeaves = 2,
    FragmentsAllowed = 4,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSJsonWritingOptions {
    PrettyPrinted = 1,
    SortedKeys = (1 << 1),
    FragmentsAllowed = (1 << 2),
    WithoutEscapingSlashes = (1 << 3),
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSLocaleLanguageDirection {
    Unknown,
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

#[derive(Debug)]
#[repr(i64)]
pub enum NSAlignmentOptions {
    MinXInward = 1 << 0,
    MinYInward = 1 << 1,
    MaxXInward = 1 << 2,
    MaxYInward = 1 << 3,
    WidthInward = 1 << 4,
    HeightInward = 1 << 5,

    MinXOutward = 1 << 8,
    MinYOutward = 1 << 9,
    MaxXOutward = 1 << 10,
    MaxYOutward = 1 << 11,
    WidthOutward = 1 << 12,
    HeightOutward = 1 << 13,

    MinXNearest = 1 << 16,
    MinYNearest = 1 << 17,
    MaxXNearest = 1 << 18,
    MaxYNearest = 1 << 19,
    WidthNearest = 1 << 20,
    HeightNearest = 1 << 21,

    RectFlipped = (1_u64 << 63) as i64,
}

impl NSAlignmentOptions {
    // pub const AllEdgesInward: Self = Self::MinXInward | Self::MaxXInward | Self::MinYInward | Self:: MaxYInward;
    // pub const AllEdgesOutward: Self = Self::MinXOutward | Self::MaxXOutward | Self:MinYOutward | Self::MaxYOutward;
    // pub const AllEdgesNearest: Self = Self::MinXNearest | Self::MaxXNearest | Self::MinYNearest | Self::MaxYNearest,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSFileWrapperReadingOptions {
    Immediate = 1 << 0,
    WithoutMapping = 1 << 1,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSFileWrapperWritingOptions {
    Atomic = 1 << 0,
    WithNameUpdating = 1 << 1,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSAttributedStringEnumerationOptions {
    None = 0,
    Reverse = 1 << 1,
    LongestEffectiveRangeNotRequired = 1 << 20,
}

#[derive(Debug)]
#[repr(i64)]
pub enum NSWritingDirection {
    Natural = -1,
    LeftToRight = 0,
    RightToLeft = 1,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSByteCountFormatterUnits {
    UseDefault = 0,
    UseBytes = 1 << 0,
    UseKB = 1 << 1,
    UseMB = 1 << 2,
    UseGB = 1 << 3,
    UseTB = 1 << 4,
    UsePB = 1 << 5,
    UseEB = 1 << 6,
    UseZB = 1 << 7,
    UseYBOrHigher = 0x0FF << 8,
    UseAll = 0x0FFFF,
}

#[derive(Debug)]
#[repr(i64)]
pub enum NSByteCountFormatterCountStyle {
    File,
    Memory,
    Decimal,
    Binary,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSUrlBookmarkCreationOptions {
    PreferFileIDResolution = 1 << 8,
    MinimalBookmark = 1 << 9,
    SuitableForBookmarkFile = 1 << 10,
    WithSecurityScope = 1 << 11,
    SecurityScopeAllowOnlyReadAccess = 1 << 12,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSUrlBookmarkResolutionOptions {
    WithoutUI = 1 << 8,
    WithoutMounting = 1 << 9,
    WithSecurityScope = 1 << 10,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSLigatureType {
    None,
    Default,
    All,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSCalendarOptions {
    None = 0,
    WrapCalendarComponents = 1 << 0,

    #[cfg(any(target_os = "ios", target_os = "macos"))]
    MatchStrictly = 1 << 1,
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    SearchBackwards = 1 << 2,

    #[cfg(any(target_os = "ios", target_os = "macos"))]
    MatchPreviousTimePreservingSmallerUnits = 1 << 8,
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    MatchNextTimePreservingSmallerUnits = 1 << 9,
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    MatchNextTime = 1 << 10,

    #[cfg(any(target_os = "ios", target_os = "macos"))]
    MatchFirst = 1 << 12,
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    MatchLast = 1 << 13,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSUrlRequestNetworkServiceType {
    Default,
    #[deprecated(note = "Use 'PushKit' framework instead.")]
    VoIP,
    Video,
    Background,
    Voice,
    ResponsiveData = 6,
    AVStreaming = 8,
    ResponsiveAV = 9,
    CallSignaling = 11,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSSortOptions {
    Concurrent = 1 << 0,
    Stable = 1 << 4,
}

#[cfg(target_os = "ios")]
#[deprecated(note = "Use 'NSWritingDirectionFormatType'.")]
#[derive(Debug)]
#[repr(i64)]
pub enum NSTextWritingDirection {
    Embedding = 0,
    Override = 2,
}

#[derive(Debug)]
#[repr(i64)]
pub enum NSUrlSessionAuthChallengeDisposition {
    UseCredential = 0,
    PerformDefaultHandling = 1,
    CancelAuthenticationChallenge = 2,
    RejectProtectionSpace = 3,
}

#[derive(Debug)]
#[repr(i64)]
pub enum NSUrlSessionTaskState {
    Running = 0,
    Suspended = 1,
    Canceling = 2,
    Completed = 3,
}

#[derive(Debug)]
#[repr(i64)]
pub enum NSUrlSessionResponseDisposition {
    Cancel = 0,
    Allow = 1,
    BecomeDownload = 2,
    BecomeStream = 3,
}

#[derive(Debug)]
#[repr(i64)]
pub enum NSUrlErrorCancelledReason {
    UserForceQuitApplication,
    BackgroundUpdatesDisabled,
    InsufficientSystemResources,
}

#[derive(Debug)]
#[repr(i64)]
pub enum NSTimeZoneNameStyle {
    Standard,
    ShortStandard,
    DaylightSaving,
    ShortDaylightSaving,
    Generic,
    ShortGeneric,
}

#[derive(Debug)]
#[repr(i64)]
pub enum NSItemProviderErrorCode {
    Unknown = -1,
    None = 0,
    ItemUnavailable = -1000,
    UnexpectedValueClass = -1100,
    UnavailableCoercion = -1200,
}

#[derive(Debug)]
#[repr(i64)]
pub enum NSDateComponentsFormatterUnitsStyle {
    Positional = 0,
    Abbreviated,
    Short,
    Full,
    SpellOut,
    Brief,
}

/// The formatting context for a formatter.
#[cfg(any(target_os = "ios", target_os = "macos"))]
#[derive(Debug)]
#[repr(i64)]
pub enum NSFormattingContext {
    /// An unknown formatting context.
    Unknown = 0,
    /// A formatting context determined automatically at runtime.
    Dynamic = 1,
    /// The formatting context for stand-alone usage.
    Standalone = 2,
    /// The formatting context for a list or menu item.
    ListItem = 3,
    /// The formatting context for the beginning of a sentence.
    BeginningOfSentence = 4,
    /// The formatting context for the middle of a sentence.
    MiddleOfSentence = 5,
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
#[derive(Debug)]
#[repr(u64)]
pub enum NSDateIntervalFormatterStyle {
    None = 0,
    Short = 1,
    Medium = 2,
    Long = 3,
    Full = 4,
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
#[derive(Debug)]
#[repr(i64)]
pub enum NSEnergyFormatterUnit {
    Joule = 11,
    Kilojoule = 14,
    Calorie = (7 << 8) + 1,
    Kilocalorie = (7 << 8) + 2,
}

/// Specifies the width of the unit, determining the textual representation.
#[cfg(any(target_os = "ios", target_os = "macos"))]
#[derive(Debug)]
#[repr(i64)]
pub enum NSFormattingUnitStyle {
    /// Specifies a short unit style.
    Short = 1,
    /// Specifies a medium unit style.
    Medium,
    /// Specifies a long unit style.
    Long,
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
#[derive(Debug)]
#[repr(i64)]
pub enum NSMassFormatterUnit {
    Gram = 11,
    Kilogram = 14,
    Ounce = (6 << 8) + 1,
    Pound = (6 << 8) + 2,
    Stone = (6 << 8) + 3,
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
#[derive(Debug)]
#[repr(i64)]
pub enum NSLengthFormatterUnit {
    Millimeter = 8,
    Centimeter = 9,
    Meter = 11,
    Kilometer = 14,
    Inch = (5 << 8) + 1,
    Foot = (5 << 8) + 2,
    Yard = (5 << 8) + 3,
    Mile = (5 << 8) + 4,
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
#[derive(Debug)]
#[repr(i64)]
pub enum NSQualityOfService {
    UserInteractive = 33,
    UserInitiated = 25,
    Utility = 17,
    Background = 9,
    Default = -1,
}

#[derive(Debug)]
#[repr(i64)]
pub enum NSUrlRelationship {
    Contains,
    Same,
    Other,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSTextCheckingType {
    Orthography = 1 << 0,
    Spelling = 1 << 1,
    Grammar = 1 << 2,
    Date = 1 << 3,
    Address = 1 << 4,
    Link = 1 << 5,
    Quote = 1 << 6,
    Dash = 1 << 7,
    Replacement = 1 << 8,
    Correction = 1 << 9,
    RegularExpression = 1 << 10,
    PhoneNumber = 1 << 11,
    TransitInformation = 1 << 12,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSRegularExpressionOptions {
    CaseInsensitive = 1 << 0,
    AllowCommentsAndWhitespace = 1 << 1,
    IgnoreMetacharacters = 1 << 2,
    DotMatchesLineSeparators = 1 << 3,
    AnchorsMatchLines = 1 << 4,
    UseUnixLineSeparators = 1 << 5,
    UseUnicodeWordBoundaries = 1 << 6,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSMatchingOptions {
    ReportProgress = 1 << 0,
    ReportCompletion = 1 << 1,
    Anchored = 1 << 2,
    WithTransparentBounds = 1 << 3,
    WithoutAnchoringBounds = 1 << 4,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSMatchingFlags {
    Progress = 1 << 0,
    Completed = 1 << 1,
    HitEnd = 1 << 2,
    RequiredEnd = 1 << 3,
    InternalError = 1 << 4,
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
#[derive(Debug)]
#[repr(u64)]
pub enum NSPersonNameComponentsFormatterOptions {
    Phonetic = (1 << 1),
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
#[derive(Debug)]
#[repr(i64)]
pub enum NSPersonNameComponentsFormatterStyle {
    Default = 0,
    Short,
    Medium,
    Long,
    Abbreviated,
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
#[derive(Debug)]
#[repr(i64)]
pub enum NSDecodingFailurePolicy {
    RaiseException,
    SetErrorAndReturn,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSIso8601DateFormatOptions {
    Year = 1 << 0,
    Month = 1 << 1,
    WeekOfYear = 1 << 2,
    Day = 1 << 4,
    Time = 1 << 5,
    TimeZone = 1 << 6,
    SpaceBetweenDateAndTime = 1 << 7,
    DashSeparatorInDate = 1 << 8,
    ColonSeparatorInTime = 1 << 9,
    ColonSeparatorInTimeZone = 1 << 10,
    FractionalSeconds = 1 << 11,
    // TODO: BitOR for `NSIso8601DateFormatOptions`

    // FullDate = Year | Month | Day | DashSeparatorInDate,
    // FullTime = Time | ColonSeparatorInTime | TimeZone | ColonSeparatorInTimeZone,
    // InternetDateTime = FullDate | FullTime,
}

#[derive(Debug)]
#[repr(i64)]
pub enum NSUrlSessionTaskMetricsResourceFetchType {
    Unknown,
    NetworkLoad,
    ServerPush,
    LocalCache,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSMeasurementFormatterUnitOptions {
    ProvidedUnit = (1 << 0),
    NaturalScale = (1 << 1),
    TemperatureWithoutUnit = (1 << 2),
}

#[derive(Debug)]
#[repr(i64)]
pub enum NSItemProviderRepresentationVisibility {
    All = 0,
    Team = 1,
    Group = 2,
    OwnProcess = 3,
}

#[derive(Debug)]
#[repr(i64)]
pub enum NSItemProviderFileOptions {
    OpenInPlace = 1,
}

#[derive(Debug)]
#[repr(i64)]
pub enum NSLinguisticTaggerUnit {
    Word,
    Sentence,
    Paragraph,
    Document,
}

#[derive(Debug)]
#[repr(i64)]
pub enum NSUrlSessionDelayedRequestDisposition {
    ContinueLoading = 0,
    UseNewRequest = 1,
    Cancel = 2,
}

#[derive(Debug)]
#[repr(u64)]
pub enum NSXpcConnectionOptions {
    Privileged = (1 << 12),
}

#[derive(Debug, Copy, Clone)]
#[repr(u64)]
pub enum NSStringEncodingConversionOptions {
    AllowLossy = 1,
    ExternalRepresentation = 2,
}

///
#[derive(Debug)]
#[repr(u64)]
pub enum NSRectEdge {
    ///
    MinXEdge,
    ///
    MinYEdge,
    ///
    MaxXEdge,
    ///
    MaxYEdge,
}

impl NSRectEdge {
    ///
    #[allow(non_upper_case_globals)]
    pub const MaxX: CGRectEdge = CGRectEdge::MaxXEdge;

    ///
    #[allow(non_upper_case_globals)]
    pub const MaxY: CGRectEdge = CGRectEdge::MaxYEdge;

    ///
    #[allow(non_upper_case_globals)]
    pub const MinX: CGRectEdge = CGRectEdge::MinXEdge;

    ///
    #[allow(non_upper_case_globals)]
    pub const MinY: CGRectEdge = CGRectEdge::MinYEdge;
}
