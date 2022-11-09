use crate::{object, objective_c_runtime::macros::{interface_impl}};

use super::INSLayoutAnchor;

object! {
    /// A factory class for creating vertical layout constraint objects using a fluent API.
    unsafe pub struct NSLayoutYAxisAnchor;
}

impl INSLayoutAnchor for NSLayoutYAxisAnchor {}

#[interface_impl(NSLayoutAnchor)]
impl NSLayoutYAxisAnchor {}
