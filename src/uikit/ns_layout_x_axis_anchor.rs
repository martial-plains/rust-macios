use std::fmt;

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::objective_c_runtime::{
    id,
    traits::{FromId, PNSObject, ToId},
};

use super::traits::{INSLayoutAnchor, INSLayoutXAxisAnchor};

/// A factory class for creating horizontal layout constraint objects using a fluent API.
pub struct NSLayoutXAxisAnchor {
    /// The undrlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSLayoutXAxisAnchor {
    fn im_class<'a>() -> &'a objc::runtime::Class {
        class!(NSLayoutXAxisAnchor)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![self.ptr, self] }
    }
}

impl INSLayoutAnchor for NSLayoutXAxisAnchor {}

impl INSLayoutXAxisAnchor for NSLayoutXAxisAnchor {}

impl fmt::Debug for NSLayoutXAxisAnchor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSLayoutXAxisAnchor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl ToId for NSLayoutXAxisAnchor {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSLayoutXAxisAnchor {
    unsafe fn from_id(id: id) -> Self {
        Self {
            ptr: Id::from_ptr(id),
        }
    }
}
