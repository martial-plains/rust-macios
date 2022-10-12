use super::{
    kCFAllocatorDefault, macros::declare_CFType, CFAllocatorRef, CFIndex, CFRange, CFTypeID,
};

/// This provides support for data objects, object-oriented wrappers for byte buffer
pub type CFDataRef = *const CFData;

declare_CFType! {
    /// This provides support for data objects, object-oriented wrappers for byte buffer
    #[derive(Debug)]
    CFData
}

/// A [`CFOptionFlags`] type for specifying options for searching.
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
        unsafe { CFDataCreate(allocator, bytes, length) }
    }

    /// Creates an immutable copy of a CFData object.
    //
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn create_copy(allocator: CFAllocatorRef, data: CFData) -> CFData {
        CFDataCreateCopy(allocator, data)
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
        unsafe { CFDataCreateWithBytesNoCopy(allocator, bytes, length, bytes_deallocator) }
    }

    /// Returns a read-only pointer to the bytes of a CFData object.
    pub fn get_byte_ptr(data: CFData) -> *const u8 {
        unsafe { CFDataGetBytePtr(data) }
    }

    /// Copies the byte contents of a CFData object to an external buffer.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    pub unsafe fn get_bytes(data: CFData, range: CFRange, buffer: *mut u8) {
        CFDataGetBytes(data, range, buffer)
    }

    /// Returns the number of bytes contained by a CFData object.
    pub fn get_length(data: CFData) -> CFIndex {
        unsafe { CFDataGetLength(data) }
    }

    /// Finds and returns the range within a data object of the first occurrence of the given data, within a given range, subject to any given options.
    pub fn find(
        data: CFData,
        data_to_find: CFData,
        search_range: CFRange,
        compare_options: CFDataSearchFlags,
    ) -> CFRange {
        unsafe { CFDataFind(data, data_to_find, search_range, compare_options) }
    }

    /// Returns the type identifier for the CFData opaque type.
    pub fn get_type_id() -> CFTypeID {
        unsafe { CFDataGetTypeID() }
    }
}

extern "C" {
    /* Creating a CFData Object
     */

    fn CFDataCreate(allocator: CFAllocatorRef, bytes: *const u8, length: CFIndex) -> CFData;

    fn CFDataCreateCopy(allocator: CFAllocatorRef, data: CFData) -> CFData;
    fn CFDataCreateWithBytesNoCopy(
        allocator: CFAllocatorRef,
        bytes: *const u8,
        length: CFIndex,
        bytes_deallocator: CFAllocatorRef,
    ) -> CFData;

    /* Examining a CFData Object
     */

    fn CFDataGetBytePtr(data: CFData) -> *const u8;

    fn CFDataGetBytes(data: CFData, range: CFRange, buffer: *mut u8);

    fn CFDataGetLength(data: CFData) -> CFIndex;

    fn CFDataFind(
        data: CFData,
        data_to_find: CFData,
        search_range: CFRange,
        compare_options: CFDataSearchFlags,
    ) -> CFRange;

    /* Getting the CFData Type ID
     */

    fn CFDataGetTypeID() -> CFTypeID;
}

impl From<&[u8]> for CFData {
    fn from(value: &[u8]) -> Self {
        unsafe { CFDataCreate(kCFAllocatorDefault, value.as_ptr(), value.len() as CFIndex) }
    }
}

#[cfg(test)]
mod tests {

    use crate::core_foundation::{kCFAllocatorDefault, CFIndex};

    use super::CFData;

    #[test]
    fn test_data_provider() {
        let buffer = vec![5];
        let ptr = (*buffer).as_ref().as_ptr();
        let len = (*buffer).as_ref().len() as CFIndex;
        let _data_ref = unsafe {
            CFData::create_with_bytes_no_copy(kCFAllocatorDefault, ptr, len, std::ptr::null())
        };
    }
}
