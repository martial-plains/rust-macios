use libc::c_ulong;

/// Priority values used for kAXPriorityKey
pub type CFIndex = c_ulong;

/// A bitfield used for passing special allocation and other requests into Core Foundation functions.
pub type CFOptionFlags = c_ulong;
