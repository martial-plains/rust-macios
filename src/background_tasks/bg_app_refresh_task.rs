use std::fmt;

use crate::{
    background_tasks::traits::IBGTask,
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};
use objc::{
    class, msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use objc_id::Id;

use crate::foundation::NSString;

/// An object representing a short task typically used to refresh content
/// that’s run while the app is in the background.
pub struct BGAppRefreshTask {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for BGAppRefreshTask {
    fn im_class<'a>() -> &'a Class {
        class!(BGAppRefreshTask)
    }

    fn im_is_equal(&self, object: &Self) -> bool {
        unsafe { to_bool(msg_send![self.ptr, isEqual: object]) }
    }

    fn ip_hash(&self) -> crate::foundation::UInt {
        unsafe { msg_send![self.ptr, hash] }
    }

    fn im_is_kind_of_class(&self, class: Class) -> bool {
        unsafe { to_bool(msg_send![self.ptr, isKindOfClass: class]) }
    }

    fn im_is_member_of_class(&self, class: Class) -> bool {
        unsafe { to_bool(msg_send![self.ptr, isMemberOfClass: class]) }
    }

    fn im_responds_to_selector(&self, selector: Sel) -> bool {
        unsafe { to_bool(msg_send![self.ptr, respondsToSelector: selector]) }
    }

    fn im_conforms_to_protocol(&self, protocol: Protocol) -> bool {
        unsafe { to_bool(msg_send![self.ptr, conformsToProtocol: protocol]) }
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
        unsafe { to_bool(msg_send![self.ptr, isProxy]) }
    }
}

impl IBGTask for BGAppRefreshTask {
    fn ip_identifier() -> NSString {
        unsafe { NSString::from_id(msg_send![class!(BGAppRefreshTask), identifier]) }
    }

    fn ip_expiration_handler() {
        unsafe { msg_send![class!(BGAppRefreshTask), expirationHandler] }
    }

    fn im_set_task_completed_with_success(&self, success: bool) {
        unsafe { msg_send![&*self.ptr, setTaskCompletedWithSuccess: success] }
    }
}

impl fmt::Debug for BGAppRefreshTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for BGAppRefreshTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}
