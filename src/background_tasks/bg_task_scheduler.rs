use std::fmt;

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::objective_c_runtime::{
    id,
    traits::{FromId, INSObject},
};

use super::traits::IBGTaskScheduler;

/// A class for scheduling task requests that launch your app in the background.
pub struct BGTaskScheduler {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl INSObject for BGTaskScheduler {
    fn new() -> Self {
        Self {
            ptr: unsafe { msg_send![class!(BGTaskScheduler), new] },
        }
    }

    fn to_id(mut self) -> id {
        &mut *self.ptr
    }

    unsafe fn from_id(obj: id) -> Self {
        Self {
            ptr: Id::from_ptr(obj),
        }
    }

    fn description(&self) -> crate::foundation::NSString {
        unsafe { crate::foundation::NSString::from_id(msg_send![self.ptr, description]) }
    }

    fn debug_description(&self) -> crate::foundation::NSString {
        unsafe { crate::foundation::NSString::from_id(msg_send![self.ptr, debugDescription]) }
    }

    fn retain(&self) -> Self {
        unsafe { Self::from_id(msg_send![self.ptr, retain]) }
    }
}

impl IBGTaskScheduler for BGTaskScheduler {
    fn tp_shared_scheduler() -> Self {
        unsafe { Self::from_id(msg_send![class!(BGTaskScheduler), sharedScheduler]) }
    }
}

impl fmt::Debug for BGTaskScheduler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.debug_description())
    }
}

impl fmt::Display for BGTaskScheduler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.description())
    }
}
