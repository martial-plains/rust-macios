use std::{
    fmt,
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    os::raw::c_float,
    sync::Arc,
};

use libc::{c_double, c_int, c_long, c_longlong, c_short, c_uint, c_ulong, c_ushort};
use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    foundation::{
        traits::{t_NSDecimalNumber, t_NSNumber},
        ComparisonResult, Locale, String,
    },
    id,
    objective_c_runtime::traits::{t_NSObject, t_NSValue},
};

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
    pub fn NSDecimalString(dcm: *const NSDecimal, locale: Locale) -> String;

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
    ) -> ComparisonResult;
}

/// An object for representing and performing arithmetic on base-10 numbers.
pub struct NSDecimalNumber {
    /// The raw pointer to the Objective-C object.
    pub obj: Id<Object>,
}

impl t_NSObject for NSDecimalNumber {
    fn init() -> Self {
        unsafe {
            let class: id = msg_send![class!(NSDecimalNumber), alloc];
            let obj = msg_send![class, init];
            NSDecimalNumber { obj }
        }
    }

    fn to_id(mut self) -> id {
        &mut *self.obj
    }

    fn from_id(obj: id) -> Self {
        Self {
            obj: unsafe { Id::from_ptr(obj) },
        }
    }

    fn description(&self) -> String {
        unsafe {
            let obj = msg_send![&*self.obj, description];
            String::from_id(obj)
        }
    }

    fn debug_description(&self) -> String {
        unsafe {
            let obj = msg_send![&*self.obj, debugDescription];
            String::from_id(obj)
        }
    }

    fn retain(&self) -> Self {
        unsafe {
            let obj = msg_send![&*self.obj, retain];
            Self::from_id(obj)
        }
    }
}

impl t_NSValue for NSDecimalNumber {}

impl t_NSNumber for NSDecimalNumber {
    fn number_with_bool(value: bool) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithBool: value] }
    }

    fn number_with_char(value: libc::c_schar) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithChar: value] }
    }

    fn number_with_double(value: c_double) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithDouble: value] }
    }

    fn number_with_float(value: libc::c_float) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithFloat: value] }
    }

    fn number_with_int(value: c_int) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithInt: value] }
    }

    fn number_with_integer(value: super::Int) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithInteger: value] }
    }

    fn number_with_long(value: c_long) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithLong: value] }
    }

    fn number_with_long_long(value: c_longlong) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithLongLong: value] }
    }

    fn number_with_short(value: c_short) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithShort: value] }
    }

    fn number_with_unsigned_char(value: libc::c_uchar) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithUnsignedChar: value] }
    }

    fn number_with_unsigned_int(value: c_uint) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithUnsignedInt: value] }
    }

    fn number_with_unsigned_integer(value: super::UInt) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithUnsignedInteger: value] }
    }

    fn number_with_unsigned_long(value: c_ulong) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithUnsignedLong: value] }
    }

    fn number_with_unsigned_long_long(value: libc::c_ulonglong) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithUnsignedLongLong: value] }
    }

    fn number_with_unsigned_short(value: c_ushort) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithUnsignedShort: value] }
    }

    fn init_with_bool(&self, _value: bool) -> Self {
        todo!()
    }

    fn init_with_char(&self, _value: libc::c_schar) -> Self {
        todo!()
    }

    fn init_with_double(&self, _value: c_double) -> Self {
        todo!()
    }

    fn init_with_float(&self, _value: libc::c_float) -> Self {
        todo!()
    }

    fn init_with_int(&self, _value: c_int) -> Self {
        todo!()
    }

    fn init_with_integer(&self, _value: super::Int) -> Self {
        todo!()
    }

    fn init_with_long(&self, _value: c_long) -> Self {
        todo!()
    }

    fn init_with_long_long(&self, _value: c_longlong) -> Self {
        todo!()
    }

    fn init_with_short(&self, _value: c_short) -> Self {
        todo!()
    }

    fn init_with_unsigned_char(&self, _value: libc::c_uchar) -> Self {
        todo!()
    }

    fn init_with_unsigned_int(&self, _value: c_uint) -> Self {
        todo!()
    }

    fn init_with_unsigned_integer(&self, _value: c_uint) -> Self {
        todo!()
    }

    fn init_with_unsigned_long(&self, _value: c_ulong) -> Self {
        todo!()
    }

    fn init_with_unsigned_long_long(&self, _value: libc::c_ulonglong) -> Self {
        todo!()
    }

    fn init_with_unsigned_short(&self, _value: c_ushort) -> Self {
        todo!()
    }

    fn bool_value(&self) -> bool {
        unsafe { msg_send![self.obj, boolValue] }
    }

    fn char_value(&self) -> libc::c_schar {
        unsafe { msg_send![self.obj, charValue] }
    }

    fn double_value(&self) -> c_double {
        unsafe { msg_send![self.obj, doubleValue] }
    }

    fn float_value(&self) -> libc::c_float {
        unsafe { msg_send![self.obj, floatValue] }
    }

    fn int_value(&self) -> c_int {
        unsafe { msg_send![self.obj, intValue] }
    }

    fn integer_value(&self) -> super::Int {
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

    fn unsigned_char_value(&self) -> libc::c_uchar {
        unsafe { msg_send![self.obj, unsignedCharValue] }
    }

    fn unsigned_integer_value(&self) -> super::UInt {
        unsafe { msg_send![self.obj, unsignedIntegerValue] }
    }

    fn unsigned_int_value(&self) -> c_uint {
        unsafe { msg_send![self.obj, unsignedIntValue] }
    }

    fn unsigned_long_long_value(&self) -> libc::c_ulonglong {
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
            let id: id = msg_send![self.obj, descriptionWithLocale: locale.obj];
            String::from_id(id)
        }
    }

    fn string_value(&self) -> String {
        unsafe {
            let id: id = msg_send![self.obj, stringValue];
            String::from_id(id)
        }
    }

    fn compare(&self, other: &Self) -> ComparisonResult {
        unsafe { msg_send![self.obj, compare: other] }
    }

    fn is_equal_to_number(&self, other: &Self) -> bool {
        unsafe { msg_send![self.obj, isEqualToNumber: other] }
    }
}

