use std::marker::PhantomData;

use objc::{class, msg_send, runtime::Object, sel, sel_impl};

use crate::{
    foundation::UInt,
    object,
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};

use super::{ns_array::INSArray, NSArray, NSRange, NSString};

object! {
    /// A dynamic ordered collection of objects.
    unsafe pub struct NSMutableArray<T> {
        _marker: PhantomData<T>,
    }
}

/// A mutable, static ordered collection of objects.
pub trait INSMutableArray<T>: INSArray<T>
where
    T: PNSObject,
{
    /* Creating and Initializing a Mutable Array
     */

    /// Creates and returns an NSMutableArray object with enough allocated memory to initially hold a given number of objects.
    fn tm_array_with_capacity(capacity: usize) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), arrayWithCapacity: capacity]) }
    }

    /// Creates and returns a mutable array containing the contents of the file specified by the given path.
    fn tm_array_with_contents_of_file(path: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), arrayWithContentsOfFile: path]) }
    }

    /// Creates and returns a mutable array containing the contents specified by a given URL.
    fn tm_array_with_contents_of_url(url: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), arrayWithContentsOfURL: url]) }
    }

    /// Returns an array, initialized with enough memory to initially hold a given number of objects.
    fn im_init_with_capacity(capacity: UInt) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            let cls = Self::m_class();
            let alloc: id = msg_send![cls, new];
            let init = msg_send![alloc, initWithCapacity: capacity];
            Self::from_id(init)
        }
    }

    /// Initializes a newly allocated mutable array with the contents of the file specified by a given path
    fn im_init_with_contents_of_file(&mut self, path: NSString) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), initWithContentsOfFile: path]) }
    }

    /* Adding Objects
     */

    /// Inserts a given object at the end of the array.
    fn im_add_object(&mut self, object: &T) {
        unsafe { msg_send![self.m_self(), addObject: object] }
    }

    /// Adds the objects contained in another given array to the end of the receiving array’s content.
    fn im_add_objects_from_array(&mut self, other_array: &NSArray<T>) {
        unsafe { msg_send![self.m_self(), addObjectsFromArray: other_array] }
    }

    /// Inserts a given object into the array’s contents at a given index.
    fn im_insert_object_at_index(&mut self, index: UInt, object: &T) {
        unsafe { msg_send![self.m_self(), insertObject: object atIndex: index] }
    }

    /* Removing Objects
     */

    /// Empties the array of all its elements.
    fn im_remove_all_objects(&mut self) {
        unsafe { msg_send![self.m_self(), removeAllObjects] }
    }

    /// Removes the object with the highest-valued index in the array
    fn im_remove_last_object(&mut self) {
        unsafe { msg_send![self.m_self(), removeLastObject] }
    }

    /// Removes all occurrences in the array of a given object.
    fn im_remove_object(&mut self, object: &T) {
        unsafe { msg_send![self.m_self(), removeObject: object] }
    }

    /// Removes all occurrences within a specified range in the array of a given object.
    fn im_remove_object_in_range(&mut self, object: &T, range: NSRange) {
        unsafe { msg_send![self.m_self(), removeObject: object inRange: range] }
    }

    /// Removes the object at index .
    fn im_remove_object_at_index(&mut self, index: UInt) {
        unsafe { msg_send![self.m_self(), removeObjectAtIndex: index] }
    }

    /// Removes all occurrences of a given object in the array.
    fn im_remove_object_identical_to(&mut self, object: &T) {
        unsafe { msg_send![self.m_self(), removeObjectIdenticalTo: object] }
    }

    /// Removes all occurrences of anObject within the specified range in the array.
    fn im_remove_object_identical_to_in_range(&mut self, object: &T, range: NSRange) {
        unsafe { msg_send![self.m_self(), removeObjectIdenticalTo: object inRange: range] }
    }

    /// Removes from the receiving array the objects in another given array.
    fn im_remove_objects_in_array(&mut self, other_array: &NSArray<T>) {
        unsafe { msg_send![self.m_self(), removeObjectsInArray: other_array.m_self()] }
    }

    /// Removes from the array each of the objects within a given range.
    fn im_remove_objects_in_range(&mut self, range: NSRange) {
        unsafe { msg_send![self.m_self(), removeObjectsInRange: range] }
    }

    /* Replacing Objects
     */

    /// Replaces the object at index with anObject.
    fn im_replace_object_at_index_with_object(&mut self, index: UInt, object: &T) {
        unsafe { msg_send![self.m_self(), replaceObjectAtIndex: index withObject: object] }
    }

    /// Sets the receiving array’s elements to those in another given array.
    fn im_set_array(&mut self, other_array: &NSArray<T>) {
        unsafe { msg_send![self.m_self(), setArray: other_array.m_self()] }
    }
}

impl<T> INSArray<T> for NSMutableArray<T> where T: PNSObject + FromId {}

impl<T> INSMutableArray<T> for NSMutableArray<T> where T: PNSObject + FromId {}

impl<T> From<id> for NSMutableArray<T> {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from(obj: id) -> Self {
        unsafe { Self::from_id(obj) }
    }
}

impl<T> From<&[T]> for NSMutableArray<T>
where
    T: PNSObject,
{
    fn from(array: &[T]) -> Self {
        unsafe {
            let cls: *mut Object = msg_send![class!(NSArray),
                arrayWithObjects:array.as_ptr()
                count:array.len()
            ];
            NSMutableArray::from(cls)
        }
    }
}

impl<T> From<UInt> for NSMutableArray<T>
where
    T: PNSObject,
{
    fn from(capacity: UInt) -> Self {
        unsafe {
            let cls: *mut Object = msg_send![class!(NSArray), arrayWithCapacity: capacity];
            NSMutableArray::from(cls)
        }
    }
}
