use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::NSString,
    objective_c_runtime::macros::{interface_impl, object},
};

use super::ICNPostalAddress;

object! {
    /// A mutable representation of the postal address for a contact.
    unsafe pub struct CNMutablePostalAddress;
}

impl ICNPostalAddress for CNMutablePostalAddress {}

#[interface_impl(CNPostalAddress)]
impl CNMutablePostalAddress {
    /* Modifying the Parts of a Postal Address
     */

    /// The street name of the address.
    #[property]
    pub fn set_street(&mut self, street: NSString) {
        unsafe { msg_send![self.m_self(), setStreet: street] }
    }

    /// The city name of the address.
    #[property]
    pub fn set_city(&mut self, city: NSString) {
        unsafe { msg_send![self.m_self(), setCity: city] }
    }

    /// The state name of the address.
    #[property]
    pub fn set_state(&mut self, state: NSString) {
        unsafe { msg_send![self.m_self(), setState: state] }
    }

    /// The postal code of the address.
    #[property]
    pub fn set_postal_code(&mut self, postal_code: NSString) {
        unsafe { msg_send![self.m_self(), setPostalCode: postal_code] }
    }

    /// The country or region name of the address.
    #[property]
    pub fn set_country(&mut self, country: NSString) {
        unsafe { msg_send![self.m_self(), setCountry: country] }
    }

    /// The ISO country code, using the ISO 3166-1 alpha-2 standard.
    #[property]
    pub fn set_iso_country_code(&mut self, iso_country_code: NSString) {
        unsafe { msg_send![self.m_self(), setISOCountryCode: iso_country_code] }
    }

    /// The subadministrative area (such as a county or other region) in a postal address.
    #[property]
    pub fn set_sub_administrative_area(&mut self, sub_administrative_area: NSString) {
        unsafe {
            msg_send![
                self.m_self(),
                setSubAdministrativeArea: sub_administrative_area
            ]
        }
    }

    /// Additional information associated with the location, typically defined at the city or town level, in a postal address.
    #[property]
    pub fn set_sub_locality(&mut self, sub_locality: NSString) {
        unsafe { msg_send![self.m_self(), setSubLocality: sub_locality] }
    }
}
