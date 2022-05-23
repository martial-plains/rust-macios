mod enums;
mod ns_string;

pub use enums::*;
pub use ns_string::*;

/// Constants representing an ICU string transform.
pub mod string_transforms;

/// Type for UTF-16 code units.
#[allow(non_camel_case_types)]
pub type unichar = u16;

/// Constants representing an ICU string transform.
pub type StringTransform = *const String;

/// Size of UTF8 encoding
const UTF8_ENCODING: usize = 4;
