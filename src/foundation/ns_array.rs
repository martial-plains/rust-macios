use std::{marker::PhantomData, slice};

use libc::c_char;
use objc::{class, msg_send, runtime::Object, sel, sel_impl};

use crate::{
    foundation::NSString,
    objective_c_runtime::{
        id,
        macros::object,
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};

use self::iter::Iter;

use super::{ns_mutable_array::NSMutableArray, NSLocale, NSNumber, NSRange, UInt, UInt8};

/// Iterator for Array
pub mod iter;

object! {
    /// A static ordered collection of objects.
    unsafe pub struct NSArray<T> {
        _marker: PhantomData<T>,
    }
}

impl<T> NSArray<T> {
    /// Creates an iterator.
    pub fn iter(&self) -> Iter<'_, T>
    where
        T: PNSObject,
    {
        Iter {
            array: self,
            index: 0,
        }
    }
}

impl<T> NSArray<T> {
    /// Creates an empty array.
    pub fn new() -> Self {
        NSArray::m_new()
    }

    /// Returns true if the obect is an instance of NSArray.
    pub fn contains(&self, object: T) -> bool
    where
        T: PNSObject,
    {
        self.m_contains_object(object)
    }

    /// Returns the number of objects in the array.
    pub fn count(&self) -> u64 {
        self.p_count()
    }
}

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
    fn m_contains_object(&self, object: T) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), containsObject: object]) }
    }

    /// The number of objects in the array.
    fn p_count(&self) -> UInt {
        unsafe { msg_send![self.m_self(), count] }
    }

    /// The first object in the array.
    fn p_first_object(&self) -> Option<T>
    where
        T: PNSObject + FromId,
    {
        unsafe {
            let id: id = msg_send![self.m_self(), firstObject];
            if id.is_null() {
                None
            } else {
                Some(T::from_id(id))
            }
        }
    }

    /// The last object in the array.
    fn p_last_object(&self) -> Option<T>
    where
        T: PNSObject + FromId,
    {
        unsafe {
            let id: id = msg_send![self.m_self(), lastObject];
            if id.is_null() {
                None
            } else {
                Some(T::from_id(id))
            }
        }
    }

    /// The object at the specified index.
    fn m_object_at_index(&self, index: UInt) -> T
    where
        T: PNSObject + FromId,
    {
        unsafe { T::from_id(msg_send![self.m_self(), objectAtIndex: index]) }
    }

    /// The index of the specified object.
    fn m_object_at_indexed_subscript(&self, index: UInt) -> Option<id> {
        unsafe {
            let id: id = msg_send![self.m_self(), objectAtIndexedSubscript: index];
            if id.is_null() {
                None
            } else {
                Some(id)
            }
        }
    }

    /* Finding Objects in an Array
     */

    /// Returns the lowest index whose corresponding array value is equal to a given object.
    fn m_index_of_object(&self, object: T) -> UInt {
        unsafe { msg_send![self.m_self(), indexOfObject: object] }
    }

    /// Returns the lowest index within a specified range whose corresponding array value is equal to a given object .
    fn m_index_of_object_in_range(&self, object: T, range: NSRange) -> UInt {
        unsafe { msg_send![self.m_self(), indexOfObject: object inRange: range] }
    }

    /// Returns the lowest index whose corresponding array value is identical to a given object.
    fn m_index_of_object_identical_to(&self, object: T) -> UInt {
        unsafe { msg_send![self.m_self(), indexOfObjectIdenticalTo: object] }
    }

    /// Returns the lowest index within a specified range whose corresponding array value is equal to a given object .
    fn m_index_of_object_identical_to_in_range(&self, object: T, range: NSRange) -> UInt {
        unsafe { msg_send![self.m_self(), indexOfObjectIdenticalTo: object inRange: range] }
    }

    /* Comparing Arrays
     */

    /// Returns the first object contained in the receiving array that’s equal to an object in another given array.
    fn m_first_object_common_with_array(&self, other: &NSArray<T>) -> Option<T>
    where
        T: PNSObject + FromId,
    {
        unsafe {
            let id: id = msg_send![self.m_self(), firstObjectCommonWithArray: other.m_self()];
            if id.is_null() {
                None
            } else {
                Some(T::from_id(id))
            }
        }
    }

    /// Compares the receiving array to another array.
    fn m_is_equal_to_array(&self, other: &NSArray<T>) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isEqualToArray: other.m_self()]) }
    }

    /* Deriving New Arrays
     */

    /// Returns a new array that is a copy of the receiving array with a given object added to the end.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    unsafe fn m_array_by_adding_object(&self, object: T) -> NSArray<T> {
        NSArray::from_id(msg_send![self.m_self(), arrayByAddingObject: object])
    }

    /// Returns a new array that is a copy of the receiving array with the objects contained in another array added to the end.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    unsafe fn m_array_by_adding_objects_from_array<A>(&self, objects: A) -> NSArray<T>
    where
        A: INSArray<T>,
    {
        NSArray::from_id(msg_send![self.m_self(), arrayByAddingObjectsFromArray: objects.m_self()])
    }

    /// Returns a new array containing the receiving array’s elements that fall within the limits specified by a given range.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer
    unsafe fn m_subarray_with_range(&self, range: NSRange) -> NSArray<T> {
        NSArray::from_id(msg_send![self.m_self(), subarrayWithRange: range])
    }
    /* Creating a Description
     */

    /// A string that represents the contents of the array, formatted as a property list.

    /// Returns a string that represents the contents of the array, formatted as a property list.
    fn m_description_with_locale(&self, locale: &NSLocale) -> NSString {
        unsafe { msg_send![self.m_self(), descriptionWithLocale: locale.m_self()] }
    }

    /// Returns a string that represents the contents of the array, formatted as a property list.
    fn m_description_with_locale_indent(&self, locale: &NSLocale, indent: UInt) -> NSString {
        unsafe { msg_send![self.m_self(), descriptionWithLocale: locale.m_self() indent: indent] }
    }
}

