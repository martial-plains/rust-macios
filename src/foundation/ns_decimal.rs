use std::{
    fmt,
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    os::raw::c_float,
    sync::Arc,
};

use libc::{c_double, c_int, c_long, c_longlong, c_short, c_uint, c_ulong, c_ushort};
use objc::{
    class, msg_send,
    runtime::{Class, Object, Protocol, Sel},
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

    fn im_is_equal(&self, object: &Self) -> bool {
        unsafe { msg_send![&*self.ptr, isEqual: object] }
    }

    fn ip_hash(&self) -> super::UInt {
        unsafe { msg_send![&*self.ptr, hash] }
    }

    fn im_is_kind_of_class(&self, class: Class) -> bool {
        unsafe { msg_send![&*self.ptr, isKindOfClass: class] }
    }

    fn im_is_member_of_class(&self, class: Class) -> bool {
        unsafe { msg_send![&*self.ptr, isMemberOfClass: class] }
    }

    fn im_responds_to_selector(&self, selector: Sel) -> bool {
        unsafe { msg_send![&*self.ptr, respondsToSelector: selector] }
    }

    fn im_conforms_to_protocol(&self, protocol: Protocol) -> bool {
        unsafe { msg_send![&*self.ptr, conformsToProtocol: protocol] }
    }

    fn ip_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![&*self.ptr, description]) }
    }

    fn ip_debug_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![&*self.ptr, debugDescription]) }
    }

    fn im_perform_selector(&self, selector: Sel) -> id {
        unsafe { msg_send![&*self.ptr, performSelector: selector] }
    }

    fn im_perform_selector_with_object(&self, selector: Sel, with_object: id) -> id {
        unsafe { msg_send![&*self.ptr, performSelector: selector withObject: with_object] }
    }

    fn im_is_proxy(&self) -> bool {
        unsafe { msg_send![&*self.ptr, isProxy] }
    }
}

impl INSValue for NSDecimalNumber {}

impl INSNumber for NSDecimalNumber {
    fn im_init_with_bool(&self, _value: bool) -> Self {
        todo!()
    }

    fn im_init_with_char(&self, _value: libc::c_schar) -> Self {
        todo!()
    }

    fn im_init_with_double(&self, _value: c_double) -> Self {
        todo!()
    }

    fn im_init_with_float(&self, _value: libc::c_float) -> Self {
        todo!()
    }

    fn im_init_with_int(&self, _value: c_int) -> Self {
        todo!()
    }

    fn im_init_with_integer(&self, _value: super::Int) -> Self {
        todo!()
    }

    fn im_init_with_long(&self, _value: c_long) -> Self {
        todo!()
    }

    fn im_init_with_long_long(&self, _value: c_longlong) -> Self {
        todo!()
    }

    fn im_init_with_short(&self, _value: c_short) -> Self {
        todo!()
    }

    fn im_init_with_unsigned_char(&self, _value: libc::c_uchar) -> Self {
        todo!()
    }

    fn im_init_with_unsigned_int(&self, _value: c_uint) -> Self {
        todo!()
    }

    fn im_init_with_unsigned_integer(&self, _value: c_uint) -> Self {
        todo!()
    }

    fn im_init_with_unsigned_long(&self, _value: c_ulong) -> Self {
        todo!()
    }

    fn im_init_with_unsigned_long_long(&self, _value: libc::c_ulonglong) -> Self {
        todo!()
    }

    fn im_init_with_unsigned_short(&self, _value: c_ushort) -> Self {
        todo!()
    }

    fn ip_bool_value(&self) -> bool {
        unsafe { msg_send![&*self.ptr, boolValue] }
    }

    fn ip_char_value(&self) -> libc::c_schar {
        unsafe { msg_send![&*self.ptr, charValue] }
    }

    fn ip_double_value(&self) -> c_double {
        unsafe { msg_send![&*self.ptr, doubleValue] }
    }

    fn ip_float_value(&self) -> libc::c_float {
        unsafe { msg_send![&*self.ptr, floatValue] }
    }

    fn ip_int_value(&self) -> c_int {
        unsafe { msg_send![&*self.ptr, intValue] }
    }

