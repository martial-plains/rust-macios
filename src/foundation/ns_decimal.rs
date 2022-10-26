use std::{
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    os::raw::c_float,
    sync::Arc,
};

use libc::{c_char, c_double, c_int, c_long, c_short, c_uint, c_ulong, c_ulonglong, c_ushort};
use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::{INSLocale, INSNumber, INSString, NSComparisonResult, NSLocale, NSString},
    objective_c_runtime::{
        macros::{interface_impl, object},
        traits::FromId,
        INSValue,
    },
};

use super::{
    ns_decimal_number_behaviors::PNSDecimalNumberBehaviors, NSCalculationError, NSDecimal,
    NSRoundingMode,
};

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    /* Creating a Decimal from Another Decimal
     */

    /// Creating a Decimal from Another Decimal
    pub fn NSDecimalCopy(destination: *mut NSDecimal, source: *const NSDecimal);

    /* Converting Between Decimals and Strings
     */

    /// Returns a string representation of the decimal value appropriate for the specified locale.
    pub fn NSDecimalString(dcm: *const NSDecimal, locale: NSLocale) -> NSString;

    /// Compacts the decimal structure for efficiency.
    pub fn NSDecimalCompact(number: *mut NSDecimal);

    /// Adds two decimal values.
    pub fn NSDecimalAdd(
        result: *mut NSDecimal,
        leftOperand: *const NSDecimal,
        rightOperand: *const NSDecimal,
        roundingMode: NSRoundingMode,
    ) -> NSCalculationError;

    /// Subtracts one decimal value from another.
    pub fn NSDecimalSubtract(
        result: *mut NSDecimal,
        leftOperand: *const NSDecimal,
        rightOperand: *const NSDecimal,
        roundingMode: NSRoundingMode,
    ) -> NSCalculationError;

    /// Subtracts one decimal value from another.
    pub fn NSDecimalMultiply(
        result: *mut NSDecimal,
        leftOperand: *const NSDecimal,
        rightOperand: *const NSDecimal,
        roundingMode: NSRoundingMode,
    ) -> NSCalculationError;

    /// Multiplies two decimal numbers together.
    pub fn NSDecimalDivide(
        result: *mut NSDecimal,
        leftOperand: *const NSDecimal,
        rightOperand: *const NSDecimal,
        roundingMode: NSRoundingMode,
    ) -> NSCalculationError;

    /// Multiplies a decimal by the specified power of 10.
    pub fn NSDecimalPower(
        result: *mut NSDecimal,
        number: *const NSDecimal,
        power: c_short,
        roundingMode: NSRoundingMode,
    ) -> NSCalculationError;

    /// Rounds off the decimal value.
    pub fn NSDecimalRound(
        result: *mut NSDecimal,
        number: *const NSDecimal,
        scale: c_short,
        roundingMode: NSRoundingMode,
    ) -> NSCalculationError;

    /// Raises the decimal value to the specified power.
    pub fn NSDecimalMultiplyByPowerOf10(
        result: *mut NSDecimal,
        number: *const NSDecimal,
        power: c_short,
        roundingMode: NSRoundingMode,
    ) -> NSCalculationError;

    /// Normalizes the internal format of two decimal numbers to simplify later operations.
    pub fn NSDecimalNormalize(
        result: *mut NSDecimal,
        number1: *const NSDecimal,
        number2: *const NSDecimal,
    );

    /* Comparing Decimals
     */

    /// Compares two decimal values.
    pub fn NSDecimalCompare(
        leftOperand: *const NSDecimal,
        rightOperand: *const NSDecimal,
    ) -> NSComparisonResult;
}

object! {
    /// An object for representing and performing arithmetic on base-10 numbers.
    unsafe pub struct NSDecimalNumber;
}

impl INSValue for NSDecimalNumber {}

impl INSNumber for NSDecimalNumber {}

#[interface_impl(NSNumber)]
impl NSDecimalNumber {
    /* Creating a Decimal Number
     */

