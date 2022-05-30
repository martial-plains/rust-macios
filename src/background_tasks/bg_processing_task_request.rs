use std::fmt;

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    foundation::NSString,
    objective_c_runtime::traits::{FromId, INSObject},
};

use super::traits::{IBGProcessingTaskRequest, IBGTaskRequest};

/// A request to launch your app in the background to execute a processing task that can take minutes to complete.
pub struct BGProcessingTaskRequest {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl INSObject for BGProcessingTaskRequest {
    fn new() -> Self {
        Self {
            ptr: unsafe { msg_send![class!(BGProcessingTaskRequest), new] },
        }
    }

    fn toId(mut self) -> crate::id {
        &mut *self.ptr
    }

    unsafe fn fromId(obj: crate::id) -> Self {
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

impl IBGTaskRequest for BGProcessingTaskRequest {
    fn earliestBeginDate() -> crate::foundation::NSDate {
        unsafe {
            crate::foundation::NSDate::from_id(msg_send![
                class!(BGProcessingTaskRequest),
                earliestBeginDate
            ])
        }
    }
}

impl IBGProcessingTaskRequest for BGProcessingTaskRequest {
    fn initWithIdentifier(identifier: NSString) -> Self {
        unsafe {
            Self::fromId(msg_send![
                class!(BGProcessingTaskRequest),
                initWithIdentifier: identifier
            ])
        }
    }

    fn requiresExternalPower() -> bool {
        unsafe { msg_send![class!(BGProcessingTaskRequest), requiresExternalPower] }
    }

    fn requiresNetworkConnectivity() -> bool {
        unsafe { msg_send![class!(BGProcessingTaskRequest), requiresNetworkConnectivity] }
    }
}

impl fmt::Debug for BGProcessingTaskRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.debugDescription())
    }
}

impl fmt::Display for BGProcessingTaskRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.description())
    }
}
