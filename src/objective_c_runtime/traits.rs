use objc::{
    msg_send,
    runtime::{Class, Protocol, Sel},
    sel, sel_impl,
};

use crate::{
    foundation::{NSString, UInt},
    utils::to_bool,
};

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
    ///  Allocates a new instance of the receiving class, sends it an init message, and returns the initialized object.
    fn m_new() -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), new]) }
    }

    /* Identifying Classes
     */

    /// Returns the class object for the receiver’s class.
    fn m_class<'a>() -> &'a Class;

    /// Returns the class object for the receiver’s superclass.
    fn ip_superclass<'a>() -> Option<&'a Class> {
        Self::m_class().superclass()
    }

    /* Identifying and Comparing Objects
     */

    /// Returns a Boolean value that indicates whether the receiver and a given object are equal.
    fn m_is_equal(&self, object: &Self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isEqual: object]) }
    }

    /// Returns an integer that can be used as a table address in a hash table structure.
    fn p_hash(&self) -> UInt {
        unsafe { msg_send![self.m_self(), hash] }
    }

    /// Returns the receiver.
    fn m_self(&self) -> id;

    /* Testing Object Inheritance, Behavior, and Conformance
     */

    /// Returns a Boolean value that indicates whether the receiver is an instance of given class or an instance of any class that inherits from that class.
    fn m_is_kind_of_class(&self, class: Class) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isKindOfClass: class]) }
    }

    /// Returns a Boolean value that indicates whether the receiver is an instance of a given class.
    fn m_is_member_of_class(&self, class: Class) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isMemberOfClass: class]) }
    }

    /// Returns a Boolean value that indicates whether the receiver implements or inherits a method that can respond to a specified message.
    fn m_responds_to_selector(&self, selector: Sel) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), respondsToSelector: selector]) }
    }

    /// Returns a Boolean value that indicates whether the receiver conforms to a given protocol.
    fn m_conforms_to_protocol(&self, protocol: Protocol) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), conformsToProtocol: protocol]) }
    }

    /* Describing Objects
     */

    /// A textual representation of the receiver.
    fn p_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), description]) }
    }

    /// A textual representation of the receiver to use with a debugger.
    fn p_debug_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), debugDescription]) }
    }

    /* Sending Messages
     */

    /// Sends a specified message to the receiver and returns the result of the message.
    fn m_perform_selector(&self, selector: Sel) -> id {
        unsafe { msg_send![self.m_self(), performSelector: selector] }
    }

    /// Sends a message to the receiver with an object as the argument.
    fn m_perform_selector_with_object(&self, selector: Sel, with_object: id) -> id {
        unsafe { msg_send![self.m_self(), performSelector: selector withObject: with_object] }
    }

    /* Identifying Proxies
     */

    /// Returns a Boolean value that indicates whether the receiver does not descend from NSObject.
    fn m_is_proxy(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isProxy]) }
    }
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
