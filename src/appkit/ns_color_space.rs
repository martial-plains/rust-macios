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

/// An object that represents a custom color space.
pub struct NSColorSpace {
    ptr: Id<Object>,
}

impl ToId for NSColorSpace {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSColorSpace {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}

impl PNSObject for NSColorSpace {
    fn im_class<'a>() -> &'a Class {
        class!(NSColorSpace)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl fmt::Debug for NSColorSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSColorSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}
