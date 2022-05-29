use std::fmt;

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{id, objective_c_runtime::traits::PNSObject, foundation::NSString};

/// A representation of a specific point in time, independent of any calendar or time zone.
pub struct NSDate {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSDate {
    fn new() -> Self {
        Self {
            ptr: unsafe { msg_send![class!(NSDate), new] },
        }
    }

    fn toId(mut self) -> id {
        &mut *self.ptr
    }

    unsafe fn fromId(obj: id) -> Self {
        Self {
            ptr: Id::from_ptr(obj),
        }
    }

    fn description(&self) -> NSString {
        unsafe { NSString::fromId(msg_send![self.ptr, description]) }
    }

    fn debugDescription(&self) -> NSString {
        unsafe { NSString::fromId(msg_send![self.ptr, debugDescription]) }
    }

    fn retain(&self) -> Self {
        unsafe { Self::fromId(msg_send![self.ptr, retain]) }
    }
}

impl fmt::Debug for NSDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.debugDescription())
    }
}

impl fmt::Display for NSDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.description())
    }
}

impl Clone for NSDate {
    fn clone(&self) -> Self {
        self.retain()
    }
}
