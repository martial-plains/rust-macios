use std::fmt;

use objc::{
    class, msg_send,
    runtime::{Class, Object, Sel},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::NSString,
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject, ToId},
    },
};

use super::{ns_menu::NSMenu, traits::INSMenuItem};

/// A command item in an app menu.
pub struct NSMenuItem {
    /// The underlying `NSMenuItem` object.
    pub ptr: Id<Object>,
}

impl NSMenuItem {
    /// Returns a new `NSMenuItem` instance.
    pub fn new() -> Self {
        unsafe { Self::from_id(msg_send![class!(NSMenuItem), new]) }
    }

    /// A Boolean value that indicates whether the menu item is enabled.
    pub fn enabled(&self) -> bool {
        self.ip_is_enabled()
    }

    /// Sets whether the menu item is enabled.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.ip_set_enabled(enabled);
    }

    /// A Boolean value that indicates whether the menu item is hidden.
    pub fn hidden(&self) -> bool {
        self.ip_is_hidden()
    }

    /// Sets whether the menu item is hidden.
    pub fn set_hidden(&mut self, hidden: bool) {
        self.ip_set_hidden(hidden);
    }

    /// A Boolean value that indicates whether the menu item or any of its superitems is hidden.
    pub fn hidden_or_has_hidden_ancestor(&self) -> bool {
        self.ip_is_hidden_or_has_hidden_ancestor()
    }

    /// The menu item's target.
    pub fn target(&self) -> id {
        self.ip_target()
    }

    /// Sets the menu item's target.
    pub fn set_target(&mut self, target: id) {
        self.ip_set_target(target);
    }

    /// The menu item's action-method selector.
    pub fn action(&self) -> Sel {
        self.ip_action()
    }

    /// Sets the menu item's action-method selector.
    pub fn set_action(&mut self, action: Sel) {
        self.ip_set_action(action);
    }

    /// The menu item's title.
    pub fn title(&self) -> NSString {
        self.ip_title()
    }

    /// Sets the menu item's title.
    pub fn set_title<S>(&mut self, title: S)
    where
        S: Into<NSString>,
    {
        self.ip_set_title(title.into());
    }

    /// The submenu of the menu item.
    pub fn submenu(&self) -> NSMenu {
        self.ip_submenu()
    }

    /// Sets the submenu of the menu item.
    pub fn set_submenu(&mut self, submenu: NSMenu) {
        self.ip_set_submenu(submenu);
    }

    /// A Boolean value that indicates whether the menu item has a submenu.
    pub fn has_submenu(&self) -> bool {
        self.ip_has_submenu()
    }

    /// The menu item whose submenu contains the receiver.
    pub fn parent_item(&self) -> Option<NSMenuItem> {
        self.ip_parent_item()
    }
}

impl Default for NSMenuItem {
    fn default() -> Self {
        Self::new()
    }
}

impl PNSObject for NSMenuItem {
    fn im_class<'a>() -> &'a Class {
        class!(NSMenuItem)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl INSMenuItem for NSMenuItem {}

impl ToId for NSMenuItem {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSMenuItem {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}

impl fmt::Debug for NSMenuItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSMenuItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}
