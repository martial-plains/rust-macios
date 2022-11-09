use libc::{c_uint, c_void, size_t};

use crate::{core_foundation::CFTypeID, object};

use super::CGFloat;

/// A general facility for defining and using callback functions.
pub type CGFunctionRef = CGFunction;

object! {
    /// A general facility for defining and using callback functions.
    unsafe pub  struct CGFunction;
}

impl CGFunction {
    /// Creates a Core Graphics function.
    ///
    /// # Parameters
    ///
    /// * `info`: A pointer to user-defined storage for data that you want to pass to your callbacks. You need to make sure that the data persists for as long as it’s needed, which can be beyond the scope in which the Core Graphics function is used.
    /// * `domain_dimension`: The number of inputs.
    /// * `domain`: An array of (2*domainDimension) floats used to specify the valid intervals of input values. For each k from 0 to (domainDimension - 1), `domain[2*k]` must be less than or equal to `domain[2*k+1]`, and the kth input value will be clipped to lie in the interval `domain[2*k]` ≤ `input[k]` ≤ `domain[2*k+1]`. If this parameter is NULL, then the input values are not clipped.
    /// * `range_dimension`: The number of outputs.
    /// * `range`: An array of (2*rangeDimension) floats that specifies the valid intervals of output values. For each k from 0 to (rangeDimension - 1), `range[2*k]` must be less than or equal to `range[2*k+1]`, and the kth output value will be clipped to lie in the interval `range[2*k]` ≤ `output[k]` `≤ range[2*k+1]`. If this parameter is NULL, then the output values are not clipped.
    /// * `callbacks`: A pointer to a callback function table. This table should contain pointers to the callbacks you provide to implement the semantics of this Core Graphics function. Core Graphics makes a copy of your table, so, for example, you could safely pass in a pointer to a structure on the stack.
    ///
    /// # Returns
    ///
    /// The new Core Graphics function. You are responsible for releasing this object using `release`.
    ///
    /// # Safety
    ///
    /// The `callbacks` parameter must be a valid pointer to a CGFunctionCallbacks structure.
    pub unsafe fn create(
        info: *mut c_void,
        domain_dimension: size_t,
        domain: *const CGFloat,
        range_dimension: size_t,
        range: *const size_t,
        callbacks: *const CGFunctionCallbacks,
    ) -> CGFunctionRef {
        CGFunctionCreate(
            info,
            domain_dimension,
            domain,
            range_dimension,
            range,
            callbacks,
        )
    }

    /// Decrements the retain count of a function object.
    ///
    /// # Parameters
    ///
    /// * `function`: The function object to release.
    ///
    /// # Safety
    ///
    /// It does not cause an error if the function parameter is null.
    pub unsafe fn release(function: CGFunctionRef) {
        CGFunctionRelease(function);
    }

    /// Increments the retain count of a function object.
    ///
    /// # Parameters
    ///
    /// * `function`: The function object to retain.
    ///
    /// # Returns
    ///
    /// The function object passed in.
    ///
    /// # Safety
    ///
    /// It does not cause an error if the function parameter is null.
    pub unsafe fn retain(function: CGFunctionRef) -> CGFunctionRef {
        CGFunctionRetain(function)
    }

    /// Returns the type identifier for Core Graphics function objects.
    ///
    /// # Returns
    ///
    /// The type identifier for Core Graphics function objects.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not ensure that the returned value is a valid type identifier.
    pub unsafe fn type_id() -> CFTypeID {
        CGFunctionGetTypeID()
    }
}

extern "C" {
    fn CGFunctionCreate(
        info: *mut c_void,
        domainDimension: size_t,
        domain: *const CGFloat,
        rangeDimension: size_t,
        range: *const size_t,
        callbacks: *const CGFunctionCallbacks,
    ) -> CGFunctionRef;

    fn CGFunctionRelease(function: CGFunctionRef);

    fn CGFunctionRetain(function: CGFunctionRef) -> CGFunctionRef;

    fn CGFunctionGetTypeID() -> CFTypeID;
}

/// Performs custom operations on the supplied input data to produce output data.
pub type CGFunctionEvaluateCallback =
    extern "C" fn(info: *mut c_void, in_values: *const CGFloat, out_values: *mut CGFloat);

/// Performs custom clean-up tasks when Core Graphics deallocates a CGFunctionRef object.
pub type CGFunctionReleaseInfoCallback = extern "C" fn(info: *mut c_void);

/// A structure that contains callbacks needed by a CGFunctionRef object.
#[derive(Debug)]
pub struct CGFunctionCallbacks {
    /// The structure version number. For this structure,the version should be 0.
    pub version: c_uint,
    /// The callback that evaluates the function.
    pub evaluate: *mut CGFunctionEvaluateCallback,
    /// If non-NULL,the callback used to release the info parameterpassed to CGFunctionCreate.
    pub release_info: *mut CGFunctionReleaseInfoCallback,
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use super::*;

    #[test]
    fn test_create() {
        unsafe {
            let callbacks = CGFunctionCallbacks {
                version: 0,
                evaluate: ptr::null_mut(),
                release_info: ptr::null_mut(),
            };

            let function =
                CGFunction::create(ptr::null_mut(), 0, ptr::null(), 0, ptr::null(), &callbacks);

            let id = CGFunction::type_id();

            assert!(id != 0);

            CGFunction::release(function);
        }
    }
}
