use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

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

use super::traits::INSStatusItem;

/// An individual element displayed in the system menu bar.
pub struct NSStatusItem {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl NSStatusItem {
    /// Creates a new status item.
    pub fn new() -> Self {
        unsafe { Self::from_id(msg_send![NSStatusItem::im_class(), new]) }
    }
}

impl Default for NSStatusItem {
    fn default() -> Self {
        Self::new()
    }
}

impl PNSObject for NSStatusItem {
    fn im_class<'a>() -> &'a Class {
        class!(NSStatusItem)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![self.ptr, self] }
    }
}

impl INSStatusItem for NSStatusItem {}

impl Debug for NSStatusItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl Display for NSStatusItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl Deref for NSStatusItem {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.ptr
    }
}

impl DerefMut for NSStatusItem {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.ptr
    }
}

impl ToId for NSStatusItem {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSStatusItem {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}
