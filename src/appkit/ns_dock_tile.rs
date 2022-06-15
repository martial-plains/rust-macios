use core::fmt;

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

/// The visual representation of your app’s miniaturized windows and app icon as they appear in the Dock.
pub struct NSDockTile {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSDockTile {
    fn im_class<'a>() -> &'a Class {
        class!(NSDockTile)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl ToId for NSDockTile {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSDockTile {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}

impl fmt::Debug for NSDockTile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSDockTile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}
