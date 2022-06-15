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

use super::traits::INSDate;

/// A representation of a specific point in time, independent of any calendar or time zone.
pub struct NSDate {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSDate {
    fn im_class<'a>() -> &'a Class {
        class!(NSDate)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl INSDate for NSDate {}

impl ToId for NSDate {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSDate {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}

impl fmt::Debug for NSDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.ip_description())
    }
}

impl Clone for NSDate {
    fn clone(&self) -> Self {
        unsafe { Self::from_id(msg_send![&*self.ptr, retain]) }
    }
}
