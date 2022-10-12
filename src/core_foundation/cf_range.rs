use super::CFIndex;

/// A structure representing a range of sequential items in a container, such as characters in a buffer or elements in a collection.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
#[repr(C)]
pub struct CFRange {
    /// An integer representing the starting location of the range. For type compatibility with the rest of the system, LONG_MAX is the maximum value you should use for location.
    pub location: CFIndex,
    /// An integer representing the number of items in the range. For type compatibility with the rest of the system, LONG_MAX is the maximum value you should use for length.
    pub length: CFIndex,
}

impl CFRange {
    /// Declares and initializes a CFRange structure.
    pub fn make(loc: CFIndex, len: CFIndex) -> CFRange {
        unsafe { CFRangeMake(loc, len) }
    }
}

extern "C" {
    fn CFRangeMake(loc: CFIndex, len: CFIndex) -> CFRange;
}
