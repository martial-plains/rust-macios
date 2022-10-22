/// Coordinates used to establish the edge in RectangleFExtensions.Divide.
#[derive(Debug)]
pub enum CGRectEdge {
    /// The minimum value for the x-coordinate of the rectangle. In macOS and iOS with the default coordinate system this is the left edge of the rectangle.
    MinXEdge,
    /// The minimum value for the y-coordinate of the rectangle. In macOS with the default coordinate system this is the bottom edge of the rectangle. In iOS with the default coordinate system this is the top edge of the rectangle.
    MinYEdge,
    /// The maximum value for the x-coordinate of the rectangle. In macOS and iOS with the default coordinate system this is the right edge of the rectangle.
    MaxXEdge,
    /// The maximum value for the y-coordinate of the rectangle. In macOS with the default coordinate system this is the top edge of the rectangle. In iOS with the default coordinate system this is the bottom edge of the rectangle.
    MaxYEdge,
}
