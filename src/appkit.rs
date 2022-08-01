//! Construct and manage a graphical, event-driven user interface for your
//! macOS app.

pub use app_kit_proc_macros::ns_application_main;

mod enums;
mod globals;
mod macros;
mod type_defs;

pub use enums::*;
pub use globals::*;
pub use macros::*;
pub use type_defs::*;

mod action_handler;
mod ns_appearance;
mod ns_application;
mod ns_application_delegate;
mod ns_button;
mod ns_color;
mod ns_color_space;
mod ns_control;
mod ns_dock_tile;
mod ns_image;
mod ns_menu;
mod ns_menu_item;
mod ns_popover;
mod ns_popover_delegate;
mod ns_responder;
mod ns_running_application;
mod ns_screen;
mod ns_status_bar;
mod ns_status_bar_button;
mod ns_status_item;
mod ns_storyboard;
mod ns_text_field;
mod ns_toolbar;
mod ns_view;
mod ns_view_controller;
mod ns_window;
mod ns_window_delegate;

pub use action_handler::*;
pub use ns_application::*;
pub use ns_application_delegate::*;
pub use ns_button::*;
pub use ns_color::*;
pub use ns_color_space::*;
pub use ns_control::*;
pub use ns_dock_tile::*;
pub use ns_image::*;
pub use ns_menu::*;
pub use ns_menu_item::*;
pub use ns_popover::*;
pub use ns_responder::*;
pub use ns_running_application::*;
pub use ns_screen::*;
pub use ns_status_bar::*;
pub use ns_status_bar_button::*;
pub use ns_status_item::*;
pub use ns_storyboard::*;
pub use ns_text_field::*;
pub use ns_toolbar::*;
pub use ns_view::*;
pub use ns_view_controller::*;
pub use ns_window::*;
