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
    foundation::{ComparisonResult, Locale, String},
    id,
    objective_c_runtime::NSObject,
};

use super::{Int, UInt};

/// The group of methods that are used with `NSNumber` objects.
pub trait NSNumberable {
    /* Creating an NSNumber Object
     */

    /// Creates and returns an NSNumber object containing a given value, treating it as a BOOL.
    fn number_with_bool(value: bool) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as a signed char.
    fn number_with_char(value: c_schar) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as a double.
    fn number_with_double(value: c_double) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as a float.
    fn number_with_float(value: c_float) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as a signed int.
    fn number_with_int(value: c_int) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an NSInteger.
    fn number_with_integer(value: Int) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as a signed long.
    fn number_with_long(value: c_long) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as a signed long long.
    fn number_with_long_long(value: c_longlong) -> Self;

    /// Creates and returns an NSNumber object containing value, treating it as a signed short.
    fn number_with_short(value: c_short) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned char.
    fn number_with_unsigned_char(value: c_uchar) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned int.
    fn number_with_unsigned_int(value: c_uint) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an NSUInteger.
    fn number_with_unsigned_integer(value: UInt) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned long.
    fn number_with_unsigned_long(value: c_ulong) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned long long.
    fn number_with_unsigned_long_long(value: c_ulonglong) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned short.
    fn number_with_unsigned_short(value: c_ushort) -> Self;

    /* Initializing an NSNumber Object
     */

    /// Returns an NSNumber object initialized to contain a given value, treated as a BOOL.
    fn init_with_bool(&self, value: bool) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as a signed char.
    fn init_with_char(&self, value: c_schar) -> Self;

    /// Returns an NSNumber object initialized to contain value, treated as a double.
    fn init_with_double(&self, value: c_double) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as a float.
    fn init_with_float(&self, value: c_float) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as a signed int.
    fn init_with_int(&self, value: c_int) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an NSInteger.
    fn init_with_integer(&self, value: Int) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as a signed long.
    fn init_with_long(&self, value: c_long) -> Self;

    /// Returns an NSNumber object initialized to contain value, treated as a signed long long.
    fn init_with_long_long(&self, value: c_longlong) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as a signed short.
    fn init_with_short(&self, value: c_short) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned char.
    fn init_with_unsigned_char(&self, value: c_uchar) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned int.
    fn init_with_unsigned_int(&self, value: c_uint) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an NSUInteger.
    fn init_with_unsigned_integer(&self, value: c_uint) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned long.
    fn init_with_unsigned_long(&self, value: c_ulong) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned long long.
    fn init_with_unsigned_long_long(&self, value: c_ulonglong) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned short.
    fn init_with_unsigned_short(&self, value: c_ushort) -> Self;

    /* Accessing Numeric Values
     */

    /// The number object's value expressed as a Boolean value.
    fn bool_value(&self) -> bool;

    /// The number object's value expressed as a char.
    fn char_value(&self) -> c_schar;

    /// The number object's value expressed as an NSDecimal structure.
    //// fn decimal_value(&self) -> NSDecimal;

    /// The number object's value expressed as a double, converted as necessary.
    fn double_value(&self) -> c_double;

    /// The number object's value expressed as a float, converted as necessary.
    fn float_value(&self) -> c_float;

    /// The number object's value expressed as an int, converted as necessary.
    fn int_value(&self) -> c_int;

    /// The number object's value expressed as an NSInteger object, converted as necessary.
    fn integer_value(&self) -> Int;

    /// The number object’s value expressed as a long long, converted as necessary.
    fn long_long_value(&self) -> c_longlong;

    /// The number object’s value expressed as a long, converted as necessary.
    fn long_value(&self) -> c_long;

    /// The number object's value expressed as a short, converted as necessary.
    fn short_value(&self) -> c_short;

    /// The number object's value expressed as an unsigned char, converted as necessary.
    fn unsigned_char_value(&self) -> c_uchar;

    /// The number object's value expressed as an NSUInteger object, converted as necessary.
    fn unsigned_integer_value(&self) -> UInt;

    /// The number object's value expressed as an unsigned int, converted as necessary.
    fn unsigned_int_value(&self) -> c_uint;

    /// The number object’s value expressed as an unsigned long long, converted as necessary.
    fn unsigned_long_long_value(&self) -> c_ulonglong;

    /// The number object's value expressed as an unsigned long, converted as necessary.
    fn unsigned_long_value(&self) -> c_ulong;

    /// The number object's value expressed as an unsigned short, converted as necessary.
    fn unsigned_short_value(&self) -> c_ushort;

