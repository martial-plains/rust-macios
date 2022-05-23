use std::{
    fmt::{self, Debug, Display, Formatter},
    marker::PhantomData,
    ops::{Deref, DerefMut, Range},
};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    foundation::{Locale, String, UInt},
    id,
    objective_c_runtime::NSObject,
    utils::to_bool,
};

use self::iter::Iter;

/// An iterator for Arrays
pub mod iter;

/// A static ordered collection of objects.
pub struct Array<T> {
    /// The underlying Objective-C object.
    pub obj: Id<Object>,
    _marker: PhantomData<T>,
}

impl<T> Array<T> {
    /// Creates a new `Array` from a raw pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    pub fn new(ptr: *mut Object) -> Self {
        Self {
            obj: unsafe { Id::from_ptr(ptr) },
            _marker: PhantomData,
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
        T: NSObject,
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
            _marker: PhantomData,
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
        T: NSObject,
    {
        unsafe { to_bool(msg_send![&*self.obj, containsObject: object]) }
    }

    /// The number of objects in the array.
    pub fn count(&self) -> UInt {
        unsafe { msg_send![self.obj, count] }
    }

    /// The first object in the array.
    pub fn first_object(&self) -> Option<T>
    where
        T: NSObject,
    {
        unsafe {
            let ptr: *mut Object = msg_send![&*self.obj, firstObject];
            if ptr.is_null() {
                None
            } else {
                Some(T::from_id(ptr))
            }
        }
    }

    /// The last object in the array.
    pub fn last_object(&self) -> Option<T>
    where
        T: NSObject,
    {
        unsafe {
            let ptr: *mut Object = msg_send![&*self.obj, lastObject];
            if ptr.is_null() {
                None
            } else {
                Some(T::from_id(ptr))
            }
        }
    }

    /// The object at the specified index.
    pub fn object_at(&self, index: UInt) -> T
    where
        T: NSObject,
    {
        unsafe {
            let ptr: *mut Object = msg_send![&*self.obj, objectAtIndex: index];
            T::from_id(ptr)
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
        T: NSObject,
    {
        unsafe { msg_send![&*self.obj, indexOfObject: object] }
    }

    /// Returns the lowest index within a specified range whose corresponding array value is equal to a given object .
    pub fn index_of_object_in_range(&self, object: T, range: Range<UInt>) -> UInt
    where
        T: NSObject,
    {
        unsafe { msg_send![self.obj, indexOfObject: object inRange: range] }
    }

    /// Returns the lowest index whose corresponding array value is identical to a given object.
    pub fn index_of_object_identical_to(&self, object: T) -> UInt
    where
        T: NSObject,
    {
        unsafe { msg_send![self.obj, indexOfObjectIdenticalTo: object] }
    }

    /// Returns the lowest index within a specified range whose corresponding array value is equal to a given object .
    pub fn index_of_object_identical_to_in_range(&self, object: T, range: Range<UInt>) -> UInt
    where
        T: NSObject,
    {
        unsafe { msg_send![self.obj, indexOfObjectIdenticalTo: object inRange: range] }
    }

    /* Comparing Arrays
     */

    /// Returns the first object contained in the receiving array that’s equal to an object in another given array.
    pub fn first_object_common_with(&self, other: &Array<T>) -> Option<T>
    where
        T: NSObject,
    {
        unsafe {
            let ptr: *mut Object =
                msg_send![&*self.obj, firstObjectCommonWithArray: other.clone().obj];
            if ptr.is_null() {
                None
            } else {
                Some(T::from_id(ptr))
            }
        }
    }

    /// Compares the receiving array to another array.
    pub fn is_equal_to(&self, other: &Array<T>) -> bool
    where
        T: NSObject,
    {
        unsafe { to_bool(msg_send![&*self.obj, isEqualToArray: other.clone().obj]) }
    }

    /* Deriving New Arrays
     */

    /// Returns a new array that is a copy of the receiving array with a given object added to the end.
    pub fn adding(&self, object: T) -> Array<T>
    where
        T: NSObject,
    {
        Array::new(unsafe { msg_send![&*self.obj, arrayByAddingObject: object] })
    }

    /// Returns a new array that is a copy of the receiving array with the objects contained in another array added to the end.
    pub fn adding_objects(&self, objects: &Array<T>) -> Array<T>
    where
        T: NSObject,
    {
        Array::new(unsafe {
            msg_send![self.obj, arrayByAddingObjectsFromArray: objects.clone().obj]
        })
    }

    /// Returns a new array containing the receiving array’s elements that fall within the limits specified by a given range.
    pub fn subarray_with_range(&self, range: Range<UInt>) -> Array<T>
    where
        T: NSObject,
    {
        Array::new(unsafe { msg_send![&*self.obj, subarrayWithRange: range] })
    }

    /* Creating a Description
     */

    /// A string that represents the contents of the array, formatted as a property list.

    /// Returns a string that represents the contents of the array, formatted as a property list.
    pub fn description_with_locale(&self, locale: &Locale) -> String {
        unsafe { msg_send![&*self.obj, descriptionWithLocale: locale.clone().obj] }
    }

    /// Returns a string that represents the contents of the array, formatted as a property list.
    pub fn description_with_locale_indent(&self, locale: &Locale, indent: UInt) -> String {
        unsafe { msg_send![&*self.obj, descriptionWithLocale: locale.clone().obj indent: indent] }
    }

    /* Rust Conversions
     */

    /// Returns an iterator over the objects in the array.
    pub fn iter(&self) -> Iter<'_, T>
    where
        T: NSObject,
    {
        Iter {
            array: self,
            index: 0,
        }
    }
}

impl<T> NSObject for Array<T> {
    fn init() -> Self {
        let obj: id = unsafe { msg_send![class!(NSArray), init] };

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

impl<T> Debug for Array<T>
where
    T: Debug + NSObject,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.debug_description())
    }
}

impl<T> Display for Array<T>
where
    T: Display + NSObject,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<T> Clone for Array<T> {
    fn clone(&self) -> Self {
        Array::new(unsafe { msg_send![self.obj, retain] })
    }
}

impl<'a, T> IntoIterator for &'a Array<T>
where
    T: NSObject,
{
    type Item = T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> From<Vec<T>> for Array<T>
where
    T: NSObject,
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

impl<T> From<MutableArray<T>> for Array<T>
where
    T: NSObject,
{
    /// Given an `Array` of `Object`s, creates a new `Array` that holds them.
    fn from(array: MutableArray<T>) -> Self {
        let arr: MutableArray<T> = MutableArray::init();
        Array::<T>::new(unsafe { msg_send![arr.obj, arrayWithArray: array] })
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

impl Debug for MutableArray<id> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.debug_description())
    }
}

impl Display for MutableArray<id> {
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