impl<T> INSArray<T> for NSArray<T> {}

impl<T> Default for NSArray<T> {
    fn default() -> Self {
        Self::m_new()
    }
}

impl<'a, T> IntoIterator for &'a NSArray<T>
where
    T: PNSObject + FromId,
{
    type Item = T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl From<(*const c_char, usize)> for NSArray<UInt8> {
    fn from((s, i): (*const c_char, usize)) -> Self {
        let v = unsafe { slice::from_raw_parts(s as *const u8, i) };
        NSArray::from(v)
    }
}

impl<T> From<id> for NSArray<T> {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from(obj: id) -> Self {
        unsafe { NSArray::from_id(obj) }
    }
}

impl<T> From<&[T]> for NSArray<T>
where
    T: PNSObject,
{
    fn from(array: &[T]) -> Self {
        unsafe {
            let cls: *mut Object = msg_send![class!(NSArray),
                arrayWithObjects:array.as_ptr()
                count:array.len()
            ];
            NSArray::from(cls)
        }
    }
}

impl<T> From<Vec<T>> for NSArray<T>
where
    T: PNSObject,
{
    /// Given a set of `Object`s, creates an `Array` that holds them.
    fn from(objects: Vec<T>) -> Self {
        unsafe {
            let cls: id = msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ];
            NSArray::from(cls)
        }
    }
}

impl From<Vec<i8>> for NSArray<NSNumber> {
    /// Given a set of `Object`s, creates an `Array` that holds them.
    fn from(objects: Vec<i8>) -> Self {
        let objects: Vec<NSNumber> = objects.iter().map(|i| NSNumber::from(*i)).collect();
        unsafe {
            let cls: id = msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ];
            NSArray::from(cls)
        }
    }
}

impl From<Vec<i16>> for NSArray<NSNumber> {
    /// Given a set of `Object`s, creates an `Array` that holds them.
    fn from(objects: Vec<i16>) -> Self {
        let objects: Vec<NSNumber> = objects.iter().map(|i| NSNumber::from(*i)).collect();
        unsafe {
            let cls: id = msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ];
            NSArray::from(cls)
        }
    }
}

