use block::{ConcreteBlock, IntoConcreteBlock};
use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::{NSArray, NSData, NSEnumerator, NSError, NSPredicate, NSString},
    objective_c_runtime::{
        id,
        macros::object,
        nil,
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};

use super::{
    CNChangeHistoryEvent, CNChangeHistoryFetchRequest, CNContact, CNContactFetchRequest,
    CNContainer, CNFetchResult, CNGroup, CNSaveRequest,
};

/// The entities the user can grant access to.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum CNEntityType {
    /// The user's contacts.
    Contacts,
}

/// An authorization status the user can grant for an app to access the specified entity type.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum CNAuthorizationStatus {
    /// The user has not yet made a choice regarding whether the application may access contact data.
    NotDetermined = 0,
    /// The application is not authorized to access contact data. The user cannot change this application’s status, possibly due to active restrictions such as parental controls being in place.
    Restricted,
    /// The user explicitly denied access to contact data for the application.
    Denied,
    /// The application is authorized to access contact data.
    Authorized,
}

object! {
    /// The object that fetches and saves contacts, groups, and containers from the user's contacts database.
    unsafe pub struct CNContactStore;
}

#[interface_impl(NSObject)]
impl CNContactStore {
    /// Requests access to the user's contacts.
    #[method]
    pub fn request_access_for_entity_type_completion_handler<F>(
        &self,
        entity_type: CNEntityType,
        completion_handler: F,
    ) where
        F: IntoConcreteBlock<(bool, NSError), Ret = ()> + 'static,
    {
        let block = ConcreteBlock::new(completion_handler);
        let block = block.copy();

        unsafe {
            msg_send![
                self.m_self(),
                requestAccessForEntityType: entity_type
                completionHandler: block
            ]
        }
    }

    /// Returns the current authorization status to access the contact data.
    #[method]
    pub fn authorization_status_for_entity_type(
        entity_type: CNEntityType,
    ) -> CNAuthorizationStatus {
        unsafe {
            msg_send![
                Self::m_class(),
                authorizationStatusForEntityType: entity_type
            ]
        }
    }

    /* Fetching Contacts
     */

    /// Returns a Boolean value that indicates whether the enumeration of all contacts matching a contact fetch request executed successfully.
    #[method]
    pub fn enumerate_contacts_with_fetch_request_using_block<F>(
        &self,
        fetch_request: CNContactFetchRequest,
        block: F,
    ) -> Result<bool, NSError>
    where
        F: IntoConcreteBlock<(CNContact, bool), Ret = ()> + 'static,
    {
        let mut error = NSError::m_alloc();
        let block = ConcreteBlock::new(block);
        let block = block.copy();

        unsafe {
            let ptr = to_bool(msg_send![
                self.m_self(),
                enumerateContactsWithFetchRequest: fetch_request
                error: &mut error
                usingBlock: block
            ]);

            if error.m_self() == nil {
                Ok(to_bool(ptr))
            } else {
                Err(error)
            }
        }
    }

    /// Fetches the unified contact that’s the me card.
    #[method]
    pub fn unified_me_contact_with_keys_to_fetch(
        &self,
        keys: NSArray<id>,
    ) -> Result<CNContact, NSError> {
        let mut error = NSError::m_alloc();

        unsafe {
            let result = CNContact::from_id(msg_send![
                self.m_self(),
                unifiedMeContactWithKeysToFetch: keys
                error: &mut error
            ]);

            if error.m_self() == nil {
                Ok(result)
            } else {
                Err(error)
            }
        }
    }

    /// Fetches a unified contact for the specified contact identifier.
    #[method]
    pub fn unified_contact_with_identifier_keys_to_fetch(
        &self,
        identifier: NSString,
        keys: NSArray<id>,
    ) -> Result<CNContact, NSError> {
        let mut error = NSError::m_alloc();
        unsafe {
            let result = CNContact::from_id(msg_send![
                self.m_self(),
                unifiedContactWithIdentifier: identifier
                keysToFetch: keys
                error: &mut error
            ]);

            if error.m_self() == nil {
                Ok(result)
            } else {
                Err(error)
            }
        }
    }

