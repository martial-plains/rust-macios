/* Basic Types
*/

use libc::c_double;

use crate::{
    core_graphics::{CGPoint, CGRect, CGSize},
    objective_c_runtime::id,
};

use super::{NSComparisonResult, NSDecimalNumber, NSRange, NSString};

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
pub type Double = c_double;

/// Type alias for `NSDecimalNumber`.
pub type NSDecimal = NSDecimalNumber;

/// Type for UTF-16 code units.
#[allow(non_camel_case_types)]
pub type unichar = u16;

/// A rectangle.
pub type NSRect = CGRect;

/// A two-dimensional size.
pub type NSSize = CGSize;

/* Strings and Text
*/

/// Constants representing an ICU string transform.
pub type NSStringTransform = *const NSString;
/// The keys used to access components of a locale.
pub type NSLocaleKey = *mut NSString;

/// These constants specify mutability options in property lists.
pub type NSPropertyListWriteOptions = super::NSPropertyListMutabilityOptions;

/// The only read options supported are described in NSPropertyListMutabilityOptions.
pub type NSPropertyListReadOptions = super::NSPropertyListMutabilityOptions;

/* Dates and Times
*/

/// A number of seconds.
pub type NSTimeInterval = c_double;

/// A point in a Cartesian coordinate system.
pub type NSPoint = CGPoint;

/// A structure that defines the name of a notification.
pub type NSNotificationName = NSString;

///
pub type NSErrorDomain = NSString;

/// These keys may exist in the user info dictionary.
pub type NSErrorUserInfoKey = NSString;

/// Attributes that you can apply to text in an attributed string.
pub type NSAttributedStringKey = NSString;

/// Type indicating a parameter is a pointer to an NSRange structure.
pub type NSRangePointer = *mut NSRange;

/// Options for importing documents.
pub type NSAttributedStringDocumentReadingOptionKey = NSString;

/// Attributes that apply to a document.
pub type NSAttributedStringDocumentAttributeKey = NSString;

/// Defines the signature for a block object used for comparison operations.
pub type NSComparator = fn(a: id, b: id) -> NSComparisonResult;
