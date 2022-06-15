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

use super::traits::{INSResponder, INSView};

/// The infrastructure for drawing, printing, and handling events in an app.
#[repr(transparent)]
pub struct NSView {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSView {
    fn im_class<'a>() -> &'a Class {
        class!(NSView)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![self.ptr, self] }
    }
}

impl INSResponder for NSView {}

impl INSView for NSView {}

impl fmt::Debug for NSView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl ToId for NSView {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSView {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}
