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

impl IBGTaskRequest for BGProcessingTaskRequest {
    fn ip_earliest_begin_date() -> crate::foundation::NSDate {
        unsafe {
            crate::foundation::NSDate::from_id(msg_send![
                class!(BGProcessingTaskRequest),
                earliestBeginDate
            ])
        }
    }
}

impl IBGProcessingTaskRequest for BGProcessingTaskRequest {
    fn im_init_with_identifier(identifier: NSString) -> Self {
        unsafe {
            Self::from_id(msg_send![
                class!(BGProcessingTaskRequest),
                initWithIdentifier: identifier
            ])
        }
    }

    fn ip_requires_external_power() -> bool {
        unsafe { msg_send![class!(BGProcessingTaskRequest), requiresExternalPower] }
    }

    fn ip_requires_network_connectivity() -> bool {
        unsafe { msg_send![class!(BGProcessingTaskRequest), requiresNetworkConnectivity] }
    }
}

impl fmt::Debug for BGProcessingTaskRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.debug_description())
    }
}

impl fmt::Display for BGProcessingTaskRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.description())
    }
}
