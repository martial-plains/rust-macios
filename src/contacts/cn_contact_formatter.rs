use objc::{msg_send, sel, sel_impl};
use rust_macios_objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::{INSFormatter, NSAttributedString, NSDictionary, NSString},
    object,
    objective_c_runtime::id,
    utils::to_optional,
};

use super::CNContact;

#[repr(i64)]
pub enum CNContactFormatterStyle {
    /// Combine the contact name components into a displayable full name.
    FullName,
    /// Combine the contact phonetic name components into a displayable phonetic full name.
    PhoneticFullName,
}

#[repr(i64)]
pub enum CNContactDisplayNameOrder {
    CNContactDisplayNameOrderUserDefault,
    CNContactDisplayNameOrderGivenNameFirst,
    CNContactDisplayNameOrderFamilyNameFirst,
}

object! {
    /// An object that you use to format contact information before displaying it to the user.
    unsafe pub struct CNContactFormatter;
}

impl INSFormatter for CNContactFormatter {}

#[interface_impl(NSFormatter)]
impl CNContactFormatter {
    /* Creating a Formatted Attributed String
     */

    /// Formats the contact name as an attributed string.
    #[method]
    pub fn attributed_string_from_contact_default_attributes<K, V>(
        &self,
        contact: CNContact,
        attributes: NSDictionary<K, V>,
    ) -> Option<NSAttributedString> {
        unsafe {
            to_optional(
                msg_send![self.m_self(), attributedStringFromContact: contact defaultAttributes: attributes],
            )
        }
    }

    /// Formats the contact name as an attributed string.
    #[method]
    pub fn attributed_string_from_contact_style_default_attributes<K, V>(
        contact: CNContact,
        style: CNContactFormatterStyle,
        attributes: NSDictionary<K, V>,
    ) -> Option<NSAttributedString> {
        unsafe {
            to_optional(
                msg_send![Self::m_class(), attributedStringFromContact: contact style: style defaultAttributes: attributes],
            )
        }
    }

    /// Formats the contact name.
    #[method]
    pub fn string_from_contact(&self, contact: CNContact) -> Option<NSString> {
        unsafe { to_optional(msg_send![self.m_self(), stringFromContact: contact]) }
    }

    /// Returns the contact name, formatted with the specified formatter.
    #[method]
    pub fn string_from_contact_style(
        contact: CNContact,
        style: CNContactFormatterStyle,
    ) -> Option<NSString> {
        unsafe { to_optional(msg_send![Self::m_class(), stringFromContact: contact style: style]) }
    }

    /* Specifying the Formatting Style
     */

    /// The formatting style for the contact name.
    #[property]
    pub fn style(&self) -> CNContactFormatterStyle {
        unsafe { msg_send![self.m_self(), style] }
    }

    /// Sets the formatting style for the contact name.
    #[property]
    pub fn set_style(&mut self) -> CNContactFormatterStyle {
        unsafe { msg_send![self.m_self(), style] }
    }

    /* Getting a Descriptor
     */

    /// Returns the required key descriptor for the specified formatting style of the contact.
    #[method]
    pub fn descriptor_for_required_keys_for_style(style: CNContactFormatterStyle) -> id {
        unsafe { msg_send![Self::m_class(), descriptorForRequiredKeysForStyle: style] }
    }

    /* Getting Format Information
     */

    /// Returns the delimiter to use between name components.
    #[method]
    pub fn delimiter_for_contact(contact: CNContact) -> NSString {
        unsafe { msg_send![Self::m_class(), delimiterForContact: contact] }
    }

    /// Returns the display name order.
    #[method]
    pub fn name_order_for_contact(contact: CNContact) -> CNContactDisplayNameOrder {
        unsafe { msg_send![Self::m_class(), nameOrderForContact: contact] }
    }

    /* Type Properties
     */

    #[property]
    pub fn descriptor_for_required_keys_for_delimiter() -> id {
        unsafe { msg_send![Self::m_class(), descriptorForRequiredKeysForDelimiter] }
    }

    #[property]
    pub fn descriptor_for_required_keys_for_name_order() -> id {
        unsafe { msg_send![Self::m_class(), descriptorForRequiredKeysForNameOrder] }
    }
}
