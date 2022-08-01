use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::INSString,
    objective_c_runtime::{
        id,
        macros::object,
        traits::{FromId, PNSObject},
    },
};

use super::{
    NSComparisonResult, NSRange, NSString, NSStringCompareOptions, NSStringTransform, UInt,
};

object! {
    /// A mutable string.
    unsafe pub struct NSMutableString;
}

#[interface_impl(NSString)]
impl NSMutableString {
    /* Creating and Initializing a Mutable String
     */

    /// Returns an empty NSMutableString object with initial storage for a
    /// given number of characters.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The number of characters to allocate space for.
    ///
    /// # Returns
    ///
    /// An empty NSMutableString object with initial storage for a given number of characters.
    #[method]
    pub fn string_with_capacity(capacity: UInt) -> NSMutableString {
        unsafe {
            NSMutableString::from_id(msg_send![Self::m_class(), stringWithCapacity: capacity])
        }
    }

    /// Returns an NSMutableString object initialized with initial storage for
    /// a given number of characters,
    ///
    /// # Arguments
    ///
    /// * `capacity` - The number of characters to allocate space for.
    #[method]
    pub fn init_with_capacity(mut self, capacity: UInt) -> NSMutableString
    where
        Self: Sized,
    {
        unsafe { msg_send![self.m_self(), initWithCapacity: capacity] }
    }

    /* Modifying a String
     */

    /// Adds a constructed string to the receiver.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to append to the receiver.
    #[method]
    pub fn append_string<S>(&mut self, string: S)
    where
        S: INSString,
    {
        unsafe { msg_send![self.m_self(), appendString: string] }
    }

    /// Transliterates the receiver by applying a specified ICU string transform.
    ///
    /// # Arguments
    ///
    /// * `transform` - The `StringTransform` to apply.
    /// * `reverse` - If `true`, the transformation is applied in reverse.
    /// * `range` - The range of characters to apply the transformation to.
    /// * `out_range` - The range of characters that were transformed.
    ///
    /// # Returns
    ///
    /// `true` if the transformation was successful, otherwise `false`.
    #[method]
    pub fn apply_transform_reverse_range_updated_range(
        &mut self,
        transform: NSStringTransform,
        reverse: bool,
        range: NSRange,
        out_range: NSRange,
    ) -> bool {
        unsafe {
            msg_send![
                self.m_self(),
                applyTransform: transform
                reverse: reverse
                range: range
                updatedRange: out_range
            ]
        }
    }

    /// Removes from the receiver the characters in a given range.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of characters to remove.
    #[method]
    pub fn delete_characters_in_range(&mut self, range: NSRange) {
        unsafe { msg_send![self.m_self(), deleteCharactersInRange: range] }
    }

    /// Inserts into the receiver the characters of a given string at a given
    /// location.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to insert into the receiver.
    /// * `loc` - The location at which to insert `string`.
    #[method]
    pub fn insert_string_at_index<S>(&mut self, string: S, loc: UInt)
    where
        S: INSString,
    {
        unsafe { msg_send![self.m_self(), insertString: string atIndex: loc] }
    }

    /// Replaces the characters from aRange with those in `string`.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of characters to replace.
    /// * `string` - The string to replace with.
    #[method]
    pub fn replace_characters_in_range_with_string<S>(&mut self, range: NSRange, string: S)
    where
        S: INSString,
    {
        unsafe {
            msg_send![
                self.m_self(),
                replaceCharactersInRange: range
                withString: string
            ]
        }
    }

    /// Replaces all occurrences of a given string in a given range with
    /// another given string, returning the number of replacements.
    #[method]
    pub fn replace_occurrences_of_string_with_string_options_range<S>(
        &mut self,
        target: NSString,
        replacement: S,
        options: NSStringCompareOptions,
        search_range: NSRange,
    ) -> UInt
    where
        S: INSString,
    {
        unsafe {
            msg_send![
                self.m_self(),
                replaceOccurrencesOfString: target
                withString: replacement
                options: options
                range: search_range
            ]
        }
    }

