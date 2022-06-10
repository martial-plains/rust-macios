use std::{
    fmt::{self, Formatter},
    marker::PhantomData,
    ops::{Deref, DerefMut, Range},
};

use objc::{
    class, msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::{NSArray, NSString, UInt},
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject, ToId},
    },
    utils::to_bool,
};

use super::traits::{INSArray, INSMutableArray};

/// A dynamic ordered collection of objects.
pub struct NSMutableArray<T> {
    /// The underlying Objective-C object.
    pub obj: Id<Object>,
    _marker: PhantomData<T>,
}

impl<T> PNSObject for NSMutableArray<T> {
    fn im_class<'a>() -> &'a Class {
        class!(NSMutableArray)
    }

    fn im_is_equal(&self, object: &Self) -> bool {
        unsafe { to_bool(msg_send![self.obj, isEqual: object]) }
    }

    fn ip_hash(&self) -> UInt {
        unsafe { msg_send![self.obj, hash] }
    }

    fn im_is_kind_of_class(&self, class: Class) -> bool {
        unsafe { to_bool(msg_send![self.obj, isKindOfClass: class]) }
    }

    fn im_is_member_of_class(&self, class: Class) -> bool {
        unsafe { to_bool(msg_send![self.obj, isMemberOfClass: class]) }
    }

    fn im_responds_to_selector(&self, selector: Sel) -> bool {
        unsafe { to_bool(msg_send![self.obj, respondsToSelector: selector]) }
    }

    fn im_conforms_to_protocol(&self, protocol: Protocol) -> bool {
        unsafe { to_bool(msg_send![self.obj, conformsToProtocol: protocol]) }
    }

    fn ip_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.obj, description]) }
    }

    fn ip_debug_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.obj, debugDescription]) }
    }

    fn im_perform_selector(&self, selector: Sel) -> id {
        unsafe { msg_send![self.obj, performSelector: selector] }
    }

    fn im_perform_selector_with_object(&self, selector: Sel, with_object: id) -> id {
        unsafe { msg_send![self.obj, performSelector: selector withObject: with_object] }
    }

    fn im_is_proxy(&self) -> bool {
        unsafe { to_bool(msg_send![self.obj, isProxy]) }
    }
}

