//! # A library to use Apple frameworks in Rust
#![warn(clippy::all)]
#![allow(improper_ctypes, deprecated, improper_ctypes_definitions)]

pub mod appkit;
pub mod background_tasks;
pub mod compression;
pub mod contacts;
pub mod core_foundation;
pub mod core_graphics;
pub mod core_location;
pub mod core_ml;
pub mod foundation;
pub mod kernel;
pub mod natural_language;
pub mod objective_c_runtime;
pub mod uikit;

pub mod user_notifications;

pub(crate) mod utils;
