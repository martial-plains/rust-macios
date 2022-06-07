use objc::Encode;

/// A structure that contains width and height values.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct CGSize {
    /// A height value.
    pub height: f64,
    /// A width value.
    pub width: f64,
}

unsafe impl Encode for CGSize {
    fn encode() -> objc::Encoding {
        unsafe { objc::Encoding::from_str("{CGSize=dd}") }
    }
}
