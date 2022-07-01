use super::{object, traits::INSImage};

object! {
    /// A high-level interface for manipulating image data.
    unsafe pub struct NSImage;
}

impl INSImage for NSImage {}
