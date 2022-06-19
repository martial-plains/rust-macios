use core::fmt;

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

use super::traits::{INSControl, INSResponder, INSTextField, INSView};

/// Text the user can select or edit to send an action message to a target when the user presses the Return key.
pub struct NSTextField {
    /// The undrlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSTextField {
    fn im_class<'a>() -> &'a Class {
        class!(NSTextField)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![self.ptr, self] }
    }
}

impl INSResponder for NSTextField {}

impl INSView for NSTextField {}

impl INSControl for NSTextField {}

impl INSTextField for NSTextField {}

impl fmt::Debug for NSTextField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSTextField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl ToId for NSTextField {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSTextField {
    unsafe fn from_id(id: id) -> Self {
        Self {
            ptr: Id::from_ptr(id),
        }
    }
}

impl Clone for NSTextField {
    fn clone(&self) -> Self {
        unsafe { Self::from_id(msg_send![self.ptr, retain]) }
    }
}
