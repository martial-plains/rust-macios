use crate::objective_c_runtime::macros::object;

object! {
    /// Information about an error condition including a domain, a domain-specific
    /// error code, and application-specific information.
    unsafe pub struct NSError;
}

impl NSError {}
