use std::{fmt, ops::Range};

use objc::{
    class, msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    id,
    objective_c_runtime::traits::{FromId, PNSObject, ToId},
    utils::to_bool,
};

use super::{
    string,
    traits::{INSMutableString, INSString},
    Int, NSComparisonResult, NSString, UInt,
};

/// A mutable string.
#[repr(C)]
pub struct NSMutableString {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSMutableString {
    fn class<'a>() -> &'a Class {
        class!(NSMutableString)
    }

    fn superclass<'a>() -> &'a Class {
        unsafe { msg_send![Self::class(), superclass] }
    }

    fn isEqual(&self, object: &Self) -> bool {
        unsafe { msg_send![self.ptr, isEqual: object] }
    }

    fn hash(&self) -> UInt {
        unsafe { msg_send![self.ptr, hash] }
    }

    fn isKindOfClass(&self, aClass: Class) -> bool {
        unsafe { msg_send![self.ptr, isKindOfClass: aClass] }
    }

    fn isMemberOfClass(&self, aClass: Class) -> bool {
        unsafe { msg_send![self.ptr, isMemberOfClass: aClass] }
    }

    fn respondsToSelector(&self, aSelector: Sel) -> bool {
        unsafe { msg_send![self.ptr, respondsToSelector: aSelector] }
    }

    fn conformsToProtocol(&self, aProtocol: Protocol) -> bool {
        unsafe { msg_send![self.ptr, conformsToProtocol: aProtocol] }
    }

    fn description(&self) -> NSString {
        unsafe { msg_send![self.ptr, description] }
    }

    fn debugDescription(&self) -> NSString {
        unsafe { msg_send![self.ptr, debugDescription] }
    }

    fn performSelector(&self, aSelector: Sel) -> id {
        unsafe { msg_send![self.ptr, performSelector: aSelector] }
    }

    fn performSelector_withObject(&self, aSelector: Sel, withObject: id) -> id {
        unsafe { msg_send![self.ptr, performSelector: aSelector withObject: withObject] }
    }

    fn isProxy(&self) -> bool {
        unsafe { msg_send![self.ptr, isProxy] }
    }
}

impl INSString for NSMutableString {
    fn tm_string() -> Self {
        unsafe {
            NSMutableString {
                ptr: msg_send![class!(NSMutableString), string],
            }
        }
    }

    fn im_init(self) -> Self {
        unsafe {
            NSMutableString {
                ptr: msg_send![self.ptr, init],
            }
        }
    }

    fn im_initWithBytes_length_encoding(
        self,
        bytes: *const libc::c_void,
        len: UInt,
        encoding: string::Encoding,
    ) -> Self {
        unsafe {
            NSMutableString {
                ptr: msg_send![self.ptr, initWithBytes: bytes length: len encoding: encoding],
            }
        }
    }

    fn im_initWithBytesNoCopy_length_encoding_freeWhenDone(
        self,
        bytes: *mut libc::c_void,
        len: UInt,
        encoding: string::Encoding,
        freeBuffer: bool,
    ) -> Self {
        unsafe {
            NSMutableString {
                ptr: msg_send![
                    class!(NSMutableString),
                    stringWithBytesNoCopy: bytes
                    length: len
                    encoding: encoding
                    freeWhenDone: freeBuffer
                ],
            }
        }
    }

    fn im_initWithCharacters_length(self, characters: *const super::unichar, len: UInt) -> Self {
        unsafe {
            NSMutableString {
                ptr: msg_send![class!(NSMutableString), stringWithCharacters: characters length: len],
            }
        }
    }

    fn im_initWithCharactersNoCopy_length_freeWhenDone(
        self,
        characters: super::unichar,
        length: UInt,
        freeBuffer: bool,
    ) -> Self {
        unsafe {
            NSMutableString {
                ptr: msg_send![
                    self.ptr,
                    initWithCharactersNoCopy: characters
                    length: length
                    freeWhenDone: freeBuffer
                ],
            }
        }
    }

    fn im_initWithString<S>(self, s: S) -> Self
    where
        S: Into<NSString>,
    {
        unsafe {
            NSMutableString {
                ptr: msg_send![self.ptr, initWithString: s.into().ptr],
            }
        }
    }

    fn im_initWithCString_encoding(
        self,
        c_str: *const libc::c_char,
        encoding: string::Encoding,
    ) -> Self {
        unsafe {
            NSMutableString {
                ptr: msg_send![self.ptr, initWithCString: c_str encoding: encoding],
            }
        }
    }

    fn im_initWithUTF8String(self, c_str: *const libc::c_char) -> Self {
        unsafe {
            NSMutableString {
                ptr: msg_send![self.ptr, initWithUTF8String: c_str],
            }
        }
    }

    fn im_initWithData_encoding(self, data: super::NSData, encoding: string::Encoding) -> Self {
        unsafe {
            NSMutableString {
                ptr: msg_send![self.ptr, initWithData: data encoding: encoding],
            }
        }
    }

