use std::{
    borrow::Borrow,
    collections::HashMap,
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use objc::{
    class, msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject, ToId},
    },
    utils::to_bool,
};

use super::{
    traits::{INSDictionary, INSMutableDictionary},
    Int, Int16, Int32, Int8, NSArray, NSDictionary, NSNumber, NSString, UInt, UInt16, UInt32,
    UInt8,
};

/// A dynamic collection of objects associated with unique keys.
pub struct NSMutableDictionary<K, V> {
    /// The raw pointer to the Objective-C object.
    pub obj: Id<Object>,
    _key: PhantomData<K>,
    _value: PhantomData<V>,
}

impl<K, V> NSMutableDictionary<K, V> {
    /// Creates a new empty dictionary.
    pub fn new() -> Self {
        Self::im_init()
    }

    /// Creates a new dictionary with the specified capacity.
    pub fn with_capacity(capacity: UInt) -> Self {
        Self::tm_dictionary_with_capacity(capacity)
    }

    /// Creates a new dictionary with the specified capacity and load factor.
    pub fn insert(&mut self, key: K, value: V)
    where
        K: PNSObject,
        V: PNSObject,
    {
        self.im_set_object_for_key(key, value)
    }
}

impl<K, V> PNSObject for NSMutableDictionary<K, V> {
    fn im_class<'a>() -> &'a Class {
        class!(NSMutableDictionary)
    }

    fn im_is_equal(&self, object: &Self) -> bool {
        unsafe { msg_send![self.obj, isEqual: object] }
    }

    fn ip_hash(&self) -> UInt {
        unsafe { msg_send![self.obj, hash] }
    }

    fn im_is_kind_of_class(&self, class: Class) -> bool {
        unsafe { msg_send![self.obj, isKindOfClass: class] }
    }

    fn im_is_member_of_class(&self, class: Class) -> bool {
        unsafe { msg_send![self.obj, isMemberOfClass: class] }
    }

    fn im_responds_to_selector(&self, selector: Sel) -> bool {
        unsafe { msg_send![self.obj, respondsToSelector: selector] }
    }

    fn im_conforms_to_protocol(&self, protocol: Protocol) -> bool {
        unsafe { msg_send![self.obj, conformsToProtocol: protocol] }
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
        unsafe { msg_send![self.obj, isProxy] }
    }
}

impl<K, V> INSDictionary<K, V> for NSMutableDictionary<K, V> {
    fn im_init_with_dictionary(&mut self, dictionary: NSDictionary<K, V>) {
        unsafe { msg_send![self.obj, initWithDictionary: dictionary] }
    }

    fn ip_count(&self) -> UInt {
        unsafe { msg_send![self.obj, count] }
    }

    fn tm_dictionary() -> Self {
        unsafe { Self::from_id(msg_send![Self::im_class(), dictionary]) }
    }

    fn im_init() -> Self {
        unsafe { Self::from_id(msg_send![Self::im_class(), new]) }
    }

    fn tm_dictionary_with_dictionary<D>(dictionary: D) -> Self
    where
        D: INSDictionary<K, V>,
    {
        unsafe {
            Self::from_id(msg_send![
                Self::im_class(),
                dictionaryWithDictionary: dictionary
            ])
        }
    }

    fn im_init_with_dictionary_copy_items(&mut self, dictionary: NSDictionary<K, V>, flag: bool) {
        unsafe { msg_send![self.obj, initWithDictionary: dictionary copyItems: flag] }
    }

    fn im_is_equal_to_dictionary<D>(&self, other: D) -> bool
    where
        D: INSDictionary<K, V>,
    {
        unsafe { to_bool(msg_send![self.obj, isEqualToDictionary: other]) }
    }

    fn ip_all_keys(&self) -> NSArray<K> {
        unsafe { NSArray::from_id(msg_send![self.obj, allKeys]) }
    }

    fn im_all_keys_for_object(&self, object: &V) -> NSArray<K> {
        unsafe { NSArray::from_id(msg_send![self.obj, allKeysForObject: object]) }
    }

    fn ip_all_values(&self) -> NSArray<V> {
        unsafe { NSArray::from_id(msg_send![self.obj, allValues]) }
    }

    fn im_value_for_key(&self, key: &K) -> V
    where
        V: FromId,
    {
        unsafe { V::from_id(msg_send![self.obj, valueForKey: key]) }
    }

    fn im_get_objects_and_keys_count(&self, objects: *mut V, keys: *mut K, count: UInt) {
        unsafe {
            msg_send![
                self.obj,
                getObjects: objects
                andKeys: keys
                count: count
            ]
        }
    }

    fn im_objects_for_keys_not_found_marker(&self, keys: &NSArray<K>, value: &V) -> NSArray<V> {
        unsafe { NSArray::from_id(msg_send![self.obj, objectsForKeys: keys notFoundMarker: value]) }
    }

    fn im_object_for_key(&self, key: K) -> V
    where
        V: FromId,
    {
        unsafe { V::from_id(msg_send![self.obj, objectForKey: key]) }
    }

