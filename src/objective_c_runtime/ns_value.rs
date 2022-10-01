use objective_c_runtime_proc_macros::interface_impl;

use super::{macros::object, traits::PNSObject};

object! {
    /// A simple container for a single C or Objective-C data item.
    unsafe pub struct NSValue;
}

#[interface_impl(NSObject)]
impl NSValue {}
