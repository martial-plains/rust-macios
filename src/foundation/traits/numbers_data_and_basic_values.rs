use std::sync::Arc;

use libc::{
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint,
    c_ulong, c_ulonglong, c_ushort, c_void,
};
use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::{
        Int, NSComparisonResult, NSData, NSDecimal, NSDecimalNumber, NSLocale, NSRoundingMode,
        NSString, UInt,
    },
    objective_c_runtime::traits::{FromId, INSValue, PNSObject},
};

use super::{INSLocale, INSString};

/// The group of methods that are used with `NSNumber` objects.
pub trait INSNumber: INSValue {
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
    fn tm_number_with_bool(value: bool) -> Self
    where
        Self: Sized + 'static + FromId,
    {
        unsafe { msg_send![Self::im_class(), numberWithBool: value] }
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
    fn tm_number_with_char(value: c_schar) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), numberWithChar: value]) }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as a double.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn tm_number_with_double(value: c_double) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), numberWithDouble: value]) }
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
    fn tm_number_with_float(value: c_float) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), numberWithFloat: value]) }
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
    fn tm_number_with_int(value: c_int) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), numberWithInt: value]) }
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
    fn tm_number_with_integer(value: Int) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), numberWithInteger: value]) }
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
    fn tm_number_with_long(value: c_long) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), numberWithLong: value]) }
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
    fn tm_number_with_long_long(value: c_longlong) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), numberWithLongLong: value]) }
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
    fn tm_number_with_short(value: c_short) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), numberWithShort: value]) }
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
    fn tm_number_with_unsigned_char(value: c_uchar) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), numberWithUnsignedChar: value]) }
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
    fn tm_number_with_unsigned_int(value: c_uint) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), numberWithUnsignedInt: value]) }
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
    fn tm_number_with_unsigned_integer(value: UInt) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                Self::im_class(),
                numberWithUnsignedInteger: value
            ])
        }
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
    fn tm_number_with_unsigned_long(value: c_ulong) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), numberWithUnsignedLong: value]) }
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
    fn tm_number_with_unsigned_long_long(value: c_ulonglong) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                Self::im_class(),
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
    fn tm_number_with_unsigned_short(value: c_ushort) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), numberWithUnsignedShort: value]) }
    }

    /* Initializing an NSNumber Object
     */

    /// Returns an NSNumber object initialized to contain a given value, treated as a BOOL.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    fn im_init_with_bool(&self, value: bool) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.im_self(), initWithBool: value]) }
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
    fn im_init_with_char(&self, value: c_schar) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.im_self(), initWithChar: value]) }
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
    fn im_init_with_double(&self, value: c_double) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.im_self(), initWithDouble: value]) }
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
    fn im_init_with_float(&self, value: c_float) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.im_self(), initWithFloat: value]) }
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
    fn im_init_with_int(&self, value: c_int) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.im_self(), initWithInt: value]) }
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
    fn im_init_with_integer(&self, value: Int) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.im_self(), initWithInteger: value]) }
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
    fn im_init_with_long(&self, value: c_long) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.im_self(), initWithLong: value]) }
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
    fn im_init_with_long_long(&self, value: c_longlong) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.im_self(), initWithLongLong: value]) }
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
    fn im_init_with_short(&self, value: c_short) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.im_self(), initWithShort: value]) }
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
    fn im_init_with_unsigned_char(&self, value: c_uchar) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.im_self(), initWithUnsignedChar: value]) }
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
    fn im_init_with_unsigned_int(&self, value: c_uint) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.im_self(), initWithUnsignedInt: value]) }
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
    fn im_init_with_unsigned_integer(&self, value: c_uint) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.im_self(), initWithUnsignedInteger: value]) }
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
    fn im_init_with_unsigned_long(&self, value: c_ulong) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.im_self(), initWithUnsignedLong: value]) }
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
    fn im_init_with_unsigned_long_long(&self, value: c_ulonglong) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.im_self(), initWithUnsignedLongLong: value]) }
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
    fn im_init_with_unsigned_short(&self, value: c_ushort) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.im_self(), initWithUnsignedShort: value]) }
    }

    /* Accessing Numeric Values
     */

    /// The number object's value expressed as a Boolean value.
    fn ip_bool_value(&self) -> bool
    where
        Self: Sized + FromId,
    {
        unsafe { msg_send![self.im_self(), boolValue] }
    }

    /// The number object's value expressed as a char.
    fn ip_char_value(&self) -> c_schar
    where
        Self: Sized + FromId,
    {
        unsafe { msg_send![self.im_self(), charValue] }
    }

    /// The number object's value expressed as an NSDecimal structure.
    fn ip_decimal_value(&self) -> NSDecimal
    where
        Self: Sized + FromId,
    {
        unsafe { msg_send![self.im_self(), decimalValue] }
    }

    /// The number object's value expressed as a double, converted as necessary.
    fn ip_double_value(&self) -> c_double
    where
        Self: Sized + FromId,
    {
        unsafe { msg_send![self.im_self(), doubleValue] }
    }

    /// The number object's value expressed as a float, converted as necessary.
    fn ip_float_value(&self) -> c_float
    where
        Self: Sized + FromId,
    {
        unsafe { msg_send![self.im_self(), floatValue] }
    }

    /// The number object's value expressed as an int, converted as necessary.
    fn ip_int_value(&self) -> c_int
    where
        Self: Sized + FromId,
    {
        unsafe { msg_send![self.im_self(), intValue] }
    }

    /// The number object's value expressed as an NSInteger object, converted as necessary.
    fn ip_integer_value(&self) -> Int
    where
        Self: Sized + FromId,
    {
        unsafe { msg_send![self.im_self(), integerValue] }
    }

    /// The number object’s value expressed as a long long, converted as necessary.
    fn ip_long_long_value(&self) -> c_longlong
    where
        Self: Sized + FromId,
    {
        unsafe { msg_send![self.im_self(), longLongValue] }
    }

    /// The number object’s value expressed as a long, converted as necessary.
    fn ip_long_value(&self) -> c_long
    where
        Self: Sized + FromId,
    {
        unsafe { msg_send![self.im_self(), longValue] }
    }

    /// The number object's value expressed as a short, converted as necessary.
    fn ip_short_value(&self) -> c_short
    where
        Self: Sized + FromId,
    {
        unsafe { msg_send![self.im_self(), shortValue] }
    }

    /// The number object's value expressed as an unsigned char, converted as necessary.
    fn ip_unsigned_char_value(&self) -> c_uchar
    where
        Self: Sized + FromId,
    {
        unsafe { msg_send![self.im_self(), unsignedCharValue] }
    }

    /// The number object's value expressed as an NSUInteger object, converted as necessary.
    fn ip_unsigned_integer_value(&self) -> UInt
    where
        Self: Sized + FromId,
    {
        unsafe { msg_send![self.im_self(), unsignedIntegerValue] }
    }

    /// The number object's value expressed as an unsigned int, converted as necessary.
    fn ip_unsigned_int_value(&self) -> c_uint
    where
        Self: Sized + FromId,
    {
        unsafe { msg_send![self.im_self(), unsignedIntValue] }
    }

    /// The number object’s value expressed as an unsigned long long, converted as necessary.
    fn ip_unsigned_long_long_value(&self) -> c_ulonglong
    where
        Self: Sized + FromId,
    {
        unsafe { msg_send![self.im_self(), unsignedLongLongValue] }
    }

    /// The number object's value expressed as an unsigned long, converted as necessary.
    fn ip_unsigned_long_value(&self) -> c_ulong
    where
        Self: Sized + FromId,
    {
        unsafe { msg_send![self.im_self(), unsignedLongValue] }
    }

    /// The number object's value expressed as an unsigned short, converted as necessary.
    fn ip_unsigned_short_value(&self) -> c_ushort
    where
        Self: Sized + FromId,
    {
        unsafe { msg_send![self.im_self(), unsignedShortValue] }
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
    fn im_description_with_locale(&self, locale: NSLocale) -> NSString
    where
        Self: Sized + FromId,
    {
        unsafe { msg_send![self.im_self(), descriptionWithLocale: locale.im_self()] }
    }

    /// The number object's value expressed as a human-readable string.
    fn ip_string_value(&self) -> NSString
    where
        Self: Sized + FromId,
    {
        unsafe { msg_send![self.im_self(), stringValue] }
    }

    /* Comparing NSNumber Objects
     */

    /// Returns an NSComparisonResult value that indicates whether the number object’s value is greater than, equal to, or less than a given number.
    ///
    /// # Arguments
    ///
    /// * `other` - The number to compare to the number object’s value.
    fn im_compare(&self, other: &Self) -> NSComparisonResult
    where
        Self: Sized + FromId,
    {
        unsafe { msg_send![self.im_self(), compare: other.im_self()] }
    }

    /// Returns a Boolean value that indicates whether the number object’s value and a given number are equal.
    ///
    /// # Arguments
    ///
    /// * `other` - The number to compare to the number object’s value.
    fn im_is_equal_to_number(&self, other: Self) -> bool
    where
        Self: Sized + FromId,
    {
        unsafe { msg_send![self.im_self(), isEqualToNumber: other.im_self()] }
    }
}

