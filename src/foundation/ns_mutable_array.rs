use std::{
    fmt::{self, Formatter},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use objc::{
    class, msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::UInt,
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject, ToId},
    },
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

    fn im_self(&self) -> id {
        unsafe { msg_send![self.obj, self] }
    }
}

impl<T> INSArray<T> for NSMutableArray<T> where T: PNSObject + FromId {}

impl<T> INSMutableArray<T> for NSMutableArray<T> where T: PNSObject + FromId {}

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
