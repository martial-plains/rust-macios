use std::fmt;

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::objective_c_runtime::{
    id,
    traits::{FromId, PNSObject, ToId},
};

use super::traits::{INSLayoutAnchor, INSLayoutYAxisAnchor};

/// A factory class for creating vertical layout constraint objects using a fluent API.
pub struct NSLayoutYAxisAnchor {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSLayoutYAxisAnchor {
    fn im_class<'a>() -> &'a objc::runtime::Class {
        class!(NSLayoutYAxisAnchor)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![self.ptr, self] }
    }
}

impl INSLayoutAnchor for NSLayoutYAxisAnchor {}

impl INSLayoutYAxisAnchor for NSLayoutYAxisAnchor {}

impl fmt::Debug for NSLayoutYAxisAnchor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSLayoutYAxisAnchor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl ToId for NSLayoutYAxisAnchor {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSLayoutYAxisAnchor {
    unsafe fn from_id(id: id) -> Self {
        NSLayoutYAxisAnchor {
            ptr: Id::from_ptr(id),
        }
    }
}
