use objc::{msg_send, runtime::Sel, sel, sel_impl};

use crate::{
    foundation::NSString,
    objective_c_runtime::{
        id, nil,
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};

use super::{interface_impl, object, NSMenu};

object! {
    /// A command item in an app menu.
    unsafe pub struct NSMenuItem;
}

#[interface_impl(NSObject)]
impl NSMenuItem {
    /* Enabling a Menu Item
     */

    /// A Boolean value that indicates whether the menu item is enabled.
    #[property]
    pub fn is_enabled(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isEnabled]) }
    }

    /// Sets whether the menu item is enabled.
    #[property]
    pub fn set_enabled(&mut self, enabled: bool) {
        unsafe { msg_send![self.m_self(), setEnabled: enabled] }
    }

    /* Managing Hidden Status
     */

    /// A Boolean value that indicates whether the menu item is hidden.
    #[property]
    pub fn is_hidden(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isHidden]) }
    }

    /// Sets whether the menu item is hidden.
    #[property]
    pub fn set_hidden(&mut self, hidden: bool) {
        unsafe { msg_send![self.m_self(), setHidden: hidden] }
    }

    /// A Boolean value that indicates whether the menu item or any of its superitems is hidden.
    #[property]
    pub fn is_hidden_or_has_hidden_ancestor(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isHiddenOrHasHiddenAncestor]) }
    }

    /*  Managing the Target and Action */

    /// The menu item's target.
    #[property]
    pub fn target(&self) -> id {
        unsafe { msg_send![self.m_self(), target] }
    }

    /// Sets the menu item's target.
    #[property]
    pub fn set_target(&mut self, target: id) {
        unsafe { msg_send![self.m_self(), setTarget: target] }
    }

    /// The menu item's action-method selector.
    #[property]
    pub fn action(&self) -> Sel {
        unsafe { msg_send![self.m_self(), action] }
    }

    /// Sets the menu item's action-method selector.
    #[property]
    pub fn set_action(&mut self, action: Sel) {
        unsafe { msg_send![self.m_self(), setAction: action] }
    }

    /* Managing the Title
     */

    /// The menu item's title.
    #[property]
    pub fn title(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), title]) }
    }

    /// Sets the menu item's title.
    #[property]
    pub fn set_title(&mut self, title: NSString) {
        unsafe { msg_send![self.m_self(), setTitle: title] }
    }

    /* Managing Submenus
     */

    /// The submenu of the menu item.
    #[property]
    pub fn submenu(&self) -> NSMenu {
        unsafe { NSMenu::from_id(msg_send![self.m_self(), submenu]) }
    }

    /// Sets the submenu of the menu item.
    #[property]
    pub fn set_submenu(&mut self, submenu: NSMenu) {
        unsafe { msg_send![self.m_self(), setSubmenu: submenu] }
    }

    /// A Boolean value that indicates whether the menu item has a submenu.
    #[property]
    pub fn has_submenu(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), hasSubmenu]) }
    }

    /// The menu item whose submenu contains the receiver.
    #[property]
    pub fn parent_item(&self) -> Option<NSMenuItem> {
        unsafe {
            let id = msg_send![self.m_self(), parentItem];
            if id == nil {
                None
            } else {
                Some(NSMenuItem::from_id(id))
            }
        }
    }
}

impl Default for NSMenuItem {
    fn default() -> Self {
        Self::m_new()
    }
}
