use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::{NSArray, NSBundle, NSDictionary, NSString},
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject},
    },
};

/// A representation of the code and resources stored in a bundle directory on disk.
pub trait INSBundle: PNSObject {
    /* Getting Standard Bundle Objects */

    /// Returns the bundle object that contains the current executable.
    fn tp_main_bundle() -> NSBundle {
        unsafe { NSBundle::from_id(msg_send![Self::im_class(), mainBundle]) }
    }

    /// Returns an array of all of the application’s bundles that represent frameworks.
    fn tp_all_frameworks() -> NSArray<NSBundle> {
        unsafe { NSArray::from_id(msg_send![Self::im_class(), allFrameworks]) }
    }

    /// Returns an array of all the application’s non-framework bundles.
    fn tp_all_bundles() -> NSArray<NSBundle> {
        unsafe { NSArray::from_id(msg_send![Self::im_class(), allBundles]) }
    }

    /// The full pathname of the bundle’s subdirectory containing resources.
    fn ip_resource_path(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), resourcePath]) }
    }

    /// The full pathname of the receiver's executable file.
    fn ip_executable_path(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), executablePath]) }
    }

    /// The full pathname of the receiver’s bundle directory.
    fn ip_bundle_path(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), bundlePath]) }
    }

    /// The receiver’s bundle identifier.
    fn ip_bundle_identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), bundleIdentifier]) }
    }

    /// A dictionary, constructed from the bundle’s Info.plist file, that contains information about the receiver.
    fn ip_info_dictionary(&self) -> NSDictionary<NSString, id> {
        unsafe { NSDictionary::from_id(msg_send![self.im_self(), infoDictionary]) }
    }
}
