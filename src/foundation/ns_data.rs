use std::fmt::{Debug, Display};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    id,
    objective_c_runtime::traits::{FromId, PNSObject, ToId},
};

use super::{traits::INSData, NSString};

/// A static byte buffer in memory.
pub struct NSData {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSData {
    fn class<'a>() -> &'a objc::runtime::Class {
        class!(NSData)
    }

    fn superclass<'a>() -> &'a objc::runtime::Class {
        unsafe { msg_send![Self::class(), superclass] }
    }

    fn isEqual(&self, object: &Self) -> bool {
        unsafe { msg_send![self.ptr, isEqual: object] }
    }

    fn hash(&self) -> super::UInt {
        unsafe { msg_send![self.ptr, hash] }
    }

    fn isKindOfClass(&self, aClass: objc::runtime::Class) -> bool {
        unsafe { msg_send![self.ptr, isKindOfClass: aClass] }
    }

    fn isMemberOfClass(&self, aClass: objc::runtime::Class) -> bool {
        unsafe { msg_send![self.ptr, isMemberOfClass: aClass] }
    }

    fn respondsToSelector(&self, aSelector: objc::runtime::Sel) -> bool {
        unsafe { msg_send![self.ptr, respondsToSelector: aSelector] }
    }

    fn conformsToProtocol(&self, aProtocol: objc::runtime::Protocol) -> bool {
        unsafe { msg_send![self.ptr, conformsToProtocol: aProtocol] }
    }

    fn description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, description]) }
    }

    fn debugDescription(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, debugDescription]) }
    }

    fn performSelector(&self, aSelector: objc::runtime::Sel) -> id {
        unsafe { msg_send![self.ptr, performSelector: aSelector] }
    }

    fn performSelector_withObject(&self, aSelector: objc::runtime::Sel, withObject: id) -> id {
        unsafe { msg_send![self.ptr, performSelector: aSelector withObject: withObject] }
    }

    fn isProxy(&self) -> bool {
        unsafe { msg_send![self.ptr, isProxy] }
    }
}

impl INSData for NSData {
    fn data() -> Self {
        let ptr = unsafe {
            let ptr = msg_send![class!(NSData), data];
            Id::from_ptr(ptr)
        };
        Self { ptr }
    }

    fn dataWithBytesLength(bytes: *const libc::c_void, length: super::UInt) -> Self {
        let ptr = unsafe {
            let ptr = msg_send![class!(NSData), dataWithBytes:bytes length:length];
            Id::from_ptr(ptr)
        };
        Self { ptr }
    }

    fn dataWithBytesNoCopyLength(bytes: *const libc::c_void, length: super::UInt) -> Self {
        let ptr = unsafe {
            let ptr = msg_send![class!(NSData), dataWithBytesNoCopy:bytes length:length];
            Id::from_ptr(ptr)
        };
        Self { ptr }
    }

    fn dataWithBytesNoCopyLengthFreeWhenDone(
        bytes: *const libc::c_void,
        length: super::UInt,
        b: bool,
    ) -> Self {
        let ptr = unsafe {
            let ptr =
                msg_send![class!(NSData), dataWithBytesNoCopy:bytes length:length freeWhenDone:b];
            Id::from_ptr(ptr)
        };
        Self { ptr }
    }

    fn dataWithData(data: NSData) -> Self {
        let ptr = unsafe {
            let ptr = msg_send![class!(NSData), dataWithData:data.ptr];
            Id::from_ptr(ptr)
        };
        Self { ptr }
    }

    fn dataWithContentsOfFile(path: NSString) -> Self {
        let ptr = unsafe {
            let ptr = msg_send![class!(NSData), dataWithContentsOfFile: path];
            Id::from_ptr(ptr)
        };
        Self { ptr }
    }

    fn bytes(&self) -> *const libc::c_void {
        unsafe { msg_send![self.ptr, bytes] }
    }
}

impl ToId for NSData {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSData {
    unsafe fn from_id(id: id) -> Self {
        Self {
            ptr: Id::from_ptr(id),
        }
    }
}

impl Debug for NSData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.debugDescription())
    }
}

impl Display for NSData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}
