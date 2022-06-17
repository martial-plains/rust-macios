use std::fmt;

use objc::{
    class, msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_id::Id;

use crate::objective_c_runtime::{
    id,
    traits::{FromId, PNSObject, ToId},
};

use super::traits::{INSPopover, INSResponder};

/// A means to display additional content related to existing content on the screen.
pub struct NSPopover {
    /// The underlying C object.
    pub ptr: Id<Object>,
}

impl NSPopover {
    /// Creates a new popover.
    pub fn new() -> Self {
        unsafe {
            let obj = msg_send![class!(NSPopover), new];
            Self {
                ptr: Id::from_ptr(obj),
            }
        }
    }
}

impl Default for NSPopover {
    fn default() -> Self {
        Self::new()
    }
}

impl PNSObject for NSPopover {
    fn im_class<'a>() -> &'a Class {
        class!(NSPopover)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl INSResponder for NSPopover {}

impl INSPopover for NSPopover {}

impl fmt::Debug for NSPopover {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSPopover {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl ToId for NSPopover {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSPopover {
    unsafe fn from_id(ptr: id) -> NSPopover {
        NSPopover {
            ptr: Id::from_ptr(ptr),
        }
    }
}

impl Clone for NSPopover {
    fn clone(&self) -> Self {
        unsafe { Self::from_id(msg_send![&*self.ptr, retain]) }
    }
}
