use libc::{c_double, c_void};

use super::CGFunction;

/// A general facility for defining and using callback functions.
pub type CGFunctionRef = CGFunction;

/// The basic type for all floating-point values.
pub type CGFloat = c_double;

/// Performs custom operations on the supplied input data to produce output data.
pub type CGFunctionEvaluateCallback =
    extern "C" fn(info: *mut c_void, in_values: *const CGFloat, out_values: *mut CGFloat);

/// Performs custom clean-up tasks when Core Graphics deallocates a CGFunctionRef object.
pub type CGFunctionReleaseInfoCallback = extern "C" fn(info: *mut c_void);
