/// Describes an integer.
#[cfg(target_pointer_width = "32")]
pub type Int = libc::c_int;

/// Describes an unsigned integer.
#[cfg(target_pointer_width = "32")]
pub type UInt = libc::c_uint;

/// Describes an integer.
#[cfg(target_pointer_width = "64")]
pub type Int = libc::c_long;

/// Describes an unsigned integer.
#[cfg(target_pointer_width = "64")]
pub type UInt = libc::c_ulong;

/// A double-precision, floating-point value type.
pub type Double = libc::c_double;