    /* Retrieving String Representations
     */

    /// Returns a string that represents the contents of the number object for a given locale.
    fn description_with_locale(&self, locale: Locale) -> String;

    /// The number object's value expressed as a human-readable string.
    fn string_value(&self) -> String;

    /* Comparing NSNumber Objects
     */

    /// Returns an NSComparisonResult value that indicates whether the number object’s value is greater than, equal to, or less than a given number.
    fn compare(&self, other: &Self) -> ComparisonResult;

    /// Returns a Boolean value that indicates whether the number object’s value and a given number are equal.    
    fn is_equal_to_number(&self, other: &Self) -> bool;
}

/// An object wrapper for primitive scalar numeric values.
pub struct NSNumber {
    /// The underlying Objective-C object.
    pub obj: Id<Object>,
}

impl NSNumberable for NSNumber {
    fn number_with_bool(value: bool) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithBool: value];
            NSNumber { obj }
        }
    }

    fn number_with_char(value: c_schar) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithChar: value];
            NSNumber { obj }
        }
    }

    fn number_with_double(value: c_double) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithDouble: value];
            NSNumber { obj }
        }
    }

    fn number_with_float(value: c_float) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithFloat: value];
            NSNumber { obj }
        }
    }

    fn number_with_int(value: c_int) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithInt: value];
            NSNumber { obj }
        }
    }

    fn number_with_integer(value: Int) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithInteger: value];
            NSNumber { obj }
        }
    }

    fn number_with_long(value: c_long) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithLong: value];
            NSNumber { obj }
        }
    }

    fn number_with_long_long(value: c_longlong) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithLongLong: value];
            NSNumber { obj }
        }
    }

    fn number_with_short(value: c_short) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithShort: value];
            NSNumber { obj }
        }
    }

    fn number_with_unsigned_char(value: c_uchar) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithUnsignedChar: value];
            NSNumber { obj }
        }
    }

    fn number_with_unsigned_int(value: c_uint) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithUnsignedInt: value];
            NSNumber { obj }
        }
    }

    fn number_with_unsigned_integer(value: UInt) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithUnsignedInteger: value];
            NSNumber { obj }
        }
    }

    fn number_with_unsigned_long(value: c_ulong) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithUnsignedLong: value];
            NSNumber { obj }
        }
    }

    fn number_with_unsigned_long_long(value: c_ulonglong) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithUnsignedLongLong: value];
            NSNumber { obj }
        }
    }

    fn number_with_unsigned_short(value: c_ushort) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithUnsignedShort: value];
            NSNumber { obj }
        }
    }

    fn init_with_bool(&self, value: bool) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithBool: value];
            NSNumber { obj }
        }
    }

    fn init_with_char(&self, value: c_schar) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithChar: value];
            NSNumber { obj }
        }
    }

    fn init_with_double(&self, value: c_double) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithDouble: value];
            NSNumber { obj }
        }
    }

    fn init_with_float(&self, value: c_float) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithFloat: value];
            NSNumber { obj }
        }
    }

    fn init_with_int(&self, value: c_int) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithInt: value];
            NSNumber { obj }
        }
    }

    fn init_with_integer(&self, value: Int) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithInteger: value];
            NSNumber { obj }
        }
    }

    fn init_with_long(&self, value: c_long) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithLong: value];
            NSNumber { obj }
        }
    }

    fn init_with_long_long(&self, value: c_longlong) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithLongLong: value];
            NSNumber { obj }
        }
    }

    fn init_with_short(&self, value: c_short) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithShort: value];
            NSNumber { obj }
        }
    }

    fn init_with_unsigned_char(&self, value: c_uchar) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithUnsignedChar: value];
            NSNumber { obj }
        }
    }

    fn init_with_unsigned_int(&self, value: c_uint) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithUnsignedInt: value];
            NSNumber { obj }
        }
    }

    fn init_with_unsigned_integer(&self, value: c_uint) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithUnsignedInteger: value];
            NSNumber { obj }
        }
    }

    fn init_with_unsigned_long(&self, value: c_ulong) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithUnsignedLong: value];
            NSNumber { obj }
        }
    }

    fn init_with_unsigned_long_long(&self, value: c_ulonglong) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithUnsignedLongLong: value];
            NSNumber { obj }
        }
    }

    fn init_with_unsigned_short(&self, value: c_ushort) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithUnsignedShort: value];
            NSNumber { obj }
        }
    }

    fn bool_value(&self) -> bool {
        unsafe { msg_send![self.obj, boolValue] }
    }

    fn char_value(&self) -> c_schar {
        unsafe { msg_send![self.obj, charValue] }
    }

    fn double_value(&self) -> c_double {
        unsafe { msg_send![self.obj, doubleValue] }
    }

    fn float_value(&self) -> c_float {
        unsafe { msg_send![self.obj, floatValue] }
    }

    fn int_value(&self) -> c_int {
        unsafe { msg_send![self.obj, intValue] }
    }

    fn integer_value(&self) -> Int {
        unsafe { msg_send![self.obj, integerValue] }
    }

    fn long_long_value(&self) -> c_longlong {
        unsafe { msg_send![self.obj, longLongValue] }
    }

    fn long_value(&self) -> c_long {
        unsafe { msg_send![self.obj, longValue] }
    }

    fn short_value(&self) -> c_short {
        unsafe { msg_send![self.obj, shortValue] }
    }

    fn unsigned_char_value(&self) -> c_uchar {
        unsafe { msg_send![self.obj, unsignedCharValue] }
    }

    fn unsigned_integer_value(&self) -> UInt {
        unsafe { msg_send![self.obj, unsignedIntegerValue] }
    }

    fn unsigned_int_value(&self) -> c_uint {
        unsafe { msg_send![self.obj, unsignedIntValue] }
    }

    fn unsigned_long_long_value(&self) -> c_ulonglong {
        unsafe { msg_send![self.obj, unsignedLongLongValue] }
    }

    fn unsigned_long_value(&self) -> c_ulong {
        unsafe { msg_send![self.obj, unsignedLongValue] }
    }

    fn unsigned_short_value(&self) -> c_ushort {
        unsafe { msg_send![self.obj, unsignedShortValue] }
    }

    fn description_with_locale(&self, locale: Locale) -> String {
        unsafe {
            let description = msg_send![self.obj, descriptionWithLocale: locale.obj];
            String::from_id(description)
        }
    }

    fn string_value(&self) -> String {
        unsafe {
            let description = msg_send![self.obj, stringValue];
            String::from_id(description)
        }
    }

    fn compare(&self, other: &Self) -> ComparisonResult {
        unsafe { msg_send![self.obj, compare: other] }
    }

    fn is_equal_to_number(&self, other: &Self) -> bool {
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

impl NSObject for NSNumber {
    fn init() -> Self {
        unsafe {
            let obj: NSNumber = msg_send![class!(NSNumber), alloc];
            let obj = msg_send![obj, init];
            NSNumber { obj }
        }
    }

    fn to_id(mut self) -> id {
        &mut *self.obj
    }

    fn from_id(obj: id) -> Self {
        NSNumber {
            obj: unsafe { Id::from_ptr(obj) },
        }
    }

    fn description(&self) -> String {
        unsafe {
            let description = msg_send![self.obj, description];
            String::from_id(description)
        }
    }

    fn debug_description(&self) -> String {
        unsafe {
            let description = msg_send![self.obj, debugDescription];
            String::from_id(description)
        }
    }

    fn retain(&self) -> Self {
        unsafe { msg_send![self.obj, retain] }
    }
}

impl fmt::Debug for NSNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.debug_description())
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
        (self.integer_value() + other.integer_value()).into()
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
        NSNumber::number_with_float(value)
    }
}

