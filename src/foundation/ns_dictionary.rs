use std::{
    fmt::{self, Formatter},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{id, objective_c_runtime::traits::t_NSObject};

use super::{traits::t_NSDictionary, Array, String, UInt};

/// A static collection of objects associated with unique keys.
pub struct Dictionary<K, V> {
    /// The raw pointer to the Objective-C object.
    pub obj: Id<Object>,
    _key: PhantomData<K>,
    _value: PhantomData<V>,
}

impl<K, V> t_NSObject for Dictionary<K, V> {
    fn init() -> Self {
        unsafe {
            let cls = class!(NSDictionary);
            let obj: *mut Object = msg_send![cls, new];
            let obj = msg_send![obj, init];
            Self {
                obj: Id::from_ptr(obj),
                _key: PhantomData,
                _value: PhantomData,
            }
        }
    }

    fn toId(self) -> id {
        todo!()
    }

    unsafe fn fromId(_obj: id) -> Self {
        todo!()
    }

    fn description(&self) -> String {
        let obj: id = unsafe { msg_send![self.obj, description] };
        unsafe { String::fromId(obj) }
    }

    fn debugDescription(&self) -> String {
        let obj: id = unsafe { msg_send![self.obj, debugDescription] };
        unsafe { String::fromId(obj) }
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

impl<K, V> t_NSDictionary<K, V> for Dictionary<K, V> {
    fn new() -> Self {
        unsafe {
            let cls = class!(NSDictionary);
            let obj: *mut Object = msg_send![cls, new];
            let obj = msg_send![obj, init];
            Self {
                obj: Id::from_ptr(obj),
                _key: PhantomData,
                _value: PhantomData,
            }
        }
    }

    fn dictionaryWithObjects(objects: Array<V>, keys: Array<K>) -> Self {
        unsafe {
            let cls = class!(NSDictionary);
            let obj: *mut Object = msg_send![cls, new];
            let obj = msg_send![obj, dictionaryWithObjects: objects forKeys: keys];
            Self {
                obj: Id::from_ptr(obj),
                _key: PhantomData,
                _value: PhantomData,
            }
        }
    }

    fn count(&self) -> UInt {
        unsafe { msg_send![self.obj, count] }
    }
}

impl<K, V> fmt::Debug for Dictionary<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.debugDescription())
    }
}

impl<K, V> fmt::Display for Dictionary<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.description())
    }
}

impl<K, V> Default for Dictionary<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> AsMut<Dictionary<K, V>> for Dictionary<K, V> {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<K, V> Deref for Dictionary<K, V> {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.obj
    }
}

impl<K, V> DerefMut for Dictionary<K, V> {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.obj
    }
}