/// An object for representing and performing arithmetic on base-10 numbers.
pub trait INSDecimalNumber: INSNumber {
    /* Creating a Decimal Number
     */

    /// Creates and returns a decimal number equivalent to a given decimal structure.
    fn tm_decimal_number_with_decimal(decimal: NSDecimalNumber) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                Self::im_class(),
                decimalNumberWithDecimal: decimal
            ])
        }
    }

    /// Creates and returns a decimal number equivalent to the number specified by the arguments.
    fn tm_decimal_number_with_mantissa(
        mantissa: c_ulonglong,
        exponent: c_short,
        is_negative: bool,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                Self::im_class(),
                decimalNumberWithMantissa: mantissa
                exponent: exponent
                isNegative: is_negative
            ])
        }
    }

    /// Creates a decimal number whose value is equivalent to that in a given numeric string.
    fn tm_decimal_number_with_string(string: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), decimalNumberWithString: string]) }
    }

    /// Creates a decimal number whose value is equivalent to that in a given numeric string, interpreted using a given locale.
    fn tm_decimal_number_with_string_locale(string: NSString, locale: NSLocale) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![Self::im_class(), decimalNumberWithString:string locale:locale])
        }
    }

    /// A decimal number equivalent to the number 1.0.
    fn tp_one() -> Self
    where
        Self: Sized + 'static + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), one]) }
    }

    /// A decimal number equivalent to the number 0.0.
    fn tp_zero() -> Self
    where
        Self: Sized + 'static + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), zero]) }
    }

    /// A decimal number that specifies no number.
    fn tp_not_a_number() -> Self
    where
        Self: Sized + 'static + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), notANumber]) }
    }

    /* Initializing a Decimal Number
     */

    /// Initializes a decimal number to represent a given decimal.
    fn im_init_with_decimal(&mut self, decimal: NSDecimalNumber) {
        unsafe { msg_send![self.im_self(), initWithDecimal: decimal] }
    }

    /// Initializes a decimal number using the given mantissa, exponent, and sign.
    fn im_init_with_mantissa_exponent_is_negative(
        &mut self,
        mantissa: c_ulonglong,
        exponent: c_short,
        is_negative: bool,
    ) {
        unsafe {
            msg_send![self.im_self(), initWithMantissa: mantissa
                exponent: exponent
                isNegative: is_negative]
        }
    }

    /// Initializes a decimal number so that its value is equivalent to that in a given numeric string.
    fn im_init_with_string<S>(&mut self, string: S)
    where
        S: INSString,
    {
        unsafe { msg_send![self.im_self(), initWithString: string] }
    }

    /// Initializes a decimal number so that its value is equivalent to that in a given numeric string, interpreted using a given locale.
    fn im_init_with_string_locale<S, L>(&mut self, string: S, locale: L)
    where
        S: INSString,
        L: INSLocale,
    {
        unsafe { msg_send![self.im_self(), initWithString: string locale: locale] }
    }

    /* Performing Arithmetic
     */

    /// Adds this number to another given number.
    fn im_decimal_number_by_adding(&self, decimal_number: Self) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.im_self(),
                decimalNumberByAdding: decimal_number
            ])
        }
    }

    /// Subtracts another given number from this one.
    fn im_decimal_number_by_subtracting(&self, decimal_number: Self) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.im_self(),
                decimalNumberBySubtracting: decimal_number
            ])
        }
    }

    /// Multiplies the number by another given number.
    fn im_decimal_number_by_multiplying_by(&self, decimal_number: Self) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.im_self(),
                decimalNumberByMultiplyingBy: decimal_number
            ])
        }
    }

    /// Divides the number by another given number.
    fn im_decimal_number_by_dividing_by(&self, decimal_number: Self) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.im_self(),
                decimalNumberByDividingBy: decimal_number
            ])
        }
    }

    /// Raises the number to a given power.
    fn im_decimal_number_by_raising_to_power(&self, power: c_uint) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.im_self(),
                decimalNumberByRaisingToPower: power
            ])
        }
    }

    /// Multiplies the number by 10 raised to the given power.
    fn im_decimal_number_by_multiplying_by_power_of_10(&self, power: c_short) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.im_self(),
                decimalNumberByMultiplyingByPowerOf10: power
            ])
        }
    }

    /// Adds this number to another given number using the specified behavior.
    fn im_decimal_number_by_adding_with_behavior(
        &self,
        decimal_number: &Self,
        with_behavior: Arc<dyn PNSDecimalNumberBehaviors>,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.im_self(),
                decimalNumberByAdding: decimal_number
                withBehavior: with_behavior
            ])
        }
    }

    /// Subtracts this a given number from this one using the specified behavior.
    fn im_decimal_number_by_subtracting_with_behavior(
        &self,
        decimal_number: &Self,
        with_behavior: Arc<dyn PNSDecimalNumberBehaviors>,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.im_self(),
                decimalNumberBySubtracting: decimal_number
                withBehavior: with_behavior
            ])
        }
    }

    /// Multiplies this number by another given number using the specified behavior.
    fn im_decimal_number_by_multiplying_by_with_behavior(
        &self,
        decimal_number: &Self,
        with_behavior: Arc<dyn PNSDecimalNumberBehaviors>,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.im_self(),
                decimalNumberByMultiplyingBy: decimal_number
                withBehavior: with_behavior
            ])
        }
    }

    /// Divides this number by another given number using the specified behavior.
    fn im_decimal_number_by_dividing_by_with_behavior(
        &self,
        decimal_number: &Self,
        with_behavior: Arc<dyn PNSDecimalNumberBehaviors>,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.im_self(),
                decimalNumberByDividingBy: decimal_number
                withBehavior: with_behavior
            ])
        }
    }

    /// Raises the number to a given power using the specified behavior.
    fn im_decimal_number_by_raising_to_power_with_behavior(
        &self,
        power: c_uint,
        with_behavior: Arc<dyn PNSDecimalNumberBehaviors>,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.im_self(),
                decimalNumberByRaisingToPower: power
                withBehavior: with_behavior
            ])
        }
    }

    /// Multiplies the number by 10 raised to the given power using the specified behavior.
    fn im_decimal_number_by_multiplying_by_power_of10_with_behavior(
        &self,
        power: c_short,
        with_behavior: Arc<dyn PNSDecimalNumberBehaviors>,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.im_self(),
                decimalNumberByMultiplyingByPowerOf10: power
                withBehavior: with_behavior
            ])
        }
    }

    /* Rounding Off
     */
    /// Returns a rounded version of the decimal number using the specified rounding behavior.
    fn im_decimal_number_by_rounding_according_to_behavior(
        &self,
        behavior: Arc<dyn PNSDecimalNumberBehaviors>,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.im_self(),
                decimalNumberByRoundingAccordingToBehavior: behavior
            ])
        }
    }

    /* Managing Behavior
     */

    /// The way arithmetic methods round off and handle error conditions.
    fn tp_default_behavior() -> Arc<dyn PNSDecimalNumberBehaviors> {
        unsafe {
            let behavior = msg_send![Self::im_class(), defaultBehavior];
            Arc::from_raw(behavior)
        }
    }

    /// Sets the way arithmetic methods round off and handle error conditions.
    fn tp_set_default_behavior(behavior: Arc<dyn PNSDecimalNumberBehaviors>) {
        unsafe { msg_send![Self::im_class(), setDefaultBehavior: behavior] }
    }

    /// The decimal number’s value, expressed as an NSDecimal structure.
    fn ip_decimal_value(&self) -> NSDecimal {
        unsafe { NSDecimal::from_id(msg_send![self.im_self(), decimalValue]) }
    }

    /// The decimal number’s closest approximate double value.
    fn ip_double_value(&self) -> f64 {
        unsafe { msg_send![self.im_self(), doubleValue] }
    }

    /// Returns a string representation of the decimal number appropriate for the specified locale.
    fn im_description_with_locale<L>(&self, locale: L) -> NSString
    where
        L: INSLocale,
    {
        unsafe {
            NSString::from_id(msg_send![
                self.im_self(),
                descriptionWithLocale: locale.im_self()
            ])
        }
    }

    /// A C string containing the Objective-C type for the data contained in the decimal number object.
    fn ip_objc_type(&self) -> *const c_char {
        unsafe { msg_send![self.im_self(), objCType] }
    }

    /* Comparing Decimal Numbers
     */

    /// Compares this decimal number and another.
    fn im_compare(&self, decimal_number: &Self) -> NSComparisonResult {
        unsafe { msg_send![self.im_self(), compare: decimal_number.im_self()] }
    }

    /* Getting Maximum and Minimum Possible Values
     */

    /// Returns the largest possible value of a decimal number.
    fn tp_maximum_decimal_number() -> NSDecimalNumber {
        unsafe { NSDecimalNumber::from_id(msg_send![Self::im_class(), maximumDecimalNumber]) }
    }

    /// Returns the smallest possible value of a decimal number.
    fn tp_minimum_decimal_number() -> NSDecimalNumber {
        unsafe { NSDecimalNumber::from_id(msg_send![Self::im_class(), minimumDecimalNumber]) }
    }
}