impl From<Vec<i32>> for NSArray<NSNumber> {
    /// Given a set of `Object`s, creates an `Array` that holds them.
    fn from(objects: Vec<i32>) -> Self {
        let objects: Vec<NSNumber> = objects.iter().map(|i| NSNumber::from(*i)).collect();
        unsafe {
            let cls: id = msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ];
            NSArray::from(cls)
        }
    }
}

impl From<Vec<i64>> for NSArray<NSNumber> {
    /// Given a set of `Object`s, creates an `Array` that holds them.
    fn from(objects: Vec<i64>) -> Self {
        let objects: Vec<NSNumber> = objects.iter().map(|i| NSNumber::from(*i)).collect();
        unsafe {
            let cls: id = msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ];
            NSArray::from(cls)
        }
    }
}

impl From<Vec<u8>> for NSArray<NSNumber> {
    /// Given a set of `Object`s, creates an `Array` that holds them.
    fn from(objects: Vec<u8>) -> Self {
        let objects: Vec<NSNumber> = objects.iter().map(|i| NSNumber::from(*i)).collect();
        unsafe {
            let cls: id = msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ];
            NSArray::from(cls)
        }
    }
}

impl From<Vec<u16>> for NSArray<NSNumber> {
    /// Given a set of `Object`s, creates an `Array` that holds them.
    fn from(objects: Vec<u16>) -> Self {
        let objects: Vec<NSNumber> = objects.iter().map(|i| NSNumber::from(*i)).collect();
        unsafe {
            let cls: id = msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ];
            NSArray::from(cls)
        }
    }
}

impl From<Vec<u32>> for NSArray<NSNumber> {
    /// Given a set of `Object`s, creates an `Array` that holds them.
    fn from(objects: Vec<u32>) -> Self {
        let objects: Vec<NSNumber> = objects.iter().map(|i| NSNumber::from(*i)).collect();
        unsafe {
            let cls: id = msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ];
            NSArray::from(cls)
        }
    }
}

impl From<Vec<u64>> for NSArray<NSNumber> {
    /// Given a set of `Object`s, creates an `Array` that holds them.
    fn from(objects: Vec<u64>) -> Self {
        let objects: Vec<NSNumber> = objects.iter().map(|i| NSNumber::from(*i)).collect();
        unsafe {
            let cls: id = msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ];
            NSArray::from(cls)
        }
    }
}

impl From<&[u8]> for NSArray<u8> {
    /// Given a set of `Object`s, creates an `Array` that holds them.
    fn from(objects: &[u8]) -> Self {
        unsafe {
            let cls: id = msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ];
            NSArray::from(cls)
        }
    }
}

impl From<Vec<&str>> for NSArray<NSString> {
    fn from(objects: Vec<&str>) -> Self {
        let objects: Vec<NSString> = objects.iter().map(|s| NSString::from(*s)).collect();
        unsafe {
            let cls: id = msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ];
            NSArray::from(cls)
        }
    }
}

impl From<Vec<String>> for NSArray<NSString> {
    fn from(objects: Vec<String>) -> Self {
        let objects: Vec<NSString> = objects.iter().map(|s| NSString::from(s.clone())).collect();
        unsafe {
            let cls: id = msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ];
            NSArray::from(cls)
        }
    }
}

impl<T> From<NSMutableArray<T>> for NSArray<T>
where
    T: PNSObject,
{
    /// Given an `Array` of `Object`s, creates a new `Array` that holds them.
    fn from(array: NSMutableArray<T>) -> Self {
        let cls: id = unsafe {
            {
                msg_send![class!(NSArray), arrayWithArray: array]
            }
        };
        NSArray::from(cls)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_from_vec() {
        let array: NSArray<NSString> = vec!["foo", "bar"].into();
        assert_eq!(array.count(), 2);
        assert_eq!(array.m_object_at_index(0), NSString::from("foo"));
        assert_eq!(array.m_object_at_index(1), NSString::from("bar"));
    }

    #[test]
    fn test_first_common_object() {
        let array: NSArray<NSString> = vec!["foo", "bar"].into();
        let array2: NSArray<NSString> = vec!["foo", "bar"].into();
        assert_eq!(
            array.m_first_object_common_with_array(&array2),
            Some(NSString::from("foo"))
        );
    }
}
