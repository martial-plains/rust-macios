use std::fmt;

use objc::{
    class, msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::NSString,
    id,
    objective_c_runtime::traits::{FromId, PNSObject},
};

/// A representation of a specific point in time, independent of any calendar or time zone.
pub struct NSDate {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSDate {
    fn class<'a>() -> &'a Class {
        class!(NSDate)
    }

    fn superclass<'a>() -> &'a Class {
        unsafe { msg_send![class!(NSDate), superclass] }
    }

    fn isEqual(&self, object: &Self) -> bool {
        unsafe { msg_send![self.ptr, isEqual: object] }
    }

    fn hash(&self) -> super::UInt {
        unsafe { msg_send![self.ptr, hash] }
    }

    fn isKindOfClass(&self, aClass: Class) -> bool {
        unsafe { msg_send![self.ptr, isKindOfClass: aClass] }
    }

    fn isMemberOfClass(&self, aClass: Class) -> bool {
        unsafe { msg_send![self.ptr, isMemberOfClass: aClass] }
    }

    fn respondsToSelector(&self, aSelector: Sel) -> bool {
        unsafe { msg_send![self.ptr, respondsToSelector: aSelector] }
    }

    fn conformsToProtocol(&self, aProtocol: Protocol) -> bool {
        unsafe { msg_send![self.ptr, conformsToProtocol: aProtocol] }
    }

    fn description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, description]) }
    }

    fn debugDescription(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, debugDescription]) }
    }

    fn performSelector(&self, aSelector: Sel) -> id {
        unsafe { msg_send![self.ptr, performSelector: aSelector] }
    }

    fn performSelector_withObject(&self, aSelector: Sel, withObject: id) -> id {
        unsafe { msg_send![self.ptr, performSelector: aSelector withObject: withObject] }
    }

    fn isProxy(&self) -> bool {
        unsafe { msg_send![self.ptr, isProxy] }
    }
}

impl FromId for NSDate {
    fn from_id(ptr: id) -> Self {
        Self {
            ptr: unsafe { Id::from_ptr(ptr) },
        }
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
        unsafe { Self::from_id(msg_send![self.ptr, copy]) }
    }
}
