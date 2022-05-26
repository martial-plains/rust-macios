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
        ComparisonResult, NSLocale, NSString,
    },
    id,
    objective_c_runtime::traits::{t_NSObject, t_NSValue},
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
    ) -> ComparisonResult;
}

/// An object for representing and performing arithmetic on base-10 numbers.
pub struct NSDecimalNumber {
    /// The raw pointer to the Objective-C object.
    pub obj: Id<Object>,
}

impl t_NSObject for NSDecimalNumber {
    fn new() -> Self {
        unsafe {
            let class: id = msg_send![class!(NSDecimalNumber), alloc];
            let obj = msg_send![class, init];
            NSDecimalNumber { obj }
        }
    }

    fn toId(mut self) -> id {
        &mut *self.obj
    }

    unsafe fn fromId(obj: id) -> Self {
        Self {
            obj: Id::from_ptr(obj),
        }
    }

    fn description(&self) -> NSString {
        unsafe {
            let obj = msg_send![&*self.obj, description];
            NSString::fromId(obj)
        }
    }

    fn debugDescription(&self) -> NSString {
        unsafe {
            let obj = msg_send![&*self.obj, debugDescription];
            NSString::fromId(obj)
        }
    }

    fn retain(&self) -> Self {
        unsafe {
            let obj = msg_send![&*self.obj, retain];
            Self::fromId(obj)
        }
    }
}

impl t_NSValue for NSDecimalNumber {}

impl t_NSNumber for NSDecimalNumber {
    fn numberWithBool(value: bool) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithBool: value] }
    }

    fn numberWithChar(value: libc::c_schar) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithChar: value] }
    }

    fn numberWithDouble(value: c_double) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithDouble: value] }
    }

    fn numberWithFloat(value: libc::c_float) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithFloat: value] }
    }

    fn numberWithInt(value: c_int) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithInt: value] }
    }

    fn numberWithInteger(value: super::Int) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithInteger: value] }
    }

    fn numberWithLong(value: c_long) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithLong: value] }
    }

    fn numberWithLongLong(value: c_longlong) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithLongLong: value] }
    }

    fn numberWithShort(value: c_short) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithShort: value] }
    }

    fn numberWithUnsignedChar(value: libc::c_uchar) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithUnsignedChar: value] }
    }

    fn numberWithUnsignedInt(value: c_uint) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithUnsignedInt: value] }
    }

    fn numberWithUnsignedInteger(value: super::UInt) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithUnsignedInteger: value] }
    }

    fn numberWithUnsignedLong(value: c_ulong) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithUnsignedLong: value] }
    }

    fn numberWithUnsignedLongLong(value: libc::c_ulonglong) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithUnsignedLongLong: value] }
    }

    fn numberWithUnsignedShort(value: c_ushort) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), numberWithUnsignedShort: value] }
    }

    fn initWithBool(&self, _value: bool) -> Self {
        todo!()
    }

    fn initWithChar(&self, _value: libc::c_schar) -> Self {
        todo!()
    }

    fn initWithDouble(&self, _value: c_double) -> Self {
        todo!()
    }

    fn initWithFloat(&self, _value: libc::c_float) -> Self {
        todo!()
    }

    fn initWithInt(&self, _value: c_int) -> Self {
        todo!()
    }

    fn initWithInteger(&self, _value: super::Int) -> Self {
        todo!()
    }

    fn initWithLong(&self, _value: c_long) -> Self {
        todo!()
    }

    fn initWithLongLong(&self, _value: c_longlong) -> Self {
        todo!()
    }

    fn initWithShort(&self, _value: c_short) -> Self {
        todo!()
    }

    fn initWithUnsignedChar(&self, _value: libc::c_uchar) -> Self {
        todo!()
    }

    fn initWithUnsignedInt(&self, _value: c_uint) -> Self {
        todo!()
    }

    fn initWithUnsignedInteger(&self, _value: c_uint) -> Self {
        todo!()
    }

    fn initWithUnsignedLong(&self, _value: c_ulong) -> Self {
        todo!()
    }

    fn initWithUnsignedLongLong(&self, _value: libc::c_ulonglong) -> Self {
        todo!()
    }

    fn initWithUnsignedShort(&self, _value: c_ushort) -> Self {
        todo!()
    }

    fn boolValue(&self) -> bool {
        unsafe { msg_send![self.obj, boolValue] }
    }

    fn charValue(&self) -> libc::c_schar {
        unsafe { msg_send![self.obj, charValue] }
    }

    fn doubleValue(&self) -> c_double {
        unsafe { msg_send![self.obj, doubleValue] }
    }

    fn floatValue(&self) -> libc::c_float {
        unsafe { msg_send![self.obj, floatValue] }
    }

    fn intValue(&self) -> c_int {
        unsafe { msg_send![self.obj, intValue] }
    }

    fn integerValue(&self) -> super::Int {
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

    fn unsignedCharValue(&self) -> libc::c_uchar {
        unsafe { msg_send![self.obj, unsignedCharValue] }
    }

    fn unsignedIntegerValue(&self) -> super::UInt {
        unsafe { msg_send![self.obj, unsignedIntegerValue] }
    }

    fn unsignedIntValue(&self) -> c_uint {
        unsafe { msg_send![self.obj, unsignedIntValue] }
    }

    fn unsignedLongLongValue(&self) -> libc::c_ulonglong {
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
            let id: id = msg_send![self.obj, descriptionWithLocale: locale.obj];
            NSString::fromId(id)
        }
    }

    fn stringValue(&self) -> NSString {
        unsafe {
            let id: id = msg_send![self.obj, stringValue];
            NSString::fromId(id)
        }
    }

    fn compare(&self, other: &Self) -> ComparisonResult {
        unsafe { msg_send![self.obj, compare: other] }
    }

    fn isEqualToNumber(&self, other: &Self) -> bool {
        unsafe { msg_send![self.obj, isEqualToNumber: other] }
    }
}

