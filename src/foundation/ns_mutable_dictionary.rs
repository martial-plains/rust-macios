use std::{
    borrow::Borrow,
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    time::Duration,
};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{id, objective_c_runtime::traits::t_NSObject};

use super::{Array, Dictionary, String};

/// A dynamic collection of objects associated with unique keys.
pub struct MutableDictionary<K, V> {
    /// The raw pointer to the Objective-C object.
    pub obj: Id<Object>,
    _key: PhantomData<K>,
    _value: PhantomData<V>,
}

impl<K, V> MutableDictionary<K, V> {
    /// Creates an empty dictionary.
    pub fn new() -> Self {
        unsafe {
            let cls = class!(NSMutableDictionary);
            Self {
                obj: Id::from_ptr(msg_send![cls, new]),
                _key: PhantomData,
                _value: PhantomData,
            }
        }
    }

    /// Creates and ini
    pub fn init(&self) -> Self {
        unsafe {
            let cls = class!(NSMutableDictionary);
            let obj: *mut Object = msg_send![cls, new];
            let obj = msg_send![obj, init];
            Self {
                obj: Id::from_ptr(obj),
                _key: PhantomData,
                _value: PhantomData,
            }
        }
    }

    /// Creates and initialize a dictionary
    pub fn init_with_dictionary(&mut self, dictionary: Dictionary<K, V>) {
        unsafe {
            let obj: *mut Object = msg_send![self.obj, setDictionary: dictionary.obj];
            self.obj = Id::from_ptr(obj);
        }
    }

    /* Adding Entries to a Mutable Dictionary
     */

    /// Adds a given key-value pair to the dictionary.
    pub fn set_object(&mut self, key: K, value: V)
    where
        K: t_NSObject,
        V: t_NSObject,
    {
        unsafe {
            let _: () = msg_send![self.obj, setObject: value.toId() forKey: &*key.toId()];

            // TODO: Fix this function where the key value can be set without having the need for the thread to sleep after the call.
            std::thread::sleep(Duration::from_micros(10));
        }
    }

    /// Adds a given key-value pair to the dictionary.
    pub fn set_object_for_keyed_superscript(&mut self, key: K, value: V)
    where
        K: Into<id>,
        V: Into<id>,
    {
        unsafe {
            let key: id = key.into();
            let value: id = value.into();
            let obj: *mut Object = msg_send![self.obj, setObject: value forKeyedSubscript: key];
            self.obj = Id::from_ptr(obj);
        }
    }

    /// Adds a given key-value pair to the dictionary.
    pub fn set_value(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<id>,
    {
        unsafe {
            let _: () = msg_send![self.obj, setValue: value.into() forKey: &*key.into()];
        }
    }

    /// Adds to the receiving dictionary the entries from another dictionary.
    pub fn add_entries_from_dictionary(&mut self, dictionary: Dictionary<K, V>) {
        unsafe {
            let obj: *mut Object = msg_send![self.obj, addEntriesFromDictionary: dictionary.obj];
            self.obj = Id::from_ptr(obj);
        }
    }

    /// Sets the contents of the receiving dictionary to entries in a given dictionary.
    pub fn set_dictionary(&mut self, dictionary: Dictionary<K, V>) {
        unsafe {
            let obj: *mut Object = msg_send![self.obj, setDictionary: dictionary.obj];
            self.obj = Id::from_ptr(obj);
        }
    }

    /* Removing Entries From a Mutable Dictionary
     */

    /// Removes a given key and its associated value from the dictionary.
    pub fn remove_object_for_key(&mut self, key: K)
    where
        K: Into<id>,
    {
        unsafe {
            let key: id = key.into();
            let obj: *mut Object = msg_send![self.obj, removeObjectForKey: key];
            self.obj = Id::from_ptr(obj);
        }
    }

    /// Empties the dictionary of its entries.
    pub fn remove_all_objects(&mut self) {
        unsafe {
            let obj: *mut Object = msg_send![self.obj, removeAllObjects];
            self.obj = Id::from_ptr(obj);
        }
    }

    /// Removes from the dictionary entries specified by elements in a given array.
    pub fn remove_objects_for_keys(&mut self, keys: Array<K>)
    where
        K: t_NSObject,
    {
        unsafe {
            let keys: id = keys.toId();
            let obj: *mut Object = msg_send![self.obj, removeObjectsForKeys: keys];
            self.obj = Id::from_ptr(obj);
        }
    }
}

impl<K, V> Default for MutableDictionary<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> t_NSObject for MutableDictionary<K, V> {
    fn init() -> Self {
        todo!()
    }

    #[allow(trivial_casts)]
    fn toId(self) -> id {
        &*self as *const _ as *mut _
    }

    fn fromId(_obj: id) -> Self {
        todo!()
    }

    fn description(&self) -> String {
        let obj: id = unsafe { msg_send![self.obj, description] };
        String::fromId(obj)
    }

    fn debugDescription(&self) -> String {
        let obj: id = unsafe { msg_send![self.obj, debugDescription] };
        String::fromId(obj)
    }

    fn retain(&self) -> Self {
        let obj: id = unsafe { msg_send![&*self.obj, retain] };
        Self {
            obj: unsafe { Id::from_ptr(obj) },

            _key: PhantomData,
            _value: PhantomData,
        }
    }
}

impl<K, V> fmt::Debug for MutableDictionary<K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.debugDescription())
    }
}

impl fmt::Display for MutableDictionary<String, String> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.description())
    }
}

impl Borrow<Dictionary<String, String>> for MutableDictionary<String, String> {
    fn borrow(&self) -> &Dictionary<String, String> {
        unsafe { msg_send![self.obj, dictionaryWithDictionary: self] }
    }
}

impl<K, V> Deref for MutableDictionary<K, V> {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.obj
    }
}

impl<K, V> DerefMut for MutableDictionary<K, V> {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.obj
    }
}

impl<K, V> From<Dictionary<K, V>> for MutableDictionary<K, V>
where
    K: Into<id>,
    V: Into<id>,
{
    fn from(dictionary: Dictionary<K, V>) -> Self {
        Self {
            obj: dictionary.obj,

            _key: PhantomData,
            _value: PhantomData,
        }
    }
}
