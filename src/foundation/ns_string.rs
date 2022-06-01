use std::{
    ffi::{CStr, CString},
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut, Range},
    string::String,
};

use libc::{c_char, c_void};
use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    foundation::{traits::INSString, Int, NSComparisonResult, NSLocale, UInt},
    id,
    objective_c_runtime::traits::{FromId, PNSObject, ToId},
    utils::to_bool,
};

use super::{
    string::Encoding, unichar, NSArray, NSData, NSMutableString, NSStringCompareOptions,
    NSStringTransform, UTF8_ENCODING,
};

/// This is a mapping to the Objective-C NSString class.
#[repr(C)]
pub struct NSString {
    /// The raw pointer to the Objective-C object.
    pub ptr: Id<Object>,
    marker: PhantomData<()>,
}

impl NSString {
    /// Returns the UTF8 bytes for this `NSString`.
    pub fn bytes(&self) -> *const c_char {
        unsafe {
            let bytes: *const c_char = msg_send![&*self.ptr, UTF8String];
            bytes
        }
    }

    /// Convert this `NSString` into a `&str`.
    pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
        let bytes = self.bytes();

        unsafe {
            let bytes = CStr::from_ptr(bytes);
            bytes.to_str()
        }
    }
}

impl PNSObject for NSString {
    fn class<'a>() -> &'a objc::runtime::Class {
        class!(NSString)
    }

    fn superclass<'a>() -> &'a objc::runtime::Class {
        unsafe { msg_send![Self::class(), superclass] }
    }

    fn isEqual(&self, object: &Self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isEqual: &*object.ptr]) }
    }

    fn hash(&self) -> UInt {
        unsafe { msg_send![&*self.ptr, hash] }
    }

    fn isKindOfClass(&self, aClass: objc::runtime::Class) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isKindOfClass: aClass]) }
    }

    fn isMemberOfClass(&self, aClass: objc::runtime::Class) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isMemberOfClass: aClass]) }
    }

    fn respondsToSelector(&self, aSelector: objc::runtime::Sel) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, respondsToSelector: aSelector]) }
    }

    fn conformsToProtocol(&self, aProtocol: objc::runtime::Protocol) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, conformsToProtocol: aProtocol]) }
    }

    fn description(&self) -> NSString {
        unsafe { msg_send![&*self.ptr, description] }
    }

    fn debugDescription(&self) -> NSString {
        unsafe { msg_send![&*self.ptr, debugDescription] }
    }

    fn performSelector(&self, aSelector: objc::runtime::Sel) -> id {
        unsafe { msg_send![&*self.ptr, performSelector: aSelector] }
    }

    fn performSelector_withObject(&self, aSelector: objc::runtime::Sel, withObject: id) -> id {
        unsafe { msg_send![&*self.ptr, performSelector: aSelector withObject: withObject] }
    }

    fn isProxy(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isProxy]) }
    }
}

impl INSString for NSString {
    fn tm_string() -> Self {
        unsafe { Self::from_id(msg_send![class!(NSMutableString), string]) }
    }

    fn im_init(self) -> Self {
        unsafe { Self::from_id(msg_send![self.ptr, init]) }
    }

    fn im_initWithBytes_length_encoding(
        self,
        bytes: *const c_void,
        len: UInt,
        encoding: Encoding,
    ) -> Self {
        unsafe {
            Self::from_id(msg_send![self.ptr, initWithBytes: bytes length: len encoding: encoding])
        }
    }

    fn im_initWithBytesNoCopy_length_encoding_freeWhenDone(
        self,
        bytes: *mut c_void,
        len: UInt,
        encoding: Encoding,
        freeBuffer: bool,
    ) -> Self {
        unsafe {
            Self::from_id(msg_send![
                class!(NSMutableString),
                stringWithBytesNoCopy: bytes
                length: len
                encoding: encoding
                freeWhenDone: freeBuffer
            ])
        }
    }

