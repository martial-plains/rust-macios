//! Gain low-level access to the Objective-C runtime and the Objective-C root types.

/// Traits for the Objective-C runtime.
pub mod traits;

mod globals;
mod type_defs;

pub use globals::*;
pub use type_defs::*;
