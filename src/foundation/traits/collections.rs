use std::ops::Range;

use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::{NSArray, NSDictionary, NSLocale, NSString, UInt},
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject},
    },
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
    fn im_contains_object(&self, object: T) -> bool;

    /// The number of objects in the array.
    fn ip_count(&self) -> UInt;

    /// The first object in the array.
    fn ip_first_object(&self) -> Option<T>
    where
        T: PNSObject + FromId;

    /// The last object in the array.
    fn ip_last_object(&self) -> Option<T>
    where
        T: PNSObject + FromId;

    /// The object at the specified index.
    fn im_object_at_index(&self, index: UInt) -> T
    where
        T: PNSObject + FromId;

    /// The index of the specified object.
    fn im_object_at_indexed_subscript(&self, index: UInt) -> Option<id>;

    /* Finding Objects in an Array
     */

    /// Returns the lowest index whose corresponding array value is equal to a given object.
    fn im_index_of_object(&self, object: T) -> UInt;

    /// Returns the lowest index within a specified range whose corresponding array value is equal to a given object .
    fn im_index_of_object_in_range(&self, object: T, range: Range<UInt>) -> UInt;

    /// Returns the lowest index whose corresponding array value is identical to a given object.
    fn im_index_of_object_identical_to(&self, object: T) -> UInt;

    /// Returns the lowest index within a specified range whose corresponding array value is equal to a given object .
    fn im_index_of_object_identical_to_in_range(&self, object: T, range: Range<UInt>) -> UInt;

    /* Comparing Arrays
     */

    /// Returns the first object contained in the receiving array that’s equal to an object in another given array.
    fn im_first_object_common_with_array(&self, other: &NSArray<T>) -> Option<T>
    where
        T: PNSObject + FromId;

    /// Compares the receiving array to another array.
    fn im_is_equal_to_array(&self, other: &NSArray<T>) -> bool;

    /* Deriving New Arrays
     */

    /// Returns a new array that is a copy of the receiving array with a given object added to the end.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    unsafe fn im_array_by_adding_object(&self, object: T) -> NSArray<T>;

    /// Returns a new array that is a copy of the receiving array with the objects contained in another array added to the end.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    unsafe fn im_array_by_adding_objects_from_array<A>(&self, objects: A) -> NSArray<T>
    where
        A: INSArray<T>;

    /// Returns a new array containing the receiving array’s elements that fall within the limits specified by a given range.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    unsafe fn im_subarray_with_range(&self, range: Range<UInt>) -> NSArray<T>;
    /* Creating a Description
     */

    /// A string that represents the contents of the array, formatted as a property list.

    /// Returns a string that represents the contents of the array, formatted as a property list.
    fn im_description_with_locale(&self, locale: &NSLocale) -> NSString;

    /// Returns a string that represents the contents of the array, formatted as a property list.
    fn im_description_with_locale_indent(&self, locale: &NSLocale, indent: UInt) -> NSString;
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
        unsafe { Self::from_id(msg_send![Self::im_class(), arrayWithCapacity: capacity]) }
    }

    /// Creates and returns a mutable array containing the contents of the file specified by the given path.
    fn tm_array_with_contents_of_file(path: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), arrayWithContentsOfFile: path]) }
    }

    /// Creates and returns a mutable array containing the contents specified by a given URL.
    fn tm_array_with_contents_of_url(url: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), arrayWithContentsOfURL: url]) }
    }

    /// Returns an array, initialized with enough memory to initially hold a given number of objects.
    fn im_init_with_capacity(capacity: UInt) -> Self;

    /// Initializes a newly allocated mutable array with the contents of the file specified by a given path
    fn im_init_with_contents_of_file(&mut self, path: NSString) -> bool;

    /* Adding Objects
     */

    /// Inserts a given object at the end of the array.
    fn im_add_object(&mut self, object: &T);

    /// Adds the objects contained in another given array to the end of the receiving array’s content.
    fn im_add_objects_from_array(&mut self, other_array: &NSArray<T>);

    /// Inserts a given object into the array’s contents at a given index.
    fn im_insert_object_at_index(&mut self, index: UInt, object: &T);

    /* Removing Objects
     */

    /// Empties the array of all its elements.
    fn im_remove_all_objects(&mut self);

    /// Removes the object with the highest-valued index in the array
    fn im_remove_last_object(&mut self);

    /// Removes all occurrences in the array of a given object.
    fn im_remove_object(&mut self, object: &T);

    /// Removes all occurrences within a specified range in the array of a given object.
    fn im_remove_object_in_range(&mut self, object: &T, range: Range<UInt>);

    /// Removes the object at index .
    fn im_remove_object_at_index(&mut self, index: UInt);

    /// Removes all occurrences of a given object in the array.
    fn im_remove_object_identical_to(&mut self, object: &T);

    /// Removes all occurrences of anObject within the specified range in the array.
    fn im_remove_object_identical_to_in_range(&mut self, object: &T, range: Range<UInt>);

    /// Removes from the receiving array the objects in another given array.
    fn im_remove_objects_in_array(&mut self, other_array: &NSArray<T>);

    /// Removes from the array each of the objects within a given range.
    fn im_remove_objects_in_range(&mut self, range: Range<UInt>);

    /* Replacing Objects
     */

    /// Replaces the object at index with anObject.
    fn im_replace_object_at_index_with_object(&mut self, index: UInt, object: &T);

    /// Sets the receiving array’s elements to those in another given array.
    fn im_set_array(&mut self, other_array: &NSArray<T>);
}

