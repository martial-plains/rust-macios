use libc::c_void;

use crate::declare_CFType;

#[derive(Debug)]
#[repr(C)]
pub struct __CFArray(c_void);

/// A reference to a CFString object.
pub type CFArrayRef = *const __CFArray;

declare_CFType! {
    /// A reference to a CFString object.
    #[repr(C)]
    CFArray, CFArrayRef
}
