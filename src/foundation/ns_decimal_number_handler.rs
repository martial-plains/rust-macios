use libc::c_short;
use objc::{msg_send, sel, sel_impl};

use crate::{
    object,
    objective_c_runtime::{
        macros::interface_impl,
        traits::{FromId, PNSObject},
    },
};

use super::NSRoundingMode;

object! {
    /// A class that adopts the decimal number behaviors protocol.
    unsafe pub struct NSDecimalNumberHandler;
}

#[interface_impl(NSObject)]
impl NSDecimalNumberHandler {
    /// Returns the default instance of [`NSDecimalNumberHandler`].
    #[property]
    pub fn default_decimal_number_handler() -> NSDecimalNumberHandler {
        unsafe { msg_send![Self::m_class(), defaultDecimalNumberHandler] }
    }

    /// Returns an [`NSDecimalNumberHandler`] object with customized behavior.
    #[method]
    pub fn decimal_number_handler_with_rounding_mode_scale_raise_on_exactness_raise_on_overflow_raise_on_underflow_raise_on_divide_by_zero(
        rounding_mode: NSRoundingMode,
        scale: c_short,
        exact: bool,
        overflow: bool,
        underflow: bool,
        divide_by_zero: bool,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(
                msg_send![Self::m_class(), decimalNumberHandlerWithRoundingMode: rounding_mode scale: scale raiseOnExactness: exact raiseOnOverflow: overflow raiseOnUnderflow: underflow raiseOnDivideByZero: divide_by_zero ],
            )
        }
    }

    /// Returns an [`NSDecimalNumberHandler`] object initialized so it behaves as specified by the method’s arguments.
    #[method]
    pub fn init_with_rounding_mode_scale_raise_on_exactness_raise_on_overflow_raise_on_underflow_raise_on_divide_by_zero(
        &mut self,
        rounding_mode: NSRoundingMode,
        scale: c_short,
        exact: bool,
        overflow: bool,
        underflow: bool,
        divide_by_zero: bool,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(
                msg_send![self.m_self(), initWithRoundingMode: rounding_mode scale: scale raiseOnExactness: exact raiseOnOverflow: overflow raiseOnUnderflow: underflow raiseOnDivideByZero: divide_by_zero ],
            )
        }
    }
}
