use objc::{msg_send, sel, sel_impl};

use crate::{
    core_graphics::CGFloat,
    object,
    objective_c_runtime::{
        nil,
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};

use super::{interface_impl, NSMenu, NSStatusBar, NSStatusBarButton, NSStatusItemBehavior};

object! {
    /// An individual element displayed in the system menu bar.
    unsafe pub struct NSStatusItem;
}

impl NSStatusItem {
    /// A status item length that is equal to the status bar’s thickness.
    pub const NSSQUARE_STATUS_ITEM_LENGTH: CGFloat = -2.0;

    /// A status item length that dynamically adjusts to the width of its contents.
    pub const NSVARIABLE_STATUS_ITEM_LENGTH: CGFloat = -1.0;

    /// Creates a new status item.
    pub fn new() -> Self {
        unsafe { Self::from_id(msg_send![NSStatusItem::m_class(), alloc]) }
    }
}

impl Default for NSStatusItem {
    fn default() -> Self {
        Self::new()
    }
}

#[interface_impl(NSObject)]
impl NSStatusItem {
    /* Getting the Item’s Status Bar
     */

    /// The status bar that displays the status item.
    #[property]
    pub fn status_bar(&self) -> NSStatusBar {
        unsafe { NSStatusBar::from_id(msg_send![self.m_self(), statusBar]) }
    }

    /* Managing the Status Item’s Behavior
     */

    /// The set of allowed behaviors for the status item.
    #[property]
    pub fn behavior(&self) -> NSStatusItemBehavior {
        unsafe { msg_send![self.m_self(), behavior] }
    }

    /// The button displayed in the status bar.
    #[property]
    pub fn button(&self) -> Option<NSStatusBarButton> {
        unsafe {
            let id = msg_send![self.m_self(), button];
            if id == nil {
                None
            } else {
                Some(NSStatusBarButton::from_id(id))
            }
        }
    }

    /// The pull-down menu displayed when the user clicks the status item.
    #[property]
    pub fn menu(&self) -> NSMenu {
        unsafe { NSMenu::from_id(msg_send![self.m_self(), menu]) }
    }

    /* Configuring the Status Item’s Appearance
     */

    /// A Boolean value indicating if the menu bar currently displays the status item.
    #[property]
    pub fn visible(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isVisible]) }
    }

    /// The amount of space in the status bar that should be allocated to the status item.
    #[property]
    pub fn length(&self) -> CGFloat {
        unsafe { msg_send![self.m_self(), length] }
    }
}
