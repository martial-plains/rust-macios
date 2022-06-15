use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

use objc::{
    class, msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::traits::INSLocale,
    objective_c_runtime::{id, traits::PNSObject},
};

/// The directions that a language may take across a page of text.
#[repr(usize)]
#[derive(Debug)]
pub enum LanguageDirection {
    /// The direction of the language is unknown.
    Unknown = 0,
    /// The language direction is from left to right.
    LeftToRight = 1,
    /// The language direction is from right to left.
    RightToLeft = 2,
    /// The language direction is from top to bottom.
    TopToBottom = 3,
    /// The language direction is from bottom to top.
    BottomToTop = 4,
}

/// The `Locale` class provides information about the user’s locale and formatting preferences.
pub struct NSLocale {
    /// The raw pointer to the Objective-C object.
    pub obj: Id<Object>,
}

impl PNSObject for NSLocale {
    fn im_class<'a>() -> &'a Class {
        class!(NSLocale)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![self.obj, self] }
    }
}

impl INSLocale for NSLocale {}

impl Display for NSLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl Debug for NSLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Locale: {}", self.ip_debug_description())
    }
}

impl Clone for NSLocale {
    fn clone(&self) -> Self {
        NSLocale {
            obj: { unsafe { msg_send![self.obj, retain] } },
        }
    }
}

impl Deref for NSLocale {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.obj
    }
}

impl DerefMut for NSLocale {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.obj
    }
}

impl From<NSLocale> for id {
    /// Converts the `Locale` into an `Object`.
    fn from(mut val: NSLocale) -> Self {
        &mut *val.obj
    }
}

impl From<id> for NSLocale {
    /// Converts the `Object` into a `Locale`.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from(val: id) -> Self {
        NSLocale {
            obj: unsafe { Id::from_ptr(val) },
        }
    }
}
