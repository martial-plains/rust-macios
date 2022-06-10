use objc::runtime::Sel;

use crate::{
    appkit::{ns_menu::NSMenu, ns_menu_item::NSMenuItem},
    core_graphics::CGFloat,
    foundation::{Int, NSString},
    objective_c_runtime::{id, traits::PNSObject},
};

/// An object that manages an app’s menus.
pub trait INSMenu: PNSObject {
    /* Managing the Menu Bar
     */

    /// Returns a Boolean value that indicates whether the menu bar is visible.
    fn tm_menu_bar_visible() -> bool;
    /// Sets whether the menu bar is visible and selectable by the user.
    fn tm_set_menu_bar_visible(visible: bool);
    /// The menu bar height for the main menu in pixels.
    fn ip_menu_bar_height(&self) -> CGFloat;

    /* Creating an NSMenu Object
     */

    /// Initializes and returns a menu having the specified title and with
    /// autoenabling of menu items turned on.
    fn im_init_with_title(title: NSString) -> Self;

    /* Adding and Removing Menu Items
     */

    /// Inserts a menu item into the menu at a specific location.
    fn im_insert_item_at_index(&mut self, new_item: NSMenuItem, index: Int);

    /// Creates and adds a menu item at a specified location in the menu.
    fn im_insert_item_with_title_action_key_equivalent_at_index(
        &mut self,
        string: NSString,
        sel: Sel,
        char_code: NSString,
        index: Int,
    ) -> NSMenuItem;

    /// Adds a menu item to the end of the menu.
    fn im_add_item(&mut self, new_item: NSMenuItem);

    /// Creates a new menu item and adds it to the end of the menu.
    fn im_add_item_with_title_action_key_equivalent(
        &mut self,
        title: NSString,
        sel: Sel,
        char_code: NSString,
    ) -> NSMenuItem;

    /// Removes a menu item from the menu.
    fn im_remove_item(&mut self, item: NSMenuItem);

    /// Removes the menu item at a specified location in the menu.
    fn im_remove_item_at_index(&mut self, index: Int);

    /// Removes all the menu items in the menu.
    fn im_remove_all_items(&mut self);
}

/// A command item in an app menu.
pub trait INSMenuItem: PNSObject {
    /* Enabling a Menu Item
     */

    /// A Boolean value that indicates whether the menu item is enabled.
    fn ip_is_enabled(&self) -> bool;

    /// Sets whether the menu item is enabled.
    fn ip_set_enabled(&mut self, enabled: bool);

    /* Managing Hidden Status
     */

    /// A Boolean value that indicates whether the menu item is hidden.
    fn ip_is_hidden(&self) -> bool;

    /// Sets whether the menu item is hidden.
    fn ip_set_hidden(&mut self, hidden: bool);

    /// A Boolean value that indicates whether the menu item or any of its superitems is hidden.
    fn ip_is_hidden_or_has_hidden_ancestor(&self) -> bool;

    /*  Managing the Target and Action */

    /// The menu item's target.
    fn ip_target(&self) -> id;

    /// Sets the menu item's target.
    fn ip_set_target(&mut self, target: id);

    /// The menu item's action-method selector.
    fn ip_action(&self) -> Sel;

    /// Sets the menu item's action-method selector.
    fn ip_set_action(&mut self, action: Sel);

    /* Managing the Title
     */

    /// The menu item's title.
    fn ip_title(&self) -> NSString;

    /// Sets the menu item's title.
    fn ip_set_title(&mut self, title: NSString);

    /* Managing Submenus
     */

    /// The submenu of the menu item.
    fn ip_submenu(&self) -> NSMenu;

    /// Sets the submenu of the menu item.
    fn ip_set_submenu(&mut self, submenu: NSMenu);

    /// A Boolean value that indicates whether the menu item has a submenu.
    fn ip_has_submenu(&self) -> bool;

    /// The menu item whose submenu contains the receiver.
    fn ip_parent_item(&self) -> Option<NSMenuItem>;
}
