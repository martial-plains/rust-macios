use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::objective_c_runtime::traits::{FromId, PNSObject};

use super::{object, NSApplicationActivationOptions};

object! {
    /// An object that can manipulate and provide information for a single instance of an app.
    unsafe pub struct NSRunningApplication;
}

#[interface_impl(NSObject)]
impl NSRunningApplication {
    /// Returns the application instance, creating it if it doesn’t exist yet.
    #[property]
    pub fn current_application() -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), currentApplication]) }
    }

    /// Indicates whether the application is currently frontmost.
    #[property]
    pub fn is_active(&self) -> bool {
        unsafe { msg_send![self.m_self(), isActive] }
    }

    /// Attempts to activate the application using the specified options.
    #[method]
    pub fn activate_with_options(&mut self, options: NSApplicationActivationOptions) {
        unsafe { msg_send![self.m_self(), activateWithOptions: options] }
    }
}
