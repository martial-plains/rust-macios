use std::ops::Add;

use libc::{
    c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint, c_ulong,
    c_ulonglong, c_ushort,
};
use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    objective_c_runtime::{
        macros::object,
        traits::{FromId, PNSObject},
        INSValue,
    },
    utils::to_bool,
};

use super::{Int, NSCoder, NSComparisonResult, NSDecimal, NSLocale, NSString, UInt};

object! {
    /// An object wrapper for primitive scalar numeric values.
    unsafe pub struct NSNumber;
}

impl INSValue for NSNumber {}

#[interface_impl(NSValue)]
impl NSNumber {
    /* Creating an NSNumber Object
     */

    /// Creates and returns an NSNumber object containing a given value, treating it as a BOOL.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn number_with_bool(value: bool) -> Self
    where
        Self: Sized + 'static + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), numberWithBool: value]) }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as a signed char.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn number_with_char(value: c_schar) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), numberWithChar: value]) }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it a double.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn number_with_double(value: c_double) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), numberWithDouble: value]) }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as a float.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn number_with_float(value: c_float) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), numberWithFloat: value]) }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as a signed int.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn number_with_int(value: c_int) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), numberWithInt: value]) }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as an NSInteger.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn number_with_integer(value: Int) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), numberWithInteger: value]) }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as a signed long.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn number_with_long(value: c_long) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), numberWithLong: value]) }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as a signed long long.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn number_with_long_long(value: c_longlong) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), numberWithLongLong: value]) }
    }
    /// Creates and returns an NSNumber object containing value, treating it as a signed short.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn number_with_short(value: c_short) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), numberWithShort: value]) }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned char.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn number_with_unsigned_char(value: c_uchar) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), numberWithUnsignedChar: value]) }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned int.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn number_with_unsigned_int(value: c_uint) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), numberWithUnsignedInt: value]) }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as an NSUInteger.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn number_with_unsigned_integer(value: UInt) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), numberWithUnsignedInteger: value]) }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned long.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn number_with_unsigned_long(value: c_ulong) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), numberWithUnsignedLong: value]) }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned long long.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn number_with_unsigned_long_long(value: c_ulonglong) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                Self::m_class(),
                numberWithUnsignedLongLong: value
            ])
        }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned short.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn number_with_unsigned_short(value: c_ushort) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), numberWithUnsignedShort: value]) }
    }

    /* Initializing an NSNumber Object
     */

    /// Returns an NSNumber object initialized to contain a given value, treated as a BOOL.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    #[method]
    pub fn init_with_bool(&mut self, value: bool) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithBool: value]) }
    }

    /// Returns an NSNumber object initialized to contain a given value, treated as a signed char.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn init_with_char(&mut self, value: c_schar) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithChar: value]) }
    }

    /// Returns an NSNumber object initialized to contain value, treated as a double.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn init_with_double(&mut self, value: c_double) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithDouble: value]) }
    }

    /// Returns an NSNumber object initialized to contain a given value, treated as a float.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn init_with_float(&mut self, value: c_float) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithFloat: value]) }
    }

    /// Returns an NSNumber object initialized to contain a given value, treated as a signed int.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn init_with_int(&mut self, value: c_int) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithInt: value]) }
    }

    /// Returns an NSNumber object initialized to contain a given value, treated as an NSInteger.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn init_with_integer(&mut self, value: Int) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithInteger: value]) }
    }

    /// Returns an NSNumber object initialized to contain a given value, treated as a signed long.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn init_with_long(&mut self, value: c_long) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithLong: value]) }
    }

    /// Returns an NSNumber object initialized to contain value, treated as a signed long long.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn init_with_long_long(&mut self, value: c_longlong) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithLongLong: value]) }
    }

    /// Returns an NSNumber object initialized to contain a given value, treated as a signed short.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn init_with_short(&mut self, value: c_short) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithShort: value]) }
    }

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned char.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn init_with_unsigned_char(&mut self, value: c_uchar) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithUnsignedChar: value]) }
    }

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned int.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn init_with_unsigned_int(&mut self, value: c_uint) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithUnsignedInt: value]) }
    }

    /// Returns an NSNumber object initialized to contain a given value, treated as an NSUInteger.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn init_with_unsigned_integer(&mut self, value: c_uint) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithUnsignedInteger: value]) }
    }

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned long.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn init_with_unsigned_long(&mut self, value: c_ulong) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithUnsignedLong: value]) }
    }

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned long long.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn init_with_unsigned_long_long(&mut self, value: c_ulonglong) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithUnsignedLongLong: value]) }
    }

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned short.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    #[method]
    pub fn init_with_unsigned_short(&mut self, value: c_ushort) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithUnsignedShort: value]) }
    }

    /* Accessing Numeric Values
     */

    /// The number object's value expressed as a Boolean value.
    #[property]
    pub fn bool_value(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), boolValue]) }
    }

    /// The number object's value expressed as a char.
    #[property]
    pub fn char_value(&self) -> c_schar {
        unsafe { msg_send![self.m_self(), charValue] }
    }

    /// The number object's value expressed as an NSDecimal structure.
    #[property]
    pub fn decimal_value(&self) -> NSDecimal {
        unsafe { NSDecimal::from_id(msg_send![self.m_self(), decimalValue]) }
    }

    /// The number object's value expressed as a double, converted as necessary.
    #[property]
    pub fn double_value(&self) -> c_double {
        unsafe { msg_send![self.m_self(), doubleValue] }
    }

    /// The number object's value expressed as a float, converted as necessary.
    #[property]
    pub fn float_value(&self) -> c_float {
        unsafe { msg_send![self.m_self(), floatValue] }
    }

    /// The number object's value expressed as an int, converted as necessary.
    #[property]
    pub fn int_value(&self) -> c_int {
        unsafe { msg_send![self.m_self(), intValue] }
    }

    /// The number object's value expressed as an NSInteger object, converted as necessary.
    #[property]
    pub fn integer_value(&self) -> Int {
        unsafe { msg_send![self.m_self(), integerValue] }
    }

    /// The number object’s value expressed as a long long, converted as necessary.
    #[property]
    pub fn long_long_value(&self) -> c_longlong {
        unsafe { msg_send![self.m_self(), longLongValue] }
    }

    /// The number object’s value expressed as a long, converted as necessary.
    #[property]
    pub fn long_value(&self) -> c_long {
        unsafe { msg_send![self.m_self(), longValue] }
    }

    /// The number object's value expressed as a short, converted as necessary.
    #[property]
    pub fn short_value(&self) -> c_short {
        unsafe { msg_send![self.m_self(), shortValue] }
    }

    /// The number object's value expressed as an unsigned char, converted as necessary.
    #[property]
    pub fn unsigned_char_value(&self) -> c_uchar {
        unsafe { msg_send![self.m_self(), unsignedCharValue] }
    }

    /// The number object's value expressed as an NSUInteger object, converted as necessary.
    #[property]
    pub fn unsigned_integer_value(&self) -> UInt {
        unsafe { msg_send![self.m_self(), unsignedIntegerValue] }
    }

    /// The number object's value expressed as an unsigned int, converted as necessary.
    #[property]
    pub fn unsigned_int_value(&self) -> c_uint {
        unsafe { msg_send![self.m_self(), unsignedIntValue] }
    }

    /// The number object’s value expressed as an unsigned long long, converted as necessary.
    #[property]
    pub fn unsigned_long_long_value(&self) -> c_ulonglong {
        unsafe { msg_send![self.m_self(), unsignedLongLongValue] }
    }

    /// The number object's value expressed as an unsigned long, converted as necessary.
    #[property]
    pub fn unsigned_long_value(&self) -> c_ulong {
        unsafe { msg_send![self.m_self(), unsignedLongValue] }
    }

    /// The number object's value expressed as an unsigned short, converted as necessary.
    #[property]
    pub fn unsigned_short_value(&self) -> c_ushort {
        unsafe { msg_send![self.m_self(), unsignedShortValue] }
    }

    /* Retrieving String Representations
     */

    /// Returns a string that represents the contents of the number object for a given locale.
    ///
    /// # Arguments
    ///
    /// * `locale` - The locale to use to format the number.
    ///
    /// # Returns
    ///
    /// A string that represents the contents of the number object formatted using the locale information in `locale`.
    #[method]
    pub fn description_with_locale(&self, locale: &NSLocale) -> NSString {
        unsafe {
            NSString::from_id(msg_send![self.m_self(), descriptionWithLocale: locale.m_self()])
        }
    }

    /// The number object's value expressed as a human-readable string.
    #[property]
    pub fn string_value(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), stringValue]) }
    }

    /* Comparing NSNumber Objects
     */

    /// Returns an NSComparisonResult value that indicates whether the number object’s value is greater than, equal to, or less than a given number.
    ///
    /// # Arguments
    ///
    /// * `other` - The number to compare to the number object’s value.
    #[method]
    pub fn compare(&self, other: &Self) -> NSComparisonResult {
        unsafe { msg_send![self.m_self(), compare: other.m_self()] }
    }

    /// Returns a Boolean value that indicates whether the number object’s value and a given number are equal.
    ///
    /// # Arguments
    ///
    /// * `other` - The number to compare to the number object’s value.
    #[method]
    pub fn is_equal_to_number(&self, other: &Self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), isEqualToNumber: other.m_self()]) }
    }

    /* Accessing Type Information
     */

    /// Returns a C string containing the Objective-C type of the data contained in the number object.
    ///
    /// # Return Value
    /// A C string containing the Objective-C type of the data contained in the number object,
    /// as encoded by the @encode() compiler directive.
    #[method]
    pub fn objc_type(&self) -> *const char {
        unsafe { msg_send![self.m_self(), objCType] }
    }

    /* Initializers */

    ///
    #[method]
    fn init_with_coder(&mut self, coder: &NSCoder) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), initWithCoder: coder.m_self()]) }
    }
}

impl Default for NSNumber {
    fn default() -> Self {
        Self::m_new()
    }
}

impl PartialEq for NSNumber {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal_to_number(other)
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
        let mut sum = NSNumber::from(0);
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
