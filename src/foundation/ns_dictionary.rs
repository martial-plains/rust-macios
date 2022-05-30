use std::{
    fmt::{self, Formatter},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    id,
    objective_c_runtime::traits::{FromId, PNSObject},
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
    fn class<'a>() -> &'a objc::runtime::Class {
        class!(NSDictionary)
    }

    fn superclass<'a>() -> &'a objc::runtime::Class {
        unsafe { msg_send![class!(NSDictionary), superclass] }
    }

    fn isEqual(&self, object: &Self) -> bool {
        unsafe { msg_send![self.obj, isEqual: object] }
    }

    fn hash(&self) -> UInt {
        unsafe { msg_send![self.obj, hash] }
    }

    fn isKindOfClass(&self, aClass: objc::runtime::Class) -> bool {
        unsafe { msg_send![self.obj, isKindOfClass: aClass] }
    }

    fn isMemberOfClass(&self, aClass: objc::runtime::Class) -> bool {
        unsafe { msg_send![self.obj, isMemberOfClass: aClass] }
    }

    fn respondsToSelector(&self, aSelector: objc::runtime::Sel) -> bool {
        unsafe { msg_send![self.obj, respondsToSelector: aSelector] }
    }

    fn conformsToProtocol(&self, aProtocol: objc::runtime::Protocol) -> bool {
        unsafe { msg_send![self.obj, conformsToProtocol: aProtocol] }
    }

    fn description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.obj, description]) }
    }

    fn debugDescription(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.obj, debugDescription]) }
    }

    fn performSelector(&self, aSelector: objc::runtime::Sel) -> id {
        unsafe { msg_send![self.obj, performSelector: aSelector] }
    }

    fn performSelector_withObject(&self, aSelector: objc::runtime::Sel, withObject: id) -> id {
        unsafe { msg_send![self.obj, performSelector: aSelector withObject: withObject] }
    }

    fn isProxy(&self) -> bool {
        unsafe { msg_send![self.obj, isProxy] }
    }
}

impl<K, V> INSDictionary<K, V> for NSDictionary<K, V> {
    fn count(&self) -> UInt {
        unsafe { msg_send![self.obj, count] }
    }
}

impl<K, V> fmt::Debug for NSDictionary<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.debugDescription())
    }
}

impl<K, V> fmt::Display for NSDictionary<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.description())
    }
}

impl<K, V> FromId for NSDictionary<K, V> {
    fn from_id(id: id) -> Self {
        NSDictionary {
            obj: unsafe { Id::from_ptr(id) },
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
        let cls: id =
            unsafe { msg_send![class!(NSDictionary), dictionaryWithDictionary: dict.clone()] };
        NSDictionary::from(cls)
    }
}
