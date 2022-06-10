use libc::{c_void, size_t};
use objc::runtime::Object;
use objc_id::Id;

use crate::core_foundation::CFTypeID;

use super::{CGFloat, CGFunctionCallbacks, CGFunctionRef};

/// A general facility for defining and using callback functions.
#[derive(Debug)]
#[repr(transparent)]
pub struct CGFunction {
    /// The underlying `CGFunctionRef`.
    pub ptr: Id<Object>,
}

impl CGFunction {
    /// Creates a Core Graphics function.
    ///
    /// # Parameters
    ///
    /// * `info`: A pointer to user-defined storage for data that you want to pass to your callbacks. You need to make sure that the data persists for as long as it’s needed, which can be beyond the scope in which the Core Graphics function is used.
    /// * `domain_dimension`: The number of inputs.
    /// * `domain`: An array of (2*domainDimension) floats used to specify the valid intervals of input values. For each k from 0 to (domainDimension - 1), domain[2*k] must be less than or equal to domain[2*k+1], and the kth input value will be clipped to lie in the interval domain[2*k] ≤ input[k] ≤ domain[2*k+1]. If this parameter is NULL, then the input values are not clipped.
    /// * `range_dimension`: The number of outputs.
    /// * `range`: An array of (2*rangeDimension) floats that specifies the valid intervals of output values. For each k from 0 to (rangeDimension - 1), range[2*k] must be less than or equal to range[2*k+1], and the kth output value will be clipped to lie in the interval range[2*k] ≤ output[k] ≤ range[2*k+1]. If this parameter is NULL, then the output values are not clipped.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        unsafe {
            let callbacks = CGFunctionCallbacks {
                version: 0,
                evaluate: None,
                release_info: None,
            };

            let function = CGFunction::create(
                std::ptr::null_mut(),
                0,
                std::ptr::null(),
                0,
                std::ptr::null(),
                &callbacks,
            );

            let id = CGFunction::type_id();

            assert!(id != 0);

            CGFunction::release(function);
        }
    }
}