    /// Creates and returns a decimal number equivalent to a given decimal structure.
    #[method]
    pub fn decimal_number_with_decimal(decimal: NSDecimalNumber) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                Self::m_class(),
                decimalNumberWithDecimal: decimal
            ])
        }
    }

    /// Creates and returns a decimal number equivalent to the number specified by the arguments.
    #[method]
    pub fn decimal_number_with_mantissa(
        mantissa: c_ulonglong,
        exponent: c_short,
        is_negative: bool,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                Self::m_class(),
                decimalNumberWithMantissa: mantissa
                exponent: exponent
                isNegative: is_negative
            ])
        }
    }

    /// Creates a decimal number whose value is equivalent to that in a given numeric string.
    #[method]
    pub fn decimal_number_with_string(string: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), decimalNumberWithString: string]) }
    }

    /// Creates a decimal number whose value is equivalent to that in a given numeric string, interpreted using a given locale.
    #[method]
    pub fn decimal_number_with_string_locale(string: NSString, locale: NSLocale) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![Self::m_class(), decimalNumberWithString:string locale:locale])
        }
    }

    /// A decimal number equivalent to the number 1.0.
    #[property]
    pub fn one() -> Self
    where
        Self: Sized + 'static + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), one]) }
    }

    /// A decimal number equivalent to the number 0.0.
    #[property]
    pub fn zero() -> Self
    where
        Self: Sized + 'static + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), zero]) }
    }

    /// A decimal number that specifies no number.
    #[property]
    pub fn not_a_number() -> Self
    where
        Self: Sized + 'static + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), notANumber]) }
    }

    /* Initializing a Decimal Number
     */

    /// Initializes a decimal number to represent a given decimal.
    #[method]
    pub fn init_with_decimal(&mut self, decimal: NSDecimalNumber) {
        unsafe { msg_send![self.m_self(), initWithDecimal: decimal] }
    }

    /// Initializes a decimal number using the given mantissa, exponent, and sign.
    #[method]
    pub fn init_with_mantissa_exponent_is_negative(
        &mut self,
        mantissa: c_ulonglong,
        exponent: c_short,
        is_negative: bool,
    ) {
        unsafe {
            msg_send![self.m_self(), initWithMantissa: mantissa
                exponent: exponent
                isNegative: is_negative]
        }
    }

    /// Initializes a decimal number so that its value is equivalent to that in a given numeric string.
    #[method]
    pub fn init_with_string<S>(&mut self, string: S)
    where
        S: INSString,
    {
        unsafe { msg_send![self.m_self(), initWithString: string] }
    }

    /// Initializes a decimal number so that its value is equivalent to that in a given numeric string, interpreted using a given locale.
    #[method]
    pub fn init_with_string_locale<S, L>(&mut self, string: S, locale: L)
    where
        S: INSString,
        L: INSLocale,
    {
        unsafe { msg_send![self.m_self(), initWithString: string locale: locale] }
    }

    /* Performing Arithmetic
     */

    /// Adds this number to another given number.
    #[method]
    pub fn decimal_number_by_adding(&self, decimal_number: Self) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.m_self(),
                decimalNumberByAdding: decimal_number
            ])
        }
    }

    /// Subtracts another given number from this one.
    #[method]
    pub fn decimal_number_by_subtracting(&self, decimal_number: Self) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.m_self(),
                decimalNumberBySubtracting: decimal_number
            ])
        }
    }

    /// Multiplies the number by another given number.
    #[method]
    pub fn decimal_number_by_multiplying_by(&self, decimal_number: Self) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.m_self(),
                decimalNumberByMultiplyingBy: decimal_number
            ])
        }
    }

    /// Divides the number by another given number.
    #[method]
    pub fn decimal_number_by_dividing_by(&self, decimal_number: Self) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.m_self(),
                decimalNumberByDividingBy: decimal_number
            ])
        }
    }

    /// Raises the number to a given power.
    #[method]
    pub fn decimal_number_by_raising_to_power(&self, power: c_uint) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.m_self(),
                decimalNumberByRaisingToPower: power
            ])
        }
    }

    /// Multiplies the number by 10 raised to the given power.
    #[method]
    pub fn decimal_number_by_multiplying_by_power_of_10(&self, power: c_short) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.m_self(),
                decimalNumberByMultiplyingByPowerOf10: power
            ])
        }
    }

    /// Adds this number to another given number using the specified behavior.
    #[method]
    pub fn decimal_number_by_adding_with_behavior(
        &self,
        decimal_number: &Self,
        with_behavior: Arc<dyn PNSDecimalNumberBehaviors>,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.m_self(),
                decimalNumberByAdding: decimal_number
                withBehavior: with_behavior
            ])
        }
    }

    /// Subtracts this a given number from this one using the specified behavior.
    #[method]
    pub fn decimal_number_by_subtracting_with_behavior(
        &self,
        decimal_number: &Self,
        with_behavior: Arc<dyn PNSDecimalNumberBehaviors>,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.m_self(),
                decimalNumberBySubtracting: decimal_number
                withBehavior: with_behavior
            ])
        }
    }

    /// Multiplies this number by another given number using the specified behavior.
    #[method]
    pub fn decimal_number_by_multiplying_by_with_behavior(
        &self,
        decimal_number: &Self,
        with_behavior: Arc<dyn PNSDecimalNumberBehaviors>,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.m_self(),
                decimalNumberByMultiplyingBy: decimal_number
                withBehavior: with_behavior
            ])
        }
    }

    /// Divides this number by another given number using the specified behavior.
    #[method]
    pub fn decimal_number_by_dividing_by_with_behavior(
        &self,
        decimal_number: &Self,
        with_behavior: Arc<dyn PNSDecimalNumberBehaviors>,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.m_self(),
                decimalNumberByDividingBy: decimal_number
                withBehavior: with_behavior
            ])
        }
    }

    /// Raises the number to a given power using the specified behavior.
    #[method]
    pub fn decimal_number_by_raising_to_power_with_behavior(
        &self,
        power: c_uint,
        with_behavior: Arc<dyn PNSDecimalNumberBehaviors>,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.m_self(),
                decimalNumberByRaisingToPower: power
                withBehavior: with_behavior
            ])
        }
    }

    /// Multiplies the number by 10 raised to the given power using the specified behavior.
    #[method]
    pub fn decimal_number_by_multiplying_by_power_of10_with_behavior(
        &self,
        power: c_short,
        with_behavior: Arc<dyn PNSDecimalNumberBehaviors>,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.m_self(),
                decimalNumberByMultiplyingByPowerOf10: power
                withBehavior: with_behavior
            ])
        }
    }

    /* Rounding Off
     */

    /// Returns a rounded version of the decimal number using the specified rounding behavior.
    #[method]
    pub fn decimal_number_by_rounding_according_to_behavior(
        &self,
        behavior: Arc<dyn PNSDecimalNumberBehaviors>,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.m_self(),
                decimalNumberByRoundingAccordingToBehavior: behavior
            ])
        }
    }

    /* Managing Behavior
     */

    /// The way arithmetic methods round off and handle error conditions.
    #[property]
    pub fn default_behavior() -> Arc<dyn PNSDecimalNumberBehaviors> {
        unsafe {
            let behavior = msg_send![Self::m_class(), defaultBehavior];
            Arc::from_raw(behavior)
        }
    }

    /// Sets the way arithmetic methods round off and handle error conditions.
    #[property]
    pub fn set_default_behavior(behavior: Arc<dyn PNSDecimalNumberBehaviors>) {
        unsafe { msg_send![Self::m_class(), setDefaultBehavior: behavior] }
    }

    /// A C string containing the Objective-C type for the data contained in the decimal number object.
    #[property]
    pub fn objc_type(&self) -> *const c_char {
        unsafe { msg_send![self.m_self(), objCType] }
    }

    /* Getting Maximum and Minimum Possible Values
     */

    /// Returns the largest possible value of a decimal number.
    #[property]
    pub fn maximum_decimal_number() -> NSDecimalNumber {
        unsafe { NSDecimalNumber::from_id(msg_send![Self::m_class(), maximumDecimalNumber]) }
    }

    /// Returns the smallest possible value of a decimal number.
    #[property]
    pub fn minimum_decimal_number() -> NSDecimalNumber {
        unsafe { NSDecimalNumber::from_id(msg_send![Self::m_class(), minimumDecimalNumber]) }
    }
}

