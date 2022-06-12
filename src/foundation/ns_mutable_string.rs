use std::{fmt, ops::Range};

use libc::c_char;
use objc::{
    class, msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject, ToId},
    },
    utils::to_bool,
};

use super::{
    string::Encoding,
    traits::{INSMutableString, INSString},
    unichar, Int, NSArray, NSComparisonResult, NSData, NSString, UInt,
};

/// A mutable string.
#[repr(C)]
pub struct NSMutableString {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSMutableString {
    fn im_class<'a>() -> &'a Class {
        class!(NSMutableString)
    }

    fn im_is_equal(&self, object: &Self) -> bool {
        unsafe { msg_send![&*self.ptr, isEqual: object] }
    }

    fn ip_hash(&self) -> UInt {
        unsafe { msg_send![&*self.ptr, hash] }
    }

    fn im_is_kind_of_class(&self, class: Class) -> bool {
        unsafe { msg_send![&*self.ptr, isKindOfClass: class] }
    }

    fn im_is_member_of_class(&self, class: Class) -> bool {
        unsafe { msg_send![&*self.ptr, isMemberOfClass: class] }
    }

    fn im_responds_to_selector(&self, selector: Sel) -> bool {
        unsafe { msg_send![&*self.ptr, respondsToSelector: selector] }
    }

    fn im_conforms_to_protocol(&self, protocol: Protocol) -> bool {
        unsafe { msg_send![&*self.ptr, conformsToProtocol: protocol] }
    }

    fn ip_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![&*self.ptr, description]) }
    }

    fn ip_debug_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![&*self.ptr, debugDescription]) }
    }

    fn im_perform_selector(&self, selector: Sel) -> id {
        unsafe { msg_send![&*self.ptr, performSelector: selector] }
    }

    fn im_perform_selector_with_object(&self, selector: Sel, with_object: id) -> id {
        unsafe { msg_send![&*self.ptr, performSelector: selector withObject: with_object] }
    }

    fn im_is_proxy(&self) -> bool {
        unsafe { msg_send![&*self.ptr, isProxy] }
    }
}

impl INSString for NSMutableString {
    fn im_init(self) -> Self {
        unsafe { Self::from_id(msg_send![&*self.ptr, init]) }
    }

    fn im_init_with_bytes_length_encoding(
        self,
        bytes: *const libc::c_void,
        len: UInt,
        encoding: Encoding,
    ) -> Self {
        unsafe {
            Self::from_id(
                msg_send![&*self.ptr, initWithBytes: bytes length: len encoding: encoding],
            )
        }
    }

    fn im_init_with_bytes_no_copy_length_encoding_free_when_done(
        self,
        bytes: *mut libc::c_void,
        len: UInt,
        encoding: Encoding,
        free_buffer: bool,
    ) -> Self {
        unsafe {
            Self::from_id(msg_send![
                class!(NSMutableString),
                stringWithBytesNoCopy: bytes
                length: len
                encoding: encoding
                freeWhenDone: free_buffer
            ])
        }
    }

    fn im_init_with_characters_length(self, characters: *const unichar, len: UInt) -> Self {
        unsafe {
            Self::from_id(
                msg_send![class!(NSMutableString), stringWithCharacters: characters length: len],
            )
        }
    }

    fn im_init_with_characters_no_copy_length_free_when_done(
        self,
        characters: unichar,
        length: UInt,
        free_buffer: bool,
    ) -> Self {
        unsafe {
            Self::from_id(msg_send![
                self.ptr,
                initWithCharactersNoCopy: characters
                length: length
                freeWhenDone: free_buffer
            ])
        }
    }

    fn im_init_with_string<S>(self, s: S) -> Self
    where
        S: Into<NSString>,
    {
        unsafe { Self::from_id(msg_send![&*self.ptr, initWithString: s.into().ptr]) }
    }

    fn im_init_with_cstring_encoding(self, c_str: *const c_char, encoding: Encoding) -> Self {
        unsafe { Self::from_id(msg_send![&*self.ptr, initWithCString: c_str encoding: encoding]) }
    }

    fn im_init_with_utf8_string(self, c_str: *const c_char) -> Self {
        unsafe { Self::from_id(msg_send![&*self.ptr, initWithUTF8String: c_str]) }
    }

