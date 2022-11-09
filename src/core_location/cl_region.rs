use rust_macios_objective_c_runtime_proc_macros::interface_impl;

use crate::{object, objective_c_runtime::traits::PNSObject};

object! {
    unsafe pub struct CLRegion;
}

#[interface_impl(NSObject)]
impl CLRegion {}
