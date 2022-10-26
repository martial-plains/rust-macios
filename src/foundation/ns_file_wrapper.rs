use crate::objective_c_runtime::{
    macros::{interface_impl, object},
    traits::PNSObject,
};

object! {
    unsafe pub struct NSFileWrapper;
}

#[interface_impl(NSObject)]
impl NSFileWrapper {}
