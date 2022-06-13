//! Construct and manage a graphical, event-driven user interface for your
//! macOS app.

pub use app_kit_proc_macros::ns_application_main;
/// Traits for interacting with the AppKit framework.
pub mod traits;

mod enums;
mod globals;
mod macros;
mod type_defs;

pub use enums::*;
pub use globals::*;
pub use macros::*;
pub use type_defs::*;

mod ns_application;
mod ns_application_delegate;
mod ns_button;
mod ns_color;
mod ns_color_space;
mod ns_dock_tile;
mod ns_image;
mod ns_menu;
mod ns_menu_item;
mod ns_running_application;
mod ns_screen;
mod ns_toolbar;
mod ns_view_controller;
mod ns_window;
mod ns_window_controller;
mod ns_window_delegate;

pub use ns_application::*;
pub use ns_button::*;
pub use ns_color::*;
pub use ns_color_space::*;
pub use ns_dock_tile::*;
pub use ns_image::*;
pub use ns_menu::*;
pub use ns_menu_item::*;
pub use ns_running_application::*;
pub use ns_screen::*;
pub use ns_toolbar::*;
pub use ns_view_controller::*;
pub use ns_window::*;
pub use ns_window_controller::*;
