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

use super::traits::{IBGProcessingTaskRequest, IBGTaskRequest};

/// A request to launch your app in the background to execute a processing task that can take minutes to complete.
pub struct BGProcessingTaskRequest {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl ToId for BGProcessingTaskRequest {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for BGProcessingTaskRequest {
    unsafe fn from_id(obj: id) -> Self {
        Self {
            ptr: Id::from_ptr(obj),
        }
    }
}

impl PNSObject for BGProcessingTaskRequest {
    fn im_class<'a>() -> &'a Class {
        class!(BGProcessingTaskRequest)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl IBGTaskRequest for BGProcessingTaskRequest {}

impl IBGProcessingTaskRequest for BGProcessingTaskRequest {}

impl fmt::Debug for BGProcessingTaskRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for BGProcessingTaskRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.ip_description())
    }
}
