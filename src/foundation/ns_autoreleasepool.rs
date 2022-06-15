use std::fmt::{Debug, Display};

use objc::{
    class, msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_id::Id;

use crate::objective_c_runtime::{
    id,
    traits::{PNSObject, ToId},
};

use super::traits::INSAutoreleasePool;

/// An object that supports Cocoa’s reference-counted memory management system.
pub struct NSAutoreleasePool {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl NSAutoreleasePool {
    /// Creates a new autorelease pool.
    pub fn new() -> Self {
        let ptr = unsafe { msg_send![class!(NSAutoreleasePool), new] };
        Self {
            ptr: unsafe { Id::from_retained_ptr(ptr) },
        }
    }

    /// In a reference-counted environment, releases and pops the receiver; in a garbage-collected environment, triggers garbage collection if the memory allocated since the last collection is greater than the current threshold.
    pub fn drain(&mut self) {
        self.im_drain();
    }

    /// Adds a given object to the active autorelease pool in the current thread.
    pub fn add_object<T>(&mut self, object: &T)
    where
        T: ToId + Clone,
    {
        self.im_add_object(object.clone().to_id());
    }
}

impl Default for NSAutoreleasePool {
    fn default() -> Self {
        Self::new()
    }
}

impl PNSObject for NSAutoreleasePool {
    fn im_class<'a>() -> &'a Class {
        class!(NSAutoreleasePool)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl INSAutoreleasePool for NSAutoreleasePool {}

impl Debug for NSAutoreleasePool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl Display for NSAutoreleasePool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}
