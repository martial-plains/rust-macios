use std::fmt;

use objc::{
    class, msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl, Encode, Encoding,
};
use objc_id::Id;

use crate::{
    core_graphics::CGFloat,
    foundation::{Int, NSString, UInt},
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject, ToId},
    },
    utils::to_bool,
};

use super::{ns_menu_item::NSMenuItem, INSMenu};

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

    fn im_conforms_to_protocol(&self, protocol: Protocol) -> bool {
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

    fn ip_superclass<'a>() -> Option<&'a Class> {
        Self::im_class().superclass()
    }
}

impl INSMenu for NSMenu {
    fn ip_menu_bar_height(&self) -> CGFloat {
        unsafe { msg_send![&*self.ptr, menuBarHeight] }
    }

    fn im_init_with_title(title: NSString) -> Self {
        unsafe {
            let ptr: id = msg_send![class!(NSMenu), new];
            Self::from_id(msg_send![ptr, initWithTitle: title])
        }
    }

    fn im_insert_item_at_index(&mut self, new_item: NSMenuItem, index: Int) {
        unsafe { msg_send![&*self.ptr, insertItem: new_item.ptr atIndex: index] }
    }

    fn im_insert_item_with_title_action_key_equivalent_at_index(
        &mut self,
        string: NSString,
        sel: Sel,
        char_code: NSString,
        index: Int,
    ) -> NSMenuItem {
        unsafe {
            NSMenuItem::from_id(msg_send![&*self.ptr, insertItemWithTitle: string
                                                                  action: sel
                                                           keyEquivalent: char_code
                                                                  atIndex: index])
        }
    }

    fn im_add_item(&mut self, new_item: NSMenuItem) {
        unsafe { msg_send![&*self.ptr, addItem: new_item] }
    }

    fn im_add_item_with_title_action_key_equivalent(
        &mut self,
        title: NSString,
        sel: Sel,
        char_code: NSString,
    ) -> NSMenuItem {
        unsafe {
            NSMenuItem::from_id(msg_send![&*self.ptr, addItemWithTitle: title
                                                                action: sel
                                                         keyEquivalent: char_code])
        }
    }

    fn im_remove_item(&mut self, item: NSMenuItem) {
        unsafe { msg_send![&*self.ptr, removeItem: item] }
    }

    fn im_remove_item_at_index(&mut self, index: Int) {
        unsafe { msg_send![&*self.ptr, removeItemAtIndex: index] }
    }

    fn im_remove_all_items(&mut self) {
        unsafe { msg_send![&*self.ptr, removeAllItems] }
    }
}

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
