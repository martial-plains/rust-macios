use std::{
    ffi::{CStr, CString},
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    string::String,
};

use libc::c_char;
use objc::{
    class, msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::{traits::INSString, NSComparisonResult},
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject, ToId},
    },
};

use super::{string::Encoding, NSMutableString, UTF8_ENCODING};

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

    /// Returns a boolean value indicating whether the string contains a given string by performing a case-sensitive, locale-unaware search.
    pub fn contains<S>(&self, string: S) -> bool
    where
        S: Into<NSString>,
    {
        self.im_contains_string(string.into())
    }

    /// Returns a Boolean value that indicates whether a given string matches the beginning characters of the receiver.
    pub fn has_prefix<S>(&self, prefix: S) -> bool
    where
        S: Into<NSString>,
    {
        self.im_has_prefix(prefix.into())
    }

    /// Returns a Boolean value that indicates whether a given string matches the ending characters of the receiver.
    pub fn has_suffix<S>(&self, suffix: S) -> bool
    where
        S: Into<NSString>,
    {
        self.im_has_suffix(suffix.into())
    }

    /// Returns a Boolean value that indicates whether a given string is equal to the receiver using a literal Unicode-based comparison.
    pub fn is_equal_to<S>(&self, string: S) -> bool
    where
        S: Into<NSString>,
    {
        self.im_is_equal_to_string(string.into())
    }

    /// Returns the result of invoking compare:options:range: with no options and the receiver’s full extent as the range.

    pub fn compare<S>(&self, string: S) -> NSComparisonResult
    where
        S: Into<NSString>,
    {
        self.im_compare(string.into())
    }

    /// Compares strings as sorted by the Finder.
    pub fn localized_standard_compare<S>(&self, string: S) -> NSComparisonResult
    where
        S: Into<NSString>,
    {
        self.im_localized_standard_compare(string.into())
    }
}

impl PNSObject for NSString {
    fn im_class<'a>() -> &'a Class {
        class!(NSString)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl INSString for NSString {}

impl Default for NSString {
    fn default() -> Self {
        Self::tm_string()
    }
}

impl fmt::Debug for NSString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description().as_str().unwrap())
    }
}

impl fmt::Display for NSString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip_description().as_str().unwrap())
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
            let ptr: id = msg_send![NSString::im_class(), alloc];
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

impl From<&&str> for NSString {
    /// Creates a new `NSString` from a `&str`.
    fn from(s: &&str) -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(
                msg_send![nsstring, initWithBytes: CString::new(*s).unwrap().into_raw() as *mut Object
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
        self.im_localized_compare(other.clone()) == NSComparisonResult::OrderedSame
    }
}

impl Eq for NSString {}

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
        assert_eq!(s.im_length_of_bytes_using_encoding(Encoding::UTF8), 13);
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
        assert_eq!(s.im_character_at_index(0), 'H');
        assert_eq!(s.im_character_at_index(1), 'e');
        assert_eq!(s.im_character_at_index(2), 'l');
        assert_eq!(s.im_character_at_index(3), 'l');
        assert_eq!(s.im_character_at_index(4), 'o');
        assert_eq!(s.im_character_at_index(5), ',');
        assert_eq!(s.im_character_at_index(6), ' ');
        assert_eq!(s.im_character_at_index(7), 'W');
        assert_eq!(s.im_character_at_index(8), 'o');
        assert_eq!(s.im_character_at_index(9), 'r');
        assert_eq!(s.im_character_at_index(10), 'l');
        assert_eq!(s.im_character_at_index(11), 'd');
        assert_eq!(s.im_character_at_index(12), '!');
    }

    #[test]
    fn test_has_prefix() {
        let s = NSString::from("Hello, World!");
        assert!(s.has_prefix("Hello"));
        assert!(!s.has_prefix("Goodbye"));
    }

    #[test]
    fn test_has_suffix() {
        let s = NSString::from("Hello, World!");
        assert!(s.has_suffix("World!"));
        assert!(!s.has_suffix("Goodbye"));
    }

    #[test]
    fn test_is_equal_to() {
        let s = NSString::from("Hello, World!");
        assert!(s.is_equal_to("Hello, World!"));
        assert!(!s.is_equal_to("Goodbye, World!"));
    }

    #[test]
    fn test_length_of_bytes() {
        let s = NSString::from("Hello, World!");
        assert_eq!(s.im_length_of_bytes_using_encoding(Encoding::UTF8), 13);
    }

    #[test]
    fn test_maximum_length_of_bytes() {
        let s = NSString::from("Hello, World!");
        assert_eq!(
            s.im_maximum_length_of_bytes_using_encoding(Encoding::UTF8),
            39
        );
    }

    #[test]
    fn test_case_insensitive_compare() {
        let s = NSString::from("Hello, World!");
        assert_eq!(
            s.im_case_insensitive_compare(NSString::from("hello, world!")),
            NSComparisonResult::OrderedSame
        );
        assert_eq!(
            s.im_case_insensitive_compare(NSString::from("goodbye, world!")),
            NSComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_compare() {
        let s = NSString::from("Hello, World!");
        assert_eq!(s.compare("Hello, World!"), NSComparisonResult::OrderedSame);
        assert_eq!(
            s.compare("Goodbye, World!"),
            NSComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_localized_standard_compare() {
        let s = NSString::from("Hello, World!");
        assert_eq!(
            s.localized_standard_compare("Hello, World!"),
            NSComparisonResult::OrderedSame
        );
        assert_eq!(
            s.localized_standard_compare("Goodbye, World!"),
            NSComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_applying_transform() {
        let mut s = NSString::from("Katakana!");
        let transform = unsafe { LatinToKatakana };
        assert_eq!(
            s.im_string_by_applying_transform_reverse(transform, false)
                .unwrap(),
            "カタカナ!"
        );
    }

    #[test]
    fn test_im_case_insensitive_compare() {
        let s = NSString::from("Hello, World!");
        assert_eq!(
            s.im_case_insensitive_compare(NSString::from("hello, world!")),
            NSComparisonResult::OrderedSame
        );
        assert_eq!(
            s.im_case_insensitive_compare(NSString::from("goodbye, world!")),
            NSComparisonResult::OrderedDescending
        );
    }

    #[test]
    fn test_string_by_padding_to_length_with_string_starting_at_index() {
        let s = NSString::from("Hello, World!");

        assert_eq!(
            s.im_string_by_padding_to_length_with_string_starting_at_index(
                20,
                NSString::from("."),
                0
            ),
            "Hello, World!......."
        );
    }
}