    fn im_object_for_keyed_subscript(&self, key: &K) -> V
    where
        V: FromId,
    {
        unsafe { V::from_id(msg_send![self.obj, objectForKeyedSubscript: key]) }
    }
}

impl<K, V> INSMutableDictionary<K, V> for NSMutableDictionary<K, V> {
    fn im_set_object_for_key(&mut self, key: K, value: V)
    where
        K: PNSObject,
        V: PNSObject,
    {
        unsafe { msg_send![self.obj, setObject: value forKey: key] }
    }

    fn im_set_object_forkeyed_superscript(&mut self, key: K, value: V)
    where
        K: Into<id>,
        V: Into<id>,
    {
        unsafe { msg_send![self.obj, setObject: value forKeyedSubscript: key] }
    }

    fn im_set_value_for_key(&mut self, key: K, value: V)
    where
        K: Into<NSString>,
        V: Into<id>,
    {
        unsafe { msg_send![self.obj, setValue: value forKey: key] }
    }

    fn im_add_entries_from_dictionary(&mut self, dictionary: NSDictionary<K, V>) {
        unsafe { msg_send![self.obj, addEntriesFromDictionary: dictionary] }
    }

    fn im_set_dictionary(&mut self, dictionary: NSDictionary<K, V>) {
        unsafe { msg_send![self.obj, setDictionary: dictionary] }
    }

    fn im_remove_object_for_key(&mut self, key: K)
    where
        K: Into<id>,
    {
        unsafe { msg_send![self.obj, removeObjectForKey: key] }
    }

    fn im_remove_all_objects(&mut self) {
        unsafe { msg_send![self.obj, removeAllObjects] }
    }

    fn im_remove_objects_for_keys(&mut self, keys: NSArray<K>)
    where
        K: PNSObject,
    {
        unsafe { msg_send![self.obj, removeObjectsForKeys: keys] }
    }

    fn tm_dictionary_with_capacity(capacity: UInt) -> Self
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
        Self::tm_dictionary_with_capacity(0)
    }
}

impl<K, V> fmt::Debug for NSMutableDictionary<K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.ip_debug_description())
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

