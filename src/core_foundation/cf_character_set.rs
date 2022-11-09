use libc::c_void;

use crate::declare_CFType;

#[derive(Debug)]
#[repr(C)]
pub struct __CFCharacterSet(c_void);

/// A reference to a CFString object.
pub type CFCharacterSetRef = *const __CFCharacterSet;

declare_CFType! {
    /// A reference to a CFString object.
    #[repr(C)]
    CFCharacterSet, CFCharacterSetRef
}
