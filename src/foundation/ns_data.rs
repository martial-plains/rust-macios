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

use super::traits::INSData;

/// A static byte buffer in memory.
pub struct NSData {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSData {
    fn im_class<'a>() -> &'a Class {
        class!(NSData)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl INSData for NSData {
    fn ip_bytes(&self) -> *const libc::c_void {
        unsafe { msg_send![&*self.ptr, bytes] }
    }
}

impl ToId for NSData {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSData {
    unsafe fn from_id(id: id) -> Self {
        Self {
            ptr: Id::from_ptr(id),
        }
    }
}

impl Debug for NSData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl Display for NSData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}
