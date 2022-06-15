use objc::{msg_send, runtime::Sel, sel, sel_impl};

use crate::{
    appkit::{
        ns_menu::NSMenu, ns_menu_item::NSMenuItem, NSStatusBar, NSStatusBarButton,
        NSStatusItemBehavior,
    },
    core_graphics::CGFloat,
    foundation::{Int, NSString},
    objective_c_runtime::{
        id, nil,
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};

use super::INSButton;

/// An object that manages an app’s menus.
pub trait INSMenu: PNSObject {
    /* Managing the Menu Bar
     */

    /// Returns a Boolean value that indicates whether the menu bar is visible.
    fn tm_menu_bar_visible() -> bool {
        unsafe { to_bool(msg_send![Self::im_class(), isMenuBarVisible]) }
    }
    /// Sets whether the menu bar is visible and selectable by the user.
    fn tm_set_menu_bar_visible(visible: bool) {
        unsafe { msg_send![Self::im_class(), setMenuBarVisible: visible] }
    }
    /// The menu bar height for the main menu in pixels.
    fn ip_menu_bar_height(&self) -> CGFloat {
        unsafe { msg_send![self.im_self(), menuBarHeight] }
    }

    /* Creating an NSMenu Object
     */

    /// Initializes and returns a menu having the specified title and with
    /// autoenabling of menu items turned on.
    fn im_init_with_title(title: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), initWithTitle: title]) }
    }

    /* Adding and Removing Menu Items
     */

    /// Inserts a menu item into the menu at a specific location.
    fn im_insert_item_at_index(&mut self, new_item: NSMenuItem, index: Int) {
        unsafe { msg_send![self.im_self(), insertItem: new_item atIndex: index] }
    }

    /// Creates and adds a menu item at a specified location in the menu.
    fn im_insert_item_with_title_action_key_equivalent_at_index(
        &mut self,
        string: NSString,
        sel: Sel,
        char_code: NSString,
        index: Int,
    ) -> NSMenuItem {
        unsafe {
            NSMenuItem::from_id(msg_send![
                self.im_self(),
                insertItemWithTitle: string
                action: sel
                keyEquivalent: char_code
                atIndex: index
            ])
        }
    }

    /// Adds a menu item to the end of the menu.
    fn im_add_item(&mut self, new_item: NSMenuItem) {
        unsafe { msg_send![self.im_self(), addItem: new_item] }
    }

    /// Creates a new menu item and adds it to the end of the menu.
    fn im_add_item_with_title_action_key_equivalent(
        &mut self,
        title: NSString,
        sel: Sel,
        char_code: NSString,
    ) -> NSMenuItem {
        unsafe {
            NSMenuItem::from_id(msg_send![
                self.im_self(),
                addItemWithTitle: title
                action: sel
                keyEquivalent: char_code
            ])
        }
    }

    /// Removes a menu item from the menu.
    fn im_remove_item(&mut self, item: NSMenuItem) {
        unsafe { msg_send![self.im_self(), removeItem: item] }
    }

    /// Removes the menu item at a specified location in the menu.
    fn im_remove_item_at_index(&mut self, index: Int) {
        unsafe { msg_send![self.im_self(), removeItemAtIndex: index] }
    }

    /// Removes all the menu items in the menu.
    fn im_remove_all_items(&mut self) {
        unsafe { msg_send![self.im_self(), removeAllItems] }
    }
}

/// A command item in an app menu.
pub trait INSMenuItem: PNSObject {
    /* Enabling a Menu Item
     */