    /// Fetches all unified contacts matching the specified predicate.
    #[method]
    pub fn unified_contacts_matching_predicate_keys_to_fetch(
        &self,
        predicate: NSPredicate,
        keys: NSArray<id>,
    ) -> Result<NSArray<CNContact>, NSError> {
        unsafe {
            let mut error = NSError::m_alloc();
            let result = NSArray::from_id(msg_send![
                self.m_self(),
                unifiedContactsMatchingPredicate: predicate
                keysToFetch: keys
                error: &mut error
            ]);

            if error.m_self() == nil {
                Ok(result)
            } else {
                Err(error)
            }
        }
    }

    /// Enumerates a contact fetch request.
    #[method]
    pub fn enumerator_for_contact_fetch_request(
        &self,
        fetch_request: CNContactFetchRequest,
    ) -> Result<CNFetchResult<NSEnumerator<CNContact>>, NSError> {
        unsafe {
            let mut error = NSError::m_alloc();
            let result = CNFetchResult::from_id(msg_send![
                self.m_self(),
                enumeratorForContactFetchRequest: fetch_request
                error: &mut error
            ]);

            if error.m_self() == nil {
                Ok(result)
            } else {
                Err(error)
            }
        }
    }

    /* Fetching Groups and Containers
     */

    /// Returns the identifier of the default container.
    #[method]
    pub fn default_container_identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), defaultContainerIdentifier]) }
    }

    /// Fetches all groups matching the specified predicate.
    #[method]
    pub fn groups_matching_predicate(
        &self,
        predicate: NSPredicate,
    ) -> Result<NSArray<CNGroup>, NSError> {
        unsafe {
            let mut error = NSError::m_alloc();

            let result = NSArray::from_id(msg_send![
                self.m_self(),
                groupsMatchingPredicate: predicate
                error: &mut error
            ]);

            if error.m_self() == nil {
                Ok(result)
            } else {
                Err(error)
            }
        }
    }

    /// Fetches all containers matching the specified predicate.
    #[method]
    pub fn containers_matching_predicate(
        &self,
        predicate: NSPredicate,
    ) -> Result<NSArray<CNContainer>, NSError> {
        unsafe {
            let mut error = NSError::m_alloc();
            let result = NSArray::from_id(msg_send![
                self.m_self(),
                containersMatchingPredicate: predicate
                error: &mut error
            ]);

            if error.m_self() == nil {
                Ok(result)
            } else {
                Err(error)
            }
        }
    }

    /* Fetching Change History Info
     */

    /// Enumerates a change history fetch request.
    #[method]
    pub fn enumerator_for_change_history_fetch_request(
        &self,
        request: CNChangeHistoryFetchRequest,
    ) -> Result<CNFetchResult<NSEnumerator<CNChangeHistoryEvent>>, NSError> {
        unsafe {
            let mut error = NSError::m_alloc();
            let result = CNFetchResult::from_id(msg_send![
                self.m_self(),
                enumeratorForChangeHistoryFetchRequest: request
                error: &mut error
            ]);

            if result.m_self() == nil {
                Ok(result)
            } else {
                Err(error)
            }
        }
    }

    /// The current history token.
    #[property]
    pub fn current_history_token(&self) -> NSData {
        unsafe { NSData::from_id(msg_send![self.m_self(), currentHistoryToken]) }
    }

    /* Saving Changes
     */

    /// Executes a save request and returns success or failure.
    #[method]
    pub fn execute_save_request(&self, request: CNSaveRequest) -> Result<bool, NSError> {
        unsafe {
            let mut error = NSError::m_alloc();

            let ptr =
                to_bool(msg_send![self.m_self(), executeSaveRequest: request error: &mut error]);

            if error.m_self() != nil {
                Err(error)
            } else {
                Ok(ptr)
            }
        }
    }
}

extern "C" {
    /// The shared contact store.
    pub static CNContactStoreDidChangeNotification: *const NSString;
}
