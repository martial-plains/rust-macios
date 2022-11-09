use crate::{object, objective_c_runtime::{
    macros::{interface_impl},
    traits::PNSObject,
}};

object! {
    ///
    unsafe pub struct CNFetchRequest;
}

#[interface_impl(NSObject)]
impl CNFetchRequest {}
