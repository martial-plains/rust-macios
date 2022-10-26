use objc::{msg_send, sel, sel_impl};

use super::{interface_impl, object, NSImageName};

use crate::objective_c_runtime::traits::PNSObject;

object! {
    /// A high-level interface for manipulating image data.
    unsafe pub struct NSImage;
}

#[interface_impl(NSObject)]
impl NSImage {
    /* Creating Images by Name
     */

    /// Returns the image object associated with the specified name.
    #[method]
    pub fn image_named(name: NSImageName) -> NSImage {
        unsafe { msg_send![NSImage::m_class(), imageNamed: name] }
    }
}
