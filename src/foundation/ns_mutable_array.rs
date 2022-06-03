use std::{
    fmt::{self, Formatter},
    marker::PhantomData,
    ops::{Deref, DerefMut, Range},
};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    foundation::{NSArray, NSString, UInt},
    id,
    objective_c_runtime::traits::{FromId, PNSObject, ToId},
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
    fn im_class<'a>() -> &'a objc::runtime::Class {
        class!(NSMutableArray)
    }

    fn im_isEqual(&self, object: &Self) -> bool {
        unsafe { to_bool(msg_send![self.obj, isEqual: object]) }
    }

    fn ip_hash(&self) -> UInt {
        unsafe { msg_send![self.obj, hash] }
    }

    fn im_isKindOfClass(&self, aClass: objc::runtime::Class) -> bool {
        unsafe { to_bool(msg_send![self.obj, isKindOfClass: aClass]) }
    }

    fn im_isMemberOfClass(&self, aClass: objc::runtime::Class) -> bool {
        unsafe { to_bool(msg_send![self.obj, isMemberOfClass: aClass]) }
    }

    fn im_respondsToSelector(&self, aSelector: objc::runtime::Sel) -> bool {
        unsafe { to_bool(msg_send![self.obj, respondsToSelector: aSelector]) }
    }

    fn im_conformsToProtocol(&self, aProtocol: objc::runtime::Protocol) -> bool {
        unsafe { to_bool(msg_send![self.obj, conformsToProtocol: aProtocol]) }
    }

    fn ip_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.obj, description]) }
    }

    fn ip_debugDescription(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.obj, debugDescription]) }
    }

    fn im_performSelector(&self, aSelector: objc::runtime::Sel) -> id {
        unsafe { msg_send![self.obj, performSelector: aSelector] }
    }

    fn im_performSelector_withObject(&self, aSelector: objc::runtime::Sel, withObject: id) -> id {
        unsafe { msg_send![self.obj, performSelector: aSelector withObject: withObject] }
    }

    fn im_isProxy(&self) -> bool {
        unsafe { to_bool(msg_send![self.obj, isProxy]) }
    }
}

