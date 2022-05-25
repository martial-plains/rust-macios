use std::{
    fmt,
    ops::{Add, Deref, DerefMut},
};

use libc::{
    c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint, c_ulong,
    c_ulonglong, c_ushort,
};
use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    foundation::{traits::t_NSNumber, ComparisonResult, Int, NSLocale, NSString, UInt},
    objective_c_runtime::traits::{t_NSObject, t_NSValue},
};

/// An object wrapper for primitive scalar numeric values.
pub struct NSNumber {
    /// The underlying Objective-C object.
    pub obj: Id<Object>,
}

impl t_NSObject for NSNumber {
    fn init() -> Self {
        let obj = unsafe { msg_send![class!(NSNumber), new] };
        NSNumber { obj }
    }

    fn toId(mut self) -> crate::id {
        &mut *self.obj
    }

    unsafe fn fromId(obj: crate::id) -> Self {
        Self {
            obj: Id::from_ptr(obj),
        }
    }

    fn description(&self) -> NSString {
        unsafe {
            let description = msg_send![self.obj, description];
            NSString::fromId(description)
        }
    }

    fn debugDescription(&self) -> NSString {
        unsafe {
            let description = msg_send![self.obj, debugDescription];
            NSString::fromId(description)
        }
    }

    fn retain(&self) -> Self {
        unsafe {
            let obj = msg_send![self.obj, retain];
            Self { obj }
        }
    }
}

impl t_NSValue for NSNumber {}

impl t_NSNumber for NSNumber {
    fn numberWithBool(value: bool) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithBool: value];
            NSNumber { obj }
        }
    }

    fn numberWithChar(value: c_schar) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithChar: value];
            NSNumber { obj }
        }
    }

    fn numberWithDouble(value: c_double) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithDouble: value];
            NSNumber { obj }
        }
    }

    fn numberWithFloat(value: c_float) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithFloat: value];
            NSNumber { obj }
        }
    }

    fn numberWithInt(value: c_int) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithInt: value];
            NSNumber { obj }
        }
    }

    fn numberWithInteger(value: Int) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithInteger: value];
            NSNumber { obj }
        }
    }

    fn numberWithLong(value: c_long) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithLong: value];
            NSNumber { obj }
        }
    }

    fn numberWithLongLong(value: c_longlong) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithLongLong: value];
            NSNumber { obj }
        }
    }

    fn numberWithShort(value: c_short) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithShort: value];
            NSNumber { obj }
        }
    }

    fn numberWithUnsignedChar(value: c_uchar) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithUnsignedChar: value];
            NSNumber { obj }
        }
    }

    fn numberWithUnsignedInt(value: c_uint) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithUnsignedInt: value];
            NSNumber { obj }
        }
    }

    fn numberWithUnsignedInteger(value: UInt) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithUnsignedInteger: value];
            NSNumber { obj }
        }
    }

    fn numberWithUnsignedLong(value: c_ulong) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithUnsignedLong: value];
            NSNumber { obj }
        }
    }

    fn numberWithUnsignedLongLong(value: c_ulonglong) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithUnsignedLongLong: value];
            NSNumber { obj }
        }
    }

    fn numberWithUnsignedShort(value: c_ushort) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithUnsignedShort: value];
            NSNumber { obj }
        }
    }

    fn initWithBool(&self, value: bool) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithBool: value];
            NSNumber { obj }
        }
    }

    fn initWithChar(&self, value: c_schar) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithChar: value];
            NSNumber { obj }
        }
    }

    fn initWithDouble(&self, value: c_double) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithDouble: value];
            NSNumber { obj }
        }
    }

    fn initWithFloat(&self, value: c_float) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithFloat: value];
            NSNumber { obj }
        }
    }

    fn initWithInt(&self, value: c_int) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithInt: value];
            NSNumber { obj }
        }
    }

    fn initWithInteger(&self, value: Int) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithInteger: value];
            NSNumber { obj }
        }
    }

    fn initWithLong(&self, value: c_long) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithLong: value];
            NSNumber { obj }
        }
    }

    fn initWithLongLong(&self, value: c_longlong) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithLongLong: value];
            NSNumber { obj }
        }
    }

    fn initWithShort(&self, value: c_short) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithShort: value];
            NSNumber { obj }
        }
    }

    fn initWithUnsignedChar(&self, value: c_uchar) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithUnsignedChar: value];
            NSNumber { obj }
        }
    }

    fn initWithUnsignedInt(&self, value: c_uint) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithUnsignedInt: value];
            NSNumber { obj }
        }
    }

    fn initWithUnsignedInteger(&self, value: c_uint) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithUnsignedInteger: value];
            NSNumber { obj }
        }
    }

    fn initWithUnsignedLong(&self, value: c_ulong) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithUnsignedLong: value];
            NSNumber { obj }
        }
    }

    fn initWithUnsignedLongLong(&self, value: c_ulonglong) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithUnsignedLongLong: value];
            NSNumber { obj }
        }
    }

    fn initWithUnsignedShort(&self, value: c_ushort) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithUnsignedShort: value];
            NSNumber { obj }
        }
    }

    fn boolValue(&self) -> bool {
        unsafe { msg_send![self.obj, boolValue] }
    }

    fn charValue(&self) -> c_schar {
        unsafe { msg_send![self.obj, charValue] }
    }

    fn doubleValue(&self) -> c_double {
        unsafe { msg_send![self.obj, doubleValue] }
    }

    fn floatValue(&self) -> c_float {
        unsafe { msg_send![self.obj, floatValue] }
    }

    fn intValue(&self) -> c_int {
        unsafe { msg_send![self.obj, intValue] }
    }

    fn integerValue(&self) -> Int {
        unsafe { msg_send![self.obj, integerValue] }
    }

    fn longLongValue(&self) -> c_longlong {
        unsafe { msg_send![self.obj, longLongValue] }
    }

    fn longValue(&self) -> c_long {
        unsafe { msg_send![self.obj, longValue] }
    }

    fn shortValue(&self) -> c_short {
        unsafe { msg_send![self.obj, shortValue] }
    }

    fn unsignedCharValue(&self) -> c_uchar {
        unsafe { msg_send![self.obj, unsignedCharValue] }
    }

    fn unsignedIntegerValue(&self) -> UInt {
        unsafe { msg_send![self.obj, unsignedIntegerValue] }
    }

    fn unsignedIntValue(&self) -> c_uint {
        unsafe { msg_send![self.obj, unsignedIntValue] }
    }

    fn unsignedLongLongValue(&self) -> c_ulonglong {
        unsafe { msg_send![self.obj, unsignedLongLongValue] }
    }

    fn unsignedLongValue(&self) -> c_ulong {
        unsafe { msg_send![self.obj, unsignedLongValue] }
    }

    fn unsignedShortValue(&self) -> c_ushort {
        unsafe { msg_send![self.obj, unsignedShortValue] }
    }

    fn descriptionWithLocale(&self, locale: NSLocale) -> NSString {
        unsafe {
            let description = msg_send![self.obj, descriptionWithLocale: locale.obj];
            NSString::fromId(description)
        }
    }

    fn stringValue(&self) -> NSString {
        unsafe {
            let description = msg_send![self.obj, stringValue];
            NSString::fromId(description)
        }
    }

    fn compare(&self, other: &Self) -> ComparisonResult {
        unsafe { msg_send![self.obj, compare: other] }
    }

    fn isEqualToNumber(&self, other: &Self) -> bool {
        unsafe { msg_send![self.obj, isEqualToNumber: other] }
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
        write!(f, "{}", self.debugDescription())
    }
}

