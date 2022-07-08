use objc::Encode;

use crate::{foundation::traits::INSNotification, objective_c_runtime::macros::object};

object! {
    /// A container for information broadcast through a notification center to all registered observers.
    unsafe pub struct NSNotification;
}

unsafe impl Encode for NSNotification {
    fn encode() -> objc::Encoding {
        unsafe { objc::Encoding::from_str("@") }
    }
}

impl INSNotification for NSNotification {}
