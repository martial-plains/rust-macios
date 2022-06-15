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

/// A representation of the code and resources stored in a bundle directory on disk.
pub struct NSBundle {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSBundle {
    fn im_class<'a>() -> &'a Class {
        class!(NSBundle)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl Debug for NSBundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl Display for NSBundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl ToId for NSBundle {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSBundle {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}
