use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::NSString,
    objective_c_runtime::{
        macros::{interface_impl, object},
        traits::{FromId, PNSObject},
    },
};

use super::CNContact;

object! {
    /// An object that represents a property of a contact.
    unsafe pub struct CNContactProperty;
}

#[interface_impl(NSObject)]
impl CNContactProperty {
    /* Getting the Contact Object
     */

    /* Getting the Contact Object */

    /// The associated contact.
    #[property]
    pub fn contact(&self) -> CNContact {
        unsafe { CNContact::from_id(msg_send![self.m_self(), contact]) }
    }

    /* Getting the Property Information
     */

    /// The key of the contact property.
    #[property]
    pub fn key(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), key]) }
    }

    /// The value of the property.
    #[property]
    pub fn value(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), value]) }
    }

    /// The label of the labeled value of the property array.
    #[property]
    pub fn label(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), label]) }
    }

    /// The label of the labeled value of the property array.
    #[property]
    pub fn identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), identifier]) }
    }
}
