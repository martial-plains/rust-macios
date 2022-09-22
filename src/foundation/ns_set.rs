use std::marker::PhantomData;

use crate::objective_c_runtime::macros::object;

object! {
    /// A static, unordered collection of unique objects.
    unsafe pub struct NSSet<T> {
        marker: PhantomData<T>,
    }
}
