use std::{collections::HashMap, marker::PhantomData};

use objc::{class, msg_send, sel, sel_impl};

use crate::{
    objective_c_runtime::{
        id,
        macros::shared_object,
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};

use super::{
    INSMutableDictionary, Int, Int16, Int32, Int8, NSArray, NSMutableDictionary, NSNumber,
    NSString, UInt, UInt16, UInt32, UInt8,
};

shared_object! {
    /// A static collection of objects associated with unique keys.
    unsafe pub struct NSDictionary<K, V> {
        _key: PhantomData<K>,
        _value: PhantomData<V>,
    }
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

/// A static collection of objects associated with unique keys.
pub trait INSDictionary<K, V>: PNSObject {
    /* Creating an Empty Dictionary
     */

    /// Creates an empty dictionary.
    fn tm_dictionary() -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), dictionary]) }
    }

    /// Initializes a newly allocated dictionary.
    fn im_init() -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), new]) }
    }

    /* Creating a Dictionary from Another Dictionary
     */

    /// Creates a dictionary containing the keys and values from another given dictionary.
    fn tm_dictionary_with_dictionary<D>(dictionary: D) -> Self
    where
        Self: Sized + FromId,
        D: INSDictionary<K, V>,
    {
        unsafe {
            Self::from_id(msg_send![
                Self::m_class(),
                dictionaryWithDictionary: dictionary
            ])
        }
    }

    /// Creates and initialize a dictionary
    fn im_init_with_dictionary(&mut self, dictionary: NSDictionary<K, V>) {
        unsafe { msg_send![self.m_self(), initWithDictionary: dictionary] }
    }

    /// Initializes a newly allocated dictionary using the objects contained in another given dictionary.
    fn im_init_with_dictionary_copy_items(&mut self, dictionary: NSDictionary<K, V>, flag: bool) {
        unsafe { msg_send![self.m_self(), initWithDictionary: dictionary copyItems: flag] }
    }

    /* Counting Entries
     */

    /// The number of entries in the dictionary.
    fn ip_count(&self) -> UInt {
        unsafe { msg_send![self.m_self(), count] }
    }

    /* Comparing Dictionaries
     */

    /// Returns a Boolean value that indicates whether the contents of the receiving dictionary are equal to the contents of another given dictionary.
    fn im_is_equal_to_dictionary<D>(&self, other: D) -> bool
    where
        D: INSDictionary<K, V>,
    {
        unsafe { to_bool(msg_send![self.m_self(), isEqualToDictionary: other.m_self()]) }
    }

    /* Accessing Keys and Values
     */

    /// A new array containing the dictionary’s keys, or an empty array if the dictionary has no entries.
    fn ip_all_keys(&self) -> NSArray<K> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), allKeys]) }
    }

    /// Returns a new array containing the keys corresponding to all occurrences of a given object in the dictionary.
    fn im_all_keys_for_object(&self, object: &V) -> NSArray<K> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), allKeysForObject: object]) }
    }

    /// A new array containing the dictionary’s values, or an empty array if the dictionary has no entries.
    fn ip_all_values(&self) -> NSArray<V> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), allValues]) }
    }

    /// Returns the value associated with a given key.
    fn im_value_for_key(&self, key: &K) -> V
    where
        V: FromId,
    {
        unsafe { V::from_id(msg_send![self.m_self(), valueForKey: key]) }
    }

    /// Returns by reference C arrays of the keys and values in the dictionary.
    fn im_get_objects_and_keys_count(&self, objects: *mut V, keys: *mut K, count: UInt) {
        unsafe { msg_send![self.m_self(), getObjects: objects andKeys: keys count: count] }
    }

    /// Returns as a static array the set of objects from the dictionary that corresponds to the specified keys.
    fn im_objects_for_keys_not_found_marker(&self, keys: &NSArray<K>, value: &V) -> NSArray<V> {
        unsafe {
            NSArray::from_id(msg_send![self.m_self(), objectsForKeys: keys notFoundMarker: value])
        }
    }

    /// Returns the value associated with a given key.
    fn im_object_for_key(&self, key: K) -> V
    where
        V: FromId,
    {
        unsafe { V::from_id(msg_send![self.m_self(), objectForKey: key]) }
    }

    /// Returns the value associated with a given key.
    fn im_object_for_keyed_subscript(&self, key: &K) -> V
    where
        V: FromId,
    {
        unsafe { V::from_id(msg_send![self.m_self(), objectForKeyedSubscript: key]) }
    }
}

impl<K, V> INSDictionary<K, V> for NSDictionary<K, V> {}

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

impl<K, V> From<id> for NSDictionary<K, V> {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from(obj: id) -> Self {
        unsafe { Self::from_id(obj) }
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
