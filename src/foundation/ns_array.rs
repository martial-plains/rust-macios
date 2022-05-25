use std::{
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut, Range},
};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    foundation::{traits::t_NSArray, Locale, String, UInt},
    id,
    objective_c_runtime::traits::t_NSObject,
    utils::to_bool,
};

use self::iter::Iter;

use super::{ns_mutable_array::MutableArray, NSNumber};

/// Iterator for Array
pub mod iter;

/// A static ordered collection of objects.
pub struct Array<T> {
    /// The underlying Objective-C object.
    pub obj: Id<Object>,
    _marker: PhantomData<T>,
}

impl<T> t_NSArray<T> for Array<T> {
    unsafe fn new(ptr: *mut Object) -> Self {
        Self {
            obj: Id::from_ptr(ptr),
            _marker: PhantomData,
        }
    }

    unsafe fn fromObjects(objects: &[T]) -> Self
    where
        T: t_NSObject,
    {
        Array::new(msg_send![class!(NSArray),
            arrayWithObjects:objects.as_ptr()
            count:objects.len()
        ])
    }

    unsafe fn fromRetained(array: id) -> Self {
        Array {
            obj: Id::from_retained_ptr(array),
            _marker: PhantomData,
        }
    }

    fn contains(&self, object: T) -> bool
    where
        T: t_NSObject,
    {
        unsafe { to_bool(msg_send![&*self.obj, containsObject: object]) }
    }

    fn count(&self) -> UInt {
        unsafe { msg_send![self.obj, count] }
    }

    fn firstObject(&self) -> Option<T>
    where
        T: t_NSObject,
    {
        unsafe {
            let ptr: *mut Object = msg_send![&*self.obj, firstObject];
            if ptr.is_null() {
                None
            } else {
                Some(T::fromId(ptr))
            }
        }
    }

    fn lastObject(&self) -> Option<T>
    where
        T: t_NSObject,
    {
        unsafe {
            let ptr: *mut Object = msg_send![&*self.obj, lastObject];
            if ptr.is_null() {
                None
            } else {
                Some(T::fromId(ptr))
            }
        }
    }

    fn objectAt(&self, index: UInt) -> T
    where
        T: t_NSObject,
    {
        unsafe {
            let ptr: *mut Object = msg_send![&*self.obj, objectAtIndex: index];
            T::fromId(ptr)
        }
    }

    fn objectAtIndexedSubscript(&self, index: UInt) -> Option<id> {
        unsafe {
            let ptr: *mut Object = msg_send![&*self.obj, objectAtIndexedSubscript: index];
            ptr.into()
        }
    }

    fn indexOf(&self, object: T) -> UInt
    where
        T: t_NSObject,
    {
        unsafe { msg_send![&*self.obj, indexOfObject: object] }
    }

    fn indexOfObjectInRange(&self, object: T, range: Range<UInt>) -> UInt
    where
        T: t_NSObject,
    {
        unsafe { msg_send![self.obj, indexOfObject: object inRange: range] }
    }

    fn indexOfObjectIdenticalTo(&self, object: T) -> UInt
    where
        T: t_NSObject,
    {
        unsafe { msg_send![self.obj, indexOfObjectIdenticalTo: object] }
    }

    fn indexOfObjectIdenticalToInRange(&self, object: T, range: Range<UInt>) -> UInt
    where
        T: t_NSObject,
    {
        unsafe { msg_send![self.obj, indexOfObjectIdenticalTo: object inRange: range] }
    }

    fn firstObjectCommonWith(&self, other: &Array<T>) -> Option<T>
    where
        T: t_NSObject,
    {
        unsafe {
            let ptr: *mut Object =
                msg_send![&*self.obj, firstObjectCommonWithArray: other.clone().obj];
            if ptr.is_null() {
                None
            } else {
                Some(T::fromId(ptr))
            }
        }
    }

    fn isEqualTo(&self, other: &Array<T>) -> bool
    where
        T: t_NSObject,
    {
        unsafe { to_bool(msg_send![&*self.obj, isEqualToArray: other.clone().obj]) }
    }

    unsafe fn adding(&self, object: T) -> Array<T>
    where
        T: t_NSObject,
    {
        Array::new(msg_send![&*self.obj, arrayByAddingObject: object])
    }

    unsafe fn addingObjects(&self, objects: &Array<T>) -> Array<T>
    where
        T: t_NSObject,
    {
        Array::new(msg_send![self.obj, arrayByAddingObjectsFromArray: objects.clone().obj])
    }

    unsafe fn subarrayWithRange(&self, range: Range<UInt>) -> Array<T>
    where
        T: t_NSObject,
    {
        Array::new(msg_send![&*self.obj, subarrayWithRange: range])
    }

    fn descriptionWithLocale(&self, locale: &Locale) -> String {
        unsafe { msg_send![&*self.obj, descriptionWithLocale: locale.clone().obj] }
    }

    fn descriptionWithLocaleIndent(&self, locale: &Locale, indent: UInt) -> String {
        unsafe { msg_send![&*self.obj, descriptionWithLocale: locale.clone().obj indent: indent] }
    }