impl<T> INSArray<T> for NSMutableArray<T>
where
    T: PNSObject + FromId,
{
    fn im_contains_object(&self, object: T) -> bool {
        unsafe { msg_send![self.obj, containsObject: object] }
    }

    fn ip_count(&self) -> UInt {
        unsafe { msg_send![self.obj, count] }
    }

    fn ip_first_object(&self) -> Option<T> {
        unsafe {
            let obj: id = msg_send![self.obj, firstObject];
            if obj.is_null() {
                None
            } else {
                Some(T::from_id(obj))
            }
        }
    }

    fn ip_last_object(&self) -> Option<T> {
        unsafe {
            let obj: id = msg_send![self.obj, lastObject];
            if obj.is_null() {
                None
            } else {
                Some(T::from_id(obj))
            }
        }
    }

    fn im_object_at_index(&self, index: UInt) -> T {
        unsafe { T::from_id(msg_send![self.obj, objectAtIndex: index]) }
    }

    fn im_object_at_indexed_subscript(&self, index: UInt) -> Option<id> {
        unsafe {
            let obj: id = msg_send![self.obj, objectAtIndexedSubscript: index];
            if obj.is_null() {
                None
            } else {
                Some(obj)
            }
        }
    }

    fn im_index_of_object(&self, object: T) -> UInt {
        unsafe { msg_send![self.obj, indexOfObject: object] }
    }

    fn im_index_of_object_in_range(&self, object: T, range: Range<UInt>) -> UInt {
        unsafe { msg_send![self.obj, indexOfObject: object inRange: range] }
    }

    fn im_index_of_object_identical_to(&self, object: T) -> UInt {
        unsafe { msg_send![self.obj, indexOfObjectIdenticalTo: object] }
    }

    fn im_index_of_object_identical_to_in_range(&self, object: T, range: Range<UInt>) -> UInt {
        unsafe { msg_send![self.obj, indexOfObjectIdenticalTo: object inRange: range] }
    }

    fn im_first_object_common_with_array(&self, other: &NSArray<T>) -> Option<T> {
        unsafe {
            let obj: id = msg_send![self.obj, firstObjectCommonWithArray: other];
            if obj.is_null() {
                None
            } else {
                Some(T::from_id(obj))
            }
        }
    }

    fn im_is_equal_to_array(&self, other: &NSArray<T>) -> bool {
        unsafe { msg_send![self.obj, isEqualToArray: other] }
    }

    unsafe fn im_array_by_adding_object(&self, object: T) -> NSArray<T> {
        NSArray::from_id(msg_send![self.obj, addingObject: object])
    }

    unsafe fn im_array_by_adding_objects_from_array<A>(&self, objects: A) -> NSArray<T>
    where
        A: INSArray<T>,
    {
        NSArray::from_id(msg_send![self.obj, arrayByAddingObjectsFromArray: objects])
    }

    unsafe fn im_subarray_with_range(&self, range: Range<UInt>) -> NSArray<T> {
        NSArray::from_id(msg_send![self.obj, subarrayWithRange: range])
    }

    fn im_description_with_locale(&self, locale: &super::NSLocale) -> NSString {
        unsafe { msg_send![self.obj, descriptionWithLocale: locale] }
    }

    fn im_description_with_locale_indent(
        &self,
        locale: &super::NSLocale,
        indent: UInt,
    ) -> NSString {
        unsafe { msg_send![self.obj, descriptionWithLocale: locale indent: indent] }
    }
}

