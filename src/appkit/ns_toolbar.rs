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

/// An object that manages the space above your app's custom content and either below or integrated with the window’s title bar.
#[repr(transparent)]
pub struct NSToolbar {
    /// The underlying `NSToolbar`.
    pub ptr: Id<Object>,
}

impl PNSObject for NSToolbar {
    fn im_class<'a>() -> &'a Class {
        class!(NSToolbar)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl fmt::Debug for NSToolbar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSToolbar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl ToId for NSToolbar {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSToolbar {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}
