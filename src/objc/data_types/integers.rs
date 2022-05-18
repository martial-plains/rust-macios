/// Describes an integer.
#[cfg(target_pointer_width = "32")]
pub type NSInteger = libc::c_int;

/// Describes an unsigned integer.
#[cfg(target_pointer_width = "32")]
pub type NSUInteger = libc::c_uint;

/// Describes an integer.
#[cfg(target_pointer_width = "64")]
pub type NSInteger = libc::c_long;

/// Describes an unsigned integer.
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = libc::c_ulong;
