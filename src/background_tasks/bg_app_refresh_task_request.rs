use std::fmt;

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    foundation::NSString,
    id,
    objective_c_runtime::traits::{FromId, INSObject},
};

/// A request to launch your app in the background to execute a short refresh task.
pub struct BGAppRefreshTaskRequest {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl INSObject for BGAppRefreshTaskRequest {
    fn new() -> Self {
        Self {
            ptr: unsafe { msg_send![class!(BGAppRefreshTaskRequest), new] },
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

    fn description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, description]) }
    }

    fn debugDescription(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, debugDescription]) }
    }

    fn retain(&self) -> Self {
        unsafe { Self::fromId(msg_send![self.ptr, retain]) }
    }
}

impl fmt::Debug for BGAppRefreshTaskRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.debugDescription())
    }
}

impl fmt::Display for BGAppRefreshTaskRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.description())
    }
}