impl fmt::Display for NSNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
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
        (self.integerValue() + other.integerValue()).into()
    }
}

impl<T> FromIterator<T> for NSNumber
where
    T: Into<NSNumber>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut sum = NSNumber::init();
        for item in iter {
            sum = sum + item.into();
        }
        sum
    }
}

impl From<c_float> for NSNumber {
    fn from(value: c_float) -> Self {
        NSNumber::numberWithFloat(value)
    }
}

impl From<c_double> for NSNumber {
    fn from(value: c_double) -> Self {
        NSNumber::numberWithDouble(value)
    }
}

impl From<c_schar> for NSNumber {
    fn from(value: c_schar) -> Self {
        NSNumber::numberWithChar(value)
    }
}

impl From<c_uchar> for NSNumber {
    fn from(value: c_uchar) -> Self {
        NSNumber::numberWithUnsignedChar(value)
    }
}

impl From<c_short> for NSNumber {
    fn from(value: c_short) -> Self {
        NSNumber::numberWithShort(value)
    }
}

impl From<c_ushort> for NSNumber {
    fn from(value: c_ushort) -> Self {
        NSNumber::numberWithUnsignedShort(value)
    }
}

impl From<c_int> for NSNumber {
    fn from(value: c_int) -> Self {
        NSNumber::numberWithInt(value)
    }
}

impl From<c_uint> for NSNumber {
    fn from(value: c_uint) -> Self {
        NSNumber::numberWithUnsignedInt(value)
    }
}

impl From<c_long> for NSNumber {
    fn from(value: c_long) -> Self {
        NSNumber::numberWithLong(value)
    }
}

impl From<c_ulong> for NSNumber {
    fn from(value: c_ulong) -> Self {
        NSNumber::numberWithUnsignedLong(value)
    }
}