impl t_NSDecimalNumber for NSDecimalNumber {
    fn decimalNumberWithDecimal(decimal: NSDecimalNumber) -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), decimalNumberWithDecimal: decimal.obj] }
    }

    fn decimalNumberWithMantissa(
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

    fn decimalNumberWithString<S>(string: S) -> Self
    where
        S: Into<NSString>,
    {
        unsafe {
            msg_send![
                class!(NSDecimalNumber),
                decimalNumberWithString: string.into()
            ]
        }
    }

    fn decimalNumberWithStringLocale<S, L>(string: S, locale: L) -> Self
    where
        S: Into<NSString>,
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

    fn notANumber() -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), notANumber] }
    }

    fn initWithDecimal(&mut self, decimal: NSDecimalNumber) {
        unsafe {
            let _: id = msg_send![self.obj, initWithDecimal: decimal.obj];
        }
    }

    fn initWithMantissaExponentIsNegative(
        &mut self,
        mantissa: libc::c_ulonglong,
        exponent: c_short,
        is_negative: bool,
    ) {
        unsafe {
            let _: id = msg_send![
                self.obj,
                initWithMantissa: mantissa
                exponent: exponent
                isNegative: is_negative
            ];
        }
    }

    fn initWithString<S>(&mut self, string: S)
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.obj, initWithString: string.into()] }
    }

    fn initWithStringLocale<S, L>(&mut self, string: S, locale: L)
    where
        S: Into<NSString>,
        L: crate::foundation::traits::t_NSLocale,
    {
        unsafe { msg_send![self.obj, initWithString: string.into() locale: locale] }
    }

    fn decimalNumberByAdding(&self, decimal_number: Self) -> Self {
        unsafe { msg_send![self.obj, decimalNumberByAdding: decimal_number] }
    }

    fn decimalNumberBySubtracting(&self, decimal_number: Self) -> Self {
        unsafe { msg_send![self.obj, decimalNumberBySubtracting: decimal_number] }
    }

    fn decimalNumberByMultiplyingBy(&self, decimal_number: Self) -> Self {
        unsafe { msg_send![self.obj, decimalNumberByMultiplyingBy: decimal_number] }
    }

    fn decimalNumberByDividingBy(&self, decimal_number: Self) -> Self {
        unsafe { msg_send![self.obj, decimalNumberByDividingBy: decimal_number] }
    }

    fn decimalNumberByRaisingToPower(&self, power: c_uint) -> Self {
        unsafe { msg_send![self.obj, decimalNumberByRaisingToPower: power] }
    }

    fn decimalNumberByMultiplyingByPowerOf10(&self, power: c_short) -> Self {
        unsafe { msg_send![self.obj, decimalNumberByMultiplyingByPowerOf10: power] }
    }

    fn decimalNumberByAddingWithBehavior(
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

    fn decimalNumberBySubtractingWithBehavior(
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

    fn decimalNumberByMultiplyingByWithBehavior(
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

    fn decimalNumberByDividingByWithBehavior(
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

    fn decimalNumberByRaisingToPowerWithBehavior(
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

    fn decimalNumberByMultiplyingByPowerOf10WithBehavior(
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

    fn decimalNumberByRoundingAccordingToBehavior(
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

    fn defaultBehavior() -> Arc<dyn crate::foundation::traits::t_NSDecimalNumberBehaviors> {
        todo!()
    }

    fn setDefaultBehavior(
        behavior: Arc<dyn crate::foundation::traits::t_NSDecimalNumberBehaviors>,
    ) {
        unsafe { msg_send![class!(NSDecimalNumber), setDefaultBehavior: behavior] }
    }

    fn doubleValue(&self) -> f64 {
        unsafe { msg_send![self.obj, doubleValue] }
    }

    fn descriptionWithLocale<L>(&self, locale: L) -> NSString
    where
        L: crate::foundation::traits::t_NSLocale,
    {
        unsafe {
            let class: NSString = msg_send![self.obj, descriptionWithLocale: locale];
            class
        }
    }

    fn objcType(&self) -> *const libc::c_char {
        unsafe { msg_send![self.obj, objCType] }
    }

    fn compare(&self, decimal_number: &Self) -> ComparisonResult {
        unsafe {
            let class: ComparisonResult = msg_send![self.obj, compare: decimal_number];
            class
        }
    }

    fn maximumDecimalNumber() -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), maximumDecimalNumber] }
    }

    fn minimumDecimalNumber() -> Self {
        unsafe { msg_send![class!(NSDecimalNumber), minimumDecimalNumber] }
    }
}

impl fmt::Debug for NSDecimalNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.debugDescription())
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
        NSDecimalNumber::numberWithUnsignedInt(value)
    }
}

