use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::{NSArray, NSData, NSDateComponents, NSString},
    object,
    objective_c_runtime::macros::interface_impl,
};

use super::{
    CNContactRelation, CNInstantMessageAddress, CNLabeledValue, CNPhoneNumber, CNSocialProfile,
    ICNContact, ICNPostalAddress,
};

object! {
    /// A mutable object that stores information about a single contact, such as the contact's first name, phone numbers, and addresses.
    unsafe pub struct CNMutableContact;
}

impl ICNContact for CNMutableContact {}

#[interface_impl(CNContact)]
impl CNMutableContact {
    /// Sets the name prefix of the contact.
    #[property]
    pub fn set_name_prefix(&mut self, name_prefix: NSString) {
        unsafe { msg_send![self.m_self(), setNamePrefix: name_prefix] }
    }

    /// Sets the given name of the contact.
    #[property]
    pub fn set_given_name(&mut self, given_name: NSString) {
        unsafe { msg_send![self.m_self(), setGivenName: given_name] }
    }

    /// Sets the middle name of the contact.
    #[property]
    pub fn set_middle_name(&mut self, middle_name: NSString) {
        unsafe { msg_send![self.m_self(), setMiddleName: middle_name] }
    }

    /// Sets the family name of the contact.
    #[property]
    pub fn set_family_name(&mut self, family_name: NSString) {
        unsafe { msg_send![self.m_self(), setFamilyName: family_name] }
    }

    /// Sets the previous family name of the contact.
    #[property]
    pub fn set_previous_family_name(&mut self, previous_family_name: NSString) {
        unsafe { msg_send![self.m_self(), setPreviousFamilyName: previous_family_name] }
    }

    /// Sets the name suffix of the contact.
    #[property]
    pub fn set_name_suffix(&mut self, name_suffix: NSString) {
        unsafe { msg_send![self.m_self(), setNameSuffix: name_suffix] }
    }

    /// Sets the nickname of the contact.
    #[property]
    pub fn set_nickname(&mut self, nickname: NSString) {
        unsafe { msg_send![self.m_self(), setNickname: nickname] }
    }

    /// Sets the phonetic given name of the contact.
    #[property]
    pub fn set_phonetic_given_name(&mut self, phonetic_given_name: NSString) {
        unsafe { msg_send![self.m_self(), setPhoneticGivenName: phonetic_given_name] }
    }

    /// Sets the phonetic middle name of the contact.
    #[property]
    pub fn set_phonetic_middle_name(&mut self, phonetic_middle_name: NSString) {
        unsafe { msg_send![self.m_self(), setPhoneticMiddleName: phonetic_middle_name] }
    }

    /// Sets the phonetic family name of the contact.
    #[property]
    pub fn set_phonetic_family_name(&mut self, phonetic_family_name: NSString) {
        unsafe { msg_send![self.m_self(), setPhoneticFamilyName: phonetic_family_name] }
    }

    /* Setting Work Information
     */

    /// The contact’s job title.
    #[property]
    pub fn set_job_title(&mut self, job_title: NSString) {
        unsafe { msg_send![self.m_self(), setJobTitle: job_title] }
    }

    /// The name of the department associated with the contact.
    #[property]
    pub fn set_department_name(&mut self, department_name: NSString) {
        unsafe { msg_send![self.m_self(), setDepartmentName: department_name] }
    }

    /// The name of the organization associated with the contact.
    #[property]
    pub fn set_organization_name(&mut self, organization_name: NSString) {
        unsafe { msg_send![self.m_self(), setOrganizationName: organization_name] }
    }

    /// The phonetic name of the organization associated with the contact.
    #[property]
    pub fn set_phonetic_organization_name(&mut self, phonetic_organization_name: NSString) {
        unsafe {
            msg_send![
                self.m_self(),
                setPhoneticOrganizationName: phonetic_organization_name
            ]
        }
    }

    /* Setting Addresses
     */

    /// An array of labeled postal addresses for a contact.
    #[property]
    pub fn set_postal_addresses<PostalAddress>(
        &mut self,
        postal_addresses: NSArray<CNLabeledValue<PostalAddress>>,
    ) where
        PostalAddress: ICNPostalAddress,
    {
        unsafe { msg_send![self.m_self(), setPostalAddresses: postal_addresses] }
    }

    /// An array of labeled email addresses for a contact.
    #[property]
    pub fn set_email_addresses(&mut self, email_addresses: NSArray<CNLabeledValue<NSString>>) {
        unsafe { msg_send![self.m_self(), setEmailAddresses: email_addresses] }
    }

