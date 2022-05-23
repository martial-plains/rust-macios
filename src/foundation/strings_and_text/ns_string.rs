use std::{
    ffi::{CStr, CString},
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut, Range},
};

use libc::c_char;
use objc::{
    class, msg_send,
    runtime::{Object, BOOL, NO},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::{traits::NSString as t_NSString, ComparisonResult, Int, Locale, UInt},
    id,
    objective_c_runtime::traits::NSObject,
    utils::to_bool,
};

use super::{unichar, CompareOptions, Encoding, StringTransform, UTF8_ENCODING};

/// This is a mapping to the Objective-C NSString class.
#[repr(C)]
pub struct String {
    /// The raw pointer to the Objective-C object.
    pub objc: Id<Object>,
    marker: PhantomData<()>,
}

impl NSObject for String {
    fn init() -> Self {
        unsafe { msg_send![class!(NSString), new] }
    }

    fn to_id(mut self) -> id {
        &mut *self.objc
    }

    fn from_id(obj: id) -> Self {
        String {
            objc: unsafe { Id::from_ptr(obj) },
            marker: PhantomData,
        }
    }

    fn description(&self) -> String {
        unsafe {
            let description: id = msg_send![&*self.objc, description];
            String::from_id(description)
        }
    }

    fn debug_description(&self) -> String {
        unsafe {
            let description: id = msg_send![&*self.objc, debugDescription];
            String::from_id(description)
        }
    }

    fn retain(&self) -> Self {
        unsafe { msg_send![&*self.objc, retain] }
    }
}

