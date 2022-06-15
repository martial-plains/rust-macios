use std::{
    fmt,
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    os::raw::c_float,
};

use libc::{c_double, c_int, c_long, c_short, c_uint, c_ulong, c_ushort};
use objc::{
    class, msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::{
        traits::{INSDecimalNumber, INSNumber},
        NSComparisonResult, NSLocale, NSString,
    },
    objective_c_runtime::{
        id,
        traits::{FromId, INSValue, PNSObject, ToId},
    },
};

use super::{NSCalculationError, NSRoundingMode};

/// Type alias for `NSDecimalNumber`.
pub type NSDecimal = NSDecimalNumber;

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

/// An object for representing and performing arithmetic on base-10 numbers.
pub struct NSDecimalNumber {
    /// The raw pointer to the Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSDecimalNumber {
    fn im_class<'a>() -> &'a Class {
        class!(NSDecimalNumber)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl INSValue for NSDecimalNumber {}

impl INSNumber for NSDecimalNumber {}

impl INSDecimalNumber for NSDecimalNumber {}

impl fmt::Debug for NSDecimalNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSDecimalNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl Clone for NSDecimalNumber {
    fn clone(&self) -> Self {
        unsafe { msg_send![&*self.ptr, retain] }
    }
}

impl Deref for NSDecimalNumber {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.ptr
    }
}

impl DerefMut for NSDecimalNumber {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.ptr
    }
}

impl ToId for NSDecimalNumber {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSDecimalNumber {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}

impl From<c_uint> for NSDecimalNumber {
    fn from(value: c_uint) -> Self {
        NSDecimalNumber::tm_number_with_unsigned_int(value)
    }
}

impl From<c_int> for NSDecimalNumber {
    fn from(value: c_int) -> Self {
        NSDecimalNumber::tm_number_with_int(value)
    }
}

impl From<c_short> for NSDecimalNumber {
    fn from(value: c_short) -> Self {
        NSDecimalNumber::tm_number_with_short(value)
    }
}

impl From<c_ushort> for NSDecimalNumber {
    fn from(value: c_ushort) -> Self {
        NSDecimalNumber::tm_number_with_unsigned_short(value)
    }
}

impl From<c_long> for NSDecimalNumber {
    fn from(value: c_long) -> Self {
        NSDecimalNumber::tm_number_with_long(value)
    }
}

impl From<c_ulong> for NSDecimalNumber {
    fn from(value: c_ulong) -> Self {
        NSDecimalNumber::tm_number_with_unsigned_long(value)
    }
}

impl From<c_float> for NSDecimalNumber {
    fn from(value: c_float) -> Self {
        NSDecimalNumber::tm_number_with_float(value)
    }
}

impl From<c_double> for NSDecimalNumber {
    fn from(value: c_double) -> Self {
        NSDecimalNumber::tm_number_with_double(value)
    }
}

impl<S> From<S> for NSDecimalNumber
where
    S: Into<NSString>,
{
    fn from(value: S) -> Self {
        NSDecimalNumber::tm_decimal_number_with_string(value.into())
    }
}

impl<T> Add<T> for NSDecimalNumber
where
    T: Into<NSDecimalNumber>,
{
    type Output = NSDecimalNumber;

    fn add(self, other: T) -> Self::Output {
        self.im_decimal_number_by_adding(other.into())
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
        self.im_decimal_number_by_subtracting(other.into())
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
        self.im_decimal_number_by_multiplying_by(other.into())
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
        self.im_decimal_number_by_dividing_by(other.into())
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
