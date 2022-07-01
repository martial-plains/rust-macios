use crate::objective_c_runtime::macros::object;

use super::traits::INSCharacterSet;

object! {
    /// A character set containing the characters in Unicode General Categories L*, M*, and N*.
    unsafe pub struct NSCharacterSet;
}

impl INSCharacterSet for NSCharacterSet {}