impl t_NSString for String {
    fn new() -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(msg_send![nsstring, init])
        };

        String {
            objc,
            marker: PhantomData,
        }
    }

    unsafe fn from_retained(object: id) -> Self {
        String {
            objc: Id::from_retained_ptr(object),
            marker: PhantomData,
        }
    }

    unsafe fn is(obj: id) -> bool {
        let result: BOOL = msg_send![obj, isKindOfClass: class!(NSString)];
        to_bool(result)
    }

    fn bytes(&self) -> *const c_char {
        unsafe {
            let bytes: *const c_char = msg_send![&*self.objc, UTF8String];
            bytes
        }
    }

    fn bytes_len(&self) -> UInt {
        unsafe { msg_send![&*self.objc, lengthOfBytesUsingEncoding: UTF8_ENCODING] }
    }

    fn as_str(&self) -> &str {
        let bytes = self.bytes();

        unsafe {
            let bytes = CStr::from_ptr(bytes);
            bytes.to_str().unwrap()
        }
    }

    fn init() -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(msg_send![nsstring, init])
        };

        String {
            objc,
            marker: PhantomData,
        }
    }

    fn init_with_bytes_len_encoding(bytes: *const c_char, len: UInt, encoding: Encoding) -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(msg_send![nsstring, initWithBytes:bytes
                length:len
                encoding:encoding
            ])
        };

        String {
            objc,
            marker: PhantomData,
        }
    }

    fn init_with_no_cpy_str(s: &str) -> Self {
        String {
            objc: unsafe {
                let nsstring: id = msg_send![class!(NSString), alloc];
                Id::from_ptr(msg_send![nsstring, initWithBytesNoCopy:s.as_ptr()
                    length:s.len()
                    encoding:UTF8_ENCODING
                    freeWhenDone:NO
                ])
            },

            marker: PhantomData,
        }
    }

    fn init_with_characters_len(characters: *const unichar, len: UInt) -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(msg_send![nsstring, initWithCharacters:characters
                length:len
            ])
        };

        String {
            objc,
            marker: PhantomData,
        }
    }

    fn init_with_str(s: &str) -> Self {
        String {
            objc: unsafe {
                let nsstring: id = msg_send![class!(NSString), alloc];
                Id::from_ptr(msg_send![nsstring, initWithString:s.as_ptr()])
            },

            marker: PhantomData,
        }
    }

    fn length(&self) -> Int {
        unsafe { msg_send![&*self.objc, length] }
    }

    fn length_of_bytes(&self, enc: Encoding) -> Int {
        unsafe { msg_send![&*self.objc, lengthOfBytesUsingEncoding: enc] }
    }

    fn maximum_length_of_bytes(&self, enc: Encoding) -> Int {
        unsafe { msg_send![&*self.objc, maximumLengthOfBytesUsingEncoding: enc] }
    }

    fn character_at(&self, index: Int) -> char {
        unsafe { msg_send![&*self.objc, characterAtIndex: index] }
    }

    fn case_insensitive_compare<T>(&self, string: T) -> ComparisonResult
    where
        T: Into<String>,
    {
        unsafe { msg_send![&*self.objc, caseInsensitiveCompare: string.into()] }
    }

    fn localized_case_insensitive_compare<T>(&self, string: T) -> ComparisonResult
    where
        T: Into<String>,
    {
        unsafe { msg_send![&*self.objc, localizedCaseInsensitiveCompare: string.into()] }
    }

    fn compare<T>(&self, string: T) -> ComparisonResult
    where
        T: Into<String>,
    {
        unsafe { msg_send![&*self.objc, compare: string.into()] }
    }

    fn localized_compare<T>(&self, string: T) -> ComparisonResult
    where
        T: Into<String>,
    {
        unsafe { msg_send![&*self.objc, localizedCompare: string.into()] }
    }

    fn localized_standard_compare<T>(&self, string: T) -> ComparisonResult
    where
        T: Into<String>,
    {
        unsafe { msg_send![&*self.objc, localizedStandardCompare: string.into()] }
    }

    fn compare_with_options<T>(&self, string: T, options: CompareOptions) -> ComparisonResult
    where
        T: Into<String>,
    {
        unsafe { msg_send![&*self.objc, compare: string.into() options: options] }
    }

    fn compare_with_options_range<T>(
        &self,
        string: T,
        options: CompareOptions,
        range: Range<UInt>,
    ) -> ComparisonResult
    where
        T: Into<String>,
    {
        unsafe { msg_send![&*self.objc, compare: string.into() options: options range: range] }
    }

    fn compare_with_options_range_locale<T>(
        &self,
        string: T,
        options: CompareOptions,
        range: Range<UInt>,
        locale: Locale,
    ) -> ComparisonResult
    where
        T: Into<String>,
    {
        unsafe {
            msg_send![&*self.objc, compare: string.into() options: options range: range locale: locale]
        }
    }

    fn has_prefix<T>(&self, prefix: T) -> bool
    where
        T: Into<String>,
    {
        unsafe {
            let result: BOOL = msg_send![&*self.objc, hasPrefix: prefix.into()];
            to_bool(result)
        }
    }

    fn has_suffix<T>(&self, suffix: T) -> bool
    where
        T: Into<String>,
    {
        unsafe {
            let result: BOOL = msg_send![&*self.objc, hasSuffix: suffix.into()];
            to_bool(result)
        }
    }

    fn is_equal_to<T>(&self, string: T) -> bool
    where
        T: Into<String>,
    {
        unsafe {
            let result: BOOL = msg_send![&*self.objc, isEqualToString: string.into()];
            to_bool(result)
        }
    }

    fn appending<T>(&self, string: T) -> String
    where
        T: Into<String>,
    {
        unsafe { msg_send![&*self.objc, stringByAppendingString: string.into()] }
    }

    fn padding<T>(&self, new_length: Int, pad_string: T, starting_at: Int) -> String
    where
        T: Into<String>,
    {
        unsafe {
            msg_send![&*self.objc, stringByPaddingToLength: new_length withString: pad_string.into() startingAtIndex: starting_at]
        }
    }

    fn contains<T>(&self, other: T) -> bool
    where
        T: Into<String>,
    {
        unsafe {
            let result: BOOL = msg_send![&*self.objc, containsString: other.into()];
            to_bool(result)
        }
    }

    fn lowercased(&self) -> String {
        unsafe { msg_send![&*self.objc, lowercaseString] }
    }

    fn localized_lowercase(&self) -> String {
        unsafe { msg_send![&*self.objc, localizedLowercaseString] }
    }

    fn uppercased(&self) -> String {
        unsafe { msg_send![&*self.objc, uppercaseString] }
    }

    fn localized_uppercase(&self) -> String {
        unsafe { msg_send![&*self.objc, localizedUppercaseString] }
    }

    fn capitalized(&self) -> String {
        unsafe { msg_send![&*self.objc, capitalizedString] }
    }

    fn localized_capitalized(&self) -> String {
        unsafe { msg_send![&*self.objc, localizedCapitalizedString] }
    }

    fn applying_transform(&mut self, transform: StringTransform, reverse: bool) -> Option<String> {
        let result: id = unsafe {
            msg_send![
            &*self.objc,
            stringByApplyingTransform: transform
            reverse: reverse
            ]
        };
        if result.is_null() {
            None
        } else {
            Some(String::from_id(result))
        }
    }
}

impl Default for String {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description().as_str())
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.debug_description().as_str())
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        unsafe { msg_send![&*self.objc, retain] }
    }
}

impl Deref for String {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.objc
    }
}

impl DerefMut for String {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.objc
    }
}

impl From<std::string::String> for String {
    /// Creates a new `NSString` from a `String`.
    fn from(s: std::string::String) -> Self {
        String {
            objc: unsafe {
                let nsstring: id = msg_send![class!(NSString), alloc];
                Id::from_ptr(msg_send![nsstring, initWithString:s.as_str()])
            },

            marker: PhantomData,
        }
    }
}

impl From<&str> for String {
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

        String {
            objc,
            marker: PhantomData,
        }
    }
}

impl From<char> for String {
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

        String {
            objc,
            marker: PhantomData,
        }
    }
}

impl From<String> for &str {
    /// Converts a `NSString` to a `&str`.
    fn from(string: String) -> Self {
        unsafe {
            let ptr: *const c_char = msg_send![string.objc, UTF8String];
            CStr::from_ptr(ptr).to_str().unwrap()
        }
    }
}

