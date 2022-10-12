//! Access low-level functions, primitive data types, and various collection
//! types that are bridged seamlessly with the Foundation framework.

mod macros;
mod traits;
mod type_defs;

mod cf_allocator;
pub use cf_allocator::*;
mod cf_data;
pub use cf_data::*;
mod cf_range;
pub use cf_range::*;
mod cf_string;
pub use cf_string::*;
mod cf_type;
pub use cf_type::*;

pub use traits::*;
pub use type_defs::*;
