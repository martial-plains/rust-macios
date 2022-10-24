use objc::runtime::{BOOL, NO, YES};

#[cfg(feature = "objective_c_runtime")]
use crate::objective_c_runtime::{id, nil, traits::FromId};
#[cfg(feature = "objective_c_runtime")]
use objc::runtime::Object;

/// A helper function to convert an Objective-C bool to a Rust bool.
#[inline(always)]
pub fn to_bool(result: BOOL) -> bool {
    match result {
        YES => true,
        NO => false,

        #[cfg(not(target_arch = "aarch64"))]
        _ => {
            std::unreachable!();
        }
    }
}

#[cfg(feature = "objective_c_runtime")]
#[inline(always)]
pub fn to_optional<T>(ptr: id) -> Option<T>
where
    T: Sized + FromId,
{
    unsafe {
        if ptr != nil {
            Some(T::from_id(ptr))
        } else {
            None
        }
    }
}

/// Getting the instance variable of an object.
#[cfg(feature = "objective_c_runtime")]
pub fn get_variable<'a, T>(this: &'a Object, ptr_name: &str) -> &'a T {
    unsafe {
        let ptr: usize = *this.get_ivar(ptr_name);
        let obj = ptr as *const T;
        &*obj
    }
}
