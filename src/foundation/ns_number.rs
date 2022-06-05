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
    id,
    objective_c_runtime::traits::{FromId, INSValue, PNSObject, ToId},
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

    fn im_isEqual(&self, object: &Self) -> bool {
        unsafe {
            let isEqual = msg_send![self.obj, isEqual: object];
            to_bool(isEqual)
        }
    }

    fn ip_hash(&self) -> UInt {
        unsafe { msg_send![self.obj, hash] }
    }

    fn im_isKindOfClass(&self, aClass: Class) -> bool {
        unsafe {
            let isKindOfClass = msg_send![self.obj, isKindOfClass: aClass];
            to_bool(isKindOfClass)
        }
    }

    fn im_isMemberOfClass(&self, aClass: Class) -> bool {
        unsafe {
            let isMemberOfClass = msg_send![self.obj, isMemberOfClass: aClass];
            to_bool(isMemberOfClass)
        }
    }

    fn im_respondsToSelector(&self, aSelector: Sel) -> bool {
        unsafe {
            let respondsToSelector = msg_send![self.obj, respondsToSelector: aSelector];
            to_bool(respondsToSelector)
        }
    }

    fn im_conformsToProtocol(&self, aProtocol: Protocol) -> bool {
        unsafe {
            let conformsToProtocol = msg_send![self.obj, conformsToProtocol: aProtocol];
            to_bool(conformsToProtocol)
        }
    }

    fn ip_description(&self) -> NSString {
        unsafe {
            let description = msg_send![self.obj, description];
            NSString::from_id(description)
        }
    }

    fn ip_debugDescription(&self) -> NSString {
        unsafe {
            let debugDescription = msg_send![self.obj, debugDescription];
            NSString::from_id(debugDescription)
        }
    }

    fn im_performSelector(&self, aSelector: Sel) -> id {
        unsafe { msg_send![self.obj, performSelector: aSelector] }
    }

    fn im_performSelector_withObject(&self, aSelector: Sel, withObject: id) -> id {
        unsafe { msg_send![self.obj, performSelector: aSelector withObject: withObject] }
    }

    fn im_isProxy(&self) -> bool {
        unsafe {
            let isProxy = msg_send![self.obj, isProxy];
            to_bool(isProxy)
        }
    }
}

impl INSValue for NSNumber {}