    fn im_initWithCharacters_length(self, characters: *const unichar, len: UInt) -> Self {
        unsafe {
            Self::from_id(
                msg_send![class!(NSMutableString), stringWithCharacters: characters length: len],
            )
        }
    }

    fn im_initWithCharactersNoCopy_length_freeWhenDone(
        self,
        characters: unichar,
        length: UInt,
        freeBuffer: bool,
    ) -> Self {
        unsafe {
            Self::from_id(msg_send![
                self.ptr,
                initWithCharactersNoCopy: characters
                length: length
                freeWhenDone: freeBuffer
            ])
        }
    }

    fn im_initWithString<S>(self, s: S) -> Self
    where
        S: Into<NSString>,
    {
        unsafe { Self::from_id(msg_send![self.ptr, initWithString: s.into().ptr]) }
    }

    fn im_initWithCString_encoding(self, c_str: *const c_char, encoding: Encoding) -> Self {
        unsafe { Self::from_id(msg_send![self.ptr, initWithCString: c_str encoding: encoding]) }
    }

    fn im_initWithUTF8String(self, c_str: *const c_char) -> Self {
        unsafe { Self::from_id(msg_send![self.ptr, initWithUTF8String: c_str]) }
    }

    fn im_initWithData_encoding(self, data: NSData, encoding: Encoding) -> Self {
        unsafe { Self::from_id(msg_send![self.ptr, initWithData: data encoding: encoding]) }
    }

    fn tm_localizedUserNotificationStringForKey_arguments<K, T>(
        key: K,
        arguments: NSArray<T>,
    ) -> NSString
    where
        K: Into<NSString>,
        T: PNSObject,
    {
        unsafe {
            NSString::from_id(msg_send![
                class!(NSMutableString),
                localizedUserNotificationStringForKey: key.into()
                arguments: arguments
            ])
        }
    }

    fn tm_stringWithCharacters_length(characters: *const unichar, length: UInt) -> Self {
        unsafe {
            Self::from_id(
                msg_send![class!(NSMutableString), stringWithCharacters: characters length: length],
            )
        }
    }

    fn tm_stringWithString<S>(s: S) -> Self
    where
        S: Into<NSString>,
    {
        unsafe { Self::from_id(msg_send![class!(NSMutableString), stringWithString: s.into()]) }
    }

    fn tm_stringWithCString_encoding(c_str: *const c_char, encoding: Encoding) -> Self {
        unsafe {
            Self::from_id(
                msg_send![class!(NSMutableString), stringWithCString: c_str encoding: encoding],
            )
        }
    }

    fn tm_stringWithUTF8String(c_str: *const c_char) -> Self {
        unsafe {
            Self::from_id(msg_send![
                class!(NSMutableString),
                stringWithUTF8String: c_str
            ])
        }
    }

    fn ip_length(&self) -> UInt {
        unsafe { msg_send![self.ptr, length] }
    }

    fn im_lengthOfBytesUsingEncoding(&self, enc: Encoding) -> UInt {
        unsafe { msg_send![self.ptr, lengthOfBytesUsingEncoding: enc] }
    }

    fn im_maximumLengthOfBytesUsingEncoding(&self, enc: Encoding) -> Int {
        unsafe { msg_send![self.ptr, maximumLengthOfBytesUsingEncoding: enc] }
    }

    fn im_characterAtIndex(&self, index: UInt) -> char {
        unsafe {
            let c: u8 = msg_send![self.ptr, characterAtIndex: index];
            c as char
        }
    }

    fn im_getCharacters_range(&self, buffer: *mut unichar, range: Range<UInt>) {
        unsafe { msg_send![self.ptr, getCharacters: buffer range: range] }
    }

