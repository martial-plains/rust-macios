use std::{
    fmt::{self, Formatter},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    id,
    objective_c_runtime::traits::{FromId, PNSObject, ToId},
};

use super::{traits::INSDictionary, NSMutableDictionary, NSString, UInt};

/// A static collection of objects associated with unique keys.
pub struct NSDictionary<K, V> {
    /// The raw pointer to the Objective-C object.
    pub obj: Id<Object>,
    _key: PhantomData<K>,
    _value: PhantomData<V>,
}

impl<K, V> PNSObject for NSDictionary<K, V> {
    fn im_class<'a>() -> &'a objc::runtime::Class {
        class!(NSDictionary)
    }

    fn im_isEqual(&self, object: &Self) -> bool {
        unsafe { msg_send![self.obj, isEqual: object] }
    }

    fn ip_hash(&self) -> UInt {
        unsafe { msg_send![self.obj, hash] }
    }

    fn im_isKindOfClass(&self, aClass: objc::runtime::Class) -> bool {
        unsafe { msg_send![self.obj, isKindOfClass: aClass] }
    }

    fn im_isMemberOfClass(&self, aClass: objc::runtime::Class) -> bool {
        unsafe { msg_send![self.obj, isMemberOfClass: aClass] }
    }

    fn im_respondsToSelector(&self, aSelector: objc::runtime::Sel) -> bool {
        unsafe { msg_send![self.obj, respondsToSelector: aSelector] }
    }

    fn im_conformsToProtocol(&self, aProtocol: objc::runtime::Protocol) -> bool {
        unsafe { msg_send![self.obj, conformsToProtocol: aProtocol] }
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
        unsafe { msg_send![self.obj, isProxy] }
    }
}

impl<K, V> INSDictionary<K, V> for NSDictionary<K, V> {
    fn ip_count(&self) -> UInt {
        unsafe { msg_send![self.obj, count] }
    }

    fn initWithDictionary(&mut self, dictionary: NSDictionary<K, V>) {
        unsafe { msg_send![self.obj, initWithDictionary: dictionary] }
    }
}

impl<K, V> fmt::Debug for NSDictionary<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.ip_debugDescription())
    }
}

impl<K, V> fmt::Display for NSDictionary<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.ip_description())
    }
}

impl<K, V> ToId for NSDictionary<K, V> {
    fn to_id(mut self) -> id {
        &mut *self.obj
    }
}

impl<K, V> FromId for NSDictionary<K, V> {
    unsafe fn from_id(id: id) -> Self {
        NSDictionary {
            obj: Id::from_ptr(id),
            _key: PhantomData,
            _value: PhantomData,
        }
    }
}

impl<K, V> Clone for NSDictionary<K, V> {
    fn clone(&self) -> Self {
        unsafe {
            let obj = msg_send![self.obj, retain];
            Self::from_id(obj)
        }
    }
}

impl<K, V> Default for NSDictionary<K, V> {
    fn default() -> Self {
        unsafe { Self::from_id(msg_send![class!(NSDictionary), dictionary]) }
    }
}

impl<K, V> AsMut<NSDictionary<K, V>> for NSDictionary<K, V> {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<K, V> Deref for NSDictionary<K, V> {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.obj
    }
}

impl<K, V> DerefMut for NSDictionary<K, V> {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.obj
    }
}

impl<K, V> From<id> for NSDictionary<K, V> {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from(obj: id) -> Self {
        Self {
            obj: unsafe { Id::from_ptr(obj) },
            _key: PhantomData,
            _value: PhantomData,
        }
    }
}

impl<K, V> From<NSMutableDictionary<K, V>> for NSDictionary<K, V>
where
    K: PNSObject,
    V: PNSObject,
{
    fn from(dict: NSMutableDictionary<K, V>) -> Self {
        let cls: id = unsafe { msg_send![class!(NSDictionary), dictionaryWithDictionary: dict] };
        NSDictionary::from(cls)
    }
}
