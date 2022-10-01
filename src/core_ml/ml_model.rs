use objective_c_runtime_proc_macros::interface_impl;

use crate::objective_c_runtime::{macros::object, traits::PNSObject};

object! {
    /// An encapsulation of all the details of your machine learning model.
    unsafe pub struct MLModel;
}

#[interface_impl(NSObject)]
impl MLModel {}
