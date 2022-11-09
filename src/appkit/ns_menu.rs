use objc::{class, msg_send, runtime::Sel, sel, sel_impl};

use crate::{
    core_graphics::CGFloat,
    foundation::{Int, NSString},
    object,
    objective_c_runtime::traits::{FromId, PNSObject},
    utils::to_bool,
};

use super::{interface_impl, NSMenuItem};

object! {
    /// An object that manages an app’s menus.
    unsafe pub struct NSMenu;
}

impl NSMenu {
    /// Returns a new `NSMenu` instance.
    pub fn new() -> Self {
        unsafe { Self::from_id(msg_send![class!(NSMenu), new]) }
    }
}

#[interface_impl(NSObject)]
impl NSMenu {
    /* Managing the Menu Bar
     */

    /// Returns a Boolean value that indicates whether the menu bar is visible.
    #[method]
    pub fn menu_bar_visible() -> bool {
        unsafe { to_bool(msg_send![Self::m_class(), isMenuBarVisible]) }
    }

    /// Sets whether the menu bar is visible and selectable by the user.
    #[method]
    pub fn set_menu_bar_visible(visible: bool) {
        unsafe { msg_send![Self::m_class(), setMenuBarVisible: visible] }
    }

    /// The menu bar height for the main menu in pixels.
    #[property]
    pub fn menu_bar_height(&self) -> CGFloat {
        unsafe { msg_send![self.m_self(), menuBarHeight] }
    }

    /* Creating an NSMenu Object
     */

    /// Initializes and returns a menu having the specified title and with
    /// autoenabling of menu items turned on.
    #[method]
    pub fn init_with_title(title: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), initWithTitle: title]) }
    }

    /* Adding and Removing Menu Items
     */

    /// Inserts a menu item into the menu at a specific location.
    #[method]
    pub fn insert_item_at_index(&mut self, new_item: NSMenuItem, index: Int) {
        unsafe { msg_send![self.m_self(), insertItem: new_item atIndex: index] }
    }

    /// Creates and adds a menu item at a specified location in the menu.
    #[method]
    pub fn insert_item_with_title_action_key_equivalent_at_index(
        &mut self,
        string: NSString,
        sel: Sel,
        char_code: NSString,
        index: Int,
    ) -> NSMenuItem {
        unsafe {
            NSMenuItem::from_id(msg_send![
                self.m_self(),
                insertItemWithTitle: string
                action: sel
                keyEquivalent: char_code
                atIndex: index
            ])
        }
    }

    /// Adds a menu item to the end of the menu.
    #[method]
    pub fn add_item(&mut self, new_item: NSMenuItem) {
        unsafe { msg_send![self.m_self(), addItem: new_item] }
    }

    /// Creates a new menu item and adds it to the end of the menu.
    #[method]
    pub fn add_item_with_title_action_key_equivalent(
        &mut self,
        title: NSString,
        sel: Sel,
        char_code: NSString,
    ) -> NSMenuItem {
        unsafe {
            NSMenuItem::from_id(msg_send![
                self.m_self(),
                addItemWithTitle: title
                action: sel
                keyEquivalent: char_code
            ])
        }
    }

    /// Removes a menu item from the menu.
    #[method]
    pub fn remove_item(&mut self, item: NSMenuItem) {
        unsafe { msg_send![self.m_self(), removeItem: item] }
    }

    /// Removes the menu item at a specified location in the menu.
    #[method]
    pub fn remove_item_at_index(&mut self, index: Int) {
        unsafe { msg_send![self.m_self(), removeItemAtIndex: index] }
    }

    /// Removes all the menu items in the menu.
    #[method]
    pub fn remove_all_items(&mut self) {
        unsafe { msg_send![self.m_self(), removeAllItems] }
    }
}

impl Default for NSMenu {
    fn default() -> Self {
        Self::new()
    }
}
