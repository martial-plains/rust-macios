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
};

/// An object that manages the space above your app's custom content and either below or integrated with the window’s title bar.
#[repr(transparent)]
pub struct NSToolbar {
    /// The underlying `NSToolbar`.
    pub ptr: Id<Object>,
}

impl PNSObject for NSToolbar {
    fn im_class<'a>() -> &'a Class {
        class!(NSToolbar)
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

impl fmt::Debug for NSToolbar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSToolbar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl ToId for NSToolbar {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSToolbar {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}
