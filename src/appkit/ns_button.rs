use std::fmt;

use objc::{
    class, msg_send,
    runtime::{Class, Object, Sel},
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

use super::INSResponder;

/// A control that defines an area on the screen that a user clicks to trigger an action.
#[repr(transparent)]
pub struct NSButton {
    /// The underlying `NSButton`.
    pub ptr: Id<Object>,
}

impl PNSObject for NSButton {
    fn im_class<'a>() -> &'a Class {
        class!(NSButton)
    }

    fn im_is_equal(&self, object: &Self) -> bool {
        unsafe { to_bool(msg_send![self.ptr, isEqual: object]) }
    }

    fn ip_hash(&self) -> UInt {
        unsafe { msg_send![self.ptr, hash] }
    }

    fn im_is_kind_of_class(&self, class: Class) -> bool {
        to_bool(unsafe { msg_send![self.ptr, isKindOfClass: class] })
    }

    fn im_is_member_of_class(&self, class: Class) -> bool {
        to_bool(unsafe { msg_send![self.ptr, isMemberOfClass: class] })
    }

    fn im_responds_to_selector(&self, selector: Sel) -> bool {
        to_bool(unsafe { msg_send![self.ptr, respondsToSelector: selector] })
    }

    fn im_conforms_to_protocol(&self, protocol: objc::runtime::Protocol) -> bool {
        to_bool(unsafe { msg_send![self.ptr, conformsToProtocol: protocol] })
    }

    fn ip_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, description]) }
    }

    fn ip_debug_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, debugDescription]) }
    }

    fn im_perform_selector(&self, selector: Sel) -> id {
        unsafe { msg_send![self.ptr, performSelector: selector] }
    }

    fn im_perform_selector_with_object(&self, selector: Sel, with_object: id) -> id {
        unsafe { msg_send![self.ptr, performSelector: selector withObject: with_object] }
    }

    fn im_is_proxy(&self) -> bool {
        to_bool(unsafe { msg_send![self.ptr, isProxy] })
    }
}

impl fmt::Debug for NSButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl INSResponder for NSButton {}

impl ToId for NSButton {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSButton {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}
