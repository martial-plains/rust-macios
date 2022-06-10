use std::{
    fmt,
    ops::{Add, Deref, DerefMut},
};

use libc::{
    c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint, c_ulong,
    c_ulonglong, c_ushort,
};
use objc::{
    class, msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::{traits::INSNumber, Int, NSComparisonResult, NSLocale, NSString, UInt},
    objective_c_runtime::{
        id,
        traits::{FromId, INSValue, PNSObject, ToId},
    },
    utils::to_bool,
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

    fn im_is_equal(&self, object: &Self) -> bool {
        unsafe {
            let is_equal = msg_send![self.obj, isEqual: object];
            to_bool(is_equal)
        }
    }

    fn ip_hash(&self) -> UInt {
        unsafe { msg_send![self.obj, hash] }
    }

    fn im_is_kind_of_class(&self, class: Class) -> bool {
        unsafe {
            let is_kind_of_class = msg_send![self.obj, isKindOfClass: class];
            to_bool(is_kind_of_class)
        }
    }

    fn im_is_member_of_class(&self, class: Class) -> bool {
        unsafe {
            let is_member_of_class = msg_send![self.obj, isMemberOfClass: class];
            to_bool(is_member_of_class)
        }
    }

    fn im_responds_to_selector(&self, selector: Sel) -> bool {
        unsafe {
            let responds_to_selector = msg_send![self.obj, respondsToSelector: selector];
            to_bool(responds_to_selector)
        }
    }

    fn im_conforms_to_protocol(&self, protocol: Protocol) -> bool {
        unsafe {
            let conforms_to_protocol = msg_send![self.obj, conformsToProtocol: protocol];
            to_bool(conforms_to_protocol)
        }
    }

    fn ip_description(&self) -> NSString {
        unsafe {
            let description = msg_send![self.obj, description];
            NSString::from_id(description)
        }
    }

    fn ip_debug_description(&self) -> NSString {
        unsafe {
            let debug_description = msg_send![self.obj, debugDescription];
            NSString::from_id(debug_description)
        }
    }

    fn im_perform_selector(&self, selector: Sel) -> id {
        unsafe { msg_send![self.obj, performSelector: selector] }
    }

    fn im_perform_selector_with_object(&self, selector: Sel, with_object: id) -> id {
        unsafe { msg_send![self.obj, performSelector: selector withObject: with_object] }
    }

    fn im_is_proxy(&self) -> bool {
        unsafe {
            let is_proxy = msg_send![self.obj, isProxy];
            to_bool(is_proxy)
        }
    }
}

impl INSValue for NSNumber {}

