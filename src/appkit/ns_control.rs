use objc::{msg_send, runtime::Sel, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::appkit::INSView;

use super::{object, INSResponder};

object! {
    /// A specialized view, such as a button or text field, that notifies your app of relevant events using the target-action design pattern.
    unsafe pub struct NSControl;
}

impl INSResponder for NSControl {}

impl INSView for NSControl {}

#[interface_impl(NSView)]
impl NSControl {
    /*  Implementing the Target-Action Mechanism */

    /// The default action-message selector associated with the control.
    #[property]
    pub fn action(&self) -> Sel {
        unsafe { msg_send![self.m_self(), action] }
    }

    /// Sets the default action-message selector associated with the control.
    ///
    /// # Arguments
    ///
    /// * `action` - The new action-message selector.
    #[property]
    pub fn set_action(&self, action: Sel) {
        unsafe { msg_send![self.m_self(), setAction: action] }
    }
}
