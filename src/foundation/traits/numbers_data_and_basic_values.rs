use std::sync::Arc;

use libc::{
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint,
    c_ulong, c_ulonglong, c_ushort, c_void,
};

use crate::{
    foundation::{
        ComparisonResult, Int, NSData, NSDecimalNumber, NSLocale, NSRoundingMode, NSString, UInt,
    },
    objective_c_runtime::traits::{INSValue, PNSObject},
};

use super::INSLocale;

/// The group of methods that are used with `NSNumber` objects.
pub trait t_NSNumber: INSValue {
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
    fn numberWithBool(value: bool) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as a signed char.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithChar(value: c_schar) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as a double.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithDouble(value: c_double) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as a float.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithFloat(value: c_float) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as a signed int.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithInt(value: c_int) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an NSInteger.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithInteger(value: Int) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as a signed long.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithLong(value: c_long) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as a signed long long.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithLongLong(value: c_longlong) -> Self;

    /// Creates and returns an NSNumber object containing value, treating it as a signed short.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithShort(value: c_short) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned char.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithUnsignedChar(value: c_uchar) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned int.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithUnsignedInt(value: c_uint) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an NSUInteger.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithUnsignedInteger(value: UInt) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned long.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithUnsignedLong(value: c_ulong) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned long long.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithUnsignedLongLong(value: c_ulonglong) -> Self;

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned short.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn numberWithUnsignedShort(value: c_ushort) -> Self;

    /* Initializing an NSNumber Object
     */

    /// Returns an NSNumber object initialized to contain a given value, treated as a BOOL.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    fn initWithBool(&self, value: bool) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as a signed char.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithChar(&self, value: c_schar) -> Self;

    /// Returns an NSNumber object initialized to contain value, treated as a double.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithDouble(&self, value: c_double) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as a float.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithFloat(&self, value: c_float) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as a signed int.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithInt(&self, value: c_int) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an NSInteger.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithInteger(&self, value: Int) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as a signed long.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithLong(&self, value: c_long) -> Self;

    /// Returns an NSNumber object initialized to contain value, treated as a signed long long.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithLongLong(&self, value: c_longlong) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as a signed short.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithShort(&self, value: c_short) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned char.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithUnsignedChar(&self, value: c_uchar) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned int.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithUnsignedInt(&self, value: c_uint) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an NSUInteger.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithUnsignedInteger(&self, value: c_uint) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned long.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithUnsignedLong(&self, value: c_ulong) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned long long.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithUnsignedLongLong(&self, value: c_ulonglong) -> Self;

    /// Returns an NSNumber object initialized to contain a given value, treated as an unsigned short.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the NSNumber object.
    ///
    /// # Returns
    ///
    /// Returns an `NSNumber` object containing the value.
    fn initWithUnsignedShort(&self, value: c_ushort) -> Self;

    /* Accessing Numeric Values
     */

    /// The number object's value expressed as a Boolean value.
    fn boolValue(&self) -> bool;

    /// The number object's value expressed as a char.
    fn charValue(&self) -> c_schar;

    /// The number object's value expressed as an NSDecimal structure.
    //// fn decimalValue(&self) -> NSDecimal;

    /// The number object's value expressed as a double, converted as necessary.
    fn doubleValue(&self) -> c_double;

    /// The number object's value expressed as a float, converted as necessary.
    fn floatValue(&self) -> c_float;

    /// The number object's value expressed as an int, converted as necessary.
    fn intValue(&self) -> c_int;

    /// The number object's value expressed as an NSInteger object, converted as necessary.
    fn integerValue(&self) -> Int;

    /// The number object’s value expressed as a long long, converted as necessary.
    fn longLongValue(&self) -> c_longlong;

    /// The number object’s value expressed as a long, converted as necessary.
    fn longValue(&self) -> c_long;

    /// The number object's value expressed as a short, converted as necessary.
    fn shortValue(&self) -> c_short;

    /// The number object's value expressed as an unsigned char, converted as necessary.
    fn unsignedCharValue(&self) -> c_uchar;

    /// The number object's value expressed as an NSUInteger object, converted as necessary.
    fn unsignedIntegerValue(&self) -> UInt;

    /// The number object's value expressed as an unsigned int, converted as necessary.
    fn unsignedIntValue(&self) -> c_uint;

