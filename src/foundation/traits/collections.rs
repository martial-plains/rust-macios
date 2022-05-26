use std::ops::Range;

use crate::{
    foundation::{NSArray, NSDictionary, NSLocale, NSString, UInt},
    id,
    objective_c_runtime::traits::t_NSObject,
};

/// A static ordered collection of objects.
pub trait t_NSArray<T>
where
    T: t_NSObject,
{
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
    fn contains(&self, object: T) -> bool;

    /// The number of objects in the array.
    fn count(&self) -> UInt;

    /// The first object in the array.
    fn firstObject(&self) -> Option<T>;

    /// The last object in the array.
    fn lastObject(&self) -> Option<T>;

    /// The object at the specified index.
    fn objectAt(&self, index: UInt) -> T;

    /// The index of the specified object.
    fn objectAtIndexedSubscript(&self, index: UInt) -> Option<id>;

    /* Finding Objects in an Array
     */

    /// Returns the lowest index whose corresponding array value is equal to a given object.
    fn indexOf(&self, object: T) -> UInt;

    /// Returns the lowest index within a specified range whose corresponding array value is equal to a given object .
    fn indexOfObjectInRange(&self, object: T, range: Range<UInt>) -> UInt;

    /// Returns the lowest index whose corresponding array value is identical to a given object.
    fn indexOfObjectIdenticalTo(&self, object: T) -> UInt;

    /// Returns the lowest index within a specified range whose corresponding array value is equal to a given object .
    fn indexOfObjectIdenticalToInRange(&self, object: T, range: Range<UInt>) -> UInt;

    /* Comparing Arrays
     */

    /// Returns the first object contained in the receiving array that’s equal to an object in another given array.
    fn firstObjectCommonWith(&self, other: &NSArray<T>) -> Option<T>;

    /// Compares the receiving array to another array.
    fn isEqualTo(&self, other: &NSArray<T>) -> bool;

    /* Deriving New Arrays
     */

    /// Returns a new array that is a copy of the receiving array with a given object added to the end.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    unsafe fn adding(&self, object: T) -> NSArray<T>;

    /// Returns a new array that is a copy of the receiving array with the objects contained in another array added to the end.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    unsafe fn arrayByAddingObjectsFromArray<A>(&self, objects: A) -> NSArray<T>
    where
        A: t_NSArray<T>;

    /// Returns a new array containing the receiving array’s elements that fall within the limits specified by a given range.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    unsafe fn subarrayWithRange(&self, range: Range<UInt>) -> NSArray<T>;
    /* Creating a Description
     */

    /// A string that represents the contents of the array, formatted as a property list.

    /// Returns a string that represents the contents of the array, formatted as a property list.
    fn descriptionWithLocale(&self, locale: &NSLocale) -> NSString;

    /// Returns a string that represents the contents of the array, formatted as a property list.
    fn descriptionWithLocaleIndent(&self, locale: &NSLocale, indent: UInt) -> NSString;
}

/// A mutable, static ordered collection of objects.
pub trait t_NSMutableArray<T>: t_NSArray<T>
where
    T: t_NSObject,
{
    /* Creating and Initializing a Mutable Array
     */

    /// Creates and returns an NSMutableArray object with enough allocated memory to initially hold a given number of objects.
    fn arrayWithCapacity(capacity: usize) -> Self;

    /// Creates and returns a mutable array containing the contents of the file specified by the given path.
    fn array_with_contents_of_file<S>(path: S) -> Self
    where
        S: Into<NSString>;

    /// Creates and returns a mutable array containing the contents specified by a given URL.
    fn arrayWithContentsOfUrl<S>(url: S) -> Self
    where
        S: Into<NSString>;

    /// Returns an array, initialized with enough memory to initially hold a given number of objects.
    fn initWithCapacity(capacity: UInt) -> Self;

    /// Initializes a newly allocated mutable array with the contents of the file specified by a given path
    fn initWithContentsOfFile<S>(&mut self, path: S) -> bool
    where
        S: Into<NSString>;

    /* Adding Objects
     */

    /// Inserts a given object at the end of the array.
    fn add(&mut self, object: &T);

    /// Adds the objects contained in another given array to the end of the receiving array’s content.
    fn addObjectsFromArray(&mut self, other_array: &NSArray<T>);

    /// Inserts a given object into the array’s contents at a given index.
    fn insert(&mut self, index: UInt, object: &T);

    /* Removing Objects
     */

    /// Empties the array of all its elements.
    fn removeAllObjects(&mut self);

    /// Removes the object with the highest-valued index in the array
    fn removeLastObject(&mut self);

    /// Removes all occurrences in the array of a given object.
    fn removeObject(&mut self, object: &T);

    /// Removes all occurrences within a specified range in the array of a given object.
    fn removeObjectInRange(&mut self, object: &T, range: Range<UInt>);

    /// Removes the object at index .
    fn removeObjectAtIndex(&mut self, index: UInt);

    /// Removes all occurrences of a given object in the array.
    fn removeObjectIdenticalTo(&mut self, object: &T);

    /// Removes all occurrences of anObject within the specified range in the array.
    fn removeObjectIdenticalToInRange(&mut self, object: &T, range: Range<UInt>);

    /// Removes from the receiving array the objects in another given array.
    fn removeObjectsInArray(&mut self, other_array: &NSArray<T>);

    /// Removes from the array each of the objects within a given range.
    fn removeObjectsInRange(&mut self, range: Range<UInt>);

    /* Replacing Objects
     */

    /// Replaces the object at index with anObject.
    fn replaceObjectAtIndex(&mut self, index: UInt, object: &T);

    /// Sets the receiving array’s elements to those in another given array.
    fn setArray(&mut self, other_array: &NSArray<T>);
}

/// A dynamic collection of objects associated with unique keys.
pub trait t_NSMutableDictionary<K, V>: t_NSDictionary<K, V> {
    /// Creates and initialize a dictionary
    fn initWithDictionary(&mut self, dictionary: NSDictionary<K, V>);

    /* Adding Entries to a Mutable Dictionary
     */

    /// Adds a given key-value pair to the dictionary.
    fn setObject(&mut self, key: K, value: V)
    where
        K: t_NSObject,
        V: t_NSObject;

    /// Adds a given key-value pair to the dictionary.
    fn setObjectForKeyedSuperscript(&mut self, key: K, value: V)
    where
        K: Into<id>,
        V: Into<id>;

    /// Adds a given key-value pair to the dictionary.
    fn setValue(&mut self, key: K, value: V)
    where
        K: Into<NSString>,
        V: Into<id>;

    /// Adds to the receiving dictionary the entries from another dictionary.
    fn addEntriesFromDictionary(&mut self, dictionary: NSDictionary<K, V>);

    /// Sets the contents of the receiving dictionary to entries in a given dictionary.
    fn setDictionary(&mut self, dictionary: NSDictionary<K, V>);

    /* Removing Entries From a Mutable Dictionary
     */

    /// Removes a given key and its associated value from the dictionary.
    fn removeObjectForKey(&mut self, key: K)
    where
        K: Into<id>;

    /// Empties the dictionary of its entries.
    fn removeAllObjects(&mut self);

    /// Removes from the dictionary entries specified by elements in a given array.
    fn removeObjectsForKeys(&mut self, keys: NSArray<K>)
    where
        K: t_NSObject;
}

/// A static collection of objects associated with unique keys.
pub trait t_NSDictionary<K, V>: t_NSObject {
    /* Counting Entries
     */

    /// The number of entries in the dictionary.
    fn count(&self) -> UInt;
}