    /// An array of labeled URL addresses for a contact.
    #[property]
    pub fn set_url_addresses(&mut self, url_addresses: NSArray<CNLabeledValue<NSString>>) {
        unsafe { msg_send![self.m_self(), setUrlAddresses: url_addresses] }
    }

    /* Setting Phone Information
     */

    /// An array of labeled phone numbers for a contact.
    #[property]
    pub fn set_phone_numbers(&mut self, phone_numbers: NSArray<CNLabeledValue<CNPhoneNumber>>) {
        unsafe { msg_send![self.m_self(), setPhoneNumbers: phone_numbers] }
    }

    /* Setting Social Profiles
     */

    /// An array of labeled social profiles for a contact.
    #[property]
    pub fn set_social_profiles(
        &mut self,
        social_profiles: NSArray<CNLabeledValue<CNSocialProfile>>,
    ) {
        unsafe { msg_send![self.m_self(), setSocialProfiles: social_profiles] }
    }

    /* Setting Birthday Information
     */

    /// An array containing labeled Gregorian dates.
    #[property]
    pub fn set_dates(&mut self, dates: NSArray<CNLabeledValue<NSDateComponents>>) {
        unsafe { msg_send![self.m_self(), setBirthdayDates: dates] }
    }

    /// A date component for the non-Gregorian birthday of the contact.
    #[property]
    pub fn set_non_gregorian_birthday(&mut self, non_gregorian_birthday: NSDateComponents) {
        unsafe {
            msg_send![
                self.m_self(),
                setNonGregorianBirthday: non_gregorian_birthday
            ]
        }
    }

    /// A date component for the Gregorian birthday of the contact.
    #[property]
    pub fn set_birthday(&mut self, birthday: NSDateComponents) {
        unsafe { msg_send![self.m_self(), setBirthday: birthday] }
    }

    /* Setting Notes
     */

    /// A string containing notes for the contact.
    #[property]
    pub fn set_note(&mut self, note: NSString) {
        unsafe { msg_send![self.m_self(), setNote: note] }
    }

    /// Setting Images
    #[property]
    pub fn set_image_data(&mut self, image_data: NSData) {
        unsafe { msg_send![self.m_self(), setImageData: image_data] }
    }

    /* Relating Other Information to the Contact
     */

    /// An array of labeled contact relations for the contact.
    #[property]
    pub fn set_contact_relations(
        &mut self,
        contact_relations: NSArray<CNLabeledValue<CNContactRelation>>,
    ) {
        unsafe { msg_send![self.m_self(), setContactRelations: contact_relations] }
    }

    /// An array of labeled IM addresses for the contact.
    #[property]
    pub fn set_instant_messenger_addresses(
        &mut self,
        instant_messenger_addresses: NSArray<CNLabeledValue<CNInstantMessageAddress>>,
    ) {
        unsafe {
            msg_send![
                self.m_self(),
                setInstantMessageAddresses: instant_messenger_addresses
            ]
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        contacts::{CNContactType, CNMutableContact, ICNContact},
        objective_c_runtime::traits::PNSObject,
    };

    #[test]
    fn test_mutable_contact() {
        let contact = CNMutableContact::m_new();

        assert!(contact.p_birthday().is_none());
        assert!(contact.p_contact_relations().count() == 0);
        assert!(contact.p_contact_type() == CNContactType::Person);
        assert!(contact.p_dates().count() == 0);
        assert!(contact.p_department_name() == "");
        assert!(contact.p_email_addresses().count() == 0);
        assert!(contact.p_family_name() == "");
        assert!(contact.p_given_name() == "");
        assert!(contact.p_identifier() != "");
        assert!(contact.p_image_data().is_none());
        assert!(!contact.p_image_data_available());
        assert!(contact.p_instant_messaging_addresses().count() == 0);
        assert!(contact.p_job_title() == "");
        assert!(contact.p_middle_name() == "");
        assert!(contact.p_name_prefix() == "");
        assert!(contact.p_name_suffix() == "");
        assert!(contact.p_nickname() == "");
        assert!(contact.p_non_gregorian_birthday().is_none());
        assert!(contact.p_note() == "");
        assert!(contact.p_organization_name() == "");
        assert!(contact.p_phone_numbers().count() == 0);
        assert!(contact.p_phonetic_given_name() == "");
        assert!(contact.p_phonetic_middle_name() == "");
        assert!(contact.p_phonetic_family_name() == "");
        assert!(contact.p_postal_addresses().count() == 0);
        assert!(contact.p_previous_family_name() == "");
        assert!(contact.p_social_profiles().count() == 0);
        assert!(contact.p_thumbnail_image_data().is_none());
        assert!(contact.p_url_addresses().count() == 0);
    }
}