    /// The number object’s value expressed as an unsigned long long, converted as necessary.
    fn unsignedLongLongValue(&self) -> c_ulonglong;

    /// The number object's value expressed as an unsigned long, converted as necessary.
    fn unsignedLongValue(&self) -> c_ulong;

    /// The number object's value expressed as an unsigned short, converted as necessary.
    fn unsignedShortValue(&self) -> c_ushort;

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
    fn descriptionWithLocale(&self, locale: NSLocale) -> NSString;

    /// The number object's value expressed as a human-readable string.
    fn stringValue(&self) -> NSString;

    /* Comparing NSNumber Objects
     */

    /// Returns an NSComparisonResult value that indicates whether the number object’s value is greater than, equal to, or less than a given number.
    ///
    /// # Arguments
    ///
    /// * `other` - The number to compare to the number object’s value.
    fn compare(&self, other: &Self) -> ComparisonResult;

    /// Returns a Boolean value that indicates whether the number object’s value and a given number are equal.
    ///
    /// # Arguments
    ///
    /// * `other` - The number to compare to the number object’s value.
    fn isEqualToNumber(&self, other: &Self) -> bool;
}

/// An object for representing and performing arithmetic on base-10 numbers.
pub trait t_NSDecimalNumber: t_NSNumber {
    /* Creating a Decimal Number
     */

    /// Creates and returns a decimal number equivalent to a given decimal structure.
    fn decimalNumberWithDecimal(decimal: NSDecimalNumber) -> Self;

    /// Creates and returns a decimal number equivalent to the number specified by the arguments.
    fn decimalNumberWithMantissa(
        mantissa: c_ulonglong,
        exponent: c_short,
        is_negative: bool,
    ) -> Self;

    /// Creates a decimal number whose value is equivalent to that in a given numeric string.
    fn decimalNumberWithString<S>(string: S) -> Self
    where
        S: Into<NSString>;

    /// Creates a decimal number whose value is equivalent to that in a given numeric string, interpreted using a given locale.
    fn decimalNumberWithStringLocale<S, L>(string: S, locale: L) -> Self
    where
        S: Into<NSString>,
        L: INSLocale;

    /// A decimal number equivalent to the number 1.0.
    fn one() -> Self;

    /// A decimal number equivalent to the number 0.0.
    fn zero() -> Self;

    /// A decimal number that specifies no number.
    fn notANumber() -> Self;

    /* Initializing a Decimal Number
     */

    /// Initializes a decimal number to represent a given decimal.
    fn initWithDecimal(&mut self, decimal: NSDecimalNumber);

    /// Initializes a decimal number using the given mantissa, exponent, and sign.
    fn initWithMantissaExponentIsNegative(
        &mut self,
        mantissa: c_ulonglong,
        exponent: c_short,
        is_negative: bool,
    );

    /// Initializes a decimal number so that its value is equivalent to that in a given numeric string.
    fn initWithString<S>(&mut self, string: S)
    where
        S: Into<NSString>;

    /// Initializes a decimal number so that its value is equivalent to that in a given numeric string, interpreted using a given locale.
    fn initWithStringLocale<S, L>(&mut self, string: S, locale: L)
    where
        S: Into<NSString>,
        L: INSLocale;

    /* Performing Arithmetic
     */

    /// Adds this number to another given number.
    fn decimalNumberByAdding(&self, decimal_number: Self) -> Self;

    /// Subtracts another given number from this one.
    fn decimalNumberBySubtracting(&self, decimal_number: Self) -> Self;

    /// Multiplies the number by another given number.
    fn decimalNumberByMultiplyingBy(&self, decimal_number: Self) -> Self;

    /// Divides the number by another given number.
    fn decimalNumberByDividingBy(&self, decimal_number: Self) -> Self;

    /// Raises the number to a given power.
    fn decimalNumberByRaisingToPower(&self, power: c_uint) -> Self;

    /// Multiplies the number by 10 raised to the given power.
    fn decimalNumberByMultiplyingByPowerOf10(&self, power: c_short) -> Self;

    /// Adds this number to another given number using the specified behavior.
    fn decimalNumberByAddingWithBehavior(
        &self,
        decimal_number: &Self,
        with_behavior: Arc<dyn t_NSDecimalNumberBehaviors>,
    ) -> Self;

