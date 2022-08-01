use objective_c_runtime_proc_macros::interface_impl;

use crate::objective_c_runtime::traits::PNSObject;

use super::object;

object! {
    /// An abstract class that forms the basis of event and command processing
    /// in AppKit.
    unsafe pub struct NSResponder;
}

#[interface_impl(NSObject)]
impl NSResponder {}
