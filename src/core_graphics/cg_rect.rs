use super::{CGPoint, CGSize};

/// A structure that contains a rectangle in a two-dimensional coordinate system.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct CGRect {
    /// A point that specifies the coordinates of the rectangle’s origin.
    pub origin: CGPoint,
    /// A size that specifies the height and width of the rectangle.
    pub size: CGSize,
}
