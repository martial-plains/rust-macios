use crate::core_graphics::CGFloat;

/// A description of the distance between the edges of two rectangles.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct NSEdgeInsets {
    /// The distance from the top of the source rectangle to the top of the result rectangle.
    pub top: CGFloat,
    /// The distance from the left side of the source rectangle to the left side of the result rectangle.
    pub left: CGFloat,
    /// The distance from the bottom of the source rectangle to the bottom of the result rectangle.
    pub bottom: CGFloat,
    /// The distance from the right side of the source rectangle to the right side of the result rectangle.
    pub right: CGFloat,
}

extern "C" {
    /// Checks if the receiver is equal to the given edge insets.
    pub fn NSEdgeInsetsEqual(insets1: NSEdgeInsets, insets2: NSEdgeInsets) -> bool;

    /// Returns a new edge insets with the given top, left, bottom, and right values.
    pub fn NSEdgeInsetsMake(
        top: CGFloat,
        left: CGFloat,
        bottom: CGFloat,
        right: CGFloat,
    ) -> NSEdgeInsets;
}
