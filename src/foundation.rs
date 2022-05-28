//! Access essential data types, collections, and operating-system services to define the base layer of functionality for your app.

mod constants;
mod enums;
mod type_defs;

/// Traits for working with the Foundation framework.
pub mod traits;

/// Macros for working with the Foundation framework.
pub mod macros;

pub use constants::*;
pub use enums::*;
pub use type_defs::*;

/*
Fundamentals
*/

mod ns_array;
mod ns_data;
mod ns_decimal;
mod ns_dictionary;
mod ns_locale;
mod ns_mutable_array;
mod ns_mutable_dictionary;
mod ns_notification;
mod ns_number;
mod ns_string;
mod string_transform;

pub use ns_array::*;
pub use ns_data::*;
pub use ns_decimal::*;
pub use ns_dictionary::*;
pub use ns_locale::*;
pub use ns_mutable_array::*;
pub use ns_mutable_dictionary::*;
pub use ns_notification::*;
pub use ns_number::*;
pub use ns_string::*;
pub use string_transform::*;