    fn tm_localizedUserNotificationStringForKey_arguments<T>(
        key: NSString,
        arguments: super::NSArray<T>,
    ) -> NSString
    where
        T: PNSObject,
    {
        unsafe {
            msg_send![
                class!(NSMutableString),
                localizedUserNotificationStringForKey: key
                arguments: arguments
            ]
        }
    }

    fn tm_stringWithCharacters_length(characters: *const super::unichar, length: UInt) -> Self {
        unsafe {
            NSMutableString {
                ptr: msg_send![class!(NSMutableString), stringWithCharacters: characters length: length],
            }
        }
    }

    fn tm_stringWithString(s: NSString) -> Self {
        unsafe {
            NSMutableString {
                ptr: msg_send![class!(NSMutableString), stringWithString: s.ptr],
            }
        }
    }

    fn tm_stringWithCString_encoding(
        c_str: *const libc::c_char,
        encoding: string::Encoding,
    ) -> Self {
        unsafe {
            NSMutableString {
                ptr: msg_send![class!(NSMutableString), stringWithCString: c_str encoding: encoding],
            }
        }
    }

    fn tm_stringWithUTF8String(c_str: *const libc::c_char) -> Self {
        unsafe {
            NSMutableString {
                ptr: msg_send![class!(NSMutableString), stringWithUTF8String: c_str],
            }
        }
    }

    fn ip_length(&self) -> Int {
        unsafe { msg_send![self.ptr, length] }
    }

    fn im_lengthOfBytesUsingEncoding(&self, enc: string::Encoding) -> Int {
        unsafe { msg_send![self.ptr, lengthOfBytesUsingEncoding: enc] }
    }

    fn im_maximumLengthOfBytesUsingEncoding(&self, enc: string::Encoding) -> Int {
        unsafe { msg_send![self.ptr, maximumLengthOfBytesUsingEncoding: enc] }
    }

    fn im_characterAtIndex(&self, index: Int) -> char {
        unsafe {
            let c: u8 = msg_send![self.ptr, characterAtIndex: index];
            c as char
        }
    }

    fn im_getCharacters_range(&self, buffer: *mut super::unichar, range: Range<UInt>) {
        unsafe { msg_send![self.ptr, getCharacters: buffer range: range] }
    }

    fn im_getBytes_maxLength_usedLength_encoding_options_range_remainingRange(
        &self,
        buffer: *mut libc::c_void,
        maxLength: Int,
        usedLength: *mut Int,
        encoding: string::Encoding,
        options: super::NSStringEncodingConversionOptions,
        range: Range<UInt>,
        remainingRange: Range<UInt>,
    ) -> bool {
        unsafe {
            msg_send![
                self.ptr,
                getBytes: buffer
                maxLength: maxLength
                usedLength: usedLength
                encoding: encoding
                options: options
                range: range
                remainingRange: remainingRange
            ]
        }
    }

    fn im_cStringUsingEncoding(&self, encoding: string::Encoding) -> *const libc::c_char {
        unsafe { msg_send![self.ptr, cStringUsingEncoding: encoding] }
    }

    fn im_getCString_maxLength_encoding(
        &self,
        buffer: *mut libc::c_char,
        maxLength: UInt,
        encoding: string::Encoding,
    ) -> bool {
        unsafe { msg_send![self.ptr, getCString: buffer maxLength: maxLength encoding: encoding] }
    }

    fn ip_UTF8String(&self) -> *const libc::c_char {
        unsafe { msg_send![self.ptr, UTF8String] }
    }

