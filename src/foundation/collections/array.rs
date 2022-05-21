use std::{
    fmt::Debug,
    ops::{Deref, DerefMut, Range},
};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    foundation::{Locale, String, UInt},
    id,
    utils::to_bool,
};

/// An iterator for `Array`
#[allow(missing_debug_implementations)]
pub struct ArrayIter<'a, T> {
    array: &'a Array<T>,
    index: UInt,
}

/// A static ordered collection of objects.
pub struct Array<T> {
    /// The underlying Objective-C object.
    pub obj: Id<Object>,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T> Iterator for ArrayIter<'a, T>
where
    T: Debug + From<id>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.array.count() {
            None
        } else {
            let item = self.array.object_at(self.index);
            self.index += 1;
            Some(item)
        }
    }
}

impl<T> Array<T> {
    /// Creates a new `Array` from a raw pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn new(ptr: *mut Object) -> Self {
        Self {
            obj: unsafe { Id::from_ptr(ptr) },
            _marker: std::marker::PhantomData,
        }
    }

    /// Creates a new array with the specified capacity.
    ///
    /// # Arguments
    ///
    /// * `objects` - Collection of objects to make into an `Array`.
    ///
    /// # Returns
    ///
    /// A new array with the specified capacity.
    pub fn from_objects(objects: &[T]) -> Self
    where
        T: From<id>,
    {
        Array::new(unsafe {
            msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ]
        })
    }

    /// In some cases, we're vended an `NSArray` by the system, and it's ideal to not retain that.
    /// This handles that edge case.
    ///
    /// # Arguments
    ///
    /// * `array` - The `NSArray` to dereference.
    ///
    /// # Returns
    ///
    /// The dereferenced `NSArray`.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it might dereference a raw pointer.
    pub unsafe fn from_retained(array: id) -> Self {
        Array {
            obj: Id::from_retained_ptr(array),
            _marker: std::marker::PhantomData,
        }
    }

    /* Querying an Array
     */

    /// Returns a Boolean value that indicates whether a given object is present in the array.
    ///
    /// # Arguments
    ///
    /// * `object` - An object to look for in the array..
    ///
    /// # Returns
    ///
    /// A Boolean value that indicates whether `object` is present in the array.
    ///
    pub fn contains(&self, object: T) -> bool
    where
        T: From<id>,
    {
        unsafe { to_bool(msg_send![self.obj, containsObject: object]) }
    }

    /// The number of objects in the array.
    pub fn count(&self) -> UInt {
        unsafe { msg_send![self.obj, count] }
    }

    /// The first object in the array.
    pub fn first_object(&self) -> Option<T>
    where
        T: From<id>,
    {
        unsafe {
            let ptr: *mut Object = msg_send![&*self.obj, firstObject];
            if ptr.is_null() {
                None
            } else {
                Some(ptr.into())
            }
        }
    }

    /// The last object in the array.
    pub fn last_object(&self) -> Option<T>
    where
        T: From<id>,
    {
        unsafe {
            let ptr: *mut Object = msg_send![&*self.obj, lastObject];
            if ptr.is_null() {
                None
            } else {
                Some(ptr.into())
            }
        }
    }

    /// The object at the specified index.
    pub fn object_at(&self, index: UInt) -> T
    where
        T: From<id>,
    {
        unsafe {
            let ptr: *mut Object = msg_send![&*self.obj, objectAtIndex: index];
            ptr.into()
        }
    }

    /// The index of the specified object.
    pub fn object_at_indexed_subscript(&self, index: UInt) -> Option<id> {
        unsafe {
            let ptr: *mut Object = msg_send![&*self.obj, objectAtIndexedSubscript: index];
            ptr.into()
        }
    }

    /* Finding Objects in an Array
     */

    /// Returns the lowest index whose corresponding array value is equal to a given object.
    pub fn index_of(&self, object: T) -> UInt
    where
        T: From<id>,
    {
        unsafe { msg_send![self.obj, indexOfObject: object] }
    }

    /// Returns the lowest index within a specified range whose corresponding array value is equal to a given object .
    pub fn index_of_object_in_range(&self, object: T, range: Range<UInt>) -> UInt
    where
        T: From<id>,
    {
        unsafe { msg_send![self.obj, indexOfObject: object inRange: range] }
    }

    /// Returns the lowest index whose corresponding array value is identical to a given object.
    pub fn index_of_object_identical_to(&self, object: T) -> UInt
    where
        T: From<id>,
    {
        unsafe { msg_send![self.obj, indexOfObjectIdenticalTo: object] }
    }

    /// Returns the lowest index within a specified range whose corresponding array value is equal to a given object .
    pub fn index_of_object_identical_to_in_range(&self, object: T, range: Range<UInt>) -> UInt
    where
        T: From<id>,
    {
        unsafe { msg_send![self.obj, indexOfObjectIdenticalTo: object inRange: range] }
    }

    /* Comparing Arrays
     */

    /// Returns the first object contained in the receiving array that’s equal to an object in another given array.
    pub fn first_object_common_with(&self, other: &Array<T>) -> Option<T>
    where
        T: From<id>,
    {
        unsafe {
            let ptr: *mut Object =
                msg_send![&*self.obj, firstObjectCommonWithArray: other.clone().obj];
            if ptr.is_null() {
                None
            } else {
                Some(ptr.into())
            }
        }
    }

    /// Compares the receiving array to another array.
    pub fn is_equal_to(&self, other: &Array<T>) -> bool
    where
        T: From<id>,
    {
        unsafe { to_bool(msg_send![self.obj, isEqualToArray: other.clone().obj]) }
    }

    /* Deriving New Arrays
     */

    /// Returns a new array that is a copy of the receiving array with a given object added to the end.
    pub fn adding(&self, object: T) -> Array<T>
    where
        T: From<id>,
    {
        Array::new(unsafe { msg_send![self.obj, arrayByAddingObject: object] })
    }

    /// Returns a new array that is a copy of the receiving array with the objects contained in another array added to the end.
    pub fn adding_objects(&self, objects: &Array<T>) -> Array<T>
    where
        T: From<id>,
    {
        Array::new(unsafe {
            msg_send![self.obj, arrayByAddingObjectsFromArray: objects.clone().obj]
        })
    }

    /// Returns a new array containing the receiving array’s elements that fall within the limits specified by a given range.
    pub fn subarray_with_range(&self, range: Range<UInt>) -> Array<T>
    where
        T: From<id>,
    {
        Array::new(unsafe { msg_send![self.obj, subarrayWithRange: range] })
    }

    /* Creating a Description
     */

    /// A string that represents the contents of the array, formatted as a property list.
    pub fn description(&self) -> String {
        unsafe { msg_send![self.obj, description] }
    }

    /// Returns a string that represents the contents of the array, formatted as a property list.
    pub fn description_with_locale(&self, locale: &Locale) -> String {
        unsafe { msg_send![self.obj, descriptionWithLocale: locale.clone().obj] }
    }

    /// Returns a string that represents the contents of the array, formatted as a property list.
    pub fn description_with_locale_indent(&self, locale: &Locale, indent: UInt) -> String {
        unsafe { msg_send![self.obj, descriptionWithLocale: locale.clone().obj indent: indent] }
    }

    /* Rust Conversions
     */

    /// Returns an iterator over the objects in the array.
    pub fn iter(&self) -> ArrayIter<'_, T>
    where
        T: Debug + From<id>,
    {
        ArrayIter {
            array: self,
            index: 0,
        }
    }
}

impl<T> Debug for Array<T>
where
    T: Debug + From<id>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for (i, obj) in self.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", obj)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T> Clone for Array<T> {
    fn clone(&self) -> Self {
        Array::new(unsafe { msg_send![self.obj, retain] })
    }
}

impl<T> From<Vec<T>> for Array<T>
where
    T: From<id>,
{
    /// Given a set of `Object`s, creates an `Array` that holds them.
    fn from(objects: Vec<T>) -> Self {
        Array::new(unsafe {
            msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ]
        })
    }
}

impl<T> From<Array<T>> for *mut Object {
    /// Consumes and returns the pointer to the underlying NSArray.
    #[allow(trivial_casts)]
    fn from(array: Array<T>) -> Self {
        &*array as *const _ as *mut _
    }
}

impl<T> Deref for Array<T> {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.obj
    }
}

impl<T> DerefMut for Array<T> {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.obj
    }
}
