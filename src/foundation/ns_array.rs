use std::{
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut, Range},
    slice,
};

use libc::c_char;
use objc::{
    class, msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::{traits::INSArray, NSLocale, NSString, UInt},
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject, ToId},
    },
    utils::to_bool,
};

use self::iter::Iter;

use super::{ns_mutable_array::NSMutableArray, NSNumber, UInt8};

/// Iterator for Array
pub mod iter;

/// A static ordered collection of objects.
pub struct NSArray<T> {
    /// The underlying Objective-C object.
    pub obj: Id<Object>,
    _marker: PhantomData<T>,
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
    /// Returns true if the obect is an instance of NSArray.
    pub fn contains(&self, object: T) -> bool
    where
        T: PNSObject,
    {
        self.im_contains_object(object)
    }

    /// Returns the number of objects in the array.
    pub fn count(&self) -> u64 {
        self.ip_count()
    }
}

impl<T> PNSObject for NSArray<T> {
    fn im_class<'a>() -> &'a Class {
        class!(NSArray)
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

impl<T> INSArray<T> for NSArray<T> {
    fn im_contains_object(&self, object: T) -> bool {
        unsafe { to_bool(msg_send![&*self.obj, containsObject: object]) }
    }

    fn ip_count(&self) -> UInt {
        unsafe { msg_send![self.obj, count] }
    }

    fn ip_first_object(&self) -> Option<T>
    where
        T: PNSObject + FromId,
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

    fn ip_last_object(&self) -> Option<T>
    where
        T: PNSObject + FromId,
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

    fn im_object_at_index(&self, index: UInt) -> T
    where
        T: PNSObject + FromId,
    {
        unsafe {
            let ptr: *mut Object = msg_send![&*self.obj, objectAtIndex: index];
            T::from_id(ptr)
        }
    }

    fn im_object_at_indexed_subscript(&self, index: UInt) -> Option<id> {
        unsafe {
            let ptr: *mut Object = msg_send![&*self.obj, objectAtIndexedSubscript: index];
            ptr.into()
        }
    }

    fn im_index_of_object(&self, object: T) -> UInt {
        unsafe { msg_send![&*self.obj, indexOfObject: object] }
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

    fn im_first_object_common_with_array(&self, other: &NSArray<T>) -> Option<T>
    where
        T: PNSObject + FromId,
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

    fn im_is_equal_to_array(&self, other: &NSArray<T>) -> bool {
        unsafe { to_bool(msg_send![&*self.obj, isEqualToArray: other.clone().obj]) }
    }

    unsafe fn im_array_by_adding_object(&self, object: T) -> NSArray<T> {
        let cls: id = msg_send![&*self.obj, arrayByAddingObject: object];
        NSArray::from(cls)
    }

    unsafe fn im_array_by_adding_objects_from_array<A>(&self, objects: A) -> NSArray<T>
    where
        A: INSArray<T>,
    {
        let cls: id = msg_send![&*self.obj, arrayByAddingObjectsFromArray: objects];
        NSArray::from(cls)
    }

    unsafe fn im_subarray_with_range(&self, range: Range<UInt>) -> NSArray<T> {
        let cls: id = msg_send![&*self.obj, subarrayWithRange: range];
        NSArray::from(cls)
    }

    fn im_description_with_locale(&self, locale: &NSLocale) -> NSString {
        unsafe { msg_send![&*self.obj, descriptionWithLocale: locale.clone().obj] }
    }

    fn im_description_with_locale_indent(&self, locale: &NSLocale, indent: UInt) -> NSString {
        unsafe { msg_send![&*self.obj, descriptionWithLocale: locale.clone().obj indent: indent] }
    }
}

impl<T> fmt::Debug for NSArray<T>
where
    T: fmt::Debug + PNSObject,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl<T> fmt::Display for NSArray<T>
where
    T: fmt::Display + PNSObject,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl<T> Clone for NSArray<T> {
    fn clone(&self) -> Self {
        let cls: id = unsafe { msg_send![self.obj, retain] };
        NSArray::from(cls)
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

impl<T> FromId for NSArray<T> {
    unsafe fn from_id(obj: id) -> Self {
        Self {
            obj: Id::from_ptr(obj),
            _marker: PhantomData,
        }
    }
}

impl<T> ToId for NSArray<T> {
    fn to_id(mut self) -> id {
        &mut *self.obj
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

impl<T> Deref for NSArray<T> {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.obj
    }
}

impl<T> DerefMut for NSArray<T> {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.obj
    }
}
