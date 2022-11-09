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
    /// An immutable object that represents the relationship between one contact to another.
    unsafe pub struct CNContactRelation;
}

#[interface_impl(NSObject)]
impl CNContactRelation {
    /// Creates an object with the name of the related contact.
    #[method]
    pub fn init_with_name(&mut self, name: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithName: name]) }
    }

    /// Instantiate a class instance with the name of the related contact.
    #[method]
    pub fn m_contact_relation_with_name(name: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), contactRelationWithName: name]) }
    }

    /// The name of the related contact.
    #[property]
    pub fn name(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), name]) }
    }
}
