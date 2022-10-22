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
