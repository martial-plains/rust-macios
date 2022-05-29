use std::fmt;

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{id, objective_c_runtime::traits::PNSObject};

use super::traits::IBGTaskScheduler;

/// A class for scheduling task requests that launch your app in the background.
pub struct BGTaskScheduler {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for BGTaskScheduler {
    fn new() -> Self {
        Self {
            ptr: unsafe { msg_send![class!(BGTaskScheduler), new] },
        }
    }

    fn toId(mut self) -> id {
        &mut *self.ptr
    }

    unsafe fn fromId(obj: id) -> Self {
        Self {
            ptr: Id::from_ptr(obj),
        }
    }

    fn description(&self) -> crate::foundation::NSString {
        unsafe { crate::foundation::NSString::fromId(msg_send![self.ptr, description]) }
    }

    fn debugDescription(&self) -> crate::foundation::NSString {
        unsafe { crate::foundation::NSString::fromId(msg_send![self.ptr, debugDescription]) }
    }

    fn retain(&self) -> Self {
        unsafe { Self::fromId(msg_send![self.ptr, retain]) }
    }
}

impl IBGTaskScheduler for BGTaskScheduler {
    fn sharedScheduler() -> Self {
        unsafe { Self::fromId(msg_send![class!(BGTaskScheduler), sharedScheduler]) }
    }
}

impl fmt::Debug for BGTaskScheduler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.debugDescription())
    }
}

impl fmt::Display for BGTaskScheduler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.description())
    }
}
