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
    id,
    objective_c_runtime::traits::{FromId, PNSObject, ToId},
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
        NSMenu::tm_menuBarVisible()
    }

    /// Sets whether the menu bar is visible and selectable by the user.
    pub fn set_menubar_visible(&self, visible: bool) {
        NSMenu::tm_setMenuBarVisible(visible);
    }

    /// The menu bar height for the main menu in pixels.
    pub fn menubar_height(&self) -> CGFloat {
        self.ip_menuBarHeight()
    }

    /// Initializes and returns a menu having the specified title and with autoenabling of menu items turned on.
    pub fn with_title<S>(title: S) -> Self
    where
        S: Into<NSString>,
    {
        Self::im_initWithTitle(title.into())
    }

    /// Inserts a menu item into the menu at a specific location.
    pub fn insert_at_index(&mut self, item: NSMenuItem, index: Int) {
        self.im_insertItem_atIndex(item, index);
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
        self.im_insertItemWithTitle_action_keyEquivalent_atIndex(
            title.into(),
            action,
            key.into(),
            index,
        )
    }

    /// Adds a menu item to the end of the menu.
    pub fn add_item(&mut self, item: NSMenuItem) {
        self.im_addItem(item);
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
        self.im_addItemWithTitle_action_keyEquivalent(title.into(), action, key.into())
    }

    /// Removes a menu item from the menu.
    pub fn remove_item(&mut self, item: NSMenuItem) {
        self.im_removeItem(item);
    }

    /// Removes the menu item at a specified location in the menu.
    pub fn remove_item_at(&mut self, index: Int) {
        self.im_removeItemAtIndex(index);
    }

    /// Removes all the menu items in the menu.
    pub fn remove_all_items(&mut self) {
        self.im_removeAllItems();
    }
}

impl PNSObject for NSMenu {
    fn im_class<'a>() -> &'a Class {
        class!(NSMenu)
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

    fn im_conformsToProtocol(&self, aProtocol: Protocol) -> bool {
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

    fn ip_superclass<'a>() -> Option<&'a Class> {
        Self::im_class().superclass()
    }
}

impl INSMenu for NSMenu {
    fn tm_menuBarVisible() -> bool {
        unsafe { to_bool(msg_send![class!(NSMenu), menuBarVisible]) }
    }

    fn tm_setMenuBarVisible(visible: bool) {
        unsafe { msg_send![class!(NSMenu), setMenuBarVisible: visible] }
    }

    fn ip_menuBarHeight(&self) -> CGFloat {
        unsafe { msg_send![self.ptr, menuBarHeight] }
    }

    fn im_initWithTitle(title: NSString) -> Self {
        unsafe {
            let ptr: id = msg_send![class!(NSMenu), new];
            Self::from_id(msg_send![ptr, initWithTitle: title])
        }
    }

    fn im_insertItem_atIndex(&mut self, newItem: NSMenuItem, index: Int) {
        unsafe { msg_send![self.ptr, insertItem: newItem.ptr atIndex: index] }
    }

    fn im_insertItemWithTitle_action_keyEquivalent_atIndex(
        &mut self,
        string: NSString,
        sel: Sel,
        charCode: NSString,
        index: Int,
    ) -> NSMenuItem {
        unsafe {
            NSMenuItem::from_id(msg_send![self.ptr, insertItemWithTitle: string
                                                                  action: sel
                                                           keyEquivalent: charCode
                                                                  atIndex: index])
        }
    }

    fn im_addItem(&mut self, newItem: NSMenuItem) {
        unsafe { msg_send![self.ptr, addItem: newItem] }
    }

    fn im_addItemWithTitle_action_keyEquivalent(
        &mut self,
        title: NSString,
        sel: Sel,
        charCode: NSString,
    ) -> NSMenuItem {
        unsafe {
            NSMenuItem::from_id(msg_send![self.ptr, addItemWithTitle: title
                                                                action: sel
                                                         keyEquivalent: charCode])
        }
    }

    fn im_removeItem(&mut self, item: NSMenuItem) {
        unsafe { msg_send![self.ptr, removeItem: item] }
    }

    fn im_removeItemAtIndex(&mut self, index: Int) {
        unsafe { msg_send![self.ptr, removeItemAtIndex: index] }
    }

    fn im_removeAllItems(&mut self) {
        unsafe { msg_send![self.ptr, removeAllItems] }
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
        unsafe { Self::from_id(msg_send![self.ptr, retain]) }
    }
}

impl fmt::Debug for NSMenu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debugDescription())
    }
}

impl fmt::Display for NSMenu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}