    fn im_getBytes_maxLength_usedLength_encoding_options_range_remainingRange(
        &self,
        buffer: *mut c_void,
        maxLength: Int,
        usedLength: *mut Int,
        encoding: Encoding,
        options: super::NSStringEncodingConversionOptions,
        range: Range<UInt>,
        remainingRange: Range<UInt>,
    ) -> bool {
        unsafe {
            to_bool(msg_send![
                self.ptr,
                getBytes: buffer
                maxLength: maxLength
                usedLength: usedLength
                encoding: encoding
                options: options
                range: range
                remainingRange: remainingRange
            ])
        }
    }

    fn im_cStringUsingEncoding(&self, encoding: Encoding) -> *const c_char {
        unsafe { msg_send![self.ptr, cStringUsingEncoding: encoding] }
    }

    fn im_getCString_maxLength_encoding(
        &self,
        buffer: *mut c_char,
        maxLength: UInt,
        encoding: Encoding,
    ) -> bool {
        unsafe {
            to_bool(msg_send![self.ptr, getCString: buffer maxLength: maxLength encoding: encoding])
        }
    }

    fn ip_UTF8String(&self) -> *const c_char {
        unsafe { msg_send![self.ptr, UTF8String] }
    }

    fn im_caseInsensitiveCompare<S>(&self, string: S) -> NSComparisonResult
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, caseInsensitiveCompare: string.into()] }
    }

    fn im_localizedCaseInsensitiveCompare<S>(&self, string: S) -> NSComparisonResult
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, localizedCaseInsensitiveCompare: string.into()] }
    }

    fn im_compare<S>(&self, string: S) -> NSComparisonResult
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, compare: string.into()] }
    }

    fn im_localizedCompare<S>(&self, string: S) -> NSComparisonResult
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, localizedCompare: string.into()] }
    }

    fn im_compare_options<S>(&self, string: S, mask: NSStringCompareOptions) -> NSComparisonResult
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, compare: string.into() options: mask] }
    }

    fn im_compare_options_range<S>(
        &self,
        string: S,
        mask: NSStringCompareOptions,
        range: Range<UInt>,
    ) -> NSComparisonResult
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, compare: string.into() options: mask range: range] }
    }

    fn im_compare_options_range_locale<S>(
        &self,
        string: S,
        mask: NSStringCompareOptions,
        range: Range<UInt>,
        locale: NSLocale,
    ) -> NSComparisonResult
    where
        S: Into<NSString>,
    {
        unsafe {
            msg_send![self.ptr, compare: string.into() options: mask range: range locale: locale]
        }
    }

    fn im_localized_standard_compare<S>(&self, string: S) -> NSComparisonResult
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, localizedStandardCompare: string.into()] }
    }

    fn im_hasPrefix<S>(&self, prefix: S) -> bool
    where
        S: Into<NSString>,
    {
        unsafe { to_bool(msg_send![self.ptr, hasPrefix: prefix.into()]) }
    }

    fn im_hasSuffix<S>(&self, suffix: S) -> bool
    where
        S: Into<NSString>,
    {
        unsafe { to_bool(msg_send![self.ptr, hasSuffix: suffix.into()]) }
    }

    fn im_isEqualToString<S>(&self, string: S) -> bool
    where
        S: Into<NSString>,
    {
        unsafe { to_bool(msg_send![self.ptr, isEqualToString: string.into()]) }
    }

    fn im_stringByAppendingString<S>(&self, string: S) -> NSString
    where
        S: Into<NSString>,
    {
        unsafe { NSString::from_id(msg_send![self.ptr, stringByAppendingString: string.into()]) }
    }

    fn im_stringByPaddingToLength_withString_startingAtIndex<S>(
        &self,
        new_length: UInt,
        pad_string: S,
        starting_at: UInt,
    ) -> NSString
    where
        S: Into<NSString>,
    {
        unsafe {
            NSString::from_id(msg_send![
                self.ptr,
                stringByPaddingToLength: new_length
                withString: pad_string.into()
                startingAtIndex: starting_at
            ])
        }
    }

    fn ip_lowercaseString(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, lowercaseString]) }
    }

    fn ip_localizedLowercaseString(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, localizedLowercaseString]) }
    }

    fn im_lowercaseStringWithLocale(&self, locale: NSLocale) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, lowercaseStringWithLocale: locale]) }
    }

    fn ip_uppercaseString(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, uppercaseString]) }
    }

    fn ip_localizedUppercaseString(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, localizedUppercaseString]) }
    }

    fn im_uppercaseStringWithLocale(&self, locale: NSLocale) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, uppercaseStringWithLocale: locale]) }
    }

    fn ip_capitalizedString(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, capitalizedString]) }
    }

    fn ip_localizedCapitalizedString(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, localizedCapitalizedString]) }
    }

    fn im_capitalizedStringWithLocale(&self, locale: NSLocale) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, capitalizedStringWithLocale: locale]) }
    }

    fn im_componentsSeparatedByString<S>(&self, separator: S) -> NSArray<NSString>
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, componentsSeparatedByString: separator.into()] }
    }

    fn contains<S>(&self, other: S) -> bool
    where
        S: Into<NSString>,
    {
        unsafe { to_bool(msg_send![self.ptr, containsString: other.into()]) }
    }

    fn stringByApplyingTransform(
        &mut self,
        transform: NSStringTransform,
        reverse: bool,
    ) -> Option<NSString> {
        unsafe {
            let result: id =
                msg_send![self.ptr, stringByApplyingTransform: transform reverse: reverse];
            if result.is_null() {
                None
            } else {
                Some(NSString::from_id(result))
            }
        }
    }

    fn availableStringEncodings() -> *const Encoding {
        unsafe { msg_send![NSString::class(), availableStringEncodings] }
    }

    fn defaultCStringEncoding() -> Encoding {
        unsafe { msg_send![NSString::class(), defaultCStringEncoding] }
    }

    fn canBeConvertedToEncoding(&self, encoding: Encoding) -> bool {
        unsafe { msg_send![self.ptr, canBeConvertedToEncoding: encoding] }
    }

    fn dataUsingEncoding(&self, encoding: Encoding) -> NSData {
        unsafe { NSData::from_id(msg_send![self.ptr, dataUsingEncoding: encoding]) }
    }
}

