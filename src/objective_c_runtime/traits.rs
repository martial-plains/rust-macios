#![allow(non_snake_case)]

use crate::{foundation::String, id};

/// The group of methods that are fundamental to all Objective-C objects.
pub trait t_NSObject {
    /* Creating, Copying, and Deallocating Objects
     */

    /// Implemented by subclasses to initialize a new object (the receiver) immediately after memory for it has been allocated.
    fn init() -> Self;

    /// Returns a an `id`.
    fn toId(self) -> id;

    /// Returns `Self` representation of the object.
    fn fromId(obj: id) -> Self;

    /* Describing Objects
     */

    /// Returns a string that represents the contents of the receiving class.
    fn description(&self) -> String;

    /* Supporting Discardable Content
     */

    /// Returns a string that represents the contents of the receiving class.
    fn debugDescription(&self) -> String;

    /* Obselte Methods
     */

    /// Increments the receiver’s reference count.
    fn retain(&self) -> Self;
}

/// The group of methods that are fundamental to all Objective-C objects.
pub trait t_NSValue: t_NSObject {}
