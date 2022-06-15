use std::fmt::{Debug, Display};

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

/// An object that describes the attributes of a computer’s monitor or screen.
pub struct NSScreen {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSScreen {
    fn im_class<'a>() -> &'a Class {
        class!(NSScreen)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl ToId for NSScreen {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSScreen {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}

impl Debug for NSScreen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl Display for NSScreen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}
