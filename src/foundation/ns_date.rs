use crate::objective_c_runtime::macros::object;

use super::traits::INSDate;

object! {
    /// A representation of a specific point in time, independent of any calendar or time zone.
    unsafe pub struct NSDate;
}

impl INSDate for NSDate {}