impl t_NSDecimalNumber for NSDecimalNumber {
    fn decimal_number_with_decimal(decimal: NSDecimalNumber) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), decimalNumberWithDecimal: decimal.obj] }
    }

    fn decimal_number_with_mantissa(
        mantissa: libc::c_ulonglong,
        exponent: c_short,
        is_negative: bool,
    ) -> Self {
        unsafe {
            msg_send![
                class!(NSDecimalNumber),
                decimalNumberWithMantissa: mantissa
                exponent: exponent
                isNegative: is_negative
            ]
        }
    }

    fn decimal_number_with_string<S>(string: S) -> Self
    where
        S: Into<String>,
    {
        unsafe {
            msg_send![
                class!(NSDecimalNumber),
                decimalNumberWithString: string.into()
            ]
        }
    }

    fn decimal_number_with_string_locale<S, L>(string: S, locale: L) -> Self
    where
        S: Into<String>,
        L: crate::foundation::traits::t_NSLocale,
    {
        unsafe {
            msg_send![
                class!(NSDecimalNumber),
                decimalNumberWithString: string.into()
                locale: locale
            ]
        }
    }

    fn one() -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), one] }
    }

    fn zero() -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), zero] }
    }

    fn not_a_number() -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), notANumber] }
    }

    fn init_with_decimal(&mut self, decimal: NSDecimalNumber) {
        unsafe {
            let _: () = msg_send![self.obj, initWithDecimal: decimal.obj];
        }
    }

    fn init_with_mantissa_exponent_is_negative(
        &mut self,
        mantissa: libc::c_ulonglong,
        exponent: c_short,
        is_negative: bool,
    ) {
        unsafe {
            let _: () = msg_send![
                self.obj,
                initWithMantissa: mantissa
                exponent: exponent
                isNegative: is_negative
            ];
        }
    }

    fn init_with_string<S>(&mut self, string: S)
    where
        S: Into<String>,
    {
        unsafe { msg_send![self.obj, initWithString: string.into()] }
    }

    fn init_with_string_locale<S, L>(&mut self, string: S, locale: L)
    where
        S: Into<String>,
        L: crate::foundation::traits::t_NSLocale,
    {
        unsafe { msg_send![self.obj, initWithString: string.into() locale: locale] }
    }

    fn decimal_number_by_adding(&self, decimal_number: Self) -> Self {
        unsafe { msg_send![self.obj, decimalNumberByAdding: decimal_number] }
    }

    fn decimal_number_by_subtracting(&self, decimal_number: Self) -> Self {
        unsafe { msg_send![self.obj, decimalNumberBySubtracting: decimal_number] }
    }

    fn decimal_number_by_multiplying_by(&self, decimal_number: Self) -> Self {
        unsafe { msg_send![self.obj, decimalNumberByMultiplyingBy: decimal_number] }
    }

    fn decimal_number_by_dividing_by(&self, decimal_number: Self) -> Self {
        unsafe { msg_send![self.obj, decimalNumberByDividingBy: decimal_number] }
    }

    fn decimal_number_by_raising_to_power(&self, power: c_uint) -> Self {
        unsafe { msg_send![self.obj, decimalNumberByRaisingToPower: power] }
    }

    fn decimal_number_by_multiplying_by_power_of_10(&self, power: c_short) -> Self {
        unsafe { msg_send![self.obj, decimalNumberByMultiplyingByPowerOf10: power] }
    }

    fn decimal_number_by_adding_with_behavior(
        &self,
        decimal_number: &Self,
        with_behavior: Arc<dyn crate::foundation::traits::t_NSDecimalNumberBehaviors>,
    ) -> Self {
        unsafe {
            msg_send![
                self.obj,
                decimalNumberByAdding: decimal_number
                withBehavior: with_behavior
            ]
        }
    }

    fn decimal_number_by_subtracting_with_behavior(
        &self,
        decimal_number: &Self,
        with_behavior: Arc<dyn crate::foundation::traits::t_NSDecimalNumberBehaviors>,
    ) -> Self {
        unsafe {
            msg_send![
                self.obj,
                decimalNumberBySubtracting: decimal_number
                withBehavior: with_behavior
            ]
        }
    }

    fn decimal_number_by_multiplying_by_with_behavior(
        &self,
        decimal_number: &Self,
        with_behavior: Arc<dyn crate::foundation::traits::t_NSDecimalNumberBehaviors>,
    ) -> Self {
        unsafe {
            msg_send![
                self.obj,
                decimalNumberByMultiplyingBy: decimal_number
                withBehavior: with_behavior
            ]
        }
    }

    fn decimal_number_by_dividing_by_with_behavior(
        &self,
        decimal_number: &Self,
        with_behavior: Arc<dyn crate::foundation::traits::t_NSDecimalNumberBehaviors>,
    ) -> Self {
        unsafe {
            msg_send![
                self.obj,
                decimalNumberByDividingBy: decimal_number
                withBehavior: with_behavior
            ]
        }
    }

    fn decimal_number_by_raising_to_power_with_behavior(
        &self,
        power: c_uint,
        with_behavior: Arc<dyn crate::foundation::traits::t_NSDecimalNumberBehaviors>,
    ) -> Self {
        unsafe {
            msg_send![
                self.obj,
                decimalNumberByRaisingToPower: power
                withBehavior: with_behavior
            ]
        }
    }

    fn decimal_number_by_multiplying_by_power_of_10_with_behavior(
        &self,
        power: c_short,
        with_behavior: Arc<dyn crate::foundation::traits::t_NSDecimalNumberBehaviors>,
    ) -> Self {
        unsafe {
            msg_send![
                self.obj,
                decimalNumberByMultiplyingByPowerOf10: power
                withBehavior: with_behavior
            ]
        }
    }

    fn decimal_number_by_rounding_according_to_behavior(
        &self,
        behavior: Arc<dyn crate::foundation::traits::t_NSDecimalNumberBehaviors>,
    ) -> Self {
        unsafe {
            msg_send![
                self.obj,
                decimalNumberByRoundingAccordingToBehavior: behavior
            ]
        }
    }

    fn default_behavior() -> Arc<dyn crate::foundation::traits::t_NSDecimalNumberBehaviors> {
        todo!()
    }

    fn set_default_behavior(
        behavior: Arc<dyn crate::foundation::traits::t_NSDecimalNumberBehaviors>,
    ) {
        unsafe { msg_send![class!(NSDecimalNumber), setDefaultBehavior: behavior] }
    }

    fn double_value(&self) -> f64 {
        unsafe { msg_send![self.obj, doubleValue] }
    }

    fn description_with_locale<L>(&self, locale: L) -> String
    where
        L: crate::foundation::traits::t_NSLocale,
    {
        unsafe {
            let class: String = msg_send![self.obj, descriptionWithLocale: locale];
            class
        }
    }

    fn objc_type(&self) -> *const libc::c_char {
        unsafe { msg_send![self.obj, objCType] }
    }

    fn compare(&self, decimal_number: &Self) -> ComparisonResult {
        unsafe {
            let class: ComparisonResult = msg_send![self.obj, compare: decimal_number];
            class
        }
    }

    fn maximum_decimal_number() -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), maximumDecimalNumber] }
    }

    fn minimum_decimal_number() -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), minimumDecimalNumber] }
    }
}