impl<K, V> From<HashMap<K, V>> for NSMutableDictionary<K, V>
where
    K: PNSObject,
    V: PNSObject,
{
    fn from(map: HashMap<K, V>) -> NSMutableDictionary<K, V> {
        let mut dictionary = NSMutableDictionary::<K, V>::default();

        for (key, value) in map {
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl<K> From<HashMap<K, &str>> for NSMutableDictionary<K, NSString>
where
    K: PNSObject,
{
    fn from(map: HashMap<K, &str>) -> NSMutableDictionary<K, NSString> {
        let mut dictionary = NSMutableDictionary::<K, NSString>::default();

        for (key, value) in map {
            let value = NSString::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl<V> From<HashMap<&str, V>> for NSMutableDictionary<NSString, V>
where
    V: PNSObject,
{
    fn from(map: HashMap<&str, V>) -> NSMutableDictionary<NSString, V> {
        let mut dictionary = NSMutableDictionary::<NSString, V>::default();

        for (key, value) in map {
            let key = NSString::from(key);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<&str, &str>> for NSMutableDictionary<NSString, NSString> {
    fn from(map: HashMap<&str, &str>) -> NSMutableDictionary<NSString, NSString> {
        let mut dictionary = NSMutableDictionary::<NSString, NSString>::default();

        for (key, value) in map {
            let key = NSString::from(key);
            let value = NSString::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<&str, UInt>> for NSMutableDictionary<NSString, NSNumber> {
    fn from(map: HashMap<&str, UInt>) -> NSMutableDictionary<NSString, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSString, NSNumber>::default();

        for (key, value) in map {
            let key = NSString::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<&str, UInt8>> for NSMutableDictionary<NSString, NSNumber> {
    fn from(map: HashMap<&str, UInt8>) -> NSMutableDictionary<NSString, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSString, NSNumber>::default();

        for (key, value) in map {
            let key = NSString::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<&str, UInt16>> for NSMutableDictionary<NSString, NSNumber> {
    fn from(map: HashMap<&str, UInt16>) -> NSMutableDictionary<NSString, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSString, NSNumber>::default();

        for (key, value) in map {
            let key = NSString::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<&str, UInt32>> for NSMutableDictionary<NSString, NSNumber> {
    fn from(map: HashMap<&str, UInt32>) -> NSMutableDictionary<NSString, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSString, NSNumber>::default();

        for (key, value) in map {
            let key = NSString::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<&str, Int>> for NSMutableDictionary<NSString, NSNumber> {
    fn from(map: HashMap<&str, Int>) -> NSMutableDictionary<NSString, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSString, NSNumber>::default();

        for (key, value) in map {
            let key = NSString::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<&str, Int8>> for NSMutableDictionary<NSString, NSNumber> {
    fn from(map: HashMap<&str, Int8>) -> NSMutableDictionary<NSString, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSString, NSNumber>::default();

        for (key, value) in map {
            let key = NSString::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<&str, Int16>> for NSMutableDictionary<NSString, NSNumber> {
    fn from(map: HashMap<&str, Int16>) -> NSMutableDictionary<NSString, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSString, NSNumber>::default();

        for (key, value) in map {
            let key = NSString::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<&str, Int32>> for NSMutableDictionary<NSString, NSNumber> {
    fn from(map: HashMap<&str, Int32>) -> NSMutableDictionary<NSString, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSString, NSNumber>::default();

        for (key, value) in map {
            let key = NSString::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt, &str>> for NSMutableDictionary<NSNumber, NSString> {
    fn from(map: HashMap<UInt, &str>) -> NSMutableDictionary<NSNumber, NSString> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSString>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSString::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt, UInt>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt, UInt>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt, UInt8>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt, UInt8>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt, UInt16>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt, UInt16>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt, UInt32>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt, UInt32>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt, Int>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt, Int>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt, Int8>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt, Int8>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt, Int16>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt, Int16>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt, Int32>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt, Int32>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt8, &str>> for NSMutableDictionary<NSNumber, NSString> {
    fn from(map: HashMap<UInt8, &str>) -> NSMutableDictionary<NSNumber, NSString> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSString>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSString::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt8, UInt>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt8, UInt>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt8, UInt8>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt8, UInt8>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt8, UInt16>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt8, UInt16>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt8, UInt32>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt8, UInt32>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt8, Int>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt8, Int>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt8, Int8>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt8, Int8>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt8, Int16>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt8, Int16>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt8, Int32>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt8, Int32>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt16, &str>> for NSMutableDictionary<NSNumber, NSString> {
    fn from(map: HashMap<UInt16, &str>) -> NSMutableDictionary<NSNumber, NSString> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSString>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSString::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt16, UInt>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt16, UInt>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt16, UInt8>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt16, UInt8>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt16, UInt16>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt16, UInt16>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt16, UInt32>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt16, UInt32>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt16, Int>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt16, Int>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt16, Int8>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt16, Int8>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt16, Int16>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt16, Int16>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt16, Int32>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt16, Int32>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt32, &str>> for NSMutableDictionary<NSNumber, NSString> {
    fn from(map: HashMap<UInt32, &str>) -> NSMutableDictionary<NSNumber, NSString> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSString>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSString::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt32, UInt>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt32, UInt>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt32, UInt8>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt32, UInt8>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt32, UInt16>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt32, UInt16>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt32, UInt32>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt32, UInt32>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt32, Int>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt32, Int>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt32, Int8>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt32, Int8>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt32, Int16>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt32, Int16>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<UInt32, Int32>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt32, Int32>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int, &str>> for NSMutableDictionary<NSNumber, NSString> {
    fn from(map: HashMap<Int, &str>) -> NSMutableDictionary<NSNumber, NSString> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSString>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSString::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int, UInt>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int, UInt>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int, UInt8>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int, UInt8>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int, UInt16>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int, UInt16>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int, UInt32>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int, UInt32>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int, Int>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int, Int>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int, Int8>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int, Int8>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int, Int16>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int, Int16>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int, Int32>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int, Int32>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int8, &str>> for NSMutableDictionary<NSNumber, NSString> {
    fn from(map: HashMap<Int8, &str>) -> NSMutableDictionary<NSNumber, NSString> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSString>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSString::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int8, UInt>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int8, UInt>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int8, UInt8>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int8, UInt8>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int8, UInt16>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int8, UInt16>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int8, UInt32>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int8, UInt32>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int8, Int>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int8, Int>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int8, Int8>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int8, Int8>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int8, Int16>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int8, Int16>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int8, Int32>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int8, Int32>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int16, &str>> for NSMutableDictionary<NSNumber, NSString> {
    fn from(map: HashMap<Int16, &str>) -> NSMutableDictionary<NSNumber, NSString> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSString>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSString::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int16, UInt>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int16, UInt>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int16, UInt8>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int16, UInt8>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int16, UInt16>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int16, UInt16>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int16, UInt32>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int16, UInt32>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int16, Int>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int16, Int>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int16, Int8>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int16, Int8>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int16, Int16>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int16, Int16>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int16, Int32>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int16, Int32>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int32, &str>> for NSMutableDictionary<NSNumber, NSString> {
    fn from(map: HashMap<Int32, &str>) -> NSMutableDictionary<NSNumber, NSString> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSString>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSString::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int32, UInt>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int32, UInt>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int32, UInt8>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int32, UInt8>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int32, UInt16>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int32, UInt16>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int32, UInt32>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int32, UInt32>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int32, Int>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int32, Int>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int32, Int8>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int32, Int8>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int32, Int16>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int32, Int16>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}

impl From<HashMap<Int32, Int32>> for NSMutableDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int32, Int32>) -> NSMutableDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary
    }
}
