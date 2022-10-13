use libc::c_void;

use super::macros::declare_CFType;

#[derive(Debug)]
#[repr(C)]
pub struct __CFLocale(c_void);

/// A reference to a CFString object.
pub type CFLocaleRef = *const __CFLocale;

declare_CFType! {
    /// A reference to a CFString object.
    #[repr(C)]
    CFLocale, CFLocaleRef
}
