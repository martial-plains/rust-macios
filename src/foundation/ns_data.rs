use crate::objective_c_runtime::macros::object;

use super::traits::INSData;

object! {
    /// A static byte buffer in memory.
    unsafe pub struct NSData;
}

impl INSData for NSData {}