    /// Replaces the characters of the receiver with those in a given string.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to replace the characters of the receiver with. string must not be `null`.
    #[method]
    pub fn set_string<S>(&mut self, string: S)
    where
        S: INSString,
    {
        unsafe { msg_send![self.m_self(), setString: string] }
    }
}

impl INSString for NSMutableString {}

impl Default for NSMutableString {
    fn default() -> Self {
        Self::m_string()
    }
}

impl PartialEq for NSMutableString {
    fn eq(&self, other: &NSMutableString) -> bool {
        self.m_localized_compare(other.clone()) == NSComparisonResult::OrderedSame
    }
}

impl PartialEq<&str> for NSMutableString {
    fn eq(&self, other: &&str) -> bool {
        self.m_localized_compare(NSString::from(*other)) == NSComparisonResult::OrderedSame
    }
}

impl From<NSString> for NSMutableString {
    fn from(string: NSString) -> Self {
        unsafe {
            let ptr: id = msg_send![NSString::m_class(), alloc];
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
        let string = NSMutableString::string_with_capacity(10);
        assert_eq!(string.m_length_of_bytes_using_encoding(Encoding::UTF8), 0);
    }

    #[test]
    fn test_im_append_string() {
        let mut string = NSMutableString::string_with_capacity(10);
        string.append_string(NSString::from("Hello"));
        assert_eq!(string.m_length_of_bytes_using_encoding(Encoding::UTF8), 5);
    }

    #[test]
    fn test_im_apply_transform_reverse_range_updated_range() {
        let mut string = NSMutableString::string_with_capacity(10);
        string.append_string(NSString::from("Hello"));
        assert_eq!(string.m_length_of_bytes_using_encoding(Encoding::UTF8), 5);
        assert!(string.m_apply_transform_reverse_range_updated_range(
            unsafe { LatinToKatakana },
            false,
            (0..5).into(),
            (0..5).into()
        ));
    }

    #[test]
    fn test_im_delete_characters_in_range() {
        let mut string = NSMutableString::string_with_capacity(10);
        string.append_string(NSString::from("Hello"));
        assert_eq!(string.m_length_of_bytes_using_encoding(Encoding::UTF8), 5);
        string.m_delete_characters_in_range((0..5).into());
        assert_eq!(string.m_length_of_bytes_using_encoding(Encoding::UTF8), 0);
    }

    #[test]
    fn test_im_insert_string_at_index() {
        let mut string = NSMutableString::string_with_capacity(10);
        string.append_string(NSString::from("Hello"));
        assert_eq!(string.m_length_of_bytes_using_encoding(Encoding::UTF8), 5);
        string.m_insert_string_at_index(NSString::from("World"), 0);
        assert_eq!(string.m_length_of_bytes_using_encoding(Encoding::UTF8), 10);
    }

    #[test]
    fn test_im_replace_characters_in_range_with_string() {
        let mut string = NSMutableString::string_with_capacity(10);
        string.append_string(NSString::from("Hello"));
        assert_eq!(string.m_length_of_bytes_using_encoding(Encoding::UTF8), 5);
        string.m_replace_characters_in_range_with_string((0..5).into(), NSString::from("World"));
        assert_eq!(string.m_length_of_bytes_using_encoding(Encoding::UTF8), 5);
    }

    #[test]
    fn test_im_replace_occurrences_of_string_with_string_options_range() {
        let mut string = NSMutableString::string_with_capacity(10);
        string.append_string(NSString::from("Hello"));
        assert_eq!(string.m_length_of_bytes_using_encoding(Encoding::UTF8), 5);
        assert_eq!(
            string.m_replace_occurrences_of_string_with_string_options_range(
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
        let mut string = NSMutableString::string_with_capacity(10);
        string.append_string(NSString::from("Hello"));
        assert_eq!(string.m_length_of_bytes_using_encoding(Encoding::UTF8), 5);
        string.set_string(NSString::from("World"));
        assert_eq!(string.m_length_of_bytes_using_encoding(Encoding::UTF8), 5);
    }
}
