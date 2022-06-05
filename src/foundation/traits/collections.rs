use std::ops::Range;

use crate::{
    foundation::{NSArray, NSDictionary, NSLocale, NSString, UInt},
    id,
    objective_c_runtime::traits::{FromId, PNSObject},
};

/// A static ordered collection of objects.
pub trait INSArray<T>: PNSObject {
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
    fn im_containsObject(&self, object: T) -> bool;

    /// The number of objects in the array.
    fn ip_count(&self) -> UInt;

    /// The first object in the array.
    fn ip_firstObject(&self) -> Option<T>
    where
        T: PNSObject + FromId;

    /// The last object in the array.
    fn ip_lastObject(&self) -> Option<T>
    where
        T: PNSObject + FromId;

    /// The object at the specified index.
    fn im_objectAtIndex(&self, index: UInt) -> T
    where
        T: PNSObject + FromId;

    /// The index of the specified object.
    fn im_objectAtIndexedSubscript(&self, index: UInt) -> Option<id>;

    /* Finding Objects in an Array
     */

    /// Returns the lowest index whose corresponding array value is equal to a given object.
    fn im_indexOfObject(&self, object: T) -> UInt;

    /// Returns the lowest index within a specified range whose corresponding array value is equal to a given object .
    fn im_indexOfObject_inRange(&self, object: T, range: Range<UInt>) -> UInt;

    /// Returns the lowest index whose corresponding array value is identical to a given object.
    fn im_indexOfObjectIdenticalTo(&self, object: T) -> UInt;

    /// Returns the lowest index within a specified range whose corresponding array value is equal to a given object .
    fn im_indexOfObjectIdenticalTo_inRange(&self, object: T, range: Range<UInt>) -> UInt;

    /* Comparing Arrays
     */

    /// Returns the first object contained in the receiving array that’s equal to an object in another given array.
    fn im_firstObjectCommonWithArray(&self, other: &NSArray<T>) -> Option<T>
    where
        T: PNSObject + FromId;

    /// Compares the receiving array to another array.
    fn im_isEqualToArray(&self, other: &NSArray<T>) -> bool;

    /* Deriving New Arrays
     */

    /// Returns a new array that is a copy of the receiving array with a given object added to the end.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    unsafe fn im_arrayByAddingObject(&self, object: T) -> NSArray<T>;

    /// Returns a new array that is a copy of the receiving array with the objects contained in another array added to the end.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    unsafe fn im_arrayByAddingObjectsFromArray<A>(&self, objects: A) -> NSArray<T>
    where
        A: INSArray<T>;

    /// Returns a new array containing the receiving array’s elements that fall within the limits specified by a given range.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    unsafe fn im_subarrayWithRange(&self, range: Range<UInt>) -> NSArray<T>;
    /* Creating a Description
     */

    /// A string that represents the contents of the array, formatted as a property list.

    /// Returns a string that represents the contents of the array, formatted as a property list.
    fn im_descriptionWithLocale(&self, locale: &NSLocale) -> NSString;

    /// Returns a string that represents the contents of the array, formatted as a property list.
    fn im_descriptionWithLocaleIndent(&self, locale: &NSLocale, indent: UInt) -> NSString;
}

/// A mutable, static ordered collection of objects.
pub trait INSMutableArray<T>: INSArray<T>
where
    T: PNSObject,
{
    /* Creating and Initializing a Mutable Array
     */

    /// Creates and returns an NSMutableArray object with enough allocated memory to initially hold a given number of objects.
    fn tm_arrayWithCapacity(capacity: usize) -> Self;

    /// Creates and returns a mutable array containing the contents of the file specified by the given path.
    fn tm_array_with_contents_of_file<S>(path: S) -> Self
    where
        S: Into<NSString>;

    /// Creates and returns a mutable array containing the contents specified by a given URL.
    fn tm_arrayWithContentsOfUrl<S>(url: S) -> Self
    where
        S: Into<NSString>;

    /// Returns an array, initialized with enough memory to initially hold a given number of objects.
    fn im_initWithCapacity(capacity: UInt) -> Self;

    /// Initializes a newly allocated mutable array with the contents of the file specified by a given path
    fn im_initWithContentsOfFile<S>(&mut self, path: S) -> bool
    where
        S: Into<NSString>;

    /* Adding Objects
     */

    /// Inserts a given object at the end of the array.
    fn im_addObject(&mut self, object: &T);

    /// Adds the objects contained in another given array to the end of the receiving array’s content.
    fn im_addObjectsFromArray(&mut self, other_array: &NSArray<T>);

    /// Inserts a given object into the array’s contents at a given index.
    fn im_insertObject_atIndex(&mut self, index: UInt, object: &T);

    /* Removing Objects
     */

    /// Empties the array of all its elements.
    fn im_removeAllObjects(&mut self);

    /// Removes the object with the highest-valued index in the array
    fn im_removeLastObject(&mut self);

    /// Removes all occurrences in the array of a given object.
    fn im_removeObject(&mut self, object: &T);

    /// Removes all occurrences within a specified range in the array of a given object.
    fn im_removeObject_inRange(&mut self, object: &T, range: Range<UInt>);

    /// Removes the object at index .
    fn im_removeObjectAtIndex(&mut self, index: UInt);

    /// Removes all occurrences of a given object in the array.
    fn im_removeObjectIdenticalTo(&mut self, object: &T);

    /// Removes all occurrences of anObject within the specified range in the array.
    fn im_removeObjectIdenticalTo_inRange(&mut self, object: &T, range: Range<UInt>);

    /// Removes from the receiving array the objects in another given array.
    fn im_removeObjectsInArray(&mut self, other_array: &NSArray<T>);

    /// Removes from the array each of the objects within a given range.
    fn im_removeObjectsInRange(&mut self, range: Range<UInt>);

    /* Replacing Objects
     */

    /// Replaces the object at index with anObject.
    fn im_replaceObjectAtIndex_withObject(&mut self, index: UInt, object: &T);

    /// Sets the receiving array’s elements to those in another given array.
    fn im_setArray(&mut self, other_array: &NSArray<T>);
}

