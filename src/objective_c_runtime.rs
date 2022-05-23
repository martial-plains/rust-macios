//! Gain low-level access to the Objective-C runtime and the Objective-C root types.

use crate::{foundation::String, id};

/// The group of methods that are fundamental to all Objective-C objects.
pub trait NSObject {
    /* Creating, Copying, and Deallocating Objects
     */

    /// Implemented by subclasses to initialize a new object (the receiver) immediately after memory for it has been allocated.
    fn init() -> Self;

    /// Returns a an `id`.
    fn to_id(self) -> id;

    /// Returns `Self` representation of the object.
    fn from_id(obj: id) -> Self;

    /* Describing Objects
     */

    /// Returns a string that represents the contents of the receiving class.
    fn description(&self) -> String;

    /* Supporting Discardable Content
     */

    /// Returns a string that represents the contents of the receiving class.
    fn debug_description(&self) -> String;
}