impl From<c_double> for NSNumber {
    fn from(value: c_double) -> Self {
        NSNumber::number_with_double(value)
    }
}

impl From<c_schar> for NSNumber {
    fn from(value: c_schar) -> Self {
        NSNumber::number_with_char(value)
    }
}

impl From<c_uchar> for NSNumber {
    fn from(value: c_uchar) -> Self {
        NSNumber::number_with_unsigned_char(value)
    }
}

impl From<c_short> for NSNumber {
    fn from(value: c_short) -> Self {
        NSNumber::number_with_short(value)
    }
}

impl From<c_ushort> for NSNumber {
    fn from(value: c_ushort) -> Self {
        NSNumber::number_with_unsigned_short(value)
    }
}

impl From<c_int> for NSNumber {
    fn from(value: c_int) -> Self {
        NSNumber::number_with_int(value)
    }
}

impl From<c_uint> for NSNumber {
    fn from(value: c_uint) -> Self {
        NSNumber::number_with_unsigned_int(value)
    }
}

impl From<c_long> for NSNumber {
    fn from(value: c_long) -> Self {
        NSNumber::number_with_long(value)
    }
}

impl From<c_ulong> for NSNumber {
    fn from(value: c_ulong) -> Self {
        NSNumber::number_with_unsigned_long(value)
    }
}
