use std::fmt;

use objc::{
    class, msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_id::Id;

use crate::objective_c_runtime::{
    id,
    traits::{FromId, PNSObject, ToId},
};

use super::{
    traits::{INSMutableString, INSString},
    NSComparisonResult, NSString,
};

/// A mutable string.
#[repr(C)]
pub struct NSMutableString {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl NSMutableString {
    /// Adds a constructed string to the receiver.
    pub fn append_string<S>(&mut self, string: S)
    where
        S: Into<NSString>,
    {
        self.im_append_string(string.into())
    }
}

impl PNSObject for NSMutableString {
    fn im_class<'a>() -> &'a Class {
        class!(NSMutableString)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl INSString for NSMutableString {}

impl INSMutableString for NSMutableString {}

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
        string.append_string(&"Hello");
        assert_eq!(string.im_length_of_bytes_using_encoding(Encoding::UTF8), 5);
    }

    #[test]
    fn test_im_apply_transform_reverse_range_updated_range() {
        let mut string = NSMutableString::tm_string_with_capacity(10);
        string.append_string(&"Hello");
        assert_eq!(string.im_length_of_bytes_using_encoding(Encoding::UTF8), 5);
        assert!(string.im_apply_transform_reverse_range_updated_range(
            unsafe { LatinToKatakana },
            false,
            (0..5).into(),
            (0..5).into()
        ));
    }

    #[test]
    fn test_im_delete_characters_in_range() {
        let mut string = NSMutableString::tm_string_with_capacity(10);
        string.append_string(&"Hello");
        assert_eq!(string.im_length_of_bytes_using_encoding(Encoding::UTF8), 5);
        string.im_delete_characters_in_range((0..5).into());
        assert_eq!(string.im_length_of_bytes_using_encoding(Encoding::UTF8), 0);
    }

    #[test]
    fn test_im_insert_string_at_index() {
        let mut string = NSMutableString::tm_string_with_capacity(10);
        string.append_string(&"Hello");
        assert_eq!(string.im_length_of_bytes_using_encoding(Encoding::UTF8), 5);
        string.im_insert_string_at_index(NSString::from("World"), 0);
        assert_eq!(string.im_length_of_bytes_using_encoding(Encoding::UTF8), 10);
    }

    #[test]
    fn test_im_replace_characters_in_range_with_string() {
        let mut string = NSMutableString::tm_string_with_capacity(10);
        string.append_string(&"Hello");
        assert_eq!(string.im_length_of_bytes_using_encoding(Encoding::UTF8), 5);
        string.im_replace_characters_in_range_with_string((0..5).into(), NSString::from("World"));
        assert_eq!(string.im_length_of_bytes_using_encoding(Encoding::UTF8), 5);
    }

    #[test]
    fn test_im_replace_occurrences_of_string_with_string_options_range() {
        let mut string = NSMutableString::tm_string_with_capacity(10);
        string.append_string(&"Hello");
        assert_eq!(string.im_length_of_bytes_using_encoding(Encoding::UTF8), 5);
        assert_eq!(
            string.im_replace_occurrences_of_string_with_string_options_range(
                NSString::from("Hello"),
                NSString::from("World"),
                NSStringCompareOptions::CaseInsensitive,
                (0..5).into()
            ),
            1
        );
    }

    #[test]
    fn test_im_set_string() {
        let mut string = NSMutableString::tm_string_with_capacity(10);
        string.append_string(&"Hello");
        assert_eq!(string.im_length_of_bytes_using_encoding(Encoding::UTF8), 5);
        string.im_set_string(NSString::from("World"));
        assert_eq!(string.im_length_of_bytes_using_encoding(Encoding::UTF8), 5);
    }
}
