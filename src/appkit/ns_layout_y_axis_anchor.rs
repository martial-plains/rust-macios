use objective_c_runtime_proc_macros::interface_impl;

use crate::objective_c_runtime::macros::object;

use super::INSLayoutAnchor;

object! {
    /// A factory class for creating vertical layout constraint objects using a fluent API.
    unsafe pub struct NSLayoutYAxisAnchor;
}

impl INSLayoutAnchor for NSLayoutYAxisAnchor {}

#[interface_impl(NSLayoutAnchor)]
impl NSLayoutYAxisAnchor {}
