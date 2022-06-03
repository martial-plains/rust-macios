use std::{
    borrow::Borrow,
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    id,
    objective_c_runtime::traits::{FromId, PNSObject, ToId},
};

use super::{
    traits::{INSDictionary, INSMutableDictionary},
    NSArray, NSDictionary, NSString, UInt,
};

/// A dynamic collection of objects associated with unique keys.
pub struct NSMutableDictionary<K, V> {
    /// The raw pointer to the Objective-C object.
    pub obj: Id<Object>,
    _key: PhantomData<K>,
    _value: PhantomData<V>,
}

impl<K, V> PNSObject for NSMutableDictionary<K, V> {
    fn im_class<'a>() -> &'a objc::runtime::Class {
        class!(NSMutableDictionary)
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

impl<K, V> INSDictionary<K, V> for NSMutableDictionary<K, V> {
    fn ip_count(&self) -> UInt {
        unsafe { msg_send![self.obj, count] }
    }

    fn initWithDictionary(&mut self, dictionary: NSDictionary<K, V>) {
        unsafe { msg_send![self.obj, initWithDictionary: dictionary] }
    }
}

impl<K, V> INSMutableDictionary<K, V> for NSMutableDictionary<K, V> {
    fn im_setObject_forKey(&mut self, key: K, value: V)
    where
        K: PNSObject,
        V: PNSObject,
    {
        unsafe { msg_send![self.obj, setObject: value forKey: key] }
    }

    fn im_setObject_forkeyedSuperscript(&mut self, key: K, value: V)
    where
        K: Into<id>,
        V: Into<id>,
    {
        unsafe { msg_send![self.obj, setObject: value forKeyedSubscript: key] }
    }

    fn im_setValue_forKey(&mut self, key: K, value: V)
    where
        K: Into<NSString>,
        V: Into<id>,
    {
        unsafe { msg_send![self.obj, setValue: value forKey: key] }
    }

    fn im_addEntriesFromDictionary(&mut self, dictionary: NSDictionary<K, V>) {
        unsafe { msg_send![self.obj, addEntriesFromDictionary: dictionary] }
    }

    fn im_setDictionary(&mut self, dictionary: NSDictionary<K, V>) {
        unsafe { msg_send![self.obj, setDictionary: dictionary] }
    }

    fn im_removeObjectForKey(&mut self, key: K)
    where
        K: Into<id>,
    {
        unsafe { msg_send![self.obj, removeObjectForKey: key] }
    }

    fn im_removeAllObjects(&mut self) {
        unsafe { msg_send![self.obj, removeAllObjects] }
    }

    fn im_removeObjectsForKeys(&mut self, keys: NSArray<K>)
    where
        K: PNSObject,
    {
        unsafe { msg_send![self.obj, removeObjectsForKeys: keys] }
    }

    fn tm_dictionaryWithCapacity(capacity: UInt) -> Self
where {
        Self {
            obj: unsafe {
                Id::from_ptr(msg_send![
                    Self::im_class(),
                    dictionaryWithCapacity: capacity
                ])
            },
            _key: PhantomData,
            _value: PhantomData,
        }
    }
}

impl<K, V> ToId for NSMutableDictionary<K, V> {
    fn to_id(mut self) -> id {
        &mut *self.obj
    }
}

impl<K, V> FromId for NSMutableDictionary<K, V> {
    unsafe fn from_id(obj: id) -> Self {
        Self {
            obj: Id::from_ptr(obj),
            _key: PhantomData,
            _value: PhantomData,
        }
    }
}

impl<K, V> Default for NSMutableDictionary<K, V> {
    fn default() -> Self {
        Self::tm_dictionaryWithCapacity(0)
    }
}

impl<K, V> fmt::Debug for NSMutableDictionary<K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.ip_debugDescription())
    }
}

impl fmt::Display for NSMutableDictionary<NSString, NSString> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.ip_description())
    }
}

impl Borrow<NSDictionary<NSString, NSString>> for NSMutableDictionary<NSString, NSString> {
    fn borrow(&self) -> &NSDictionary<NSString, NSString> {
        unsafe { msg_send![self.obj, dictionaryWithDictionary: self] }
    }
}

impl<K, V> Deref for NSMutableDictionary<K, V> {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.obj
    }
}

impl<K, V> DerefMut for NSMutableDictionary<K, V> {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.obj
    }
}

impl<K, V> From<NSDictionary<K, V>> for NSMutableDictionary<K, V>
where
    K: PNSObject,
    V: PNSObject,
{
    fn from(dictionary: NSDictionary<K, V>) -> Self {
        Self {
            obj: dictionary.obj,

            _key: PhantomData,
            _value: PhantomData,
        }
    }
}

impl<K, V> From<UInt> for NSMutableDictionary<K, V>
where
    K: PNSObject,
    V: PNSObject,
{
    fn from(capacity: UInt) -> Self {
        Self {
            obj: unsafe {
                msg_send![
                    class!(NSMutableDictionary),
                    dictionaryWithCapacity: capacity
                ]
            },

            _key: PhantomData,
            _value: PhantomData,
        }
    }
}

impl<K, V> Clone for NSMutableDictionary<K, V> {
    fn clone(&self) -> Self {
        Self {
            obj: unsafe { msg_send![self.obj, retain] },

            _key: PhantomData,
            _value: PhantomData,
        }
    }
}
