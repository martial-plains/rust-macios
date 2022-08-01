use libc::c_short;

use super::NSRoundingMode;

/// A protocol that declares three methods that control the discretionary aspects of working with decimal numbers.
pub trait PNSDecimalNumberBehaviors {
    /* Rounding Behavior
     */

    /// Returns the way that NSDecimalNumber's decimalNumberBy... methods round their return values.
    fn im_rounding_mode(&self) -> NSRoundingMode;

    /// Returns the number of digits allowed after the decimal separator.
    fn im_scale(&self) -> c_short;
}