impl INSNumber for NSNumber {
    fn tm_numberWithBool(value: bool) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithBool: value];
            NSNumber { obj }
        }
    }

    fn tm_numberWithChar(value: c_schar) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithChar: value];
            NSNumber { obj }
        }
    }

    fn tm_numberWithDouble(value: c_double) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithDouble: value];
            NSNumber { obj }
        }
    }

    fn tm_numberWithFloat(value: c_float) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithFloat: value];
            NSNumber { obj }
        }
    }

    fn tm_numberWithInt(value: c_int) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithInt: value];
            NSNumber { obj }
        }
    }

    fn tm_numberWithInteger(value: Int) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithInteger: value];
            NSNumber { obj }
        }
    }

    fn tm_numberWithLong(value: c_long) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithLong: value];
            NSNumber { obj }
        }
    }

    fn tm_numberWithLongLong(value: c_longlong) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithLongLong: value];
            NSNumber { obj }
        }
    }

    fn tm_numberWithShort(value: c_short) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithShort: value];
            NSNumber { obj }
        }
    }

    fn tm_numberWithUnsignedChar(value: c_uchar) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithUnsignedChar: value];
            NSNumber { obj }
        }
    }

    fn tm_numberWithUnsignedInt(value: c_uint) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithUnsignedInt: value];
            NSNumber { obj }
        }
    }

    fn tm_numberWithUnsignedInteger(value: UInt) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithUnsignedInteger: value];
            NSNumber { obj }
        }
    }

    fn tm_numberWithUnsignedLong(value: c_ulong) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithUnsignedLong: value];
            NSNumber { obj }
        }
    }

    fn tm_numberWithUnsignedLongLong(value: c_ulonglong) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithUnsignedLongLong: value];
            NSNumber { obj }
        }
    }

    fn tm_numberWithUnsignedShort(value: c_ushort) -> Self {
        unsafe {
            let obj = msg_send![class!(NSNumber), numberWithUnsignedShort: value];
            NSNumber { obj }
        }
    }

    fn im_initWithBool(&self, value: bool) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithBool: value];
            NSNumber { obj }
        }
    }

    fn im_initWithChar(&self, value: c_schar) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithChar: value];
            NSNumber { obj }
        }
    }

    fn im_initWithDouble(&self, value: c_double) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithDouble: value];
            NSNumber { obj }
        }
    }

    fn im_initWithFloat(&self, value: c_float) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithFloat: value];
            NSNumber { obj }
        }
    }

    fn im_initWithInt(&self, value: c_int) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithInt: value];
            NSNumber { obj }
        }
    }

    fn im_initWithInteger(&self, value: Int) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithInteger: value];
            NSNumber { obj }
        }
    }

    fn im_initWithLong(&self, value: c_long) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithLong: value];
            NSNumber { obj }
        }
    }

    fn im_initWithLongLong(&self, value: c_longlong) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithLongLong: value];
            NSNumber { obj }
        }
    }

    fn im_initWithShort(&self, value: c_short) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithShort: value];
            NSNumber { obj }
        }
    }

    fn im_initWithUnsignedChar(&self, value: c_uchar) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithUnsignedChar: value];
            NSNumber { obj }
        }
    }

    fn im_initWithUnsignedInt(&self, value: c_uint) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithUnsignedInt: value];
            NSNumber { obj }
        }
    }

    fn im_initWithUnsignedInteger(&self, value: c_uint) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithUnsignedInteger: value];
            NSNumber { obj }
        }
    }

    fn im_initWithUnsignedLong(&self, value: c_ulong) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithUnsignedLong: value];
            NSNumber { obj }
        }
    }

    fn im_initWithUnsignedLongLong(&self, value: c_ulonglong) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithUnsignedLongLong: value];
            NSNumber { obj }
        }
    }

    fn im_initWithUnsignedShort(&self, value: c_ushort) -> Self {
        unsafe {
            let obj: NSNumber = msg_send![self.obj, alloc];
            let obj = msg_send![obj, initWithUnsignedShort: value];
            NSNumber { obj }
        }
    }

    fn ip_boolValue(&self) -> bool {
        unsafe { msg_send![self.obj, boolValue] }
    }

    fn ip_charValue(&self) -> c_schar {
        unsafe { msg_send![self.obj, charValue] }
    }

    fn ip_doubleValue(&self) -> c_double {
        unsafe { msg_send![self.obj, doubleValue] }
    }

    fn ip_floatValue(&self) -> c_float {
        unsafe { msg_send![self.obj, floatValue] }
    }

    fn ip_intValue(&self) -> c_int {
        unsafe { msg_send![self.obj, intValue] }
    }

    fn ip_integerValue(&self) -> Int {
        unsafe { msg_send![self.obj, integerValue] }
    }

    fn ip_longLongValue(&self) -> c_longlong {
        unsafe { msg_send![self.obj, longLongValue] }
    }

    fn ip_longValue(&self) -> c_long {
        unsafe { msg_send![self.obj, longValue] }
    }

    fn ip_shortValue(&self) -> c_short {
        unsafe { msg_send![self.obj, shortValue] }
    }

    fn ip_unsignedCharValue(&self) -> c_uchar {
        unsafe { msg_send![self.obj, unsignedCharValue] }
    }

    fn ip_unsignedIntegerValue(&self) -> UInt {
        unsafe { msg_send![self.obj, unsignedIntegerValue] }
    }

    fn ip_unsignedIntValue(&self) -> c_uint {
        unsafe { msg_send![self.obj, unsignedIntValue] }
    }

    fn ip_unsignedLongLongValue(&self) -> c_ulonglong {
        unsafe { msg_send![self.obj, unsignedLongLongValue] }
    }

    fn ip_unsignedLongValue(&self) -> c_ulong {
        unsafe { msg_send![self.obj, unsignedLongValue] }
    }

    fn ip_unsignedShortValue(&self) -> c_ushort {
        unsafe { msg_send![self.obj, unsignedShortValue] }
    }

    fn im_descriptionWithLocale(&self, locale: NSLocale) -> NSString {
        unsafe {
            let description = msg_send![self.obj, descriptionWithLocale: locale.obj];
            NSString::from_id(description)
        }
    }

    fn ip_stringValue(&self) -> NSString {
        unsafe {
            let description = msg_send![self.obj, stringValue];
            NSString::from_id(description)
        }
    }

    fn im_compare(&self, other: &Self) -> NSComparisonResult {
        unsafe { msg_send![self.obj, compare: other] }
    }

    fn im_isEqualToNumber(&self, other: Self) -> bool {
        unsafe { msg_send![self.obj, isEqualToNumber: other] }
    }

    fn ip_decimalValue(&self) -> super::NSDecimal {
        unsafe { msg_send![self.obj, decimalValue] }
    }
}

impl PartialEq for NSNumber {
    fn eq(&self, other: &Self) -> bool {
        self.im_isEqualToNumber(other.clone())
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
        write!(f, "{}", self.ip_debugDescription())
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
        (self.ip_integerValue() + other.ip_integerValue()).into()
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
        NSNumber::tm_numberWithFloat(value)
    }
}

impl From<c_double> for NSNumber {
    fn from(value: c_double) -> Self {
        NSNumber::tm_numberWithDouble(value)
    }
}

impl From<c_schar> for NSNumber {
    fn from(value: c_schar) -> Self {
        NSNumber::tm_numberWithChar(value)
    }
}

impl From<c_uchar> for NSNumber {
    fn from(value: c_uchar) -> Self {
        NSNumber::tm_numberWithUnsignedChar(value)
    }
}

impl From<c_short> for NSNumber {
    fn from(value: c_short) -> Self {
        NSNumber::tm_numberWithShort(value)
    }
}

impl From<c_ushort> for NSNumber {
    fn from(value: c_ushort) -> Self {
        NSNumber::tm_numberWithUnsignedShort(value)
    }
}

impl From<c_int> for NSNumber {
    fn from(value: c_int) -> Self {
        NSNumber::tm_numberWithInt(value)
    }
}

impl From<c_uint> for NSNumber {
    fn from(value: c_uint) -> Self {
        NSNumber::tm_numberWithUnsignedInt(value)
    }
}

impl From<c_long> for NSNumber {
    fn from(value: c_long) -> Self {
        NSNumber::tm_numberWithLong(value)
    }
}

impl From<c_ulong> for NSNumber {
    fn from(value: c_ulong) -> Self {
        NSNumber::tm_numberWithUnsignedLong(value)
    }
}
