use objective_c_runtime_proc_macros::interface_impl;

use crate::objective_c_runtime::{macros::object, traits::PNSObject};

object! {
    /// An object that represents the location of a resource, such as an item on a remote server or the path to a local file.
    unsafe pub struct NSURL;
}

#[interface_impl(NSObject)]
impl NSURL {}
