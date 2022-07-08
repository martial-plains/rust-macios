use std::{borrow::Borrow, collections::HashMap, marker::PhantomData};

use objc::{class, msg_send, sel, sel_impl};

use crate::objective_c_runtime::{
    macros::object,
    traits::{FromId, PNSObject},
};

use super::{
    traits::{INSDictionary, INSMutableDictionary},
    Int, Int16, Int32, Int8, NSDictionary, NSNumber, NSString, UInt, UInt16, UInt32, UInt8,
};

object! {
    /// A dynamic collection of objects associated with unique keys.
    unsafe pub struct NSMutableDictionary<K,V> {
        _key: PhantomData<K>,
        _value: PhantomData<V>,
    }
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

impl<K, V> INSDictionary<K, V> for NSMutableDictionary<K, V> {}

impl<K, V> INSMutableDictionary<K, V> for NSMutableDictionary<K, V> {}

impl<K, V> Default for NSMutableDictionary<K, V> {
    fn default() -> Self {
        Self::tm_dictionary_with_capacity(0)
    }
}

impl Borrow<NSDictionary<NSString, NSString>> for NSMutableDictionary<NSString, NSString> {
    fn borrow(&self) -> &NSDictionary<NSString, NSString> {
        unsafe { msg_send![self.im_self(), dictionaryWithDictionary: self] }
    }
}

impl<K, V> From<NSDictionary<K, V>> for NSMutableDictionary<K, V>
where
    K: PNSObject,
    V: PNSObject,
{
    fn from(dictionary: NSDictionary<K, V>) -> Self {
        unsafe { Self::from_id(dictionary.im_self()) }
    }
}

impl<K, V> From<UInt> for NSMutableDictionary<K, V>
where
    K: PNSObject,
    V: PNSObject,
{
    fn from(capacity: UInt) -> Self {
        unsafe {
            Self::from_id(msg_send![
                class!(NSMutableDictionary),
                dictionaryWithCapacity: capacity
            ])
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
