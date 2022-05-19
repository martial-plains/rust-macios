use objc::runtime::{BOOL, NO, YES};

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
