use std::fmt;

use libc::c_void;

use super::{
    kCFAllocatorDefault, macros::declare_CFType, CFAllocatorRef, CFIndex, CFRange, CFType,
    CFTypeID, CFTypeObject, CFTypeRef,
};

#[derive(Debug)]
#[repr(C)]
pub struct __CFData(c_void);

/// This provides support for data objects, object-oriented wrappers for byte buffer
pub type CFDataRef = *const __CFData;

declare_CFType! {
    /// This provides support for data objects, object-oriented wrappers for byte buffer
    CFData, CFDataRef
}

/// A CFOptionFlags type for specifying options for searching.
#[derive(Debug)]
#[repr(u64)]
pub enum CFDataSearchFlags {
    ///
    KCFDataSearchBackwards = 1 << 0,
    ///
    KCFDataSearchAnchored = 1 << 1,
}

impl CFData {
    /// Creates an immutable CFData object using data copied from a specified byte buffer.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn create(allocator: CFAllocatorRef, bytes: *const u8, length: CFIndex) -> CFData {
        unsafe { Self::create_with_ref(CFDataCreate(allocator, bytes, length)) }
    }

    /// Creates an immutable copy of a CFData object.
    //
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn create_copy(allocator: CFAllocatorRef, data: CFDataRef) -> CFData {
        Self::create_with_ref(CFDataCreateCopy(allocator, data))
    }

    /// Creates an immutable CFData object from an external (client-owned) byte buffer.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn create_with_bytes_no_copy(
        allocator: CFAllocatorRef,
        bytes: *const u8,
        length: CFIndex,
        bytes_deallocator: CFAllocatorRef,
    ) -> CFData {
        Self::create_with_ref(CFDataCreateWithBytesNoCopy(
            allocator,
            bytes,
            length,
            bytes_deallocator,
        ))
    }

    /// Returns a read-only pointer to the bytes of a CFData object.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_byte_ptr(data: CFDataRef) -> *const u8 {
        CFDataGetBytePtr(data)
    }

    /// Copies the byte contents of a CFData object to an external buffer.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_bytes(data: CFDataRef, range: CFRange, buffer: *mut u8) {
        CFDataGetBytes(data, range, buffer)
    }

    /// Returns the number of bytes contained by a CFData object.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_length(data: CFDataRef) -> CFIndex {
        CFDataGetLength(data)
    }

    /// Finds and returns the range within a data object of the first occurrence of the given data, within a given range, subject to any given options.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn find(
        data: CFDataRef,
        data_to_find: CFDataRef,
        search_range: CFRange,
        compare_options: CFDataSearchFlags,
    ) -> CFRange {
        CFDataFind(data, data_to_find, search_range, compare_options)
    }

    /// Returns the type identifier for the CFData opaque type.
    pub fn get_type_id() -> CFTypeID {
        unsafe { CFDataGetTypeID() }
    }
}

impl fmt::Debug for CFData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            write!(
                f,
                "{:?}",
                CFType::copy_description(self.get_internal_object() as CFTypeRef)
            )
        }
    }
}

impl From<&[u8]> for CFData {
    fn from(value: &[u8]) -> Self {
        unsafe {
            Self::create_with_ref(CFDataCreate(
                kCFAllocatorDefault,
                value.as_ptr(),
                value.len() as CFIndex,
            ))
        }
    }
}

extern "C" {
    /* Creating a CFData Object
     */

    pub fn CFDataCreate(allocator: CFAllocatorRef, bytes: *const u8, length: CFIndex) -> CFDataRef;

    pub fn CFDataCreateCopy(allocator: CFAllocatorRef, data: CFDataRef) -> CFDataRef;
    pub fn CFDataCreateWithBytesNoCopy(
        allocator: CFAllocatorRef,
        bytes: *const u8,
        length: CFIndex,
        bytes_deallocator: CFAllocatorRef,
    ) -> CFDataRef;

    /* Examining a CFData Object
     */

    pub fn CFDataGetBytePtr(data: CFDataRef) -> *const u8;

    pub fn CFDataGetBytes(data: CFDataRef, range: CFRange, buffer: *mut u8);

    pub fn CFDataGetLength(data: CFDataRef) -> CFIndex;

    pub fn CFDataFind(
        data: CFDataRef,
        data_to_find: CFDataRef,
        search_range: CFRange,
        compare_options: CFDataSearchFlags,
    ) -> CFRange;

    /* Getting the CFData Type ID
     */

    pub fn CFDataGetTypeID() -> CFTypeID;
}
