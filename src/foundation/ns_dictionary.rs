use std::{
    fmt::{self, Formatter},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{id, objective_c_runtime::traits::t_NSObject};

use super::{traits::t_NSDictionary, NSString, UInt};

/// A static collection of objects associated with unique keys.
pub struct NSDictionary<K, V> {
    /// The raw pointer to the Objective-C object.
    pub obj: Id<Object>,
    _key: PhantomData<K>,
    _value: PhantomData<V>,
}

impl<K, V> t_NSObject for NSDictionary<K, V> {
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

    fn toId(self) -> id {
        todo!()
    }

    unsafe fn fromId(_obj: id) -> Self {
        todo!()
    }

    fn description(&self) -> NSString {
        let obj: id = unsafe { msg_send![self.obj, description] };
        unsafe { NSString::fromId(obj) }
    }

    fn debugDescription(&self) -> NSString {
        let obj: id = unsafe { msg_send![self.obj, debugDescription] };
        unsafe { NSString::fromId(obj) }
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

impl<K, V> t_NSDictionary<K, V> for NSDictionary<K, V> {
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

impl<K, V> Default for NSDictionary<K, V> {
    fn default() -> Self {
        Self::new()
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