impl<T> INSMutableArray<T> for NSMutableArray<T>
where
    T: PNSObject + FromId,
{
    /* Creating and Initializing a Mutable Array
     */

    /// Creates and returns an NSMutableArray object with enough allocated memory to initially hold a given number of objects.
    fn tm_array_with_capacity(capacity: usize) -> Self {
        NSMutableArray {
            obj: unsafe { msg_send![class!(NSMutableArray), arrayWithCapacity: capacity] },
            _marker: PhantomData,
        }
    }

    /// Creates and returns a mutable array containing the contents of the file specified by the given path.
    fn tm_array_with_contents_of_file<S>(path: S) -> Self
    where
        S: Into<NSString>,
    {
        NSMutableArray {
            obj: unsafe { msg_send![class!(NSMutableArray), arrayWithContentsOfFile: path.into()] },
            _marker: PhantomData,
        }
    }

    /* Adding Objects
     */

    fn tm_array_with_contents_of_url<S>(url: S) -> Self
    where
        S: Into<NSString>,
    {
        NSMutableArray {
            obj: unsafe { msg_send![class!(NSMutableArray), arrayWithContentsOfURL: url.into()] },
            _marker: PhantomData,
        }
    }

    /// Returns an array, initialized with enough memory to initially hold a given number of objects.
    fn im_init_with_capacity(capacity: UInt) -> Self {
        NSMutableArray {
            obj: unsafe { msg_send![class!(NSMutableArray), arrayWithCapacity: capacity] },
            _marker: PhantomData,
        }
    }

    /* Removing Objects
     */

    fn im_init_with_contents_of_file<S>(&mut self, path: S) -> bool
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.obj, initWithContentsOfFile: path.into()] }
    }

    /// Inserts a given object at the end of the array.
    fn im_add_object(&mut self, object: &T) {
        unsafe { msg_send![&mut *self.obj, addObject: object] }
    }

    /// Adds the objects contained in another given array to the end of the receiving array’s content.
    fn im_add_objects_from_array(&mut self, other_array: &NSArray<T>) {
        unsafe { msg_send![&mut *self.obj, addObjectsFromArray: other_array] }
    }

    /// Inserts a given object into the array’s contents at a given index.
    fn im_insert_object_at_index(&mut self, index: UInt, object: &T) {
        unsafe { msg_send![&mut *self.obj, insertObject: object atIndex: index] }
    }

    /// Empties the array of all its elements.
    fn im_remove_all_objects(&mut self) {
        unsafe { msg_send![&mut *self.obj, removeAllObjects] }
    }

    /// Removes the object with the highest-valued index in the array
    fn im_remove_last_object(&mut self) {
        unsafe { msg_send![&mut *self.obj, removeLastObject] }
    }

    /* Replacing Objects
     */

    /// Removes all occurrences in the array of a given object.
    fn im_remove_object(&mut self, object: &T) {
        unsafe { msg_send![&mut *self.obj, removeObject: object] }
    }

    /// Removes all occurrences within a specified range in the array of a given object.
    fn im_remove_object_in_range(&mut self, object: &T, range: Range<UInt>) {
        unsafe { msg_send![&mut *self.obj, removeObject: object inRange: range] }
    }

    /// Removes the object at index .
    fn im_remove_object_at_index(&mut self, index: UInt) {
        unsafe { msg_send![&mut *self.obj, removeObjectAtIndex: index] }
    }

    /// Removes all occurrences of a given object in the array.
    fn im_remove_object_identical_to(&mut self, object: &T) {
        unsafe { msg_send![&mut *self.obj, removeObjectIdenticalTo: object] }
    }

    /// Removes all occurrences of anObject within the specified range in the array.
    fn im_remove_object_identical_to_in_range(&mut self, object: &T, range: Range<UInt>) {
        unsafe { msg_send![&mut *self.obj, removeObjectIdenticalTo: object inRange: range] }
    }

    /// Removes from the receiving array the objects in another given array.
    fn im_remove_objects_in_array(&mut self, other_array: &NSArray<T>) {
        unsafe { msg_send![&mut *self.obj, removeObjectsInArray: other_array] }
    }

    /// Removes from the array each of the objects within a given range.
    fn im_remove_objects_in_range(&mut self, range: Range<UInt>) {
        unsafe { msg_send![&mut *self.obj, removeObjectsInRange: range] }
    }

    /// Replaces the object at index with anObject.
    fn im_replace_object_at_index_with_object(&mut self, index: UInt, object: &T) {
        unsafe { msg_send![&mut *self.obj, replaceObjectAtIndex: index withObject: object] }
    }

    /// Sets the receiving array’s elements to those in another given array.
    fn im_set_array(&mut self, other_array: &NSArray<T>) {
        unsafe { msg_send![&mut *self.obj, setArray: other_array] }
    }
}

impl<T> Default for NSMutableArray<T>
where
    T: PNSObject,
{
    fn default() -> Self {
        unsafe { Self::from_id(msg_send![class!(NSMutableArray), init]) }
    }
}

impl fmt::Debug for NSMutableArray<id> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSMutableArray<id> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl<T> Deref for NSMutableArray<T> {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.obj
    }
}

impl<T> DerefMut for NSMutableArray<T> {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.obj
    }
}

impl<T> Clone for NSMutableArray<T>
where
    T: PNSObject,
{
    fn clone(&self) -> Self {
        unsafe { Self::from_id(msg_send![self.obj, retain]) }
    }
}

impl<T> ToId for NSMutableArray<T> {
    /// Converts the NSMutableArray to an id.
    fn to_id(mut self) -> id {
        &mut *self.obj
    }
}

impl<T> FromId for NSMutableArray<T>
where
    T: PNSObject,
{
    unsafe fn from_id(id: id) -> Self {
        NSMutableArray {
            obj: Id::from_ptr(id),
            _marker: PhantomData,
        }
    }
}

impl<T> From<id> for NSMutableArray<T> {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from(obj: id) -> Self {
        unsafe {
            Self {
                obj: Id::from_ptr(obj),
                _marker: PhantomData,
            }
        }
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
