use std::fmt;

use libc::c_void;

use crate::declare_CFType;

use super::{CFIndex, CFOptionFlags, CFStringRef, CFType, CFTypeObject, CFTypeRef};

#[repr(C)]
pub struct __CFAllocator(c_void);

/// A reference to a [`CFAllocator`] object.
pub type CFAllocatorRef = *const __CFAllocator;

declare_CFType! {
    /// CFAllocator is an opaque type that allocates and deallocates memory for you.
    CFAllocator, CFAllocatorRef
}

/// A prototype for a function callback that retains the given data.
pub type CFAllocatorRetainCallBack = extern "C" fn(info: *mut c_void) -> *mut c_void;
/// A prototype for a function callback that releases the given data.
pub type CFAllocatorReleaseCallBack = extern "C" fn(info: *mut c_void);
/// A prototype for a function callback that provides a description of the specified data.
pub type CFAllocatorCopyDescriptionCallBack = extern "C" fn(info: *mut c_void) -> CFStringRef;
/// A prototype for a function callback that allocates memory of a requested size.
pub type CFAllocatorAllocateCallBack =
    extern "C" fn(allocSize: CFIndex, hint: CFOptionFlags, info: *mut c_void) -> *mut c_void;
/// A prototype for a function callback that reallocates memory of a requested size for an existing block of memory.
pub type CFAllocatorReallocateCallBack = extern "C" fn(
    ptr: *mut c_void,
    newsize: CFIndex,
    hint: CFOptionFlags,
    info: *mut c_void,
) -> *mut c_void;
/// A prototype for a function callback that deallocates a block of memory.
pub type CFAllocatorDeallocateCallBack = extern "C" fn(ptr: *mut c_void, info: *mut c_void);
/// A prototype for a function callback that gives the size of memory likely to be allocated, given a certain request.
pub type CFAllocatorPreferredSizeCallBack =
    extern "C" fn(size: CFIndex, hint: CFOptionFlags, info: *mut c_void) -> CFIndex;

/// A structure that defines the context or operating environment for an allocator (CFAllocator) object. Every Core Foundation allocator object must have a context defined for it.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CFAllocatorContext {
    /// An integer of type CFIndex. Assign the version number of the allocator. Currently the only valid value is 0.
    pub version: CFIndex,
    /// An untyped pointer to program-defined data. Allocate memory for this data and assign a pointer to it. This data is often control information for the allocator. You may assign NULL.
    pub info: *mut c_void,
    /// A prototype for a function callback that retains the data pointed to by the info field. In implementing this function, retain the data you have defined for the allocator context in this field. (This might make sense only if the data is a Core Foundation object.) You may set this function pointer to NULL.
    pub retain: Option<CFAllocatorRetainCallBack>,
    /// A prototype for a function callback that releases the data pointed to by the info field. In implementing this function, release (or free) the data you have defined for the allocator context. You may set this function pointer to NULL, but doing so might result in memory leaks.
    pub release: Option<CFAllocatorReleaseCallBack>,
    /// A prototype for a function callback that provides a description of the data pointed to by the info field. In implementing this function, return a reference to a CFString object that describes your allocator, particularly some characteristics of your program-defined data. You may set this function pointer to NULL, in which case Core Foundation will provide a rudimentary description.
    pub copy_description: Option<CFAllocatorCopyDescriptionCallBack>,
    /// A prototype for a function callback that allocates memory of a requested size. In implementing this function, allocate a block of memory of at least size bytes and return a pointer to the start of the block. The hint argument is a bitfield that you should currently not use (that is, assign 0). The size parameter should always be greater than 0. If it is not, or if problems in allocation occur, return NULL. This function pointer may not be assigned NULL.
    pub allocate: Option<CFAllocatorAllocateCallBack>,
    /// A prototype for a function callback that reallocates memory of a requested size for an existing block of memory.
    pub reallocate: Option<CFAllocatorReallocateCallBack>,
    /// A prototype for a function callback that deallocates a given block of memory. In implementing this function, make the block of memory pointed to by ptr available for subsequent reuse by the allocator but unavailable for continued use by the program. The ptr parameter cannot be NULL and if the ptr parameter is not a block of memory that has been previously allocated by the allocator, the results are undefined; abnormal program termination can occur. You can set this callback to NULL, in which case the CFAllocatorDeallocate function has no effect.
    pub deallocate: Option<CFAllocatorDeallocateCallBack>,
    /// A prototype for a function callback that determines whether there is enough free memory to satisfy a request. In implementing this function, return the actual size the allocator is likely to allocate given a request for a block of memory of size size. The hint argument is a bitfield that you should currently not use.
    pub preferred_size: Option<CFAllocatorPreferredSizeCallBack>,
}

impl fmt::Debug for CFAllocator {
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

extern "C" {
    /* Predefined Allocators
     */

    /// This is a synonym for NULL.
    pub static kCFAllocatorDefault: CFAllocatorRef;
    /// Default system allocator.
    pub static kCFAllocatorSystemDefault: CFAllocatorRef;
    /// This allocator uses malloc(), realloc(), and free().
    pub static kCFAllocatorMalloc: CFAllocatorRef;
    /// This allocator explicitly uses the default malloc zone, returned by malloc_default_zone().
    pub static kCFAllocatorMallocZone: CFAllocatorRef;
    /// This allocator does nothing—it allocates no memory.
    pub static kCFAllocatorNull: CFAllocatorRef;
    /// Special allocator argument to CFAllocatorCreate—it uses the functions given in the context to allocate the allocator.
    pub static kCFAllocatorUseContext: CFAllocatorRef;
}
