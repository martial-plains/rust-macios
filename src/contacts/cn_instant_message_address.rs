use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::NSString,
    objective_c_runtime::{
        macros::{interface_impl, object},
        traits::{FromId, PNSObject},
    },
};

object! {
    /// An immutable object representing an instant message address for the contact.
    unsafe pub struct CNInstantMessageAddress;
}

#[interface_impl(NSObject)]
impl CNInstantMessageAddress {
    /// Returns a CNInstantMessageAddress object initialized with the specified user name and service.
    #[method]
    pub fn init_with_username_service(&mut self, username: NSString, service: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![self.m_self(), initWithUsername: username service: service])
        }
    }

    /* Getting the Address Information
     */

    /// The service type of the instant message address.
    #[property]
    pub fn service(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), service]) }
    }
    /// The username of the instant message address.
    #[property]
    pub fn username(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), username]) }
    }

    /* Getting Localized Address Information
     */

    /// Returns a string containing the localized property name.
    #[method]
    pub fn localized_string_for_key(key: NSString) -> NSString {
        unsafe { NSString::from_id(msg_send![Self::m_class(), localizedStringForKey: key]) }
    }

    /// Returns a string containing the localized name of the specified service.
    #[method]
    pub fn localized_string_for_service(service: NSString) -> NSString {
        unsafe {
            NSString::from_id(msg_send![
                Self::m_class(),
                localizedStringForService: service
            ])
        }
    }
}
