/// Internal object of objects that belong to the Core Graphics framework
pub trait CGTypeObject {
    type Ref;

    /// Retrieves the internal object
    fn get_internal_object(&self) -> Self::Ref;

    fn create_with_ref(obj: Self::Ref) -> Self;
}
