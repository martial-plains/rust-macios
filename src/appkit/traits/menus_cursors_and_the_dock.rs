use objc::runtime::Sel;

use crate::{
    appkit::{ns_menu::NSMenu, ns_menu_item::NSMenuItem},
    core_graphics::CGFloat,
    foundation::{Int, NSString},
    id,
    objective_c_runtime::traits::PNSObject,
};

/// An object that manages an app’s menus.
pub trait INSMenu: PNSObject {
    /* Managing the Menu Bar
     */

    /// Returns a Boolean value that indicates whether the menu bar is visible.
    fn tm_menuBarVisible() -> bool;
    /// Sets whether the menu bar is visible and selectable by the user.
    fn tm_setMenuBarVisible(visible: bool);
    /// The menu bar height for the main menu in pixels.
    fn ip_menuBarHeight(&self) -> CGFloat;

    /* Creating an NSMenu Object
     */

    /// Initializes and returns a menu having the specified title and with
    /// autoenabling of menu items turned on.
    fn im_initWithTitle(title: NSString) -> Self;

    /* Adding and Removing Menu Items
     */

    /// Inserts a menu item into the menu at a specific location.
    fn im_insertItem_atIndex(&mut self, newItem: NSMenuItem, index: Int);

    /// Creates and adds a menu item at a specified location in the menu.
    fn im_insertItemWithTitle_action_keyEquivalent_atIndex(
        &mut self,
        string: NSString,
        sel: Sel,
        charCode: NSString,
        index: Int,
    ) -> NSMenuItem;

    /// Adds a menu item to the end of the menu.
    fn im_addItem(&mut self, newItem: NSMenuItem);

    /// Creates a new menu item and adds it to the end of the menu.
    fn im_addItemWithTitle_action_keyEquivalent(
        &mut self,
        title: NSString,
        sel: Sel,
        charCode: NSString,
    ) -> NSMenuItem;

    /// Removes a menu item from the menu.
    fn im_removeItem(&mut self, item: NSMenuItem);

    /// Removes the menu item at a specified location in the menu.
    fn im_removeItemAtIndex(&mut self, index: Int);

    /// Removes all the menu items in the menu.
    fn im_removeAllItems(&mut self);
}

/// A command item in an app menu.
pub trait INSMenuItem: PNSObject {
    /* Enabling a Menu Item
     */

    /// A Boolean value that indicates whether the menu item is enabled.
    fn ip_isEnabled(&self) -> bool;

    /// Sets whether the menu item is enabled.
    fn ip_setEnabled(&mut self, enabled: bool);

    /* Managing Hidden Status
     */

    /// A Boolean value that indicates whether the menu item is hidden.
    fn ip_isHidden(&self) -> bool;

    /// Sets whether the menu item is hidden.
    fn ip_setHidden(&mut self, hidden: bool);

    /// A Boolean value that indicates whether the menu item or any of its superitems is hidden.
    fn ip_isHiddenOrHasHiddenAncestor(&self) -> bool;

    /*  Managing the Target and Action */

    /// The menu item's target.
    fn ip_target(&self) -> id;

    /// Sets the menu item's target.
    fn ip_setTarget(&mut self, target: id);

    /// The menu item's action-method selector.
    fn ip_action(&self) -> Sel;

    /// Sets the menu item's action-method selector.
    fn ip_setAction(&mut self, action: Sel);

    /* Managing the Title
     */

    /// The menu item's title.
    fn ip_title(&self) -> NSString;

    /// Sets the menu item's title.
    fn ip_setTitle(&mut self, title: NSString);

    /* Managing Submenus
     */

    /// The submenu of the menu item.
    fn ip_submenu(&self) -> NSMenu;

    /// Sets the submenu of the menu item.
    fn ip_setSubmenu(&mut self, submenu: NSMenu);

    /// A Boolean value that indicates whether the menu item has a submenu.
    fn ip_hasSubmenu(&self) -> bool;

    /// The menu item whose submenu contains the receiver.
    fn ip_parentItem(&self) -> Option<NSMenuItem>;
}
