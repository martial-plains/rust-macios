use crate::objective_c_runtime::macros::{interface_impl, object};

use super::INSLayoutAnchor;

object! {
    /// A factory class for creating horizontal layout constraint objects using a fluent API.
    unsafe pub struct NSLayoutXAxisAnchor;
}

impl INSLayoutAnchor for NSLayoutXAxisAnchor {}

#[interface_impl(NSLayoutAnchor)]
impl NSLayoutXAxisAnchor {}
