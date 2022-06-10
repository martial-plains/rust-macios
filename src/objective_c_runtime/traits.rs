use objc::runtime::{Class, Protocol, Sel};

use crate::foundation::{NSString, UInt};

use super::id;

/// The group of methods that are fundamental to all Objective-C objects.
pub trait INSObject {
    /* Creating, Copying, and Deallocating Objects
     */

    /// Implemented by subclasses to initialize a new object (the receiver) immediately after memory for it has been allocated.
    fn new() -> Self;

    /// Returns a an `id`.
    fn to_id(self) -> id;

    /// Returns `Self` representation of the object.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    unsafe fn from_id(obj: id) -> Self;

    /* Describing Objects
     */

    /// Returns a string that represents the contents of the receiving class.
    fn description(&self) -> NSString;

    /* Supporting Discardable Content
     */

    /// Returns a string that represents the contents of the receiving class.
    fn debug_description(&self) -> NSString;

    /* Obselte Methods
     */

    /// Increments the receiver’s reference count.
    fn retain(&self) -> Self;
}

/// The group of methods that are fundamental to all Objective-C objects.
pub trait PNSObject {
    /* Identifying Classes
     */

    /// Returns the class object for the receiver’s class.
    fn im_class<'a>() -> &'a Class;

    /// Returns the class object for the receiver’s superclass.
    fn ip_superclass<'a>() -> Option<&'a Class> {
        Self::im_class().superclass()
    }

    /* Identifying and Comparing Objects
     */

    /// Returns a Boolean value that indicates whether the receiver and a given object are equal.
    fn im_is_equal(&self, object: &Self) -> bool;

    /// Returns an integer that can be used as a table address in a hash table structure.
    fn ip_hash(&self) -> UInt;

    /* Testing Object Inheritance, Behavior, and Conformance
     */

    /// Returns a Boolean value that indicates whether the receiver is an instance of given class or an instance of any class that inherits from that class.
    fn im_is_kind_of_class(&self, class: Class) -> bool;

    /// Returns a Boolean value that indicates whether the receiver is an instance of a given class.
    fn im_is_member_of_class(&self, class: Class) -> bool;

    /// Returns a Boolean value that indicates whether the receiver implements or inherits a method that can respond to a specified message.
    fn im_responds_to_selector(&self, selector: Sel) -> bool;

    /// Returns a Boolean value that indicates whether the receiver conforms to a given protocol.
    fn im_conforms_to_protocol(&self, protocol: Protocol) -> bool;

    /* Describing Objects
     */

    /// A textual representation of the receiver.
    fn ip_description(&self) -> NSString;

    /// A textual representation of the receiver to use with a debugger.
    fn ip_debug_description(&self) -> NSString;

    /* Sending Messages
     */

    /// Sends a specified message to the receiver and returns the result of the message.
    fn im_perform_selector(&self, selector: Sel) -> id;

    /// Sends a message to the receiver with an object as the argument.
    fn im_perform_selector_with_object(&self, selector: Sel, with_object: id) -> id;

    /* Identifying Proxies
     */

    /// Returns a Boolean value that indicates whether the receiver does not descend from NSObject.
    fn im_is_proxy(&self) -> bool;
}

/// The group of methods that are fundamental to all Objective-C objects.
pub trait INSValue: PNSObject {}

/// Converting an Objective-C pointer to Object
pub trait FromId: ToId {
    /// Returns `Self` representation of the object.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    unsafe fn from_id(ptr: id) -> Self;
}

/// Converting Object to an Objective-C pointer
pub trait ToId {
    /// Returns `id` representation of the object.
    fn to_id(self) -> id;
}
