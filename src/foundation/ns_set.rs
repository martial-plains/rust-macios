use std::marker::PhantomData;

use crate::object;

object! {
    /// A static, unordered collection of unique objects.
    unsafe pub struct NSSet<T> {
        marker: PhantomData<T>,
    }
}
