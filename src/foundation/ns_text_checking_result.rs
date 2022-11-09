use objc::Encode;

use crate::object;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u64)]
pub enum NSTextCheckingTypes {
    AllSystemTypes = 0xffffffff,
    AllCustomTypes = 0xffffffff << 32,
    AllTypes = (Self::AllSystemTypes as u64 | Self::AllCustomTypes as u64),
}

unsafe impl Encode for NSTextCheckingTypes {
    fn encode() -> objc::Encoding {
        unsafe { objc::Encoding::from_str("Q") }
    }
}

object! {
    unsafe pub struct NSTextCheckingResult;
}