    fn im_init_with_data_encoding(self, data: NSData, encoding: Encoding) -> Self {
        unsafe { Self::from_id(msg_send![&*self.ptr, initWithData: data encoding: encoding]) }
    }

    fn ip_length(&self) -> UInt {
        unsafe { msg_send![&*self.ptr, length] }
    }

    fn im_length_of_bytes_using_encoding(&self, enc: Encoding) -> UInt {
        unsafe { msg_send![&*self.ptr, lengthOfBytesUsingEncoding: enc] }
    }

    fn im_maximum_length_of_bytes_using_encoding(&self, enc: Encoding) -> Int {
        unsafe { msg_send![&*self.ptr, maximumLengthOfBytesUsingEncoding: enc] }
    }

    fn im_character_at_index(&self, index: UInt) -> char {
        unsafe {
            let c: u8 = msg_send![&*self.ptr, characterAtIndex: index];
            c as char
        }
    }

    fn im_get_characters_range(&self, buffer: *mut unichar, range: Range<UInt>) {
        unsafe { msg_send![&*self.ptr, getCharacters: buffer range: range] }
    }

    fn im_get_bytes_max_length_used_length_encoding_options_range_remaining_range(
        &self,
        buffer: *mut libc::c_void,
        max_length: Int,
        used_length: *mut Int,
        encoding: Encoding,
        options: super::NSStringEncodingConversionOptions,
        range: Range<UInt>,
        remaining_range: Range<UInt>,
    ) -> bool {
        unsafe {
            to_bool(msg_send![
                self.ptr,
                getBytes: buffer
                maxLength: max_length
                usedLength: used_length
                encoding: encoding
                options: options
                range: range
                remainingRange: remaining_range
            ])
        }
    }

    fn im_c_string_using_encoding(&self, encoding: Encoding) -> *const c_char {
        unsafe { msg_send![&*self.ptr, cStringUsingEncoding: encoding] }
    }

    fn im_get_cstring_max_length_encoding(
        &self,
        buffer: *mut c_char,
        max_length: UInt,
        encoding: Encoding,
    ) -> bool {
        unsafe {
            to_bool(
                msg_send![&*self.ptr, getCString: buffer maxLength: max_length encoding: encoding],
            )
        }
    }

    fn ip_utf8_string(&self) -> *const c_char {
        unsafe { msg_send![&*self.ptr, UTF8String] }
    }

