use std::fmt::{Debug, Display};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::objective_c_runtime::traits::PNSObject;

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
}

impl Default for NSAutoreleasePool {
    fn default() -> Self {
        Self::new()
    }
}

impl PNSObject for NSAutoreleasePool {
    fn im_class<'a>() -> &'a objc::runtime::Class {
        class!(NSAutoreleasePool)
    }

    fn im_isEqual(&self, object: &Self) -> bool {
        unsafe { msg_send![self.ptr, isEqual: object] }
    }

    fn ip_hash(&self) -> super::UInt {
        unsafe { msg_send![self.ptr, hash] }
    }

    fn im_isKindOfClass(&self, aClass: objc::runtime::Class) -> bool {
        unsafe { msg_send![self.ptr, isKindOfClass: aClass] }
    }

    fn im_isMemberOfClass(&self, aClass: objc::runtime::Class) -> bool {
        unsafe { msg_send![self.ptr, isMemberOfClass: aClass] }
    }

    fn im_respondsToSelector(&self, aSelector: objc::runtime::Sel) -> bool {
        unsafe { msg_send![self.ptr, respondsToSelector: aSelector] }
    }

    fn im_conformsToProtocol(&self, aProtocol: objc::runtime::Protocol) -> bool {
        unsafe { msg_send![self.ptr, conformsToProtocol: aProtocol] }
    }

    fn ip_description(&self) -> super::NSString {
        unsafe { msg_send![self.ptr, description] }
    }

    fn ip_debugDescription(&self) -> super::NSString {
        unsafe { msg_send![self.ptr, debugDescription] }
    }

    fn im_performSelector(&self, aSelector: objc::runtime::Sel) -> crate::id {
        unsafe { msg_send![self.ptr, performSelector: aSelector] }
    }

    fn im_performSelector_withObject(
        &self,
        aSelector: objc::runtime::Sel,
        withObject: crate::id,
    ) -> crate::id {
        unsafe { msg_send![self.ptr, performSelector: aSelector withObject: withObject] }
    }

    fn im_isProxy(&self) -> bool {
        unsafe { msg_send![self.ptr, isProxy] }
    }
}

impl INSAutoreleasePool for NSAutoreleasePool {
    fn im_drain(&mut self) {
        unsafe { msg_send![self.ptr, drain] }
    }

    fn im_addObject(&mut self, object: crate::id) {
        unsafe { msg_send![self.ptr, addObject: object] }
    }
}

impl Debug for NSAutoreleasePool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip_debugDescription())
    }
}

impl Display for NSAutoreleasePool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}
