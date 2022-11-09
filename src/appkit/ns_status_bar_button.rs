#![allow(trivial_casts)]

use objc::{msg_send, sel, sel_impl};

use crate::{object, objective_c_runtime::traits::PNSObject, utils::to_bool};

use super::{interface_impl, ActionHandler, INSButton, INSControl, INSResponder, INSView};

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
        let this = self.m_self();
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

#[interface_impl(NSButton)]
impl NSStatusBarButton {
    ///
    #[property]
    pub fn appears_disabled(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), appearsDisabled]) }
    }
}