impl From<c_uint> for NSDecimalNumber {
    fn from(value: c_uint) -> Self {
        NSDecimalNumber::m_number_with_unsigned_int(value)
    }
}

impl From<c_int> for NSDecimalNumber {
    fn from(value: c_int) -> Self {
        NSDecimalNumber::m_number_with_int(value)
    }
}

impl From<c_short> for NSDecimalNumber {
    fn from(value: c_short) -> Self {
        NSDecimalNumber::m_number_with_short(value)
    }
}

impl From<c_ushort> for NSDecimalNumber {
    fn from(value: c_ushort) -> Self {
        NSDecimalNumber::m_number_with_unsigned_short(value)
    }
}

impl From<c_long> for NSDecimalNumber {
    fn from(value: c_long) -> Self {
        NSDecimalNumber::m_number_with_long(value)
    }
}

impl From<c_ulong> for NSDecimalNumber {
    fn from(value: c_ulong) -> Self {
        NSDecimalNumber::m_number_with_unsigned_long(value)
    }
}

impl From<c_float> for NSDecimalNumber {
    fn from(value: c_float) -> Self {
        NSDecimalNumber::m_number_with_float(value)
    }
}

impl From<c_double> for NSDecimalNumber {
    fn from(value: c_double) -> Self {
        NSDecimalNumber::m_number_with_double(value)
    }
}

