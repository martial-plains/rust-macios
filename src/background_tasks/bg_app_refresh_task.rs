use crate::background_tasks::traits::IBGTask;
use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{foundation::NSString, objective_c_runtime::traits::PNSObject};

/// An object representing a short task typically used to refresh content
/// that’s run while the app is in the background.
pub struct BGAppRefreshTask {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for BGAppRefreshTask {
    fn new() -> Self {
        Self {
            ptr: unsafe { msg_send![class!(BGAppRefreshTask), new] },
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
        unsafe { NSString::fromId(msg_send![self.ptr, description]) }
    }

    fn debugDescription(&self) -> NSString {
        unsafe { NSString::fromId(msg_send![self.ptr, debugDescription]) }
    }

    fn retain(&self) -> Self {
        unsafe { Self::fromId(msg_send![self.ptr, retain]) }
    }
}

impl IBGTask for BGAppRefreshTask {
    fn identifier() -> NSString {
        unsafe { NSString::fromId(msg_send![class!(BGAppRefreshTask), identifier]) }
    }

    fn expirationHandler() {
        unsafe { msg_send![class!(BGAppRefreshTask), expirationHandler] }
    }

    fn setTaskCompletedWithSuccess(&self, success: bool) {
        unsafe { msg_send![self.ptr, setTaskCompletedWithSuccess: success] }
    }
}
