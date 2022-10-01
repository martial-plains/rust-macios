//! Gain low-level access to the Objective-C runtime and the Objective-C root types.

/// Traits for the Objective-C runtime.
pub mod traits;

/// Macros for working with the Objective-C runtime
pub mod macros;

mod globals;
mod ns_value;
mod reexports;
mod type_defs;

pub use globals::*;
pub use ns_value::*;
pub use reexports::*;
pub use type_defs::*;

pub use objective_c_runtime_proc_macros::*;
