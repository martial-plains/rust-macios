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

/// Describes an 8-bit signed integer.
pub type Int8 = libc::c_schar;

/// Describes an 8-bit unsigned integer.
pub type UInt8 = libc::c_uchar;

/// Describes a 16-bit signed integer.
pub type Int16 = libc::c_short;

/// Describes a 16-bit unsigned integer.
pub type UInt16 = libc::c_ushort;

/// Describes a 32-bit signed integer.
pub type Int32 = libc::c_int;

/// Describes a 32-bit unsigned integer.
pub type UInt32 = libc::c_uint;

/// Describes a 64-bit signed integer.
pub type Int64 = libc::c_longlong;

/// Describes a 64-bit unsigned integer.
pub type UInt64 = libc::c_ulonglong;

/// A double-precision, floating-point value type.
pub type Double = libc::c_double;
