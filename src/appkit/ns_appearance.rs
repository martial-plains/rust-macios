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

#[repr(transparent)]
pub struct NSAppearance {
    ptr: Id<Object>,
}

impl PNSObject for NSAppearance {
    fn im_class<'a>() -> &'a Class {
        class!(NSAppearance)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![self.ptr, self] }
    }
}

impl fmt::Debug for NSAppearance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSAppearance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl ToId for NSAppearance {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSAppearance {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}
