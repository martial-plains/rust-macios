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
    /// An immutable object that represents one of the user's social profiles.
    unsafe pub struct CNSocialProfile;
}

#[interface_impl(NSObject)]
impl CNSocialProfile {
    /// Initializes a new social profile object with the specified URL.
    #[method]
    pub fn init_with_url_string_username_user_identifier_service(
        &mut self,
        url_string: NSString,
        username: NSString,
        user_identifier: NSString,
        service: NSString,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(
                msg_send![self.m_self(), initWithUrlString: url_string username: username userIdentifier: user_identifier service: service],
            )
        }
    }

    /* Getting Social Profile Information
     */

    /// The user name for the social profile.
    #[property]
    pub fn username(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), username]) }
    }

    /// The social profile’s service name.
    #[property]
    pub fn service(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), service]) }
    }

    /// The URL associated with the social profile.
    #[property]
    pub fn url_string(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), urlString]) }
    }

    /// The service’s user identifier associated with the social profile.
    #[property]
    pub fn user_identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), userIdentifier]) }
    }

    /* Getting Localized User Profile Information
     */

    /// Returns the localized name of the property for the specified key.
    #[method]
    pub fn localized_string_for_key(key: NSString) -> NSString {
        unsafe { NSString::from_id(msg_send![Self::m_class(), localizedStringForKey: key]) }
    }

    /// Returns the localized name of the specified service.
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
