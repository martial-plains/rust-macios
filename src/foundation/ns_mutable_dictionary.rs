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
    objective_c_runtime::traits::{FromId, PNSObject},
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
    fn class<'a>() -> &'a objc::runtime::Class {
        class!(NSMutableDictionary)
    }

    fn superclass<'a>() -> &'a objc::runtime::Class {
        unsafe { msg_send![Self::class(), superclass] }
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
        unsafe { msg_send![self.obj, description] }
    }

    fn debugDescription(&self) -> NSString {
        unsafe { msg_send![self.obj, debugDescription] }
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

impl<K, V> INSDictionary<K, V> for NSMutableDictionary<K, V> {
    fn count(&self) -> UInt {
        unsafe { msg_send![self.obj, count] }
    }
}

impl<K, V> INSMutableDictionary<K, V> for NSMutableDictionary<K, V> {
    fn initWithDictionary(&mut self, dictionary: NSDictionary<K, V>) {
        unsafe { msg_send![self.obj, initWithDictionary: dictionary] }
    }

    fn setObject(&mut self, key: K, value: V)
    where
        K: PNSObject,
        V: PNSObject,
    {
        unsafe { msg_send![self.obj, setObject: value forKey: key] }
    }

    fn setObjectForKeyedSuperscript(&mut self, key: K, value: V)
    where
        K: Into<id>,
        V: Into<id>,
    {
        unsafe { msg_send![self.obj, setObject: value forKeyedSubscript: key] }
    }

    fn setValue(&mut self, key: K, value: V)
    where
        K: Into<NSString>,
        V: Into<id>,
    {
        unsafe { msg_send![self.obj, setValue: value forKey: key] }
    }

    fn addEntriesFromDictionary(&mut self, dictionary: NSDictionary<K, V>) {
        unsafe { msg_send![self.obj, addEntriesFromDictionary: dictionary] }
    }

    fn setDictionary(&mut self, dictionary: NSDictionary<K, V>) {
        unsafe { msg_send![self.obj, setDictionary: dictionary] }
    }

    fn removeObjectForKey(&mut self, key: K)
    where
        K: Into<id>,
    {
        unsafe { msg_send![self.obj, removeObjectForKey: key] }
    }

    fn removeAllObjects(&mut self) {
        unsafe { msg_send![self.obj, removeAllObjects] }
    }

    fn removeObjectsForKeys(&mut self, keys: NSArray<K>)
    where
        K: PNSObject,
    {
        unsafe { msg_send![self.obj, removeObjectsForKeys: keys] }
    }

    fn dictionaryWithCapacity(capacity: UInt) -> Self
where {
        Self {
            obj: unsafe {
                Id::from_ptr(msg_send![Self::class(), dictionaryWithCapacity: capacity])
            },
            _key: PhantomData,
            _value: PhantomData,
        }
    }
}

impl FromId for NSMutableDictionary<NSString, NSString> {
    fn from_id(obj: id) -> Self {
        Self {
            obj: unsafe { Id::from_ptr(obj) },
            _key: PhantomData,
            _value: PhantomData,
        }
    }
}

impl<K, V> Default for NSMutableDictionary<K, V> {
    fn default() -> Self {
        Self::dictionaryWithCapacity(0)
    }
}

impl<K, V> fmt::Debug for NSMutableDictionary<K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.debugDescription())
    }
}

impl fmt::Display for NSMutableDictionary<NSString, NSString> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.description())
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
