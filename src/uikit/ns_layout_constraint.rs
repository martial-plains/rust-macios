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

use super::traits::INSLayoutConstraint;

/// The relationship between two user interface objects that must be satisfied by the constraint-based layout system.
pub struct NSLayoutConstraint {
    /// The undrlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSLayoutConstraint {
    fn im_class<'a>() -> &'a Class {
        class!(NSLayoutConstraint)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![self.ptr, self] }
    }
}

impl INSLayoutConstraint for NSLayoutConstraint {}

impl fmt::Debug for NSLayoutConstraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSLayoutConstraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl ToId for NSLayoutConstraint {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSLayoutConstraint {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}
