use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

pub(crate) use crate::objective_c_runtime::macros::object;
use crate::{foundation::NSString, objective_c_runtime::traits::FromId};

use super::{INSControl, INSResponder, INSView, NSImage};

object! {
    /// A control that defines an area on the screen that a user clicks to trigger an action.
    unsafe pub struct NSButton;
}

impl INSResponder for NSButton {}

impl INSView for NSButton {}

impl INSControl for NSButton {}

#[interface_impl(NSControl)]
impl NSButton {
    /* Configuring Buttons
     */

    /// The title displayed on the button when it’s in an off state.
    #[property]
    pub fn title(&self) -> NSString {
        unsafe { msg_send![self.m_self(), title] }
    }

    /// Sets the title displayed on the button when it’s in an off state.
    #[property]
    pub fn set_title(&self, title: NSString) {
        unsafe { msg_send![self.m_self(), setTitle: title] }
    }

    /* Configuring Button Images
     */

    /// The image that appears on the button when it’s in an off state, or nil if there is no such image.
    #[property]
    pub fn image(&self) -> NSImage {
        unsafe { NSImage::from_id(msg_send![self.m_self(), image]) }
    }

    /// Sets the image that appears on the button when it’s in an off state, or nil if there is no such image.
    ///
    /// # Arguments
    ///
    /// * `image` - The new image.
    #[property]
    pub fn set_image(&self, image: NSImage) {
        unsafe { msg_send![self.m_self(), setImage: image] }
    }
}
