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

use super::traits::INSImage;

/// A high-level interface for manipulating image data.
pub struct NSImage {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSImage {
    fn im_class<'a>() -> &'a Class {
        class!(NSImage)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl INSImage for NSImage {}

impl ToId for NSImage {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSImage {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}
impl fmt::Debug for NSImage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSImage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}
