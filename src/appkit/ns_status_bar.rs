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

use super::traits::INSStatusBar;

/// An individual element displayed in the system menu bar.
pub struct NSStatusBar {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSStatusBar {
    fn im_class<'a>() -> &'a Class {
        class!(NSStatusBar)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![self.ptr, self] }
    }
}

impl INSStatusBar for NSStatusBar {}

impl ToId for NSStatusBar {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSStatusBar {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}

impl fmt::Debug for NSStatusBar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSStatusBar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}
