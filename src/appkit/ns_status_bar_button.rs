#![allow(trivial_casts)]

use crate::objective_c_runtime::traits::PNSObject;

use super::{
    object,
    traits::{INSButton, INSControl, INSResponder, INSStatusBarButton, INSView},
    ActionHandler,
};

object! {
    /// The appearance and behavior of an item in the systemwide menu bar.
    unsafe pub struct NSStatusBarButton;
}

/// This is an action handler for `NSStatusBarButton`
pub static mut NSSTATUS_BAR_BUTTON_HANDLER: Option<ActionHandler> = None;

impl NSStatusBarButton {
    /// Sets the default action-message selector associated with the control.
    ///
    /// # Arguments
    ///
    /// * `action` - The new action-message.
    pub fn set_action<F: Fn() + Send + Sync + 'static>(&mut self, action: F) {
        let this = self.im_self();
        let handler = ActionHandler::new(unsafe { &*this }, action);
        unsafe {
            NSSTATUS_BAR_BUTTON_HANDLER = Some(handler);
        }
    }
}

impl INSResponder for NSStatusBarButton {}

impl INSView for NSStatusBarButton {}

impl INSControl for NSStatusBarButton {}

impl INSButton for NSStatusBarButton {}

impl INSStatusBarButton for NSStatusBarButton {}