impl fmt::Debug for NSDecimalNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.debug_description())
    }
}

impl fmt::Display for NSDecimalNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Clone for NSDecimalNumber {
    fn clone(&self) -> Self {
        self.retain()
    }
}

impl Deref for NSDecimalNumber {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.obj
    }
}

impl DerefMut for NSDecimalNumber {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.obj
    }
}

impl From<c_uint> for NSDecimalNumber {
    fn from(value: c_uint) -> Self {
        NSDecimalNumber::number_with_unsigned_int(value)
    }
}

impl From<c_int> for NSDecimalNumber {
    fn from(value: c_int) -> Self {
        NSDecimalNumber::number_with_int(value)
    }
}

impl From<c_short> for NSDecimalNumber {
    fn from(value: c_short) -> Self {
        NSDecimalNumber::number_with_short(value)
    }
}

impl From<c_ushort> for NSDecimalNumber {
    fn from(value: c_ushort) -> Self {
        NSDecimalNumber::number_with_unsigned_short(value)
    }
}

impl From<c_long> for NSDecimalNumber {
    fn from(value: c_long) -> Self {
        NSDecimalNumber::number_with_long(value)
    }
}

impl From<c_ulong> for NSDecimalNumber {
    fn from(value: c_ulong) -> Self {
        NSDecimalNumber::number_with_unsigned_long(value)
    }
}

