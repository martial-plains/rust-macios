use std::marker::PhantomData;

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
