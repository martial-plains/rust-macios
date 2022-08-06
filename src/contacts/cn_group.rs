use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::{NSArray, NSPredicate, NSString},
    objective_c_runtime::{
        macros::object,
        traits::{FromId, PNSObject},
    },
};

object! {
    /// An immutable object that represents a group of contacts.
    unsafe pub struct CNGroup;
}

#[interface_impl(NSObject)]
impl CNGroup {
    /// The name of the group.
    #[property]
    pub fn name(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), name]) }
    }

    /// The unique identifier for a group on the device.
    #[property]
    pub fn identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), identifier]) }
    }

    /* Generating Search Predicates for Groups
     */

    /// Returns a predicate to find groups with the specified identifiers.
    #[method]
    pub fn predicate_for_groups_with_identifiers(identifiers: NSArray<NSString>) -> NSPredicate {
        unsafe {
            NSPredicate::from_id(msg_send![
                CNGroup::m_class(),
                predicateForGroupsWithIdentifiers: identifiers
            ])
        }
    }

    /// Returns a predicate to find groups in the specified container.
    #[method]
    pub fn predicate_for_groups_in_container_with_identifiers(container: NSString) -> NSPredicate {
        unsafe {
            NSPredicate::from_id(msg_send![
                CNGroup::m_class(),
                predicateForGroupsInContainer: container
            ])
        }
    }

    /// Returns a predicate to find subgroups in the specified parent group.
    #[method]
    pub fn predicate_for_subgroups_in_group_with_identifier(parent_group: NSString) -> NSPredicate {
        unsafe {
            NSPredicate::from_id(msg_send![
                CNGroup::m_class(),
                predicateForSubgroupsInGroup: parent_group
            ])
        }
    }
}
