use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::NSString,
    objective_c_runtime::{
        macros::object,
        nil,
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};

use super::{CNMutableContact, CNMutableGroup, ICNContact, ICNGroup};

object! {
    /// An object that collects the changes you want to save to the user's contacts database.
    unsafe pub struct CNSaveRequest;
}

#[interface_impl(NSObject)]
impl CNSaveRequest {
    /* Saving a Contact Changes
     */

    /// Adds the specified contact to the contact store.
    ///
    /// # Arguments
    ///
    /// * `contact` - The contact to add to the contact store.
    /// * `container_identifier` - The identifier of the container to add the new contact. To add the new contact to the default container set identifier to [`None`].
    #[method]
    pub fn add_contact_to_container_with_identifier<Contact>(
        &self,
        contact: Contact,
        identifier: Option<NSString>,
    ) where
        Contact: ICNContact,
    {
        match identifier {
            Some(identifier) => unsafe {
                msg_send![self.m_self(), addContact: contact.m_self() toContainerWithIdentifier: identifier]
            },
            None => unsafe {
                msg_send![self.m_self(), addContact: contact.m_self() toContainerWithIdentifier: nil]
            },
        }
    }

    /// Updates an existing contact in the contact store.
    #[method]
    pub fn update_contact(&self, contact: CNMutableContact) {
        unsafe { msg_send![self.m_self(), updateContact: contact] }
    }

    /// Removes the specified contact from the contact store.
    #[method]
    pub fn delete_contact(&self, contact: CNMutableContact) {
        unsafe { msg_send![self.m_self(), deleteContact: contact] }
    }

    /* Saving Group Changes */

    /// Adds a group to the contact store.
    #[method]
    pub fn add_group_to_container_with_identifier(
        &self,
        group: CNMutableGroup,
        identifier: NSString,
    ) {
        unsafe { msg_send![self.m_self(), addGroup: group toContainerWithIdentifier: identifier] }
    }

    /// Updates an existing group in the contact store.
    #[method]
    pub fn update_group(&self, group: CNMutableGroup) {
        unsafe { msg_send![self.m_self(), updateGroup: group] }
    }

    /// Deletes a group from the contact store.
    #[method]
    pub fn delete_group(&self, group: CNMutableGroup) {
        unsafe { msg_send![self.m_self(), deleteGroup: group] }
    }

    /// Adds a contact as a member of a group.
    #[method]
    pub fn add_member_to_group<Contact, Group>(&self, contact: Contact, group: Group)
    where
        Contact: ICNContact,
        Group: ICNGroup,
    {
        unsafe { msg_send![self.m_self(), addMember: contact toGroup: group] }
    }

    /// Removes a contact from a group.
    #[method]
    pub fn remove_member_from_group<Contact, Group>(&self, contact: Contact, group: Group)
    where
        Contact: ICNContact,
        Group: ICNGroup,
    {
        unsafe { msg_send![self.m_self(), removeMember: contact fromGroup: group] }
    }

    /* Adding and Removing Subgroups
     */

    /// Add the specified group to a parent group.
    #[method]
    pub fn add_subgroup_to_group<Group, ParentGroup>(&self, group: Group, parent_group: ParentGroup)
    where
        Group: ICNGroup,
        ParentGroup: ICNGroup,
    {
        unsafe { msg_send![self.m_self(), addSubgroup: group toGroup: parent_group] }
    }

    /// Remove a subgroup from the specified parent group.
    #[method]
    pub fn remove_subgroup_from_group<Group, ParentGroup>(
        &self,
        group: Group,
        parent_group: ParentGroup,
    ) where
        Group: ICNGroup,
        ParentGroup: ICNGroup,
    {
        unsafe { msg_send![self.m_self(), removeSubgroup: group fromGroup: parent_group] }
    }

    /* Instance Properties
     */

    ///
    #[property]
    pub fn should_refetch_contacts(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), shouldRefetchContacts]) }
    }

    ///
    #[property]
    pub fn transaction_author(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), transactionAuthor]) }
    }
}