impl<T> INSArray<T> for NSMutableArray<T>
where
    T: PNSObject + FromId,
{
    fn im_containsObject(&self, object: T) -> bool {
        unsafe { msg_send![self.obj, containsObject: object] }
    }

    fn ip_count(&self) -> UInt {
        unsafe { msg_send![self.obj, count] }
    }

    fn ip_firstObject(&self) -> Option<T> {
        unsafe {
            let obj: id = msg_send![self.obj, firstObject];
            if obj.is_null() {
                None
            } else {
                Some(T::from_id(obj))
            }
        }
    }

    fn ip_lastObject(&self) -> Option<T> {
        unsafe {
            let obj: id = msg_send![self.obj, lastObject];
            if obj.is_null() {
                None
            } else {
                Some(T::from_id(obj))
            }
        }
    }

    fn im_objectAtIndex(&self, index: UInt) -> T {
        unsafe { T::from_id(msg_send![self.obj, objectAtIndex: index]) }
    }

    fn im_objectAtIndexedSubscript(&self, index: UInt) -> Option<id> {
        unsafe {
            let obj: id = msg_send![self.obj, objectAtIndexedSubscript: index];
            if obj.is_null() {
                None
            } else {
                Some(obj)
            }
        }
    }

    fn im_indexOfObject(&self, object: T) -> UInt {
        unsafe { msg_send![self.obj, indexOfObject: object] }
    }

    fn im_indexOfObject_inRange(&self, object: T, range: Range<UInt>) -> UInt {
        unsafe { msg_send![self.obj, indexOfObject: object inRange: range] }
    }

    fn im_indexOfObjectIdenticalTo(&self, object: T) -> UInt {
        unsafe { msg_send![self.obj, indexOfObjectIdenticalTo: object] }
    }

    fn im_indexOfObjectIdenticalTo_inRange(&self, object: T, range: Range<UInt>) -> UInt {
        unsafe { msg_send![self.obj, indexOfObjectIdenticalTo: object inRange: range] }
    }

    fn im_firstObjectCommonWithArray(&self, other: &NSArray<T>) -> Option<T> {
        unsafe {
            let obj: id = msg_send![self.obj, firstObjectCommonWithArray: other];
            if obj.is_null() {
                None
            } else {
                Some(T::from_id(obj))
            }
        }
    }

    fn im_isEqualToArray(&self, other: &NSArray<T>) -> bool {
        unsafe { msg_send![self.obj, isEqualToArray: other] }
    }

    unsafe fn im_arrayByAddingObject(&self, object: T) -> NSArray<T> {
        NSArray::from_id(msg_send![self.obj, addingObject: object])
    }

    unsafe fn im_arrayByAddingObjectsFromArray<A>(&self, objects: A) -> NSArray<T>
    where
        A: INSArray<T>,
    {
        NSArray::from_id(msg_send![self.obj, arrayByAddingObjectsFromArray: objects])
    }

    unsafe fn im_subarrayWithRange(&self, range: Range<UInt>) -> NSArray<T> {
        NSArray::from_id(msg_send![self.obj, subarrayWithRange: range])
    }

    fn im_descriptionWithLocale(&self, locale: &super::NSLocale) -> NSString {
        unsafe { msg_send![self.obj, descriptionWithLocale: locale] }
    }

    fn im_descriptionWithLocaleIndent(&self, locale: &super::NSLocale, indent: UInt) -> NSString {
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
    fn tm_arrayWithCapacity(capacity: usize) -> Self {
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

    fn tm_arrayWithContentsOfUrl<S>(url: S) -> Self
    where
        S: Into<NSString>,
    {
        NSMutableArray {
            obj: unsafe { msg_send![class!(NSMutableArray), arrayWithContentsOfURL: url.into()] },
            _marker: PhantomData,
        }
    }

    /// Returns an array, initialized with enough memory to initially hold a given number of objects.
    fn im_initWithCapacity(capacity: UInt) -> Self {
        NSMutableArray {
            obj: unsafe { msg_send![class!(NSMutableArray), arrayWithCapacity: capacity] },
            _marker: PhantomData,
        }
    }

    /* Removing Objects
     */

    fn im_initWithContentsOfFile<S>(&mut self, path: S) -> bool
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.obj, initWithContentsOfFile: path.into()] }
    }

    /// Inserts a given object at the end of the array.
    fn im_addObject(&mut self, object: &T) {
        unsafe { msg_send![&mut *self.obj, addObject: object] }
    }

    /// Adds the objects contained in another given array to the end of the receiving array’s content.
    fn im_addObjectsFromArray(&mut self, other_array: &NSArray<T>) {
        unsafe { msg_send![&mut *self.obj, addObjectsFromArray: other_array] }
    }

    /// Inserts a given object into the array’s contents at a given index.
    fn im_insertObject_atIndex(&mut self, index: UInt, object: &T) {
        unsafe { msg_send![&mut *self.obj, insertObject: object atIndex: index] }
    }

    /// Empties the array of all its elements.
    fn im_removeAllObjects(&mut self) {
        unsafe { msg_send![&mut *self.obj, removeAllObjects] }
    }

    /// Removes the object with the highest-valued index in the array
    fn im_removeLastObject(&mut self) {
        unsafe { msg_send![&mut *self.obj, removeLastObject] }
    }

    /* Replacing Objects
     */

    /// Removes all occurrences in the array of a given object.
    fn im_removeObject(&mut self, object: &T) {
        unsafe { msg_send![&mut *self.obj, removeObject: object] }
    }

    /// Removes all occurrences within a specified range in the array of a given object.
    fn im_removeObject_inRange(&mut self, object: &T, range: Range<UInt>) {
        unsafe { msg_send![&mut *self.obj, removeObject: object inRange: range] }
    }

    /// Removes the object at index .
    fn im_removeObjectAtIndex(&mut self, index: UInt) {
        unsafe { msg_send![&mut *self.obj, removeObjectAtIndex: index] }
    }

    /// Removes all occurrences of a given object in the array.
    fn im_removeObjectIdenticalTo(&mut self, object: &T) {
        unsafe { msg_send![&mut *self.obj, removeObjectIdenticalTo: object] }
    }

    /// Removes all occurrences of anObject within the specified range in the array.
    fn im_removeObjectIdenticalTo_inRange(&mut self, object: &T, range: Range<UInt>) {
        unsafe { msg_send![&mut *self.obj, removeObjectIdenticalTo: object inRange: range] }
    }

    /// Removes from the receiving array the objects in another given array.
    fn im_removeObjectsInArray(&mut self, other_array: &NSArray<T>) {
        unsafe { msg_send![&mut *self.obj, removeObjectsInArray: other_array] }
    }

    /// Removes from the array each of the objects within a given range.
    fn im_removeObjectsInRange(&mut self, range: Range<UInt>) {
        unsafe { msg_send![&mut *self.obj, removeObjectsInRange: range] }
    }

    /// Replaces the object at index with anObject.
    fn im_replaceObjectAtIndex_withObject(&mut self, index: UInt, object: &T) {
        unsafe { msg_send![&mut *self.obj, replaceObjectAtIndex: index withObject: object] }
    }

    /// Sets the receiving array’s elements to those in another given array.
    fn im_setArray(&mut self, other_array: &NSArray<T>) {
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
        write!(f, "{}", self.ip_debugDescription())
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
