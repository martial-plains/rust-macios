use crate::{object, objective_c_runtime::traits::PNSObject};

use super::interface_impl;

object! {
    /// An abstract class that forms the basis of event and command processing
    /// in AppKit.
    unsafe pub struct NSResponder;
}

#[interface_impl(NSObject)]
impl NSResponder {}
