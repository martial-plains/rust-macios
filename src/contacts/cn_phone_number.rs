use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::NSString,
    object,
    objective_c_runtime::{
        macros::interface_impl,
        traits::{FromId, PNSObject},
    },
};

object! {
    /// An immutable object representing a phone number for a contact.
    unsafe pub struct CNPhoneNumber;
}

#[interface_impl(NSObject)]
impl CNPhoneNumber {
    /* Creating a Phone Number Object
     */

    /// Returns a new phone number object initialized with the specified phone number string.
    #[method]
    pub fn init_with_string_value(&mut self, phone_number: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithString: phone_number]) }
    }

    /// Returns a new phone number object initialized with the specified phone number string.
    #[method]
    pub fn phone_number_with_string_value(phone_number: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                Self::m_class(),
                phoneNumberWithString: phone_number
            ])
        }
    }

    /* Getting the Phone Number
     */

    /// The string value of the phone number.
    #[property]
    pub fn string_value(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), stringValue]) }
    }
}