    /// A Boolean value that indicates whether the menu item is enabled.
    fn ip_is_enabled(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), isEnabled]) }
    }

    /// Sets whether the menu item is enabled.
    fn ip_set_enabled(&mut self, enabled: bool) {
        unsafe { msg_send![self.im_self(), setEnabled: enabled] }
    }

    /* Managing Hidden Status
     */

    /// A Boolean value that indicates whether the menu item is hidden.
    fn ip_is_hidden(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), isHidden]) }
    }

    /// Sets whether the menu item is hidden.
    fn ip_set_hidden(&mut self, hidden: bool) {
        unsafe { msg_send![self.im_self(), setHidden: hidden] }
    }

    /// A Boolean value that indicates whether the menu item or any of its superitems is hidden.
    fn ip_is_hidden_or_has_hidden_ancestor(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), isHiddenOrHasHiddenAncestor]) }
    }

    /*  Managing the Target and Action */

    /// The menu item's target.
    fn ip_target(&self) -> id {
        unsafe { msg_send![self.im_self(), target] }
    }

    /// Sets the menu item's target.
    fn ip_set_target(&mut self, target: id) {
        unsafe { msg_send![self.im_self(), setTarget: target] }
    }

    /// The menu item's action-method selector.
    fn ip_action(&self) -> Sel {
        unsafe { msg_send![self.im_self(), action] }
    }

    /// Sets the menu item's action-method selector.
    fn ip_set_action(&mut self, action: Sel) {
        unsafe { msg_send![self.im_self(), setAction: action] }
    }

    /* Managing the Title
     */

    /// The menu item's title.
    fn ip_title(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.im_self(), title]) }
    }

    /// Sets the menu item's title.
    fn ip_set_title(&mut self, title: NSString) {
        unsafe { msg_send![self.im_self(), setTitle: title] }
    }

    /* Managing Submenus
     */

    /// The submenu of the menu item.
    fn ip_submenu(&self) -> NSMenu {
        unsafe { NSMenu::from_id(msg_send![self.im_self(), submenu]) }
    }

    /// Sets the submenu of the menu item.
    fn ip_set_submenu(&mut self, submenu: NSMenu) {
        unsafe { msg_send![self.im_self(), setSubmenu: submenu] }
    }

    /// A Boolean value that indicates whether the menu item has a submenu.
    fn ip_has_submenu(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), hasSubmenu]) }
    }

    /// The menu item whose submenu contains the receiver.
    fn ip_parent_item(&self) -> Option<NSMenuItem> {
        unsafe {
            let id = msg_send![self.im_self(), parentItem];
            if id == nil {
                None
            } else {
                Some(NSMenuItem::from_id(id))
            }
        }
    }
}

/// An individual element displayed in the system menu bar.
pub trait INSStatusItem: PNSObject {
    /* Getting the Item’s Status Bar
     */

    /// The status bar that displays the status item.
    fn ip_status_bar(&self) -> NSStatusBar {
        unsafe { NSStatusBar::from_id(msg_send![self.im_self(), statusBar]) }
    }

    /* Managing the Status Item’s Behavior
     */

    /// The set of allowed behaviors for the status item.
    fn ip_behavior(&self) -> NSStatusItemBehavior {
        unsafe { msg_send![self.im_self(), behavior] }
    }

    /// The button displayed in the status bar.
    fn ip_button(&self) -> Option<NSStatusBarButton> {
        unsafe {
            let id = msg_send![self.im_self(), button];
            if id == nil {
                None
            } else {
                Some(NSStatusBarButton::from_id(id))
            }
        }
    }

    /// The pull-down menu displayed when the user clicks the status item.
    fn ip_menu(&self) -> NSMenu {
        unsafe { NSMenu::from_id(msg_send![self.im_self(), menu]) }
    }

    /* Configuring the Status Item’s Appearance
     */

    /// A Boolean value indicating if the menu bar currently displays the status item.
    fn ip_visible(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), isVisible]) }
    }

    /// The amount of space in the status bar that should be allocated to the status item.
    fn ip_length(&self) -> CGFloat {
        unsafe { msg_send![self.im_self(), length] }
    }
}

/// The appearance and behavior of an item in the systemwide menu bar.
pub trait INSStatusBarButton: INSButton {
    ///
    fn ip_appears_disabled(&self) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), appearsDisabled]) }
    }
}
