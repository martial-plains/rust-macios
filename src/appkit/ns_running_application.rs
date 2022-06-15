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

use super::{traits::INSRunningApplication, NSApplicationActivationOptions};

/// An object that can manipulate and provide information for a single instance of an app.
pub struct NSRunningApplication {
    /// The underlying `NSRunningApplication` object.
    pub ptr: Id<Object>,
}

impl NSRunningApplication {
    /// Returns the application instance, creating it if it doesn’t exist yet.
    pub fn current_application() -> Self {
        Self::tp_current_application()
    }

    /// Indicates whether the application is currently frontmost.
    pub fn active(&self) -> bool {
        self.ip_is_active()
    }

    /// Attempts to activate the application using the specified options.

    pub fn activate_with_options(&mut self, options: NSApplicationActivationOptions) {
        self.im_activate_with_options(options)
    }
}

impl PNSObject for NSRunningApplication {
    fn im_class<'a>() -> &'a Class {
        class!(NSRunningApplication)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl INSRunningApplication for NSRunningApplication {}

impl fmt::Debug for NSRunningApplication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSRunningApplication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl ToId for NSRunningApplication {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSRunningApplication {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}
