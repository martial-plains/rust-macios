use std::fmt;

use objc::{
    class, msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::{NSString, UInt},
    id,
    objective_c_runtime::traits::{FromId, PNSObject, ToId},
    utils::to_bool,
};

use super::{INSRunningApplication, NSApplicationActivationOptions};

/// An object that can manipulate and provide information for a single instance of an app.
pub struct NSRunningApplication {
    /// The underlying `NSRunningApplication` object.
    pub ptr: Id<Object>,
}

impl NSRunningApplication {
    /// Returns the application instance, creating it if it doesn’t exist yet.
    pub fn current_application() -> Self {
        Self::tp_currentApplication()
    }

    /// Indicates whether the application is currently frontmost.
    pub fn active(&self) -> bool {
        self.ip_isActive()
    }

    /// Attempts to activate the application using the specified options.

    pub fn activate_with_options(&mut self, options: NSApplicationActivationOptions) {
        self.im_activateWithOptions(options)
    }
}

impl PNSObject for NSRunningApplication {
    fn im_class<'a>() -> &'a Class {
        class!(NSRunningApplication)
    }

    fn im_isEqual(&self, object: &Self) -> bool {
        unsafe { msg_send![self.ptr, isEqual: object] }
    }

    fn ip_hash(&self) -> UInt {
        unsafe { msg_send![self.ptr, hash] }
    }

    fn im_isKindOfClass(&self, aClass: Class) -> bool {
        unsafe { msg_send![self.ptr, isKindOfClass: aClass] }
    }

    fn im_isMemberOfClass(&self, aClass: Class) -> bool {
        unsafe { msg_send![self.ptr, isMemberOfClass: aClass] }
    }

    fn im_respondsToSelector(&self, aSelector: Sel) -> bool {
        unsafe { msg_send![self.ptr, respondsToSelector: aSelector] }
    }

    fn im_conformsToProtocol(&self, aProtocol: Protocol) -> bool {
        unsafe { msg_send![self.ptr, conformsToProtocol: aProtocol] }
    }

    fn ip_description(&self) -> NSString {
        unsafe { msg_send![self.ptr, description] }
    }

    fn ip_debugDescription(&self) -> NSString {
        unsafe { msg_send![self.ptr, debugDescription] }
    }

    fn im_performSelector(&self, aSelector: Sel) -> id {
        unsafe { msg_send![self.ptr, performSelector: aSelector] }
    }

    fn im_performSelector_withObject(&self, aSelector: Sel, withObject: id) -> id {
        unsafe { msg_send![self.ptr, performSelector: aSelector withObject: withObject] }
    }

    fn im_isProxy(&self) -> bool {
        unsafe { msg_send![self.ptr, isProxy] }
    }
}

impl INSRunningApplication for NSRunningApplication {
    fn tp_currentApplication() -> Self {
        unsafe { Self::from_id(msg_send![Self::im_class(), currentApplication]) }
    }

    fn ip_isActive(&self) -> bool {
        to_bool(unsafe { msg_send![self.ptr, isActive] })
    }

    fn im_activateWithOptions(&mut self, options: NSApplicationActivationOptions) {
        unsafe { msg_send![self.ptr, activateWithOptions: options] }
    }
}

impl fmt::Debug for NSRunningApplication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debugDescription())
    }
}

impl fmt::Display for NSRunningApplication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl ToId for NSRunningApplication {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSRunningApplication {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}
