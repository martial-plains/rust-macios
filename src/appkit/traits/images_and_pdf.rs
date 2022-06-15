use objc::{msg_send, sel, sel_impl};

use crate::{
    appkit::{NSImage, NSImageName},
    objective_c_runtime::traits::PNSObject,
};

/// A high-level interface for manipulating image data.
pub trait INSImage: PNSObject {
    /* Creating Images by Name
     */

    /// Returns the image object associated with the specified name.
    fn tm_image_named(name: NSImageName) -> NSImage {
        unsafe { msg_send![NSImage::im_class(), imageNamed: name] }
    }
}
