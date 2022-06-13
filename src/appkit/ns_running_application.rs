use std::fmt;

use objc::{
    class, msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::{NSString, UInt},
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject, ToId},
    },
    utils::to_bool,
};

use super::{NSApplicationActivationOptions, traits::INSRunningApplication};

/// An object that can manipulate and provide information for a single instance of an app.
pub struct NSRunningApplication {
    /// The underlying `NSRunningApplication` object.
    pub ptr: Id<Object>,
}

impl NSRunningApplication {
    /// Returns the application instance, creating it if it doesn’t exist yet.
    pub fn current_application() -> Self {
        Self::tp_current_application()
    }

    /// Indicates whether the application is currently frontmost.
    pub fn active(&self) -> bool {
        self.ip_is_active()
    }

    /// Attempts to activate the application using the specified options.

    pub fn activate_with_options(&mut self, options: NSApplicationActivationOptions) {
        self.im_activate_with_options(options)
    }
}

impl PNSObject for NSRunningApplication {
    fn im_class<'a>() -> &'a Class {
        class!(NSRunningApplication)
    }

    fn im_is_equal(&self, object: &Self) -> bool {
        unsafe { msg_send![&*self.ptr, isEqual: object] }
    }

    fn ip_hash(&self) -> UInt {
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

    fn ip_description(&self) -> NSString {
        unsafe { msg_send![&*self.ptr, description] }
    }

    fn ip_debug_description(&self) -> NSString {
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

impl INSRunningApplication for NSRunningApplication {
    fn ip_is_active(&self) -> bool {
        to_bool(unsafe { msg_send![&*self.ptr, isActive] })
    }

    fn im_activate_with_options(&mut self, options: NSApplicationActivationOptions) {
        unsafe { msg_send![&*self.ptr, activateWithOptions: options] }
    }
}

impl fmt::Debug for NSRunningApplication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
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
