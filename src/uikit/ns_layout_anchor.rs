use core::fmt;

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::objective_c_runtime::{id, traits::PNSObject};

use super::traits::INSLayoutAnchor;

/// A factory class for creating layout constraint objects using a fluent API.
pub struct NSLayoutAnchor {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSLayoutAnchor {
    fn im_class<'a>() -> &'a objc::runtime::Class {
        class!(NSLayoutAnchor)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![self.ptr, self] }
    }
}

impl INSLayoutAnchor for NSLayoutAnchor {}

impl fmt::Debug for NSLayoutAnchor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSLayoutAnchor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}
