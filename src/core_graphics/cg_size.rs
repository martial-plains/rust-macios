/// A structure that contains width and height values.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct CGSize {
    /// A height value.
    pub height: f64,
    /// A width value.
    pub width: f64,
}
