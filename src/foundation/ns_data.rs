use std::fmt::{Debug, Display};

use objc::{
    class, msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use objc_id::Id;

use crate::objective_c_runtime::{
    id,
    traits::{FromId, PNSObject, ToId},
};

use super::{traits::INSData, NSString};

/// A static byte buffer in memory.
pub struct NSData {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSData {
    fn im_class<'a>() -> &'a Class {
        class!(NSData)
    }

    fn im_is_equal(&self, object: &Self) -> bool {
        unsafe { msg_send![self.ptr, isEqual: object] }
    }

    fn ip_hash(&self) -> super::UInt {
        unsafe { msg_send![self.ptr, hash] }
    }

    fn im_is_kind_of_class(&self, class: Class) -> bool {
        unsafe { msg_send![self.ptr, isKindOfClass: class] }
    }

    fn im_is_member_of_class(&self, class: Class) -> bool {
        unsafe { msg_send![self.ptr, isMemberOfClass: class] }
    }

    fn im_responds_to_selector(&self, selector: Sel) -> bool {
        unsafe { msg_send![self.ptr, respondsToSelector: selector] }
    }

    fn im_conforms_to_protocol(&self, protocol: Protocol) -> bool {
        unsafe { msg_send![self.ptr, conformsToProtocol: protocol] }
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

    fn tm_data_with_bytes_length(bytes: *const libc::c_void, length: super::UInt) -> Self {
        let ptr = unsafe {
            let ptr = msg_send![class!(NSData), dataWithBytes:bytes length:length];
            Id::from_ptr(ptr)
        };
        Self { ptr }
    }

    fn tm_data_with_bytes_no_copy_length(bytes: *const libc::c_void, length: super::UInt) -> Self {
        let ptr = unsafe {
            let ptr = msg_send![class!(NSData), dataWithBytesNoCopy:bytes length:length];
            Id::from_ptr(ptr)
        };
        Self { ptr }
    }

    fn tm_data_with_bytes_no_copy_length_free_when_done(
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

    fn tm_data_with_data(data: NSData) -> Self {
        let ptr = unsafe {
            let ptr = msg_send![class!(NSData), dataWithData:data.ptr];
            Id::from_ptr(ptr)
        };
        Self { ptr }
    }

    fn tm_data_with_contents_of_file(path: NSString) -> Self {
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
        write!(f, "{}", self.ip_debug_description())
    }
}

impl Display for NSData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}
