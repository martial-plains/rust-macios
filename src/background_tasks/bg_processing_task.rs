use std::fmt;

use objc::{
    class, msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_id::Id;

use crate::objective_c_runtime::{id, traits::PNSObject};

use super::traits::{IBGProcessingTask, IBGTask};

/// A time-consuming processing task that runs while the app is in the background.
pub struct BGProcessingTask {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for BGProcessingTask {
    fn im_class<'a>() -> &'a Class {
        class!(BGProcessingTask)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl IBGTask for BGProcessingTask {}

impl IBGProcessingTask for BGProcessingTask {}

impl fmt::Debug for BGProcessingTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for BGProcessingTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}
