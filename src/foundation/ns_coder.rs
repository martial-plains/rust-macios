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

/// An abstract class that serves as the basis for objects that enable archiving and distribution of other objects.
pub struct NSCoder {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSCoder {
    fn class<'a>() -> &'a Class {
        class!(NSCoder)
    }

    fn superclass<'a>() -> &'a Class {
        unsafe { msg_send![Self::class(), superclass] }
    }

    fn isEqual(&self, object: &Self) -> bool {
        to_bool(unsafe { msg_send![self.ptr, isEqual: object] })
    }

    fn hash(&self) -> UInt {
        unsafe { msg_send![self.ptr, hash] }
    }

    fn isKindOfClass(&self, aClass: Class) -> bool {
        to_bool(unsafe { msg_send![self.ptr, isKindOfClass: aClass] })
    }

    fn isMemberOfClass(&self, aClass: Class) -> bool {
        to_bool(unsafe { msg_send![self.ptr, isMemberOfClass: aClass] })
    }

    fn respondsToSelector(&self, aSelector: Sel) -> bool {
        to_bool(unsafe { msg_send![self.ptr, respondsToSelector: aSelector] })
    }

    fn conformsToProtocol(&self, aProtocol: Protocol) -> bool {
        to_bool(unsafe { msg_send![self.ptr, conformsToProtocol: aProtocol] })
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
        to_bool(unsafe { msg_send![self.ptr, isProxy] })
    }
}

impl fmt::Debug for NSCoder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.debugDescription())
    }
}

impl fmt::Display for NSCoder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl ToId for NSCoder {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSCoder {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}
