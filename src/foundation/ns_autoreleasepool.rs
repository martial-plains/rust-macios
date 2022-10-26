use objc::{msg_send, sel, sel_impl};

use crate::objective_c_runtime::{
    id,
    macros::{interface_impl, object},
    traits::PNSObject,
};

object! {
    /// An object that supports Cocoa’s reference-counted memory management system.
    unsafe pub struct NSAutoreleasePool;
}

#[interface_impl(NSObject)]
impl NSAutoreleasePool {
    /* Managing a Pool
     */

    /// In a reference-counted environment, releases and pops the receiver; in
    /// a garbage-collected environment, triggers garbage collection if the
    /// memory allocated since the last collection is greater than the current
    /// threshold.
    #[method]
    pub fn drain(&mut self) {
        unsafe { msg_send![self.m_self(), drain] }
    }

    /// Adds a given object to the active autorelease pool in the current thread.
    ///
    /// # Arguments
    ///
    /// * `object` - The object to be added to the pool in the current thread.
    #[method]
    pub fn add_object(&mut self, object: id) {
        unsafe { msg_send![self.m_self(), addObject: object] }
    }
}

impl Default for NSAutoreleasePool {
    fn default() -> Self {
        Self::m_new()
    }
}
