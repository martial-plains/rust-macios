use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use objc::{class, msg_send, runtime::Object, sel, sel_impl, Encode};
use objc_id::Id;

use crate::{
    foundation::traits::INSNotification,
    objective_c_runtime::{id, traits::PNSObject},
};

/// A container for information broadcast through a notification center to all registered observers.
pub struct NSNotification {
    obj: Id<Object>,
}

impl PNSObject for NSNotification {
    fn im_class<'a>() -> &'a objc::runtime::Class {
        class!(NSNotification)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![self.obj, self] }
    }
}

unsafe impl Encode for NSNotification {
    fn encode() -> objc::Encoding {
        unsafe { objc::Encoding::from_str("@") }
    }
}

impl fmt::Debug for NSNotification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSNotification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl INSNotification for NSNotification {}

impl Deref for NSNotification {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.obj
    }
}

impl DerefMut for NSNotification {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.obj
    }
}
