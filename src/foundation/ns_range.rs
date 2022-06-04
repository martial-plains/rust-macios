use std::ops::Range;

use super::{NSString, UInt};

/// A structure used to describe a portion of a series, such as characters in a string or objects in an array.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct NSRange {
    /// The number of items in the range (can be 0). For type compatibility with the rest of the system, LONG_MAX is the maximum value you should use for length.
    pub location: UInt,
    /// The start index (0 is the first, as in C arrays). For type compatibility with the rest of the system, LONG_MAX is the maximum value you should use for location.
    pub length: UInt,
}

extern "C" {
    /// Creates a new NSRange from the specified values.
    pub fn NSMakeRange(loc: UInt, len: UInt) -> NSRange;

    /// Returns the sum of the location and length of the range.
    pub fn NSMaxRange(range: NSRange) -> UInt;

    /// Returns the intersection of the specified ranges.
    pub fn NSIntersectionRange(range1: NSRange, range2: NSRange) -> NSRange;

    /// Returns the union of the specified ranges.
    pub fn NSUnionRange(range1: NSRange, range2: NSRange) -> NSRange;

    /// Returns a Boolean value that indicates whether a specified position is in a given range.
    pub fn NSLocationInRange(loc: UInt, range: NSRange) -> bool;

    /// Returns a Boolean value that indicates whether two given ranges are equal.
    pub fn NSEqualRanges(range1: NSRange, range2: NSRange) -> bool;

    /// Returns a range from a textual representation.
    pub fn NSRangeFromString(aString: NSString) -> NSRange;

    /// Returns a string representation of a range.
    pub fn NSStringFromRange(range: NSRange) -> NSString;
}

impl From<Range<usize>> for NSRange {
    fn from(range: Range<usize>) -> Self {
        NSRange {
            location: range.start as UInt,
            length: (range.end - range.start) as UInt,
        }
    }
}

impl From<NSRange> for Range<usize> {
    fn from(range: NSRange) -> Self {
        range.location as usize..(range.location + range.length) as usize
    }
}
