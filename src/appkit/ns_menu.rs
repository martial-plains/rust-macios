use std::fmt;

use objc::{
    class, msg_send,
    runtime::{Class, Object, Sel},
    sel, sel_impl, Encode, Encoding,
};
use objc_id::Id;

use crate::{
    core_graphics::CGFloat,
    foundation::{Int, NSString},
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject, ToId},
    },
};

use super::{ns_menu_item::NSMenuItem, traits::INSMenu};

/// An object that manages an app’s menus.
#[repr(transparent)]
pub struct NSMenu {
    /// The underlying `NSMenu` object.
    pub ptr: Id<Object>,
}

impl NSMenu {
    /// Returns a new `NSMenu` instance.
    pub fn new() -> Self {
        unsafe { Self::from_id(msg_send![class!(NSMenu), new]) }
    }
}

impl NSMenu {
    /// Returns a Boolean value that indicates whether the menu bar is visible.
    pub fn menubar_visible() -> bool {
        NSMenu::tm_menu_bar_visible()
    }

    /// Sets whether the menu bar is visible and selectable by the user.
    pub fn set_menubar_visible(&self, visible: bool) {
        NSMenu::tm_set_menu_bar_visible(visible);
    }

    /// The menu bar height for the main menu in pixels.
    pub fn menubar_height(&self) -> CGFloat {
        self.ip_menu_bar_height()
    }

    /// Initializes and returns a menu having the specified title and with autoenabling of menu items turned on.
    pub fn with_title<S>(title: S) -> Self
    where
        S: Into<NSString>,
    {
        Self::im_init_with_title(title.into())
    }

    /// Inserts a menu item into the menu at a specific location.
    pub fn insert_at_index(&mut self, item: NSMenuItem, index: Int) {
        self.im_insert_item_at_index(item, index);
    }

    /// Creates and adds a menu item at a specified location in the menu.

    pub fn insert_with_title_action_key_at_index<Title, Key>(
        &mut self,
        title: Title,
        action: Sel,
        key: Key,
        index: Int,
    ) -> NSMenuItem
    where
        Title: Into<NSString>,
        Key: Into<NSString>,
    {
        self.im_insert_item_with_title_action_key_equivalent_at_index(
            title.into(),
            action,
            key.into(),
            index,
        )
    }

    /// Adds a menu item to the end of the menu.
    pub fn add_item(&mut self, item: NSMenuItem) {
        self.im_add_item(item);
    }

    /// Creates a new menu item and adds it to the end of the menu.
    pub fn add_item_with_title_action_key<Title, Key>(
        &mut self,
        title: Title,
        action: Sel,
        key: Key,
    ) -> NSMenuItem
    where
        Title: Into<NSString>,
        Key: Into<NSString>,
    {
        self.im_add_item_with_title_action_key_equivalent(title.into(), action, key.into())
    }

    /// Removes a menu item from the menu.
    pub fn remove_item(&mut self, item: NSMenuItem) {
        self.im_remove_item(item);
    }

    /// Removes the menu item at a specified location in the menu.
    pub fn remove_item_at(&mut self, index: Int) {
        self.im_remove_item_at_index(index);
    }

    /// Removes all the menu items in the menu.
    pub fn remove_all_items(&mut self) {
        self.im_remove_all_items();
    }
}

impl PNSObject for NSMenu {
    fn im_class<'a>() -> &'a Class {
        class!(NSMenu)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl INSMenu for NSMenu {}

impl ToId for NSMenu {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSMenu {
    unsafe fn from_id(ptr: id) -> Self {
        NSMenu {
            ptr: Id::from_ptr(ptr),
        }
    }
}

unsafe impl Encode for NSMenu {
    fn encode() -> Encoding {
        unsafe { Encoding::from_str("@") }
    }
}

impl Default for NSMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for NSMenu {
    fn clone(&self) -> Self {
        unsafe { Self::from_id(msg_send![&*self.ptr, retain]) }
    }
}

impl fmt::Debug for NSMenu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSMenu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}
