use std::fmt;

use objc::{
    class, msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_id::Id;

use crate::objective_c_runtime::{id, traits::PNSObject};

/// A request to launch your app in the background to execute a short refresh task.
pub struct BGAppRefreshTaskRequest {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for BGAppRefreshTaskRequest {
    fn im_class<'a>() -> &'a Class {
        class!(BGAppRefreshTaskRequest)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl fmt::Debug for BGAppRefreshTaskRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for BGAppRefreshTaskRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.ip_description())
    }
}
