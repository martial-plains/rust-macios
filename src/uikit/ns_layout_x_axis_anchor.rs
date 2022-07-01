
use crate::objective_c_runtime::macros::object;

use super::traits::{INSLayoutAnchor, INSLayoutXAxisAnchor};

object! {
    /// A factory class for creating horizontal layout constraint objects using a fluent API.
    unsafe pub struct NSLayoutXAxisAnchor;
}

impl INSLayoutAnchor for NSLayoutXAxisAnchor {}

impl INSLayoutXAxisAnchor for NSLayoutXAxisAnchor {}
