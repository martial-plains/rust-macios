use libc::c_void;

/// Internal object of objects that belong to the Core Foundation framework
pub trait CFTypeObject {
    /// Retrieves the internal object
    fn get_internal_object(&self) -> &c_void;
}
