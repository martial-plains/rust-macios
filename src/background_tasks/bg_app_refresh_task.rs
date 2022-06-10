use std::fmt;

use crate::{
    background_tasks::traits::IBGTask,
    objective_c_runtime::{id, traits::FromId},
};
use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{foundation::NSString, objective_c_runtime::traits::INSObject};

/// An object representing a short task typically used to refresh content
/// that’s run while the app is in the background.
pub struct BGAppRefreshTask {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl INSObject for BGAppRefreshTask {
    fn new() -> Self {
        Self {
            ptr: unsafe { Id::from_ptr(msg_send![class!(BGAppRefreshTask), new]) },
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

    fn description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, description]) }
    }

    fn debug_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, debugDescription]) }
    }

    fn retain(&self) -> Self {
        unsafe { Self::from_id(msg_send![self.ptr, retain]) }
    }
}

impl IBGTask for BGAppRefreshTask {
    fn ip_identifier() -> NSString {
        unsafe { NSString::from_id(msg_send![class!(BGAppRefreshTask), identifier]) }
    }

    fn ip_expiration_handler() {
        unsafe { msg_send![class!(BGAppRefreshTask), expirationHandler] }
    }

    fn im_set_task_completed_with_success(&self, success: bool) {
        unsafe { msg_send![self.ptr, setTaskCompletedWithSuccess: success] }
    }
}

impl fmt::Debug for BGAppRefreshTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.debug_description())
    }
}

impl fmt::Display for BGAppRefreshTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
