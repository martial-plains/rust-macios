use std::fmt;

use crate::{
    background_tasks::traits::IBGTask,
    objective_c_runtime::{id, traits::PNSObject},
};
use objc::{
    class, msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_id::Id;

/// An object representing a short task typically used to refresh content
/// that’s run while the app is in the background.
pub struct BGAppRefreshTask {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for BGAppRefreshTask {
    fn im_class<'a>() -> &'a Class {
        class!(BGAppRefreshTask)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl IBGTask for BGAppRefreshTask {}

impl fmt::Debug for BGAppRefreshTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for BGAppRefreshTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}
