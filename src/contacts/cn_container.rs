use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::{NSArray, NSPredicate, NSString},
    objective_c_runtime::{
        macros::{interface_impl, object},
        traits::{FromId, PNSObject},
    },
};

/// The container may be local on the device or associated with a server account that has contacts.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum CNContainerType {
    ///
    Unassigned = 0,
    /// A container for contacts only stored locally on the device. There is only one local container for a device.
    Local,
    /// A container for contacts stored in an Exchange folder from an Exchange server.
    Exchange,
    /// A container for contacts stored in an CardDAV server, such as iCloud.
    CardDav,
}

object! {
    /// An immutable object that represents a collection of contacts.
    unsafe pub struct CNContainer;
}

#[interface_impl(NSObject)]
impl CNContainer {
    /* Getting the Container Information
     */

    /// The name of the container.
    #[property]
    pub fn name(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), name]) }
    }

    /// The unique identifier for a contacts container on the device.
    #[property]
    pub fn identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), identifier]) }
    }

    /// The type of the container.
    #[property]
    pub fn type_(&self) -> CNContainerType {
        unsafe { msg_send![self.m_self(), type] }
    }

    /// Returns a predicate to find the container of the specified contact.
    #[method]
    pub fn predicate_for_container_of_contact_identifier(identifier: NSString) -> NSPredicate {
        unsafe {
            NSPredicate::from_id(msg_send![
                Self::m_class(),
                predicateForContainerOfContactWithIdentifier: identifier
            ])
        }
    }

    /// Returns a predicate to find the containers with the specified identifiers.
    #[method]
    pub fn predicate_for_containers_with_identifiers(
        identifiers: NSArray<NSString>,
    ) -> NSPredicate {
        unsafe {
            NSPredicate::from_id(msg_send![
                Self::m_class(),
                predicateForContainersWithIdentifiers: identifiers
            ])
        }
    }

    /// Returns a predicate to find the container of the specified group.
    #[method]
    pub fn predicate_for_container_of_group_with_identifier(identifier: NSString) -> NSPredicate {
        unsafe {
            NSPredicate::from_id(msg_send![
                Self::m_class(),
                predicateForContainerOfGroupWithIdentifier: identifier
            ])
        }
    }
}