    fn im_caseInsensitiveCompare<S>(&self, string: S) -> NSComparisonResult
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, caseInsensitiveCompare: string.into().ptr] }
    }

    fn im_localizedCaseInsensitiveCompare<S>(&self, string: S) -> NSComparisonResult
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, localizedCaseInsensitiveCompare: string.into().ptr] }
    }

    fn im_compare<S>(&self, string: S) -> NSComparisonResult
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, compare: string.into().ptr] }
    }

    fn im_localizedCompare<S>(&self, string: S) -> NSComparisonResult
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, localizedCompare: string.into().ptr] }
    }

    fn im_compare_options<S>(
        &self,
        string: S,
        mask: super::NSStringCompareOptions,
    ) -> NSComparisonResult
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, compare: string.into() options: mask] }
    }

    fn im_compare_options_range<S>(
        &self,
        string: S,
        mask: super::NSStringCompareOptions,
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
        mask: super::NSStringCompareOptions,
        range: Range<UInt>,
        locale: super::NSLocale,
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
        unsafe { msg_send![self.ptr, hasPrefix: prefix.into()] }
    }

    fn im_hasSuffix<S>(&self, suffix: S) -> bool
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, hasSuffix: suffix.into()] }
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
        unsafe { msg_send![self.ptr, stringByAppendingString: string.into()] }
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
            msg_send![
                self.ptr,
                stringByPaddingToLength: new_length
                withString: pad_string.into()
                startingAtIndex: starting_at
            ]
        }
    }

    fn ip_lowercaseString(&self) -> NSString {
        unsafe { msg_send![self.ptr, lowercaseString] }
    }

    fn ip_localizedLowercaseString(&self) -> NSString {
        unsafe { msg_send![self.ptr, localizedLowercaseString] }
    }

    fn im_lowercaseStringWithLocale(&self, locale: super::NSLocale) -> NSString {
        unsafe { msg_send![self.ptr, lowercaseStringWithLocale: locale] }
    }

    fn ip_uppercaseString(&self) -> NSString {
        unsafe { msg_send![self.ptr, uppercaseString] }
    }

    fn ip_localizedUppercaseString(&self) -> NSString {
        unsafe { msg_send![self.ptr, localizedUppercaseString] }
    }

    fn im_uppercaseStringWithLocale(&self, locale: super::NSLocale) -> NSString {
        unsafe { msg_send![self.ptr, uppercaseStringWithLocale: locale] }
    }

    fn ip_capitalizedString(&self) -> NSString {
        unsafe { msg_send![self.ptr, capitalizedString] }
    }

    fn ip_localizedCapitalizedString(&self) -> NSString {
        unsafe { msg_send![self.ptr, localizedCapitalizedString] }
    }

    fn im_capitalizedStringWithLocale(&self, locale: super::NSLocale) -> NSString {
        unsafe { msg_send![self.ptr, capitalizedStringWithLocale: locale] }
    }

    fn im_componentsSeparatedByString<S>(&self, separator: S) -> super::NSArray<NSString>
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, componentsSeparatedByString: separator.into()] }
    }

    fn contains<S>(&self, other: S) -> bool
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, containsString: other.into()] }
    }

    fn stringByApplyingTransform(
        &mut self,
        transform: super::NSStringTransform,
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

    fn availableStringEncodings() -> *const string::Encoding {
        unsafe { msg_send![NSString::class(), availableStringEncodings] }
    }

    fn defaultCStringEncoding() -> string::Encoding {
        unsafe { msg_send![NSString::class(), defaultCStringEncoding] }
    }

    fn canBeConvertedToEncoding(&self, encoding: string::Encoding) -> bool {
        unsafe { msg_send![self.ptr, canBeConvertedToEncoding: encoding] }
    }

    fn dataUsingEncoding(&self, encoding: string::Encoding) -> super::NSData {
        unsafe { msg_send![self.ptr, dataUsingEncoding: encoding] }
    }
}

impl INSMutableString for NSMutableString {
    fn tm_stringWithCapacity(capacity: UInt) -> NSMutableString {
        unsafe { msg_send![NSMutableString::class(), stringWithCapacity: capacity] }
    }

    fn im_initWithCapacity(self, capacity: UInt) -> NSMutableString {
        unsafe { msg_send![self.ptr, initWithCapacity: capacity] }
    }

    fn im_appendString<S>(&mut self, string: S)
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, appendString: string.into()] }
    }

    fn im_applyTransform_reverse_range_updatedRange(
        &mut self,
        transform: super::NSStringTransform,
        reverse: bool,
        range: Range<UInt>,
        out_range: Range<UInt>,
    ) -> bool {
        unsafe {
            msg_send![
                self.ptr,
                applyTransform: transform
                reverse: reverse
                range: range
                updatedRange: out_range
            ]
        }
    }

    fn im_deleteCharactersInRange(&mut self, range: Range<UInt>) {
        unsafe { msg_send![self.ptr, deleteCharactersInRange: range] }
    }

    fn im_insertString_atIndex<S>(&mut self, string: S, loc: UInt)
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, insertString: string.into() atIndex: loc] }
    }

    fn im_replaceCharactersInRange_withString<S>(&mut self, range: Range<UInt>, string: S)
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, replaceCharactersInRange: range withString: string.into()] }
    }

    fn im_replaceOccurrencesOfString_withString_options_range<S>(
        &mut self,
        target: NSString,
        replacement: S,
        options: super::NSStringCompareOptions,
        searchRange: Range<UInt>,
    ) -> UInt
    where
        S: Into<NSString>,
    {
        unsafe {
            msg_send![
                self.ptr,
                replaceOccurrencesOfString: target
                withString: replacement.into()
                options: options
                range: searchRange
            ]
        }
    }

    fn im_setString<S>(&mut self, string: S)
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, setString: string.into()] }
    }
}

impl fmt::Debug for NSMutableString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description().as_str().unwrap())
    }
}

impl fmt::Display for NSMutableString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description().as_str().unwrap())
    }
}

impl Clone for NSMutableString {
    fn clone(&self) -> Self {
        unsafe { msg_send![self.ptr, retain] }
    }
}

impl PartialEq for NSMutableString {
    fn eq(&self, other: &NSMutableString) -> bool {
        self.im_localizedCompare(other.clone()) == NSComparisonResult::OrderedSame
    }
}

impl ToId for NSMutableString {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSMutableString {
    unsafe fn from_id(id: id) -> Self {
        Self {
            ptr: Id::from_ptr(id),
        }
    }
}

impl From<NSString> for NSMutableString {
    fn from(string: NSString) -> Self {
        unsafe {
            let ptr: id = msg_send![NSString::class(), alloc];
            let ptr: id = msg_send![ptr, initWithString: string];
            NSMutableString::from_id(ptr)
        }
    }
}
