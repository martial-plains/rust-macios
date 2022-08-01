use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::objective_c_runtime::{
    id,
    macros::object,
    traits::{FromId, PNSObject},
};

use super::{NSArray, NSDictionary, NSString};

object! {
    /// A representation of the code and resources stored in a bundle directory on disk.
    unsafe pub struct NSBundle;
}

#[interface_impl(NSObject)]
impl NSBundle {
    /* Getting Standard Bundle Objects */

    /// Returns the bundle object that contains the current executable.
    #[property]
    pub fn main_bundle() -> NSBundle {
        unsafe { NSBundle::from_id(msg_send![Self::m_class(), mainBundle]) }
    }

    /// Returns an array of all of the application’s bundles that represent frameworks.
    #[property]
    pub fn all_frameworks() -> NSArray<NSBundle> {
        unsafe { NSArray::from_id(msg_send![Self::m_class(), allFrameworks]) }
    }

    /// Returns an array of all the application’s non-framework bundles.
    #[property]
    pub fn all_bundles() -> NSArray<NSBundle> {
        unsafe { NSArray::from_id(msg_send![Self::m_class(), allBundles]) }
    }

    /// The full pathname of the bundle’s subdirectory containing resources.
    #[property]
    pub fn resource_path(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), resourcePath]) }
    }

    /// The full pathname of the receiver's executable file.
    #[property]
    pub fn executable_path(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), executablePath]) }
    }

    /// The full pathname of the receiver’s bundle directory.
    #[property]
    pub fn bundle_path(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), bundlePath]) }
    }

    /// The receiver’s bundle identifier.
    #[property]
    pub fn bundle_identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), bundleIdentifier]) }
    }

    /// A dictionary, constructed from the bundle’s Info.plist file, that contains information about the receiver.
    #[property]
    pub fn info_dictionary(&self) -> NSDictionary<NSString, id> {
        unsafe { NSDictionary::from_id(msg_send![self.m_self(), infoDictionary]) }
    }
}
