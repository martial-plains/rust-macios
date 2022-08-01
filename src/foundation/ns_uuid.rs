use std::ffi::c_uchar;

use objective_c_runtime_proc_macros::interface_impl;

use crate::objective_c_runtime::{
    macros::object,
    msg_send, sel, sel_impl,
    traits::{FromId, PNSObject},
};

use super::{NSComparisonResult, NSString};

object! {
    /// A universally unique value that can be used to identify types, interfaces, and other items.
    unsafe pub struct NSUUID;
}

#[interface_impl(NSObject)]
impl NSUUID {
    /* Creating UUIDs
     */

    /// Create and returns a new UUID with RFC 4122 version 4 random bytes.
    #[method]
    pub fn uuid() -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), UUID]) }
    }

    /// Initializes a new UUID with RFC 4122 version 4 random bytes.
    #[method]
    pub fn init(&mut self) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), init]) }
    }

    /// Initializes a new UUID with the formatted string.
    #[method]
    pub fn init_with_uuid_string(&mut self, string: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithUUIDString: string]) }
    }

    /// Initializes a new UUID with the given bytes.
    ///
    /// # Arguments
    ///
    /// * bytes - Raw UUID bytes to use to create the UUID.
    ///
    /// # Returns
    ///
    /// A new UUID object.
    #[method]
    pub fn init_with_uuid_bytes(&mut self, bytes: *const c_uchar) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithUUIDBytes: bytes]) }
    }

    /* Get UUID Values
     */

    /// Returns the UUID as bytes.
    ///
    /// # Arguments
    ///
    /// * uuid - The value of uuid represented as raw bytes.
    #[method]
    pub fn get_uuid_bytes(&self, uuid: *mut c_uchar) {
        unsafe { msg_send![self.m_self(), getUUIDBytes: uuid] }
    }

    /// The UUID as a string.
    #[property]
    pub fn uuid_string(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), UUIDString]) }
    }

    /// Compares [`NSUUID`] with another
    #[method]
    pub fn compare(&self, other_uuid: NSUUID) -> NSComparisonResult {
        unsafe { msg_send![self.m_self(), compare: other_uuid] }
    }
}
