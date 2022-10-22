use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    objective_c_runtime::{
        id,
        macros::object,
        nil,
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};

use super::{
    NSAttributedStringDocumentAttributeKey, NSAttributedStringDocumentReadingOptionKey,
    NSAttributedStringKey, NSData, NSDictionary, NSError, NSRange, NSRangePointer, NSString, UInt,
};

object! {
    /// A string with associated attributes (such as visual style, hyperlinks,
    /// or accessibility data) for portions of its text.
    unsafe pub struct NSAttributedString;
}

#[interface_impl(NSObject)]
impl NSAttributedString {
    /* Creating an Attributed String
     */

    /// Creates an attributed string with the characters of the specified string and no attribute information.
    #[method]
    pub fn init_with_string(string: &NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), initWithString: string.m_self()]) }
    }

    /// Creates an attributed string with the specified string and attributes.
    #[method]
    pub fn init_with_string_attributes(
        string: &NSString,
        attributes: &NSDictionary<NSAttributedStringKey, id>,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(
                msg_send![Self::m_class(), initWithString: string.m_self() attributes: attributes.m_self()],
            )
        }
    }

    /// Creates an attributed string with the characters and attributes of the specified attributed string.
    #[method]
    pub fn init_with_attributed_string(attr_string: &NSAttributedString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                Self::m_class(),
                initWithAttributedString: attr_string.m_self()
            ])
        }
    }

    /// Creates an attributed string from the data in the specified data object.
    #[method]
    pub fn init_with_data_options_document_attributes(
        data: &NSData,
        options: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, id>,
        document: &NSDictionary<NSAttributedStringDocumentAttributeKey, id>,
    ) -> Result<Self, NSError>
    where
        Self: Sized + FromId,
    {
        unsafe {
            let mut error = NSError::m_alloc();
            let result = Self::from_id(
                msg_send![Self::m_class(), initWithData: data.m_self() options: options.m_self() documentAttributes: document.m_self() error: &mut error],
            );

            if error.m_self() == nil {
                Ok(result)
            } else {
                Err(error)
            }
        }
    }

    /* Retrieving Character Information
     */

    /// The character contents of the attributed string as a string.
    #[property]
    pub fn string(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), string]) }
    }

    /// The length of the attributed string.
    #[property]
    pub fn length(&self) -> UInt {
        unsafe { msg_send![self.m_self(), length] }
    }

    /* Retrieving Attribute Information
     */

    /// Returns the attributes for the character at the specified index.
    #[method]
    pub fn attributes_at_index_effective_range(
        &self,
        location: UInt,
        range: NSRangePointer,
    ) -> NSDictionary<NSAttributedStringKey, id> {
        unsafe {
            NSDictionary::from_id(
                msg_send![self.m_self(), attributesAtIndex: location effectiveRange: range],
            )
        }
    }

    /// Returns the attributes for the character at the specified index and, by reference, the range where the attributes apply.
    #[method]
    pub fn attributes_at_index_longest_effective_range_in_range(
        &self,
        location: UInt,
        range: NSRangePointer,
        range_limit: NSRange,
    ) -> NSDictionary<NSAttributedStringKey, id> {
        unsafe {
            NSDictionary::from_id(
                msg_send![self.m_self(), attributesAtIndex: location longestEffectiveRange: range inRange: range_limit],
            )
        }
    }

    /* Comparing Attributed Strings
     */

    /// Returns a Boolean value that indicates whether the attributed string is equal to another attributed string.
    #[method]
    pub fn is_equal_to_attributed_string(&self, other: &NSAttributedString) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isEqualToAttributedString: other.m_self()]) }
    }

    /* Extracting a Substring
     */

    /// Returns an attributed string consisting of the characters and attributes within the specified range in the attributed string.
    #[method]
    pub fn attributed_substring_from_range(&self, range: NSRange) -> NSAttributedString {
        unsafe {
            NSAttributedString::from_id(msg_send![
                self.m_self(),
                attributedSubstringFromRange: range
            ])
        }
    }
}
