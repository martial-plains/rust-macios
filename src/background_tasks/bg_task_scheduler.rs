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

use super::traits::IBGTaskScheduler;

/// A class for scheduling task requests that launch your app in the background.
pub struct BGTaskScheduler {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for BGTaskScheduler {
    fn im_class<'a>() -> &'a Class {
        class!(BGTaskScheduler)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl IBGTaskScheduler for BGTaskScheduler {}

impl ToId for BGTaskScheduler {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for BGTaskScheduler {
    unsafe fn from_id(obj: id) -> Self {
        Self {
            ptr: Id::from_ptr(obj),
        }
    }
}

impl fmt::Debug for BGTaskScheduler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for BGTaskScheduler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.ip_description())
    }
}
