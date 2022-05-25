//! # A library to use Apple frameworks in Rust
#![deny(
    const_err,
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
#![allow(nonstandard_style, improper_ctypes)]

use objc::runtime::Object;

pub mod foundation;
pub mod natural_language;
pub mod objective_c_runtime;

pub(crate) mod utils;

/// An ID for an Objective-C object.
pub type id = *mut Object;

// TODO: Create better dictionary implementation
// TODO: Create better array implementation
// TODO: Create macro for creating a dictionary
// TODO: Create macro for creating an array
