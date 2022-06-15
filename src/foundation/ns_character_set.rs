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

use super::traits::INSCharacterSet;

/// A character set containing the characters in Unicode General Categories L*, M*, and N*.
pub struct NSCharacterSet {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSCharacterSet {
    fn im_class<'a>() -> &'a Class {
        class!(NSCharacterSet)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl INSCharacterSet for NSCharacterSet {}

impl ToId for NSCharacterSet {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSCharacterSet {
    unsafe fn from_id(id: id) -> Self {
        Self {
            ptr: Id::from_ptr(id),
        }
    }
}

impl fmt::Debug for NSCharacterSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSCharacterSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}
