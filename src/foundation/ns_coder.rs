use crate::objective_c_runtime::macros::object;

use super::traits::INSCoder;

object! {
    /// An abstract class that serves as the basis for objects that enable archiving and distribution of other objects.
    unsafe pub struct NSCoder;
}

impl INSCoder for NSCoder {}