    fn im_case_insensitive_compare<S>(&self, string: S) -> NSComparisonResult
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![&*self.ptr, caseInsensitiveCompare: string.into()] }
    }

    fn im_localized_case_insensitive_compare<S>(&self, string: S) -> NSComparisonResult
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![&*self.ptr, localizedCaseInsensitiveCompare: string.into()] }
    }

    fn im_compare<S>(&self, string: S) -> NSComparisonResult
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![&*self.ptr, compare: string.into()] }
    }

    fn im_localized_compare<S>(&self, string: S) -> NSComparisonResult
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![&*self.ptr, localizedCompare: string.into()] }
    }

    fn im_compare_options<S>(
        &self,
        string: S,
        mask: super::NSStringCompareOptions,
    ) -> NSComparisonResult
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![&*self.ptr, compare: string.into() options: mask] }
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
        unsafe { msg_send![&*self.ptr, compare: string.into() options: mask range: range] }
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
            msg_send![&*self.ptr, compare: string.into() options: mask range: range locale: locale]
        }
    }

    fn im_localized_standard_compare<S>(&self, string: S) -> NSComparisonResult
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![&*self.ptr, localizedStandardCompare: string.into()] }
    }

    fn im_has_prefix<S>(&self, prefix: S) -> bool
    where
        S: Into<NSString>,
    {
        unsafe { to_bool(msg_send![&*self.ptr, hasPrefix: prefix.into()]) }
    }

    fn im_has_suffix<S>(&self, suffix: S) -> bool
    where
        S: Into<NSString>,
    {
        unsafe { to_bool(msg_send![&*self.ptr, hasSuffix: suffix.into()]) }
    }

    fn im_is_equal_to_string<S>(&self, string: S) -> bool
    where
        S: Into<NSString>,
    {
        unsafe { to_bool(msg_send![&*self.ptr, isEqualToString: string.into()]) }
    }

    fn im_string_by_appending_string<S>(&self, string: S) -> NSString
    where
        S: Into<NSString>,
    {
        unsafe { NSString::from_id(msg_send![&*self.ptr, stringByAppendingString: string.into()]) }
    }

    fn im_string_by_padding_to_length_with_string_starting_at_index<S>(
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

    fn ip_lowercase_string(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![&*self.ptr, lowercaseString]) }
    }

    fn ip_localized_lowercase_string(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![&*self.ptr, localizedLowercaseString]) }
    }

    fn im_lowercase_string_with_locale(&self, locale: super::NSLocale) -> NSString {
        unsafe { NSString::from_id(msg_send![&*self.ptr, lowercaseStringWithLocale: locale]) }
    }

    fn ip_uppercase_string(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![&*self.ptr, uppercaseString]) }
    }

    fn ip_localized_uppercase_string(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![&*self.ptr, localizedUppercaseString]) }
    }

    fn im_uppercase_string_with_locale(&self, locale: super::NSLocale) -> NSString {
        unsafe { NSString::from_id(msg_send![&*self.ptr, uppercaseStringWithLocale: locale]) }
    }

    fn ip_capitalized_string(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![&*self.ptr, capitalizedString]) }
    }

    fn ip_localized_capitalized_string(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![&*self.ptr, localizedCapitalizedString]) }
    }

    fn im_capitalized_string_with_locale(&self, locale: super::NSLocale) -> NSString {
        unsafe { NSString::from_id(msg_send![&*self.ptr, capitalizedStringWithLocale: locale]) }
    }

    fn im_components_separated_by_string<S>(&self, separator: S) -> NSArray<NSString>
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![&*self.ptr, componentsSeparatedByString: separator.into()] }
    }

    fn im_contains_string<S>(&self, other: S) -> bool
    where
        S: Into<NSString>,
    {
        unsafe { to_bool(msg_send![&*self.ptr, containsString: other.into()]) }
    }

    fn im_string_by_applying_transform_reverse(
        &mut self,
        transform: super::NSStringTransform,
        reverse: bool,
    ) -> Option<NSString> {
        unsafe {
            let result: id =
                msg_send![&*self.ptr, stringByApplyingTransform: transform reverse: reverse];
            if result.is_null() {
                None
            } else {
                Some(NSString::from_id(result))
            }
        }
    }

    fn im_can_be_converted_to_encoding(&self, encoding: Encoding) -> bool {
        unsafe { msg_send![&*self.ptr, canBeConvertedToEncoding: encoding] }
    }

    fn im_data_using_encoding(&self, encoding: Encoding) -> NSData {
        unsafe { NSData::from_id(msg_send![&*self.ptr, dataUsingEncoding: encoding]) }
    }
}

impl INSMutableString for NSMutableString {
    fn im_init_with_capacity(self, capacity: UInt) -> NSMutableString {
        unsafe { Self::from_id(msg_send![&*self.ptr, initWithCapacity: capacity]) }
    }

    fn im_append_string<S>(&mut self, string: S)
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![&*self.ptr, appendString: string.into()] }
    }

    fn im_apply_transform_reverse_range_updated_range(
        &mut self,
        transform: super::NSStringTransform,
        reverse: bool,
        range: Range<UInt>,
        out_range: Range<UInt>,
    ) -> bool {
        unsafe {
            to_bool(msg_send![
                self.ptr,
                applyTransform: transform
                reverse: reverse
                range: range
                updatedRange: out_range
            ])
        }
    }

    fn im_delete_characters_in_range(&mut self, range: Range<UInt>) {
        unsafe { msg_send![&*self.ptr, deleteCharactersInRange: range] }
    }

    fn im_insert_string_at_index<S>(&mut self, string: S, loc: UInt)
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![&*self.ptr, insertString: string.into() atIndex: loc] }
    }

    fn im_replace_characters_in_range_with_string<S>(&mut self, range: Range<UInt>, string: S)
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![&*self.ptr, replaceCharactersInRange: range withString: string.into()] }
    }

    fn im_replace_occurrences_of_string_with_string_options_range<S>(
        &mut self,
        target: NSString,
        replacement: S,
        options: super::NSStringCompareOptions,
        search_range: Range<UInt>,
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
                range: search_range
            ]
        }
    }

    fn im_set_string<S>(&mut self, string: S)
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![&*self.ptr, setString: string.into()] }
    }
}

