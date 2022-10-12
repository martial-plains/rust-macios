//! # A library to use Apple frameworks in Rust
#![deny(
    dead_code,
    improper_ctypes,
    missing_debug_implementations,
    missing_docs,
    no_mangle_generic_items,
    non_shorthand_field_patterns,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    trivial_casts,
    trivial_numeric_casts,
    unconditional_recursion,
    unused_allocation,
    unused_comparisons,
    unused_extern_crates,
    unused_import_braces,
    unused_parens,
    unused_qualifications,
    unused_results,
    unused,
    while_true
)]
#![warn(clippy::all, missing_debug_implementations)]
#![allow(improper_ctypes, deprecated)]
#![feature(decl_macro)]
#![feature(type_ascription)]

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
