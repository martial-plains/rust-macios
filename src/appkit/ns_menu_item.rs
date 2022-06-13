use std::fmt;

use objc::{
    class, msg_send,
    runtime::{Class, Object, Sel},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::{NSString, UInt},
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject, ToId},
    },
    utils::to_bool,
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

    fn im_is_equal(&self, object: &Self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isEqual: object]) }
    }

    fn ip_hash(&self) -> UInt {
        unsafe { msg_send![&*self.ptr, hash] }
    }

    fn im_is_kind_of_class(&self, class: Class) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isKindOfClass: class]) }
    }

    fn im_is_member_of_class(&self, class: Class) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isMemberOfClass: class]) }
    }

    fn im_responds_to_selector(&self, selector: Sel) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, respondsToSelector: selector]) }
    }

    fn im_conforms_to_protocol(&self, protocol: objc::runtime::Protocol) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, conformsToProtocol: protocol]) }
    }

    fn ip_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![&*self.ptr, description]) }
    }

    fn ip_debug_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![&*self.ptr, debugDescription]) }
    }

    fn im_perform_selector(&self, selector: Sel) -> id {
        unsafe { msg_send![&*self.ptr, performSelector: selector] }
    }

    fn im_perform_selector_with_object(&self, selector: Sel, with_object: id) -> id {
        unsafe { msg_send![&*self.ptr, performSelector: selector withObject: with_object] }
    }

    fn im_is_proxy(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isProxy]) }
    }
}

impl INSMenuItem for NSMenuItem {
    fn ip_is_enabled(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isEnabled]) }
    }

    fn ip_set_enabled(&mut self, enabled: bool) {
        unsafe { msg_send![&*self.ptr, setEnabled: enabled] }
    }

    fn ip_is_hidden(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isHidden]) }
    }

    fn ip_set_hidden(&mut self, hidden: bool) {
        unsafe { msg_send![&*self.ptr, setHidden: hidden] }
    }

    fn ip_is_hidden_or_has_hidden_ancestor(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isHiddenOrHasHiddenAncestor]) }
    }

    fn ip_target(&self) -> id {
        unsafe { msg_send![&*self.ptr, target] }
    }

    fn ip_set_target(&mut self, target: id) {
        unsafe { msg_send![&*self.ptr, setTarget: target] }
    }

    fn ip_action(&self) -> Sel {
        unsafe { msg_send![&*self.ptr, action] }
    }

    fn ip_set_action(&mut self, action: Sel) {
        unsafe { msg_send![&*self.ptr, setAction: action] }
    }

    fn ip_title(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![&*self.ptr, title]) }
    }

    fn ip_set_title(&mut self, title: NSString) {
        unsafe { msg_send![&*self.ptr, setTitle: title] }
    }

    fn ip_submenu(&self) -> NSMenu {
        unsafe { NSMenu::from_id(msg_send![&*self.ptr, submenu]) }
    }

    fn ip_set_submenu(&mut self, submenu: NSMenu) {
        unsafe { msg_send![&*self.ptr, setSubmenu: submenu] }
    }

    fn ip_has_submenu(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, hasSubmenu]) }
    }

    fn ip_parent_item(&self) -> Option<NSMenuItem> {
        unsafe {
            let ptr: id = msg_send![&*self.ptr, parentItem];
            if ptr.is_null() {
                None
            } else {
                Some(NSMenuItem::from_id(ptr))
            }
        }
    }
}

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
