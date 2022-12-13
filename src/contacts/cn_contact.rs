use std::fmt::Debug;

use crate::{
    foundation::{NSArray, NSComparator, NSData, NSDateComponents, NSPredicate, NSString},
    object,
    objective_c_runtime::{
        id,
        macros::interface_impl,
        nil,
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};
use objc::{msg_send, sel, sel_impl};

use super::{
    CNContactRelation, CNInstantMessageAddress, CNLabeledValue, CNPhoneNumber, CNPostalAddress,
    CNSocialProfile,
};

/// The types a contact can be.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum CNContactType {
    /// A person.
    Person,
    /// A organization.
    Organization,
}

/// Indicates the sorting order for contacts.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum CNContactSortOrder {
    /// No sorting order.
    None,
    /// The user’s default sorting order.
    UserDefault,
    /// Sorting contacts by given name.
    GivenName,
    /// Sorting contacts by family name.
    FamilyName,
}

object! {
    /// An immutable object that stores information about a single contact, such as the contact's first name, phone numbers, and addresses.
    unsafe pub struct CNContact;
}

#[interface_impl(NSObject)]
impl CNContact {
    /* Identifying the Contact
     */

    /// A value that uniquely identifies a contact on the device.
    #[property]
    pub fn identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), identifier]) }
    }

    /// An enum identifying the contact type.
    #[property]
    pub fn contact_type(&self) -> CNContactType {
        unsafe { msg_send![self.m_self(), contactType] }
    }

    /// The name prefix of the contact.
    #[property]
    pub fn name_prefix(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), namePrefix]) }
    }

    /// The given name of the contact.
    #[property]
    pub fn given_name(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), givenName]) }
    }

    /// The middle name of the contact.
    #[property]
    pub fn middle_name(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), middleName]) }
    }

    /// A string for the previous family name of the contact.
    #[property]
    pub fn family_name(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), familyName]) }
    }

    /// A string for the previous family name of the contact.
    #[property]
    pub fn previous_family_name(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), previousFamilyName]) }
    }

    /// The name suffix of the contact.
    #[property]
    pub fn name_suffix(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), nameSuffix]) }
    }

    /// The nickname of the contact.
    #[property]
    pub fn nickname(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), nickname]) }
    }

    /// The phonetic given name of the contact.
    #[property]
    pub fn phonetic_given_name(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), phoneticGivenName]) }
    }

    /// The phonetic middle name of the contact.
    #[property]
    pub fn phonetic_middle_name(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), phoneticMiddleName]) }
    }

    /// The phonetic family name of the contact.
    #[property]
    pub fn phonetic_family_name(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), phoneticFamilyName]) }
    }

    /* Getting Work Information
     */

    /// The contact’s job title.
    #[property]
    pub fn job_title(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), jobTitle]) }
    }

    /// The name of the department associated with the contact.
    #[property]
    pub fn department_name(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), departmentName]) }
    }

    /// The name of the organization associated with the contact.
    #[property]
    pub fn organization_name(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), organizationName]) }
    }

    /// The phonetic name of the organization associated with the contact.
    #[property]
    pub fn phonetic_organization_name(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), phoneticOrganizationName]) }
    }

    /* Getting Addresses
     */

    /// An array of labeled postal addresses for a contact.
    #[property]
    pub fn postal_addresses(&self) -> NSArray<CNLabeledValue<CNPostalAddress>> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), postalAddresses]) }
    }

    /// An array of labeled email addresses for the contact.
    #[property]
    pub fn email_addresses(&self) -> NSArray<CNLabeledValue<NSString>> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), emailAddresses]) }
    }

    /// An array of labeled URL addresses for a contact.
    #[property]
    pub fn url_addresses(&self) -> NSArray<CNLabeledValue<NSString>> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), urlAddresses]) }
    }

    /* Getting Phone Information
     */

    /// An array of labeled phone numbers for a contact.
    #[property]
    pub fn phone_numbers(&self) -> NSArray<CNLabeledValue<CNPhoneNumber>> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), phoneNumbers]) }
    }

    /* Getting Social Profiles
     */

    /// An array of labeled social profiles for a contact.
    #[property]
    pub fn social_profiles(&self) -> NSArray<CNLabeledValue<CNSocialProfile>> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), socialProfiles]) }
    }

    /// A date component for the Gregorian birthday of the contact.
    #[property]
    pub fn birthday(&self) -> Option<NSDateComponents> {
        unsafe {
            let ptr = msg_send![self.m_self(), birthday];

            if ptr != nil {
                Some(NSDateComponents::from_id(ptr))
            } else {
                None
            }
        }
    }

    /* Getting Birthday Information
     */

    /// A date component for the non-Gregorian birthday of the contact.
    #[property]
    pub fn non_gregorian_birthday(&self) -> Option<NSDateComponents> {
        unsafe {
            let ptr = msg_send![self.m_self(), nonGregorianBirthday];

            if ptr != nil {
                Some(NSDateComponents::from_id(ptr))
            } else {
                None
            }
        }
    }

    /// An array containing labeled Gregorian dates.
    #[property]
    pub fn dates(&self) -> NSArray<CNLabeledValue<NSDateComponents>> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), dates]) }
    }

    /// A string containing notes for the contact.
    #[property]
    pub fn note(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), note]) }
    }

    /* Getting Contact Images
     */

    /// The profile picture of a contact.
    #[property]
    pub fn image_data(&self) -> Option<NSData> {
        unsafe {
            let ptr = msg_send![self.m_self(), imageData];

            if ptr != nil {
                Some(NSData::from_id(ptr))
            } else {
                None
            }
        }
    }

    /// The thumbnail version of the contact’s profile picture.
    #[property]
    pub fn thumbnail_image_data(&self) -> Option<NSData> {
        unsafe {
            let ptr = msg_send![self.m_self(), thumbnailImageData];

            if ptr != nil {
                Some(NSData::from_id(ptr))
            } else {
                None
            }
        }
    }

    /// A Boolean indicating whether a contact has a profile picture.
    #[property]
    pub fn image_data_available(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), imageDataAvailable]) }
    }

    /* Getting Related Information
     */

    /// An array of labeled relations for the contact.
    #[property]
    pub fn contact_relations(&self) -> NSArray<CNLabeledValue<CNContactRelation>> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), contactRelations]) }
    }

    /// An array of labeled IM addresses for the contact.
    #[property]
    pub fn instant_messaging_addresses(&self) -> NSArray<CNLabeledValue<CNInstantMessageAddress>> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), instantMessageAddresses]) }
    }

    /* Localizing Contact Data
     */

    /// Returns a string containing the localized contact property name.
    #[method]
    pub fn localized_string_for_key(property: NSString) -> NSString {
        unsafe { NSString::from_id(msg_send![Self::m_class(), localizedStringForKey: property]) }
    }

    /* Comparing Contacts
     */

    /// Fetches all the keys required for the contact sort comparator.
    #[method]
    pub fn descriptor_for_all_comparator_keys() -> id {
        unsafe { msg_send![Self::m_class(), descriptorForAllComparatorKeys] }
    }

    /// Returns a comparator to sort contacts with the specified order.
    #[method]
    pub fn comparator_for_name_sort_order(sort_order: CNContactSortOrder) -> NSComparator {
        unsafe { msg_send![Self::m_class(), comparatorForNameSortOrder: sort_order] }
    }

    /// Returns a Boolean indicating whether the current contact is a unified contact and includes a contact with the specified identifier.
    #[method]
    pub fn is_unified_with_contact_with_identifier(&self, identifier: NSString) -> bool {
        unsafe {
            to_bool(msg_send![
                self.m_self(),
                isUnifiedWithContactWithIdentifier: identifier
            ])
        }
    }

    /// Determines whether the contact property value for the specified key is fetched.
    #[method]
    pub fn is_key_available(&self, key: NSString) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isKeyAvailable: key]) }
    }

    /// Determines whether all contact property values for the specified keys are fetched.
    #[method]
    pub fn are_keys_available(&self, key_descriptors: NSArray<id>) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), areKeysAvailable: key_descriptors]) }
    }

    /* Getting Search Predicates
     */

    /// Returns a predicate to find the contacts matching the specified name.
    #[method]
    pub fn predicate_for_contacts_matching_name(name: NSString) -> NSPredicate {
        unsafe {
            NSPredicate::from_id(msg_send![
                Self::m_class(),
                predicateForContactsMatchingName: name
            ])
        }
    }

    /// Returns a predicate to find the contacts matching the specified identifiers.
    #[method]
    pub fn predicate_for_contacts_with_identifiers(identifiers: NSArray<String>) -> NSPredicate {
        unsafe {
            NSPredicate::from_id(msg_send![
                Self::m_class(),
                predicateForContactsWithIdentifiers: identifiers
            ])
        }
    }

    /// Returns a predicate to find the contacts that are members in the specified group.
    #[method]
    pub fn predicate_for_contacts_in_group_with_identifier(group: NSString) -> NSPredicate {
        unsafe {
            NSPredicate::from_id(msg_send![
                Self::m_class(),
                predicateForContactsInGroupWithIdentifier: group
            ])
        }
    }

    /// Returns a predicate to find the contacts in the specified container.
    #[method]
    pub fn predicate_for_contacts_in_container_with_identifier(container: NSString) -> NSPredicate {
        unsafe {
            NSPredicate::from_id(msg_send![
                Self::m_class(),
                predicateForContactsInContainerWithIdentifier: container
            ])
        }
    }

    /// Returns a predicate to find the contacts whose phone number matches the specified value.
    #[method]
    pub fn predicate_for_contacts_matching_phone_number(
        phone_number: CNPhoneNumber,
    ) -> NSPredicate {
        unsafe {
            NSPredicate::from_id(msg_send![
                Self::m_class(),
                predicateForContactsMatchingPhoneNumber: phone_number
            ])
        }
    }

    /// Returns a predicate to find the contacts whose email address matches the specified value.
    #[method]
    pub fn predicate_for_contacts_matching_email_address(email_address: NSString) -> NSPredicate {
        unsafe {
            NSPredicate::from_id(msg_send![
                Self::m_class(),
                predicateForContactsMatchingEmailAddress: email_address
            ])
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{contacts::CNContactType, objective_c_runtime::traits::PNSObject};

    use super::CNContact;

    #[test]
    fn test_contact() {
        let contact = CNContact::m_new();

        assert!(contact.birthday().is_none());
        assert!(contact.contact_relations().count() == 0);
        assert!(contact.contact_type() == CNContactType::Person);
        assert!(contact.dates().count() == 0);
        assert!(contact.department_name() == "");
        assert!(contact.email_addresses().count() == 0);
        assert!(contact.family_name() == "");
        assert!(contact.given_name() == "");
        assert!(contact.identifier() != "");
        assert!(contact.image_data().is_none());
        assert!(!contact.image_data_available());
        assert!(contact.instant_messaging_addresses().count() == 0);
        assert!(contact.job_title() == "");
        assert!(contact.middle_name() == "");
        assert!(contact.name_prefix() == "");
        assert!(contact.name_suffix() == "");
        assert!(contact.nickname() == "");
        assert!(contact.non_gregorian_birthday().is_none());
        assert!(contact.note() == "");
        assert!(contact.organization_name() == "");
        assert!(contact.phone_numbers().count() == 0);
        assert!(contact.phonetic_given_name() == "");
        assert!(contact.phonetic_middle_name() == "");
        assert!(contact.phonetic_family_name() == "");
        assert!(contact.postal_addresses().count() == 0);
        assert!(contact.previous_family_name() == "");
        assert!(contact.social_profiles().count() == 0);
        assert!(contact.thumbnail_image_data().is_none());
        assert!(contact.url_addresses().count() == 0);
    }
}