impl Default for NSString {
    fn default() -> Self {
        Self::tm_string()
    }
}

impl fmt::Debug for NSString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description().as_str().unwrap())
    }
}

impl fmt::Display for NSString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description().as_str().unwrap())
    }
}

impl Clone for NSString {
    fn clone(&self) -> Self {
        unsafe { msg_send![&*self.ptr, retain] }
    }
}

impl Deref for NSString {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.ptr
    }
}

impl DerefMut for NSString {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.ptr
    }
}

impl ToId for NSString {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSString {
    unsafe fn from_id(id: id) -> Self {
        NSString {
            ptr: Id::from_ptr(id),
            marker: PhantomData,
        }
    }
}

impl From<NSMutableString> for NSString {
    fn from(string: NSMutableString) -> Self {
        unsafe {
            let ptr: id = msg_send![NSString::class(), alloc];
            let ptr = msg_send![ptr, initWithString: string];
            NSString::from_id(ptr)
        }
    }
}

impl From<String> for NSString {
    /// Creates a new `NSString` from a `String`.
    fn from(s: String) -> Self {
        let c_string = CString::new(s.clone()).unwrap();
        NSString {
            ptr: unsafe {
                let nsstring: id = msg_send![class!(NSString), alloc];
                Id::from_ptr(
                    msg_send![nsstring, initWithBytes:c_string.into_raw() as *mut Object
                        length:s.len()
                        encoding:UTF8_ENCODING
                    ],
                )
            },

            marker: PhantomData,
        }
    }
}

impl From<&str> for NSString {
    /// Creates a new `NSString` from a `&str`.
    fn from(s: &str) -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(
                msg_send![nsstring, initWithBytes: CString::new(s).unwrap().into_raw() as *mut Object
                    length:s.len()
                    encoding:UTF8_ENCODING
                ],
            )
        };

