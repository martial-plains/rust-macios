use objc::{msg_send, sel, sel_impl};

use crate::objective_c_runtime::{
    macros::object,
    traits::{PNSObject, ToId},
};

use super::traits::INSAutoreleasePool;

object! {
    /// An object that supports Cocoa’s reference-counted memory management system.
    unsafe pub struct NSAutoreleasePool;
}

impl NSAutoreleasePool {
    /// Creates a new autorelease pool.
    pub fn new() -> Self {
        unsafe { msg_send![Self::im_class(), new] }
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

impl INSAutoreleasePool for NSAutoreleasePool {}
