use crate::objective_c_runtime::macros::object;

use super::traits::INSLayoutConstraint;

object! {
    /// The relationship between two user interface objects that must be satisfied by the constraint-based layout system.
    unsafe pub struct NSLayoutConstraint;
}

impl INSLayoutConstraint for NSLayoutConstraint {}
