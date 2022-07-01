use objc::{class, msg_send, sel, sel_impl};

use crate::objective_c_runtime::traits::{FromId, PNSObject};

use super::{
    object,
    traits::{INSPopover, INSResponder},
};

object! {
    /// A means to display additional content related to existing content on the screen.
    unsafe pub struct NSPopover;
}

impl NSPopover {
    /// Creates a new popover.
    pub fn new() -> Self {
        unsafe { Self::from_id(msg_send![class!(NSPopover), new]) }
    }
}

impl Default for NSPopover {
    fn default() -> Self {
        Self::new()
    }
}

impl INSResponder for NSPopover {}

impl INSPopover for NSPopover {}

impl Clone for NSPopover {
    fn clone(&self) -> Self {
        unsafe { Self::from_id(msg_send![self.im_self(), retain]) }
    }
}
