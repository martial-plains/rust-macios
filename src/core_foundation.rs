//! Access low-level functions, primitive data types, and various collection
//! types that are bridged seamlessly with the Foundation framework.

mod macros;
mod traits;
pub use traits::*;

use libc::{c_double, c_ulong, c_void};

/* Callbacks */

pub type CFComparatorFunction =
    fn(val1: *const c_void, val2: *const c_void, context: *mut c_void) -> CFComparisonResult;

/* Data Types */

/// Priority values used for kAXPriorityKey
pub type CFIndex = c_ulong;

/// A bitfield used for passing special allocation and other requests into Core Foundation functions.
pub type CFOptionFlags = c_ulong;

/* Constants */

/// Constants returned by comparison functions, indicating whether a value is equal to, less than, or greater than another value.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i64)]
pub enum CFComparisonResult {
    /// Returned by a comparison function if the first value is less than the second value.
    KCFCompareLessThan = -1,
    /// Returned by a comparison function if the first value is equal to the second value.
    KCFCompareEqualTo = 0,
    /// Returned by a comparison function if the first value is greater than the second value.
    KCFCompareGreaterThan = 1,
}

extern "C" {
    pub static kCFCoreFoundationVersionNumber: c_double;
}

/* Opaque Types
 */
mod cf_allocator;
pub use cf_allocator::*;
mod cf_array;
pub use cf_array::*;
mod cf_character_set;
pub use cf_character_set::*;
mod cf_data;
pub use cf_data::*;
mod cf_dictionary;
pub use cf_dictionary::*;
mod cf_locale;
pub use cf_locale::*;
mod cf_range;
pub use cf_range::*;
mod cf_string;
pub use cf_string::*;
mod cf_type;
pub use cf_type::*;
