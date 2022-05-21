//! Gain low-level access to the Objective-C runtime and the Objective-C root types.

use crate::foundation::String;

/// The group of methods that are fundamental to all Objective-C objects.
pub trait NSObjectProtocol {
    /// Returns a string that represents the contents of the receiving class.
    fn description(&self) -> String;

    /// Returns a string that represents the contents of the receiving class.
    fn debug_description(&self) -> String;
}