    fn ip_integer_value(&self) -> super::Int {
        unsafe { msg_send![&*self.ptr, integerValue] }
    }

    fn ip_long_long_value(&self) -> c_longlong {
        unsafe { msg_send![&*self.ptr, longLongValue] }
    }

    fn ip_long_value(&self) -> c_long {
        unsafe { msg_send![&*self.ptr, longValue] }
    }

    fn ip_short_value(&self) -> c_short {
        unsafe { msg_send![&*self.ptr, shortValue] }
    }

    fn ip_unsigned_char_value(&self) -> libc::c_uchar {
        unsafe { msg_send![&*self.ptr, unsignedCharValue] }
    }

    fn ip_unsigned_integer_value(&self) -> super::UInt {
        unsafe { msg_send![&*self.ptr, unsignedIntegerValue] }
    }

    fn ip_unsigned_int_value(&self) -> c_uint {
        unsafe { msg_send![&*self.ptr, unsignedIntValue] }
    }

    fn ip_unsigned_long_long_value(&self) -> libc::c_ulonglong {
        unsafe { msg_send![&*self.ptr, unsignedLongLongValue] }
    }

    fn ip_unsigned_long_value(&self) -> c_ulong {
        unsafe { msg_send![&*self.ptr, unsignedLongValue] }
    }

    fn ip_unsigned_short_value(&self) -> c_ushort {
        unsafe { msg_send![&*self.ptr, unsignedShortValue] }
    }

    fn im_description_with_locale(&self, locale: NSLocale) -> NSString {
        unsafe {
            let id: id = msg_send![&*self.ptr, descriptionWithLocale: locale.obj];
            NSString::from_id(id)
        }
    }

    fn ip_string_value(&self) -> NSString {
        unsafe {
            let id: id = msg_send![&*self.ptr, stringValue];
            NSString::from_id(id)
        }
    }

    fn im_compare(&self, other: &Self) -> NSComparisonResult {
        unsafe { msg_send![&*self.ptr, compare: other] }
    }

    fn im_is_equal_to_number(&self, other: Self) -> bool {
        unsafe { msg_send![&*self.ptr, isEqualToNumber: other] }
    }

    fn ip_decimal_value(&self) -> NSDecimal {
        unsafe { msg_send![&*self.ptr, decimalValue] }
    }
}

impl INSDecimalNumber for NSDecimalNumber {
    fn im_init_with_decimal(&mut self, decimal: NSDecimalNumber) {
        unsafe {
            let _: id = msg_send![&*self.ptr, initWithDecimal: decimal.ptr];
        }
    }

    fn im_init_with_mantissa_exponent_is_negative(
        &mut self,
        mantissa: libc::c_ulonglong,
        exponent: c_short,
        is_negative: bool,
    ) {
        unsafe {
            let _: id = msg_send![
                self.ptr,
                initWithMantissa: mantissa
                exponent: exponent
                isNegative: is_negative
            ];
        }
    }