    /// Subtracts this a given number from this one using the specified behavior.
    fn decimalNumberBySubtractingWithBehavior(
        &self,
        decimal_number: &Self,
        with_behavior: Arc<dyn t_NSDecimalNumberBehaviors>,
    ) -> Self;

    /// Multiplies this number by another given number using the specified behavior.
    fn decimalNumberByMultiplyingByWithBehavior(
        &self,
        decimal_number: &Self,
        with_behavior: Arc<dyn t_NSDecimalNumberBehaviors>,
    ) -> Self;

    /// Divides this number by another given number using the specified behavior.
    fn decimalNumberByDividingByWithBehavior(
        &self,
        decimal_number: &Self,
        with_behavior: Arc<dyn t_NSDecimalNumberBehaviors>,
    ) -> Self;

    /// Raises the number to a given power using the specified behavior.
    fn decimalNumberByRaisingToPowerWithBehavior(
        &self,
        power: c_uint,
        with_behavior: Arc<dyn t_NSDecimalNumberBehaviors>,
    ) -> Self;

    /// Multiplies the number by 10 raised to the given power using the specified behavior.
    fn decimalNumberByMultiplyingByPowerOf10WithBehavior(
        &self,
        power: c_short,
        with_behavior: Arc<dyn t_NSDecimalNumberBehaviors>,
    ) -> Self;

    /* Rounding Off
     */
    /// Returns a rounded version of the decimal number using the specified rounding behavior.
    fn decimalNumberByRoundingAccordingToBehavior(
        &self,
        behavior: Arc<dyn t_NSDecimalNumberBehaviors>,
    ) -> Self;

    /* Managing Behavior
     */

    /// The way arithmetic methods round off and handle error conditions.
    fn defaultBehavior() -> Arc<dyn t_NSDecimalNumberBehaviors>;

    /// Sets the way arithmetic methods round off and handle error conditions.
    fn setDefaultBehavior(behavior: Arc<dyn t_NSDecimalNumberBehaviors>);

    /// The decimal number’s closest approximate double value.
    fn doubleValue(&self) -> f64;

    /// Returns a string representation of the decimal number appropriate for the specified locale.
    fn descriptionWithLocale<L>(&self, locale: L) -> NSString
    where
        L: INSLocale;

    /// A C string containing the Objective-C type for the data contained in the decimal number object.
    fn objcType(&self) -> *const c_char;

    /* Comparing Decimal Numbers
     */

    /// Compares this decimal number and another.
    fn compare(&self, decimal_number: &Self) -> ComparisonResult;

    /* Getting Maximum and Minimum Possible Values
     */

    /// Returns the largest possible value of a decimal number.
    fn maximumDecimalNumber() -> Self;

    /// Returns the smallest possible value of a decimal number.
    fn minimumDecimalNumber() -> Self;
}

/// A protocol that declares three methods that control the discretionary aspects of working with decimal numbers.
pub trait t_NSDecimalNumberBehaviors {
    /* Rounding Behavior
     */

    /// Returns the way that NSDecimalNumber's decimalNumberBy... methods round their return values.
    fn roundingMode(&self) -> NSRoundingMode;

    /// Returns the number of digits allowed after the decimal separator.
    fn scale(&self) -> c_short;
}

/// A static byte buffer in memory.
pub trait INSData: PNSObject {
    /* Creating Data
     */

    /// Creates an empty data object.
    fn data() -> Self;

    /// Creates a data object containing a given number of bytes copied from a given buffer.
    fn dataWithBytesLength(bytes: *const c_void, length: UInt) -> Self;

    /// Creates a data object that holds a given number of bytes from a given buffer.
    fn dataWithBytesNoCopyLength(bytes: *const c_void, length: UInt) -> Self;

    /// Creates a data object that holds a given number of bytes from a given buffer.
    fn dataWithBytesNoCopyLengthFreeWhenDone(bytes: *const c_void, length: UInt, b: bool) -> Self;

    /// Creates a data object containing the contents of another data object.
    fn dataWithData(data: NSData) -> Self;

    /*Reading Data from a File
     */

    /// Creates a data object by reading every byte from the file at a given path.
    fn dataWithContentsOfFile(path: NSString) -> Self;

    /// A pointer to the data object's contents.
    fn bytes(&self) -> *const c_void;
}
