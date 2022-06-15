//! Gain low-level access to the Objective-C runtime and the Objective-C root types.

/// Traits for the Objective-C runtime.
pub mod traits;

/// Macros for working with the Objective-C runtime
pub mod macros;

mod globals;
mod type_defs;
mod reexports;

pub use globals::*;
pub use type_defs::*;
pub use reexports::*;
