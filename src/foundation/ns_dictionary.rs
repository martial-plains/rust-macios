use std::{
    collections::HashMap,
    fmt::{self, Formatter},
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
    Int, Int16, Int32, Int8, NSArray, NSMutableDictionary, NSNumber, NSString, UInt, UInt16,
    UInt32, UInt8,
};

/// A static collection of objects associated with unique keys.
pub struct NSDictionary<K, V> {
    /// The raw pointer to the Objective-C object.
    pub obj: Id<Object>,
    _key: PhantomData<K>,
    _value: PhantomData<V>,
}

impl<K, V> NSDictionary<K, V> {
    /// Returns an empty dictionary.
    pub fn new() -> Self {
        Self::tm_dictionary()
    }

    /// The number of entries in the dictionary.
    pub fn count(&self) -> UInt {
        self.ip_count()
    }

    /// Returns the value associated with a given key.
    pub fn object_for_key(&self, key: K) -> V
    where
        K: Clone,
        V: FromId,
    {
        self.im_object_for_key(key)
    }
}

impl<K, V> PNSObject for NSDictionary<K, V> {
    fn im_class<'a>() -> &'a Class {
        class!(NSDictionary)
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

impl<K, V> INSDictionary<K, V> for NSDictionary<K, V> {
    fn tm_dictionary() -> Self {
        unsafe { Self::from_id(msg_send![Self::im_class(), dictionary]) }
    }

    fn im_init() -> Self {
        unsafe { Self::from_id(msg_send![Self::im_class(), init]) }
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

    fn im_init_with_dictionary(&mut self, dictionary: NSDictionary<K, V>) {
        unsafe { msg_send![self.obj, initWithDictionary: dictionary] }
    }

    fn im_init_with_dictionary_copy_items(&mut self, dictionary: NSDictionary<K, V>, flag: bool) {
        unsafe { msg_send![self.obj, initWithDictionary: dictionary copyItems: flag] }
    }

    fn ip_count(&self) -> UInt {
        unsafe { msg_send![self.obj, count] }
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
        unsafe { msg_send![self.obj, getObjects: objects andKeys: keys count: count] }
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

impl<K, V> fmt::Debug for NSDictionary<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.ip_debug_description())
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
            let obj: id = msg_send![self.obj, retain];
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

impl<K, V> From<HashMap<K, V>> for NSDictionary<K, V>
where
    K: PNSObject,
    V: PNSObject,
{
    fn from(map: HashMap<K, V>) -> NSDictionary<K, V> {
        let mut dictionary = NSMutableDictionary::<K, V>::default();

        for (key, value) in map {
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl<K> From<HashMap<K, &str>> for NSDictionary<K, NSString>
where
    K: PNSObject,
{
    fn from(map: HashMap<K, &str>) -> NSDictionary<K, NSString> {
        let mut dictionary = NSMutableDictionary::<K, NSString>::default();

        for (key, value) in map {
            let value = NSString::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl<V> From<HashMap<&str, V>> for NSDictionary<NSString, V>
where
    V: PNSObject,
{
    fn from(map: HashMap<&str, V>) -> NSDictionary<NSString, V> {
        let mut dictionary = NSMutableDictionary::<NSString, V>::default();

        for (key, value) in map {
            let key = NSString::from(key);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<&str, &str>> for NSDictionary<NSString, NSString> {
    fn from(map: HashMap<&str, &str>) -> NSDictionary<NSString, NSString> {
        let mut dictionary = NSMutableDictionary::<NSString, NSString>::default();

        for (key, value) in map {
            let key = NSString::from(key);
            let value = NSString::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<&str, UInt>> for NSDictionary<NSString, NSNumber> {
    fn from(map: HashMap<&str, UInt>) -> NSDictionary<NSString, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSString, NSNumber>::default();

        for (key, value) in map {
            let key = NSString::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<&str, UInt8>> for NSDictionary<NSString, NSNumber> {
    fn from(map: HashMap<&str, UInt8>) -> NSDictionary<NSString, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSString, NSNumber>::default();

        for (key, value) in map {
            let key = NSString::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<&str, UInt16>> for NSDictionary<NSString, NSNumber> {
    fn from(map: HashMap<&str, UInt16>) -> NSDictionary<NSString, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSString, NSNumber>::default();

        for (key, value) in map {
            let key = NSString::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<&str, UInt32>> for NSDictionary<NSString, NSNumber> {
    fn from(map: HashMap<&str, UInt32>) -> NSDictionary<NSString, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSString, NSNumber>::default();

        for (key, value) in map {
            let key = NSString::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<&str, Int>> for NSDictionary<NSString, NSNumber> {
    fn from(map: HashMap<&str, Int>) -> NSDictionary<NSString, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSString, NSNumber>::default();

        for (key, value) in map {
            let key = NSString::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<&str, Int8>> for NSDictionary<NSString, NSNumber> {
    fn from(map: HashMap<&str, Int8>) -> NSDictionary<NSString, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSString, NSNumber>::default();

        for (key, value) in map {
            let key = NSString::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<&str, Int16>> for NSDictionary<NSString, NSNumber> {
    fn from(map: HashMap<&str, Int16>) -> NSDictionary<NSString, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSString, NSNumber>::default();

        for (key, value) in map {
            let key = NSString::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<&str, Int32>> for NSDictionary<NSString, NSNumber> {
    fn from(map: HashMap<&str, Int32>) -> NSDictionary<NSString, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSString, NSNumber>::default();

        for (key, value) in map {
            let key = NSString::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt, &str>> for NSDictionary<NSNumber, NSString> {
    fn from(map: HashMap<UInt, &str>) -> NSDictionary<NSNumber, NSString> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSString>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSString::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt, UInt>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt, UInt>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt, UInt8>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt, UInt8>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt, UInt16>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt, UInt16>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt, UInt32>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt, UInt32>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt, Int>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt, Int>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt, Int8>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt, Int8>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt, Int16>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt, Int16>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt, Int32>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt, Int32>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt8, &str>> for NSDictionary<NSNumber, NSString> {
    fn from(map: HashMap<UInt8, &str>) -> NSDictionary<NSNumber, NSString> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSString>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSString::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt8, UInt>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt8, UInt>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt8, UInt8>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt8, UInt8>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt8, UInt16>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt8, UInt16>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt8, UInt32>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt8, UInt32>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt8, Int>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt8, Int>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt8, Int8>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt8, Int8>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt8, Int16>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt8, Int16>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt8, Int32>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt8, Int32>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt16, &str>> for NSDictionary<NSNumber, NSString> {
    fn from(map: HashMap<UInt16, &str>) -> NSDictionary<NSNumber, NSString> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSString>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSString::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt16, UInt>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt16, UInt>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt16, UInt8>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt16, UInt8>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt16, UInt16>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt16, UInt16>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt16, UInt32>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt16, UInt32>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt16, Int>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt16, Int>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt16, Int8>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt16, Int8>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt16, Int16>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt16, Int16>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt16, Int32>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt16, Int32>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt32, &str>> for NSDictionary<NSNumber, NSString> {
    fn from(map: HashMap<UInt32, &str>) -> NSDictionary<NSNumber, NSString> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSString>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSString::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt32, UInt>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt32, UInt>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt32, UInt8>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt32, UInt8>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt32, UInt16>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt32, UInt16>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt32, UInt32>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt32, UInt32>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt32, Int>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt32, Int>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt32, Int8>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt32, Int8>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt32, Int16>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt32, Int16>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<UInt32, Int32>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<UInt32, Int32>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int, &str>> for NSDictionary<NSNumber, NSString> {
    fn from(map: HashMap<Int, &str>) -> NSDictionary<NSNumber, NSString> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSString>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSString::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int, UInt>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int, UInt>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int, UInt8>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int, UInt8>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int, UInt16>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int, UInt16>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int, UInt32>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int, UInt32>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int, Int>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int, Int>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int, Int8>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int, Int8>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int, Int16>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int, Int16>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int, Int32>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int, Int32>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int8, &str>> for NSDictionary<NSNumber, NSString> {
    fn from(map: HashMap<Int8, &str>) -> NSDictionary<NSNumber, NSString> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSString>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSString::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int8, UInt>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int8, UInt>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int8, UInt8>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int8, UInt8>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int8, UInt16>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int8, UInt16>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int8, UInt32>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int8, UInt32>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int8, Int>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int8, Int>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int8, Int8>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int8, Int8>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int8, Int16>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int8, Int16>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int8, Int32>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int8, Int32>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int16, &str>> for NSDictionary<NSNumber, NSString> {
    fn from(map: HashMap<Int16, &str>) -> NSDictionary<NSNumber, NSString> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSString>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSString::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int16, UInt>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int16, UInt>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int16, UInt8>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int16, UInt8>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int16, UInt16>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int16, UInt16>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int16, UInt32>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int16, UInt32>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int16, Int>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int16, Int>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int16, Int8>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int16, Int8>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int16, Int16>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int16, Int16>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int16, Int32>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int16, Int32>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int32, &str>> for NSDictionary<NSNumber, NSString> {
    fn from(map: HashMap<Int32, &str>) -> NSDictionary<NSNumber, NSString> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSString>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSString::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int32, UInt>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int32, UInt>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int32, UInt8>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int32, UInt8>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int32, UInt16>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int32, UInt16>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int32, UInt32>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int32, UInt32>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int32, Int>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int32, Int>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int32, Int8>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int32, Int8>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int32, Int16>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int32, Int16>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}

impl From<HashMap<Int32, Int32>> for NSDictionary<NSNumber, NSNumber> {
    fn from(map: HashMap<Int32, Int32>) -> NSDictionary<NSNumber, NSNumber> {
        let mut dictionary = NSMutableDictionary::<NSNumber, NSNumber>::default();

        for (key, value) in map {
            let key = NSNumber::from(key);
            let value = NSNumber::from(value);
            dictionary.im_set_object_for_key(key, value);
        }

        dictionary.into()
    }
}