impl From<c_int> for NSDecimalNumber {
    fn from(value: c_int) -> Self {
        NSDecimalNumber::numberWithInt(value)
    }
}

impl From<c_short> for NSDecimalNumber {
    fn from(value: c_short) -> Self {
        NSDecimalNumber::numberWithShort(value)
    }
}

impl From<c_ushort> for NSDecimalNumber {
    fn from(value: c_ushort) -> Self {
        NSDecimalNumber::numberWithUnsignedShort(value)
    }
}

impl From<c_long> for NSDecimalNumber {
    fn from(value: c_long) -> Self {
        NSDecimalNumber::numberWithLong(value)
    }
}

impl From<c_ulong> for NSDecimalNumber {
    fn from(value: c_ulong) -> Self {
        NSDecimalNumber::numberWithUnsignedLong(value)
    }
}

impl From<c_float> for NSDecimalNumber {
    fn from(value: c_float) -> Self {
        NSDecimalNumber::numberWithFloat(value)
    }
}

impl From<c_double> for NSDecimalNumber {
    fn from(value: c_double) -> Self {
        NSDecimalNumber::numberWithDouble(value)
    }
}

impl<S> From<S> for NSDecimalNumber
where
    S: Into<NSString>,
{
    fn from(value: S) -> Self {
        NSDecimalNumber::decimalNumberWithString(value)
    }
}

impl<T> Add<T> for NSDecimalNumber
where
    T: Into<NSDecimalNumber>,
{
    type Output = NSDecimalNumber;

    fn add(self, other: T) -> Self::Output {
        self.decimalNumberByAdding(other.into())
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
        self.decimalNumberBySubtracting(other.into())
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
        self.decimalNumberByMultiplyingBy(other.into())
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
        self.decimalNumberByDividingBy(other.into())
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
