use rust_macios_objective_c_runtime_proc_macros::interface_impl;

use crate::objective_c_runtime::{macros::object, traits::PNSObject};

object! {
    unsafe pub struct CLRegion;
}

#[interface_impl(NSObject)]
impl CLRegion {}