    fn iter(&self) -> Iter<'_, T>
    where
        T: t_NSObject,
    {
        Iter {
            array: self,
            index: 0,
        }
    }
}

impl<T> t_NSObject for Array<T> {
    fn init() -> Self {
        let obj: id = unsafe { msg_send![class!(NSArray), init] };

        Self {
            obj: unsafe { Id::from_ptr(obj) },
            _marker: PhantomData,
        }
    }

    fn toId(mut self) -> id {
        &mut *self.obj
    }

    unsafe fn fromId(_obj: id) -> Self {
        todo!()
    }

    fn description(&self) -> String {
        let obj: id = unsafe { msg_send![&*self.obj, description] };
        unsafe { String::fromId(obj) }
    }

    fn debugDescription(&self) -> String {
        let obj: id = unsafe { msg_send![&*self.obj, debugDescription] };
        unsafe { String::fromId(obj) }
    }

    fn retain(&self) -> Self {
        let obj: id = unsafe { msg_send![&*self.obj, retain] };
        Self {
            obj: unsafe { Id::from_ptr(obj) },
            _marker: PhantomData,
        }
    }
}

impl<T> fmt::Debug for Array<T>
where
    T: fmt::Debug + t_NSObject,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.debugDescription())
    }
}

impl<T> fmt::Display for Array<T>
where
    T: fmt::Display + t_NSObject,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<T> Clone for Array<T> {
    fn clone(&self) -> Self {
        unsafe { Array::new(msg_send![self.obj, retain]) }
    }
}

impl<'a, T> IntoIterator for &'a Array<T>
where
    T: t_NSObject,
{
    type Item = T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> From<Vec<T>> for Array<T>
where
    T: t_NSObject,
{
    /// Given a set of `Object`s, creates an `Array` that holds them.
    fn from(objects: Vec<T>) -> Self {
        unsafe {
            Array::new(msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ])
        }
    }
}

impl From<Vec<i8>> for Array<NSNumber> {
    /// Given a set of `Object`s, creates an `Array` that holds them.
    fn from(objects: Vec<i8>) -> Self {
        let objects: Vec<NSNumber> = objects.iter().map(|i| NSNumber::from(*i)).collect();
        unsafe {
            Array::new(msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ])
        }
    }
}

impl From<Vec<i16>> for Array<NSNumber> {
    /// Given a set of `Object`s, creates an `Array` that holds them.
    fn from(objects: Vec<i16>) -> Self {
        let objects: Vec<NSNumber> = objects.iter().map(|i| NSNumber::from(*i)).collect();
        unsafe {
            Array::new(msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ])
        }
    }
}

impl From<Vec<i32>> for Array<NSNumber> {
    /// Given a set of `Object`s, creates an `Array` that holds them.
    fn from(objects: Vec<i32>) -> Self {
        let objects: Vec<NSNumber> = objects.iter().map(|i| NSNumber::from(*i)).collect();
        unsafe {
            Array::new(msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ])
        }
    }
}

impl From<Vec<i64>> for Array<NSNumber> {
    /// Given a set of `Object`s, creates an `Array` that holds them.
    fn from(objects: Vec<i64>) -> Self {
        let objects: Vec<NSNumber> = objects.iter().map(|i| NSNumber::from(*i)).collect();
        unsafe {
            Array::new(msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ])
        }
    }
}

impl From<Vec<u8>> for Array<NSNumber> {
    /// Given a set of `Object`s, creates an `Array` that holds them.
    fn from(objects: Vec<u8>) -> Self {
        let objects: Vec<NSNumber> = objects.iter().map(|i| NSNumber::from(*i)).collect();
        unsafe {
            Array::new(msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ])
        }
    }
}

impl From<Vec<u16>> for Array<NSNumber> {
    /// Given a set of `Object`s, creates an `Array` that holds them.
    fn from(objects: Vec<u16>) -> Self {
        let objects: Vec<NSNumber> = objects.iter().map(|i| NSNumber::from(*i)).collect();
        unsafe {
            Array::new(msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ])
        }
    }
}

impl From<Vec<u32>> for Array<NSNumber> {
    /// Given a set of `Object`s, creates an `Array` that holds them.
    fn from(objects: Vec<u32>) -> Self {
        let objects: Vec<NSNumber> = objects.iter().map(|i| NSNumber::from(*i)).collect();
        unsafe {
            Array::new(msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ])
        }
    }
}

impl From<Vec<u64>> for Array<NSNumber> {
    /// Given a set of `Object`s, creates an `Array` that holds them.
    fn from(objects: Vec<u64>) -> Self {
        let objects: Vec<NSNumber> = objects.iter().map(|i| NSNumber::from(*i)).collect();
        unsafe {
            Array::new(msg_send![class!(NSArray),
                arrayWithObjects:objects.as_ptr()
                count:objects.len()
            ])
        }
    }
}

impl<T> From<MutableArray<T>> for Array<T>
where
    T: t_NSObject,
{
    /// Given an `Array` of `Object`s, creates a new `Array` that holds them.
    fn from(array: MutableArray<T>) -> Self {
        let arr: MutableArray<T> = MutableArray::init();
        unsafe { Array::<T>::new(msg_send![arr.obj, arrayWithArray: array]) }
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
