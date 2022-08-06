use block::{ConcreteBlock, IntoConcreteBlock};
use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::{NSArray, NSData, NSEnumerator, NSError, NSPredicate, NSString},
    objective_c_runtime::{
        id,
        macros::object,
        traits::{FromId, PNSObject},
    },
};

use super::{
    CNAuthorizationStatus, CNChangeHistoryEvent, CNChangeHistoryFetchRequest, CNContact,
    CNContactFetchRequest, CNContainer, CNEntityType, CNFetchResult, CNGroup, CNSaveRequest,
};

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
    pub fn enumerate_contacts_with_fetch_request_error_using_block<F>(
        &self,
        fetch_request: CNContactFetchRequest,
        error: &mut NSError,
        block: F,
    ) where
        F: IntoConcreteBlock<(CNContact, bool), Ret = ()> + 'static,
    {
        let block = ConcreteBlock::new(block);
        let block = block.copy();

        unsafe {
            msg_send![
                self.m_self(),
                enumerateContactsWithFetchRequest: fetch_request
                error: error
                usingBlock: block
            ]
        }
    }

    /// Fetches the unified contact that’s the me card.
    #[method]
    pub fn unified_me_contact_with_keys_to_fetch_error(
        &self,
        keys: NSArray<id>,
        error: &mut NSError,
    ) -> CNContact {
        unsafe {
            CNContact::from_id(msg_send![
                self.m_self(),
                unifiedMeContactWithKeysToFetch: keys
                error: error
            ])
        }
    }

    /// Fetches a unified contact for the specified contact identifier.
    #[method]
    pub fn unified_contact_with_identifier_keys_to_fetch_error(
        &self,
        identifier: NSString,
        keys: NSArray<id>,
        error: &mut NSError,
    ) -> CNContact {
        unsafe {
            CNContact::from_id(msg_send![
                self.m_self(),
                unifiedContactWithIdentifier: identifier
                keysToFetch: keys
                error: error
            ])
        }
    }

    /// Fetches all unified contacts matching the specified predicate.
    #[method]
    pub fn unified_contacts_matching_predicate_keys_to_fetch_error(
        &self,
        predicate: NSPredicate,
        keys: NSArray<id>,
        error: &mut NSError,
    ) -> NSArray<CNContact> {
        unsafe {
            NSArray::from_id(msg_send![
                self.m_self(),
                unifiedContactsMatchingPredicate: predicate
                keysToFetch: keys
                error: error
            ])
        }
    }

    /// Enumerates a contact fetch request.
    #[method]
    pub fn enumerator_for_contact_fetch_request_error(
        &self,
        fetch_request: CNContactFetchRequest,
        error: &mut NSError,
    ) -> CNFetchResult<NSEnumerator<CNContact>> {
        unsafe {
            CNFetchResult::from_id(msg_send![
                self.m_self(),
                enumeratorForContactFetchRequest: fetch_request
                error: error
            ])
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
    pub fn groups_matching_predicate_error(
        &self,
        predicate: NSPredicate,
        error: &mut NSError,
    ) -> NSArray<CNGroup> {
        unsafe {
            NSArray::from_id(msg_send![
                self.m_self(),
                groupsMatchingPredicate: predicate
                error: error
            ])
        }
    }

    /// Fetches all containers matching the specified predicate.
    #[method]
    pub fn containers_matching_predicate_error(
        &self,
        predicate: NSPredicate,
        error: &mut NSError,
    ) -> NSArray<CNContainer> {
        unsafe {
            NSArray::from_id(msg_send![
                self.m_self(),
                containersMatchingPredicate: predicate
                error: error
            ])
        }
    }

    /* Fetching Change History Info
     */

    /// Enumerates a change history fetch request.
    #[method]
    pub fn enumerator_for_change_history_fetch_request_error(
        &self,
        request: CNChangeHistoryFetchRequest,
        error: &mut NSError,
    ) -> CNFetchResult<NSEnumerator<CNChangeHistoryEvent>> {
        unsafe {
            CNFetchResult::from_id(msg_send![
                self.m_self(),
                enumeratorForChangeHistoryFetchRequest: request
                error: error
            ])
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
    pub fn execute_save_request_error(&self, request: CNSaveRequest, error: &mut NSError) -> bool {
        unsafe { msg_send![self.m_self(), executeSaveRequest: request error: error] }
    }
}

extern "C" {
    /// The shared contact store.
    pub static CNContactStoreDidChangeNotification: *const NSString;
}
