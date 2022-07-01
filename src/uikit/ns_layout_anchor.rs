use crate::objective_c_runtime::macros::object;

use super::traits::INSLayoutAnchor;

object! {
    /// A factory class for creating layout constraint objects using a fluent API.
    unsafe pub struct NSLayoutAnchor;
}

impl INSLayoutAnchor for NSLayoutAnchor {}