/// A static collection of objects associated with unique keys.
pub trait INSDictionary<K, V>: PNSObject {
    /* Creating an Empty Dictionary
     */

    /// Creates an empty dictionary.
    fn tm_dictionary() -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), dictionary]) }
    }

    /// Initializes a newly allocated dictionary.
    fn im_init() -> Self;

    /* Creating a Dictionary from Another Dictionary
     */

    /// Creates a dictionary containing the keys and values from another given dictionary.
    fn tm_dictionary_with_dictionary<D>(dictionary: D) -> Self
    where
        Self: Sized + FromId,
        D: INSDictionary<K, V>,
    {
        unsafe {
            Self::from_id(msg_send![
                Self::im_class(),
                dictionaryWithDictionary: dictionary
            ])
        }
    }

    /// Creates and initialize a dictionary
    fn im_init_with_dictionary(&mut self, dictionary: NSDictionary<K, V>);

    /// Initializes a newly allocated dictionary using the objects contained in another given dictionary.
    fn im_init_with_dictionary_copy_items(&mut self, dictionary: NSDictionary<K, V>, flag: bool);

    /* Counting Entries
     */

    /// The number of entries in the dictionary.
    fn ip_count(&self) -> UInt;

    /* Comparing Dictionaries
     */

    /// Returns a Boolean value that indicates whether the contents of the receiving dictionary are equal to the contents of another given dictionary.
    fn im_is_equal_to_dictionary<D>(&self, other: D) -> bool
    where
        D: INSDictionary<K, V>;

    /* Accessing Keys and Values
     */

    /// A new array containing the dictionary’s keys, or an empty array if the dictionary has no entries.
    fn ip_all_keys(&self) -> NSArray<K>;

    /// Returns a new array containing the keys corresponding to all occurrences of a given object in the dictionary.
    fn im_all_keys_for_object(&self, object: &V) -> NSArray<K>;

    /// A new array containing the dictionary’s values, or an empty array if the dictionary has no entries.
    fn ip_all_values(&self) -> NSArray<V>;

    /// Returns the value associated with a given key.
    fn im_value_for_key(&self, key: &K) -> V
    where
        V: FromId;

    /// Returns by reference C arrays of the keys and values in the dictionary.
    fn im_get_objects_and_keys_count(&self, objects: *mut V, keys: *mut K, count: UInt);

    /// Returns as a static array the set of objects from the dictionary that corresponds to the specified keys.
    fn im_objects_for_keys_not_found_marker(&self, keys: &NSArray<K>, value: &V) -> NSArray<V>;

    /// Returns the value associated with a given key.
    fn im_object_for_key(&self, key: K) -> V
    where
        V: FromId;

    /// Returns the value associated with a given key.
    fn im_object_for_keyed_subscript(&self, key: &K) -> V
    where
        V: FromId;
}

/// A dynamic collection of objects associated with unique keys.
pub trait INSMutableDictionary<K, V>: INSDictionary<K, V> {
    /* Creating and Initializing a Mutable Dictionary
     */

    /// Creates and returns a mutable dictionary, initially giving it enough allocated memory to hold a given number of entries.
    fn tm_dictionary_with_capacity(capacity: UInt) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                Self::im_class(),
                dictionaryWithCapacity: capacity
            ])
        }
    }

    /* Adding Entries to a Mutable Dictionary
     */

    /// Adds a given key-value pair to the dictionary.
    fn im_set_object_for_key(&mut self, key: K, value: V)
    where
        K: PNSObject,
        V: PNSObject;

    /// Adds a given key-value pair to the dictionary.
    fn im_set_object_forkeyed_superscript(&mut self, key: K, value: V)
    where
        K: Into<id>,
        V: Into<id>;

    /// Adds a given key-value pair to the dictionary.
    fn im_set_value_for_key(&mut self, key: K, value: V)
    where
        K: Into<NSString>,
        V: Into<id>;

    /// Adds to the receiving dictionary the entries from another dictionary.
    fn im_add_entries_from_dictionary(&mut self, dictionary: NSDictionary<K, V>);

    /// Sets the contents of the receiving dictionary to entries in a given dictionary.
    fn im_set_dictionary(&mut self, dictionary: NSDictionary<K, V>);

    /* Removing Entries From a Mutable Dictionary
     */

    /// Removes a given key and its associated value from the dictionary.
    fn im_remove_object_for_key(&mut self, key: K)
    where
        K: Into<id>;

    /// Empties the dictionary of its entries.
    fn im_remove_all_objects(&mut self);

    /// Removes from the dictionary entries specified by elements in a given array.
    fn im_remove_objects_for_keys(&mut self, keys: NSArray<K>)
    where
        K: PNSObject;
}