        NSString {
            ptr: objc,
            marker: PhantomData,
        }
    }
}

impl From<char> for NSString {
    /// Creates a new `NSString` from a `char`.
    fn from(c: char) -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(
                msg_send![nsstring, initWithBytes: c.encode_utf8(&mut [0; 4]).as_ptr()
                    length:1
                    encoding:UTF8_ENCODING
                ],
            )
        };

        NSString {
            ptr: objc,
            marker: PhantomData,
        }
    }
}

impl From<NSString> for &str {
    /// Converts a `NSString` to a `&str`.
    fn from(string: NSString) -> Self {
        unsafe {
            let ptr: *const c_char = msg_send![string.ptr, UTF8String];
            CStr::from_ptr(ptr).to_str().unwrap()
        }
    }
}

impl From<(&str, Encoding)> for NSString {
    /// Creates a new `NSString` from a `&str` and an encoding.
    fn from((s, encoding): (&str, Encoding)) -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(msg_send![nsstring, initWithBytes:s.as_ptr()
                length:s.len()
                encoding:encoding
            ])
        };

        NSString {
            ptr: objc,
            marker: PhantomData,
        }
    }
}

impl PartialEq for NSString {
    /// Checks if two `NSString`s are equal.
    fn eq(&self, other: &Self) -> bool {
        self.im_localizedCompare(other.clone()) == NSComparisonResult::OrderedSame
    }
}

impl PartialEq<&str> for NSString {
    /// Checks if a `NSString` is equal to a `&str`.
    fn eq(&self, other: &&str) -> bool {
        self.as_str().unwrap() == *other
    }
}

#[cfg(test)]
mod tests {
    use crate::foundation::{
        string::Encoding, string_transform::LatinToKatakana, traits::INSString, NSComparisonResult,
    };

    use super::*;

    #[test]
    fn test_from_str() {
        let s = NSString::from("Hello, World!");
        assert_eq!(s, "Hello, World!");
    }

    #[test]
    fn test_bytes() {
        let s = NSString::from("Hello, World!");
        let other = s.bytes();
        assert_eq!(s.bytes(), other);
    }

    #[test]
    fn test_bytes_len() {
        let s = NSString::from("Hello, World!");
        assert_eq!(s.im_lengthOfBytesUsingEncoding(Encoding::UTF8), 13);
    }

    #[test]
    fn test_as_str() {
        let s = NSString::from("Hello, World!");
        assert_eq!(s, "Hello, World!");
    }

    #[test]
    fn test_to_string() {
        let s = NSString::from("Hello, World!");
        assert_eq!(s.to_string(), "Hello, World!".to_string());
    }

    #[test]
    fn test_length() {
        let s = NSString::from("Hello, World!");
        assert_eq!(s.ip_length(), 13);
    }

    #[test]
    fn test_contains() {
        let s = NSString::from("Hello, World!");
        assert!(s.contains("Hello"));
        assert!(!s.contains("Goodbye"));
    }

    #[test]
    fn test_character_at() {
        let s = NSString::from("Hello, World!");
        assert_eq!(s.im_characterAtIndex(0), 'H');
        assert_eq!(s.im_characterAtIndex(1), 'e');
        assert_eq!(s.im_characterAtIndex(2), 'l');
        assert_eq!(s.im_characterAtIndex(3), 'l');
        assert_eq!(s.im_characterAtIndex(4), 'o');
        assert_eq!(s.im_characterAtIndex(5), ',');
        assert_eq!(s.im_characterAtIndex(6), ' ');
        assert_eq!(s.im_characterAtIndex(7), 'W');
        assert_eq!(s.im_characterAtIndex(8), 'o');
        assert_eq!(s.im_characterAtIndex(9), 'r');
        assert_eq!(s.im_characterAtIndex(10), 'l');
        assert_eq!(s.im_characterAtIndex(11), 'd');
        assert_eq!(s.im_characterAtIndex(12), '!');
    }

