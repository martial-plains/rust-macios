use std::fmt::{Debug, Display};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{id, objective_c_runtime::traits::PNSObject};

use super::{traits::INSData, NSString};

/// A static byte buffer in memory.
pub struct NSData {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSData {
    fn new() -> Self {
        Self {
            ptr: unsafe { msg_send![class!(NSData), new] },
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
        unsafe {
            let ptr = msg_send![self.ptr, description];
            NSString::fromId(ptr)
        }
    }

    fn debugDescription(&self) -> NSString {
        unsafe {
            let ptr = msg_send![self.ptr, debugDescription];
            NSString::fromId(ptr)
        }
    }

    fn retain(&self) -> Self {
        unsafe {
            let ptr = msg_send![self.ptr, retain];
            Self::fromId(ptr)
        }
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