/// A protocol that declares three methods that control the discretionary aspects of working with decimal numbers.
pub trait PNSDecimalNumberBehaviors {
    /* Rounding Behavior
     */

    /// Returns the way that NSDecimalNumber's decimalNumberBy... methods round their return values.
    fn im_rounding_mode(&self) -> NSRoundingMode;

    /// Returns the number of digits allowed after the decimal separator.
    fn im_scale(&self) -> c_short;
}

/// A static byte buffer in memory.
pub trait INSData: PNSObject {
    /* Creating Data
     */

    /// Creates an empty data object.
    fn tm_data() -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), data]) }
    }

    /// Creates a data object containing a given number of bytes copied from a given buffer.
    fn tm_data_with_bytes_length(bytes: *const c_void, length: UInt) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), dataWithBytes: bytes length: length]) }
    }

    /// Creates a data object that holds a given number of bytes from a given buffer.
    fn tm_data_with_bytes_no_copy_length(bytes: *const c_void, length: UInt) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![Self::im_class(), dataWithBytesNoCopy: bytes
                                                                  length: length])
        }
    }

    /// Creates a data object that holds a given number of bytes from a given buffer.
    fn tm_data_with_bytes_no_copy_length_free_when_done(
        bytes: *const c_void,
        length: UInt,
        b: bool,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(
                msg_send![Self::im_class(), dataWithBytesNoCopy:bytes length:length freeWhenDone:b],
            )
        }
    }

    /// Creates a data object containing the contents of another data object.
    fn tm_data_with_data(data: NSData) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), dataWithData: data]) }
    }

    /*Reading Data from a File
     */

    /// Creates a data object by reading every byte from the file at a given path.
    fn tm_data_with_contents_of_file(path: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), dataWithContentsOfFile: path]) }
    }

    /* Accessing Underlying Bytes

    */

    /// A pointer to the data object's contents.
    fn ip_bytes(&self) -> *const c_void {
        unsafe { msg_send![self.im_self(), bytes] }
    }
}
