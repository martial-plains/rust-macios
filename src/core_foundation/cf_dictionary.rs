use libc::c_void;

use super::macros::declare_CFType;

#[derive(Debug)]
#[repr(C)]
pub struct __CFDictionary(c_void);

/// A reference to a CFString object.
pub type CFDictionaryRef = *const __CFDictionary;

declare_CFType! {
    /// A reference to a CFString object.
    #[repr(C)]
    CFDictionary, CFDictionaryRef
}
