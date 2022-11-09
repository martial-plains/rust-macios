use std::marker::PhantomData;

use objc::{msg_send, sel, sel_impl};

use crate::{
    object,
    objective_c_runtime::traits::{FromId, PNSObject},
};

use super::NSArray;

object! {
    /// An abstract class whose subclasses enumerate collections of objects, such as arrays and dictionaries.
    unsafe pub struct NSEnumerator<ObjectType> {
        marker: PhantomData<ObjectType>,
    }
}

/// An abstract class whose subclasses enumerate collections of objects, such as arrays and dictionaries.
pub trait INSEnumerator<ObjectType>: PNSObject
where
    ObjectType: PNSObject + FromId,
{
    /// Returns the next object from the collection being enumerated.
    fn m_next_object(&self) -> ObjectType {
        unsafe { ObjectType::from_id(msg_send![self.m_self(), nextObject]) }
    }

    /// The array of unenumerated objects.
    fn m_all_objects(&self) -> NSArray<ObjectType> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), allObjects]) }
    }
}

impl<ObjectType> INSEnumerator<ObjectType> for NSEnumerator<ObjectType> where
    ObjectType: PNSObject + FromId
{
}
