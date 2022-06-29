use crate::objective_c_runtime::macros::object;

use super::traits::INSNull;

object! {
    /// Returns the singleton instance of NSNull.
    unsafe pub struct NSNull;

}

impl INSNull for NSNull {}
