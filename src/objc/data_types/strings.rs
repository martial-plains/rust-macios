use std::{
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    slice,
    str::FromStr,
};

use libc::c_char;
use objc::{
    class, msg_send,
    runtime::{Object, BOOL, NO},
    sel, sel_impl,
};
use objc_id::Id;

use crate::foundation::Encoding;

use super::{id, to_bool, NSUInteger};

/// Size of UTF8 encoding
const UTF8_ENCODING: usize = 4;

/// This is a mapping to the Objective-C NSString class.
#[derive(Debug)]
#[repr(C)]
pub struct NSString<'a> {
    /// The raw pointer to the Objective-C object.
    pub objc: Id<Object>,
    marker: PhantomData<&'a ()>,
}

impl<'a> FromStr for NSString<'a> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(msg_send![nsstring, initWithBytes:s.as_ptr()
                length:s.len()
                encoding:UTF8_ENCODING
            ])
        };

        Ok(NSString {
            objc,
            marker: PhantomData,
        })
    }
}

impl<'a> NSString<'a> {
    /// Creates a new `NSString`
    pub fn new() -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(msg_send![nsstring, init])
        };

        NSString {
            objc,
            marker: PhantomData,
        }
    }

    /// Creates a new `NSString` from a string slice. without copying the bytes.
    ///
    /// # Arguments
    ///
    /// * `s` - The string slice to create the `NSString` from.
    pub fn from_no_cpy_str(s: &'a str) -> Self {
        NSString {
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

    // In cases where we're vended an `NSString` by the system, this can be used to wrap and
    /// retain it.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it dereferrences a raw pointer.
    pub unsafe fn retain(object: id) -> Self {
        NSString {
            objc: Id::from_ptr(object),
            marker: PhantomData,
        }
    }

    /// In some cases, we want to wrap a system-provided NSString without retaining it.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it dereferrences a raw pointer.
    pub unsafe fn from_retained(object: id) -> Self {
        NSString {
            objc: Id::from_retained_ptr(object),
            marker: PhantomData,
        }
    }

    /// Utility method for checking whether an `NSObject` is an `NSString`.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it dereferrences a raw pointer.
    pub unsafe fn is(obj: id) -> bool {
        let result: BOOL = msg_send![obj, isKindOfClass: class!(NSString)];
        to_bool(result)
    }

    /// Returns the UTF8 bytes for this `NSString`.
    fn bytes(&self) -> *const u8 {
        unsafe {
            let bytes: *const c_char = msg_send![&*self.objc, UTF8String];
            bytes as *const u8
        }
    }

    /// Gets the proper byte length for this `NSString` (the UTF8 variant).
    fn bytes_len(&self) -> NSUInteger {
        unsafe { msg_send![&*self.objc, lengthOfBytesUsingEncoding: UTF8_ENCODING] }
    }

    /// Convert this `NSString` into a `&str`.
    pub fn as_str(&self) -> &str {
        let bytes = self.bytes();
        let len = self.bytes_len();

        unsafe {
            let bytes = slice::from_raw_parts(bytes, len as usize);
            std::str::from_utf8(bytes).unwrap()
        }
    }
}

impl<'a> Default for NSString<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for NSString<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<NSString<'_>> for id {
    /// Consumes and returns the pointer to the underlying NSString instance.
    fn from(mut string: NSString) -> Self {
        &mut *string.objc
    }
}

impl Deref for NSString<'_> {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.objc
    }
}

impl DerefMut for NSString<'_> {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.objc
    }
}

impl From<String> for NSString<'_> {
    /// Creates a new `NSString` from a `String`.
    fn from(s: String) -> Self {
        NSString {
            objc: unsafe {
                let nsstring: id = msg_send![class!(NSString), alloc];
                Id::from_ptr(msg_send![nsstring, initWithString:s.as_str()])
            },

            marker: PhantomData,
        }
    }
}

impl From<&str> for NSString<'_> {
    /// Creates a new `NSString` from a `&str`.
    fn from(s: &str) -> Self {
        let objc = unsafe {
            let nsstring: *mut Object = msg_send![class!(NSString), alloc];
            Id::from_ptr(msg_send![nsstring, initWithBytes:s.as_ptr()
                length:s.len()
                encoding:UTF8_ENCODING
            ])
        };

        NSString {
            objc,
            marker: PhantomData,
        }
    }
}

impl From<(&str, Encoding)> for NSString<'_> {
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
            objc,
            marker: PhantomData,
        }
    }
}

impl PartialEq for NSString<'_> {
    /// Checks if two `NSString`s are equal.
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let result: BOOL = msg_send![&*self.objc, isEqualToString:&*other.objc];
            to_bool(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let s = NSString::from_str("Hello, World!").unwrap();
        assert_eq!(s.as_str(), "Hello, World!");
    }

    #[test]
    fn test_from_no_cpy_str() {
        let s = NSString::from_no_cpy_str("Hello, World!");
        assert_eq!(s.as_str(), "Hello, World!");
    }

    #[test]
    fn test_bytes_len() {
        let s = NSString::from_str("Hello, World!").unwrap();
        assert_eq!(s.bytes_len(), 13);
    }

    #[test]
    fn test_as_str() {
        let s = NSString::from_str("Hello, World!").unwrap();
        assert_eq!(s.as_str(), "Hello, World!");
    }

    #[test]
    fn test_to_string() {
        let s = NSString::from_str("Hello, World!").unwrap();
        assert_eq!(s.to_string(), "Hello, World!".to_string());
    }

    #[test]
    fn test_eq() {
        let s1 = NSString::from_str("Hello, World!").unwrap();
        let s2 = NSString::from_str("Hello, World!").unwrap();
        let s3 = NSString::from_str("Hello, World!").unwrap();
        let s4 = NSString::from_str("Hello, World!").unwrap();

        assert_eq!(s1, s2);
        assert_eq!(s1, s3);
        assert_eq!(s1, s4);
    }
}