impl INSNumber for NSNumber {
    fn tm_number_with_bool(value: bool) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithBool: value];
            NSNumber { obj }
        }
    }

    fn tm_number_with_char(value: c_schar) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithChar: value];
            NSNumber { obj }
        }
    }

    fn tm_number_with_double(value: c_double) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithDouble: value];
            NSNumber { obj }
        }
    }

    fn tm_number_with_float(value: c_float) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithFloat: value];
            NSNumber { obj }
        }
    }

    fn tm_number_with_int(value: c_int) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithInt: value];
            NSNumber { obj }
        }
    }

    fn tm_number_with_integer(value: Int) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithInteger: value];
            NSNumber { obj }
        }
    }

    fn tm_number_with_long(value: c_long) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithLong: value];
            NSNumber { obj }
        }
    }

    fn tm_number_with_long_long(value: c_longlong) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithLongLong: value];
            NSNumber { obj }
        }
    }

    fn tm_number_with_short(value: c_short) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithShort: value];
            NSNumber { obj }
        }
    }

    fn tm_number_with_unsigned_char(value: c_uchar) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithUnsignedChar: value];
            NSNumber { obj }
        }
    }

    fn tm_number_with_unsigned_int(value: c_uint) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithUnsignedInt: value];
            NSNumber { obj }
        }
    }

    fn tm_number_with_unsigned_integer(value: UInt) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithUnsignedInteger: value];
            NSNumber { obj }
        }
    }

    fn tm_number_with_unsigned_long(value: c_ulong) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithUnsignedLong: value];
            NSNumber { obj }
        }
    }

    fn tm_number_with_unsigned_long_long(value: c_ulonglong) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithUnsignedLongLong: value];
            NSNumber { obj }
        }
    }

    fn tm_number_with_unsigned_short(value: c_ushort) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithUnsignedShort: value];
            NSNumber { obj }
        }
    }

    fn im_init_with_bool(&self, value: bool) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithBool: value];
            NSNumber { obj }
        }
    }

    fn im_init_with_char(&self, value: c_schar) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithChar: value];
            NSNumber { obj }
        }
    }

    fn im_init_with_double(&self, value: c_double) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithDouble: value];
            NSNumber { obj }
        }
    }

    fn im_init_with_float(&self, value: c_float) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithFloat: value];
            NSNumber { obj }
        }
    }

    fn im_init_with_int(&self, value: c_int) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithInt: value];
            NSNumber { obj }
        }
    }

    fn im_init_with_integer(&self, value: Int) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithInteger: value];
            NSNumber { obj }
        }
    }

    fn im_init_with_long(&self, value: c_long) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithLong: value];
            NSNumber { obj }
        }
    }

    fn im_init_with_long_long(&self, value: c_longlong) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithLongLong: value];
            NSNumber { obj }
        }
    }

    fn im_init_with_short(&self, value: c_short) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithShort: value];
            NSNumber { obj }
        }
    }

    fn im_init_with_unsigned_char(&self, value: c_uchar) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithUnsignedChar: value];
            NSNumber { obj }
        }
    }

    fn im_init_with_unsigned_int(&self, value: c_uint) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithUnsignedInt: value];
            NSNumber { obj }
        }
    }

    fn im_init_with_unsigned_integer(&self, value: c_uint) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithUnsignedInteger: value];
            NSNumber { obj }
        }
    }

    fn im_init_with_unsigned_long(&self, value: c_ulong) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithUnsignedLong: value];
            NSNumber { obj }
        }
    }

    fn im_init_with_unsigned_long_long(&self, value: c_ulonglong) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithUnsignedLongLong: value];
            NSNumber { obj }
        }
    }

    fn im_init_with_unsigned_short(&self, value: c_ushort) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithUnsignedShort: value];
            NSNumber { obj }
        }
    }

    fn ip_bool_value(&self) -> bool {
        unsafe { msg_send![self.obj, boolValue] }
    }

    fn ip_char_value(&self) -> c_schar {
        unsafe { msg_send![self.obj, charValue] }
    }

    fn ip_double_value(&self) -> c_double {
        unsafe { msg_send![self.obj, doubleValue] }
    }

    fn ip_float_value(&self) -> c_float {
        unsafe { msg_send![self.obj, floatValue] }
    }

    fn ip_int_value(&self) -> c_int {
        unsafe { msg_send![self.obj, intValue] }
    }

    fn ip_integer_value(&self) -> Int {
        unsafe { msg_send![self.obj, integerValue] }
    }

    fn ip_long_long_value(&self) -> c_longlong {
        unsafe { msg_send![self.obj, longLongValue] }
    }

    fn ip_long_value(&self) -> c_long {
        unsafe { msg_send![self.obj, longValue] }
    }

    fn ip_short_value(&self) -> c_short {
        unsafe { msg_send![self.obj, shortValue] }
    }

    fn ip_unsigned_char_value(&self) -> c_uchar {
        unsafe { msg_send![self.obj, unsignedCharValue] }
    }

    fn ip_unsigned_integer_value(&self) -> UInt {
        unsafe { msg_send![self.obj, unsignedIntegerValue] }
    }

    fn ip_unsigned_int_value(&self) -> c_uint {
        unsafe { msg_send![self.obj, unsignedIntValue] }
    }

    fn ip_unsigned_long_long_value(&self) -> c_ulonglong {
        unsafe { msg_send![self.obj, unsignedLongLongValue] }
    }

    fn ip_unsigned_long_value(&self) -> c_ulong {
        unsafe { msg_send![self.obj, unsignedLongValue] }
    }

    fn ip_unsigned_short_value(&self) -> c_ushort {
        unsafe { msg_send![self.obj, unsignedShortValue] }
    }

    fn im_description_with_locale(&self, locale: NSLocale) -> NSString {
        unsafe {
            let description = msg_send![self.obj, descriptionWithLocale: locale.obj];
            NSString::from_id(description)
        }
    }

    fn ip_string_value(&self) -> NSString {
        unsafe {
            let description = msg_send![self.obj, stringValue];
            NSString::from_id(description)
        }
    }

    fn im_compare(&self, other: &Self) -> NSComparisonResult {
        unsafe { msg_send![self.obj, compare: other] }
    }

    fn im_is_equal_to_number(&self, other: Self) -> bool {
        unsafe { msg_send![self.obj, isEqualToNumber: other] }
    }

    fn ip_decimal_value(&self) -> super::NSDecimal {
        unsafe { msg_send![self.obj, decimalValue] }
    }
}

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
