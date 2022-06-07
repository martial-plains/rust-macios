use std::fmt;

use objc::{
    class, msg_send,
    runtime::{Class, Object, Sel},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::{NSString, UInt},
    id,
    objective_c_runtime::traits::{FromId, PNSObject, ToId},
    utils::to_bool,
};

use super::{ns_menu::NSMenu, INSMenuItem};

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
        self.ip_isEnabled()
    }

    /// Sets whether the menu item is enabled.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.ip_setEnabled(enabled);
    }

    /// A Boolean value that indicates whether the menu item is hidden.
    pub fn hidden(&self) -> bool {
        self.ip_isHidden()
    }

    /// Sets whether the menu item is hidden.
    pub fn set_hidden(&mut self, hidden: bool) {
        self.ip_setHidden(hidden);
    }

    /// A Boolean value that indicates whether the menu item or any of its superitems is hidden.
    pub fn hidden_or_has_hidden_ancestor(&self) -> bool {
        self.ip_isHiddenOrHasHiddenAncestor()
    }

    /// The menu item's target.
    pub fn target(&self) -> id {
        self.ip_target()
    }

    /// Sets the menu item's target.
    pub fn set_target(&mut self, target: id) {
        self.ip_setTarget(target);
    }

    /// The menu item's action-method selector.
    pub fn action(&self) -> Sel {
        self.ip_action()
    }

    /// Sets the menu item's action-method selector.
    pub fn set_action(&mut self, action: Sel) {
        self.ip_setAction(action);
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
        self.ip_setTitle(title.into());
    }

    /// The submenu of the menu item.
    pub fn submenu(&self) -> NSMenu {
        self.ip_submenu()
    }

    /// Sets the submenu of the menu item.
    pub fn set_submenu(&mut self, submenu: NSMenu) {
        self.ip_setSubmenu(submenu);
    }

    /// A Boolean value that indicates whether the menu item has a submenu.
    pub fn has_submenu(&self) -> bool {
        self.ip_hasSubmenu()
    }

    /// The menu item whose submenu contains the receiver.
    pub fn parent_item(&self) -> Option<NSMenuItem> {
        self.ip_parentItem()
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

    fn im_isEqual(&self, object: &Self) -> bool {
        unsafe { to_bool(msg_send![self.ptr, isEqual: object]) }
    }

    fn ip_hash(&self) -> UInt {
        unsafe { msg_send![self.ptr, hash] }
    }

    fn im_isKindOfClass(&self, aClass: Class) -> bool {
        unsafe { to_bool(msg_send![self.ptr, isKindOfClass: aClass]) }
    }

    fn im_isMemberOfClass(&self, aClass: Class) -> bool {
        unsafe { to_bool(msg_send![self.ptr, isMemberOfClass: aClass]) }
    }

    fn im_respondsToSelector(&self, aSelector: Sel) -> bool {
        unsafe { to_bool(msg_send![self.ptr, respondsToSelector: aSelector]) }
    }

    fn im_conformsToProtocol(&self, aProtocol: objc::runtime::Protocol) -> bool {
        unsafe { to_bool(msg_send![self.ptr, conformsToProtocol: aProtocol]) }
    }

    fn ip_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, description]) }
    }

    fn ip_debugDescription(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, debugDescription]) }
    }

    fn im_performSelector(&self, aSelector: Sel) -> id {
        unsafe { msg_send![self.ptr, performSelector: aSelector] }
    }

    fn im_performSelector_withObject(&self, aSelector: Sel, withObject: id) -> id {
        unsafe { msg_send![self.ptr, performSelector: aSelector withObject: withObject] }
    }

    fn im_isProxy(&self) -> bool {
        unsafe { to_bool(msg_send![self.ptr, isProxy]) }
    }
}

impl INSMenuItem for NSMenuItem {
    fn ip_isEnabled(&self) -> bool {
        unsafe { to_bool(msg_send![self.ptr, isEnabled]) }
    }

    fn ip_setEnabled(&mut self, enabled: bool) {
        unsafe { msg_send![self.ptr, setEnabled: enabled] }
    }

    fn ip_isHidden(&self) -> bool {
        unsafe { to_bool(msg_send![self.ptr, isHidden]) }
    }

    fn ip_setHidden(&mut self, hidden: bool) {
        unsafe { msg_send![self.ptr, setHidden: hidden] }
    }

    fn ip_isHiddenOrHasHiddenAncestor(&self) -> bool {
        unsafe { to_bool(msg_send![self.ptr, isHiddenOrHasHiddenAncestor]) }
    }

    fn ip_target(&self) -> id {
        unsafe { msg_send![self.ptr, target] }
    }

    fn ip_setTarget(&mut self, target: id) {
        unsafe { msg_send![self.ptr, setTarget: target] }
    }

    fn ip_action(&self) -> Sel {
        unsafe { msg_send![self.ptr, action] }
    }

    fn ip_setAction(&mut self, action: Sel) {
        unsafe { msg_send![self.ptr, setAction: action] }
    }

    fn ip_title(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, title]) }
    }

    fn ip_setTitle(&mut self, title: NSString) {
        unsafe { msg_send![self.ptr, setTitle: title] }
    }

    fn ip_submenu(&self) -> NSMenu {
        unsafe { NSMenu::from_id(msg_send![self.ptr, submenu]) }
    }

    fn ip_setSubmenu(&mut self, submenu: NSMenu) {
        unsafe { msg_send![self.ptr, setSubmenu: submenu] }
    }

    fn ip_hasSubmenu(&self) -> bool {
        unsafe { to_bool(msg_send![self.ptr, hasSubmenu]) }
    }

    fn ip_parentItem(&self) -> Option<NSMenuItem> {
        unsafe {
            let ptr: id = msg_send![self.ptr, parentItem];
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
        write!(f, "{}", self.ip_debugDescription())
    }
}

impl fmt::Display for NSMenuItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}
