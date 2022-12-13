//! Access the user's contacts and format and localize contact information.

/* Essentials
 */

mod cn_contact_store;

pub use cn_contact_store::*;

/* Contact Data
 */
mod cn_contact;
pub use cn_contact::*;

mod cn_mutable_contact;
pub use cn_mutable_contact::*;

/* Contact Keys
***************/
extern "C" {
    /* Contact Identification
     */

    /// The contact’s unique identifier.
    pub static CNContactIdentifierKey: NSString;
    /// The type of contact.
    pub static CNContactTypeKey: NSString;
    /// The contact's name component property key.
    pub static CNContactPropertyAttribute: NSString;

    /* Name
     */

    /// The prefix for the contact's name.
    pub static CNContactNamePrefixKey: NSString;
    /// The contact's given name.
    pub static CNContactGivenNameKey: NSString;
    /// The contact's middle name.
    pub static CNContactMiddleNameKey: NSString;
    /// The contact's family name.
    pub static CNContactFamilyNameKey: NSString;
    /// The contact's previous family name.
    pub static CNContactPreviousFamilyNameKey: NSString;
    /// The contact's name suffix.
    pub static CNContactNameSuffixKey: NSString;
    /// The contact's nickname.
    pub static CNContactNicknameKey: NSString;
    /// The phonetic spelling of the contact's given name.
    pub static CNContactPhoneticGivenNameKey: NSString;
    /// The phonetic spelling of the contact's middle name.
    pub static CNContactPhoneticMiddleNameKey: NSString;
    /// The phonetic spelling of the contact's family name.
    pub static CNContactPhoneticFamilyNameKey: NSString;

    /* Work
     */

    /// The contact's job title.
    pub static CNContactJobTitleKey: NSString;
    /// The contact's department name.
    pub static CNContactDepartmentNameKey: NSString;
    /// The contact's organization name.
    pub static CNContactOrganizationNameKey: NSString;
    /// The phonetic spelling of the contact's organization name.
    pub static CNContactPhoneticOrganizationNameKey: NSString;

    /* Addresses
     */

    /// The postal addresses of the contact.
    pub static CNContactPostalAddressesKey: NSString;
    /// The email addresses of the contact.
    pub static CNContactEmailAddressesKey: NSString;
    /// The URL addresses of the contact.
    pub static CNContactUrlAddressesKey: NSString;
    /// The instant message addresses of the contact.
    pub static CNContactInstantMessageAddressesKey: NSString;

    /* Phone
     */

    /// A phone numbers of a contact.
    pub static CNContactPhoneNumbersKey: NSString;

    /* Social Profiles
     */

    /// A social profiles of a contact.
    pub static CNContactSocialProfilesKey: NSString;

    /* Birthday
     */

    /// The birthday of a contact.
    pub static CNContactBirthdayKey: NSString;
    /// The non-Gregorian birthday of the contact.
    pub static CNContactNonGregorianBirthdayKey: NSString;
    /// Dates associated with a contact.
    pub static CNContactDatesKey: NSString;

    /* Notes
     */

    /// A note associated with a contact.
    pub static CNContactNoteKey: NSString;

    /* Images
     */

    /// Image data for a contact.
    pub static CNContactImageDataKey: NSString;
    /// Thumbnail data for a contact.
    pub static CNContactThumbnailImageDataKey: NSString;
    /// Image data availability for a contact.
    pub static CNContactImageDataAvailableKey: NSString;

    /* Relationships
     */

    /// The relationships of the contact.
    pub static CNContactRelationsKey: NSString;

    /* Groups and Containers
     */

    /// The name of the group.
    pub static CNGroupNameKey: NSString;
    /// The identifier of the group.
    pub static CNGroupIdentifierKey: NSString;
    /// The name of the container.
    pub static CNContainerNameKey: NSString;
    /// The type of the container.
    pub static CNContainerTypeKey: NSString;

    /* Instant Messaging Keys
     */

    /// Instant message address service key.
    pub static CNInstantMessageAddressServiceKey: NSString;
    /// Instant message address username key.
    pub static CNInstantMessageAddressUsernameKey: NSString;

    /* Social Profile Keys
     */

    /// The social profile service.
    pub static CNSocialProfileServiceKey: NSString;
    /// The social profile URL.
    pub static CNSocialProfileURLStringKey: NSString;
    /// The social profile user name.
    pub static CNSocialProfileUsernameKey: NSString;
    /// The social profile user identifier.
    pub static CNSocialProfileUserIdentifierKey: NSString;
}

/* Date Objects
***************/

/* Addresses
 */
mod cn_postal_address;
pub use cn_postal_address::*;

mod cn_mutable_postal_address;
pub use cn_mutable_postal_address::*;

mod cn_instant_message_address;
pub use cn_instant_message_address::*;

/* Phone Numbers
 */

mod cn_phone_number;
pub use cn_phone_number::*;

/* Groups and Containers
 */

mod cn_group;
pub use cn_group::*;

mod cn_mutable_group;
pub use cn_mutable_group::*;

mod cn_container;
pub use cn_container::*;

/* Social Profiles
 */

mod cn_social_profile;
pub use cn_social_profile::*;

/* Related Data
 */

mod cn_contact_relation;
pub use cn_contact_relation::*;

/* Generic Types
 */

mod cn_labeled_value;
pub use cn_labeled_value::*;

mod cn_contact_property;
pub use cn_contact_property::*;

/* Fetch and Save Requests
 */

mod cn_fetch_request;
pub use cn_fetch_request::*;

mod cn_fetch_result;
pub use cn_fetch_result::*;

mod cn_contact_fetch_request;
pub use cn_contact_fetch_request::*;

mod cn_save_request;
pub use cn_save_request::*;

/* Change History Data
 */

mod cn_change_history_add_contact_event;
pub use cn_change_history_add_contact_event::*;

mod cn_change_history_add_group_event;
pub use cn_change_history_add_group_event::*;

mod cn_change_history_event;
pub use cn_change_history_event::*;

mod cn_change_history_fetch_request;
pub use cn_change_history_fetch_request::*;

/* Formatters
 */

mod cn_contact_formatter;
pub use cn_contact_formatter::*;

use crate::foundation::NSString;
