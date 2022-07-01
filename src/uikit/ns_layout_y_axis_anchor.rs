use crate::objective_c_runtime::macros::object;

use super::traits::{INSLayoutAnchor, INSLayoutYAxisAnchor};

object! {
    /// A factory class for creating vertical layout constraint objects using a fluent API.
    unsafe pub struct NSLayoutYAxisAnchor;
}

impl INSLayoutAnchor for NSLayoutYAxisAnchor {}

impl INSLayoutYAxisAnchor for NSLayoutYAxisAnchor {}
