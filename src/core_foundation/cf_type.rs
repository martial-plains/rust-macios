use libc::{c_ulong, c_void};

use super::{macros::declare_CFType, CFAllocatorRef, CFIndex, CFStringRef};

/// A type for unique, constant integer values that identify particular Core Foundation opaque types.
pub type CFTypeID = c_ulong;

/// An untyped "generic" reference to any Core Foundation object.
pub type CFTypeRef = *const c_void;

/// A type for hash codes returned by the CFHash function.
pub type CFHashCode = c_ulong;

declare_CFType! {
    ///
    #[derive(Debug)]
    CFType
}

impl CFType {
    /* Memory Management
     */

    /// Returns the allocator used to allocate a Core Foundation object.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer.
    pub unsafe fn get_allocator(cf: CFTypeRef) -> CFAllocatorRef {
        CFGetAllocator(cf)
    }

    /// Returns the reference count of a Core Foundation object.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer.
    pub unsafe fn get_retain_count(cf: CFTypeRef) -> CFIndex {
        CFGetRetainCount(cf)
    }

    /// Makes a newly-allocated Core Foundation object eligible for garbage collection.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer.
    pub unsafe fn make_collectable(cf: CFTypeRef) -> CFTypeRef {
        CFMakeCollectable(cf)
    }

    /// Releases a Core Foundation object.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer.
    pub unsafe fn release(cf: CFTypeRef) {
        CFRelease(cf)
    }

    /// Retains a Core Foundation object.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer.
    pub unsafe fn retain(cf: CFTypeRef) -> CFTypeRef {
        CFRetain(cf)
    }

    /* Determining Equality
     */

    /// Determines whether two Core Foundation objects are considered equal.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer.
    pub unsafe fn equal(cf1: CFTypeRef, cf2: CFTypeRef) -> bool {
        CFEqual(cf1, cf2)
    }

    /* Hashing
     */

    /// Returns a code that can be used to identify an object in a hashing structure.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer.
    pub unsafe fn hash(cf: CFTypeRef) -> CFHashCode {
        CFHash(cf)
    }

    /* Miscellaneous Functions
     */

    /// Returns a textual description of a Core Foundation object.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer.
    pub unsafe fn copy_description(cf: CFTypeRef) -> CFStringRef {
        CFCopyDescription(cf)
    }

    /// Returns a textual description of a Core Foundation type, as identified by its type ID, which can be used when debugging.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer.
    pub unsafe fn copy_type_id_description(type_id: CFTypeID) -> CFStringRef {
        CFCopyTypeIDDescription(type_id)
    }

    /// Returns the unique identifier of an opaque type to which a Core Foundation object belongs.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer.
    pub unsafe fn get_type_id(cf: CFTypeRef) -> CFTypeID {
        CFGetTypeID(cf)
    }

    /// Prints a description of a Core Foundation object to stderr.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer.
    pub unsafe fn show(obj: CFTypeRef) {
        CFShow(obj)
    }
}

extern "C" {
    fn CFGetAllocator(cf: CFTypeRef) -> CFAllocatorRef;

    fn CFGetRetainCount(cf: CFTypeRef) -> CFIndex;

    fn CFMakeCollectable(cf: CFTypeRef) -> CFTypeRef;

    fn CFRelease(cf: CFTypeRef);

    fn CFRetain(cf: CFTypeRef) -> CFTypeRef;

    fn CFEqual(cf1: CFTypeRef, cf2: CFTypeRef) -> bool;

    fn CFHash(cf: CFTypeRef) -> CFHashCode;

    fn CFCopyDescription(cf: CFTypeRef) -> CFStringRef;

    fn CFCopyTypeIDDescription(type_id: CFTypeID) -> CFStringRef;

    fn CFGetTypeID(cf: CFTypeRef) -> CFTypeID;

    fn CFShow(obj: CFTypeRef);

}