impl<S> From<S> for NSDecimalNumber
where
    S: Into<NSString>,
{
    fn from(value: S) -> Self {
        NSDecimalNumber::m_decimal_number_with_string(value.into())
    }
}

impl<T> Add<T> for NSDecimalNumber
where
    T: Into<NSDecimalNumber>,
{
    type Output = NSDecimalNumber;

    fn add(self, other: T) -> Self::Output {
        self.m_decimal_number_by_adding(other.into())
    }
}

impl<T> AddAssign<T> for NSDecimalNumber
where
    T: Into<NSDecimalNumber>,
{
    fn add_assign(&mut self, other: T) {
        *self = self.clone().add(other.into());
    }
}

impl<T> Sub<T> for NSDecimalNumber
where
    T: Into<NSDecimalNumber>,
{
    type Output = NSDecimalNumber;

    fn sub(self, other: T) -> Self::Output {
        self.m_decimal_number_by_subtracting(other.into())
    }
}

impl<T> SubAssign<T> for NSDecimalNumber
where
    T: Into<NSDecimalNumber>,
{
    fn sub_assign(&mut self, other: T) {
        *self = self.clone().sub(other.into());
    }
}

impl<T> Mul<T> for NSDecimalNumber
where
    T: Into<NSDecimalNumber>,
{
    type Output = NSDecimalNumber;

    fn mul(self, other: T) -> Self::Output {
        self.m_decimal_number_by_multiplying_by(other.into())
    }
}

impl<T> MulAssign<T> for NSDecimalNumber
where
    T: Into<NSDecimalNumber>,
{
    fn mul_assign(&mut self, other: T) {
        *self = self.clone().mul(other.into());
    }
}

impl<T> Div<T> for NSDecimalNumber
where
    T: Into<NSDecimalNumber>,
{
    type Output = NSDecimalNumber;

    fn div(self, other: T) -> Self::Output {
        self.m_decimal_number_by_dividing_by(other.into())
    }
}

impl<T> DivAssign<T> for NSDecimalNumber
where
    T: Into<NSDecimalNumber>,
{
    fn div_assign(&mut self, other: T) {
        *self = self.clone().div(other.into());
    }
}

impl Add<NSDecimalNumber> for f64 {
    type Output = NSDecimalNumber;

    fn add(self, other: NSDecimalNumber) -> Self::Output {
        other.add(self)
    }
}
