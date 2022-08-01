use libc::c_void;
use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::objective_c_runtime::{
    macros::object,
    traits::{FromId, PNSObject},
};

use super::{NSString, UInt};

object! {
    /// A static byte buffer in memory.
    unsafe pub struct NSData;
}

#[interface_impl(NSObject)]
impl NSData {
    /* Creating Data
     */

    /// Creates an empty data object.
    #[method]
    pub fn data() -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), data]) }
    }

    /// Creates a data object containing a given number of bytes copied from a given buffer.
    #[method]
    pub fn data_with_bytes_length(bytes: *const c_void, length: UInt) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), dataWithBytes: bytes length: length]) }
    }

    /// Creates a data object that holds a given number of bytes from a given buffer.
    #[method]
    pub fn data_with_bytes_no_copy_length(bytes: *const c_void, length: UInt) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![Self::m_class(), dataWithBytesNoCopy: bytes
                                                                  length: length])
        }
    }

    /// Creates a data object that holds a given number of bytes from a given buffer.
    #[method]
    pub fn data_with_bytes_no_copy_length_free_when_done(
        bytes: *const c_void,
        length: UInt,
        b: bool,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(
                msg_send![Self::m_class(), dataWithBytesNoCopy:bytes length:length freeWhenDone:b],
            )
        }
    }

    /// Creates a data object containing the contents of another data object.
    #[method]
    pub fn data_with_data(data: NSData) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), dataWithData: data]) }
    }

    /*Reading Data from a File
     */

    /// Creates a data object by reading every byte from the file at a given path.
    #[method]
    pub fn data_with_contents_of_file(path: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), dataWithContentsOfFile: path]) }
    }

    /* Accessing Underlying Bytes

    */

    /// A pointer to the data object's contents.
    #[property]
    pub fn bytes(&self) -> *const c_void {
        unsafe { msg_send![self.m_self(), bytes] }
    }
}