    fn im_init_with_string<S>(&mut self, string: S)
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![&*self.ptr, initWithString: string.into()] }
    }

    fn im_init_with_string_locale<S, L>(&mut self, string: S, locale: L)
    where
        S: Into<NSString>,
        L: crate::foundation::traits::INSLocale,
    {
        unsafe { msg_send![&*self.ptr, initWithString: string.into() locale: locale] }
    }

    fn im_decimal_number_by_adding(&self, decimal_number: Self) -> Self {
        unsafe { msg_send![&*self.ptr, decimalNumberByAdding: decimal_number] }
    }

    fn im_decimal_number_by_subtracting(&self, decimal_number: Self) -> Self {
        unsafe { msg_send![&*self.ptr, decimalNumberBySubtracting: decimal_number] }
    }

    fn im_decimal_number_by_multiplying_by(&self, decimal_number: Self) -> Self {
        unsafe { msg_send![&*self.ptr, decimalNumberByMultiplyingBy: decimal_number] }
    }

    fn im_decimal_number_by_dividing_by(&self, decimal_number: Self) -> Self {
        unsafe { msg_send![&*self.ptr, decimalNumberByDividingBy: decimal_number] }
    }

    fn im_decimal_number_by_raising_to_power(&self, power: c_uint) -> Self {
        unsafe { msg_send![&*self.ptr, decimalNumberByRaisingToPower: power] }
    }

    fn im_decimal_number_by_multiplying_by_power_of_10(&self, power: c_short) -> Self {
        unsafe { msg_send![&*self.ptr, decimalNumberByMultiplyingByPowerOf10: power] }
    }

    fn im_decimal_number_by_adding_with_behavior(
        &self,
        decimal_number: &Self,
        with_behavior: Arc<dyn crate::foundation::traits::PNSDecimalNumberBehaviors>,
    ) -> Self {
        unsafe {
            msg_send![
                self.ptr,
                decimalNumberByAdding: decimal_number
                withBehavior: with_behavior
            ]
        }
    }

    fn im_decimal_number_by_subtracting_with_behavior(
        &self,
        decimal_number: &Self,
        with_behavior: Arc<dyn crate::foundation::traits::PNSDecimalNumberBehaviors>,
    ) -> Self {
        unsafe {
            msg_send![
                self.ptr,
                decimalNumberBySubtracting: decimal_number
                withBehavior: with_behavior
            ]
        }
    }

    fn im_decimal_number_by_multiplying_by_with_behavior(
        &self,
        decimal_number: &Self,
        with_behavior: Arc<dyn crate::foundation::traits::PNSDecimalNumberBehaviors>,
    ) -> Self {
        unsafe {
            msg_send![
                self.ptr,
                decimalNumberByMultiplyingBy: decimal_number
                withBehavior: with_behavior
            ]
        }
    }

    fn im_decimal_number_by_dividing_by_with_behavior(
        &self,
        decimal_number: &Self,
        with_behavior: Arc<dyn crate::foundation::traits::PNSDecimalNumberBehaviors>,
    ) -> Self {
        unsafe {
            msg_send![
                self.ptr,
                decimalNumberByDividingBy: decimal_number
                withBehavior: with_behavior
            ]
        }
    }

    fn im_decimal_number_by_raising_to_power_with_behavior(
        &self,
        power: c_uint,
        with_behavior: Arc<dyn crate::foundation::traits::PNSDecimalNumberBehaviors>,
    ) -> Self {
        unsafe {
            msg_send![
                self.ptr,
                decimalNumberByRaisingToPower: power
                withBehavior: with_behavior
            ]
        }
    }

    fn im_decimal_number_by_multiplying_by_power_of10_with_behavior(
        &self,
        power: c_short,
        with_behavior: Arc<dyn crate::foundation::traits::PNSDecimalNumberBehaviors>,
    ) -> Self {
        unsafe {
            msg_send![
                self.ptr,
                decimalNumberByMultiplyingByPowerOf10: power
                withBehavior: with_behavior
            ]
        }
    }

    fn im_decimal_number_by_rounding_according_to_behavior(
        &self,
        behavior: Arc<dyn crate::foundation::traits::PNSDecimalNumberBehaviors>,
    ) -> Self {
        unsafe {
            msg_send![
                self.ptr,
                decimalNumberByRoundingAccordingToBehavior: behavior
            ]
        }
    }

    fn ip_decimal_value(&self) -> NSDecimal {
        unsafe { msg_send![&*self.ptr, decimalValue] }
    }

    fn ip_double_value(&self) -> f64 {
        unsafe { msg_send![&*self.ptr, doubleValue] }
    }

    fn im_description_with_locale<L>(&self, locale: L) -> NSString
    where
        L: crate::foundation::traits::INSLocale,
    {
        unsafe {
            let class: NSString = msg_send![&*self.ptr, descriptionWithLocale: locale];
            class
        }
    }

    fn ip_objc_type(&self) -> *const libc::c_char {
        unsafe { msg_send![&*self.ptr, objCType] }
    }

    fn im_compare(&self, decimal_number: &Self) -> NSComparisonResult {
        unsafe {
            let class: NSComparisonResult = msg_send![&*self.ptr, compare: decimal_number];
            class
        }
    }
}

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
