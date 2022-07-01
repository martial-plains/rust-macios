use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};

use crate::{
    foundation::UInt,
    objective_c_runtime::{
        id,
        macros::object,
        traits::{FromId, PNSObject},
    },
};

use super::traits::{INSArray, INSMutableArray};

object! {
    /// A dynamic ordered collection of objects.
    unsafe pub struct NSMutableArray<T> {
        _marker: PhantomData<T>,
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

impl<T> Deref for NSMutableArray<T> {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        unsafe { &*self.im_self() }
    }
}

impl<T> DerefMut for NSMutableArray<T> {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        unsafe { &mut *self.im_self() }
    }
}

impl<T> Clone for NSMutableArray<T>
where
    T: PNSObject,
{
    fn clone(&self) -> Self {
        unsafe { Self::from_id(msg_send![self.im_self(), retain]) }
    }
}

impl<T> From<id> for NSMutableArray<T> {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from(obj: id) -> Self {
        unsafe { Self::from_id(obj) }
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
