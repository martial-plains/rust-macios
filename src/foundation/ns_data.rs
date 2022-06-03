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
    fn im_class<'a>() -> &'a objc::runtime::Class {
        class!(NSData)
    }

    fn im_isEqual(&self, object: &Self) -> bool {
        unsafe { msg_send![self.ptr, isEqual: object] }
    }

    fn ip_hash(&self) -> super::UInt {
        unsafe { msg_send![self.ptr, hash] }
    }

    fn im_isKindOfClass(&self, aClass: objc::runtime::Class) -> bool {
        unsafe { msg_send![self.ptr, isKindOfClass: aClass] }
    }

    fn im_isMemberOfClass(&self, aClass: objc::runtime::Class) -> bool {
        unsafe { msg_send![self.ptr, isMemberOfClass: aClass] }
    }

    fn im_respondsToSelector(&self, aSelector: objc::runtime::Sel) -> bool {
        unsafe { msg_send![self.ptr, respondsToSelector: aSelector] }
    }

    fn im_conformsToProtocol(&self, aProtocol: objc::runtime::Protocol) -> bool {
        unsafe { msg_send![self.ptr, conformsToProtocol: aProtocol] }
    }

    fn ip_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, description]) }
    }

    fn ip_debugDescription(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, debugDescription]) }
    }

    fn im_performSelector(&self, aSelector: objc::runtime::Sel) -> id {
        unsafe { msg_send![self.ptr, performSelector: aSelector] }
    }

    fn im_performSelector_withObject(&self, aSelector: objc::runtime::Sel, withObject: id) -> id {
        unsafe { msg_send![self.ptr, performSelector: aSelector withObject: withObject] }
    }

    fn im_isProxy(&self) -> bool {
        unsafe { msg_send![self.ptr, isProxy] }
    }
}

impl INSData for NSData {
    fn tm_data() -> Self {
        let ptr = unsafe {
            let ptr = msg_send![class!(NSData), data];
            Id::from_ptr(ptr)
        };
        Self { ptr }
    }

    fn tm_dataWithBytes_length(bytes: *const libc::c_void, length: super::UInt) -> Self {
        let ptr = unsafe {
            let ptr = msg_send![class!(NSData), dataWithBytes:bytes length:length];
            Id::from_ptr(ptr)
        };
        Self { ptr }
    }

    fn tm_dataWithBytesNoCopy_length(bytes: *const libc::c_void, length: super::UInt) -> Self {
        let ptr = unsafe {
            let ptr = msg_send![class!(NSData), dataWithBytesNoCopy:bytes length:length];
            Id::from_ptr(ptr)
        };
        Self { ptr }
    }

    fn tm_dataWithBytesNoCopy_length_freeWhenDone(
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

    fn tm_dataWithData(data: NSData) -> Self {
        let ptr = unsafe {
            let ptr = msg_send![class!(NSData), dataWithData:data.ptr];
            Id::from_ptr(ptr)
        };
        Self { ptr }
    }

    fn tm_dataWithContentsOfFile(path: NSString) -> Self {
        let ptr = unsafe {
            let ptr = msg_send![class!(NSData), dataWithContentsOfFile: path];
            Id::from_ptr(ptr)
        };
        Self { ptr }
    }

    fn ip_bytes(&self) -> *const libc::c_void {
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
        write!(f, "{}", self.ip_debugDescription())
    }
}

impl Display for NSData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}
