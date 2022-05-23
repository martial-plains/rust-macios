use std::{
    fmt::{self, Formatter},
    marker::PhantomData,
    ops::{Deref, DerefMut, Range},
};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    foundation::{Array, String, UInt},
    id,
    objective_c_runtime::traits::NSObject,
};

/// A dynamic ordered collection of objects.
pub struct MutableArray<T> {
    /// The underlying Objective-C object.
    pub obj: Id<Object>,
    _marker: PhantomData<T>,
}

impl<T> MutableArray<T>
where
    T: NSObject,
{
    /// Creates a new `MutableArray`.
    pub fn new() -> Self {
        MutableArray {
            obj: unsafe { msg_send![class!(NSMutableArray), array] },
            _marker: PhantomData,
        }
    }

    /// The number of objects in the array.
    pub fn count(&self) -> UInt {
        unsafe { msg_send![self.obj, count] }
    }

    /* Creating and Initializing a Mutable Array
     */

    /// Creates and returns an NSMutableArray object with enough allocated memory to initially hold a given number of objects.
    pub fn array_with_capacity(capacity: usize) -> Self {
        MutableArray {
            obj: unsafe { msg_send![class!(NSMutableArray), arrayWithCapacity: capacity] },
            _marker: PhantomData,
        }
    }

    /// Creates and returns a mutable array containing the contents of the file specified by the given path.
    pub fn array_with_contents_of_file<S>(path: S) -> Self
    where
        S: Into<String>,
    {
        MutableArray {
            obj: unsafe { msg_send![class!(NSMutableArray), arrayWithContentsOfFile: path.into()] },
            _marker: PhantomData,
        }
    }

    /// Creates and returns a mutable array containing the contents specified by a given URL.
    pub fn array_with_contents_of_url<S>(url: S) -> Self
    where
        S: Into<String>,
    {
        MutableArray {
            obj: unsafe { msg_send![class!(NSMutableArray), arrayWithContentsOfURL: url.into()] },
            _marker: PhantomData,
        }
    }

    /// Returns an array, initialized with enough memory to initially hold a given number of objects.
    pub fn init_with_capacity(capacity: UInt) -> Self {
        MutableArray {
            obj: unsafe { msg_send![class!(NSMutableArray), arrayWithCapacity: capacity] },
            _marker: PhantomData,
        }
    }

    /// Initializes a newly allocated mutable array with the contents of the file specified by a given path
    pub fn init_with_contents_of_file<S>(&mut self, path: S) -> bool
    where
        S: Into<String>,
    {
        unsafe { msg_send![&mut *self.obj, initWithContentsOfFile: path.into()] }
    }

    /* Adding Objects
     */

    /// Inserts a given object at the end of the array.
    pub fn add(&mut self, object: &T) {
        unsafe { msg_send![&mut *self.obj, addObject: object.retain().to_id()] }
    }

    /// Adds the objects contained in another given array to the end of the receiving array’s content.
    pub fn add_objects_from_array(&mut self, other_array: &Array<T>) {
        unsafe { msg_send![&mut *self.obj, addObjectsFromArray: other_array] }
    }

    /// Inserts a given object into the array’s contents at a given index.
    pub fn insert(&mut self, index: UInt, object: &T) {
        unsafe { msg_send![&mut *self.obj, insertObject: object.retain().to_id() atIndex: index] }
    }

    /* Removing Objects
     */

    /// Empties the array of all its elements.
    pub fn remove_all_objects(&mut self) {
        unsafe { msg_send![&mut *self.obj, removeAllObjects] }
    }

    /// Removes the object with the highest-valued index in the array
    pub fn remove_last_object(&mut self) {
        unsafe { msg_send![&mut *self.obj, removeLastObject] }
    }

    /// Removes all occurrences in the array of a given object.
    pub fn remove_object(&mut self, object: &T) {
        unsafe { msg_send![&mut *self.obj, removeObject: object.retain().to_id()] }
    }

    /// Removes all occurrences within a specified range in the array of a given object.
    pub fn remove_object_in_range(&mut self, object: &T, range: Range<UInt>) {
        unsafe { msg_send![&mut *self.obj, removeObject: object.retain().to_id() inRange: range] }
    }

    /// Removes the object at index .
    pub fn remove_object_at_index(&mut self, index: UInt) {
        unsafe { msg_send![&mut *self.obj, removeObjectAtIndex: index] }
    }

    /// Removes all occurrences of a given object in the array.
    pub fn remove_object_identical_to(&mut self, object: &T) {
        unsafe { msg_send![&mut *self.obj, removeObjectIdenticalTo: object.retain().to_id()] }
    }

    /// Removes all occurrences of anObject within the specified range in the array.
    pub fn remove_object_identical_to_in_range(&mut self, object: &T, range: Range<UInt>) {
        unsafe {
            msg_send![&mut *self.obj, removeObjectIdenticalTo: object.retain().to_id() inRange: range]
        }
    }

    /// Removes from the receiving array the objects in another given array.
    pub fn remove_objects_in_array(&mut self, other_array: &Array<T>) {
        unsafe { msg_send![&mut *self.obj, removeObjectsInArray: other_array] }
    }

    /// Removes from the array each of the objects within a given range.
    pub fn remove_objects_in_range(&mut self, range: Range<UInt>) {
        unsafe { msg_send![&mut *self.obj, removeObjectsInRange: range] }
    }

    /* Replacing Objects
     */

    /// Replaces the object at index with anObject.
    pub fn replace_object_at_index(&mut self, index: UInt, object: &T) {
        unsafe {
            msg_send![&mut *self.obj, replaceObjectAtIndex: index withObject: object.retain().to_id()]
        }
    }

    /// Sets the receiving array’s elements to those in another given array.
    pub fn set_array(&mut self, other_array: &Array<T>) {
        unsafe { msg_send![&mut *self.obj, setArray: other_array] }
    }
}

impl<T> Default for MutableArray<T>
where
    T: NSObject,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> NSObject for MutableArray<T> {
    fn init() -> Self {
        let obj: id = unsafe { msg_send![class!(NSMutableArray), init] };

        Self {
            obj: unsafe { Id::from_ptr(obj) },
            _marker: PhantomData,
        }
    }

    #[allow(trivial_casts)]
    fn to_id(self) -> id {
        &*self as *const _ as *mut _
    }

    fn from_id(_obj: id) -> Self {
        todo!()
    }

    fn description(&self) -> String {
        let obj: id = unsafe { msg_send![&*self.obj, description] };
        String::from_id(obj)
    }

    fn debug_description(&self) -> String {
        let obj: id = unsafe { msg_send![&*self.obj, debugDescription] };
        String::from_id(obj)
    }

    fn retain(&self) -> Self {
        let obj: id = unsafe { msg_send![&*self.obj, retain] };
        Self {
            obj: unsafe { Id::from_ptr(obj) },
            _marker: PhantomData,
        }
    }
}

impl fmt::Debug for MutableArray<id> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.debug_description())
    }
}

impl fmt::Display for MutableArray<id> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<T> Deref for MutableArray<T> {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.obj
    }
}

impl<T> DerefMut for MutableArray<T> {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.obj
    }
}

impl<T> Clone for MutableArray<T> {
    fn clone(&self) -> Self {
        self.retain()
    }
}
