use objc::{class, msg_send, sel, sel_impl};

use crate::objective_c_runtime::traits::FromId;

use super::{
    object,
    traits::{INSResponder, INSViewController},
};

object! {
    /// A controller that manages a view, typically loaded from a nib file.
    unsafe pub struct NSViewController;
}

impl NSViewController {
    /// Creates a new view controller.
    pub fn new() -> Self {
        unsafe { Self::from_id(msg_send![class!(NSViewController), new]) }
    }
}

impl Default for NSViewController {
    fn default() -> Self {
        Self::new()
    }
}

impl INSResponder for NSViewController {}

impl INSViewController for NSViewController {}