impl From<c_float> for NSDecimalNumber {
    fn from(value: c_float) -> Self {
        NSDecimalNumber::number_with_float(value)
    }
}

impl From<c_double> for NSDecimalNumber {
    fn from(value: c_double) -> Self {
        NSDecimalNumber::number_with_double(value)
    }
}

impl<S> From<S> for NSDecimalNumber
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        NSDecimalNumber::decimal_number_with_string(value)
    }
}

impl<T> Add<T> for NSDecimalNumber
where
    T: Into<NSDecimalNumber>,
{
    type Output = NSDecimalNumber;

    fn add(self, other: T) -> Self::Output {
        self.decimal_number_by_adding(other.into())
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
        self.decimal_number_by_subtracting(other.into())
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
        self.decimal_number_by_multiplying_by(other.into())
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
        self.decimal_number_by_dividing_by(other.into())
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

/// These constants specify rounding behaviors.
#[derive(Debug)]
#[repr(u64)]
pub enum NSRoundingMode {
    /// Round to the closest possible return value; when caught halfway between two positive numbers, round up; when caught between two negative numbers, round down.
    RoundPlain,
    /// Round return values down.
    RoundDown,
    /// Round return values up.
    RoundUp,
    /// Round to the closest possible return value; when halfway between two possibilities, return the possibility whose last digit is even.
    RoundBankers,
}

/// Calculation error constants used to describe an error in exceptionDuringOperation:error:leftOperand:rightOperand:.
#[derive(Debug)]
#[repr(u64)]
pub enum NSCalculationError {
    /// No error occurred.
    None,
    /// The number can’t be represented in 38 significant digits.
    PrecisionLoss,
    /// The number is too small to represent.
    Underflow,
    /// The number is too large to represent.
    Overflow,
    /// The caller tried to divide by 0.
    DivideByZero,
}
