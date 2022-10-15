use super::object;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u64)]
pub enum NSDisplayGamut {
    Srgb = 1,
    P3,
}

object! {
    /// An object that describes the attributes of a computer’s monitor or screen.
    unsafe pub struct NSScreen;
}
