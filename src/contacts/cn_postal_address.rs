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
    /// An immutable representation of the postal address for a contact.
    unsafe pub struct CNPostalAddress;
}

#[interface_impl(NSObject)]
impl CNPostalAddress {
    /* Getting the Parts of a Postal Address
     */

    /// The street name in a postal address.
    #[property]
    pub fn street(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), street]) }
    }

    /// The city name in a postal address.
    #[property]
    pub fn city(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), city]) }
    }

    /// The state name in a postal address.
    #[property]
    pub fn state(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), state]) }
    }

    /// The postal code in a postal address.
    #[property]
    pub fn postal_code(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), postalCode]) }
    }

    /// The country or region name in a postal address.
    #[property]
    pub fn country(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), country]) }
    }

    /// The ISO country code for the country or region in a postal address, using the ISO 3166-1 alpha-2 standard.
    #[property]
    pub fn iso_country_code(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), ISOCountryCode]) }
    }

    /// The subadministrative area (such as a county or other region) in a postal address.
    #[property]
    pub fn sub_administrative_area(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), subAdministrativeArea]) }
    }

    /// Additional information associated with the location, typically defined at the city or town level, in a postal address.
    #[property]
    pub fn sub_locality(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), subLocality]) }
    }

    /* Getting Localized Postal Values
     */

    /// Returns the localized name for the property associated with the specified key.
    pub fn localized_string_for_key(key: NSString) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), localizedStringForKey: key]) }
    }
}