/// A static collection of objects associated with unique keys.
pub trait INSDictionary<K, V>: PNSObject {
    /* Creating an Empty Dictionary
     */

    /// Creates an empty dictionary.
    fn tm_dictionary() -> Self;

    /// Initializes a newly allocated dictionary.
    fn im_init() -> Self;

    /* Creating a Dictionary from Another Dictionary
     */

    /// Creates a dictionary containing the keys and values from another given dictionary.
    fn tm_dictionaryWithDictionary<D>(dictionary: D) -> Self
    where
        D: INSDictionary<K, V>;

    /// Creates and initialize a dictionary
    fn im_initWithDictionary(&mut self, dictionary: NSDictionary<K, V>);

    /// Initializes a newly allocated dictionary using the objects contained in another given dictionary.
    fn im_initWithDictionary_copyItems(&mut self, dictionary: NSDictionary<K, V>, flag: bool);

    /* Counting Entries
     */

    /// The number of entries in the dictionary.
    fn ip_count(&self) -> UInt;

    /* Comparing Dictionaries
     */

    /// Returns a Boolean value that indicates whether the contents of the receiving dictionary are equal to the contents of another given dictionary.
    fn im_isEqualToDictionary<D>(&self, other: D) -> bool
    where
        D: INSDictionary<K, V>;

    /* Accessing Keys and Values
     */

    /// A new array containing the dictionary’s keys, or an empty array if the dictionary has no entries.
    fn ip_allKeys(&self) -> NSArray<K>;

    /// Returns a new array containing the keys corresponding to all occurrences of a given object in the dictionary.
    fn im_allKeysForObject(&self, anObject: &V) -> NSArray<K>;

    /// A new array containing the dictionary’s values, or an empty array if the dictionary has no entries.
    fn ip_allValues(&self) -> NSArray<V>;

    /// Returns the value associated with a given key.
    fn im_valueForKey(&self, key: &K) -> V
    where
        V: FromId;

    /// Returns by reference C arrays of the keys and values in the dictionary.
    fn im_getObjects_andKeys_count(&self, objects: *mut V, keys: *mut K, count: UInt);

    /// Returns as a static array the set of objects from the dictionary that corresponds to the specified keys.
    fn im_objectsForKeys_notFoundMarker(&self, keys: &NSArray<K>, value: &V) -> NSArray<V>;

    /// Returns the value associated with a given key.
    fn im_objectForKey(&self, key: K) -> V
    where
        V: FromId;

    /// Returns the value associated with a given key.
    fn im_objectForKeyedSubscript(&self, key: &K) -> V
    where
        V: FromId;
}

/// A dynamic collection of objects associated with unique keys.
pub trait INSMutableDictionary<K, V>: INSDictionary<K, V> {
    /* Creating and Initializing a Mutable Dictionary
     */

    /// Creates and returns a mutable dictionary, initially giving it enough allocated memory to hold a given number of entries.
    fn tm_dictionaryWithCapacity(capacity: UInt) -> Self;

    /* Adding Entries to a Mutable Dictionary
     */

    /// Adds a given key-value pair to the dictionary.
    fn im_setObject_forKey(&mut self, key: K, value: V)
    where
        K: PNSObject,
        V: PNSObject;

    /// Adds a given key-value pair to the dictionary.
    fn im_setObject_forkeyedSuperscript(&mut self, key: K, value: V)
    where
        K: Into<id>,
        V: Into<id>;

    /// Adds a given key-value pair to the dictionary.
    fn im_setValue_forKey(&mut self, key: K, value: V)
    where
        K: Into<NSString>,
        V: Into<id>;

    /// Adds to the receiving dictionary the entries from another dictionary.
    fn im_addEntriesFromDictionary(&mut self, dictionary: NSDictionary<K, V>);

    /// Sets the contents of the receiving dictionary to entries in a given dictionary.
    fn im_setDictionary(&mut self, dictionary: NSDictionary<K, V>);

    /* Removing Entries From a Mutable Dictionary
     */

    /// Removes a given key and its associated value from the dictionary.
    fn im_removeObjectForKey(&mut self, key: K)
    where
        K: Into<id>;

    /// Empties the dictionary of its entries.
    fn im_removeAllObjects(&mut self);

    /// Removes from the dictionary entries specified by elements in a given array.
    fn im_removeObjectsForKeys(&mut self, keys: NSArray<K>)
    where
        K: PNSObject;
}
