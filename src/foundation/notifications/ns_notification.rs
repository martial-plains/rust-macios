use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    foundation::{traits::t_NSNotification, String},
    id,
    objective_c_runtime::traits::t_NSObject,
};

/// A container for information broadcast through a notification center to all registered observers.
pub struct NSNotification {
    obj: Id<Object>,
}

impl t_NSObject for NSNotification {
    fn init() -> Self {
        let obj = unsafe { msg_send![class!(NSNotification), new] };
        Self { obj }
    }

    fn toId(mut self) -> id {
        &mut *self.obj
    }

    fn fromId(obj: id) -> Self {
        Self {
            obj: unsafe { Id::from_ptr(obj) },
        }
    }

    fn description(&self) -> String {
        unsafe {
            let description: id = msg_send![&*self.obj, description];
            String::fromId(description)
        }
    }

    fn debugDescription(&self) -> String {
        unsafe {
            let debug_description: id = msg_send![&*self.obj, debugDescription];
            String::fromId(debug_description)
        }
    }

    fn retain(&self) -> Self {
        unsafe { msg_send![&*self.obj, retain] }
    }
}

impl fmt::Debug for NSNotification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.debugDescription())
    }
}

impl fmt::Display for NSNotification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl t_NSNotification for NSNotification {
    fn init() -> Self {
        unsafe {
            NSNotification {
                obj: msg_send![class!(NSNotification), new],
            }
        }
    }
}

impl Deref for NSNotification {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.obj
    }
}

impl DerefMut for NSNotification {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.obj
    }
}
