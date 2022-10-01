use objective_c_runtime_proc_macros::interface_impl;

use crate::objective_c_runtime::{macros::object, traits::PNSObject};

object! {
    /// An immutable collection of unique integer values that represent indexes in another collection.
    unsafe pub struct NSIndexSet;
}

#[interface_impl(NSObject)]
impl NSIndexSet {}