impl Default for NSMutableString {
    fn default() -> Self {
        Self::tm_string()
    }
}

impl fmt::Debug for NSMutableString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ip_description().as_str().unwrap())
    }
}

impl fmt::Display for NSMutableString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ip_description().as_str().unwrap())
    }
}

impl Clone for NSMutableString {
    fn clone(&self) -> Self {
        unsafe { msg_send![&*self.ptr, retain] }
    }
}

impl PartialEq for NSMutableString {
    fn eq(&self, other: &NSMutableString) -> bool {
        self.im_localized_compare(other.clone()) == NSComparisonResult::OrderedSame
    }
}

impl PartialEq<&str> for NSMutableString {
    fn eq(&self, other: &&str) -> bool {
        self.im_localized_compare(NSString::from(*other)) == NSComparisonResult::OrderedSame
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
            let ptr: id = msg_send![NSString::im_class(), alloc];
            let ptr: id = msg_send![ptr, initWithString: string];
            NSMutableString::from_id(ptr)
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::foundation::{string::Encoding, LatinToKatakana, NSStringCompareOptions};

    use super::*;

    #[test]
    fn test_tm_string_with_capacity() {
        let string = NSMutableString::tm_string_with_capacity(10);
        assert_eq!(string.im_length_of_bytes_using_encoding(Encoding::UTF8), 0);
    }

    #[test]
    fn test_im_append_string() {
        let mut string = NSMutableString::tm_string_with_capacity(10);
        string.im_append_string("Hello");
        assert_eq!(string.im_length_of_bytes_using_encoding(Encoding::UTF8), 5);
    }

    #[test]
    fn test_im_apply_transform_reverse_range_updated_range() {
        let mut string = NSMutableString::tm_string_with_capacity(10);
        string.im_append_string("Hello");
        assert_eq!(string.im_length_of_bytes_using_encoding(Encoding::UTF8), 5);
        assert!(string.im_apply_transform_reverse_range_updated_range(
            unsafe { LatinToKatakana },
            false,
            0..5,
            0..5
        ));
    }

    #[test]
    fn test_im_delete_characters_in_range() {
        let mut string = NSMutableString::tm_string_with_capacity(10);
        string.im_append_string("Hello");
        assert_eq!(string.im_length_of_bytes_using_encoding(Encoding::UTF8), 5);
        string.im_delete_characters_in_range(0..5);
        assert_eq!(string.im_length_of_bytes_using_encoding(Encoding::UTF8), 0);
    }

    #[test]
    fn test_im_insert_string_at_index() {
        let mut string = NSMutableString::tm_string_with_capacity(10);
        string.im_append_string("Hello");
        assert_eq!(string.im_length_of_bytes_using_encoding(Encoding::UTF8), 5);
        string.im_insert_string_at_index("World", 0);
        assert_eq!(string.im_length_of_bytes_using_encoding(Encoding::UTF8), 10);
    }

    #[test]
    fn test_im_replace_characters_in_range_with_string() {
        let mut string = NSMutableString::tm_string_with_capacity(10);
        string.im_append_string("Hello");
        assert_eq!(string.im_length_of_bytes_using_encoding(Encoding::UTF8), 5);
        string.im_replace_characters_in_range_with_string(0..5, "World");
        assert_eq!(string.im_length_of_bytes_using_encoding(Encoding::UTF8), 5);
    }

    #[test]
    fn test_im_replace_occurrences_of_string_with_string_options_range() {
        let mut string = NSMutableString::tm_string_with_capacity(10);
        string.im_append_string("Hello");
        assert_eq!(string.im_length_of_bytes_using_encoding(Encoding::UTF8), 5);
        assert_eq!(
            string.im_replace_occurrences_of_string_with_string_options_range(
                "Hello".into(),
                "World",
                NSStringCompareOptions::CaseInsensitive,
                0..5
            ),
            1
        );
    }

    #[test]
    fn test_im_set_string() {
        let mut string = NSMutableString::tm_string_with_capacity(10);
        string.im_append_string("Hello");
        assert_eq!(string.im_length_of_bytes_using_encoding(Encoding::UTF8), 5);
        string.im_set_string("World");
        assert_eq!(string.im_length_of_bytes_using_encoding(Encoding::UTF8), 5);
    }
}
