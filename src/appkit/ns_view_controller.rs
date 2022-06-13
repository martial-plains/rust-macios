use std::fmt;

use objc::{
    class, msg_send,
    runtime::{Class, Object, Sel},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{foundation::UInt, objective_c_runtime::traits::PNSObject, utils::to_bool};

use super::traits::INSResponder;

/// A controller that manages a view, typically loaded from a nib file.
pub struct NSViewController {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSViewController {
    fn im_class<'a>() -> &'a Class {
        class!(NSViewController)
    }

    fn im_is_equal(&self, object: &Self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isEqual: object]) }
    }

    fn ip_hash(&self) -> UInt {
        unsafe { msg_send![&*self.ptr, hash] }
    }

    fn im_is_kind_of_class(&self, class: Class) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isKindOfClass: class]) }
    }

    fn im_is_member_of_class(&self, class: Class) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isMemberOfClass: class]) }
    }

    fn im_responds_to_selector(&self, selector: Sel) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, respondsToSelector: selector]) }
    }

    fn im_conforms_to_protocol(&self, protocol: objc::runtime::Protocol) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, conformsToProtocol: protocol]) }
    }

    fn ip_description(&self) -> crate::foundation::NSString {
        unsafe { msg_send![&*self.ptr, description] }
    }

    fn ip_debug_description(&self) -> crate::foundation::NSString {
        unsafe { msg_send![&*self.ptr, debugDescription] }
    }

    fn im_perform_selector(&self, selector: Sel) -> crate::objective_c_runtime::id {
        unsafe { msg_send![&*self.ptr, performSelector: selector] }
    }

    fn im_perform_selector_with_object(
        &self,
        selector: Sel,
        with_object: crate::objective_c_runtime::id,
    ) -> crate::objective_c_runtime::id {
        unsafe { msg_send![&*self.ptr, performSelector: selector withObject: with_object] }
    }

    fn im_is_proxy(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isProxy]) }
    }
}

impl INSResponder for NSViewController {}

impl fmt::Debug for NSViewController {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSViewController {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}
