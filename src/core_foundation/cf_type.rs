use core::fmt;

use libc::{c_ulong, c_void};

use crate::{declare_CFType, kernel::Boolean};

use super::{CFAllocator, CFAllocatorRef, CFIndex, CFString, CFStringRef, CFTypeObject};

/// A type for unique, constant integer values that identify particular Core Foundation opaque types.
pub type CFTypeID = c_ulong;

/// An untyped "generic" reference to any Core Foundation object.
pub type CFTypeRef = *const c_void;

/// A type for hash codes returned by the CFHash function.
pub type CFHashCode = c_ulong;

declare_CFType! {
    ///
    CFType, CFTypeRef
}

impl CFType {
    /* Memory Management
     */

    /// Returns the allocator used to allocate a Core Foundation object.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer.
    pub unsafe fn get_allocator(cf: CFTypeRef) -> CFAllocator {
        CFAllocator::create_with_ref(CFGetAllocator(cf))
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
    pub unsafe fn make_collectable(cf: CFTypeRef) -> CFType {
        CFType::create_with_ref(CFMakeCollectable(cf))
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
    pub unsafe fn retain(cf: CFTypeRef) -> CFType {
        CFType::create_with_ref(CFRetain(cf))
    }

    /* Determining Equality
     */

    /// Determines whether two Core Foundation objects are considered equal.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer.
    pub unsafe fn equal(cf1: CFTypeRef, cf2: CFTypeRef) -> Boolean {
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
    pub unsafe fn copy_description(cf: CFTypeRef) -> CFString {
        CFString::create_with_ref(CFCopyDescription(cf))
    }

    /// Returns a textual description of a Core Foundation type, as identified by its type ID, which can be used when debugging.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer.
    pub unsafe fn copy_type_id_description(type_id: CFTypeID) -> CFString {
        CFString::create_with_ref(CFCopyTypeIDDescription(type_id))
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

impl fmt::Debug for CFType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            write!(
                f,
                "{:?}",
                CFType::copy_description(self.get_internal_object())
            )
        }
    }
}

extern "C" {
    pub fn CFGetAllocator(cf: CFTypeRef) -> CFAllocatorRef;

    pub fn CFGetRetainCount(cf: CFTypeRef) -> CFIndex;

    pub fn CFMakeCollectable(cf: CFTypeRef) -> CFTypeRef;

    pub fn CFRelease(cf: CFTypeRef);

    pub fn CFRetain(cf: CFTypeRef) -> CFTypeRef;

    pub fn CFEqual(cf1: CFTypeRef, cf2: CFTypeRef) -> Boolean;

    pub fn CFHash(cf: CFTypeRef) -> CFHashCode;

    pub fn CFCopyDescription(cf: CFTypeRef) -> CFStringRef;

    pub fn CFCopyTypeIDDescription(type_id: CFTypeID) -> CFStringRef;

    pub fn CFGetTypeID(cf: CFTypeRef) -> CFTypeID;

    pub fn CFShow(obj: CFTypeRef);

}
