use std::{
    fmt,
    ops::{Add, Deref, DerefMut},
};

use libc::{
    c_double, c_float, c_int, c_long, c_schar, c_short, c_uchar, c_uint, c_ulong, c_ushort,
};
use objc::{
    class, msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::traits::INSNumber,
    objective_c_runtime::{
        id,
        traits::{FromId, INSValue, PNSObject, ToId},
    },
};

/// An object wrapper for primitive scalar numeric values.
pub struct NSNumber {
    /// The underlying Objective-C object.
    pub obj: Id<Object>,
}

impl PNSObject for NSNumber {
    fn im_class<'a>() -> &'a Class {
        class!(NSNumber)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![self.obj, self] }
    }
}

impl INSValue for NSNumber {}

impl INSNumber for NSNumber {}

impl PartialEq for NSNumber {
    fn eq(&self, other: &Self) -> bool {
        self.im_is_equal_to_number(other.clone())
    }
}

impl Deref for NSNumber {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.obj
    }
}

impl DerefMut for NSNumber {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.obj
    }
}

impl fmt::Debug for NSNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl Clone for NSNumber {
    fn clone(&self) -> Self {
        unsafe {
            let obj = msg_send![self.obj, retain];
            NSNumber { obj }
        }
    }
}

impl Add for NSNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        (self.ip_integer_value() + other.ip_integer_value()).into()
    }
}

impl<T> FromIterator<T> for NSNumber
where
    T: Into<NSNumber>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut sum = NSNumber::from(0);
        for item in iter {
            sum = sum + item.into();
        }
        sum
    }
}

impl ToId for NSNumber {
    fn to_id(mut self) -> id {
        &mut *self.obj
    }
}

impl FromId for NSNumber {
    unsafe fn from_id(obj: id) -> Self {
        NSNumber {
            obj: Id::from_ptr(obj),
        }
    }
}

impl From<c_float> for NSNumber {
    fn from(value: c_float) -> Self {
        NSNumber::tm_number_with_float(value)
    }
}

impl From<c_double> for NSNumber {
    fn from(value: c_double) -> Self {
        NSNumber::tm_number_with_double(value)
    }
}

impl From<c_schar> for NSNumber {
    fn from(value: c_schar) -> Self {
        NSNumber::tm_number_with_char(value)
    }
}

impl From<c_uchar> for NSNumber {
    fn from(value: c_uchar) -> Self {
        NSNumber::tm_number_with_unsigned_char(value)
    }
}

impl From<c_short> for NSNumber {
    fn from(value: c_short) -> Self {
        NSNumber::tm_number_with_short(value)
    }
}

impl From<c_ushort> for NSNumber {
    fn from(value: c_ushort) -> Self {
        NSNumber::tm_number_with_unsigned_short(value)
    }
}

impl From<c_int> for NSNumber {
    fn from(value: c_int) -> Self {
        NSNumber::tm_number_with_int(value)
    }
}

impl From<c_uint> for NSNumber {
    fn from(value: c_uint) -> Self {
        NSNumber::tm_number_with_unsigned_int(value)
    }
}

impl From<c_long> for NSNumber {
    fn from(value: c_long) -> Self {
        NSNumber::tm_number_with_long(value)
    }
}

impl From<c_ulong> for NSNumber {
    fn from(value: c_ulong) -> Self {
        NSNumber::tm_number_with_unsigned_long(value)
    }
}
