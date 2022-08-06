use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::{NSArray, NSPredicate, NSString},
    objective_c_runtime::{id, macros::object, traits::FromId},
    utils::to_bool,
};

use super::{CNContactSortOrder, ICNFetchRequest};

object! {
    /// An object that defines the options to use when fetching contacts.
    unsafe pub struct CNContactFetchRequest;
}

impl ICNFetchRequest for CNContactFetchRequest {}

#[interface_impl(CNFetchRequest)]
impl CNContactFetchRequest {
    /* Creating a Fetch Request
     */

    /// Creates a fetch request for the specified keys.
    #[method]
    pub fn init_with_keys_to_fetch(&self, keys: NSArray<NSString>) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), fetchRequestWithKeys: keys]) }
    }

    /* Specifying the Search Predicate
     */

    /// The predicate to match contacts against.
    #[property]
    pub fn predicate(&self) -> NSPredicate {
        unsafe { NSPredicate::from_id(msg_send![self.m_self(), predicate]) }
    }

    /// Sets the predicate to match contacts against.
    #[property]
    pub fn set_predicate(&mut self, predicate: NSPredicate) {
        unsafe { msg_send![self.m_self(), setPredicate: predicate] }
    }

    /// Configuring the Fetch Options
    #[property]
    pub fn mutable_objects(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), mutableObjects]) }
    }

    /// Sets whether the fetch request should return mutable objects.
    #[property]
    pub fn set_mutable_objects(&mut self, mutable_objects: bool) {
        unsafe { msg_send![self.m_self(), setMutableObjects: mutable_objects] }
    }

    /// A Boolean value that indicates whether to return linked contacts as unified contacts.
    #[property]
    pub fn unify_results(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), unifyResults]) }
    }

    /// Sets whether to return linked contacts as unified contacts.
    #[property]
    pub fn set_unify_results(&mut self, unify_results: bool) {
        unsafe { msg_send![self.m_self(), setUnifyResults: unify_results] }
    }

    /// The sort order for contacts.
    #[property]
    pub fn sort_order(&self) -> CNContactSortOrder {
        unsafe { msg_send![self.m_self(), sortOrder] }
    }

    /* Specifying the Keys to Fetch
     */

    /// The properties to fetch in the returned contacts.
    #[property]
    pub fn keys_to_fetch(&self) -> NSArray<id> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), keysToFetch]) }
    }

    /// Sets the properties to fetch in the returned contacts.
    #[property]
    pub fn set_keys_to_fetch(&mut self, keys_to_fetch: NSArray<id>) {
        unsafe { msg_send![self.m_self(), setKeysToFetch: keys_to_fetch] }
    }
}
