/// Internal object of objects that belong to the Core Foundation framework
pub trait CFTypeObject {
    type Ref;

    /// Retrieves the internal object
    fn get_internal_object(&self) -> Self::Ref;

    fn create_with_ref(obj: Self::Ref) -> Self;
}