impl From<(&str, Encoding)> for String {
    /// Creates a new `NSString` from a `&str` and an encoding.
    fn from((s, encoding): (&str, Encoding)) -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(msg_send![nsstring, initWithBytes:s.as_ptr()
                length:s.len()
                encoding:encoding
            ])
        };

        String {
            objc,
            marker: PhantomData,
        }
    }
}

impl PartialEq for String {
    /// Checks if two `NSString`s are equal.
    fn eq(&self, other: &Self) -> bool {
        self.localized_compare(other.clone()) == ComparisonResult::OrderedSame
    }
}

impl PartialEq<&str> for String {
    /// Checks if a `NSString` is equal to a `&str`.
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

#[cfg(test)]
mod tests {
    use crate::foundation::{string_transforms::LatinToKatakana, ComparisonResult};

    use super::*;

    #[test]
    fn test_from_str() {
        let s = String::from("Hello, World!");
        assert_eq!(s.as_str(), "Hello, World!");
    }

    #[test]
    fn test_from_no_cpy_str() {
        let s = String::init_with_no_cpy_str("Hello, World!");
        assert_eq!(s.as_str(), "Hello, World!");
    }

    #[test]
    fn test_bytes() {
        let s = String::from("Hello, World!");
        let other = s.bytes();
        assert_eq!(s.bytes(), other);
    }

    #[test]
    fn test_bytes_len() {
        let s = String::from("Hello, World!");
        assert_eq!(s.bytes_len(), 13);
    }

    #[test]
    fn test_as_str() {
        let s = String::from("Hello, World!");
        assert_eq!(s.as_str(), "Hello, World!");
    }

    #[test]
    fn test_to_string() {
        let s = String::from("Hello, World!");
        assert_eq!(s.to_string(), "Hello, World!".to_string());
    }

    #[test]
    fn test_length() {
        let s = String::from("Hello, World!");
        assert_eq!(s.length(), 13);
    }

    #[test]
    fn test_contains() {
        let s = String::from("Hello, World!");
        assert!(s.contains("Hello"));
        assert!(!s.contains("Goodbye"));
    }

    #[test]
    fn test_character_at() {
        let s = String::from("Hello, World!");
        assert_eq!(s.character_at(0), 'H');
        assert_eq!(s.character_at(1), 'e');
        assert_eq!(s.character_at(2), 'l');
        assert_eq!(s.character_at(3), 'l');
        assert_eq!(s.character_at(4), 'o');
        assert_eq!(s.character_at(5), ',');
        assert_eq!(s.character_at(6), ' ');
        assert_eq!(s.character_at(7), 'W');
        assert_eq!(s.character_at(8), 'o');
        assert_eq!(s.character_at(9), 'r');
        assert_eq!(s.character_at(10), 'l');
        assert_eq!(s.character_at(11), 'd');
        assert_eq!(s.character_at(12), '!');
    }

    #[test]
    fn test_has_prefix() {
        let s = String::from("Hello, World!");
        assert!(s.has_prefix("Hello"));
        assert!(!s.has_prefix("Goodbye"));
    }

    #[test]
    fn test_has_suffix() {
        let s = String::from("Hello, World!");
        assert!(s.has_suffix("World!"));
        assert!(!s.has_suffix("Goodbye"));
    }

    #[test]
    fn test_is_equal_to() {
        let s = String::from("Hello, World!");
        assert!(s.is_equal_to("Hello, World!"));
        assert!(!s.is_equal_to("Goodbye, World!"));
    }

    #[test]
    fn test_length_of_bytes() {
        let s = String::from("Hello, World!");
        assert_eq!(s.length_of_bytes(Encoding::UTF8), 13);
    }

    #[test]
    fn test_maximum_length_of_bytes() {
        let s = String::from("Hello, World!");
        assert_eq!(s.maximum_length_of_bytes(Encoding::UTF8), 39);
    }

    #[test]
    fn test_case_insensitive_compare() {
        let s = String::from("Hello, World!");
        assert_eq!(
            s.case_insensitive_compare("hello, world!"),
            ComparisonResult::OrderedSame
        );
        assert_eq!(
            s.case_insensitive_compare("goodbye, world!"),
            ComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_localized_case_insensitive_compare() {
        let s = String::from("Hello, World!");
        assert_eq!(
            s.localized_case_insensitive_compare("hello, world!"),
            ComparisonResult::OrderedSame
        );
        assert_eq!(
            s.localized_case_insensitive_compare("goodbye, world!"),
            ComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_compare() {
        let s = String::from("Hello, World!");
        assert_eq!(s.compare("Hello, World!"), ComparisonResult::OrderedSame);
        assert_eq!(
            s.compare("Goodbye, World!"),
            ComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_localized_standard_compare() {
        let s = String::from("Hello, World!");
        assert_eq!(
            s.localized_standard_compare("Hello, World!"),
            ComparisonResult::OrderedSame
        );
        assert_eq!(
            s.localized_standard_compare("Goodbye, World!"),
            ComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_applying_transform() {
        let mut s = String::from("Katakana!");
        let transform = unsafe { LatinToKatakana };
        assert_eq!(s.applying_transform(transform, false).unwrap(), "カタカナ!");
    }
}