    #[test]
    fn test_has_prefix() {
        let s = NSString::from("Hello, World!");
        assert!(s.im_hasPrefix("Hello"));
        assert!(!s.im_hasPrefix("Goodbye"));
    }

    #[test]
    fn test_has_suffix() {
        let s = NSString::from("Hello, World!");
        assert!(s.im_hasSuffix("World!"));
        assert!(!s.im_hasSuffix("Goodbye"));
    }

    #[test]
    fn test_is_equal_to() {
        let s = NSString::from("Hello, World!");
        assert!(s.im_isEqualToString("Hello, World!"));
        assert!(!s.im_isEqualToString("Goodbye, World!"));
    }

    #[test]
    fn test_length_of_bytes() {
        let s = NSString::from("Hello, World!");
        assert_eq!(s.im_lengthOfBytesUsingEncoding(Encoding::UTF8), 13);
    }

    #[test]
    fn test_maximum_length_of_bytes() {
        let s = NSString::from("Hello, World!");
        assert_eq!(s.im_maximumLengthOfBytesUsingEncoding(Encoding::UTF8), 39);
    }

    #[test]
    fn test_case_insensitive_compare() {
        let s = NSString::from("Hello, World!");
        assert_eq!(
            s.im_caseInsensitiveCompare("hello, world!"),
            NSComparisonResult::OrderedSame
        );
        assert_eq!(
            s.im_caseInsensitiveCompare("goodbye, world!"),
            NSComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_localized_case_insensitive_compare() {
        let s = NSString::from("Hello, World!");
        assert_eq!(
            s.im_localizedCaseInsensitiveCompare("hello, world!"),
            NSComparisonResult::OrderedSame
        );
        assert_eq!(
            s.im_localizedCaseInsensitiveCompare("goodbye, world!"),
            NSComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_compare() {
        let s = NSString::from("Hello, World!");
        assert_eq!(
            s.im_compare("Hello, World!"),
            NSComparisonResult::OrderedSame
        );
        assert_eq!(
            s.im_compare("Goodbye, World!"),
            NSComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_localized_standard_compare() {
        let s = NSString::from("Hello, World!");
        assert_eq!(
            s.im_localized_standard_compare("Hello, World!"),
            NSComparisonResult::OrderedSame
        );
        assert_eq!(
            s.im_localized_standard_compare("Goodbye, World!"),
            NSComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_applying_transform() {
        let mut s = NSString::from("Katakana!");
        let transform = unsafe { LatinToKatakana };
        assert_eq!(
            s.stringByApplyingTransform(transform, false).unwrap(),
            "カタカナ!"
        );
    }

    #[test]
    fn test_im_caseInsensitiveCompare() {
        let s = NSString::from("Hello, World!");
        assert_eq!(
            s.im_caseInsensitiveCompare("hello, world!"),
            NSComparisonResult::OrderedSame
        );
        assert_eq!(
            s.im_caseInsensitiveCompare("goodbye, world!"),
            NSComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_im_hasPrefix() {
        let s = NSString::from("Hello, World!");
        assert!(s.im_hasPrefix("Hello"));
        assert!(!s.im_hasPrefix("Goodbye"));
    }

    #[test]
    fn test_im_hasSuffix() {
        let s = NSString::from("Hello, World!");
        assert!(s.im_hasSuffix("World!"));
        assert!(!s.im_hasSuffix("Goodbye"));
    }

    #[test]
    fn test_im_stringByPaddingToLength_withString_startingAtIndex() {
        let s = NSString::from("Hello, World!");

        assert_eq!(
            s.im_stringByPaddingToLength_withString_startingAtIndex(20, ".", 0),
            "Hello, World!......."
        );
    }
}
