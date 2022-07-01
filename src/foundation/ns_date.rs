use objc::{msg_send, sel, sel_impl};

use crate::objective_c_runtime::{
    macros::object,
    traits::{FromId, PNSObject},
};

use super::traits::INSDate;

object! {
    /// A representation of a specific point in time, independent of any calendar or time zone.
    unsafe pub struct NSDate;
}

impl INSDate for NSDate {}

impl Clone for NSDate {
    fn clone(&self) -> Self {
        unsafe { Self::from_id(msg_send![self.im_self(), retain]) }
    }
}
