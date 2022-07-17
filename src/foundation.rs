//! Access essential data types, collections, and operating-system services to define the base layer of functionality for your app.

mod enums;
mod globals;
mod type_defs;

/// Traits for working with the Foundation framework.
pub mod traits;

/// Macros for working with the Foundation framework.
pub mod macros;

pub use enums::*;
pub use globals::*;
pub use type_defs::*;

/*
Fundamentals
*/

/// A type that represents a pointer to an Objective-C object.
pub mod ns_array;

pub use ns_array::NSArray;

mod ns_bundle;
mod ns_character_set;
mod ns_coder;
mod ns_data;
mod ns_date;
mod ns_decimal;
mod ns_dictionary;
mod ns_edge_insets;
mod ns_error;
mod ns_locale;
mod ns_mutable_array;
mod ns_mutable_dictionary;
mod ns_mutable_string;
mod ns_notification;
mod ns_null;
mod ns_number;
mod ns_orthography;
mod ns_range;
mod ns_string;
mod ns_uuid;
mod string_transform;

pub use ns_bundle::*;
pub use ns_character_set::*;
pub use ns_coder::*;
pub use ns_data::*;
pub use ns_date::*;
pub use ns_decimal::*;
pub use ns_dictionary::*;
pub use ns_edge_insets::*;
pub use ns_error::*;
pub use ns_locale::*;
pub use ns_mutable_array::NSMutableArray;
pub use ns_mutable_dictionary::*;
pub use ns_mutable_string::*;
pub use ns_notification::*;
pub use ns_null::*;
pub use ns_number::*;
pub use ns_orthography::*;
pub use ns_range::*;
pub use ns_string::*;
pub use ns_uuid::*;
pub use string_transform::*;

/* App Support
*/

#[allow(improper_ctypes)]
extern "C" {
    /// Respond to problem situations in your interactions with APIs, and fine-tune your app for better debugging.
    pub fn NSLog(format: NSString, ...);

}

/*Low-Level Utilities
*/

mod ns_autoreleasepool;

pub use ns_autoreleasepool::*;
