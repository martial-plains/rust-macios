use std::fmt;

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    foundation::NSString,
    objective_c_runtime::{
        id,
        traits::{FromId, INSObject},
    },
};

use super::traits::{IBGProcessingTask, IBGTask};

/// A time-consuming processing task that runs while the app is in the background.
pub struct BGProcessingTask {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl INSObject for BGProcessingTask {
    fn new() -> Self {
        Self {
            ptr: unsafe { msg_send![class!(BGProcessingTask), new] },
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

impl IBGTask for BGProcessingTask {
    fn ip_identifier() -> NSString {
        unsafe { NSString::from_id(msg_send![class!(BGProcessingTask), identifier]) }
    }

    fn ip_expiration_handler() {
        unsafe { msg_send![class!(BGProcessingTask), expirationHandler] }
    }

    fn im_set_task_completed_with_success(&self, success: bool) {
        unsafe { msg_send![self.ptr, setTaskCompletedWithSuccess: success] }
    }
}

impl IBGProcessingTask for BGProcessingTask {}

impl fmt::Debug for BGProcessingTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.debug_description())
    }
}

impl fmt::Display for BGProcessingTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
