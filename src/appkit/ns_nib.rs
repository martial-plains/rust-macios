use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::{NSArray, NSBundle, NSData, NSDictionary, NSString, NSURL},
    objective_c_runtime::{
        id, nil,
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};

use super::object;

pub type NSNibName = NSString;

object! {
    /// An object that manages an app’s menus.
    unsafe pub struct NSNib;
}

#[interface_impl(NSObject)]
impl NSNib {
    /* Initializing a Nib
     */

    /// Returns an [`NSNib`] object initialized to the nib file at the specified URL.
    #[deprecated = "use [`init_with_nib_data_bundle`] instead."]
    #[method]
    pub fn init_with_contents_of_url(&mut self, nib_file_url: NSURL) -> id {
        unsafe { msg_send![self.m_self(), initWithContentsOfURL: nib_file_url] }
    }

    /// Returns an [`NSNib`] object initialized to the nib file in the specified bundle.
    #[method]
    pub fn init_with_nib_named_bundle(&mut self, nib_name: NSNibName, bundle: &NSBundle) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(
                msg_send![self.m_self(), initWithNibNamed: nib_name bundle: bundle.m_self()],
            )
        }
    }

    /// Initializes an instance with nib data and specified bundle for locating resources.
    #[method]
    pub fn init_with_nib_data_bundle(&mut self, nib_data: &NSData, bundle: &NSBundle) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(
                msg_send![self.m_self(), initWithNibData: nib_data.m_self() bundle: bundle.m_self()],
            )
        }
    }

    /* Instantiating a Nib
     */

    /// Unarchives and instantiates the in-memory contents of the receiver's nib file, creating a distinct object tree and set of top level objects.
    #[deprecated = "use [`instantiate_with_owner_top_level_objects`] instead."]
    #[method]
    pub fn instantiate_nib_with_owner_top_level_objects<T>(
        &mut self,
        owner: id,
        top_lvl_objects: &NSArray<T>,
    ) -> bool {
        unsafe {
            to_bool(
                msg_send![self.m_self(), instantiateNibWithOwner: owner topLevelObjects: top_lvl_objects.m_self()],
            )
        }
    }

    /// Unarchives and instantiates the in-memory contents of the receiver's nib file, creating a distinct object tree and top level objects.
    #[deprecated = "use [`instantiate_with_owner_top_level_objects`] instead."]
    #[method]
    pub fn instantiate_nib_with_external_name_table<K, V>(
        &self,
        external_name_table: &NSDictionary<K, V>,
    ) -> bool {
        unsafe {
            to_bool(
                msg_send![self.m_self(), instantiateNibWithExternalNameTable: external_name_table.m_self()],
            )
        }
    }

    /// Instantiates objects in the nib file with the specified owner.
    #[method]
    pub fn instantiate_with_owner_top_level_objects<T>(
        &self,
        owner: id,
        top_lvl_objects: Option<NSArray<T>>,
    ) {
        unsafe {
            let top_lvl_objects = match top_lvl_objects {
                Some(value) => value.m_self(),
                None => nil,
            };

            msg_send![self.m_self(), instantiateWithOwner: owner topLevelObjects: top_lvl_objects]
        }
    }
}

extern "C" {
    /*Nib Loading Keys
     */

    /// The external object that is responsible for the instantiated nib.
    #[deprecated]
    pub static NSNibOwner: NSString;

    /// An NSMutableArray object that, if present, is populated with the top-level objects of the newly instantiated nib.
    #[deprecated]
    pub static NSNibTopLevelObjects: NSString;
}
