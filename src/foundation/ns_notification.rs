use std::ops::{Deref, DerefMut};

use objc::{runtime::Object, Encode};

use crate::{
    foundation::traits::INSNotification,
    objective_c_runtime::{macros::object, traits::PNSObject},
};

object! {
    /// A container for information broadcast through a notification center to all registered observers.
    unsafe pub struct NSNotification;
}

unsafe impl Encode for NSNotification {
    fn encode() -> objc::Encoding {
        unsafe { objc::Encoding::from_str("@") }
    }
}

impl INSNotification for NSNotification {}

impl Deref for NSNotification {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        unsafe { &*self.im_self() }
    }
}

impl DerefMut for NSNotification {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        unsafe { &mut *self.im_self() }
    }
}
