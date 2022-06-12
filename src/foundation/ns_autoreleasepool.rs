use std::fmt::{Debug, Display};

use objc::{
    class, msg_send,
    runtime::{Class, Object, Protocol, Sel},
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

    fn im_is_equal(&self, object: &Self) -> bool {
        unsafe { msg_send![&*self.ptr, isEqual: object] }
    }

    fn ip_hash(&self) -> super::UInt {
        unsafe { msg_send![&*self.ptr, hash] }
    }

    fn im_is_kind_of_class(&self, class: Class) -> bool {
        unsafe { msg_send![&*self.ptr, isKindOfClass: class] }
    }

    fn im_is_member_of_class(&self, class: Class) -> bool {
        unsafe { msg_send![&*self.ptr, isMemberOfClass: class] }
    }

    fn im_responds_to_selector(&self, selector: Sel) -> bool {
        unsafe { msg_send![&*self.ptr, respondsToSelector: selector] }
    }

    fn im_conforms_to_protocol(&self, protocol: Protocol) -> bool {
        unsafe { msg_send![&*self.ptr, conformsToProtocol: protocol] }
    }

    fn ip_description(&self) -> super::NSString {
        unsafe { msg_send![&*self.ptr, description] }
    }

    fn ip_debug_description(&self) -> super::NSString {
        unsafe { msg_send![&*self.ptr, debugDescription] }
    }

    fn im_perform_selector(&self, selector: Sel) -> id {
        unsafe { msg_send![&*self.ptr, performSelector: selector] }
    }

    fn im_perform_selector_with_object(&self, selector: Sel, with_object: id) -> id {
        unsafe { msg_send![&*self.ptr, performSelector: selector withObject: with_object] }
    }

    fn im_is_proxy(&self) -> bool {
        unsafe { msg_send![&*self.ptr, isProxy] }
    }
}

impl INSAutoreleasePool for NSAutoreleasePool {
    fn im_drain(&mut self) {
        unsafe { msg_send![&*self.ptr, drain] }
    }

    fn im_add_object(&mut self, object: id) {
        unsafe { msg_send![&*self.ptr, addObject: object] }
    }
}

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
