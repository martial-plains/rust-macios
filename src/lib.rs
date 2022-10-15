//! # A library to use Apple frameworks in Rust
#![warn(clippy::all)]
#![allow(improper_ctypes, deprecated, improper_ctypes_definitions)]
#![feature(decl_macro)]
#![feature(type_ascription)]
#![feature(c_variadic)]

#[cfg(feature = "appkit")]
pub mod appkit;
#[cfg(feature = "background_tasks")]
pub mod background_tasks;
#[cfg(feature = "compression")]
pub mod compression;
#[cfg(feature = "contacts")]
pub mod contacts;
#[cfg(feature = "core_foundation")]
pub mod core_foundation;
#[cfg(feature = "core_graphics")]
pub mod core_graphics;
#[cfg(feature = "core_ml")]
pub mod core_ml;
#[cfg(feature = "foundation")]
pub mod foundation;
#[cfg(feature = "kernel")]
pub mod kernel;
#[cfg(feature = "natural_language")]
pub mod natural_language;
#[cfg(feature = "objective_c_runtime")]
pub mod objective_c_runtime;
#[cfg(feature = "uikit")]
pub mod uikit;

#[cfg(feature = "user_notifications")]
pub mod user_notifications;

pub(crate) mod utils;
