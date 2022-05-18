use objc::runtime::Object;
pub use objc::runtime::{BOOL, NO, YES};

/// An ID for an Objective-C object.
#[allow(non_camel_case_types)]
pub type id = *mut Object;

mod integers;
mod strings;

pub use integers::*;
pub use strings::*;

/// A helper function to convert an Objective-C bool to a Rust bool.
#[inline(always)]
pub fn to_bool(result: BOOL) -> bool {
    match result {
        YES => true,
        NO => false,

        #[cfg(not(target_arch = "aarch64"))]
        _ => {
            std::unreachable!();
        }
    }
}
